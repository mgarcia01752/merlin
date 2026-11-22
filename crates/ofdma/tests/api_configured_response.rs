/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent transfer/impulse anchors and passive configured-response API acceptance.
use docsis_ofdma::api::*;
use docsis_ofdma::{
    channel::Channel,
    types::{Identity, SampleBlock},
};
use std::f64::consts::FRAC_PI_2;

const PREFIX_SAMPLES: usize = 192;
const ROLLOFF_SAMPLES: usize = 64;
const FRAME_SYMBOLS: usize = 6;
const BANDWIDTH_HZ: u32 = 20_000_000;
const PROFILE_ID: u16 = 224;
const DATA_IUC: u8 = 5;
const PROFILE_RANDOMIZER_SEED: u32 = 0x7f_ffff;
const SOURCE_PRBS15_SEED: u16 = 1;
const FIRST_ACTIVE_CARRIER: usize = 128;
const ACTIVE_MINISLOTS: usize = 8;
const DATA_LOADING: u8 = 4;
const FIRST_50KHZ_PILOT_PATTERN: u8 = 1;
const FIRST_25KHZ_PILOT_PATTERN: u8 = 8;
const GRANT_ID: u64 = 7;
const GRANT_EPOCH_SAMPLE: u64 = 73;
const FIRST_FRAME_ID: u64 = 3;
const GRANT_FRAMES: usize = 2;
const ESTIMATOR_VARIANCE: f64 = 0.02;
const ESTIMATOR_POWER_FLOOR: f64 = 1e-12;
const LDPC_ITERATION_BUDGET: usize = 50;
const NUMERICAL_ABSOLUTE_TOLERANCE: f64 = 1e-11;

fn configuration(mode: Numerology) -> ExperimentConfig {
    let width = mode.carriers_per_minislot();
    let pattern = match mode {
        Numerology::Khz50 => FIRST_50KHZ_PILOT_PATTERN,
        Numerology::Khz25 => FIRST_25KHZ_PILOT_PATTERN,
    };
    let profile = Profile::new(
        PROFILE_ID,
        DATA_IUC,
        PROFILE_RANDOMIZER_SEED,
        (0..ACTIVE_MINISLOTS)
            .map(|index| Minislot {
                first_carrier: FIRST_ACTIVE_CARRIER + index * width,
                loading: DATA_LOADING,
                pilot_pattern: pattern,
            })
            .collect(),
    )
    .unwrap();
    let grant = SyntheticGrant {
        id: GRANT_ID,
        profile_id: PROFILE_ID,
        epoch_sample: GRANT_EPOCH_SAMPLE,
        first_frame: FIRST_FRAME_ID,
        frames: GRANT_FRAMES,
        minislots: (0..ACTIVE_MINISLOTS).collect(),
    };
    let us = UsConfiguration::new(
        Waveform::new(
            mode,
            PREFIX_SAMPLES,
            ROLLOFF_SAMPLES,
            FRAME_SYMBOLS,
            BANDWIDTH_HZ,
        )
        .unwrap(),
        vec![profile],
        grant,
        vec![0, 1],
    )
    .unwrap();
    ExperimentConfig::new(
        us,
        SOURCE_PRBS15_SEED,
        ReceiverConfig::new(
            Estimator::FrameFlatLs,
            ESTIMATOR_VARIANCE,
            ESTIMATOR_POWER_FLOOR,
            LDPC_ITERATION_BUDGET,
        )
        .unwrap(),
    )
    .unwrap()
}
fn session(configuration: ExperimentConfig) -> Simulation {
    let mut simulation = Simulation::new(ExperimentOptions::default()).unwrap();
    simulation.configure(configuration).unwrap();
    simulation
}
fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < NUMERICAL_ABSOLUTE_TOLERANCE,
        "actual={actual}, expected={expected}"
    );
}
fn iq(actual: Iq, real: f64, imaginary: f64) {
    close(actual.re, real);
    close(actual.im, imaginary);
}

