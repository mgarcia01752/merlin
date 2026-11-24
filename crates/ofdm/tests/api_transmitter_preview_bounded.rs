/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Real legacy/bounded waveform parity; synthetic geometry is a fixture, not a PHY oracle.
#![cfg(all(feature = "simulation", feature = "pnm-input"))]
mod support;
use docsis_ofdm_fec::{api::*, transmission::layout::PhysicalCarrierRole};
use serde_json::{json, Value};

const ORIGINAL_PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
/// Same finite physical envelope as the established support geometry.
const FIXTURE_BANDWIDTH_HZ: u32 = 24_000_000;
/// Shift the existing small geometry beyond mandatory excluded FFT edges;
/// whole MHz keeps the PLC neighborhood aligned in both numerologies.
const FIXTURE_ACTIVE_OFFSET_HZ: u32 = 8_000_000;
/// Exact finite source includes BOM, NUL and CRLF to detect replacement.
const SOURCE_BYTES: &[u8] = b"\xef\xbb\xbfbounded preview\0source\r\n";
/// Each sample page is bounded independently of the full result budget.
const PAGE_SAMPLES: usize = 4096;

fn request(
    spacing: SubcarrierSpacing,
    file: bool,
) -> (TransmitterPreviewRequest, SourceInput, Vec<u8>) {
    let layout = support::layout(spacing);
    let active_bins = (FIXTURE_BANDWIDTH_HZ / spacing.frequency_hz()) as usize;
    let zero_frequency_hz = layout.frequency_hz(0).unwrap();
    let mut captured: Value = serde_json::from_slice(ORIGINAL_PROFILE).unwrap();
    let templates = captured["profiles"][0]["schemes"]
        .as_array()
        .unwrap()
        .clone();
    let schemes: Vec<Value> = layout.static_roles()[0..active_bins]
        .iter()
        .map(|role| {
            let name = match role {
                PhysicalCarrierRole::Data => "data",
                PhysicalCarrierRole::ContinuousPilot => "continuous_pilot",
                PhysicalCarrierRole::Plc => "plc",
                PhysicalCarrierRole::Excluded => "exclusion",
                PhysicalCarrierRole::ScatteredPilot => {
                    unreachable!("static roles have no scattered pilots")
                }
            };
            if matches!(role, PhysicalCarrierRole::Excluded) {
                // Explicit decoded exclusion assignment; the original source has none.
                return json!({"schema_type":"range", "num_subcarriers":1,
                    "modulation_order_code":16, "modulation_order":"exclusion",
                    "bit_loading":null, "carrier_type":"exclusion"});
            }
            let mut scheme = templates
                .iter()
                .find(|s| s["carrier_type"] == name)
                .unwrap()
                .clone();
            scheme["num_subcarriers"] = json!(1); // One synthetic assignment per physical bin.
            scheme
        })
        .collect();
    captured["subcarrier_zero_frequency"] = json!(zero_frequency_hz);
    captured["first_active_subcarrier_index"] =
        json!(FIXTURE_ACTIVE_OFFSET_HZ / spacing.frequency_hz());
    captured["subcarrier_spacing"] = json!(spacing.frequency_hz());
    captured["num_profiles"] = json!(1); // One admitted fixture profile, not a protocol cap.
    let mut profile = captured["profiles"][0].clone();
    profile["profile_id"] = json!(0); // Legacy bootstrap Profile A.
    profile["schemes"] = json!(schemes);
    captured["profiles"] = json!([profile]);
    let original = serde_json::to_vec(&captured).unwrap();
    let profile = Dpd::from_bytes(
        &NamedCapture::new("synthetic protected grid", original.clone()).unwrap(),
        ProfileId::new(0).unwrap(),
    )
    .unwrap();
    let transmission = TransmissionConfiguration::new(TransmissionSettings {
        occupied_bandwidth_hz: FIXTURE_BANDWIDTH_HZ,
        subcarrier_spacing: spacing,
        ..Default::default()
    })
    .unwrap();
    let source = if file {
        SourceInput::file(
            NamedCapture::new("exact source", SOURCE_BYTES.to_vec()).unwrap(),
            None,
        )
        .unwrap()
    } else {
        SourceInput::prbs15(
            PrbsSeed::new(docsis_ofdm_fec::common::constants::PRBS15_MAX_SEED).unwrap(),
        )
    };
    (
        TransmitterPreviewRequest::new(&transmission, &profile, &source).unwrap(),
        source,
        original,
    )
}

