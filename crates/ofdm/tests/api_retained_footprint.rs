/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Passive current-root metadata over the established real small full-loop fixture.
#![cfg(all(feature = "simulation", feature = "pnm-input"))]
use docsis_ofdm_fec::api::*;
use serde_json::{json, Value};
use std::mem::size_of;

const PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
const RXMER: &[u8] = include_bytes!("../../../examples/measurements/single-channel/rxmer.json");
/// Synthetic clean residual fixture in dB, not a normative receiver threshold.
const CLEAN_FIXTURE_MER_DB: f64 = 80.0;
/// Existing one-millisecond supported full-loop test window.
const FIXTURE_DURATION_MS: u64 = 1;
/// Existing small-fixture LDPC iteration budget, independent of duration.
const FIXTURE_MAX_ITERATIONS: usize = 10;
/// Host safety timeout in milliseconds, independent of the observation window.
const FIXTURE_MAX_RUNTIME_MS: u64 = 60_000;
/// Single-mode playback clock is retained but never advances to a second capture.
const FIXTURE_INTERVAL_SYMBOLS: u64 = 1_000;
/// Finite decoder prefix exercises actual LLR/context retention and archive copying.
const DECODER_PREFIX_ELEMENTS: usize = 128;
/// A small actual-cell query, independent of decoder-prefix retention.
const QUERY_CELL_COUNT: usize = 3;

