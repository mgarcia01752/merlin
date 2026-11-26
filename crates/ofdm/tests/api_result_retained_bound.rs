/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Opt-in pre-production result reservation over the established real small fixture.
#![cfg(all(feature = "simulation", feature = "pnm-input"))]
use docsis_ofdm_fec::api::*;
use merlin_api::RetainedFootprint;
use serde_json::{json, Value};

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

fn request(source: Option<SourceInput>) -> FullLoopRequest {
    request_at_duration(source, FIXTURE_DURATION_MS)
}

fn request_at_duration(source: Option<SourceInput>, duration_ms: u64) -> FullLoopRequest {
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
            duration_ms,
            max_iterations: FIXTURE_MAX_ITERATIONS,
            max_runtime_ms: FIXTURE_MAX_RUNTIME_MS,
            ..Default::default()
        },
    )
    .unwrap()
}

fn observed(request: &FullLoopRequest) -> FullLoopRequest {
    request
        .with_observations(FullLoopObservationPolicy::new(Default::default()).unwrap())
        .unwrap()
}

fn fits(actual: RetainedFootprint, bound: RetainedFootprint) -> bool {
    actual.payload_bytes <= bound.payload_bytes
        && actual.ordered_entries <= bound.ordered_entries
        && actual.shared_roots <= bound.shared_roots
}

#[test]
fn actual_owned_query_copies_fit_the_source_envelope_and_keep_original_coordinates() {
    let request = observed(&request(None));
    let result = request
        .run_bounded(
            &FullLoopControl::new(),
            request.result_retained_bound().unwrap(),
        )
        .unwrap();
    let source = result.retained_footprint().unwrap();
    let inventory = result.stage_inventory();
    assert_eq!(inventory.capacity(), inventory.len());
    assert!(
        std::mem::size_of_val(&inventory)
            + inventory.capacity() * std::mem::size_of::<FullLoopStageInfo>()
            <= source.payload_bytes
    );
    let source_inventory = serde_json::to_value(&inventory).unwrap();
    let mut copies = Vec::new();
    for info in inventory {
        for index in 0..info.snapshots {
            let snapshot = result.stage(info.stage, index, Default::default()).unwrap();
            let actual = merlin_api::retention_support::retained_footprint(&snapshot).unwrap();
            assert!(actual.payload_bytes <= source.payload_bytes);
            assert_eq!(actual.ordered_entries, 0);
            assert_eq!(actual.shared_roots, 0);
            assert_eq!(snapshot.stage, info.stage);
            assert_eq!(snapshot.retained_index, index);
            copies.push(snapshot);
        }
    }
    assert!(!copies.is_empty());
    assert_eq!(
        serde_json::to_value(result.stage_inventory()).unwrap(),
        source_inventory
    );
    assert_eq!(result.retained_footprint().unwrap(), source);
    let first = serde_json::to_value(&copies[0]).unwrap();
    drop(result);
    drop(request);
    assert_eq!(serde_json::to_value(&copies[0]).unwrap(), first);
}

#[test]
fn capture_disabled_inventory_has_exact_empty_storage_and_preserves_missing_coverage() {
    let request = request(None);
    let result = request
        .run_bounded(
            &FullLoopControl::new(),
            request.result_retained_bound().unwrap(),
        )
        .unwrap();
    let inventory = result.stage_inventory();
    assert!(inventory.is_empty());
    assert_eq!(inventory.capacity(), 0);
    assert!(
        std::mem::size_of_val(&inventory) <= result.retained_footprint().unwrap().payload_bytes
    );
    assert_eq!(
        result
            .stage(FullLoopStage::PhysicalBins, 0, Default::default())
            .err()
            .unwrap()
            .code(),
        ErrorCode::MissingCoverage
    );
}

fn stable_report(result: &FullLoopResult) -> Value {
    let mut value = serde_json::to_value(result.report()).unwrap();
    // Host wall time is independent of deterministic scientific output.
    value.as_object_mut().unwrap().remove("elapsed_ms");
    value
}

