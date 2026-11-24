/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Queued input/document ownership, target identity and rejected adoption.
use super::*;
use crate::downstream::workbench::session;
use crate::shell::{Direction, RetainedResources};

const PROJECT: &[u8] =
    include_bytes!("../../../../../../examples/workbench/single/workbench.merlin.json");
fn candidate(context: &Context) -> PreparedProject {
    PreparedProject::prepare(
        serde_json::from_slice(PROJECT).unwrap(),
        None,
        &context.application,
        context.application.snapshot(Direction::Downstream).key,
        session::snapshot(context),
    )
    .unwrap()
}

#[test]
fn queued_input_owner_stays_charged_until_disposal_or_successful_target_transfer() {
    let context = Context::isolated();
    let budget = context.application.budget().clone();
    let before = budget.usage();
    let prepared = candidate(&context);
    let charge = prepared.accepted.fixture_reserved_resources();
    let complete = budget.usage();
    assert!(complete.payload_bytes >= before.payload_bytes + charge.payload_bytes);
    assert!(complete.ordered_entries >= before.ordered_entries + charge.ordered_entries);
    assert!(complete.shared_roots >= before.shared_roots + charge.shared_roots);
    assert!(prepared
        .models
        .receiver
        .as_ref()
        .unwrap()
        .check_prepared_target(&budget)
        .unwrap());
    prepared
        .models
        .transmitter
        .as_ref()
        .unwrap()
        .check_prepared_target(&budget)
        .unwrap();
    assert!(session::snapshot(&context).source.is_none());
    drop(prepared);
    assert_eq!(budget.usage(), before);
    let prepared = candidate(&context);
    prepared.adopt(&context).unwrap();
    assert!(session::snapshot(&context).source.is_some());
    assert_eq!(budget.usage(), complete);
    drop(context);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}

#[test]
fn stale_identity_foreign_identity_and_foreign_budget_preserve_target_inputs_and_models() {
    for rejection in 0..3 {
        let target = Context::isolated();
        let foreign = Context::isolated();
        session::update(&target, |_, sections| {
            sections.insert("source".into(), json!({"accepted":"preserve"}));
            Ok(())
        })
        .unwrap();
        // A populated pending editor proves rejection does not overwrite the transfer store.
        target.prepared.with(|models| {
            models.borrow_mut().source = Some(Box::new(
                crate::downstream::inputs::source_input::model::SourceModel::default()
                    .admit(target.application.budget())
                    .unwrap(),
            ));
        });
        let original = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let original_bytes = std::fs::read(&original).unwrap();
        crate::platform::input_file::read_local(
            &target.application,
            Direction::Downstream,
            &format!("file://{}", original.display()),
            original_bytes.len(),
        )
        .unwrap();
        assert!(crate::platform::input_file::is_original(
            &target.application,
            &original
        ));
        target
            .application
            .register_input_path(Direction::Upstream, original.canonicalize().unwrap())
            .unwrap();
        let key = target.application.snapshot(Direction::Downstream).key;
        let (application, identity) = match rejection {
            0 => (&target.application, key),
            1 => (
                &target.application,
                foreign.application.snapshot(Direction::Downstream).key,
            ),
            _ => (&foreign.application, key),
        };
        let baseline = target.application.budget().usage();
        let prepared = PreparedProject::prepare(
            serde_json::from_slice(PROJECT).unwrap(),
            None,
            application,
            identity,
            session::snapshot(&target),
        )
        .unwrap();
        if rejection == 0 {
            target.application.configure_downstream(false).unwrap();
        }
        let before = session::saved_sections(&target);
        assert!(prepared.adopt(&target).is_err());
        assert!(crate::platform::input_file::is_original(
            &target.application,
            &original
        ));
        assert_eq!(std::fs::read(&original).unwrap(), original_bytes);
        assert_eq!(&*session::saved_sections(&target), &*before);
        assert!(target.prepared.borrow().source.is_some());
        assert_eq!(target.application.budget().usage(), baseline);
        target.application.clear_input_paths(Direction::Downstream);
        assert!(crate::platform::input_file::is_original(
            &target.application,
            &original
        ));
        target.application.clear_input_paths(Direction::Upstream);
        assert!(!crate::platform::input_file::is_original(
            &target.application,
            &original
        ));
    }
}

