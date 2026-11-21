/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! High-level Rust acceptance; independent source observer, state and partials.
use docsis_ofdma::api::*;
use std::{
    thread,
    time::{Duration, Instant},
};

fn configuration(mode: Numerology, loading: u8) -> ExperimentConfig {
    let width = mode.carriers_per_minislot();
    let waveform = Waveform::new(mode, 192, 64, 6, 20_000_000).unwrap();
    let profile = Profile::new(
        224,
        5,
        0x7f_ffff,
        (0..8)
            .map(|i| Minislot {
                first_carrier: 128 + i * width,
                loading,
                pilot_pattern: if width == 8 { 1 } else { 8 },
            })
            .collect(),
    )
    .unwrap();
    let grant = SyntheticGrant {
        id: 7,
        profile_id: 224,
        epoch_sample: 73,
        first_frame: 3,
        frames: 2,
        minislots: (0..8).collect(),
    };
    let us = UsConfiguration::new(waveform, vec![profile], grant, vec![0, 1]).unwrap();
    ExperimentConfig::new(
        us,
        1,
        ReceiverConfig::new(Estimator::FrameFlatLs, 0.02, 1e-12, 50).unwrap(),
    )
    .unwrap()
}

#[test]
fn both_numerologies_high_level_exact_source_coverage_and_owned_results() {
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let options = ExperimentOptions {
            grants: 3,
            chunk_samples: 997,
            ..ExperimentOptions::default()
        };
        let mut session = Simulation::new(options).unwrap();
        assert_eq!(session.preview().unwrap_err().code(), "MissingSetting");
        session.configure(configuration(mode, 4)).unwrap();
        let preview = session.preview().unwrap();
        assert_eq!(preview.profile_ids, vec![224]);
        let first = session.run().unwrap();
        assert_eq!(first.status, RunStatus::Completed);
        assert!(first.reference_recovered);
        assert_eq!(first.bit_errors, 0);
        assert_eq!(first.ber, Some(0.0));
        assert_eq!(first.generated_source_bits, preview.source_bits);
        assert_eq!(first.compared_source_bits, preview.source_bits);
        assert!(first.not_started_grants.is_empty());
        let retained = first.clone();
        let second = session.run().unwrap();
        assert!(second.reference_recovered);
        assert_eq!(first, retained);
        assert_eq!(first.grants[0].words, second.grants[0].words);
        assert_ne!(first.grants[0].identity.run, second.grants[0].identity.run);
        let json: serde_json::Value = serde_json::from_str(&first.to_json().unwrap()).unwrap();
        assert_eq!(json["format"], "merlin.upstream.experiment");
        assert_eq!(json["schema_version"], 1);
        assert_eq!(json["reference_recovered"], true);
    }
}

#[test]
fn failed_configuration_and_budget_changes_preserve_accepted_settings() {
    let mut session = Simulation::new(ExperimentOptions::default()).unwrap();
    session
        .configure(configuration(Numerology::Khz50, 4))
        .unwrap();
    let accepted = session.preview().unwrap();
    let mut bad = configuration(Numerology::Khz50, 4);
    bad.source_seed = 0;
    assert_eq!(session.configure(bad).unwrap_err().code(), "InvalidInput");
    assert!(session
        .set_options(ExperimentOptions {
            maximum_samples: 1,
            ..ExperimentOptions::default()
        })
        .is_err());
    assert_eq!(session.preview().unwrap(), accepted);
    assert!(session.run().unwrap().reference_recovered);
}

#[test]
fn pre_cancel_does_not_generate_or_claim_recovery() {
    let mut session = Simulation::new(ExperimentOptions {
        grants: 3,
        ..ExperimentOptions::default()
    })
    .unwrap();
    session
        .configure(configuration(Numerology::Khz50, 4))
        .unwrap();
    let control = RunControl::new();
    control.cancel();
    let report = session.run_with_control(&control).unwrap();
    assert_eq!(report.status, RunStatus::Cancelled);
    assert_eq!(report.generated_source_bits, 0);
    assert_eq!(report.compared_source_bits, 0);
    assert_eq!(report.ber, None);
    assert!(!report.reference_recovered);
    assert_eq!(report.not_started_grants, vec![7, 8, 9]);
    assert_eq!(control.progress().status, RunStatus::Cancelled);
}

