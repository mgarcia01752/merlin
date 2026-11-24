/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Exact selected/catalog/all-profile transactions and original saved bytes.
use super::*;
use crate::downstream::workbench::session::{self, Context};
use crate::shell::{ApplicationSessions, RetainedResources};
const ORIGINAL: &[u8] =
    include_bytes!("../../../../../../examples/measurements/single-channel/modulation_profile.bin");
fn model() -> ProfileModel {
    let context = Context::for_application(
        ApplicationSessions::new(RetainedResources {
            payload_bytes: usize::MAX,
            ordered_entries: usize::MAX,
            shared_roots: usize::MAX,
            channel_count: usize::MAX,
            slot_count: usize::MAX,
        })
        .unwrap(),
    )
    .unwrap();
    let mut model = ProfileModel {
        context,
        ..Default::default()
    };
    model
        .stage_bytes("original.bin".into(), ORIGINAL.to_vec())
        .unwrap();
    model
}
fn pressure(used: RetainedResources, dimension: usize) -> RetainedResources {
    // Actual remaining host capacity, not a scientific allocation estimate.
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
fn profile_apply_couples_selected_complete_catalog_and_saved_removal() {
    let mut model = model();
    let settings = json!(TransmissionSettings::default()).to_string();
    let ids = model.draft.as_ref().unwrap().catalog.profile_ids().to_vec();
    let selected = ids[0];
    model
        .apply_transactional(i32::from(selected), &settings)
        .unwrap();
    let inputs = session::snapshot(&model.context);
    assert_eq!(inputs.profile.as_ref().unwrap().profile_id, selected);
    assert_eq!(
        inputs
            .profiles
            .iter()
            .map(|profile| profile.profile_id)
            .collect::<Vec<_>>(),
        ids
    );
    assert_eq!(
        inputs
            .profile_capture
            .as_ref()
            .unwrap()
            .provenance()
            .original_bytes(),
        ORIGINAL
    );
    let section = session::saved(&model.context, "profile");
    assert_eq!(section["profile_id"], selected);
    let (_, bytes) = crate::downstream::workbench::project_file::decode(&section["file"]).unwrap();
    assert_eq!(bytes, ORIGINAL);
    drop(inputs);
    drop(section);
    model.draft = None;
    model.apply_transactional(-1, &settings).unwrap();
    let inputs = session::snapshot(&model.context);
    assert!(
        inputs.profile.is_none() && inputs.profiles.is_empty() && inputs.profile_capture.is_none()
    );
    assert!(session::saved(&model.context, "profile").is_null());
    assert!(model.accepted.is_none());
}
#[test]
fn profile_host_rejection_preserves_all_accepted_fields_and_original_section_in_each_dimension() {
    for dimension in 0..3 {
        // Positive whole accepted-state payload/entry/root dimensions.
        let mut model = model();
        let settings = json!(TransmissionSettings::default()).to_string();
        let ids = model.draft.as_ref().unwrap().catalog.profile_ids().to_vec();
        let accepted = ids[0];
        let candidate = ids[1]; // Distinct original captured identities.
        model
            .apply_transactional(i32::from(accepted), &settings)
            .unwrap();
        let context = model.context.clone();
        let old = session::snapshot(&context);
        let saved = session::saved(&context, "profile");
        let budget = context.application.budget();
        let before = budget.usage();
        let guard = budget
            .reserve_resources(pressure(before, dimension))
            .unwrap();
        assert!(model
            .apply_transactional(i32::from(candidate), &settings)
            .is_err());
        assert_eq!(model.accepted.as_ref().unwrap().1, accepted);
        assert_eq!(
            session::snapshot(&context)
                .profile
                .as_ref()
                .unwrap()
                .profile_id,
            old.profile.as_ref().unwrap().profile_id
        );
        assert_eq!(
            session::snapshot(&context)
                .profile_capture
                .as_ref()
                .unwrap()
                .provenance()
                .original_bytes(),
            ORIGINAL
        );
        assert_eq!(
            session::snapshot(&context)
                .profiles
                .iter()
                .map(|profile| profile.profile_id)
                .collect::<Vec<_>>(),
            ids
        );
        assert_eq!(&*session::saved(&context, "profile"), &*saved);
        drop(guard);
        assert_eq!(budget.usage(), before);
    }
}
