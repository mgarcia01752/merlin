/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual encoded data/NCP traverses waveform samples after acquired PLC configuration.
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    simulation::{
        payload::reference::{
            run_payload_waveform, run_payload_waveform_with_source, PayloadSource,
            PayloadWaveformExperiment,
        },
        test_points::{ProbeEvent, ProbeObserver, TestPointProbe},
    },
    transmission::{packing::NcpModulation, SubcarrierSpacing},
    LabError,
};
struct NoObserver;
impl ProbeObserver for NoObserver {
    fn observe(&mut self, _event: &ProbeEvent<'_>) -> Result<(), LabError> {
        panic!("disabled observer called");
    }
}
#[test]
fn both_spacings_recover_real_payloads_after_plc_acquisition_through_samples() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let (mut settings, layout) = acquisition_support::acquisition_setup(spacing, true);
        settings.time_interleaver_depth = spacing.max_time_interleaver_depth();
        let experiment = PayloadWaveformExperiment {
            settings,
            layout,
            ncp_modulation: NcpModulation::Qpsk,
            source_symbols: 160,
            payload_at_symbols: vec![0, 61, 126, 140],
            seed: 123,
            receive_block_samples: 137,
        };
        for source in [
            PayloadSource::Seeded { seed: 123 },
            PayloadSource::Prbs15 { seed: 0x7fff },
        ] {
            let result = run_payload_waveform_with_source(
                &experiment,
                source,
                &mut TestPointProbe::disabled(),
                &mut NoObserver,
            )
            .unwrap();
            assert_eq!(result.source, source);
            if matches!(source, PayloadSource::Prbs15 { .. }) {
                let comparison = result.prbs_comparison.as_ref().unwrap();
                assert_eq!(comparison.bits_tested, 4 * 14232);
                assert_eq!(comparison.bit_errors, 0);
                assert_eq!(comparison.first_error_bit, None);
            } else {
                assert!(result.prbs_comparison.is_none());
            }
            assert_eq!(result.source_payloads, 4);
            assert_eq!(result.received_payloads.len(), 4);
            assert_eq!(result.payload_bit_errors, 0);
            assert_eq!(result.pending_payload_bits, 0);
            assert_eq!(result.accepted_ncp_fields, 160);
            assert!(!result.ncp_ldpc_iterations.is_empty());
            assert_eq!(
                result.warmup_symbols,
                usize::from(spacing.max_time_interleaver_depth()) - 1
            );
            assert_eq!(result.drain_symbols, result.warmup_symbols);
            assert!(result.unavailable_tx_data_cells > 0);
            assert_eq!(result.waveform_remainder_samples, 128);
            assert_eq!(result.link.plc_bit_errors, 0);
            assert!(result.link.preambles_match);
            assert!(result.link.maximum_bin_error < 1e-11);
            assert!(result.link.plc_frames >= 1);
            assert!(result.received_payloads.iter().all(|word| word.ldpc_success
                && word.bch_success
                && word.bit_errors == 0
                && word.payload_bits == 14232));
            let reset = result.payload_origin_symbol + 128;
            assert!(result
                .received_payloads
                .iter()
                .any(|word| word.first_source_symbol < reset && word.last_source_symbol >= reset));
        }
    }
}
#[test]
fn malformed_reference_schedule_is_rejected_before_running() {
    let (settings, layout) =
        acquisition_support::acquisition_setup(SubcarrierSpacing::Khz25, false);
    let mut experiment = PayloadWaveformExperiment {
        settings,
        layout,
        ncp_modulation: NcpModulation::Qpsk,
        source_symbols: 10,
        payload_at_symbols: vec![5, 5],
        seed: 123,
        receive_block_samples: 137,
    };
    assert!(run_payload_waveform(
        &experiment,
        &mut TestPointProbe::disabled(),
        &mut NoObserver
    )
    .is_err());
    experiment.payload_at_symbols = vec![10];
    assert!(run_payload_waveform(
        &experiment,
        &mut TestPointProbe::disabled(),
        &mut NoObserver
    )
    .is_err());
    experiment.payload_at_symbols = vec![0];
    experiment.source_symbols = 513;
    assert!(run_payload_waveform(
        &experiment,
        &mut TestPointProbe::disabled(),
        &mut NoObserver
    )
    .is_err());
    experiment.source_symbols = 1;
    assert!(run_payload_waveform(
        &experiment,
        &mut TestPointProbe::disabled(),
        &mut NoObserver
    )
    .is_err());
}

#[test]
fn invalid_prbs_seed_is_rejected_before_waveform_work() {
    let (settings, layout) =
        acquisition_support::acquisition_setup(SubcarrierSpacing::Khz25, false);
    let experiment = PayloadWaveformExperiment {
        settings,
        layout,
        ncp_modulation: NcpModulation::Qpsk,
        source_symbols: 160,
        payload_at_symbols: vec![0],
        seed: 123,
        receive_block_samples: 137,
    };
    for seed in [0, 0x8000] {
        assert!(run_payload_waveform_with_source(
            &experiment,
            PayloadSource::Prbs15 { seed },
            &mut TestPointProbe::disabled(),
            &mut NoObserver,
        )
        .is_err());
    }
}
