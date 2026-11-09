/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    signal_processing::{
        channel::impairments::NoiseModel, receiver::channel_estimation::ReceiverMode,
    },
    simulation::{
        channel::{
            protected_constellation::ProtectedCaptureSelection, run_channel, ChannelJobLimits,
            ChannelJobStatus, ChannelRunSettings,
        },
        test_points::{NoopObserver, TestPointProbe},
    },
    transmission::SubcarrierSpacing,
};
use std::sync::atomic::AtomicBool;
fn settings() -> ChannelRunSettings {
    let (tx, layout) = acquisition_support::acquisition_setup(SubcarrierSpacing::Khz50, false);
    let mut settings = ChannelRunSettings::new(tx, layout);
    settings.duration = std::time::Duration::from_micros(100);
    settings
}
fn run(settings: &ChannelRunSettings) -> docsis_ofdm_fec::simulation::channel::ChannelJobResult {
    run_channel(
        settings,
        &ChannelJobLimits::default(),
        &AtomicBool::new(false),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap()
}
#[test]
fn bounded_all_streams_have_exact_pair_identity_and_do_not_change_noisy_fec() {
    let mut settings = settings();
    settings.impairments.noise = NoiseModel::Awgn {
        sample_power: 0.0001,
    };
    settings.receiver_mode = ReceiverMode::PilotEstimated;
    let baseline = run(&settings);
    assert_eq!(baseline.status, ChannelJobStatus::Complete);
    settings.protected_constellation = Some(ProtectedCaptureSelection {
        pairs_per_stream: 17,
        first_channel_symbol: 0,
        end_channel_symbol_exclusive: 1024,
    });
    let result = run(&settings);
    assert_eq!(result.status, ChannelJobStatus::Complete);
    assert_eq!(
        serde_json::to_value(&baseline.summary).unwrap(),
        serde_json::to_value(&result.summary).unwrap()
    );
    let capture = result.protected_constellation.unwrap();
    for stream in [
        &capture.continuous_pilots,
        &capture.scattered_pilots,
        &capture.plc_data,
        &capture.plc_preamble,
    ] {
        assert_eq!(stream.pairs.len(), 17);
        assert_eq!(stream.collected_pairs, 17);
        assert!(stream.pairs.windows(2).all(|pair| (
            pair[0].channel_symbol_index,
            pair[0].physical_bin
        ) < (
            pair[1].channel_symbol_index,
            pair[1].physical_bin
        )));
    }
    assert!(capture
        .plc_data
        .pairs
        .iter()
        .all(|pair| pair.plc_phase < 120));
    assert!(capture
        .plc_preamble
        .pairs
        .iter()
        .all(|pair| pair.plc_phase >= 120));
    assert!(result.channel_estimate.unwrap().ready);
}
#[test]
fn failed_plc_and_cancelled_jobs_preserve_honest_capture_status() {
    let mut settings = settings();
    settings.impairments.noise = NoiseModel::Awgn { sample_power: 10.0 };
    settings.protected_constellation = Some(Default::default());
    let result = run(&settings);
    assert_eq!(result.status, ChannelJobStatus::Failed);
    let capture = result.protected_constellation.unwrap();
    assert!(!capture.plc_data.pairs.is_empty());
    assert!(!capture.plc_preamble.pairs.is_empty());
    let cancelled = run_channel(
        &settings,
        &ChannelJobLimits::default(),
        &AtomicBool::new(true),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap();
    assert_eq!(cancelled.status, ChannelJobStatus::Cancelled);
    assert_eq!(
        cancelled
            .protected_constellation
            .unwrap()
            .plc_data
            .collected_pairs,
        0
    );
    settings
        .protected_constellation
        .as_mut()
        .unwrap()
        .pairs_per_stream = 8193;
    assert!(settings.validate().is_err());
}
