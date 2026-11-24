/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual independent input/idle-preview ownership and passive current-root metadata.
#![cfg(all(feature = "simulation", feature = "pnm-input"))]
use docsis_ofdm_fec::api::*;
use std::mem::size_of;

const PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
const CHANNEL: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/channel_estimation.json");
/// Original captured source ID in the established supported preview fixture.
const FIXTURE_PROFILE_ID: u8 = 0;
/// Independent small contiguous sample query; zero selects the whole PSD separately.
const SAMPLE_QUERY_COUNT: usize = 3;

fn preview_request() -> TransmitterPreviewRequest {
    let transmission = TransmissionConfiguration::new(TransmissionSettings::default()).unwrap();
    let profile = Dpd::from_bytes(
        &NamedCapture::new("original profile", PROFILE.to_vec()).unwrap(),
        ProfileId::new(FIXTURE_PROFILE_ID).unwrap(),
    )
    .unwrap();
    let source = SourceInput::prbs15(
        PrbsSeed::new(docsis_ofdm_fec::common::constants::PRBS15_MAX_SEED).unwrap(),
    );
    TransmitterPreviewRequest::new(&transmission, &profile, &source).unwrap()
}

#[test]
fn named_capture_counts_actual_label_spare_capacity_and_immutable_original_bytes() {
    const LABEL_SPARE_BYTES: usize = 17; // Unused UTF-8 storage, not source data.
    const ORIGINAL_BYTES: &[u8] = b"\xef\xbb\xbfexact\0capture\r\n";
    let mut name = String::with_capacity("original".len() + LABEL_SPARE_BYTES);
    name.push_str("original");
    let label_capacity = name.capacity();
    let capture = NamedCapture::new(name, ORIGINAL_BYTES.to_vec()).unwrap();
    let original_pointer = capture.bytes().as_ptr();
    let footprint = capture.retained_footprint().unwrap();
    assert_eq!(
        footprint.payload_bytes,
        size_of::<NamedCapture>() + label_capacity + ORIGINAL_BYTES.len()
    );
    assert_eq!(footprint.shared_roots, 1);
    assert_eq!(footprint.ordered_entries, 0);
    assert_eq!(capture.retained_footprint().unwrap(), footprint);
    assert_eq!(capture.bytes().as_ptr(), original_pointer);
    assert_eq!(capture.bytes(), ORIGINAL_BYTES);
    assert_eq!(capture.name(), "original");
}

#[test]
fn independent_source_catalog_response_and_provenance_metadata_preserve_originals() {
    const FILE_BYTES: &[u8] = b"\xef\xbb\xbfsource\0bytes\r\n";
    let source = SourceInput::file(
        NamedCapture::new("source", FILE_BYTES.to_vec()).unwrap(),
        None,
    )
    .unwrap();
    let source_hash = source.original_sha256().unwrap().to_owned();
    let source_footprint = source.retained_footprint().unwrap();
    assert!(source_footprint.payload_bytes > size_of::<SourceInput>() + FILE_BYTES.len());
    assert_eq!(source_footprint.shared_roots, 1);
    assert_eq!(source.retained_footprint().unwrap(), source_footprint);
    assert_eq!(source.original_bytes().unwrap(), FILE_BYTES);
    assert_eq!(source.original_sha256().unwrap(), source_hash);

    let original = NamedCapture::new("original catalog", PROFILE.to_vec()).unwrap();
    let catalog = ProfileCapture::from_bytes(&original).unwrap();
    let metadata = serde_json::to_value(catalog.metadata()).unwrap();
    let summary = serde_json::to_value(catalog.summary(FIXTURE_PROFILE_ID).unwrap()).unwrap();
    let catalog_footprint = catalog.retained_footprint().unwrap();
    assert!(catalog_footprint.payload_bytes > size_of::<ProfileCapture>());
    assert!(catalog_footprint.ordered_entries > 0); // Original compact scheme map.
    let provenance = catalog.provenance();
    let provenance_before = serde_json::to_value(provenance).unwrap();
    let provenance_footprint = provenance.retained_footprint().unwrap();
    assert_eq!(provenance_footprint.shared_roots, 2); // Original and decoded edges.
    assert!(
        provenance_footprint.payload_bytes >= size_of::<CaptureProvenance>() + PROFILE.len() * 2
    );
    assert_eq!(
        provenance.retained_footprint().unwrap(),
        provenance_footprint
    );
    assert_eq!(provenance.original_bytes(), PROFILE);
    assert_eq!(provenance.decoded_bytes(), PROFILE);
    assert_eq!(serde_json::to_value(provenance).unwrap(), provenance_before);
    assert_eq!(catalog.retained_footprint().unwrap(), catalog_footprint);
    assert_eq!(serde_json::to_value(catalog.metadata()).unwrap(), metadata);
    assert_eq!(
        serde_json::to_value(catalog.summary(FIXTURE_PROFILE_ID).unwrap()).unwrap(),
        summary
    );
    assert_eq!(original.bytes(), PROFILE);

    let response_input = NamedCapture::new("original H", CHANNEL.to_vec()).unwrap();
    let response = ChannelEstimate::from_bytes(&response_input).unwrap();
    let frame = serde_json::to_value(response.frame()).unwrap();
    let response_footprint = response.retained_footprint().unwrap();
    assert!(response_footprint.payload_bytes > size_of::<ChannelEstimate>());
    assert_eq!(response_footprint.shared_roots, 2); // Original and decoded H bytes.
    assert_eq!(response.retained_footprint().unwrap(), response_footprint);
    assert_eq!(serde_json::to_value(response.frame()).unwrap(), frame);
    assert_eq!(response.provenance().original_bytes(), CHANNEL);
    assert_eq!(response_input.bytes(), CHANNEL);
}