#[test]
fn literal_complex_two_tap_delay_phase_and_periodic_impulse_match_both_numerologies() {
    const PHYSICAL_DELAY_SAMPLES: usize = 2;
    const SECOND_TAP_AMPLITUDE: f64 = 0.5;
    const EXPECTED_DC_AMPLITUDE_DB: f64 = 3.521_825_181_113_624_7;
    const EXPECTED_SECOND_IMPULSE_RELATIVE_DB: f64 = -6.020_599_913_279_624;
    const EXPECTED_QUARTER_CLOCK_HZ: f64 = 25_600_000.0;
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let mut configuration = configuration(mode);
        configuration.channel.taps = vec![
            Iq { re: 1.0, im: 0.0 },
            Iq {
                re: SECOND_TAP_AMPLITUDE,
                im: 0.0,
            },
        ];
        configuration.channel.delay_samples = PHYSICAL_DELAY_SAMPLES;
        configuration.channel.phase_radians = FRAC_PI_2;
        let snapshot = session(configuration)
            .configured_response(ConfiguredResponseRequest {
                include_impulse: true,
                ..Default::default()
            })
            .unwrap();
        let length = snapshot.fft_samples;
        iq(snapshot.points[0].value, 0.0, 0.5);
        iq(snapshot.points[length / 4].value, 0.5, -1.0);
        iq(snapshot.points[length / 2].value, 0.0, 1.5);
        iq(snapshot.points[3 * length / 4].value, -0.5, -1.0);
        close(
            snapshot.points[length / 4].frequency_hz,
            -EXPECTED_QUARTER_CLOCK_HZ,
        );
        close(
            snapshot.points[3 * length / 4].frequency_hz,
            EXPECTED_QUARTER_CLOCK_HZ,
        );
        close(
            snapshot.points[length / 2].amplitude_db.unwrap(),
            EXPECTED_DC_AMPLITUDE_DB,
        );
        close(
            snapshot.points[length / 2].phase_radians.unwrap(),
            FRAC_PI_2,
        );
        assert!(snapshot.points[0].excluded);
        assert_eq!(snapshot.pre_eq_generation, None);
        assert_eq!(
            snapshot.reference_plane,
            "configured_deterministic_physical_h"
        );
        let impulse = snapshot.impulse.unwrap();
        assert_eq!(impulse.period_samples, length);
        assert_eq!(impulse.points.len(), length);
        close(impulse.peak_amplitude, 1.0);
        iq(impulse.points[PHYSICAL_DELAY_SAMPLES].value, 0.0, 1.0);
        iq(
            impulse.points[PHYSICAL_DELAY_SAMPLES + 1].value,
            0.0,
            SECOND_TAP_AMPLITUDE,
        );
        close(
            impulse.points[PHYSICAL_DELAY_SAMPLES + 1]
                .relative_power_db
                .unwrap(),
            EXPECTED_SECOND_IMPULSE_RELATIVE_DB,
        );
        for point in &impulse.points {
            if point.sample_index != PHYSICAL_DELAY_SAMPLES
                && point.sample_index != PHYSICAL_DELAY_SAMPLES + 1
            {
                iq(point.value, 0.0, 0.0);
            }
        }
    }
}

