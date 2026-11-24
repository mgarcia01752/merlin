/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual source-editor candidates and coupled accepted-state admission.
use super::*;
use crate::downstream::workbench::session::{self, Context};
use crate::shell::ApplicationSessions;

// Explicit fixture ceilings support saturation from actual usage, not size estimates.
const FIXTURE_LIMIT: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};
// Payload bytes, ordered entries and shared roots are three independent quantities.
const RETENTION_DIMENSION_COUNT: usize = 3;
// Independent exact-byte source anchors preserve null and newline contents.
const ACCEPTED_BYTES: &[u8] = b"\0accepted\r\n";
const DRAFT_BYTES: &[u8] = b"\0draft\n";
const REPLACEMENT_BYTES: &[u8] = b"\0replacement\r\n";

fn context() -> Context {
    let mut context = Context::isolated();
    context.application = ApplicationSessions::new(FIXTURE_LIMIT).unwrap();
    context
}

fn options() -> String {
    serde_json::to_string(&Options {
        mode: "file".into(),
        seed: INITIAL_CONTINUOUS_SOURCE_SEED,
        append: false,
        target_bits: 0,
        fill_seed: INITIAL_FILE_FILL_SEED,
    })
    .unwrap()
}

fn source() -> SourceModel {
    let mut model = SourceModel::default();
    model
        .stage("accepted.bin".into(), ACCEPTED_BYTES.to_vec())
        .unwrap();
    model.accept(&options()).unwrap();
    model
}

fn accepted(context: &Context) -> (SourceModel, RetentionReservation) {
    let mut model = source();
    let reservation = model.apply_transactional(context, &options()).unwrap();
    (model, reservation)
}

fn plus(left: RetainedResources, right: RetainedResources) -> RetainedResources {
    RetainedResources {
        payload_bytes: left.payload_bytes.checked_add(right.payload_bytes).unwrap(),
        ordered_entries: left
            .ordered_entries
            .checked_add(right.ordered_entries)
            .unwrap(),
        shared_roots: left.shared_roots.checked_add(right.shared_roots).unwrap(),
    }
}

fn pressures(
    actual: RetainedResources,
    usage: RetainedResources,
) -> [(usize, RetainedResources); RETENTION_DIMENSION_COUNT] {
    [
        (
            actual.payload_bytes,
            RetainedResources {
                payload_bytes: usize::MAX - usage.payload_bytes,
                ..Default::default()
            },
        ),
        (
            actual.ordered_entries,
            RetainedResources {
                ordered_entries: usize::MAX - usage.ordered_entries,
                ..Default::default()
            },
        ),
        (
            actual.shared_roots,
            RetainedResources {
                shared_roots: usize::MAX - usage.shared_roots,
                ..Default::default()
            },
        ),
    ]
}

