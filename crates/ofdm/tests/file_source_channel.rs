/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    simulation::{
        channel::{
            run_channel, ChannelJobLimits, ChannelJobStatus, ChannelRun, ChannelRunSettings,
        },
        source::{
            file::{FilePayload, PrbsFill},
            PayloadSourceConfig,
        },
        test_points::{
            NoopObserver, ProbeConfig, ProbeEvent, ProbeObserver, ProbeSelection, TestPointId,
            TestPointProbe,
        },
    },
    transmission::SubcarrierSpacing,
    SimulationError,
};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
fn settings(
    spacing: SubcarrierSpacing,
    bytes: &[u8],
    fill: Option<PrbsFill>,
) -> ChannelRunSettings {
    let (tx, layout) = acquisition_support::acquisition_setup(spacing, false);
    let mut settings = ChannelRunSettings::new(tx, layout);
    settings.duration = Duration::from_millis(1);
    settings.source = PayloadSourceConfig::File {
        payload: FilePayload::new(bytes, fill).unwrap(),
    };
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
fn both_spacings_recover_original_and_fill_through_real_waveform_with_eof_idle() {
    let bytes: Vec<u8> = (0..2000).map(|index| index as u8).collect();
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let target = 14232 * 3 + 5;
        let result = run(&settings(
            spacing,
            &bytes,
            Some(PrbsFill {
                target_bits: target,
                seed: 1,
            }),
        ));
        assert_eq!(result.status, ChannelJobStatus::Complete);
        assert_eq!(result.summary.generated_payloads, 4);
        assert_eq!(result.summary.recovered_payloads, 4);
        assert_eq!(result.summary.comparison.bits_tested, target);
        assert_eq!(result.summary.comparison.bit_errors, 0);
        let source = result.summary.file_source.as_ref().unwrap();
        assert!(source.fully_generated && source.fully_compared && source.file_fully_compared);
        assert_eq!(source.file.bits_tested, bytes.len() as u64 * 8);
        assert_eq!(source.fill.bits_tested, target - bytes.len() as u64 * 8);
        assert_eq!(source.coding_padding_bits, 14232 - 5);
        assert_eq!(result.recovered_file.as_ref().unwrap().bytes(), bytes);
        assert_eq!(result.summary.pending_payload_bits, 0);
        assert_eq!(result.summary.payload_codeword_errors, 0);
        assert_eq!(result.summary.plc_bit_errors, 0);
        assert!(result.summary.window.unwrap().compared_bits <= target);
    }
}
#[test]
fn empty_and_short_inputs_finish_without_comparing_padding_or_fabricating_data() {
    for bytes in [&[][..], &[0xff, 0, b'\r', b'\n'][..]] {
        let result = run(&settings(SubcarrierSpacing::Khz50, bytes, None));
        assert_eq!(result.status, ChannelJobStatus::Complete);
        assert_eq!(
            result.summary.comparison.bits_tested,
            bytes.len() as u64 * 8
        );
        assert_eq!(
            result.summary.generated_payloads,
            u64::from(!bytes.is_empty())
        );
        assert_eq!(result.recovered_file.as_ref().unwrap().bytes(), bytes);
        assert!(result.summary.file_source.unwrap().fully_compared);
        if bytes.is_empty() {
            assert_eq!(result.summary.payload_ber, None);
        }
    }
}
#[test]
fn short_duration_and_cancellation_report_partial_source_and_preserve_recovered_prefix() {
    let bytes = vec![0x81; 200_000];
    let mut config = settings(SubcarrierSpacing::Khz50, &bytes, None);
    config.duration = Duration::from_micros(25);
    let result = run(&config);
    assert_eq!(result.status, ChannelJobStatus::Complete); // Requested duration completed.
    let source = result.summary.file_source.as_ref().unwrap();
    assert!(!source.fully_generated && !source.fully_compared);
    let prefix = result.recovered_file.as_ref().unwrap();
    assert!(!prefix.complete);
    assert!(!prefix.bytes().is_empty());
    assert_eq!(prefix.bytes(), &bytes[..prefix.bytes().len()]);
    assert_eq!(source.generated_bits, result.summary.comparison.bits_tested);
    let cancelled = run_channel(
        &config,
        &ChannelJobLimits::default(),
        &AtomicBool::new(true),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap();
    assert_eq!(cancelled.status, ChannelJobStatus::Cancelled);
    assert_eq!(cancelled.summary.comparison.bits_tested, 0);
    assert!(cancelled.recovered_file.unwrap().bytes().is_empty());
    let token = AtomicBool::new(false);
    config.duration = Duration::from_millis(5);
    let cancelled = run_channel(
        &config,
        &ChannelJobLimits::default(),
        &token,
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |progress| {
            if progress.recovered_payloads > 0 {
                token.store(true, Ordering::Relaxed);
            }
        },
    )
    .unwrap();
    assert_eq!(cancelled.status, ChannelJobStatus::Cancelled);
    let prefix = cancelled.recovered_file.unwrap();
    assert!(!prefix.bytes().is_empty());
    assert_eq!(prefix.bytes(), &bytes[..prefix.bytes().len()]);
    assert!(!cancelled.summary.file_source.unwrap().fully_compared);
}
struct Sink;
impl ProbeObserver for Sink {
    fn observe(&mut self, _event: &ProbeEvent<'_>) -> Result<(), SimulationError> {
        Ok(())
    }
}
#[test]
fn file_observation_on_off_and_exhaustion_preserve_future_recovery_and_noise() {
    let bytes = vec![0x5a; 5000];
    let mut config = settings(
        SubcarrierSpacing::Khz50,
        &bytes,
        Some(PrbsFill {
            target_bits: 100_000,
            seed: 7,
        }),
    );
    config.impairments.noise =
        docsis_ofdm_fec::signal_processing::channel::impairments::NoiseModel::Awgn {
            sample_power: 1e-6,
        };
    let baseline = run(&config);
    let observed = run_channel(
        &config,
        &ChannelJobLimits::default(),
        &AtomicBool::new(false),
        &mut TestPointProbe::new(ProbeConfig {
            selections: vec![ProbeSelection {
                stage: TestPointId::SourcePayload,
                source_symbols: None,
                channel_symbols: None,
                elements: 0..14232,
            }],
            max_events: 1,
            max_elements_per_event: 14232,
            max_total_elements: 14232,
        })
        .unwrap(),
        &mut Sink,
        |_| {},
    )
    .unwrap();
    assert_eq!(observed.status, ChannelJobStatus::Complete);
    assert_eq!(observed.summary.comparison, baseline.summary.comparison);
    assert_eq!(
        observed.summary.payload_ldpc_iterations,
        baseline.summary.payload_ldpc_iterations
    );
    assert_eq!(
        observed.recovered_file.unwrap().bytes(),
        baseline.recovered_file.unwrap().bytes()
    );
    let mut stepper = ChannelRun::new(&config).unwrap();
    config.source = PayloadSourceConfig::Prbs15; // Frozen session still owns file input.
    while stepper
        .advance(&mut TestPointProbe::disabled(), &mut NoopObserver)
        .unwrap()
        .is_some()
    {}
    assert_eq!(stepper.recovered_file().unwrap().bytes(), bytes);
}