#[test]
fn real_background_loader_rejection_preserves_current_inputs_original_paths_and_releases_candidate()
{
    // Accounting-only pressure fills the actual remaining byte policy after
    // worker admission; the actual project candidate still reaches its old gate.
    let context = Context::for_application(
        crate::shell::ApplicationSessions::new(RetainedResources {
            payload_bytes: usize::MAX,
            ordered_entries: usize::MAX,
            shared_roots: usize::MAX,
            channel_count: usize::MAX,
            slot_count: usize::MAX,
        })
        .unwrap(),
    )
    .unwrap();
    session::update(&context, |_, sections| {
        sections.insert("source".into(), json!({"accepted":"preserve"}));
        Ok(())
    })
    .unwrap();
    let original = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    crate::platform::input_file::read_local(
        &context.application,
        Direction::Downstream,
        &format!("file://{}", original.display()),
        std::fs::metadata(&original)
            .unwrap()
            .len()
            .try_into()
            .unwrap(),
    )
    .unwrap();
    let budget = context.application.budget();
    let before = budget.usage();
    let mut loader = Loader::default();
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/workbench/single/workbench.merlin.json");
    let application = context.application.clone();
    let target = application.snapshot(Direction::Downstream).key;
    let expected = session::snapshot(&context);
    let worker_budget = budget.clone();
    let pressure = Arc::new(std::sync::Mutex::new(None));
    let worker_pressure = Arc::clone(&pressure);
    let started = loader.start_with(budget, move |_, cancelled| {
        let mut reader = std::fs::File::open(&path).map_err(|error| error.to_string())?;
        let bytes = read_bounded(&mut reader, cancelled, MAX_PROJECT_BYTES)?.unwrap();
        let project = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
        drop(bytes);

        // Pressure begins after actual transport admission, preserving this
        // test's original background candidate-rejection boundary.
        let used = worker_budget.usage();
        *worker_pressure.lock().unwrap() = Some(
            worker_budget
                .reserve_resources(RetainedResources {
                    payload_bytes: usize::MAX - used.payload_bytes,
                    ..Default::default()
                })
                .unwrap(),
        );
        PreparedProject::prepare(project, Some(path), &application, target, expected).map(Some)
    });
    assert_eq!(started["loading"], true);
    let (status, prepared) = super::tests::drain(&mut loader);
    assert!(status["error"]
        .as_str()
        .unwrap()
        .contains("retained-data limit"));
    assert!(prepared.is_none());
    assert_eq!(session::saved(&context, "source")["accepted"], "preserve");
    assert!(crate::platform::input_file::is_original(
        &context.application,
        &original
    ));
    assert!(
        pressure.lock().unwrap().is_some(),
        "The real worker reached candidate pressure"
    );
    drop(pressure.lock().unwrap().take());
    assert_eq!(budget.usage(), before);
}

#[test]
fn intervening_accepted_edit_rejects_open_even_before_the_canonical_generation_advances() {
    let target = Context::isolated();
    session::update(&target, |_, sections| {
        sections.insert("source".into(), json!({"accepted":"original"}));
        Ok(())
    })
    .unwrap();
    let prepared = candidate(&target);
    let key = target.application.snapshot(Direction::Downstream).key;
    session::update(&target, |_, sections| {
        sections.insert("source".into(), json!({"accepted":"intervening"}));
        Ok(())
    })
    .unwrap();
    assert_eq!(target.application.snapshot(Direction::Downstream).key, key);
    assert!(prepared
        .adopt(&target)
        .unwrap_err()
        .contains("input state is stale"));
    assert_eq!(session::saved(&target, "source")["accepted"], "intervening");
    assert!(take(&target, |models| models.source.take()).is_none());
}

#[test]
fn successful_open_releases_only_downstream_path_records_and_preserves_original_files() {
    let context = Context::isolated();
    let application = &context.application;
    let downstream_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let shared_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/workbench/single/workbench.merlin.json");
    let downstream_bytes = std::fs::read(&downstream_path).unwrap();
    let shared_bytes = std::fs::read(&shared_path).unwrap();
    application
        .register_input_path(
            Direction::Downstream,
            downstream_path.canonicalize().unwrap(),
        )
        .unwrap();
    application
        .register_input_path(Direction::Downstream, shared_path.canonicalize().unwrap())
        .unwrap();
    application
        .register_input_path(Direction::Upstream, shared_path.canonicalize().unwrap())
        .unwrap();
    let upstream = application.snapshot(Direction::Upstream);
    candidate(&context).adopt(&context).unwrap();
    assert!(!crate::platform::input_file::is_original(
        application,
        &downstream_path
    ));
    assert!(crate::platform::input_file::is_original(
        application,
        &shared_path
    ));
    assert_eq!(application.snapshot(Direction::Upstream), upstream);
    assert_eq!(std::fs::read(&downstream_path).unwrap(), downstream_bytes);
    assert_eq!(std::fs::read(&shared_path).unwrap(), shared_bytes);
    assert!(session::snapshot(&context).source.is_some());
    application.clear_input_paths(Direction::Upstream);
    assert!(!crate::platform::input_file::is_original(
        application,
        &shared_path
    ));
}
