/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual receiver observation contracts; literal gains are independent scientific anchors.
use docsis_ofdma::api::*;

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
const NUMERICAL_TOLERANCE: f64 = 1e-11;
const RETAINED_FRAME_BUDGET: usize = 3;
const RETENTION_SCALAR_BUDGET: usize = 262_144;
const ONE_50KHZ_ESTIMATE_SCALARS: usize = 4096;
const ONE_25KHZ_ESTIMATE_SCALARS: usize = 8192;
const MULTI_GRANT_COUNT: usize = 3;
const EXPECTED_OMITTED_FRAMES: usize = 3;
const ZOOM_POINT_COUNT: usize = 3;
const OVER_QUERY_POINT_CEILING: usize = 4097;
const LEGACY_RUN_ID: u64 = 91;
const LEGACY_GENERATION: u64 = 92;
const ACTUAL_NOISE_VARIANCE: f64 = 1e-4;
const CORRECTIVE_BANK_GENERATION: u64 = 11;

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
fn session(config: ExperimentConfig, options: ExperimentOptions) -> Simulation {
    let mut session = Simulation::new(options).unwrap();
    session.configure(config).unwrap();
    session
}
fn enabled(session: &mut Simulation) {
    session
        .set_receiver_observation_policy(
            ReceiverObservationPolicy::new(RETAINED_FRAME_BUDGET, RETENTION_SCALAR_BUDGET).unwrap(),
        )
        .unwrap();
}
fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < NUMERICAL_TOLERANCE,
        "{actual} != {expected}"
    );
}

#[test]
fn both_modes_retain_actual_complex_effective_response_and_original_physical_holes() {
    // Literal H=2+j and normalized C=j give C*H=-1+2j; the receiver sees only samples.
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let mut config = configuration(mode);
        config.channel.taps = vec![Iq { re: 2.0, im: 1.0 }];
        let mut corrective = vec![Iq { re: 0.0, im: 1.0 }; mode.fft_len()];
        for &carrier in config.us_configuration.excluded() {
            corrective[carrier] = Iq::default();
        }
        config.pre_eq = PreEqBank::normalized(
            &config.us_configuration,
            corrective,
            CORRECTIVE_BANK_GENERATION,
        )
        .unwrap();
        let mut session = session(config, ExperimentOptions::default());
        let generation = session
            .configured_response(Default::default())
            .unwrap()
            .configuration_generation;
        enabled(&mut session);
        let report = session.run().unwrap();
        assert!(report.reference_recovered);
        let summary = report.receiver_observations();
        assert_eq!(summary.completed_frames, GRANT_FRAMES);
        assert_eq!(summary.retained_frames, GRANT_FRAMES);
        assert_eq!(summary.dropped_frames, 0);
        for entry in &summary.frames {
            let snapshot = report
                .receiver_estimate(ReceiverEstimateRequest {
                    retained_index: entry.retained_index,
                    ..Default::default()
                })
                .unwrap();
            assert_eq!(&snapshot.frame, entry);
            assert_eq!(Some(entry.identity.generation), generation);
            assert_eq!(entry.identity.grant, GRANT_ID);
            assert_eq!(entry.frame, FIRST_FRAME_ID + entry.retained_index as u64);
            assert_eq!(
                snapshot.reference_plane,
                "received_pilot_effective_c_times_h"
            );
            assert_eq!(snapshot.estimator, Estimator::FrameFlatLs);
            assert_eq!(snapshot.fft_samples, mode.fft_len());
            assert_eq!(snapshot.points[0].value, None);
            assert_eq!(snapshot.points[1].value, None);
            for (local_index, point) in snapshot.points[FIRST_ACTIVE_CARRIER
                ..FIRST_ACTIVE_CARRIER + ACTIVE_MINISLOTS * mode.carriers_per_minislot()]
                .iter()
                .enumerate()
            {
                let value = point.value.unwrap();
                close(value.re, -1.0);
                close(value.im, 2.0);
                assert_eq!(point.physical_carrier, FIRST_ACTIVE_CARRIER + local_index);
            }
            close(snapshot.points[0].frequency_hz, -51_200_000.0);
            close(snapshot.points[mode.fft_len() / 2].frequency_hz, 0.0);
        }
    }
}