#[test]
fn asynchronous_cancel_retains_actual_completed_prefix() {
    let options = ExperimentOptions {
        grants: 128,
        chunk_samples: 313,
        ..ExperimentOptions::default()
    };
    let mut session = Simulation::new(options).unwrap();
    session
        .configure(configuration(Numerology::Khz50, 4))
        .unwrap();
    let control = RunControl::new();
    let worker_control = control.clone();
    let worker = thread::spawn(move || session.run_with_control(&worker_control).unwrap());
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let progress = control.progress();
        if progress.compared_bits > 0 {
            control.cancel();
            break;
        }
        assert!(Instant::now() < deadline, "no bounded progress");
        thread::yield_now();
    }
    let report = worker.join().unwrap();
    assert_eq!(report.status, RunStatus::Cancelled);
    assert!(report.compared_source_bits > 0);
    assert!(report.compared_source_bits < report.planned.source_bits);
    assert_eq!(report.bit_errors, 0);
    assert!(!report.reference_recovered);
    assert_eq!(
        report.compared_source_bits,
        report
            .grants
            .iter()
            .map(|grant| grant.compared_bits)
            .sum::<usize>()
    );
    assert!(report
        .grants
        .iter()
        .flat_map(|grant| &grant.words)
        .all(|word| word.decoded.converged));
}

#[test]
fn passive_qam_sa_queries_and_retention_leave_all_science_unchanged() {
    let taps = vec![
        Tap::TxGrid,
        Tap::TxOutput,
        Tap::ChannelInput,
        Tap::ChannelOutput,
        Tap::RxInput,
        Tap::RxFft,
        Tap::RxEqualized,
    ];
    let mut enabled = Simulation::new(ExperimentOptions {
        retained_points: 65536,
        retained_taps: taps,
        ..ExperimentOptions::default()
    })
    .unwrap();
    enabled
        .configure(configuration(Numerology::Khz50, 4))
        .unwrap();
    let first = enabled.run().unwrap();
    let tx = first.spectrum(Tap::TxOutput, Numerology::Khz50).unwrap();
    for tap in [Tap::ChannelInput, Tap::ChannelOutput, Tap::RxInput] {
        let spectrum = first.spectrum(tap, Numerology::Khz50).unwrap();
        assert_eq!(spectrum.first_sample, tx.first_sample);
        assert_eq!(spectrum.power_per_hz, tx.power_per_hz);
    }
    let qam = first.qam(Tap::RxEqualized, Some(4), 100).unwrap();
    assert!(!qam.points.is_empty());
    assert!(qam
        .points
        .iter()
        .all(|point| point.loading == 4 && point.carrier >= 128));
    assert!(first.spectrum(Tap::RxFft, Numerology::Khz50).is_err());
    let mut disabled = Simulation::new(ExperimentOptions {
        retained_points: 0,
        ..ExperimentOptions::default()
    })
    .unwrap();
    disabled
        .configure(configuration(Numerology::Khz50, 4))
        .unwrap();
    let second = disabled.run().unwrap();
    assert_eq!(first.grants[0].words, second.grants[0].words);
    assert_eq!(first.compared_source_bits, second.compared_source_bits);
    assert_eq!(first.bit_errors, second.bit_errors);
    assert!(second.qam(Tap::RxEqualized, None, 10).is_err());
    assert!(second.spectrum(Tap::TxOutput, Numerology::Khz50).is_err());
}

#[test]
fn synthetic_gain_is_recovered_from_pilots_but_destructive_channel_is_not_success() {
    let mut session = Simulation::new(ExperimentOptions::default()).unwrap();
    let mut config = configuration(Numerology::Khz50, 4);
    config.channel.taps = vec![Iq { re: 2.0, im: 3.0 }];
    session.configure(config.clone()).unwrap();
    assert!(session.run().unwrap().reference_recovered);
    config.channel.taps = vec![Iq::default()];
    session.configure(config).unwrap();
    let destroyed = session.run().unwrap();
    assert_eq!(destroyed.status, RunStatus::Completed);
    assert!(!destroyed.reference_recovered);
    assert!(destroyed.bit_errors > 0);
    assert_eq!(
        destroyed.generated_source_bits,
        destroyed.compared_source_bits
    );
    assert!(destroyed
        .grants
        .iter()
        .all(|grant| grant.missing_symbols == 0));
}

