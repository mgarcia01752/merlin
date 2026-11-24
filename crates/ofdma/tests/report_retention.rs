/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Pre-production retained-product admission and unchanged actual upstream science.
use docsis_ofdma::api::*;
use merlin_api::RetainedFootprint;
use serde_json::Value;
use std::{
    thread,
    time::{Duration, Instant},
};

const DOCUMENT: &[u8] = include_bytes!("../../../examples/cli/upstream/us-50khz.json");
const PLAYBACK_DOCUMENT: &[u8] =
    include_bytes!("../../../examples/cli/upstream/us-synthetic-playback.json");
const SECOND_NUMEROLOGY_DOCUMENT: &[u8] =
    include_bytes!("../../../examples/cli/upstream/us-25khz.json");
const MEASURED_PROVENANCE: &str =
    "{\"format\":\"merlin.upstream.measured-provenance\",\"schema_version\":1}";
// Independent application test policies, not new scientific limits.
const ESTIMATE_FRAMES: usize = 2;
const ESTIMATE_SCALAR_SLOTS: usize = 262_144;
const SOFT_CELLS: usize = 16;
const SOFT_WORDS: usize = 2;
const SOFT_WORD_LLRS: usize = 262_144;
const CANCELLABLE_GRANTS: usize = 128;
const ASYNC_PROGRESS_DEADLINE_SECONDS: u64 = 20;
const SPARE_INPUT_TAPS: usize = 29;

fn session(document: &[u8], observed: bool) -> Simulation {
    let document = ConfigurationDocument::from_json(document).unwrap();
    let (_, options) = document.prepare().unwrap();
    let mut session = Simulation::new(options).unwrap();
    session.configure_document(document).unwrap();
    if observed {
        session
            .set_receiver_observation_policy(
                ReceiverObservationPolicy::new(ESTIMATE_FRAMES, ESTIMATE_SCALAR_SLOTS)
                    .unwrap()
                    .with_soft_metrics(
                        SoftObservationPolicy::new(SOFT_CELLS, SOFT_WORDS, SOFT_WORD_LLRS).unwrap(),
                    ),
            )
            .unwrap();
    }
    session
}

fn assert_fits(actual: RetainedFootprint, bound: RetainedFootprint) {
    assert!(actual.payload_bytes <= bound.payload_bytes);
    assert!(actual.ordered_entries <= bound.ordered_entries);
    assert!(actual.shared_roots <= bound.shared_roots);
}

// Run identity and measured active elapsed time intentionally differ between calls.
// Every scientific/configuration/source/provenance/observation field is retained.
fn stable_science(mut value: Value) -> Value {
    fn visit(value: &mut Value) {
        match value {
            Value::Object(object) => {
                let _ = object.remove("elapsed_ms");
                if object.contains_key("generation") && object.contains_key("grant") {
                    let _ = object.remove("run");
                }
                for child in object.values_mut() {
                    visit(child);
                }
            }
            Value::Array(values) => {
                for child in values {
                    visit(child);
                }
            }
            _ => {}
        }
    }
    visit(&mut value);
    value
}

fn check_legacy_parity(session: &mut Simulation) -> BatchReport {
    let accepted = session.retained_footprint().unwrap();
    let preview = session.preview().unwrap();
    let bound = session.batch_report_retained_bound().unwrap();
    assert_eq!(session.retained_footprint().unwrap(), accepted);
    assert_eq!(session.preview().unwrap(), preview);
    assert_eq!(session.batch_report_retained_bound().unwrap(), bound);
    let legacy = session.run().unwrap();
    let bounded = session
        .run_with_control_bounded(&RunControl::new(), bound)
        .unwrap();
    assert_fits(bounded.retained_footprint().unwrap(), bound);
    assert_eq!(
        stable_science(serde_json::to_value(&bounded).unwrap()),
        stable_science(serde_json::to_value(&legacy).unwrap())
    );
    assert_eq!(session.retained_footprint().unwrap(), accepted);
    bounded
}

#[test]
fn missing_configuration_rejects_without_claiming_control() {
    let mut session = Simulation::new(ExperimentOptions::default()).unwrap();
    let control = RunControl::new();
    let progress = control.progress();
    assert_eq!(
        session.batch_report_retained_bound().unwrap_err().code(),
        "MissingSetting"
    );
    assert_eq!(
        session
            .run_with_control_bounded(&control, RetainedFootprint::default())
            .unwrap_err()
            .code(),
        "MissingSetting"
    );
    assert_eq!(control.progress(), progress);
}

