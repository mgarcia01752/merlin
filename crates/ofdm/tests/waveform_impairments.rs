/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    scenario::{ChannelFrame, FrequencyGrid, RxMerFrame, ScenarioKey, SourceProvenance},
    signal_processing::{
        channel::{
            awgn::SampleAwgn,
            impairments::{ChannelImpairments, NoiseModel},
        },
        waveform::{fft::ForwardTransform, SampleBlock},
    },
    simulation::{
        channel::{ChannelPhase, ChannelRun, ChannelRunSettings},
        test_points::{NoopObserver, TestPointProbe},
    },
    transmission::SubcarrierSpacing,
};
use std::time::Duration;
fn frame(spacing: SubcarrierSpacing, mer: f64) -> RxMerFrame {
    let count = spacing.fft_samples() as usize;
    RxMerFrame {
        key: ScenarioKey {
            mac_address: [0, 202, 0, 0, 0, 1],
            channel_id: 7,
            grid: FrequencyGrid {
                zero_frequency_hz: 100_000_000,
                first_subcarrier_index: 0,
                spacing_hz: spacing.frequency_hz(),
                sample_count: count,
            },
        },
        source: SourceProvenance {
            name: "synthetic".into(),
            sha256: "0".repeat(64),
            byte_length: 0,
            capture_time: Some(1),
            file_kind: "synthetic".into(),
            format_major: 1,
            format_minor: 0,
        },
        capture_time: 1,
        mer_db: vec![Some(mer); count],
    }
}
fn settings() -> ChannelRunSettings {
    let (transmission, layout) =
        acquisition_support::acquisition_setup(SubcarrierSpacing::Khz50, false);
    let mut settings = ChannelRunSettings::new(transmission, layout);
    settings.duration = Duration::from_micros(100);
    settings
}
fn run(settings: &ChannelRunSettings) -> docsis_ofdm_fec::simulation::channel::ChannelSummary {
    let mut run = ChannelRun::new(settings).unwrap();
    while run
        .advance(&mut TestPointProbe::disabled(), &mut NoopObserver)
        .unwrap()
        .is_some()
    {}
    run.summary().clone()
}
#[test]
fn sample_awgn_is_chunk_invariant_and_zero_is_exact() {
    let input = vec![[1.0, -0.5]; 4096];
    let original = SampleBlock::new(0, input.clone()).unwrap();
    assert_eq!(
        SampleAwgn::new(0.0, 91, 0)
            .unwrap()
            .transfer(&original)
            .unwrap(),
        original
    );
    let all = SampleAwgn::new(0.001, 91, 0)
        .unwrap()
        .transfer(&original)
        .unwrap();
    let mut stream = SampleAwgn::new(0.001, 91, 0).unwrap();
    let mut split = Vec::new();
    for start in (0..4096).step_by(137) {
        let end = (start + 137).min(4096);
        split.extend_from_slice(
            stream
                .transfer(&SampleBlock::new(start as u64, input[start..end].to_vec()).unwrap())
                .unwrap()
                .samples(),
        );
    }
    assert_eq!(split, all.samples());
    assert!(stream.transfer(&original).is_err());
    assert!(SampleAwgn::new(f64::NAN, 0, 0).is_err());
}
#[test]
fn unitary_fft_preserves_normalized_gaussian_noise_power() {
    let mut channel = SampleAwgn::new(0.001, 41, 0).unwrap();
    let mut energy = 0.0;
    let mut mean = [0.0, 0.0];
    for block in 0..16 {
        let noise = channel
            .transfer(&SampleBlock::new(block * 4096, vec![[0.0, 0.0]; 4096]).unwrap())
            .unwrap();
        let bins = ForwardTransform::new(SubcarrierSpacing::Khz50)
            .apply(noise.samples())
            .unwrap();
        for point in bins {
            energy += point[0] * point[0] + point[1] * point[1];
            mean[0] += point[0];
            mean[1] += point[1];
        }
    }
    assert!((energy / 65536.0 - 0.001).abs() < 0.00002);
    assert!(mean
        .iter()
        .all(|component| (component / 65536.0).abs() < 0.0004));
}
#[test]
fn real_waveform_awgn_recovers_payload_and_reports_mer_iterations() {
    let mut settings = settings();
    let baseline = run(&settings);
    settings.impairments = ChannelImpairments {
        seed: 41,
        noise: NoiseModel::Awgn {
            sample_power: 0.001,
        },
        response: None,
    };
    let noisy = run(&settings);
    assert_eq!(noisy.phase, ChannelPhase::Finished);
    assert_eq!(noisy.comparison.bit_errors, 0);
    assert!(noisy.measured_mer_db.unwrap() > 29.0 && noisy.measured_mer_db.unwrap() < 31.0);
    assert!(noisy.maximum_bin_error > baseline.maximum_bin_error);
    assert!(noisy.payload_ldpc_iterations.iter().sum::<u64>() > 0);
    settings.receive_block_samples = 137;
    assert_eq!(
        serde_json::to_value(run(&settings)).unwrap(),
        serde_json::to_value(noisy).unwrap()
    );
}
#[test]
fn per_carrier_rxmer_and_complex_response_pass_through_samples_and_equalization() {
    let mut settings = settings();
    let frame = frame(SubcarrierSpacing::Khz50, 35.0);
    let response = ChannelFrame {
        key: frame.key.clone(),
        source: frame.source.clone(),
        capture_time: 1,
        iq: vec![Some([0.5, 0.5]); frame.mer_db.len()],
    };
    settings.impairments = ChannelImpairments {
        seed: 31,
        noise: NoiseModel::Rxmer {
            frame: frame.clone(),
        },
        response: None,
    };
    let unity = run(&settings);
    settings.impairments.response = Some(response);
    let equalized = run(&settings);
    assert_eq!(equalized.comparison.bit_errors, 0);
    assert_eq!(equalized.recovered_payloads, unity.recovered_payloads);
    assert!((equalized.measured_mer_db.unwrap() - unity.measured_mer_db.unwrap()).abs() < 1e-8);
    assert!(equalized.measured_mer_db.unwrap() > 34.0 && equalized.measured_mer_db.unwrap() < 36.0);
    let mut invalid = frame;
    invalid.mer_db[200] = None;
    settings.impairments.noise = NoiseModel::Rxmer { frame: invalid };
    assert!(ChannelRun::new(&settings).is_err());
}

