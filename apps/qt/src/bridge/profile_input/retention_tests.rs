/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual profile editor replacement, coupled Apply and queued guard transfer.
use super::*;
use crate::downstream::workbench::session;
use crate::shell::{ApplicationSessions, RetainedResources};
use merlin::downstream::TransmissionSettings;

const ORIGINAL: &[u8] =
    include_bytes!("../../../../../examples/measurements/single-channel/modulation_profile.bin");
const TEST_MAXIMUM: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};

fn new_owner() -> ProfileInputRust {
    let mut context = Context::isolated();
    context.application = ApplicationSessions::new(TEST_MAXIMUM).unwrap();
    ProfileInputRust {
        context,
        ..Default::default()
    }
}
fn draft() -> ProfileModel {
    let mut model = ProfileModel::default();
    model
        .stage_bytes("original.bin".into(), ORIGINAL.to_vec())
        .unwrap();
    model
}
fn pressure(used: RetainedResources, dimension: usize) -> RetainedResources {
    // Independent saturation of actual byte, ordered-entry or shared-root usage.
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
fn profile_editor_candidate_and_apply_rejection_preserve_old_model_inputs_and_guard() {
    let mut owner = new_owner();
    let budget = owner.context.application.budget().clone();
    owner.replace_model(draft()).unwrap();
    let settings = json!(TransmissionSettings::default()).to_string();
    let ids = owner
        .model
        .draft
        .as_ref()
        .unwrap()
        .catalog
        .profile_ids()
        .to_vec();
    owner.apply_profile(i32::from(ids[0]), &settings).unwrap();
    let saved = session::saved(&owner.context, "profile");
    let usage = budget.usage();
    assert!(owner.model_guard.is_some());
    // All three actual profile owner dimensions are positive.
    for dimension in 0..3 {
        let guard = budget
            .reserve_resources(pressure(usage, dimension))
            .unwrap();
        let mut candidate = owner.model.clone();
        candidate.draft.as_mut().unwrap().name = "replacement.bin".into();
        assert!(owner.replace_model(candidate).is_err());
        assert!(owner.apply_profile(i32::from(ids[1]), &settings).is_err());
        assert_eq!(owner.model.draft.as_ref().unwrap().name, "original.bin");
        assert_eq!(owner.model.accepted.as_ref().unwrap().1, ids[0]);
        assert_eq!(&*session::saved(&owner.context, "profile"), &*saved);
        assert_eq!(
            session::snapshot(&owner.context)
                .profile
                .as_ref()
                .unwrap()
                .profile_id,
            ids[0]
        );
        drop(guard);
        assert_eq!(budget.usage(), usage);
    }
    drop(saved);
    drop(owner);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn profile_queue_transfer_needs_no_new_reservation_and_foreign_transfer_preserves_target() {
    let mut owner = new_owner();
    let foreign = new_owner();
    let budget = owner.context.application.budget().clone();
    let rejected = draft().admit(foreign.context.application.budget()).unwrap();
    let before = budget.usage();
    assert!(owner.adopt_model(rejected).is_err());
    assert_eq!(budget.usage(), before);
    assert!(owner.model.draft.is_none());
    let prepared = draft().admit(&budget).unwrap();
    let used = budget.usage();
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - used.payload_bytes,
            ordered_entries: usize::MAX - used.ordered_entries,
            shared_roots: usize::MAX - used.shared_roots,
        })
        .unwrap();
    owner.adopt_model(prepared).unwrap();
    assert_eq!(budget.usage(), TEST_MAXIMUM);
    assert_eq!(
        owner.model.draft.as_ref().unwrap().original.bytes(),
        ORIGINAL
    );
    drop(pressure);
    assert_eq!(budget.usage(), used);
    drop(owner);
    assert_eq!(budget.usage(), RetainedResources::default());
}