#[test]
fn observation_enabled_exhausted_and_disabled_preserve_actual_noisy_decisions_and_default_json() {
    let mut config = configuration(Numerology::Khz50);
    config.channel.noise_variance = ACTUAL_NOISE_VARIANCE;
    let mut session = session(config.clone(), ExperimentOptions::default());
    let unobserved = session.run().unwrap();
    assert_eq!(unobserved.receiver_observations().policy.maximum_frames, 0);
    assert!(unobserved.receiver_estimate(Default::default()).is_err());
    let json: serde_json::Value = serde_json::from_str(&unobserved.to_json().unwrap()).unwrap();
    assert!(json["observations"].get("receiver_estimates").is_none());
    enabled(&mut session);
    let observed = session.run().unwrap();
    session
        .set_receiver_observation_policy(
            ReceiverObservationPolicy::new(RETAINED_FRAME_BUDGET, ONE_50KHZ_ESTIMATE_SCALARS)
                .unwrap(),
        )
        .unwrap();
    let exhausted = session.run().unwrap();
    assert_eq!(exhausted.receiver_observations().retained_frames, 1);
    for candidate in [&observed, &exhausted] {
        assert_eq!(candidate.grants[0].words, unobserved.grants[0].words);
        assert_eq!(
            candidate.grants[0].matched_data_mer,
            unobserved.grants[0].matched_data_mer
        );
        assert_eq!(
            candidate.compared_source_bits,
            unobserved.compared_source_bits
        );
        assert_eq!(candidate.bit_errors, unobserved.bit_errors);
        assert_eq!(candidate.ber, unobserved.ber);
    }
    // Legacy low-level entrypoint keeps the exact disabled contract.
    let legacy = docsis_ofdma::simulation::run(
        &config,
        &ExperimentOptions::default(),
        LEGACY_RUN_ID,
        LEGACY_GENERATION,
        &RunControl::new(),
    )
    .unwrap();
    assert_eq!(legacy.receiver_observations().policy.maximum_frames, 0);
    assert_eq!(legacy.grants[0].words, unobserved.grants[0].words);
}

#[test]
fn aggregate_frame_and_scalar_limits_span_grants_without_renumbering_history() {
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let options = ExperimentOptions {
            grants: MULTI_GRANT_COUNT,
            ..Default::default()
        };
        let mut session = session(configuration(mode), options);
        enabled(&mut session);
        let report = session.run().unwrap();
        let summary = report.receiver_observations();
        assert_eq!(summary.completed_frames, MULTI_GRANT_COUNT * GRANT_FRAMES);
        assert_eq!(summary.retained_frames, RETAINED_FRAME_BUDGET);
        assert_eq!(summary.dropped_frames, EXPECTED_OMITTED_FRAMES);
        let scalar_cost = match mode {
            Numerology::Khz50 => ONE_50KHZ_ESTIMATE_SCALARS,
            Numerology::Khz25 => ONE_25KHZ_ESTIMATE_SCALARS,
        };
        assert_eq!(
            summary.used_scalar_slots,
            scalar_cost * RETAINED_FRAME_BUDGET
        );
        assert_eq!(summary.frames[2].identity.grant, GRANT_ID + 1);
        assert_eq!(summary.frames[2].frame, FIRST_FRAME_ID + 2);
        assert!(report
            .receiver_estimate(ReceiverEstimateRequest {
                retained_index: RETAINED_FRAME_BUDGET,
                ..Default::default()
            })
            .is_err());
    }
}

