/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    signal_processing::{
        channel::filter::{ChannelFilterSettings, EchoPath, StreamingChannelFilter},
        waveform::SampleBlock,
    },
    transmission::SubcarrierSpacing,
};

fn plan(
    settings: ChannelFilterSettings,
) -> docsis_ofdm_fec::signal_processing::channel::filter::ChannelFilterPlan {
    settings
        .prepare(
            SubcarrierSpacing::Khz50,
            &vec![[1.0, 0.0]; 4096],
            &vec![true; 4096],
        )
        .unwrap()
        .unwrap()
}
#[test]
fn sparse_echo_delay_and_tail_follow_analytic_impulses_across_chunks() {
    let plan = plan(ChannelFilterSettings::Echoes {
        paths: vec![
            EchoPath {
                delay_samples: 0,
                gain: [1.0, 0.0],
            },
            EchoPath {
                delay_samples: 5,
                gain: [0.0, 0.25],
            },
        ],
    });
    let mut stream = StreamingChannelFilter::new(&plan, 100).unwrap();
    let mut result = stream
        .transfer(&SampleBlock::new(100, vec![[1.0, 0.0], [0.0, 0.0]]).unwrap())
        .unwrap()
        .samples()
        .to_vec();
    assert!(stream
        .transfer(&SampleBlock::new(100, vec![[1.0, 0.0]]).unwrap())
        .is_err());
    result.extend_from_slice(
        stream
            .transfer(&SampleBlock::new(102, vec![[0.0, 0.0]; 2]).unwrap())
            .unwrap()
            .samples(),
    );
    result.extend_from_slice(stream.finish().unwrap().samples());
    assert_eq!(result.len(), 9);
    assert_eq!(result[0], [1.0, 0.0]);
    assert_eq!(result[5], [0.0, 0.25]);
    assert!(result
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != 0 && *index != 5)
        .all(|(_, value)| *value == [0.0, 0.0]));
    assert!(stream.finish().is_err());
}
#[test]
fn dense_captured_filter_fft_matches_independent_direct_convolution_and_chunking() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let size = spacing.fft_samples() as usize;
        let coefficients: Vec<_> = (0..65)
            .map(|index| {
                if index == 0 {
                    [1.0, 0.0]
                } else {
                    [0.001, index as f64 * 1e-5]
                }
            })
            .collect();
        let response: Vec<_> = (0..size)
            .map(|bin| {
                let mut sum = [0.0, 0.0];
                for (delay, gain) in coefficients.iter().enumerate() {
                    let phase =
                        -std::f64::consts::TAU * (bin as f64 - size as f64 / 2.0) * delay as f64
                            / size as f64;
                    sum[0] += gain[0] * phase.cos() - gain[1] * phase.sin();
                    sum[1] += gain[0] * phase.sin() + gain[1] * phase.cos();
                }
                sum
            })
            .collect();
        let settings = ChannelFilterSettings::CapturedFir {
            max_delay_samples: 64,
            causal_shift_samples: 0,
            maximum_rms_error: 1e-9,
            maximum_bin_error: 1e-9,
        };
        let plan = settings
            .prepare(spacing, &response, &vec![true; size])
            .unwrap()
            .unwrap();
        assert!(plan.diagnostics().rms_complex_error < 1e-12);
        let input: Vec<_> = (0..4277)
            .map(|index| [(index as f64 * 0.7).sin(), (index as f64 * 0.2).cos()])
            .collect();
        let mut expected = vec![[0.0, 0.0]; input.len() + 64];
        for (index, point) in input.iter().enumerate() {
            for (delay, gain) in coefficients.iter().enumerate() {
                expected[index + delay][0] += point[0] * gain[0] - point[1] * gain[1];
                expected[index + delay][1] += point[0] * gain[1] + point[1] * gain[0];
            }
        }
        for chunk_size in [4277, 137] {
            let mut stream = StreamingChannelFilter::new(&plan, 0).unwrap();
            let mut result = Vec::new();
            for (chunk, points) in input.chunks(chunk_size).enumerate() {
                result.extend_from_slice(
                    stream
                        .transfer(
                            &SampleBlock::new((chunk * chunk_size) as u64, points.to_vec())
                                .unwrap(),
                        )
                        .unwrap()
                        .samples(),
                );
            }
            result.extend_from_slice(stream.finish().unwrap().samples());
            assert_eq!(result.len(), expected.len());
            assert!(result
                .iter()
                .zip(&expected)
                .all(|(left, right)| (left[0] - right[0]).abs() < 1e-10
                    && (left[1] - right[1]).abs() < 1e-10));
        }
    }
}
#[test]
fn rejects_unbounded_delays_and_inaccurate_truncation_without_altering_inputs() {
    let settings = ChannelFilterSettings::Echoes {
        paths: vec![
            EchoPath {
                delay_samples: 0,
                gain: [1.0, 0.0],
            },
            EchoPath {
                delay_samples: 2049,
                gain: [0.1, 0.0],
            },
        ],
    };
    assert!(settings.validate().is_err());
    let size = 4096;
    let response: Vec<_> = (0..size)
        .map(|bin| {
            let phase =
                -std::f64::consts::TAU * (bin as f64 - size as f64 / 2.0) * 500.0 / size as f64;
            [phase.cos(), phase.sin()]
        })
        .collect();
    let original = response.clone();
    let settings = ChannelFilterSettings::CapturedFir {
        max_delay_samples: 10,
        causal_shift_samples: 0,
        maximum_rms_error: 0.01,
        maximum_bin_error: 0.1,
    };
    assert_eq!(
        settings
            .prepare(SubcarrierSpacing::Khz50, &response, &vec![true; size])
            .err()
            .unwrap()
            .code,
        "ChannelFilterFitFailed"
    );
    assert_eq!(response, original);
}

#[test]
fn captured_filter_explicitly_extends_excluded_bins_to_unity_without_mutating_input() {
    let mut response = vec![[9.0, -3.0]; 4096];
    let mut active = vec![false; 4096];
    active[2048] = true;
    response[2048] = [1.0, 0.0];
    let original = response.clone();
    let settings = ChannelFilterSettings::CapturedFir {
        max_delay_samples: 0,
        causal_shift_samples: 0,
        maximum_rms_error: 1e-9,
        maximum_bin_error: 1e-9,
    };
    let plan = settings
        .prepare(SubcarrierSpacing::Khz50, &response, &active)
        .unwrap()
        .unwrap();
    assert_eq!(plan.coefficients(), &[[1.0, 0.0]]);
    assert!(plan.response().iter().all(|gain| *gain == [1.0, 0.0]));
    assert_eq!(response, original);
}
