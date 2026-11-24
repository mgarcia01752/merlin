/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! All-target project preflight, final readiness rollback and reservation-free attachment.
use super::*;
use crate::downstream::workbench::session;
use crate::shell::{ApplicationSessions, Direction, RetainedResources};
use std::time::Duration;

const PROJECT: &[u8] =
    include_bytes!("../../../../../../examples/workbench/single/workbench.merlin.json");
// A small real observation is independent of host scheduling/counter policies.
const OBSERVATION_MS: u64 = 1;
const HOST_WATCHDOG: Duration = Duration::from_secs(30);
const HOST_POLL_INTERVAL: Duration = Duration::from_millis(1);
const TEST_MAXIMUM: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};
const TEST_TRANSFER_CAPACITY: usize = 1;

fn context() -> Context {
    Context::for_application(ApplicationSessions::new(TEST_MAXIMUM).unwrap()).unwrap()
}
fn project() -> Project {
    let mut value: Value = serde_json::from_slice(PROJECT).unwrap();
    value["receiver"]["duration_ms"] = json!(OBSERVATION_MS);
    serde_json::from_value(value).unwrap()
}
fn candidate(context: &Context) -> Result<PreparedProject, String> {
    PreparedProject::prepare(
        project(),
        None,
        &context.application,
        context.application.snapshot(Direction::Downstream).key,
        session::snapshot(context),
    )
}
fn pressure(used: RetainedResources, dimension: usize) -> RetainedResources {
    // Independent accounting-only pressure; no payload allocation or science-size estimate.
    match dimension {
        0 => RetainedResources {
            payload_bytes: usize::MAX - used.payload_bytes,
            ..Default::default()
        },
        1 => RetainedResources {
            ordered_entries: usize::MAX - used.ordered_entries,
            ..Default::default()
        },
        _ => RetainedResources {
            shared_roots: usize::MAX - used.shared_roots,
            ..Default::default()
        },
    }
}

#[test]
fn every_target_dimension_rejects_before_project_key_inputs_or_pending_models_change() {
    for dimension in 0..3 {
        // Payload, ordered entries and shared roots each have positive complete-project owners.
        let context = context();
        session::update(&context, |_, sections| {
            sections.insert("source".into(), json!({"accepted":"preserve"}));
            Ok(())
        })
        .unwrap();
        context.prepared.borrow_mut().source = Some(Box::new(
            crate::downstream::inputs::source_input::model::SourceModel::default()
                .admit(context.application.budget())
                .unwrap(),
        ));
        let key = context.application.snapshot(Direction::Downstream);
        let usage = context.application.budget().usage();
        let guard = context
            .application
            .budget()
            .reserve_resources(pressure(usage, dimension))
            .unwrap();
        let before = context.application.budget().usage();
        assert!(candidate(&context).is_err());
        assert_eq!(context.application.snapshot(Direction::Downstream), key);
        assert_eq!(session::saved(&context, "source")["accepted"], "preserve");
        assert!(context.prepared.borrow().source.is_some());
        assert_eq!(context.application.budget().usage(), before);
        drop(guard);
        assert_eq!(context.application.budget().usage(), usage);
    }
}