#[test]
fn pause_resume_preserves_noisy_pipeline_and_cancel_wakes_paused_worker() {
    let options = ExperimentOptions {
        grants: 12,
        chunk_samples: 127,
        ..ExperimentOptions::default()
    };
    let mut config = configuration(Numerology::Khz50, 4);
    config.channel.taps = vec![
        Iq { re: 0.9, im: 0.2 },
        Iq {
            re: 0.01,
            im: -0.02,
        },
    ];
    config.channel.noise_variance = 1e-6;
    config.channel.delay_samples = 1;
    let mut baseline = Simulation::new(options.clone()).unwrap();
    baseline.configure(config.clone()).unwrap();
    let expected = baseline.run().unwrap();
    let mut session = Simulation::new(options).unwrap();
    session.configure(config).unwrap();
    let control = RunControl::new();
    control.pause();
    let worker_control = control.clone();
    let worker = thread::spawn(move || session.run_with_control(&worker_control).unwrap());
    wait_status(&control, RunStatus::Paused);
    assert_eq!(control.progress().compared_bits, 0);
    control.resume();
    let deadline = Instant::now() + Duration::from_secs(10);
    while control.progress().compared_bits == 0 {
        assert!(Instant::now() < deadline);
        thread::yield_now();
    }
    control.pause();
    wait_status(&control, RunStatus::Paused);
    let frozen = control.progress();
    thread::sleep(Duration::from_millis(150));
    assert_eq!(control.progress(), frozen);
    control.resume();
    let actual = worker.join().unwrap();
    assert_eq!(actual.status, RunStatus::Completed);
    assert_eq!(actual.bit_errors, expected.bit_errors);
    assert_eq!(actual.compared_source_bits, expected.compared_source_bits);
    for (actual, expected) in actual.grants.iter().zip(&expected.grants) {
        assert_eq!(actual.words, expected.words);
        assert_eq!(actual.emitted_samples, expected.emitted_samples);
        assert_eq!(actual.received_samples, expected.received_samples);
    }
    let mut session = Simulation::new(ExperimentOptions::default()).unwrap();
    session
        .configure(configuration(Numerology::Khz50, 4))
        .unwrap();
    let control = RunControl::new();
    control.pause();
    let worker_control = control.clone();
    let worker = thread::spawn(move || session.run_with_control(&worker_control).unwrap());
    wait_status(&control, RunStatus::Paused);
    control.cancel();
    let report = worker.join().unwrap();
    assert_eq!(report.status, RunStatus::Cancelled);
    assert_eq!(report.generated_source_bits, 0);
    assert_eq!(report.elapsed_ms, control.progress().elapsed_ms);
}

fn wait_status(control: &RunControl, status: RunStatus) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while control.progress().status != status {
        assert!(
            Instant::now() < deadline,
            "missing expected {status:?} acknowledgement"
        );
        thread::yield_now();
    }
}

#[test]
fn synthetic_measurement_modes_preflight_all_banks_and_preserve_chunk_invariance() {
    let mut config = configuration(Numerology::Khz50, 4);
    let us = &config.us_configuration;
    let first = SyntheticRxmer::new(us, 1, vec![Some(35.0); 2048]).unwrap();
    let second = SyntheticRxmer::new(us, 2, vec![Some(45.0); 2048]).unwrap();
    config.rxmer = Some(
        RxmerPlayback::new(
            us,
            vec![first, second],
            RxmerPolicy::Interpolate {
                offsets: vec![0, 6],
            },
        )
        .unwrap(),
    );
    config.pre_eq_updates = Some(
        PreEqPlayback::new(
            us,
            1,
            config.pre_eq.clone(),
            vec![PreEqEvent {
                id: 2,
                symbol: 6,
                operation: PreEqOperation::Snapshot(config.pre_eq.clone()),
            }],
            vec![],
        )
        .unwrap(),
    );
    let mut one = Simulation::new(ExperimentOptions {
        chunk_samples: 127,
        ..ExperimentOptions::default()
    })
    .unwrap();
    one.configure(config.clone()).unwrap();
    let first = one.run().unwrap();
    let mut second = Simulation::new(ExperimentOptions {
        chunk_samples: 4096,
        ..ExperimentOptions::default()
    })
    .unwrap();
    second.configure(config.clone()).unwrap();
    let other = second.run().unwrap();
    assert!(first.reference_recovered);
    assert_eq!(first.grants[0].words, other.grants[0].words);
    assert_eq!(
        first.grants[0].matched_data_mer,
        other.grants[0].matched_data_mer
    );
    assert!(first.grants[0].matched_data_mer.pairs > 0);
    let n = 2048;
    let mut raw = vec![Iq { re: 0.0, im: 1.0 }; n];
    raw[0] = Iq::default();
    raw[1] = Iq::default();
    let imaginary = PreEqBank::normalized(&config.us_configuration, raw, 2).unwrap();
    config.pre_eq_updates = Some(
        PreEqPlayback::new(
            &config.us_configuration,
            1,
            config.pre_eq.clone(),
            vec![PreEqEvent {
                id: 2,
                symbol: 6,
                operation: PreEqOperation::Snapshot(imaginary),
            }],
            vec![],
        )
        .unwrap(),
    );
    assert_eq!(
        one.configure(config.clone()).unwrap_err().code(),
        "UnsupportedCapability"
    );
    assert!(one.run().unwrap().reference_recovered);
    // Ordinary synthetic response can replay nonunity complete banks; received
    // pilots track the new frame bank with no supplied C at the RX boundary.
    config.rxmer = None;
    one.configure(config).unwrap();
    assert!(one.run().unwrap().reference_recovered);
}

