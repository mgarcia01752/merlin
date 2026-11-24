/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Preview control layouts, one-shot lifecycle and unchanged actual TX output.
use super::*;
use std::mem::size_of;

const PROFILE: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../examples/measurements/single-channel/modulation_profile.json"
));

// Scalar progress anchors only; no new TX window or execution policy.
const COMPLETED_SYMBOLS: usize = 1;
const TOTAL_SYMBOLS: usize = 3;

// Small contiguous actual-I/Q comparison window, within existing query admission.
const SAMPLE_VIEW_COUNT: usize = 8;

fn expected() -> RetainedFootprint {
    RetainedFootprint {
        payload_bytes: size_of::<TransmitterPreviewControl>()
            + size_of::<Shared>()
            + size_of::<std::sync::atomic::AtomicBool>(),
        ordered_entries: 0,
        shared_roots: 2,
    }
}

fn assert_passive(control: &TransmitterPreviewControl) {
    let before = serde_json::to_value(control.snapshot()).unwrap();
    assert_eq!(control.retained_footprint().unwrap(), expected());
    assert_eq!(serde_json::to_value(control.snapshot()).unwrap(), before);
}

#[test]
fn preview_control_layout_and_aliases_count_the_common_atomic_once() {
    let control = TransmitterPreviewControl::new();
    let weak = Arc::downgrade(&control.0);
    let alias = control.clone();
    let stop = control.stop();
    assert!(Arc::ptr_eq(&control.0, &alias.0));
    assert_passive(&control);
    assert_eq!(control.snapshot().state, TransmitterPreviewState::Idle);
    drop(control);
    assert!(weak.upgrade().is_some());
    stop.cancel();
    assert_passive(&alias);
    assert!(alias.snapshot().cancellation_requested);
    drop(alias);
    assert!(weak.upgrade().is_none());
    assert!(stop.is_cancelled());
}

#[test]
fn progress_and_complete_failed_cancelled_states_are_passive_and_stay_one_shot() {
    for success in [false, true] {
        let control = TransmitterPreviewControl::new();
        assert_passive(&control);
        control.begin().unwrap();
        control.progress(COMPLETED_SYMBOLS, TOTAL_SYMBOLS);
        assert_passive(&control);
        assert_eq!(control.snapshot().completed_symbols, COMPLETED_SYMBOLS);
        assert_eq!(control.snapshot().total_symbols, TOTAL_SYMBOLS);
        control.finish(success);
        assert_passive(&control);
        assert_eq!(
            control.snapshot().state,
            if success {
                TransmitterPreviewState::Completed
            } else {
                TransmitterPreviewState::Failed
            }
        );
        assert_eq!(control.begin().unwrap_err().code(), ErrorCode::Busy);
    }
    let cancelled = TransmitterPreviewControl::new();
    cancelled.cancel();
    assert_passive(&cancelled);
    assert_eq!(cancelled.snapshot().state, TransmitterPreviewState::Idle);
    cancelled.begin().unwrap();
    cancelled.finish(false);
    assert_passive(&cancelled);
    assert_eq!(
        cancelled.snapshot().state,
        TransmitterPreviewState::Cancelled
    );
}

#[test]
fn metadata_queries_preserve_supported_preview_samples_psd_and_source_provenance() {
    use crate::api::*;
    let profile = Dpd::from_bytes(
        &NamedCapture::new("profile", PROFILE.to_vec()).unwrap(),
        ProfileId::new(0).unwrap(),
    )
    .unwrap();
    let source =
        SourceInput::prbs15(PrbsSeed::new(crate::common::constants::PRBS15_MAX_SEED).unwrap());
    let request = TransmitterPreviewRequest::new(
        &TransmissionConfiguration::new(TransmissionSettings::default()).unwrap(),
        &profile,
        &source,
    )
    .unwrap();
    let original = request.retained_footprint().unwrap();
    let baseline = request.generate(&TransmitterPreviewControl::new()).unwrap();
    let control = TransmitterPreviewControl::new();
    assert_passive(&control);
    let measured = request.generate(&control).unwrap();
    assert_passive(&control);
    assert_eq!(control.snapshot().state, TransmitterPreviewState::Completed);
    assert_eq!(
        serde_json::to_value(measured.info()).unwrap(),
        serde_json::to_value(baseline.info()).unwrap()
    );
    assert_eq!(
        serde_json::to_value(measured.sample_view(0, SAMPLE_VIEW_COUNT).unwrap()).unwrap(),
        serde_json::to_value(baseline.sample_view(0, SAMPLE_VIEW_COUNT).unwrap()).unwrap()
    );
    assert_eq!(
        serde_json::to_value(measured.spectrum(0, 0).unwrap()).unwrap(),
        serde_json::to_value(baseline.spectrum(0, 0).unwrap()).unwrap()
    );
    assert_eq!(request.retained_footprint().unwrap(), original);
}