#[test]
fn admitted_candidate_overlap_and_parts_transfer_keep_both_actual_owners_charged() {
    let budget = RetentionBudget::with_limits(FIXTURE_LIMIT);
    let old = source().admit(&budget).unwrap();
    let old_charge = old.fixture_reserved_resources();
    let candidate = old
        .value()
        .stage_admitted(
            &budget,
            "replacement.bin".into(),
            REPLACEMENT_BYTES.to_vec(),
        )
        .unwrap();
    let candidate_charge = candidate.fixture_reserved_resources();
    assert_eq!(budget.usage(), plus(old_charge, candidate_charge));
    assert_eq!(old.value().file_summary()["name"], "accepted.bin");
    let (model, reservation) = candidate.into_parts();
    assert_eq!(budget.usage(), plus(old_charge, candidate_charge));
    assert_eq!(model.file_summary()["name"], "replacement.bin");
    assert_eq!(
        model.snapshot().unwrap().config.original_bytes(),
        Some(ACCEPTED_BYTES)
    );
    drop(old);
    assert_eq!(budget.usage(), candidate_charge);
    drop(model);
    assert_eq!(budget.usage(), candidate_charge);
    drop(reservation);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn begin_stage_and_fallback_reload_rejection_preserve_old_editor_and_guard() {
    let context = context();
    let (mut model, mut reservation) = accepted(&context);
    let staged = model
        .stage_admitted(
            context.application.budget(),
            "draft.bin".into(),
            DRAFT_BYTES.to_vec(),
        )
        .unwrap();
    let (candidate, next_reservation) = staged.into_parts();
    model = candidate;
    drop(std::mem::replace(&mut reservation, next_reservation));
    let budget = context.application.budget();
    let before = budget.usage();
    let editor = model.export();
    let draft = model.file_summary();
    let saved = session::saved(&context, "source");
    let actual = model.retained_resources().unwrap();
    for (population, pressure) in pressures(actual, before) {
        if population == 0 {
            continue;
        }
        // Only actual positive model dimensions are pressured; source files need no trees.
        let guard = budget.reserve_resources(pressure).unwrap();
        let full = budget.usage();
        assert!(model.begin_admitted(budget).is_err());
        assert!(model
            .stage_admitted(budget, "replacement.bin".into(), REPLACEMENT_BYTES.to_vec())
            .is_err());
        assert!(SourceModel::from_saved(&saved)
            .and_then(|model| model.admit(budget))
            .is_err());
        assert_eq!(model.file_summary(), draft);
        assert_eq!(model.export(), editor);
        assert_eq!(&*session::saved(&context, "source"), &*saved);
        assert_eq!(budget.usage(), full);
        drop(guard);
        assert_eq!(budget.usage(), before);
    }
    drop(model);
    drop(reservation);
}

#[test]
fn apply_rejection_in_all_dimensions_preserves_model_source_and_saved_section() {
    for dimension in 0..RETENTION_DIMENSION_COUNT {
        // Payload, ordered entries and shared roots are independent host quantities.
        let context = context();
        let (mut model, mut reservation) = accepted(&context);
        let staged = model
            .stage_admitted(
                context.application.budget(),
                "draft.bin".into(),
                DRAFT_BYTES.to_vec(),
            )
            .unwrap();
        let (candidate, next_reservation) = staged.into_parts();
        model = candidate;
        drop(std::mem::replace(&mut reservation, next_reservation));
        let before_editor = model.export();
        let before_draft = model.file_summary();
        let saved = session::saved(&context, "source");
        let inputs = session::snapshot(&context);
        let budget = context.application.budget();
        let before = budget.usage();
        // This actual file model owns no ordered entries. Entry-only pressure
        // therefore exercises rollback after model admission at session::update.
        assert_eq!(model.retained_resources().unwrap().ordered_entries, 0);
        let guard = budget
            .reserve_resources(pressures(before, before)[dimension].1)
            .unwrap();
        let full = budget.usage();
        assert!(model.apply_transactional(&context, &options()).is_err());
        assert_eq!(model.export(), before_editor);
        assert_eq!(model.file_summary(), before_draft);
        assert_eq!(&*session::saved(&context, "source"), &*saved);
        assert_eq!(
            session::snapshot(&context)
                .source
                .as_ref()
                .unwrap()
                .config
                .original_bytes(),
            inputs.source.as_ref().unwrap().config.original_bytes()
        );
        assert_eq!(budget.usage(), full);
        drop(guard);
        assert_eq!(budget.usage(), before);
        drop(model);
        drop(reservation);
    }
}

#[test]
fn successful_apply_keeps_model_and_accepted_originals_until_each_final_owner_drops() {
    let context = context();
    let budget = context.application.budget().clone();
    let (mut model, mut reservation) = accepted(&context);
    let staged = model
        .stage_admitted(
            &budget,
            "replacement.bin".into(),
            REPLACEMENT_BYTES.to_vec(),
        )
        .unwrap();
    let (candidate, next_reservation) = staged.into_parts();
    model = candidate;
    drop(std::mem::replace(&mut reservation, next_reservation));
    let next_reservation = model.apply_transactional(&context, &options()).unwrap();
    drop(std::mem::replace(&mut reservation, next_reservation));
    let inputs = session::snapshot(&context);
    assert_eq!(
        model.snapshot().unwrap().config.original_bytes(),
        Some(REPLACEMENT_BYTES)
    );
    assert_eq!(
        inputs.source.as_ref().unwrap().config.original_bytes(),
        Some(REPLACEMENT_BYTES)
    );
    assert_eq!(&*session::saved(&context, "source"), &model.export());
    drop(context);
    drop(model);
    let with_model_charge = budget.usage();
    drop(reservation);
    assert!(budget.usage().payload_bytes < with_model_charge.payload_bytes);
    assert!(budget.usage().payload_bytes > 0);
    assert_eq!(
        inputs.source.as_ref().unwrap().config.original_bytes(),
        Some(REPLACEMENT_BYTES)
    );
    drop(inputs);
    assert_eq!(budget.usage(), RetainedResources::default());
}
