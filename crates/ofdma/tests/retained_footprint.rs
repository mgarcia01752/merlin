/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent current-root capacity/delta checks; not future-output bounds.
use docsis_ofdma::{api::*, simulation::RunFailure};
use merlin_api::retention_support::retained_footprint;
use std::mem::size_of;

const SOURCE_DOCUMENT: &[u8] = include_bytes!("../../../examples/cli/upstream/us-50khz.json");

// Spare-capacity probes are metadata anchors, not scientific resource limits.
const SPARE_VECTOR_ELEMENTS: usize = 7;
const SPARE_STRING_BYTES: usize = 11;

fn session() -> Simulation {
    let document = ConfigurationDocument::from_json(SOURCE_DOCUMENT).unwrap();
    let (_, options) = document.prepare().unwrap();
    let mut session = Simulation::new(options).unwrap();
    session.configure_document(document).unwrap();
    session
}

fn grow_vector<Value>(values: &mut Vec<Value>) -> usize {
    let before = values.capacity();
    values.reserve_exact(before + SPARE_VECTOR_ELEMENTS);
    (values.capacity() - before) * size_of::<Value>()
}

fn grow_string(value: &mut String) -> usize {
    let before = value.capacity();
    value.reserve_exact(before + SPARE_STRING_BYTES);
    value.capacity() - before
}

#[test]
fn idle_root_fixed_storage_and_caller_option_spare_capacity_are_exact() {
    let mut options = ExperimentOptions {
        retained_taps: Vec::new(),
        ..ExperimentOptions::default()
    };
    let first = Simulation::new(options.clone()).unwrap();
    assert_eq!(
        first.retained_footprint().unwrap().payload_bytes,
        size_of::<Simulation>()
    );
    assert_eq!(first.retained_footprint().unwrap().ordered_entries, 0);
    assert_eq!(first.retained_footprint().unwrap().shared_roots, 0);
    options.retained_taps.reserve_exact(SPARE_VECTOR_ELEMENTS);
    let requested = options.retained_taps.capacity() * size_of::<Tap>();
    let spare = Simulation::new(options).unwrap();
    assert_eq!(
        spare.retained_footprint().unwrap().payload_bytes,
        size_of::<Simulation>() + requested
    );
}

#[test]
fn source_document_spare_strings_vectors_and_original_shared_bytes_are_retained() {
    let mut document = ConfigurationDocument::from_json(SOURCE_DOCUMENT).unwrap();
    let before_json = document.to_json().unwrap();
    let before = retained_footprint(&document).unwrap();
    let delta = grow_string(&mut document.presets[0].description)
        + grow_string(&mut document.selected_preset)
        + grow_vector(&mut document.presets)
        + grow_vector(&mut document.simulation.minislots)
        + grow_vector(&mut document.simulation.retained_taps)
        + grow_string(&mut document.simulation.retained_taps[0]);
    let after = retained_footprint(&document).unwrap();
    assert_eq!(after.payload_bytes - before.payload_bytes, delta);
    assert_eq!(before.shared_roots, 1);
    assert_eq!(after.shared_roots, before.shared_roots);
    assert_eq!(document.original_bytes(), Some(SOURCE_DOCUMENT));
    assert_eq!(document.to_json().unwrap(), before_json);
}

#[test]
fn measured_provenance_capacity_survives_actual_session_admission() {
    let document = ConfigurationDocument::from_json(SOURCE_DOCUMENT).unwrap();
    let (configuration, options) = document.prepare().unwrap();
    let provenance =
        String::from("{\"format\":\"merlin.upstream.measured-provenance\",\"schema_version\":1}");
    let mut spare_provenance = provenance.clone();
    grow_string(&mut spare_provenance);
    let delta = spare_provenance.capacity() - provenance.capacity();

    // Clone both contexts so unrelated parser/configuration spare capacity matches.
    let admit = |provenance| {
        let mut session = Simulation::new(options.clone()).unwrap();
        session
            .configure_measured(
                MeasuredConfiguration::new(
                    configuration.clone(),
                    options.clone(),
                    document.clone(),
                    provenance,
                )
                .unwrap(),
            )
            .unwrap();
        session
    };
    let first = admit(provenance);
    let spare = admit(spare_provenance);
    assert_eq!(
        spare.retained_footprint().unwrap().payload_bytes
            - first.retained_footprint().unwrap().payload_bytes,
        delta
    );
}

