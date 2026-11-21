/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent zero-rolloff assembly and configured receive-window anchors.
mod common;
use docsis_ofdma::{
    configuration::{Estimator, Numerology, ReceiverConfig, Waveform},
    receiver::Receiver,
    signal_processing::transmit_window,
    transmitter::BurstAssembler,
    types::{Identity, Iq, SampleBlock},
};
use std::sync::Arc;

#[test]
fn zero_rolloff_is_exact_prefix_then_useful_samples_without_tail() {
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let geometry = Waveform::new(mode, 192, 0, 6, 20_000_000).unwrap();
        assert!(transmit_window(&geometry)
            .iter()
            .all(|weight| *weight == 1.0));
        let length = mode.fft_len();
        let numbered: Vec<_> = (0..length)
            .map(|i| Iq {
                re: i as f64,
                im: -(i as f64),
            })
            .collect();
        let id = Identity {
            run: 1,
            generation: 1,
            grant: 7,
        };
        let mut assembler = BurstAssembler::new(id, geometry, 73, 2).unwrap();
        for symbol in 0..2 {
            let block = assembler.push(&numbered).unwrap();
            assert_eq!(block.start, 73 + symbol * (length + 192) as u64);
            assert_eq!(&block.samples[..192], &numbered[length - 192..]);
            assert_eq!(&block.samples[192..], numbered);
        }
        assert!(assembler.finish().unwrap().samples.is_empty());
    }
}

#[test]
fn received_window_discards_numbered_prefix_and_never_repairs_a_missing_sample() {
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let pattern = if mode == Numerology::Khz50 { 1 } else { 8 };
        let config = Arc::new(common::configuration(mode, 4, pattern, 1, 8));
        let length = mode.fft_len();
        let stride = length + 192;
        let mut samples = vec![Iq::default(); 6 * stride + 64];
        for symbol in 0..6 {
            samples[symbol * stride..symbol * stride + 192].fill(Iq { re: 7.0, im: -9.0 });
        }
        samples[192] = Iq { re: 1.0, im: 0.0 };
        let id = Identity {
            run: 1,
            generation: 1,
            grant: 7,
        };
        for missing in [false, true] {
            let mut receiver = Receiver::new(
                config.clone(),
                id,
                ReceiverConfig::new(Estimator::FrameFlatLs, 0.02, 1e-12, 1).unwrap(),
            )
            .unwrap();
            let skipped = if missing { 193 } else { 0 };
            if missing {
                receiver.gap(73, skipped).unwrap();
            }
            receiver
                .push(&SampleBlock {
                    identity: id,
                    start: 73 + skipped as u64,
                    samples: samples[skipped..].to_vec(),
                })
                .unwrap();
            receiver.finish().unwrap();
            assert_eq!(receiver.missing_symbols(), usize::from(missing));
            assert_eq!(receiver.frames(), 1);
            if missing {
                assert!(receiver.last_grid().is_none());
            } else {
                let expected = (length as f64).sqrt().recip();
                assert!(receiver
                    .last_grid()
                    .unwrap()
                    .iter()
                    .all(|z| (z.re - expected).abs() < 1e-13 && z.im == 0.0));
            }
        }
    }
}