#[test]
fn admitted_idle_request_metadata_is_stable_without_generating_a_preview() {
    let request = preview_request();
    let control = TransmitterPreviewControl::new();
    let initial = serde_json::to_value(control.snapshot()).unwrap();
    let footprint = request.retained_footprint().unwrap();
    assert!(footprint.payload_bytes > size_of::<TransmitterPreviewRequest>());
    assert_eq!(footprint.shared_roots, 2); // Original/decoded profile provenance.
    assert_eq!(footprint.ordered_entries, 0);
    assert_eq!(request.retained_footprint().unwrap(), footprint);
    assert_eq!(serde_json::to_value(control.snapshot()).unwrap(), initial);
    assert_eq!(control.snapshot().state, TransmitterPreviewState::Idle);
}

#[test]
fn actual_preview_metadata_preserves_samples_full_psd_provenance_and_alias_lifetime() {
    let request = preview_request();
    let control = TransmitterPreviewControl::new();
    let output = request.generate(&control).unwrap();
    assert_eq!(control.snapshot().state, TransmitterPreviewState::Completed);
    let info = output.info();
    assert!(info.generated_source_bits > 0 && info.sample_count > 0);
    let settings = serde_json::to_value(&info).unwrap();
    let samples = serde_json::to_value(output.sample_view(0, SAMPLE_QUERY_COUNT).unwrap()).unwrap();
    let spectrum = output.spectrum(0, 0).unwrap();
    let minimum_sample_and_psd_bytes =
        info.sample_count * size_of::<[f64; 2]>() + spectrum.fft_samples * size_of::<f64>();
    let spectrum_before = serde_json::to_value(spectrum).unwrap();
    let progress = serde_json::to_value(control.snapshot()).unwrap();
    let footprint = output.retained_footprint().unwrap();
    assert!(footprint.payload_bytes > minimum_sample_and_psd_bytes);
    assert_eq!(footprint.shared_roots, 3); // Engine result and two provenance edges.
    assert_eq!(footprint.ordered_entries, 0);
    assert_eq!(output.retained_footprint().unwrap(), footprint);
    let alias = output.clone();
    let alias_footprint = alias.retained_footprint().unwrap();
    // Each clone has actual capacities; only its engine payload is necessarily shared.
    assert_eq!(alias.retained_footprint().unwrap(), alias_footprint);
    drop(output);
    drop(request);
    assert_eq!(alias.retained_footprint().unwrap(), alias_footprint);
    assert_eq!(serde_json::to_value(alias.info()).unwrap(), settings);
    assert_eq!(
        serde_json::to_value(alias.sample_view(0, SAMPLE_QUERY_COUNT).unwrap()).unwrap(),
        samples
    );
    assert_eq!(
        serde_json::to_value(alias.spectrum(0, 0).unwrap()).unwrap(),
        spectrum_before
    );
    assert_eq!(serde_json::to_value(control.snapshot()).unwrap(), progress);
    assert_eq!(alias.info().sources[0].original_bytes(), PROFILE);
}
