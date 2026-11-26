/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Real RX/H Apply transactions and exact profile-presence refresh dependency.
use super::*;
use crate::downstream::inputs::profile_input::ProfileModel;
use crate::downstream::workbench::session::{self, Context};
use crate::shell::{ApplicationSessions, RetainedResources};
const PROFILE: &[u8] = include_bytes!(
    "../../../../../../../examples/measurements/single-channel/modulation_profile.bin"
);
fn prepared() -> (Context, ProfileModel, Model) {
    let context = Context::for_application(
        ApplicationSessions::new(RetainedResources {
            payload_bytes: usize::MAX,
            ordered_entries: usize::MAX,
            shared_roots: usize::MAX,
            channel_count: usize::MAX,
            slot_count: usize::MAX,
            presentation_utf16_bytes: 0,
            presentation_entries: 0,
            retained_views: 0,
            open_views: 0,
        })
        .unwrap(),
    )
    .unwrap();
    let mut profile = ProfileModel {
        context: context.clone(),
        ..Default::default()
    };
    profile
        .stage_bytes("original.bin".into(), PROFILE.to_vec())
        .unwrap();
    profile
        .apply_transactional(0, &json!(TransmissionSettings::default()).to_string())
        .unwrap(); // Actual supported source Profile A fixture.
    let mut model = Model::default();
    let rx = Capture::decode(
        "rxmer.bin".into(),
        include_bytes!(
            "../../../../../../../examples/measurements/rxmer-fec-stress/rxmer-baseline.bin"
        )
        .to_vec(),
        true,
    )
    .unwrap();
    model.stage(vec![rx], true).unwrap();
    (context, profile, model)
}
fn response() -> Capture {
    Capture::decode(
        "response.bin".into(),
        include_bytes!(
            "../../../../../../../examples/workbench/downstream/single/ds-channel-estimate.bin"
        )
        .to_vec(),
        false,
    )
    .unwrap()
}
fn pressure(used: RetainedResources, dimension: usize) -> RetainedResources {
    // Synthetic accounting-only pressure uses actual current usage in each dimension.
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
fn measurement_apply_couples_inputs_and_section_then_refreshes_only_actual_presence_mismatches() {
    let (context, mut profile, mut model) = prepared();
    let settings = TransmissionSettings::default();
    model
        .apply_transactional(&context, true, Options::default(), &settings)
        .unwrap();
    model.stage(vec![response()], false).unwrap();
    let options = Options::default();
    let tolerance = options.tolerance_seconds;
    model
        .apply_transactional(&context, false, options, &settings)
        .unwrap();
    let inputs = session::snapshot(&context);
    assert!(inputs.playback.is_some() && inputs.response.is_some());
    assert_eq!(inputs.response_tolerance, tolerance);
    drop(inputs);
    assert_eq!(&*session::saved(&context, "measurements"), &model.export());
    let editor = model.export();
    let budget = context.application.budget();
    let before = budget.usage();
    let pressure = budget.reserve_resources(pressure(before, 0)).unwrap();
    model.refresh_transactional(&context).unwrap(); // Cannot reserve a replacement: the matching path must not publish/clone.
    drop(pressure);
    assert_eq!(budget.usage(), before);
    profile.draft = None;
    profile
        .apply_transactional(-1, &json!(settings).to_string())
        .unwrap();
    assert!(session::snapshot(&context).playback.is_some()); // Existing dependent value remains until refresh.
    model.refresh_transactional(&context).unwrap();
    assert!(session::snapshot(&context).playback.is_none());
    assert_eq!(model.export(), editor);
    assert_eq!(&*session::saved(&context, "measurements"), &editor);
    profile
        .stage_bytes("restored.bin".into(), PROFILE.to_vec())
        .unwrap();
    profile
        .apply_transactional(0, &json!(settings).to_string())
        .unwrap();
    model.refresh_transactional(&context).unwrap();
    assert!(session::snapshot(&context).playback.is_some());
    assert_eq!(model.export(), editor);
}
#[test]
fn measurement_host_rejection_preserves_accepted_rx_response_tolerance_section_and_inputs() {
    for dimension in 0..3 {
        // All positive accepted-state byte/entry/root dimensions.
        let (context, _profile, mut model) = prepared();
        let settings = TransmissionSettings::default();
        model
            .apply_transactional(&context, true, Options::default(), &settings)
            .unwrap();
        let editor = model.export();
        let old = session::snapshot(&context);
        let saved = session::saved(&context, "measurements");
        model.stage(vec![response()], false).unwrap();
        let budget = context.application.budget();
        let before = budget.usage();
        let guard = budget
            .reserve_resources(pressure(before, dimension))
            .unwrap();
        assert!(model
            .apply_transactional(&context, false, Options::default(), &settings)
            .is_err());
        assert_eq!(model.export(), editor);
        assert!(model.accepted_response.is_none());
        let current = session::snapshot(&context);
        assert!(current.response.is_none());
        assert_eq!(current.response_tolerance, old.response_tolerance);
        assert_eq!(
            serde_json::to_value(&current.playback.as_ref().unwrap().frames).unwrap(),
            serde_json::to_value(&old.playback.as_ref().unwrap().frames).unwrap()
        );
        assert_eq!(&*session::saved(&context, "measurements"), &*saved);
        drop(current);
        drop(guard);
        assert_eq!(budget.usage(), before);
    }
}
