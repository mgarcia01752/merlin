/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Independent causal convolution, integer-delay, phase and AWGN arithmetic.
use docsis_ofdma::{
    channel::{Channel, ChannelConfig},
    types::{Identity, Iq, SampleBlock},
};
use std::f64::consts::FRAC_PI_2;

fn process(config: ChannelConfig, values: &[Iq], chunk: usize) -> Vec<Iq> {
    let id = Identity {
        run: 1,
        generation: 1,
        grant: 0,
    };
    let mut channel = Channel::new(config, id, 100).unwrap();
    let mut output = Vec::new();
    for (i, part) in values.chunks(chunk).enumerate() {
        output.extend(
            channel
                .push(&SampleBlock {
                    identity: id,
                    start: 100 + (i * chunk) as u64,
                    samples: part.to_vec(),
                })
                .unwrap()
                .samples,
        );
    }
    output.extend(channel.finish().unwrap().samples);
    assert!(channel.finish().unwrap().samples.is_empty());
    output
}

#[test]
fn literal_convolution_delay_phase_and_exact_drain_order() {
    let input: Vec<_> = [1.0, 2.0, 3.0]
        .into_iter()
        .map(|re| Iq { re, im: 0.0 })
        .collect();
    let response = ChannelConfig {
        taps: vec![Iq { re: 1.0, im: 0.0 }, Iq { re: 2.0, im: 0.0 }],
        ..ChannelConfig::default()
    };
    assert_eq!(
        process(response.clone(), &input, 3)
            .iter()
            .map(|z| z.re)
            .collect::<Vec<_>>(),
        vec![1.0, 4.0, 7.0, 6.0]
    );
    assert_eq!(
        process(response.clone(), &input, 1),
        process(response, &input, 3)
    );
    let delay = ChannelConfig {
        delay_samples: 2,
        ..ChannelConfig::default()
    };
    assert_eq!(
        process(delay, &input, 1)
            .iter()
            .map(|z| z.re)
            .collect::<Vec<_>>(),
        vec![0.0, 0.0, 1.0, 2.0, 3.0]
    );
    let rotated = process(
        ChannelConfig {
            phase_radians: FRAC_PI_2,
            ..ChannelConfig::default()
        },
        &[Iq { re: 2.0, im: 3.0 }],
        1,
    );
    assert!((rotated[0] - Iq { re: -3.0, im: 2.0 }).power() < 1e-28);
    let tone = process(
        ChannelConfig {
            frequency_offset_hz: 25_600_000.0,
            ..ChannelConfig::default()
        },
        &[Iq { re: 1.0, im: 0.0 }; 4],
        2,
    );
    for (actual, expected) in tone.iter().zip([
        Iq { re: 1.0, im: 0.0 },
        Iq { re: 0.0, im: 1.0 },
        Iq { re: -1.0, im: 0.0 },
        Iq { re: 0.0, im: -1.0 },
    ]) {
        assert!((*actual - expected).power() < 1e-28);
    }
    assert!(ChannelConfig {
        phase_radians: f64::NAN,
        ..ChannelConfig::default()
    }
    .validate()
    .is_err());
    assert!(ChannelConfig {
        frequency_offset_hz: f64::INFINITY,
        ..ChannelConfig::default()
    }
    .validate()
    .is_err());
    assert!(ChannelConfig {
        noise_variance: -1.0,
        ..ChannelConfig::default()
    }
    .validate()
    .is_err());
}

#[test]
fn exact_unity_signed_zeros_and_pinned_noise_chunk_invariance() {
    let input = vec![Iq { re: -0.0, im: 0.0 }, Iq { re: 2.0, im: -3.0 }];
    let output = process(ChannelConfig::default(), &input, 1);
    for (left, right) in output.iter().zip(&input) {
        assert_eq!(
            (left.re.to_bits(), left.im.to_bits()),
            (right.re.to_bits(), right.im.to_bits())
        );
    }
    let noise = ChannelConfig {
        noise_variance: 0.2,
        noise_seed: [42; 32],
        ..ChannelConfig::default()
    };
    let zero = vec![Iq::default(); 65536];
    let one = process(noise.clone(), &zero, 997);
    assert_eq!(one, process(noise, &zero, 4096));
    let mean_i = one.iter().map(|z| z.re).sum::<f64>() / one.len() as f64;
    let mean_q = one.iter().map(|z| z.im).sum::<f64>() / one.len() as f64;
    let power = one.iter().map(|z| z.power()).sum::<f64>() / one.len() as f64;
    assert!(mean_i.abs() < 0.01 && mean_q.abs() < 0.01);
    assert!((power - 0.2).abs() < 0.01);
}

#[test]
fn rejected_blocks_preserve_history_rng_cursor_and_terminal_state() {
    let identity = Identity {
        run: 1,
        generation: 1,
        grant: 7,
    };
    let config = ChannelConfig {
        taps: vec![Iq { re: 1.0, im: 0.0 }, Iq { re: 0.1, im: 0.2 }],
        noise_variance: 0.01,
        noise_seed: [17; 32],
        ..ChannelConfig::default()
    };
    let mut reference = Channel::new(config.clone(), identity, 10).unwrap();
    let mut channel = Channel::new(config, identity, 10).unwrap();
    let empty = SampleBlock {
        identity,
        start: 10,
        samples: vec![],
    };
    assert_eq!(channel.push(&empty).unwrap().samples, Vec::<Iq>::new());
    for bad in [
        SampleBlock {
            identity,
            start: 11,
            samples: vec![Iq::default()],
        },
        SampleBlock {
            identity: Identity { run: 2, ..identity },
            start: 10,
            samples: vec![Iq::default()],
        },
        SampleBlock {
            identity,
            start: 10,
            samples: vec![Iq {
                re: f64::NAN,
                im: 0.0,
            }],
        },
        SampleBlock {
            identity,
            start: 10,
            samples: vec![Iq::default(); 65537],
        },
    ] {
        assert!(channel.push(&bad).is_err());
    }
    let valid = SampleBlock {
        identity,
        start: 10,
        samples: vec![Iq { re: 0.2, im: -0.3 }; 8],
    };
    assert_eq!(
        channel.push(&valid).unwrap(),
        reference.push(&valid).unwrap()
    );
    assert!(channel.push(&valid).is_err());
    assert_eq!(channel.finish().unwrap(), reference.finish().unwrap());
    assert!(channel.finish().unwrap().samples.is_empty());
    assert!(channel
        .push(&SampleBlock {
            identity,
            start: 18,
            samples: vec![]
        })
        .is_err());
    let mut overflow = Channel::new(ChannelConfig::default(), identity, u64::MAX).unwrap();
    assert!(overflow
        .push(&SampleBlock {
            identity,
            start: u64::MAX,
            samples: vec![Iq::default()]
        })
        .is_err());
    let tiny = Iq {
        re: f64::from_bits(1),
        im: -f64::from_bits(1),
    };
    let result = process(ChannelConfig::default(), &[tiny], 1);
    assert_eq!(result[0].re.to_bits(), tiny.re.to_bits());
    assert_eq!(result[0].im.to_bits(), tiny.im.to_bits());
}
