/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    signal_processing::{
        channel::{
            filter::{ChannelFilterSettings, EchoPath},
            impairments::NoiseModel,
        },
        receiver::channel_estimation::{
            ChannelEstimate, MceSettings, PilotEstimate, ReceiverMode, ReliabilityEstimator,
        },
    },
    simulation::{
        channel::{ChannelPhase, ChannelRun, ChannelRunSettings},
        test_points::{NoopObserver, TestPointProbe},
    },
    transmission::SubcarrierSpacing,
};

fn estimates(symbol: u64, offset: f64) -> ChannelEstimate {
    ChannelEstimate {
        channel_symbol_index: symbol,
        ready: symbol >= 127,
        observed_bins: 2,
        maximum_age_symbols: 0,
        phase_slope_radians_per_bin: 0.0,
        response: vec![[1.0, 0.0]; 4096],
        pilots: (0..2)
            .map(|bin| PilotEstimate {
                bin,
                scattered: false,
                reference: [2.0, 0.0],
                received: [2.0 + offset, 0.0],
                response: [1.0 + offset / 2.0, 0.0],
                prior_prediction: None,
            })
            .collect(),
    }
}
#[test]
fn received_only_reliability_has_independent_difference_math_and_atomic_errors() {
    let mut estimator = ReliabilityEstimator::new(
        4096,
        MceSettings {
            minimum_variance: 1e-8,
            averaging_symbols: 1,
        },
    )
    .unwrap();
    assert_eq!(estimator.observe(&estimates(0, 0.0)).unwrap(), 1e-8);
    // Raw pilots differ by 0.2: half squared difference is 0.02.
    let mut second = estimates(1, 0.2);
    second.pilots[0].prior_prediction = Some([99.0, 99.0]);
    assert!((estimator.observe(&second).unwrap() - 0.02).abs() < 1e-12);
    assert_eq!(estimator.status().unwrap().compared_pilots, 2);
    assert!(!estimator.status().unwrap().tracking);
    assert!(estimator.observe(&estimates(3, 0.2)).is_err());
    assert_eq!(estimator.status().unwrap().channel_symbol_index, 1);
    assert_eq!(estimator.observe(&estimates(2, 0.2)).unwrap(), 1e-8);
    assert!(ReliabilityEstimator::new(
        4096,
        MceSettings {
            minimum_variance: f64::NAN,
            averaging_symbols: 0
        }
    )
    .is_err());
}
fn run(spacing: SubcarrierSpacing, variance: f64, echo: Option<usize>) -> (String, f64) {
    let (transmission, layout) = acquisition_support::acquisition_setup(spacing, false);
    let mut settings = ChannelRunSettings::new(transmission, layout);
    assert_eq!(settings.receiver_mode, ReceiverMode::SuppliedChannel);
    settings.receiver_mode = ReceiverMode::MerlinEstimated;
    settings.duration = std::time::Duration::from_micros(100);
    settings.llr_variance = variance;
    settings.impairments.noise = NoiseModel::Awgn { sample_power: 1e-5 };
    if let Some(delay) = echo {
        settings.channel_filter = ChannelFilterSettings::Echoes {
            paths: vec![
                EchoPath {
                    delay_samples: 0,
                    gain: [1.0, 0.0],
                },
                EchoPath {
                    delay_samples: delay,
                    gain: [0.05, 0.02],
                },
            ],
        };
    }
    let mut run = ChannelRun::new(&settings).unwrap();
    while run
        .advance(&mut TestPointProbe::disabled(), &mut NoopObserver)
        .unwrap()
        .is_some()
    {}
    assert_eq!(run.summary().phase, ChannelPhase::Finished);
    assert_eq!(run.summary().comparison.bit_errors, 0);
    assert!(run.summary().comparison.bits_tested > 0);
    assert_eq!(run.summary().acquisition_symbols, 128);
    assert_eq!(run.summary().channel_filter_tail_samples, echo.unwrap_or(0));
    assert!(run.receiver_reliability().unwrap().tracking);
    (
        serde_json::to_string(run.summary()).unwrap(),
        run.receiver_reliability().unwrap().raw_noise_power,
    )
}
#[test]
fn mce_recovers_noisy_echo_waveform_at_25khz() {
    run(SubcarrierSpacing::Khz25, 0.01, Some(8));
}
#[test]
fn mce_recovers_noisy_echo_waveform_at_50khz() {
    run(SubcarrierSpacing::Khz50, 0.01, Some(8));
}
#[test]
fn mce_decisions_ignore_simulator_llr_calibration() {
    let (first, power) = run(SubcarrierSpacing::Khz50, 0.01, None);
    let (second, other_power) = run(SubcarrierSpacing::Khz50, 123.0, None);
    assert_eq!(first, second);
    assert_eq!(power, other_power);
    assert!(
        (power / 1e-5 - 1.0).abs() < 0.6,
        "measured raw noise {power}"
    );
}

