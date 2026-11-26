/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Deterministic transaction/cancellation/finish boundaries without timing-based stop races.
use super::*;
use crate::{api::Simulation, receiver::Receiver, source::Prbs15, transmitter::Transmitter};

const RETAINED_FRAME_BUDGET: usize = 3;
const RETENTION_SCALAR_BUDGET: usize = 262_144;
const OBSERVATION_RUN_ID: u64 = 41;
const OBSERVATION_GENERATION: u64 = 53;
const EXPECTED_GENERIC_GRANT_ID: u64 = 1;
const EXPECTED_GENERIC_FIRST_FRAME: u64 = 0;
const EXPECTED_GENERIC_FRAMES: usize = 2;
const GENERIC_50KHZ: &[u8] = include_bytes!("../../../../examples/cli/upstream/us-50khz.json");

fn setup() -> (crate::simulation::ExperimentConfig, Identity, Receiver) {
    let mut session = Simulation::new(Default::default()).unwrap();
    session.configure_json(GENERIC_50KHZ).unwrap();
    let config = session.configuration().unwrap().clone();
    let identity = Identity {
        run: OBSERVATION_RUN_ID,
        generation: OBSERVATION_GENERATION,
        grant: EXPECTED_GENERIC_GRANT_ID,
    };
    let mut receiver = Receiver::new(
        Arc::new(config.us_configuration.clone()),
        identity,
        config.receiver.clone(),
    )
    .unwrap();
    receiver.replace_estimate_store(ReceiverEstimateStore::new(
        ReceiverObservationPolicy::new(RETAINED_FRAME_BUDGET, RETENTION_SCALAR_BUDGET).unwrap(),
    ));
    (config, identity, receiver)
}

#[test]
fn invalid_transaction_and_mid_frame_cancel_preserve_previous_completed_estimate() {
    let (config, identity, mut receiver) = setup();
    let mut source = Prbs15::new(config.source_seed).unwrap();
    let mut transmitter = Transmitter::new(
        Arc::new(config.us_configuration),
        identity,
        &mut source,
        config.pre_eq,
    )
    .unwrap();
    let first = transmitter.next_frame().unwrap().unwrap();
    let first_words = receiver.push(&first.block).unwrap();
    assert!(!first_words.is_empty());
    assert_eq!(receiver.frames(), 1);
    let second = transmitter.next_frame().unwrap().unwrap();
    let mut invalid = second.block.clone();
    invalid.samples[0].re = f64::NAN;
    let before = receiver.replace_estimate_store(None);
    receiver.replace_estimate_store(before.clone());
    assert!(receiver.push(&invalid).is_err());
    assert_eq!(receiver.replace_estimate_store(before.clone()), before);
    // Stop at the first cell of the second frame, after its samples reached the receiver.
    let partial = receiver
        .push_with_control(&second.block, &mut || true)
        .unwrap();
    assert!(partial.interrupted);
    assert!(partial.words.is_empty());
    assert_eq!(receiver.frames(), 1);
    let retained = receiver.replace_estimate_store(None).unwrap();
    assert_eq!(retained.summary().completed_frames, 1);
    assert_eq!(retained.summary().retained_frames, 1);
    assert_eq!(retained.frames[0].frame, EXPECTED_GENERIC_FIRST_FRAME);
    assert!(retained.frames[0].estimates.iter().any(Option::is_some));
}

#[test]
fn finish_retains_complete_missing_frames_as_unavailable_without_inventing_zero_estimates() {
    let (_, _, mut receiver) = setup();
    receiver.finish().unwrap();
    let retained = receiver.replace_estimate_store(None).unwrap();
    assert_eq!(receiver.frames(), EXPECTED_GENERIC_FRAMES);
    assert_eq!(retained.summary().completed_frames, EXPECTED_GENERIC_FRAMES);
    assert_eq!(retained.summary().retained_frames, EXPECTED_GENERIC_FRAMES);
    assert!(retained
        .frames
        .iter()
        .all(|frame| frame.estimates.iter().all(Option::is_none)));
    receiver.finish().unwrap();
    assert_eq!(receiver.frames(), EXPECTED_GENERIC_FRAMES);
}

