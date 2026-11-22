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