#[test]
fn changing_receiver_mode_keeps_the_actual_noisy_channel_samples_identical() {
    use docsis_ofdm_fec::simulation::channel::{
        constellation::ConstellationSelection, run_channel, ChannelJobLimits, ChannelJobStatus,
    };
    let (transmission, layout) =
        acquisition_support::acquisition_setup(SubcarrierSpacing::Khz50, false);
    let mut settings = ChannelRunSettings::new(transmission, layout);
    settings.duration = std::time::Duration::from_micros(100);
    settings.impairments.noise = NoiseModel::Awgn { sample_power: 1e-5 };
    settings.channel_filter = ChannelFilterSettings::Echoes {
        paths: vec![
            EchoPath {
                delay_samples: 0,
                gain: [1.0, 0.0],
            },
            EchoPath {
                delay_samples: 8,
                gain: [0.05, 0.02],
            },
        ],
    };
    settings.constellation = Some(ConstellationSelection {
        requested_pairs: 128,
        ..Default::default()
    });
    let mut raw = None;
    for mode in [ReceiverMode::SuppliedChannel, ReceiverMode::MerlinEstimated] {
        settings.receiver_mode = mode;
        let result = run_channel(
            &settings,
            &ChannelJobLimits::default(),
            &std::sync::atomic::AtomicBool::new(false),
            &mut TestPointProbe::disabled(),
            &mut NoopObserver,
            |_| {},
        )
        .unwrap();
        assert_eq!(result.status, ChannelJobStatus::Complete);
        let pairs = &result.constellation.as_ref().unwrap().pairs;
        assert!(!pairs.is_empty());
        let samples: Vec<_> = pairs
            .iter()
            .map(|pair| {
                (
                    pair.channel_symbol_index,
                    pair.physical_bin,
                    pair.before_equalization,
                )
            })
            .collect();
        if let Some(previous) = &raw {
            assert_eq!(previous, &samples);
        } else {
            raw = Some(samples);
        }
        assert_eq!(
            result.receiver_reliability.is_some(),
            mode == ReceiverMode::MerlinEstimated
        );
    }
}

#[test]
fn excessive_delay_reports_real_degradation_instead_of_perfect_correction() {
    let (transmission, layout) =
        acquisition_support::acquisition_setup(SubcarrierSpacing::Khz50, false);
    let mut settings = ChannelRunSettings::new(transmission, layout);
    settings.receiver_mode = ReceiverMode::MerlinEstimated;
    settings.duration = std::time::Duration::from_micros(100);
    settings.channel_filter = ChannelFilterSettings::Echoes {
        paths: vec![
            EchoPath {
                delay_samples: 0,
                gain: [1.0, 0.0],
            },
            EchoPath {
                delay_samples: 2048,
                gain: [0.8, 0.0],
            },
        ],
    };
    let mut run = ChannelRun::new(&settings).unwrap();
    let mut failed = false;
    for _ in 0..2048 {
        match run.advance(&mut TestPointProbe::disabled(), &mut NoopObserver) {
            Ok(Some(_)) => {}
            Ok(None) => break,
            Err(_) => {
                failed = true;
                break;
            }
        }
    }
    assert!(
        failed
            || run.summary().payload_ldpc_failures > 0
            || run.summary().comparison.bit_errors > 0
    );
}

#[test]
fn mce_recovers_echo_near_usable_guard_at_25khz() {
    run(SubcarrierSpacing::Khz25, 0.01, Some(256));
}
#[test]
fn mce_recovers_echo_near_usable_guard_at_50khz() {
    run(SubcarrierSpacing::Khz50, 0.01, Some(256));
}

#[test]
fn reference_and_legacy_pilot_rx_ignore_filter_nulls_only_on_excluded_bins() {
    for mode in [ReceiverMode::SuppliedChannel, ReceiverMode::PilotEstimated] {
        let (transmission, layout) =
            acquisition_support::acquisition_setup(SubcarrierSpacing::Khz50, false);
        let mut settings = ChannelRunSettings::new(transmission, layout);
        settings.receiver_mode = mode;
        settings.duration = std::time::Duration::from_micros(100);
        settings.channel_filter = ChannelFilterSettings::Echoes {
            paths: vec![
                EchoPath {
                    delay_samples: 0,
                    gain: [1.0, 0.0],
                },
                EchoPath {
                    delay_samples: 1,
                    gain: [1.0, 0.0],
                },
            ],
        };
        // 1 + exp(-j*w) is zero at the excluded Nyquist edge, not active data.
        let mut channel = ChannelRun::new(&settings).unwrap();
        while channel
            .advance(&mut TestPointProbe::disabled(), &mut NoopObserver)
            .unwrap()
            .is_some()
        {}
        assert_eq!(channel.summary().comparison.bit_errors, 0);
        assert_eq!(channel.summary().phase, ChannelPhase::Finished);
    }
}
