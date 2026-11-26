/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Opt-in retained-target admission over the existing real small closed-loop fixture.
#![cfg(all(feature = "simulation", feature = "pnm-input"))]
use docsis_ofdm_fec::{
    api::*,
    simulation::channel::protected_constellation::{ProtectedConstellation, ProtectedPair},
};
use merlin_api::RetainedFootprint;
use serde::Serialize;
use serde_json::{json, Value};
use std::mem::size_of;

const PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
const RXMER: &[u8] = include_bytes!("../../../examples/measurements/single-channel/rxmer.json");
/// Clean synthetic residual in dB; not a normative receiver threshold.
const FIXTURE_MER_DB: f64 = 80.0;
/// Existing one-millisecond closed-loop fixture and independent decoder/runtime budgets.
const FIXTURE_DURATION_MS: u64 = 1;
const FIXTURE_MAX_ITERATIONS: usize = 10;
const FIXTURE_MAX_RUNTIME_MS: u64 = 60_000;
/// Retained single-capture clock, unused for advancing to a second original capture.
const FIXTURE_INTERVAL_SYMBOLS: u64 = 1_000;
/// One actual event prefix suffices; archive prefix is deliberately shorter.
const RETAINED_DECODER_ELEMENTS: usize = 128;
const ARCHIVED_DECODER_ELEMENTS: usize = 5;
/// Uniform small role selections exercise original floor coordinates and all query planes.
const ARCHIVED_CELLS_PER_ROLE: usize = 3;