fn assert_parity(legacy: &FullLoopResult, bounded: &FullLoopResult) {
    assert_eq!(stable_report(bounded), stable_report(legacy));
    assert_eq!(
        serde_json::to_value(bounded.fec_recovery()).unwrap(),
        serde_json::to_value(legacy.fec_recovery()).unwrap()
    );
    assert_eq!(bounded.recovered_bytes(), legacy.recovered_bytes());
    assert_eq!(
        serde_json::to_value(bounded.inputs().summary().unwrap()).unwrap(),
        serde_json::to_value(legacy.inputs().summary().unwrap()).unwrap()
    );
    assert_eq!(
        serde_json::to_value(bounded.profile_performance().unwrap()).unwrap(),
        serde_json::to_value(legacy.profile_performance().unwrap()).unwrap()
    );
    assert_eq!(
        serde_json::to_value(bounded.observation_stats()).unwrap(),
        serde_json::to_value(legacy.observation_stats()).unwrap()
    );
    let inventory = bounded.stage_inventory();
    assert_eq!(
        serde_json::to_value(&inventory).unwrap(),
        serde_json::to_value(legacy.stage_inventory()).unwrap()
    );
    for stage in inventory {
        assert_eq!(
            serde_json::to_value(bounded.stage(stage.stage, 0, Default::default()).unwrap())
                .unwrap(),
            serde_json::to_value(legacy.stage(stage.stage, 0, Default::default()).unwrap())
                .unwrap()
        );
    }
    assert_eq!(
        bounded.report().carrier_mer.capacity(),
        bounded.report().carrier_mer.len()
    );
    assert_eq!(
        bounded.report().retained_words.capacity(),
        bounded.report().retained_words.len()
    );
    assert_eq!(
        bounded.report().summary.payload_ldpc_iterations.capacity(),
        bounded.report().summary.payload_ldpc_iterations.len()
    );
    if let Some(estimate) = &bounded.report().channel_estimate {
        assert_eq!(estimate.pilots.capacity(), estimate.pilots.len());
        assert_eq!(estimate.response.capacity(), estimate.response.len());
    }
    if let Some(capture) = &bounded.report().constellation {
        assert_eq!(capture.pairs.capacity(), capture.pairs.len());
        assert_eq!(capture.ncp_pairs.capacity(), capture.ncp_pairs.len());
        for llrs in capture
            .pairs
            .iter()
            .map(|pair| &pair.llrs)
            .chain(capture.ncp_pairs.iter().map(|pair| &pair.llrs))
        {
            assert_eq!(llrs.capacity(), llrs.len());
        }
    }
    if let Some(capture) = &bounded.report().protected_constellation {
        for stream in [
            &capture.continuous_pilots,
            &capture.scattered_pilots,
            &capture.plc_data,
            &capture.plc_preamble,
        ] {
            assert_eq!(stream.pairs.capacity(), stream.pairs.len());
        }
    }
}

#[test]
fn complete_unobserved_result_fits_exact_reservation_and_preserves_legacy_queries() {
    let request = request(None);
    let before = request.retained_footprint().unwrap();
    let bound = request.result_retained_bound().unwrap();
    assert_eq!(bound.ordered_entries, 0);
    assert_eq!(request.result_retained_bound().unwrap(), bound);
    let legacy = request.run(&FullLoopControl::new()).unwrap();
    let bounded = request.run_bounded(&FullLoopControl::new(), bound).unwrap();
    assert_eq!(bounded.report().status, ChannelJobStatus::Complete);
    assert!(bounded.report().summary.recovered_payloads > 0);
    assert!(fits(bounded.retained_footprint().unwrap(), bound));
    assert_parity(&legacy, &bounded);
    assert_eq!(request.retained_footprint().unwrap(), before);
    assert_eq!(request.result_retained_bound().unwrap(), bound);
    let retained = bounded.retained_footprint().unwrap();
    let alias = bounded.clone();
    drop(bounded);
    // A caller-created clone is a separate root; Clone may change spare capacity.
    let alias_metadata = alias.retained_footprint().unwrap();
    assert_eq!(alias.retained_footprint().unwrap(), alias_metadata);
    assert_eq!(stable_report(&alias), stable_report(&legacy));
    assert!(retained.payload_bytes > 0);
}

#[test]
fn complete_all_selected_stages_fit_and_preserve_original_context_and_statistics() {
    let request = observed(&request(None));
    let bound = request.result_retained_bound().unwrap();
    assert!(bound.ordered_entries > 0);
    let legacy = request.run(&FullLoopControl::new()).unwrap();
    let bounded = request.run_bounded(&FullLoopControl::new(), bound).unwrap();
    assert_eq!(bounded.report().status, ChannelJobStatus::Complete);
    assert!(bounded.observation_stats().retained_events > 0);
    assert!(fits(bounded.retained_footprint().unwrap(), bound));
    assert_parity(&legacy, &bounded);
}