fn parity(spacing: SubcarrierSpacing, file: bool) {
    let (request, source, original) = request(spacing, file);
    let footprint = request.retained_footprint().unwrap();
    let bound = request.preview_retained_bound().unwrap();
    assert_eq!(bound, request.preview_retained_bound().unwrap());
    let legacy = request.generate(&TransmitterPreviewControl::new()).unwrap();
    let control = TransmitterPreviewControl::new();
    let bounded = request.generate_bounded(&control, bound).unwrap();
    assert_eq!(control.snapshot().state, TransmitterPreviewState::Completed);
    assert_eq!(bounded.retained_footprint().unwrap(), bound);
    let info = bounded.info();
    assert_eq!(
        serde_json::to_value(&info).unwrap(),
        serde_json::to_value(legacy.info()).unwrap()
    );
    for start in (0..info.sample_count).step_by(PAGE_SAMPLES) {
        let count = PAGE_SAMPLES.min(info.sample_count - start);
        assert_eq!(
            bounded.sample_view(start, count).unwrap().samples,
            legacy.sample_view(start, count).unwrap().samples
        );
    }
    // Whole-band buckets and each bounded page retain the authoritative full PSD.
    assert_eq!(
        serde_json::to_value(bounded.spectrum(0, 0).unwrap()).unwrap(),
        serde_json::to_value(legacy.spectrum(0, 0).unwrap()).unwrap()
    );
    assert_eq!(request.retained_footprint().unwrap(), footprint);
    assert_eq!(info.sources[0].original_bytes(), original);
    if file {
        assert!(info.source_exhausted);
        assert_eq!(source.original_bytes().unwrap(), SOURCE_BYTES);
        assert_eq!(info.source_sha256.as_deref(), source.original_sha256());
    }
    let alias = bounded.clone();
    let before = serde_json::to_value(alias.info()).unwrap();
    drop(bounded);
    drop(request);
    drop(source);
    assert_eq!(serde_json::to_value(alias.info()).unwrap(), before);
    assert_eq!(alias.info().sources[0].original_bytes(), original);
}

#[test]
fn prbs_25khz_preview_fits_exact_bound_and_preserves_every_sample() {
    parity(SubcarrierSpacing::Khz25, false);
}
#[test]
fn prbs_50khz_preview_fits_exact_bound_and_preserves_every_sample() {
    parity(SubcarrierSpacing::Khz50, false);
}
#[test]
fn file_25khz_preview_fits_exact_bound_and_preserves_original_source() {
    parity(SubcarrierSpacing::Khz25, true);
}
#[test]
fn file_50khz_preview_fits_exact_bound_and_preserves_original_source() {
    parity(SubcarrierSpacing::Khz50, true);
}

#[test]
fn insufficient_dimensions_reject_before_control_admission_and_keep_old_preview() {
    let (request, source, _) = request(SubcarrierSpacing::Khz25, true);
    let bound = request.preview_retained_bound().unwrap();
    let old = request
        .generate_bounded(&TransmitterPreviewControl::new(), bound)
        .unwrap();
    let before = serde_json::to_value(old.info()).unwrap();
    for limit in [
        merlin_api::RetainedFootprint {
            payload_bytes: bound.payload_bytes - 1,
            ..bound
        },
        merlin_api::RetainedFootprint {
            shared_roots: bound.shared_roots - 1,
            ..bound
        },
    ] {
        let control = TransmitterPreviewControl::new();
        assert_eq!(
            request
                .generate_bounded(&control, limit)
                .err()
                .unwrap()
                .code(),
            ErrorCode::ResourceLimit
        );
        assert_eq!(control.snapshot().state, TransmitterPreviewState::Idle);
        assert_eq!(control.snapshot().completed_symbols, 0);
        assert_eq!(source.original_bytes().unwrap(), SOURCE_BYTES);
        assert_eq!(serde_json::to_value(old.info()).unwrap(), before);
    }
    assert_eq!(bound.ordered_entries, 0); // This output owns no ordered container.
}

#[test]
fn cancelled_and_reused_controls_preserve_legacy_errors_without_replacement() {
    let (request, _, _) = request(SubcarrierSpacing::Khz50, false);
    let bound = request.preview_retained_bound().unwrap();
    let cancelled = TransmitterPreviewControl::new();
    cancelled.cancel();
    assert_eq!(
        request
            .generate_bounded(&cancelled, bound)
            .err()
            .unwrap()
            .code(),
        ErrorCode::InvalidInput
    );
    assert_eq!(
        cancelled.snapshot().state,
        TransmitterPreviewState::Cancelled
    );
    let control = TransmitterPreviewControl::new();
    let accepted = request.generate_bounded(&control, bound).unwrap();
    assert_eq!(
        request
            .generate_bounded(&control, bound)
            .err()
            .unwrap()
            .code(),
        ErrorCode::Busy
    );
    assert_eq!(accepted.retained_footprint().unwrap(), bound);
}