#[test]
fn typed_unobserved_exact_bound_preserves_legacy_recovery_and_removes_spare_capacity() {
    let document = ConfigurationDocument::from_json(DOCUMENT).unwrap();
    let (mut configuration, mut options) = document.prepare().unwrap();
    configuration.channel.taps.reserve_exact(SPARE_INPUT_TAPS);
    options.retained_points = 0;
    options.retained_taps = Vec::with_capacity(SPARE_INPUT_TAPS);
    let mut session = Simulation::new(options).unwrap();
    session.configure(configuration).unwrap();
    let report = check_legacy_parity(&mut session);
    assert!(report.reference_recovered);
    assert_eq!(report.source_configuration_original, None);
    assert_eq!(report.source_configuration_json, None);
    assert_eq!(report.retained_footprint().unwrap().shared_roots, 0);
    assert_eq!(
        report.configuration.channel.taps.capacity(),
        report.configuration.channel.taps.len()
    );
    assert_eq!(report.options.retained_taps.capacity(), 0);
    assert_eq!(report.grants.capacity(), report.grants.len());
    for grant in &report.grants {
        assert_eq!(grant.words.capacity(), grant.words.len());
        for word in &grant.words {
            assert_eq!(word.payload.capacity(), word.payload.len());
            assert_eq!(
                word.decoded.information_bits.capacity(),
                word.decoded.information_bits.len()
            );
            assert_eq!(
                word.decoded.mother_codeword.capacity(),
                word.decoded.mother_codeword.len()
            );
        }
    }
}

fn rejected_dimension(dimension: usize) {
    let mut session = session(DOCUMENT, true);
    let accepted = session.retained_footprint().unwrap();
    let preview = session.preview().unwrap();
    let source = session.document().unwrap().to_json().unwrap();
    let bound = session.batch_report_retained_bound().unwrap();
    let mut insufficient = bound;
    match dimension {
        0 => insufficient.payload_bytes = bound.payload_bytes.checked_sub(1).unwrap(),
        1 => insufficient.ordered_entries = bound.ordered_entries.checked_sub(1).unwrap(),
        2 => insufficient.shared_roots = bound.shared_roots.checked_sub(1).unwrap(),
        _ => unreachable!(),
    }
    let control = RunControl::new();
    let progress = control.progress();
    assert_eq!(
        session
            .run_with_control_bounded(&control, insufficient)
            .unwrap_err()
            .code(),
        "ResourceLimit"
    );
    assert_eq!(control.progress(), progress);
    assert!(!control.is_cancelled());
    assert_eq!(session.retained_footprint().unwrap(), accepted);
    assert_eq!(session.preview().unwrap(), preview);
    assert_eq!(session.document().unwrap().to_json().unwrap(), source);
    let report = session.run_with_control_bounded(&control, bound).unwrap();
    assert!(report.reference_recovered);
    assert_fits(report.retained_footprint().unwrap(), bound);
}

#[test]
fn one_byte_over_policy_rejects_before_source_and_control_claim() {
    rejected_dimension(0);
}
#[test]
fn one_ordered_entry_over_policy_rejects_before_source_and_control_claim() {
    rejected_dimension(1);
}
#[test]
fn one_shared_root_over_policy_rejects_before_source_and_control_claim() {
    rejected_dimension(2);
}

#[test]
fn document_observed_exact_bound_preserves_source_bytes_hashes_and_actual_metrics() {
    let mut session = session(DOCUMENT, true);
    let original_hash = session.document().unwrap().original_sha256();
    let report = check_legacy_parity(&mut session);
    assert_eq!(
        report.source_configuration_original.as_deref(),
        Some(DOCUMENT)
    );
    assert_eq!(report.source_configuration_sha256, original_hash);
    assert!(report.receiver_observations().retained_frames > 0);
    assert!(report.soft_observations().retained_cells > 0);
    assert!(report.soft_observations().retained_words > 0);
    assert_eq!(
        report
            .source_configuration_original
            .as_ref()
            .unwrap()
            .capacity(),
        DOCUMENT.len()
    );
    assert_eq!(
        report
            .source_configuration_json
            .as_ref()
            .unwrap()
            .capacity(),
        report.source_configuration_json.as_ref().unwrap().len()
    );
}

#[test]
fn synthetic_playback_nested_banks_targets_caches_and_missing_values_match_legacy() {
    check_legacy_parity(&mut session(PLAYBACK_DOCUMENT, true));
}

#[test]
fn second_numerology_observed_complete_report_fits_same_public_contract() {
    check_legacy_parity(&mut session(SECOND_NUMEROLOGY_DOCUMENT, true));
}

#[test]
fn measured_observed_context_and_provenance_match_legacy_without_source_changes() {
    let document = ConfigurationDocument::from_json(DOCUMENT).unwrap();
    let (configuration, options) = document.prepare().unwrap();
    let mut session = Simulation::new(options.clone()).unwrap();
    session
        .configure_measured(
            MeasuredConfiguration::new(
                configuration,
                options,
                document,
                MEASURED_PROVENANCE.into(),
            )
            .unwrap(),
        )
        .unwrap();
    session
        .set_receiver_observation_policy(
            ReceiverObservationPolicy::new(ESTIMATE_FRAMES, ESTIMATE_SCALAR_SLOTS).unwrap(),
        )
        .unwrap();
    let report = check_legacy_parity(&mut session);
    assert_eq!(
        report.measurement_provenance_json.as_deref(),
        Some(MEASURED_PROVENANCE)
    );
    assert_eq!(
        report.source_configuration_original.as_deref(),
        Some(DOCUMENT)
    );
}