#[test]
fn zero_loading_requires_zero_pilot_pattern_and_has_no_payload_recovery() {
    let mut profile = configuration(Numerology::Khz50, 4)
        .us_configuration
        .selected_profile()
        .minislots()
        .to_vec();
    for slot in &mut profile {
        slot.loading = 0;
        slot.pilot_pattern = 0;
    }
    let original = configuration(Numerology::Khz50, 4);
    let profile = Profile::new(224, 5, 0x7fffff, profile).unwrap();
    let us = UsConfiguration::new(
        original.us_configuration.waveform().clone(),
        vec![profile],
        original.us_configuration.grant().clone(),
        vec![0, 1],
    )
    .unwrap();
    let config = ExperimentConfig::new(us, 1, original.receiver).unwrap();
    let mut session = Simulation::new(ExperimentOptions::default()).unwrap();
    session.configure(config).unwrap();
    let report = session.run().unwrap();
    assert_eq!(report.status, RunStatus::Completed);
    assert_eq!(report.generated_source_bits, 0);
    assert_eq!(report.compared_source_bits, 0);
    assert_eq!(report.ber, None);
    assert!(!report.reference_recovered);
    assert_eq!(report.grants[0].emitted_samples, 0);
    assert_eq!(
        report.grants[0].matched_data_mer_value,
        docsis_ofdma::instruments::MerValue::Unavailable
    );
}

#[test]
fn explicit_guards_preserve_useful_timing_and_residual_guard_rng_policy() {
    for residual in [false, true] {
        let mut config = configuration(Numerology::Khz50, 4);
        if residual {
            config.rxmer = Some(
                RxmerPlayback::new(
                    &config.us_configuration,
                    vec![
                        SyntheticRxmer::new(&config.us_configuration, 1, vec![Some(40.0); 2048])
                            .unwrap(),
                    ],
                    RxmerPolicy::Single,
                )
                .unwrap(),
            );
        }
        let mut baseline = Simulation::new(ExperimentOptions::default()).unwrap();
        baseline.configure(config.clone()).unwrap();
        let baseline = baseline.run().unwrap();
        config.guards = GuardIntervals {
            leading: 17,
            trailing: 23,
        };
        let mut session = Simulation::new(ExperimentOptions {
            grants: 3,
            chunk_samples: 127,
            ..ExperimentOptions::default()
        })
        .unwrap();
        session.configure(config).unwrap();
        let report = session.run().unwrap();
        assert!(report.reference_recovered);
        assert_eq!(report.grants[0].words, baseline.grants[0].words);
        assert_eq!(
            report.grants[0].emitted_samples,
            baseline.grants[0].emitted_samples + 40
        );
        assert_eq!(report.grants[0].pending_transmit_samples, 0);
        assert_eq!(report.unverified_source_bits, 0);
        assert_eq!(
            report.grants[0].matched_data_mer,
            baseline.grants[0].matched_data_mer
        );
        for grant in &report.grants {
            assert_eq!(grant.received_samples, grant.planned_transmit_samples);
        }
    }
}

#[test]
fn acknowledged_paused_time_does_not_consume_active_runtime_budget() {
    let mut session = Simulation::new(ExperimentOptions {
        timeout_ms: 1000,
        ..ExperimentOptions::default()
    })
    .unwrap();
    session
        .configure(configuration(Numerology::Khz50, 4))
        .unwrap();
    let control = RunControl::new();
    control.pause();
    let worker_control = control.clone();
    let worker = thread::spawn(move || session.run_with_control(&worker_control).unwrap());
    wait_status(&control, RunStatus::Paused);
    thread::sleep(Duration::from_millis(1100));
    assert_eq!(control.progress().status, RunStatus::Paused);
    control.resume();
    let report = worker.join().unwrap();
    assert_eq!(report.status, RunStatus::Completed);
    assert!(report.reference_recovered);
    assert!(report.elapsed_ms < 1000);
}
