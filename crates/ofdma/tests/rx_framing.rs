/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Explicit CP/useful gaps, truncated observation and retained cancellation words.
mod common;
use docsis_ofdma::{
    configuration::{Estimator, Numerology, ReceiverConfig},
    receiver::Receiver,
    signal_processing::PreEqBank,
    source::Prbs15,
    transmitter::Transmitter,
    types::{Identity, SampleBlock},
};
use std::sync::Arc;

#[test]
fn cp_gap_is_discarded_but_useful_gap_is_missing_without_sliding() {
    for useful_missing in [false, true] {
        let config = Arc::new(common::configuration(Numerology::Khz50, 4, 1, 1, 8));
        let id = Identity {
            run: 1,
            generation: 1,
            grant: 7,
        };
        let mut source = Prbs15::new(1).unwrap();
        let mut tx =
            Transmitter::new(config.clone(), id, &mut source, PreEqBank::unity(&config)).unwrap();
        let frame = tx.next_frame().unwrap().unwrap();
        let mut rx = Receiver::new(
            config,
            id,
            ReceiverConfig::new(Estimator::FrameFlatLs, 0.02, 1e-12, 10).unwrap(),
        )
        .unwrap();
        let skipped = 192 + usize::from(useful_missing);
        assert!(rx.gap(73, skipped).unwrap().is_empty());
        let block = SampleBlock {
            identity: id,
            start: 73 + skipped as u64,
            samples: frame.block.samples[skipped..].to_vec(),
        };
        let words = rx.push(&block).unwrap();
        rx.finish().unwrap();
        assert_eq!(rx.missing_symbols(), usize::from(useful_missing));
        assert_eq!(
            words.iter().map(|word| word.payload.len()).sum::<usize>(),
            tx.plan().source_bits
        );
    }
}

#[test]
fn cancellation_mid_frame_returns_completed_words_and_actual_failed_decisions() {
    let config = Arc::new(common::configuration(Numerology::Khz50, 4, 1, 1, 8));
    let id = Identity {
        run: 1,
        generation: 1,
        grant: 7,
    };
    let mut source = Prbs15::new(1).unwrap();
    let mut tx =
        Transmitter::new(config.clone(), id, &mut source, PreEqBank::unity(&config)).unwrap();
    let frame = tx.next_frame().unwrap().unwrap();
    let total_words = tx.plan().codewords.len();
    let mut rx = Receiver::new(
        config,
        id,
        ReceiverConfig::new(Estimator::FrameFlatLs, 0.02, 1e-12, 10).unwrap(),
    )
    .unwrap();
    let mut probes = 0;
    let batch = rx
        .push_with_control(&frame.block, &mut || {
            probes += 1;
            probes >= 250
        })
        .unwrap();
    assert!(batch.interrupted);
    assert!(!batch.words.is_empty());
    assert!(batch.words.len() < total_words);
    assert!(batch
        .words
        .iter()
        .all(|word| word.decoded.converged && word.decoded.parity_valid));
    assert!(rx.push(&frame.block).is_err());
}
