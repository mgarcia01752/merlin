/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Concrete accepted owner arithmetic, rollback and final-alias conservation.
use super::*;
use crate::downstream::workbench::session::{self, AcceptedSource};
use crate::shell::ApplicationSessions;
use merlin::downstream::PrbsSeed;
use serde_json::json;

// Independent test pressure policy; never a desktop or scientific limit.
const TEST_MAXIMUM: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
    presentation_utf16_bytes: 0,
    presentation_entries: 0,
    retained_views: 0,
    open_views: 0,
};
// Fixture spare capacity and two distinct valid PRBS identities.
const SPARE_BYTES: usize = 29;
const SPARE_VALUES: usize = 11;
const OLD_SEED: u16 = 17;
const NEW_SEED: u16 = 31;

fn context() -> Context {
    Context::for_application(ApplicationSessions::new(TEST_MAXIMUM).unwrap()).unwrap()
}
fn accept(context: &Context, seed: u16) -> Result<(), String> {
    session::update(context, |inputs, sections| {
        inputs.source = Some(AcceptedSource {
            config: SourceInput::prbs15(PrbsSeed::new(seed).unwrap()),
            seed,
        });
        sections.insert("source".into(), json!({"seed":seed}));
        Ok(())
    })
}

#[test]
fn actual_empty_root_and_json_spare_storage_match_independent_layout() {
    let inputs = Inputs::default();
    let mut sections = Sections::new();
    assert_eq!(
        resources(&inputs, &sections).unwrap(),
        RetainedResources {
            payload_bytes: size_of::<Retained<State>>(),
            ordered_entries: 0,
            shared_roots: 1,
            ..Default::default()
        }
    );
    let mut key = String::with_capacity(SPARE_BYTES);
    key.push_str("source");
    let mut label = String::with_capacity(SPARE_BYTES);
    label.push_str("original");
    let mut values = Vec::with_capacity(SPARE_VALUES);
    let expected = size_of::<Retained<State>>()
        + size_of::<(String, Value)>()
        + key.capacity()
        + values.capacity() * size_of::<Value>()
        + label.capacity();
    values.push(Value::String(label));
    sections.insert(key, Value::Array(values));
    assert_eq!(
        resources(&inputs, &sections).unwrap(),
        RetainedResources {
            payload_bytes: expected,
            ordered_entries: 1,
            shared_roots: 1,
            ..Default::default()
        }
    );
    assert_eq!(sections["source"], json!(["original"]));
}

#[test]
fn each_rejected_candidate_dimension_preserves_matching_inputs_documents_and_aliases() {
    for dimension in 0..3 {
        let context = context();
        accept(&context, OLD_SEED).unwrap();
        let original = snapshot(&context);
        let owner = original.owner.as_ref().unwrap();
        let before = context.application.budget().usage();
        let mut pressure = RetainedResources::default();
        match dimension {
            0 => pressure.payload_bytes = usize::MAX - before.payload_bytes,
            1 => pressure.ordered_entries = usize::MAX - before.ordered_entries,
            _ => pressure.shared_roots = usize::MAX - before.shared_roots,
        }
        let guard = context
            .application
            .budget()
            .reserve_resources(pressure)
            .unwrap();
        let pressured = context.application.budget().usage();
        assert!(accept(&context, NEW_SEED).is_err());
        // New uses this same fallible restore before clearing its native paths.
        if dimension != 1 {
            assert!(session::restore(&context, Default::default(), Default::default()).is_err());
        }
        let current = snapshot(&context);
        assert!(Arc::ptr_eq(current.owner.as_ref().unwrap(), owner));
        assert_eq!(current.source.as_ref().unwrap().seed, OLD_SEED);
        assert_eq!(current.sections()["source"]["seed"], OLD_SEED);
        assert_eq!(context.application.budget().usage(), pressured);
        drop(guard);
        assert_eq!(context.application.budget().usage(), before);
        // A failed candidate closure is equally transactional, before admission.
        let result = session::update(&context, |inputs, sections| {
            inputs.source = None;
            sections.clear();
            Err("Rejected candidate".into())
        });
        assert!(result.is_err());
        assert!(Arc::ptr_eq(
            snapshot(&context).owner.as_ref().unwrap(),
            owner
        ));
        assert_eq!(context.application.budget().usage(), before);
        accept(&context, NEW_SEED).unwrap();
        assert_eq!(snapshot(&context).source.as_ref().unwrap().seed, NEW_SEED);
        assert_eq!(original.source.as_ref().unwrap().seed, OLD_SEED);
        assert_eq!(original.sections()["source"]["seed"], OLD_SEED);
    }
}

#[test]
fn read_handles_and_old_sections_keep_exact_original_charges_until_the_final_drop() {
    let context = context();
    let budget = context.application.budget().clone();
    accept(&context, OLD_SEED).unwrap();
    let old = snapshot(&context);
    let old_sections = sections(&context);
    let old_section = section(&context, "source");
    let original_charge = old.owner.as_ref().unwrap().fixture_reserved_resources();
    let before_reads = budget.usage();
    let clone = old.clone();
    assert_eq!(budget.usage(), before_reads);
    accept(&context, NEW_SEED).unwrap();
    let new = snapshot(&context);
    assert!(!Arc::ptr_eq(
        old.owner.as_ref().unwrap(),
        new.owner.as_ref().unwrap()
    ));
    let mut both = original_charge;
    storage::append(
        &mut both,
        new.owner.as_ref().unwrap().fixture_reserved_resources(),
    )
    .unwrap();
    drop(context);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
            .checked_add(both)
            .unwrap()
    );
    assert_eq!(old_section["seed"], OLD_SEED);
    assert_eq!(old_sections["source"]["seed"], OLD_SEED);
    drop(new);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
            .checked_add(original_charge)
            .unwrap()
    );
    drop(old);
    drop(old_section);
    drop(old_sections);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
            .checked_add(original_charge)
            .unwrap()
    );
    drop(clone);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}

#[test]
fn foreign_host_adoption_rejects_without_replacing_accepted_state() {
    let source = context();
    let target = context();
    accept(&source, NEW_SEED).unwrap();
    accept(&target, OLD_SEED).unwrap();
    let owner = snapshot(&target);
    let before = target.application.budget().usage();
    let candidate = snapshot(&source).owner.unwrap();
    assert!(session::adopt_accepted(&target, candidate, &owner).is_err());
    assert!(Arc::ptr_eq(
        snapshot(&target).owner.as_ref().unwrap(),
        owner.owner.as_ref().unwrap()
    ));
    assert_eq!(target.application.budget().usage(), before);
    assert_eq!(snapshot(&target).source.as_ref().unwrap().seed, OLD_SEED);
}