#[test]
fn soft_cancellation_keeps_completed_words_and_computed_cells_without_exporting_partial_word() {
    const SOFT_CELL_BUDGET: usize = 1024;
    const SOFT_WORD_BUDGET: usize = 16;
    const WORD_INPUT_BUDGET: usize = 262_144;
    // In this two-frame fixture, this stops in frame two after one more complete word.
    const CANCEL_AFTER_RECEIVER_PROBES: usize = 250;
    let (config, identity, mut receiver) = setup();
    receiver.replace_estimate_store(ReceiverEstimateStore::new(
        ReceiverObservationPolicy::default().with_soft_metrics(
            crate::instruments::SoftObservationPolicy::new(
                SOFT_CELL_BUDGET,
                SOFT_WORD_BUDGET,
                WORD_INPUT_BUDGET,
            )
            .unwrap(),
        ),
    ));
    let mut source = Prbs15::new(config.source_seed).unwrap();
    let mut transmitter = Transmitter::new(
        Arc::new(config.us_configuration),
        identity,
        &mut source,
        config.pre_eq,
    )
    .unwrap();
    let total_words = transmitter.plan().codewords.len();
    let first = transmitter.next_frame().unwrap().unwrap();
    let first_words = receiver.push(&first.block).unwrap();
    assert!(!first_words.is_empty());
    let second = transmitter.next_frame().unwrap().unwrap();
    let mut probes = 0;
    let partial = receiver
        .push_with_control(&second.block, &mut || {
            probes += 1;
            probes >= CANCEL_AFTER_RECEIVER_PROBES
        })
        .unwrap();
    assert!(partial.interrupted);
    assert!(!partial.words.is_empty());
    let completed = first_words.len() + partial.words.len();
    assert!(completed < total_words);
    let observer = receiver.replace_estimate_store(None).unwrap();
    let data = serde_json::to_value(observer.soft.unwrap()).unwrap();
    assert_eq!(data["completed_words"], completed);
    assert_eq!(data["words"].as_array().unwrap().len(), completed);
    assert!(data["computed_cells"].as_u64().unwrap() > 0);
    for word in &partial.words {
        assert_eq!(
            data["words"][word.ordinal]["word"]["word_ordinal"],
            word.ordinal
        );
        assert_eq!(
            data["words"][word.ordinal]["parity_valid"],
            word.decoded.parity_valid
        );
    }
}

#[test]
fn soft_missing_frame_finish_records_actual_erasure_metrics_and_failed_codec_outcomes() {
    const SOFT_CELL_BUDGET: usize = 1024;
    const SOFT_WORD_BUDGET: usize = 16;
    const WORD_INPUT_BUDGET: usize = 262_144;
    let (_, _, mut receiver) = setup();
    receiver.replace_estimate_store(ReceiverEstimateStore::new(
        ReceiverObservationPolicy::default().with_soft_metrics(
            crate::instruments::SoftObservationPolicy::new(
                SOFT_CELL_BUDGET,
                SOFT_WORD_BUDGET,
                WORD_INPUT_BUDGET,
            )
            .unwrap(),
        ),
    ));
    let words = receiver.finish().unwrap();
    assert!(!words.is_empty());
    let observer = receiver.replace_estimate_store(None).unwrap();
    let data = serde_json::to_value(observer.soft.unwrap()).unwrap();
    assert_eq!(data["completed_words"], words.len());
    for cell in data["cells"].as_array().unwrap() {
        assert!(cell["corrected"].is_null());
        assert!(cell["demapper_llrs"]
            .as_array()
            .unwrap()
            .iter()
            .all(|llr| llr == 0.0));
    }
    for word in words {
        assert_eq!(
            data["words"][word.ordinal]["converged"],
            word.decoded.converged
        );
        assert_eq!(
            data["words"][word.ordinal]["parity_valid"],
            word.decoded.parity_valid
        );
    }
}