#[test]
fn actual_report_public_spare_buffers_and_decoder_decisions_are_counted() {
    let mut session = session();
    let mut report = session.run().unwrap();
    assert!(report.reference_recovered);
    let before_json = report.to_json().unwrap();
    let before = report.retained_footprint().unwrap();
    let word = &mut report.grants[0].words[0];
    let word_delta = grow_vector(&mut word.payload)
        + grow_vector(&mut word.decoded.information_bits)
        + grow_vector(&mut word.decoded.mother_codeword);
    let delta = word_delta
        + grow_vector(&mut report.grants[0].words)
        + grow_vector(&mut report.grants)
        + grow_vector(&mut report.not_started_grants)
        + grow_vector(&mut report.planned.profile_ids)
        + grow_vector(&mut report.planned.grant_plan.codewords)
        + grow_vector(&mut report.options.retained_taps)
        + grow_vector(&mut report.configuration.channel.taps)
        + grow_string(report.source_configuration_json.as_mut().unwrap())
        + grow_string(report.source_configuration_sha256.as_mut().unwrap())
        + grow_vector(report.source_configuration_original.as_mut().unwrap());
    let after = report.retained_footprint().unwrap();
    assert_eq!(after.payload_bytes - before.payload_bytes, delta);
    assert_eq!(after.ordered_entries, before.ordered_entries);
    assert_eq!(after.shared_roots, before.shared_roots);
    assert_eq!(report.to_json().unwrap(), before_json);
    let prior = after;
    let diagnostic = String::with_capacity(SPARE_STRING_BYTES);
    let charge = diagnostic.capacity();
    report.failure = Some(RunFailure {
        diagnostic,
        category: "static-category-is-borrowed",
        field: Some("static-field-is-borrowed"),
    });
    assert_eq!(
        report.retained_footprint().unwrap().payload_bytes - prior.payload_bytes,
        charge
    );
}

#[test]
fn complete_and_pre_cancelled_actual_results_have_passive_repeatable_metadata() {
    let mut session = session();
    let preview = session.preview().unwrap();
    let source = session.document().unwrap().to_json().unwrap();
    let footprint = session.retained_footprint().unwrap();
    assert_eq!(session.retained_footprint().unwrap(), footprint);
    assert_eq!(session.preview().unwrap(), preview);
    assert_eq!(session.document().unwrap().to_json().unwrap(), source);
    let completed = session.run().unwrap();
    assert!(completed.reference_recovered);
    let original = completed.to_json().unwrap();
    let complete_size = completed.retained_footprint().unwrap();
    assert_eq!(completed.retained_footprint().unwrap(), complete_size);
    assert_eq!(completed.to_json().unwrap(), original);
    assert_eq!(session.retained_footprint().unwrap(), footprint);
    let control = RunControl::new();
    control.cancel();
    let partial = session.run_with_control(&control).unwrap();
    assert_eq!(partial.status, RunStatus::Cancelled);
    assert!(partial.grants.is_empty());
    assert!(!partial.not_started_grants.is_empty());
    let partial_size = partial.retained_footprint().unwrap();
    assert!(complete_size.payload_bytes > partial_size.payload_bytes);
    assert!(partial_size.payload_bytes > size_of::<BatchReport>());
    assert_eq!(partial.retained_footprint().unwrap(), partial_size);
    assert_eq!(session.preview().unwrap(), preview);
}

#[test]
fn actual_receiver_and_soft_shared_stores_are_included_without_query_copies() {
    const RETAINED_ESTIMATE_FRAMES: usize = 2;
    const RETAINED_SOFT_CELLS: usize = 3;
    const RETAINED_SOFT_WORDS: usize = 1;
    let mut session = session();
    let carriers = session
        .configuration()
        .unwrap()
        .us_configuration
        .waveform()
        .numerology()
        .fft_len();

    // Every retained complex estimate owns its real and imaginary scalar slots.
    let estimate_scalars = RETAINED_ESTIMATE_FRAMES * carriers * 2;
    let word_llrs = session.preview().unwrap().grant_plan.codewords[0].transmitted_bits;
    session
        .set_receiver_observation_policy(
            ReceiverObservationPolicy::new(RETAINED_ESTIMATE_FRAMES, estimate_scalars)
                .unwrap()
                .with_soft_metrics(
                    SoftObservationPolicy::new(RETAINED_SOFT_CELLS, RETAINED_SOFT_WORDS, word_llrs)
                        .unwrap(),
                ),
        )
        .unwrap();
    let report = session.run().unwrap();
    assert!(report.reference_recovered);
    let footprint = report.retained_footprint().unwrap();
    assert!(
        footprint.shared_roots
            >= RETAINED_ESTIMATE_FRAMES + RETAINED_SOFT_CELLS + RETAINED_SOFT_WORDS
    );
    assert_eq!(
        footprint.ordered_entries,
        report.options.retained_taps.len()
    );
    let json = report.to_json().unwrap();
    assert_eq!(report.retained_footprint().unwrap(), footprint);
    assert_eq!(report.to_json().unwrap(), json);
}