#[test]
fn prepared_receiver_and_preview_attach_under_full_counter_pressure_without_another_generation() {
    let context = context();
    let budget = context.application.budget().clone();
    candidate(&context).unwrap().adopt(&context).unwrap();
    let accepted = context.application.snapshot(Direction::Downstream);
    let usage = budget.usage();
    let guard = budget
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - usage.payload_bytes,
            ordered_entries: usize::MAX - usage.ordered_entries,
            shared_roots: usize::MAX - usage.shared_roots,
        })
        .unwrap();
    let before = budget.usage();
    let mut receiver = take(&context, |models| models.receiver.take()).unwrap();
    let mut preview = take(&context, |models| models.transmitter.take()).unwrap();
    receiver.attach_prepared(context.clone()).unwrap();
    preview.rebind_prepared(context.clone()).unwrap();
    assert_eq!(
        receiver.context.application.snapshot(Direction::Downstream),
        accepted
    );
    assert_eq!(
        context.application.snapshot(Direction::Downstream),
        accepted
    );
    assert_eq!(budget.usage(), before);
    drop(guard);
    assert_eq!(budget.usage(), usage);
    drop(receiver);
    drop(preview);
    drop(context);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn late_readiness_metadata_rejection_keeps_the_real_old_report_inputs_key_and_pending_models() {
    let context = context();
    // This rollback fixture needs one real old report, not the sample's legacy
    // all-profile sweep. Keep that producer compatibility covered independently.
    let mut old_project = project();
    old_project.receiver.as_mut().unwrap().all_profiles = false;
    PreparedProject::prepare(
        old_project,
        None,
        &context.application,
        context.application.snapshot(Direction::Downstream).key,
        session::snapshot(&context),
    )
    .unwrap()
    .adopt(&context)
    .unwrap();
    let mut model = take(&context, |models| models.receiver.take()).unwrap();
    model.attach_prepared(context.clone()).unwrap();
    model.start();
    let deadline = Instant::now() + HOST_WATCHDOG;
    while model.poll()["busy"] != false {
        assert!(
            Instant::now() < deadline,
            "Real old report did not complete: {}",
            model.state()
        );
        std::thread::sleep(HOST_POLL_INTERVAL);
    }
    let report = crate::downstream::simulation::closed_loop::latest(&context).unwrap();
    assert_eq!(
        report.job.status,
        merlin::downstream::ChannelJobStatus::Complete
    );
    let key = context.application.snapshot(Direction::Downstream);
    let sections = session::saved_sections(&context);
    let baseline = context.application.budget().usage();
    let pending = candidate(&context).unwrap();
    let usage = context.application.budget().usage();
    let guard = context
        .application
        .budget()
        .reserve_resources(pressure(usage, 0))
        .unwrap();
    assert!(pending
        .adopt(&context)
        .unwrap_err()
        .contains("retained-data limit"));
    assert_eq!(context.application.snapshot(Direction::Downstream), key);
    assert_eq!(&*session::saved_sections(&context), &*sections);
    assert!(Arc::ptr_eq(
        &crate::downstream::simulation::closed_loop::latest(&context).unwrap(),
        &report
    ));
    assert_eq!(
        model.context.application.snapshot(Direction::Downstream),
        key
    );
    assert!(context.prepared.borrow().transmitter.is_some());
    drop(guard);
    assert_eq!(context.application.budget().usage(), baseline);
}

#[test]
fn fully_prepared_cancelled_load_releases_all_target_owners_only_when_its_worker_is_drained() {
    let context = context();
    let budget = context.application.budget().clone();
    let application = context.application.clone();
    let key = context.application.snapshot(Direction::Downstream).key;
    let expected = session::snapshot(&context);
    let baseline = budget.usage();
    let (ready, wait_ready) = mpsc::sync_channel(TEST_TRANSFER_CAPACITY);
    let (release, wait_release) = mpsc::sync_channel(TEST_TRANSFER_CAPACITY);
    let mut loader = Loader::default();
    loader.start_with(move |_, _| {
        let prepared = PreparedProject::prepare(project(), None, &application, key, expected)?;
        ready.send(()).unwrap();
        wait_release.recv_timeout(HOST_WATCHDOG).unwrap();
        Ok(Some(prepared))
    });
    wait_ready.recv_timeout(HOST_WATCHDOG).unwrap();
    let queued = budget.usage();
    assert!(queued.payload_bytes > baseline.payload_bytes);
    loader.cancel();
    assert_eq!(budget.usage(), queued);
    release.send(()).unwrap();
    let (status, project) = super::tests::drain(&mut loader);
    assert_eq!(status["cancelled"], true);
    assert!(project.is_none());
    assert_eq!(budget.usage(), baseline);
    assert_eq!(context.application.snapshot(Direction::Downstream).key, key);
    assert!(session::saved_sections(&context).is_empty());
}

#[test]
fn successful_edit_and_new_discard_old_prepared_models_but_rejected_edit_keeps_them() {
    let context = context();
    candidate(&context).unwrap().adopt(&context).unwrap();
    let usage = context.application.budget().usage();
    let guard = context
        .application
        .budget()
        .reserve_resources(pressure(usage, 0))
        .unwrap();
    assert!(session::update(&context, |_, sections| {
        sections.insert("channel_model".into(), Value::Null);
        Ok(())
    })
    .is_err());
    assert!(context.prepared.borrow().receiver.is_some());
    drop(guard);
    session::update(&context, |_, sections| {
        sections.insert("channel_model".into(), Value::Null);
        Ok(())
    })
    .unwrap();
    assert!(context.prepared.borrow().receiver.is_none());
    assert!(context.prepared.borrow().transmitter.is_none());
    candidate(&context).unwrap().adopt(&context).unwrap();
    session::restore(&context, Default::default(), Default::default()).unwrap();
    assert!(context.prepared.borrow().source.is_none());
    assert!(context.prepared.borrow().receiver.is_none());
    assert!(session::saved_sections(&context).is_empty());
}