#[test]
fn every_insufficient_dimension_rejects_before_control_or_source_progress() {
    const ORIGINAL_BYTES: &[u8] = b"original finite source remains unchanged after rejection";
    let request = observed(&request(Some(
        SourceInput::file(
            NamedCapture::new("original", ORIGINAL_BYTES.to_vec()).unwrap(),
            None,
        )
        .unwrap(),
    )));
    let before = request.retained_footprint().unwrap();
    let settings = serde_json::to_value(request.settings()).unwrap();
    let bound = request.result_retained_bound().unwrap();
    for limit in [
        RetainedFootprint {
            payload_bytes: bound.payload_bytes - 1,
            ..bound
        },
        RetainedFootprint {
            ordered_entries: bound.ordered_entries - 1,
            ..bound
        },
        RetainedFootprint {
            shared_roots: bound.shared_roots - 1,
            ..bound
        },
        RetainedFootprint {
            shared_roots: 0,
            ..bound
        },
    ] {
        let control = FullLoopControl::new();
        let initial = serde_json::to_value(control.snapshot()).unwrap();
        let Err(error) = request.run_bounded(&control, limit) else {
            panic!("insufficient reservation must reject");
        };
        assert_eq!(error.code(), ErrorCode::ResourceLimit);
        assert_eq!(serde_json::to_value(control.snapshot()).unwrap(), initial);
        assert!(control.last_progress_age_ms().is_none());
        assert_eq!(request.retained_footprint().unwrap(), before);
        assert_eq!(serde_json::to_value(request.settings()).unwrap(), settings);
        assert_eq!(request.result_retained_bound().unwrap(), bound);
    }
    let result = request.run_bounded(&FullLoopControl::new(), bound).unwrap();
    assert_eq!(result.recovered_bytes().unwrap(), ORIGINAL_BYTES);
}

#[test]
fn one_extra_unit_in_each_dimension_admits_the_actual_file_result() {
    const ORIGINAL_BYTES: &[u8] = b"finite output preserves nulls \0 and exact original bytes";
    let source = SourceInput::file(
        NamedCapture::new("finite", ORIGINAL_BYTES.to_vec()).unwrap(),
        None,
    )
    .unwrap();
    let request = request(Some(source));
    let bound = request.result_retained_bound().unwrap();
    let limit = RetainedFootprint {
        payload_bytes: bound.payload_bytes + 1,
        ordered_entries: bound.ordered_entries + 1,
        shared_roots: bound.shared_roots + 1,
    };
    let legacy = request.run(&FullLoopControl::new()).unwrap();
    let result = request.run_bounded(&FullLoopControl::new(), limit).unwrap();
    assert_eq!(result.recovered_bytes().unwrap(), ORIGINAL_BYTES);
    assert!(fits(result.retained_footprint().unwrap(), bound));
    assert_parity(&legacy, &result);
}

#[test]
fn precancelled_observed_and_unobserved_results_preserve_honest_partial_reports() {
    let base = request(None);
    for request in [base.clone(), observed(&base)] {
        let bound = request.result_retained_bound().unwrap();
        let control = FullLoopControl::new();
        control.cancel();
        let legacy_control = FullLoopControl::new();
        legacy_control.cancel();
        let legacy = request.run(&legacy_control).unwrap();
        let result = request.run_bounded(&control, bound).unwrap();
        assert_eq!(result.report().status, ChannelJobStatus::Cancelled);
        assert_eq!(result.report().summary.recovered_payloads, 0);
        assert!(fits(result.retained_footprint().unwrap(), bound));
        assert_parity(&legacy, &result);
        let Err(error) = request.run_bounded(&control, bound) else {
            panic!("used control must reject");
        };
        assert_eq!(error.code(), ErrorCode::Busy);
    }
}

#[test]
fn cancellation_after_completed_trials_preserves_observed_and_unobserved_partial_results() {
    use std::time::{Duration, Instant};
    // Existing maximum observation window keeps the worker active while the host
    // observes its first genuine completed trial; this adds no scientific limit.
    const CANCELLATION_WINDOW_MS: u64 = MAX_FULL_LOOP_DURATION_MS;
    // Host-only polling interval; it never enters scientific configuration.
    const PROGRESS_POLL_MS: u64 = 1;
    let base = request_at_duration(None, CANCELLATION_WINDOW_MS);
    for request in [base.clone(), observed(&base)] {
        let bound = request.result_retained_bound().unwrap();
        let control = FullLoopControl::new();
        let worker_control = control.clone();
        let worker = std::thread::spawn(move || request.run_bounded(&worker_control, bound));
        let deadline = Instant::now() + Duration::from_millis(FIXTURE_MAX_RUNTIME_MS);
        let mut completed = 0;
        while Instant::now() < deadline {
            let snapshot = control.snapshot();
            if let Some(progress) = snapshot.progress {
                completed = progress.recovered_payloads;
                if completed > 0 {
                    break;
                }
            }
            if !matches!(
                snapshot.state,
                FullLoopState::Idle | FullLoopState::Running | FullLoopState::Paused
            ) {
                break;
            }
            std::thread::sleep(Duration::from_millis(PROGRESS_POLL_MS));
        }
        control.cancel();
        let result = worker.join().unwrap().unwrap();
        assert!(
            completed > 0,
            "fixture must expose a genuine completed trial"
        );
        assert_eq!(result.report().status, ChannelJobStatus::Cancelled);
        assert!(result.report().summary.recovered_payloads >= completed);
        assert!(result.report().summary.fec.total_codewords() >= completed);
        assert!(fits(result.retained_footprint().unwrap(), bound));
    }
}
