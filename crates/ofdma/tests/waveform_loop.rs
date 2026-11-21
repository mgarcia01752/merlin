/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Internal source→TX→exact channel→received-only RX→independent source observer.
mod common;
use docsis_ofdma::{
    channel::{Channel, ChannelConfig},
    configuration::{Estimator, Numerology, ReceiverConfig},
    receiver::Receiver,
    signal_processing::PreEqBank,
    source::{Prbs15, PrbsVerifier},
    transmitter::Transmitter,
    types::{Identity, Iq, SampleBlock},
};
use std::sync::Arc;

fn run(mode: Numerology, loading: u8, pattern: u8, frames: usize, chunk: usize, phase_bank: bool) {
    let config = Arc::new(common::configuration(mode, loading, pattern, frames, 8));
    let identity = Identity {
        run: 1,
        generation: 1,
        grant: 7,
    };
    let bank = if phase_bank {
        let mut raw = vec![Iq { re: 0.0, im: 2.0 }; mode.fft_len()];
        for &bin in config.excluded() {
            raw[bin] = Iq::default();
        }
        PreEqBank::normalized(&config, raw, 2).unwrap()
    } else {
        PreEqBank::unity(&config)
    };
    let mut source = Prbs15::new(1).unwrap();
    let mut tx = Transmitter::new(config.clone(), identity, &mut source, bank).unwrap();
    assert!(tx.plan().transmit);
    let expected = tx.plan().source_bits;
    let mut channel = Channel::new(
        ChannelConfig::default(),
        identity,
        config.grant().epoch_sample,
    )
    .unwrap();
    let policy = ReceiverConfig::new(Estimator::FrameFlatLs, 0.02, 1e-12, 50).unwrap();
    let mut rx = Receiver::new(config.clone(), identity, policy).unwrap();
    let mut words = Vec::new();
    let mut emitted = 0;
    while let Some(frame) = tx.next_frame().unwrap() {
        for (index, part) in frame.block.samples.chunks(chunk).enumerate() {
            let block = SampleBlock {
                identity,
                start: frame.block.start + (index * chunk) as u64,
                samples: part.to_vec(),
            };
            let received = channel.push(&block).unwrap();
            words.extend(rx.push(&received).unwrap());
            emitted += part.len();
        }
    }
    assert!(channel.finish().unwrap().samples.is_empty());
    words.extend(rx.finish().unwrap());
    assert_eq!(emitted, frames * 6 * config.waveform().stride() + 64);
    assert_eq!(rx.missing_symbols(), 0);
    assert_eq!(rx.frames(), frames);
    let mut observer = PrbsVerifier::new(1).unwrap();
    let mut compared = 0;
    for word in words {
        assert!(
            word.decoded.converged
                && word.decoded.parity_valid
                && word.decoded.shortened_zeros_valid
                && word.information_fill_valid
        );
        assert_eq!(word.payload_offset, compared);
        assert_eq!(
            observer
                .compare(compared as u64, &word.payload)
                .unwrap()
                .bit_errors,
            0
        );
        compared += word.payload.len();
    }
    assert_eq!(compared, expected);
    assert_eq!(source.position(), expected as u64);
}

#[test]
fn both_modes_all_pilot_patterns_and_cross_frame_words_recover() {
    for pattern in 1..=14 {
        run(
            if pattern <= 7 {
                Numerology::Khz50
            } else {
                Numerology::Khz25
            },
            4,
            pattern,
            2,
            997,
            false,
        );
    }
}

#[test]
fn all_upstream_qam_orders_and_received_pilot_phase_recovery() {
    for bits in 2..=12 {
        run(Numerology::Khz50, bits, 1, 1, 4096, false);
    }
    run(Numerology::Khz25, 4, 8, 2, 313, true);
}

#[test]
fn rejected_and_truncated_samples_preserve_real_coverage() {
    let config = Arc::new(common::configuration(Numerology::Khz50, 4, 1, 1, 8));
    let identity = Identity {
        run: 1,
        generation: 1,
        grant: 7,
    };
    let mut source = Prbs15::new(1).unwrap();
    let mut tx = Transmitter::new(
        config.clone(),
        identity,
        &mut source,
        PreEqBank::unity(&config),
    )
    .unwrap();
    let frame = tx.next_frame().unwrap().unwrap();
    let mut rx = Receiver::new(
        config,
        identity,
        ReceiverConfig::new(Estimator::FrameFlatLs, 0.02, 1e-12, 1).unwrap(),
    )
    .unwrap();
    let first = SampleBlock {
        identity,
        start: 73,
        samples: frame.block.samples[..317].to_vec(),
    };
    assert!(rx
        .push(&SampleBlock {
            start: 74,
            ..first.clone()
        })
        .is_err());
    let mut invalid = first.clone();
    invalid.samples[1].re = f64::NAN;
    assert!(rx.push(&invalid).is_err());
    assert!(rx.push(&first).unwrap().is_empty());
    assert!(rx.push(&first).is_err());
    let words = rx.finish().unwrap();
    assert_eq!(rx.missing_symbols(), 6);
    assert!(!words.is_empty());
    assert!(rx.finish().unwrap().is_empty());
    assert!(rx.push(&first).is_err());
}
