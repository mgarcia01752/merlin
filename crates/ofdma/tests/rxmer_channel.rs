/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Residual actual-waveform proof with independent physical-bin RNG draw order.
mod common;
use docsis_ofdma::{
    channel::RxmerChannel,
    configuration::Numerology,
    measurements::*,
    resources::ResourcePlan,
    signal_processing::Transform,
    types::{Identity, Iq, SampleBlock},
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::StandardNormal;
use std::sync::Arc;

#[test]
fn every_bin_draw_order_actual_useful_samples_and_chunk_retry_invariance() {
    let config = Arc::new(common::configuration(Numerology::Khz50, 4, 5, 2, 8));
    let sources = vec![
        SyntheticRxmer::new(&config, 1, vec![Some(20.0); 2048]).unwrap(),
        SyntheticRxmer::new(&config, 2, vec![Some(30.0); 2048]).unwrap(),
    ];
    let playback = RxmerPlayback::new(
        &config,
        sources,
        RxmerPolicy::Hold {
            offsets: vec![0, 6],
        },
    )
    .unwrap();
    let identity = Identity {
        run: 1,
        generation: 1,
        grant: 7,
    };
    let seed = [42; 32];
    let mut whole = RxmerChannel::new(config.clone(), playback.clone(), identity, seed).unwrap();
    let samples = 12 * config.waveform().stride() + 64;
    let input = vec![Iq::default(); samples];
    let first = whole
        .push(&SampleBlock {
            identity,
            start: 73,
            samples: input.clone(),
        })
        .unwrap();
    assert!(whole.finish().unwrap().samples.is_empty());
    assert!(whole.finish().unwrap().samples.is_empty());
    let mut split = RxmerChannel::new(config.clone(), playback, identity, seed).unwrap();
    assert!(split
        .push(&SampleBlock {
            identity,
            start: 74,
            samples: vec![Iq::default()]
        })
        .is_err());
    let mut output = Vec::new();
    for (i, part) in input.chunks(997).enumerate() {
        output.extend(
            split
                .push(&SampleBlock {
                    identity,
                    start: 73 + (i * 997) as u64,
                    samples: part.to_vec(),
                })
                .unwrap()
                .samples,
        );
    }
    split.finish().unwrap();
    assert_eq!(first.samples, output);
    let mut reference = ChaCha8Rng::from_seed(seed);
    let transform = Transform::new(Numerology::Khz50);
    let resources = ResourcePlan::new(&config).unwrap();
    for symbol in 0..12 {
        let start = symbol * config.waveform().stride() + 192;
        let grid = transform.forward(&output[start..start + 2048]).unwrap();
        for (bin, actual) in grid.iter().enumerate() {
            let gi: f64 = reference.sample(StandardNormal);
            let gq: f64 = reference.sample(StandardNormal);
            let active = resources
                .visits
                .iter()
                .any(|cell| cell.symbol == symbol % 6 && cell.carrier == bin);
            let rho: f64 = if symbol < 6 { 0.01 } else { 0.001 };
            let expected = if active {
                Iq {
                    re: gi * (rho / 2.0).sqrt(),
                    im: gq * (rho / 2.0).sqrt(),
                }
            } else {
                Iq::default()
            };
            // Includes boosted pilots: no pilot gain multiplies the error power.
            assert!(
                (*actual - expected).power() < 1e-26,
                "symbol={symbol} bin={bin}"
            );
        }
    }
}