fn request() -> FullLoopRequest {
    let profile = Dpd::from_bytes(
        &NamedCapture::new("profile", PROFILE.to_vec()).unwrap(),
        ProfileId::new(0).unwrap(), // Existing captured fixture identity.
    )
    .unwrap();
    let transmission = TransmissionConfiguration::new(TransmissionSettings::default()).unwrap();
    let mut capture: Value = serde_json::from_slice(RXMER).unwrap();
    let carriers = capture["values"].as_array().unwrap().len();
    capture["values"] = json!(vec![FIXTURE_MER_DB; carriers]);
    capture["pnm_header"]["capture_time"] = json!(profile.sources()[0].original_epoch().value());
    let measured = RxMer::from_bytes(
        &NamedCapture::new(
            "clean synthetic residual",
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
            source: SourceInput::prbs15(
                PrbsSeed::new(docsis_ofdm_fec::common::constants::PRBS15_MAX_SEED).unwrap(),
            ),
            channel: ChannelConfiguration::new(Default::default()).unwrap(),
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
    .with_observations(
        FullLoopObservationPolicy::new(FullLoopObservationInput {
            stages: vec![FullLoopStageRetention {
                stage: FullLoopStage::PayloadLdpcInput,
                max_events: 1, // One original decoder event, preserving its actual phase.
                observation_only: false,
            }],
            max_events: 1,
            max_elements_per_event: RETAINED_DECODER_ELEMENTS,
            max_total_elements: RETAINED_DECODER_ELEMENTS,
            ..Default::default()
        })
        .unwrap(),
    )
    .unwrap()
}

fn policy() -> FullLoopReceiverArchivePolicy {
    FullLoopReceiverArchivePolicy::new(FullLoopReceiverArchiveInput {
        payload_cells: ARCHIVED_CELLS_PER_ROLE,
        ncp_cells: ARCHIVED_CELLS_PER_ROLE,
        protected_cells_per_stream: ARCHIVED_CELLS_PER_ROLE,
        decoder_llrs: ARCHIVED_DECODER_ELEMENTS,
        ..Default::default()
    })
    .unwrap()
}

fn projected<Value: Serialize>(result: ApiResult<Value>) -> serde_json::Value {
    match result {
        Ok(value) => json!({"value": value}),
        Err(error) => json!({"error": error}),
    }
}

#[test]
fn actual_complete_bound_matches_archive_and_preserves_all_legacy_query_planes_and_source() {
    let request = request();
    let result = request.run(&FullLoopControl::new()).unwrap();
    assert_eq!(result.report().status, ChannelJobStatus::Complete);
    assert_eq!(result.report().summary.comparison.bit_errors, 0);
    let source = serde_json::to_value(result.report()).unwrap();
    let source_metadata = result.retained_footprint().unwrap();
    let protected = result.report().protected_constellation.as_ref().unwrap();
    let public_pair_bytes = [
        &protected.continuous_pilots,
        &protected.scattered_pilots,
        &protected.plc_data,
        &protected.plc_preamble,
    ]
    .iter()
    .map(|stream| stream.pairs.capacity() * size_of::<ProtectedPair>())
    .sum::<usize>();
    // The real source owns a nonempty serialization-skipped pilot cache. The detached
    // constructor omits it, which is independently checked by exact target equality.
    assert!(
        merlin_api::retention_support::retained_footprint(protected)
            .unwrap()
            .payload_bytes
            > size_of::<ProtectedConstellation>() + public_pair_bytes
    );
    let bound = result.receiver_archive_retained_bound(&policy()).unwrap();
    assert_eq!(result.retained_footprint().unwrap(), source_metadata);
    assert_eq!(serde_json::to_value(result.report()).unwrap(), source);
    let legacy = result.receiver_archive(policy()).unwrap();
    let archive = result.receiver_archive_bounded(policy(), bound).unwrap();
    assert_eq!(archive.retained_footprint().unwrap(), bound);
    assert_eq!(bound.ordered_entries, 0);
    assert_eq!(bound.shared_roots, 1);
    assert_eq!(
        serde_json::to_value(archive.info()).unwrap(),
        serde_json::to_value(legacy.info()).unwrap()
    );
    assert_eq!(
        archive.info().retained_bytes + size_of::<FullLoopReceiverArchive>(),
        bound.payload_bytes
    );
    for stream in [
        FullLoopCellStream::Payload,
        FullLoopCellStream::Ncp,
        FullLoopCellStream::ContinuousPilots,
        FullLoopCellStream::ScatteredPilots,
        FullLoopCellStream::PlcData,
        FullLoopCellStream::PlcPreamble,
    ] {
        let query = FullLoopCellQuery {
            stream,
            count: ARCHIVED_CELLS_PER_ROLE,
            ..Default::default()
        };
        assert_eq!(
            projected(archive.qam(query.clone())),
            projected(legacy.qam(query.clone()))
        );
        assert_eq!(
            projected(archive.cell_decisions(query.clone())),
            projected(legacy.cell_decisions(query.clone()))
        );
        assert_eq!(
            projected(archive.cell_soft_view(query.clone())),
            projected(legacy.cell_soft_view(query))
        );
    }
    assert_eq!(
        projected(archive.decoder_soft_view()),
        projected(legacy.decoder_soft_view())
    );
    assert_eq!(archive.info().decoder_llrs, ARCHIVED_DECODER_ELEMENTS);
    for insufficient in [
        RetainedFootprint {
            payload_bytes: bound.payload_bytes - 1,
            ..bound
        },
        RetainedFootprint {
            shared_roots: 0,
            ..bound
        },
    ] {
        let Err(error) = result.receiver_archive_bounded(policy(), insufficient) else {
            panic!("Insufficient target must reject before copying");
        };
        assert_eq!(error.code(), ErrorCode::ResourceLimit);
        assert_eq!(serde_json::to_value(result.report()).unwrap(), source);
        assert_eq!(result.retained_footprint().unwrap(), source_metadata);
    }
    let alias = archive.clone();
    let info = serde_json::to_value(alias.info()).unwrap();
    let cells = projected(alias.qam(Default::default()));
    let decoder = projected(alias.decoder_soft_view());
    drop(archive);
    drop(result);
    drop(request);
    assert_eq!(alias.retained_footprint().unwrap(), bound);
    assert_eq!(serde_json::to_value(alias.info()).unwrap(), info);
    assert_eq!(projected(alias.qam(Default::default())), cells);
    assert_eq!(projected(alias.decoder_soft_view()), decoder);
}

#[test]
fn actual_precancelled_result_and_omitted_policy_admit_exact_zero_entry_target() {
    let request = request();
    let control = FullLoopControl::new();
    control.cancel();
    let result = request.run(&control).unwrap();
    assert_eq!(result.report().status, ChannelJobStatus::Cancelled);
    let source = serde_json::to_value(result.report()).unwrap();
    let omitted = || {
        FullLoopReceiverArchivePolicy::new(FullLoopReceiverArchiveInput {
            payload_cells: 0,
            ncp_cells: 0,
            protected_cells_per_stream: 0,
            decoder_llrs: 0,
            ..Default::default()
        })
        .unwrap()
    };
    let bound = result.receiver_archive_retained_bound(&omitted()).unwrap();
    assert_eq!(bound.ordered_entries, 0);
    assert_eq!(bound.shared_roots, 1);
    let archive = result.receiver_archive_bounded(omitted(), bound).unwrap();
    assert_eq!(archive.retained_footprint().unwrap(), bound);
    let legacy = result.receiver_archive(omitted()).unwrap();
    assert_eq!(
        serde_json::to_value(archive.info()).unwrap(),
        serde_json::to_value(legacy.info()).unwrap()
    );
    assert_eq!(
        projected(archive.decoder_soft_view()),
        projected(legacy.decoder_soft_view())
    );
    assert_eq!(serde_json::to_value(result.report()).unwrap(), source);
}