fn request(source: Option<SourceInput>) -> FullLoopRequest {
    let profile = Dpd::from_bytes(
        &NamedCapture::new("profile", PROFILE.to_vec()).unwrap(),
        ProfileId::new(0).unwrap(), // Existing captured fixture profile identity.
    )
    .unwrap();
    let transmission = TransmissionConfiguration::new(TransmissionSettings::default()).unwrap();
    let mut capture: Value = serde_json::from_slice(RXMER).unwrap();
    let source_carriers = capture["values"].as_array().unwrap().len();
    capture["values"] = json!(vec![CLEAN_FIXTURE_MER_DB; source_carriers]);
    capture["pnm_header"]["capture_time"] = json!(profile.sources()[0].original_epoch().value());
    let measured = RxMer::from_bytes(
        &NamedCapture::new(
            "synthetic clean residual",
            serde_json::to_vec(&capture).unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    let measurements = MeasurementSequence::new(
        &profile,
        &transmission,
        vec![measured],
        MeasurementSequenceOptions {
            mode: MeasurementMode::Single,
            interval_symbols: FIXTURE_INTERVAL_SYMBOLS,
            tolerance: AssociationTolerance::ZERO,
        },
    )
    .unwrap();
    Simulation::prepare_full_loop(
        FullLoopInputs {
            transmission,
            profile,
            measurements,
            source: source.unwrap_or_else(|| {
                SourceInput::prbs15(
                    PrbsSeed::new(docsis_ofdm_fec::common::constants::PRBS15_MAX_SEED).unwrap(),
                )
            }),
            channel: ChannelConfiguration::new(ChannelConfigurationInput::default()).unwrap(),
            response: None,
        },
        FullLoopOptions {
            duration_ms: FIXTURE_DURATION_MS,
            max_iterations: FIXTURE_MAX_ITERATIONS,
            max_runtime_ms: FIXTURE_MAX_RUNTIME_MS,
            ..Default::default()
        },
    )
    .unwrap()
}

fn observed(request: &FullLoopRequest) -> FullLoopRequest {
    request
        .with_observations(
            FullLoopObservationPolicy::new(FullLoopObservationInput {
                stages: vec![FullLoopStageRetention {
                    stage: FullLoopStage::PayloadLdpcInput,
                    max_events: 1, // One original actual decoder event suffices.
                    observation_only: false, // Preserve actual warm-up/observation/drain tags.
                }],
                max_events: 1,
                max_elements_per_event: DECODER_PREFIX_ELEMENTS,
                max_total_elements: DECODER_PREFIX_ELEMENTS,
                ..Default::default()
            })
            .unwrap(),
        )
        .unwrap()
}

#[test]
fn prepared_request_and_sweep_count_selected_spares_without_starting_science() {
    // Both inventories contain the same identity; only caller spare slots differ.
    const SMALL_SELECTION_SLOTS: usize = 3;
    const LARGE_SELECTION_SLOTS: usize = 11;
    let request = request(None);
    let settings = serde_json::to_value(request.settings()).unwrap();
    let options = serde_json::to_value(request.options()).unwrap();
    let control = FullLoopControl::new();
    let initial = serde_json::to_value(control.snapshot()).unwrap();
    let footprint = request.retained_footprint().unwrap();
    assert!(footprint.payload_bytes > size_of::<FullLoopRequest>());
    assert!(footprint.shared_roots > 0);
    let catalog =
        ProfileCapture::from_bytes(&NamedCapture::new("catalog", PROFILE.to_vec()).unwrap())
            .unwrap();
    let plan = |capacity| {
        let mut identities = Vec::with_capacity(capacity);
        identities.push(ProfileId::new(0).unwrap());
        let actual_capacity = identities.capacity();
        (
            FullLoopSweepPlan::new(
                &request,
                &catalog,
                FullLoopProfileSelection::Selected(identities),
            )
            .unwrap(),
            actual_capacity,
        )
    };
    let (small, small_capacity) = plan(SMALL_SELECTION_SLOTS);
    let (large, large_capacity) = plan(LARGE_SELECTION_SLOTS);
    let small_footprint = small.retained_footprint().unwrap();
    let large_footprint = large.retained_footprint().unwrap();
    assert_eq!(
        large_footprint.payload_bytes - small_footprint.payload_bytes,
        (large_capacity - small_capacity) * size_of::<ProfileId>()
    );
    assert_eq!(large_footprint.shared_roots, small_footprint.shared_roots);
    assert_eq!(
        large_footprint.ordered_entries,
        small_footprint.ordered_entries
    );
    assert_eq!(
        serde_json::to_value(large.info()).unwrap(),
        serde_json::to_value(small.info()).unwrap()
    );
    assert_eq!(request.retained_footprint().unwrap(), footprint);
    assert_eq!(serde_json::to_value(request.settings()).unwrap(), settings);
    assert_eq!(serde_json::to_value(request.options()).unwrap(), options);
    assert_eq!(serde_json::to_value(control.snapshot()).unwrap(), initial);
}

#[test]
fn complete_actual_result_metadata_preserves_report_inputs_stages_and_aliases() {
    let request = observed(&request(None));
    let control = FullLoopControl::new();
    let result = request.run(&control).unwrap();
    assert_eq!(result.report().status, ChannelJobStatus::Complete);
    assert_eq!(result.report().summary.comparison.bit_errors, 0);
    assert!(result.report().summary.recovered_payloads > 0);
    let before = serde_json::to_value(result.report()).unwrap();
    let inputs = serde_json::to_value(result.inputs().summary().unwrap()).unwrap();
    let stage = serde_json::to_value(
        result
            .stage(FullLoopStage::PayloadLdpcInput, 0, Default::default())
            .unwrap(),
    )
    .unwrap();
    let progress = serde_json::to_value(control.snapshot()).unwrap();
    let alias = result.clone();
    let footprint = result.retained_footprint().unwrap();
    assert!(footprint.payload_bytes > size_of::<FullLoopResult>());
    assert!(footprint.shared_roots >= 2); // Actual report and stage-store owners.
    assert!(footprint.ordered_entries > 0); // Actual selected stage's map entry.
    let alias_footprint = alias.retained_footprint().unwrap();
    assert_eq!(alias.retained_footprint().unwrap(), alias_footprint);
    assert_eq!(serde_json::to_value(alias.report()).unwrap(), before);
    drop(alias);
    assert_eq!(result.retained_footprint().unwrap(), footprint);
    assert_eq!(serde_json::to_value(result.report()).unwrap(), before);
    assert_eq!(
        serde_json::to_value(result.inputs().summary().unwrap()).unwrap(),
        inputs
    );
    assert_eq!(
        serde_json::to_value(
            result
                .stage(FullLoopStage::PayloadLdpcInput, 0, Default::default())
                .unwrap()
        )
        .unwrap(),
        stage
    );
    assert_eq!(serde_json::to_value(control.snapshot()).unwrap(), progress);
}

#[test]
fn precancelled_actual_result_counts_retained_inputs_without_fabricating_work() {
    let request = request(None);
    let control = FullLoopControl::new();
    control.cancel();
    let result = request.run(&control).unwrap();
    assert_eq!(result.report().status, ChannelJobStatus::Cancelled);
    assert_eq!(result.report().summary.recovered_payloads, 0);
    let report = serde_json::to_value(result.report()).unwrap();
    let progress = serde_json::to_value(control.snapshot()).unwrap();
    let footprint = result.retained_footprint().unwrap();
    assert!(footprint.payload_bytes > size_of::<FullLoopResult>());
    assert!(footprint.shared_roots >= 2); // Report/stages exist even before work.
    assert_eq!(result.retained_footprint().unwrap(), footprint);
    assert_eq!(serde_json::to_value(result.report()).unwrap(), report);
    assert_eq!(serde_json::to_value(control.snapshot()).unwrap(), progress);
}

#[test]
fn finite_file_and_detached_archive_metadata_preserve_owned_observations_and_legacy_bytes() {
    const FILE_BYTES: &[u8] = b"independent finite source for retained metadata";
    let source = SourceInput::file(
        NamedCapture::new("finite source", FILE_BYTES.to_vec()).unwrap(),
        None,
    )
    .unwrap();
    let request = observed(&request(Some(source)));
    let result = request.run(&FullLoopControl::new()).unwrap();
    assert_eq!(result.report().status, ChannelJobStatus::Complete);
    assert_eq!(result.recovered_bytes().unwrap(), FILE_BYTES);
    let recovered = result.recovered_bytes().unwrap().to_vec();
    let report = serde_json::to_value(result.report()).unwrap();
    result.retained_footprint().unwrap();
    assert_eq!(result.recovered_bytes().unwrap(), recovered);
    assert_eq!(serde_json::to_value(result.report()).unwrap(), report);
    let archive = result
        .receiver_archive(FullLoopReceiverArchivePolicy::new(Default::default()).unwrap())
        .unwrap();
    let legacy_bytes = archive.info().retained_bytes;
    let info = serde_json::to_value(archive.info()).unwrap();
    let cells = || FullLoopCellQuery {
        count: QUERY_CELL_COUNT,
        ..Default::default()
    };
    let qam = serde_json::to_value(archive.qam(cells()).unwrap()).unwrap();
    let decoder = serde_json::to_value(archive.decoder_soft_view().unwrap()).unwrap();
    let footprint = archive.retained_footprint().unwrap();
    assert_eq!(footprint.shared_roots, 1); // Sole detached Retained Arc root.
    assert_eq!(footprint.ordered_entries, 0);
    assert!(footprint.payload_bytes >= legacy_bytes);
    let alias = archive.clone();
    assert_eq!(alias.retained_footprint().unwrap(), footprint);
    drop(result);
    drop(request);
    drop(archive);
    assert_eq!(alias.retained_footprint().unwrap(), footprint);
    assert_eq!(alias.info().retained_bytes, legacy_bytes);
    assert_eq!(serde_json::to_value(alias.info()).unwrap(), info);
    assert_eq!(
        serde_json::to_value(alias.qam(cells()).unwrap()).unwrap(),
        qam
    );
    assert_eq!(
        serde_json::to_value(alias.decoder_soft_view().unwrap()).unwrap(),
        decoder
    );
}