#[test]
fn corrective_c_composite_excluded_zeros_and_full_impulse_are_independent_of_zoom() {
    const CORRECTIVE_BANK_GENERATION: u64 = 11;
    const RAW_CORRECTIVE_IMAGINARY_GAIN: f64 = 2.0;
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let mut configuration = configuration(mode);
        let mut raw = vec![
            Iq {
                re: 0.0,
                im: RAW_CORRECTIVE_IMAGINARY_GAIN
            };
            mode.fft_len()
        ];
        for &carrier in configuration.us_configuration.excluded() {
            raw[carrier] = Iq::default();
        }
        configuration.pre_eq = PreEqBank::normalized(
            &configuration.us_configuration,
            raw,
            CORRECTIVE_BANK_GENERATION,
        )
        .unwrap();
        let simulation = session(configuration);
        let c = simulation
            .configured_response(ConfiguredResponseRequest {
                source: ConfiguredResponseSource::CmPreEq,
                include_impulse: true,
                ..Default::default()
            })
            .unwrap();
        iq(c.points[FIRST_ACTIVE_CARRIER].value, 0.0, 1.0);
        for &carrier in &[0, 1] {
            iq(c.points[carrier].value, 0.0, 0.0);
            assert_eq!(c.points[carrier].amplitude_db, None);
            assert_eq!(c.points[carrier].phase_radians, None);
        }
        close(
            c.impulse.as_ref().unwrap().points[0].value.im,
            (mode.fft_len() - 2) as f64 / mode.fft_len() as f64,
        );
        assert_eq!(c.pre_eq_generation, Some(CORRECTIVE_BANK_GENERATION));
        let composite = simulation
            .configured_response(ConfiguredResponseRequest {
                source: ConfiguredResponseSource::Composite,
                ..Default::default()
            })
            .unwrap();
        for (actual, expected) in composite.points.iter().zip(&c.points) {
            iq(actual.value, expected.value.re, expected.value.im);
        }
        let zoom = simulation
            .configured_response(ConfiguredResponseRequest {
                source: ConfiguredResponseSource::CmPreEq,
                first_carrier: FIRST_ACTIVE_CARRIER,
                carrier_count: 1,
                maximum_points: 1,
                include_impulse: true,
                ..Default::default()
            })
            .unwrap();
        assert_eq!(zoom.points.len(), 1);
        assert_eq!(zoom.impulse, c.impulse);
        assert_eq!(zoom.configuration_generation, c.configuration_generation);
    }
}

#[test]
fn selected_original_symbol_uses_actual_replay_bank_generation_independently_of_event_ids() {
    const INITIAL_REPLAY_BANK_ID: u64 = 5;
    const SNAPSHOT_EVENT_ID: u64 = 17;
    const SNAPSHOT_BANK_GENERATION: u64 = 11;
    const UPDATE_GRANT_SYMBOL: usize = 3;
    const SNAPSHOT_RAW_IMAGINARY_GAIN: f64 = 2.0;
    let mut configuration = configuration(Numerology::Khz50);
    let mut raw = vec![
        Iq {
            re: 0.0,
            im: SNAPSHOT_RAW_IMAGINARY_GAIN
        };
        Numerology::Khz50.fft_len()
    ];
    for &carrier in configuration.us_configuration.excluded() {
        raw[carrier] = Iq::default();
    }
    let bank = PreEqBank::normalized(
        &configuration.us_configuration,
        raw,
        SNAPSHOT_BANK_GENERATION,
    )
    .unwrap();
    configuration.pre_eq_updates = Some(
        PreEqPlayback::new(
            &configuration.us_configuration,
            INITIAL_REPLAY_BANK_ID,
            configuration.pre_eq.clone(),
            vec![PreEqEvent {
                id: SNAPSHOT_EVENT_ID,
                symbol: UPDATE_GRANT_SYMBOL,
                operation: PreEqOperation::Snapshot(bank),
            }],
            vec![],
        )
        .unwrap(),
    );
    let simulation = session(configuration);
    let request = ConfiguredResponseRequest {
        source: ConfiguredResponseSource::CmPreEq,
        first_carrier: FIRST_ACTIVE_CARRIER,
        carrier_count: 1,
        ..Default::default()
    };
    let before = simulation.configured_response(request).unwrap();
    iq(before.points[0].value, 1.0, 0.0);
    let after = simulation
        .configured_response(ConfiguredResponseRequest {
            grant_symbol: UPDATE_GRANT_SYMBOL,
            ..request
        })
        .unwrap();
    iq(after.points[0].value, 0.0, 1.0);
    assert_eq!(after.pre_eq_generation, Some(SNAPSHOT_BANK_GENERATION));
    assert_ne!(after.pre_eq_generation, Some(SNAPSHOT_EVENT_ID));
    assert_eq!(
        after.configuration_generation,
        before.configuration_generation
    );
}