#[test]
fn pre_cancelled_bounded_report_preserves_all_unstarted_grants_and_source() {
    let mut session = session(DOCUMENT, true);
    let accepted = session.retained_footprint().unwrap();
    let bound = session.batch_report_retained_bound().unwrap();
    let control = RunControl::new();
    control.cancel();
    let report = session.run_with_control_bounded(&control, bound).unwrap();
    assert_eq!(report.status, RunStatus::Cancelled);
    assert!(report.grants.is_empty());
    assert_eq!(report.not_started_grants.len(), report.options.grants);
    assert_eq!(
        report.not_started_grants.capacity(),
        report.not_started_grants.len()
    );
    assert_eq!(report.generated_source_bits, 0);
    assert_eq!(report.compared_source_bits, 0);
    assert_eq!(session.retained_footprint().unwrap(), accepted);
    assert_fits(report.retained_footprint().unwrap(), bound);
}

#[test]
fn runtime_cancel_keeps_completed_prefix_within_pre_admitted_bound() {
    let document = ConfigurationDocument::from_json(DOCUMENT).unwrap();
    let (configuration, mut options) = document.prepare().unwrap();
    options.grants = CANCELLABLE_GRANTS;
    let mut session = Simulation::new(options).unwrap();
    session.configure(configuration).unwrap();
    let bound = session.batch_report_retained_bound().unwrap();
    let control = RunControl::new();
    let worker_control = control.clone();
    let worker = thread::spawn(move || {
        session
            .run_with_control_bounded(&worker_control, bound)
            .unwrap()
    });
    let deadline = Instant::now() + Duration::from_secs(ASYNC_PROGRESS_DEADLINE_SECONDS);
    loop {
        if control.progress().compared_bits > 0 {
            control.cancel();
            break;
        }
        assert!(Instant::now() < deadline, "no bounded completed progress");
        thread::yield_now();
    }
    let report = worker.join().unwrap();
    assert_eq!(report.status, RunStatus::Cancelled);
    assert!(report.compared_source_bits > 0);
    assert!(report.grants.iter().any(|grant| !grant.words.is_empty()));
    assert_eq!(
        report.compared_source_bits,
        report
            .grants
            .iter()
            .map(|grant| grant.compared_bits)
            .sum::<usize>()
    );
    assert_fits(report.retained_footprint().unwrap(), bound);
}

#[test]
fn no_transmission_grants_preserve_legacy_source_and_unavailable_observations() {
    let document = ConfigurationDocument::from_json(DOCUMENT).unwrap();
    let (configuration, options) = document.prepare().unwrap();
    let base = &configuration.us_configuration;
    let original = base.selected_profile();
    let minislots = original
        .minislots()
        .iter()
        .map(|slot| Minislot {
            first_carrier: slot.first_carrier,
            loading: 0,
            pilot_pattern: 0,
        })
        .collect();
    let profile = Profile::new(original.id(), original.iuc(), original.seed(), minislots).unwrap();
    let us = UsConfiguration::new(
        base.waveform().clone(),
        vec![profile],
        base.grant().clone(),
        base.excluded().to_vec(),
    )
    .unwrap();
    let configuration =
        ExperimentConfig::new(us, configuration.source_seed, configuration.receiver).unwrap();
    let mut session = Simulation::new(options).unwrap();
    session.configure(configuration).unwrap();
    let report = check_legacy_parity(&mut session);
    assert_eq!(report.generated_source_bits, 0);
    assert_eq!(report.compared_source_bits, 0);
    assert!(report.grants.iter().all(|grant| grant.words.is_empty()));
    assert!(!report.reference_recovered);
}

#[test]
fn pilot_linear_observed_complete_report_preserves_legacy_science_and_bound() {
    const LINEAR_ESTIMATOR_NAME: &str = "frame_pilot_linear";
    let mut document = ConfigurationDocument::from_json(DOCUMENT).unwrap();
    document.simulation.estimator = LINEAR_ESTIMATOR_NAME.into();
    let (_, options) = document.prepare().unwrap();
    let mut session = Simulation::new(options).unwrap();
    session.configure_document(document).unwrap();
    session
        .set_receiver_observation_policy(
            ReceiverObservationPolicy::new(ESTIMATE_FRAMES, ESTIMATE_SCALAR_SLOTS)
                .unwrap()
                .with_soft_metrics(
                    SoftObservationPolicy::new(SOFT_CELLS, SOFT_WORDS, SOFT_WORD_LLRS).unwrap(),
                ),
        )
        .unwrap();
    let report = check_legacy_parity(&mut session);
    assert_eq!(report.status, RunStatus::Completed);
    assert!(report.reference_recovered);
    assert!(matches!(
        report.configuration.receiver.estimator(),
        Estimator::FramePilotLinear { .. }
    ));
    assert!(report.receiver_observations().retained_frames > 0);
    assert!(report.soft_observations().retained_cells > 0);
    assert!(report.soft_observations().retained_words > 0);
}