#[test]
fn invalid_policy_and_numerology_replacement_are_atomic_and_do_not_mint_generation() {
    const OVER_FRAME_LIMIT: usize = 9;
    const OVER_SCALAR_LIMIT: usize = 262_145;
    for (frames, scalars) in [
        (0, 1),
        (1, 0),
        (OVER_FRAME_LIMIT, RETENTION_SCALAR_BUDGET),
        (1, OVER_SCALAR_LIMIT),
    ] {
        assert!(ReceiverObservationPolicy::new(frames, scalars).is_err());
    }
    let mut session = session(
        configuration(Numerology::Khz50),
        ExperimentOptions::default(),
    );
    let before = session.configured_response(Default::default()).unwrap();
    let policy = ReceiverObservationPolicy::new(1, ONE_50KHZ_ESTIMATE_SCALARS).unwrap();
    session.set_receiver_observation_policy(policy).unwrap();
    assert!(session
        .set_receiver_observation_policy(ReceiverObservationPolicy::new(1, 1).unwrap())
        .is_err());
    assert_eq!(session.receiver_observation_policy(), policy);
    assert!(session.configure(configuration(Numerology::Khz25)).is_err());
    assert_eq!(
        session.configured_response(Default::default()).unwrap(),
        before
    );
    assert_eq!(session.receiver_observation_policy(), policy);
    // A policy admitted before configuration is checked when its mode becomes known.
    let mut empty = Simulation::new(Default::default()).unwrap();
    empty.set_receiver_observation_policy(policy).unwrap();
    assert!(empty.configure(configuration(Numerology::Khz25)).is_err());
    assert!(empty.configuration().is_none());
}

#[test]
fn window_budgets_and_owned_results_preserve_full_original_optional_estimates() {
    let mut session = session(
        configuration(Numerology::Khz50),
        ExperimentOptions::default(),
    );
    enabled(&mut session);
    let report = session.run().unwrap();
    let full = report.receiver_estimate(Default::default()).unwrap();
    let request = ReceiverEstimateRequest {
        first_carrier: FIRST_ACTIVE_CARRIER,
        carrier_count: ZOOM_POINT_COUNT,
        maximum_points: ZOOM_POINT_COUNT,
        ..Default::default()
    };
    let zoom = report.receiver_estimate(request).unwrap();
    assert_eq!(
        zoom.points,
        full.points[FIRST_ACTIVE_CARRIER..FIRST_ACTIVE_CARRIER + ZOOM_POINT_COUNT]
    );
    for invalid in [
        ReceiverEstimateRequest {
            maximum_points: 0,
            ..request
        },
        ReceiverEstimateRequest {
            maximum_points: OVER_QUERY_POINT_CEILING,
            ..request
        },
        ReceiverEstimateRequest {
            maximum_points: 1,
            ..request
        },
        ReceiverEstimateRequest {
            first_carrier: full.fft_samples,
            ..request
        },
        ReceiverEstimateRequest {
            carrier_count: usize::MAX,
            ..request
        },
    ] {
        assert!(report.receiver_estimate(invalid).is_err());
    }
    assert_eq!(report.receiver_estimate(Default::default()).unwrap(), full);
    session.configure(configuration(Numerology::Khz25)).unwrap();
    drop(session);
    drop(report);
    close(zoom.points[0].value.unwrap().re, 1.0);
}

#[test]
fn zero_observed_channel_is_some_zero_while_unused_bins_and_never_started_frames_are_absent() {
    let mut config = configuration(Numerology::Khz50);
    config.channel.taps = vec![Iq::default()];
    let mut session = session(config, ExperimentOptions::default());
    enabled(&mut session);
    let report = session.run().unwrap();
    assert!(!report.reference_recovered);
    let snapshot = report.receiver_estimate(Default::default()).unwrap();
    assert_eq!(snapshot.points[0].value, None);
    assert_eq!(
        snapshot.points[FIRST_ACTIVE_CARRIER].value,
        Some(Iq::default())
    );
    let control = RunControl::new();
    control.cancel();
    let cancelled = session.run_with_control(&control).unwrap();
    assert_eq!(cancelled.status, RunStatus::Cancelled);
    let summary = cancelled.receiver_observations();
    assert_eq!(summary.policy.maximum_frames, RETAINED_FRAME_BUDGET);
    assert_eq!(summary.completed_frames, 0);
    assert!(summary.frames.is_empty());
    assert!(cancelled.receiver_estimate(Default::default()).is_err());
}