#[test]
fn zero_transfer_cyclic_delay_alias_and_time_varying_h_have_explicit_states() {
    const NONZERO_FREQUENCY_OFFSET_HZ: f64 = 1000.0;
    let mode = Numerology::Khz50;
    let mut zero = configuration(mode);
    zero.channel.taps = vec![Iq::default()];
    let snapshot = session(zero)
        .configured_response(ConfiguredResponseRequest {
            include_impulse: true,
            ..Default::default()
        })
        .unwrap();
    assert!(snapshot
        .points
        .iter()
        .all(|point| point.value == Iq::default()
            && point.amplitude_db.is_none()
            && point.phase_radians.is_none()));
    let impulse = snapshot.impulse.unwrap();
    assert_eq!(impulse.peak_amplitude, 0.0);
    assert!(impulse
        .points
        .iter()
        .all(|point| point.value == Iq::default() && point.relative_power_db.is_none()));
    let mut delayed = configuration(mode);
    delayed.channel.delay_samples = mode.fft_len();
    let alias = session(delayed)
        .configured_response(ConfiguredResponseRequest {
            include_impulse: true,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(alias.physical_delay_samples, mode.fft_len());
    iq(alias.impulse.unwrap().points[0].value, 1.0, 0.0);
    let mut time_varying = configuration(mode);
    time_varying.channel.frequency_offset_hz = NONZERO_FREQUENCY_OFFSET_HZ;
    let simulation = session(time_varying);
    for source in [
        ConfiguredResponseSource::PhysicalChannel,
        ConfiguredResponseSource::Composite,
    ] {
        assert_eq!(
            simulation
                .configured_response(ConfiguredResponseRequest {
                    source,
                    ..Default::default()
                })
                .unwrap_err()
                .code(),
            "UnsupportedCapability"
        );
    }
    assert!(simulation
        .configured_response(ConfiguredResponseRequest {
            source: ConfiguredResponseSource::CmPreEq,
            ..Default::default()
        })
        .is_ok());
}

#[test]
fn missing_invalid_overflowing_queries_preserve_accepted_settings_and_owned_snapshots() {
    const OVER_FREQUENCY_POINT_BUDGET: usize = 4097;
    const FINITE_OVERFLOW_TAP_COMPONENT: f64 = 1e308;
    const OVERFLOW_TAP_COUNT: usize = 2;
    let mut simulation = Simulation::new(ExperimentOptions::default()).unwrap();
    assert_eq!(
        simulation
            .configured_response(Default::default())
            .unwrap_err()
            .code(),
        "MissingSetting"
    );
    simulation
        .configure(configuration(Numerology::Khz25))
        .unwrap();
    let original = simulation.configured_response(Default::default()).unwrap();
    let preview = simulation.preview().unwrap();
    for request in [
        ConfiguredResponseRequest {
            maximum_points: 0,
            ..Default::default()
        },
        ConfiguredResponseRequest {
            maximum_points: OVER_FREQUENCY_POINT_BUDGET,
            ..Default::default()
        },
        ConfiguredResponseRequest {
            maximum_points: 1,
            ..Default::default()
        },
        ConfiguredResponseRequest {
            first_carrier: usize::MAX,
            ..Default::default()
        },
        ConfiguredResponseRequest {
            carrier_count: usize::MAX,
            ..Default::default()
        },
        ConfiguredResponseRequest {
            grant_symbol: usize::MAX,
            ..Default::default()
        },
    ] {
        assert!(simulation.configured_response(request).is_err());
    }
    assert!(serde_json::from_str::<ConfiguredResponseRequest>(r#"{"unknown":1}"#).is_err());
    assert_eq!(
        simulation.configured_response(Default::default()).unwrap(),
        original
    );
    assert_eq!(simulation.preview().unwrap(), preview);
    simulation
        .configure(configuration(Numerology::Khz50))
        .unwrap();
    assert_ne!(
        simulation
            .configured_response(Default::default())
            .unwrap()
            .configuration_generation,
        original.configuration_generation
    );
    let retained = original.clone();
    drop(simulation);
    assert_eq!(retained, original);
    let mut huge = configuration(Numerology::Khz50);
    huge.channel.taps = vec![
        Iq {
            re: FINITE_OVERFLOW_TAP_COMPONENT,
            im: FINITE_OVERFLOW_TAP_COMPONENT
        };
        OVERFLOW_TAP_COUNT
    ];
    assert!(session(huge)
        .configured_response(Default::default())
        .is_err());
}

#[test]
fn passive_configured_queries_preserve_noisy_actual_recovery_and_frozen_report_identity() {
    const CHANNEL_NOISE_VARIANCE: f64 = 1e-6;
    const CHANNEL_NOISE_SEED: [u8; 32] = [19; 32];
    let mut config = configuration(Numerology::Khz50);
    config.channel.noise_variance = CHANNEL_NOISE_VARIANCE;
    config.channel.noise_seed = CHANNEL_NOISE_SEED;
    let mut observed = session(config.clone());
    let idle = observed
        .configured_response(ConfiguredResponseRequest {
            include_impulse: true,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(
        idle.configured_sample_noise_variance,
        CHANNEL_NOISE_VARIANCE
    );
    assert_eq!(idle.run_id, None);
    let report = observed.run().unwrap();
    let expected = session(config).run().unwrap();
    assert_eq!(report.grants[0].words, expected.grants[0].words);
    assert_eq!(report.bit_errors, expected.bit_errors);
    assert_eq!(report.reference_recovered, expected.reference_recovered);
    let frozen = report.configured_response(Default::default()).unwrap();
    assert_eq!(
        frozen.configuration_generation,
        idle.configuration_generation
    );
    assert_eq!(frozen.run_id, Some(report.grants[0].identity.run));
    observed
        .configure(configuration(Numerology::Khz50))
        .unwrap();
    assert_eq!(
        report.configured_response(Default::default()).unwrap(),
        frozen
    );
}

#[test]
fn pre_cancelled_report_does_not_invent_a_retained_execution_identity() {
    let mut simulation = session(configuration(Numerology::Khz50));
    let control = RunControl::new();
    control.cancel();
    let report = simulation.run_with_control(&control).unwrap();
    let snapshot = report.configured_response(Default::default()).unwrap();
    assert_eq!(snapshot.run_id, None);
    assert_eq!(snapshot.configuration_generation, None);
    assert_eq!(snapshot.grant_id, GRANT_ID);
}

#[test]
fn configured_constant_phase_matches_the_actual_channel_owner_for_wrapped_and_large_angles() {
    const LARGE_PHASE_RADIANS: f64 = 1e308;
    const SCIENTIFIC_TEST_RUN_ID: u64 = 19;
    const SCIENTIFIC_TEST_GENERATION: u64 = 11;
    const IMPULSE_SAMPLE_ORIGIN: u64 = 5;
    for phase_radians in [-FRAC_PI_2, LARGE_PHASE_RADIANS] {
        let mut configuration = configuration(Numerology::Khz50);
        configuration.channel.phase_radians = phase_radians;
        let identity = Identity {
            run: SCIENTIFIC_TEST_RUN_ID,
            generation: SCIENTIFIC_TEST_GENERATION,
            grant: GRANT_ID,
        };
        let mut channel = Channel::new(
            configuration.channel.clone(),
            identity,
            IMPULSE_SAMPLE_ORIGIN,
        )
        .unwrap();
        let actual = channel
            .push(&SampleBlock {
                identity,
                start: IMPULSE_SAMPLE_ORIGIN,
                samples: vec![Iq { re: 1.0, im: 0.0 }],
            })
            .unwrap();
        let response = session(configuration)
            .configured_response(ConfiguredResponseRequest {
                first_carrier: FIRST_ACTIVE_CARRIER,
                carrier_count: 1,
                ..Default::default()
            })
            .unwrap();
        iq(
            response.points[0].value,
            actual.samples[0].re,
            actual.samples[0].im,
        );
    }
}
