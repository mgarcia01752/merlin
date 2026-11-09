/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Continuous duration stepping through actual waveform/control/payload stages.
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    simulation::{
        channel::{ChannelPhase, ChannelRun, ChannelRunSettings},
        test_points::{
            ProbeConfig, ProbeEvent, ProbeObserver, ProbeSelection, TestPointId, TestPointProbe,
        },
    },
    transmission::SubcarrierSpacing,
    SimulationError,
};
use std::time::Duration;
struct Unused;
impl ProbeObserver for Unused {
    fn observe(&mut self, _event: &ProbeEvent<'_>) -> Result<(), SimulationError> {
        panic!("disabled observer called");
    }
}

#[test]
fn both_spacings_stream_exact_source_interval_then_complete_and_drain() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let (mut settings, layout) = acquisition_support::acquisition_setup(spacing, false);
        settings.time_interleaver_depth = spacing.max_time_interleaver_depth();
        let mut config = ChannelRunSettings::new(settings, layout);
        assert_eq!(config.duration, Duration::from_secs(1));
        config.duration = Duration::from_millis(3);
        config.receive_block_samples = 137;
        let expected = config.validate().unwrap();
        let mut run = ChannelRun::new(&config).unwrap();
        config.duration = Duration::from_secs(9);
        config.prbs_seed = 1;
        assert!(run.summary().payload_ber.is_none());
        let mut probe = TestPointProbe::disabled();
        let mut next_channel = 0;
        let mut bit_position = 0;
        let mut observation = 0;
        let mut completions = 0;
        while let Some(step) = run.advance(&mut probe, &mut Unused).unwrap() {
            if let Some(index) = step.channel_symbol_index {
                assert_eq!(index, next_channel);
                next_channel += 1;
            }
            if step.phase == ChannelPhase::Observation {
                observation += 1;
            }
            if step.phase == ChannelPhase::Completion {
                completions += 1;
            }
            for word in step.recovered {
                assert_eq!(word.comparison.first_bit, bit_position);
                assert_eq!(word.comparison.bit_errors, 0);
                assert_eq!(word.receive_sequence, bit_position / 14232);
                bit_position += word.comparison.bits_tested;
                assert!(word.ldpc_success && word.bch_success);
            }
            assert_eq!(run.summary().payload_ldpc_iterations.len(), 101);
        }
        let result = run.summary();
        assert_eq!(result.phase, ChannelPhase::Finished);
        assert_eq!(result.timing, expected);
        assert_eq!(result.observation_symbols, expected.complete_symbols);
        assert_eq!(result.observation_symbols, observation);
        assert_eq!(result.completion_symbols, completions);
        assert!(completions > 0);
        assert_eq!(result.generated_payloads, result.recovered_payloads);
        assert_eq!(result.comparison.bits_tested, bit_position);
        assert_eq!(bit_position, result.generated_payloads * 14232);
        assert_eq!(result.payload_codeword_errors, 0);
        assert_eq!(result.payload_ber, Some(0.0));
        assert_eq!(result.pending_payload_bits, 0);
        assert_eq!(result.plc_bit_errors, 0);
        assert!(result.plc_frames > 0);
        assert_eq!(
            result.rx_warmup_symbols,
            u64::from(spacing.max_time_interleaver_depth()) - 1
        );
        assert_eq!(result.interleaver_drain_symbols, result.rx_warmup_symbols);
        assert_eq!(result.accepted_ncp_fields, observation + completions);
        assert_eq!(
            result.total_transmitted_samples,
            next_channel * u64::from(expected.symbol_period_samples)
                + result.waveform_tail_samples as u64
        );
        assert_eq!(
            result.payload_ldpc_iterations.iter().sum::<u64>(),
            result.recovered_payloads
        );
        assert!(run.advance(&mut probe, &mut Unused).unwrap().is_none());
    }
}

#[test]
fn sample_chunk_size_does_not_change_source_or_decoding_outcomes() {
    let (settings, layout) =
        acquisition_support::acquisition_setup(SubcarrierSpacing::Khz50, false);
    let mut config = ChannelRunSettings::new(settings, layout);
    config.duration = Duration::from_micros(200);
    let mut results = Vec::new();
    for chunks in [137, 4096] {
        config.receive_block_samples = chunks;
        let mut run = ChannelRun::new(&config).unwrap();
        while run
            .advance(&mut TestPointProbe::disabled(), &mut Unused)
            .unwrap()
            .is_some()
        {}
        results.push(serde_json::to_value(run.summary()).unwrap());
    }
    assert_eq!(results[0], results[1]);
}

#[test]
fn invalid_configuration_rejects_before_processing_and_long_plan_is_bounded() {
    let (settings, layout) =
        acquisition_support::acquisition_setup(SubcarrierSpacing::Khz25, false);
    let mut config = ChannelRunSettings::new(settings, layout);
    for duration in [
        Duration::ZERO,
        Duration::from_nanos(1),
        Duration::from_secs(3601),
    ] {
        config.duration = duration;
        assert!(ChannelRun::new(&config).is_err());
    }
    config.duration = Duration::from_secs(3600);
    let run = ChannelRun::new(&config).unwrap();
    assert_eq!(run.summary().timing.observation_samples, 3600 * 204_800_000);
    assert_eq!(run.summary().generated_payloads, 0);
    for seed in [0, 0x8000] {
        config.prbs_seed = seed;
        assert!(ChannelRun::new(&config).is_err());
    }
    config.prbs_seed = 1;
    config.receive_block_samples = 0;
    assert!(ChannelRun::new(&config).is_err());
    config.receive_block_samples = 4096;
    config.llr_variance = f64::NAN;
    assert!(ChannelRun::new(&config).is_err());
}

#[test]
fn observer_failure_closes_session_without_claiming_finished_or_zero_ber() {
    struct Failure;
    impl ProbeObserver for Failure {
        fn observe(&mut self, _event: &ProbeEvent<'_>) -> Result<(), SimulationError> {
            Err(SimulationError::new(
                "TestObserverFailure",
                "Injected observer failure.",
            ))
        }
    }
    let (settings, layout) =
        acquisition_support::acquisition_setup(SubcarrierSpacing::Khz50, false);
    let mut run = ChannelRun::new(&ChannelRunSettings::new(settings, layout)).unwrap();
    let mut probe = TestPointProbe::new(ProbeConfig {
        selections: vec![ProbeSelection {
            stage: TestPointId::TxIfft,
            elements: 0..1,
            source_symbols: None,
            channel_symbols: None,
        }],
        max_events: 1,
        max_elements_per_event: 1,
        max_total_elements: 1,
    })
    .unwrap();
    assert_eq!(
        run.advance(&mut probe, &mut Failure).unwrap_err().code,
        "TestObserverFailure"
    );
    assert_eq!(run.summary().phase, ChannelPhase::Failed);
    assert_eq!(run.summary().recovered_payloads, 0);
    assert_eq!(run.summary().payload_ber, None);
    assert_eq!(
        run.advance(&mut probe, &mut Failure).unwrap_err().code,
        "ChannelSessionClosed"
    );
}