#[test]
fn noisy_failed_control_preserves_partial_science_without_fake_zero_ber() {
    use docsis_ofdm_fec::simulation::channel::{run_channel, ChannelJobLimits, ChannelJobStatus};
    let mut config = settings();
    config.impairments = ChannelImpairments {
        seed: 41,
        noise: NoiseModel::Awgn { sample_power: 10.0 },
        response: None,
    };
    let result = run_channel(
        &config,
        &ChannelJobLimits::default(),
        &std::sync::atomic::AtomicBool::new(false),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap();
    assert_eq!(result.status, ChannelJobStatus::Failed);
    assert!(result.error.is_some());
    assert_eq!(result.summary.phase, ChannelPhase::Failed);
    assert_eq!(result.summary.payload_ber, None);
    assert_eq!(result.summary.payload_mean_ldpc_iterations, None);
}

#[test]
fn exhausted_channel_probes_do_not_change_noisy_scientific_outcomes() {
    use docsis_ofdm_fec::simulation::test_points::{ProbeConfig, ProbeSelection, TestPointId};
    let mut config = settings();
    config.impairments = ChannelImpairments {
        seed: 41,
        noise: NoiseModel::Awgn {
            sample_power: 0.001,
        },
        response: None,
    };
    let baseline = run(&config);
    let mut session = ChannelRun::new(&config).unwrap();
    let mut probes = TestPointProbe::new(ProbeConfig {
        selections: vec![ProbeSelection {
            stage: TestPointId::ChannelOutput,
            elements: 0..4,
            source_symbols: None,
            channel_symbols: None,
        }],
        max_events: 1,
        max_elements_per_event: 4,
        max_total_elements: 4,
    })
    .unwrap();
    while session
        .advance(&mut probes, &mut NoopObserver)
        .unwrap()
        .is_some()
    {}
    assert_eq!(
        serde_json::to_value(session.summary()).unwrap(),
        serde_json::to_value(baseline).unwrap()
    );
}
