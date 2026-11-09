/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Both spacings recover actual PLC messages from windowed time-domain samples.
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    simulation::{
        test_points::{
            ProbeConfig, ProbeEvent, ProbeObserver, ProbeSelection, SignalDirection, TestPointId,
            TestPointProbe,
        },
        waveform::reference::{run_reference_waveform, ReferenceExperiment},
    },
    transmission::{plc::receiver_configuration::ReceiverConfigurationStatus, SubcarrierSpacing},
    SimulationError,
};
struct Sink {
    events: usize,
    seen_tx: bool,
    seen_rx: bool,
    stages: std::collections::BTreeSet<TestPointId>,
}
impl ProbeObserver for Sink {
    fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), SimulationError> {
        self.events += 1;
        self.stages.insert(event.stage);
        self.seen_tx |= event.direction == SignalDirection::Transmit;
        self.seen_rx |= event.direction == SignalDirection::Receive;
        assert!(event.shape[0] <= 3);
        assert_eq!(event.element_start, 1);
        let waveform = event.context.waveform_grid.as_ref().unwrap();
        assert_eq!(waveform.sample_rate_hz, 204_800_000);
        assert!(event.context.sample_start_index.is_some());
        assert!(event.total_elements >= event.shape[0]);
        Ok(())
    }
}
#[test]
fn actual_two_frame_waveform_recovers_pilots_plc_and_configuration_with_zero_errors() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let (mut settings, mut layout) = acquisition_support::acquisition_setup(spacing, true);
        settings.occupied_bandwidth_hz = 192_000_000;
        layout.first_active_bin = spacing.mandatory_excluded_edge_bins() as u16;
        layout.active_end_exclusive =
            (spacing.fft_samples() as usize - spacing.mandatory_excluded_edge_bins()) as u16;
        let experiment = ReferenceExperiment {
            settings,
            layout,
            frames: 2,
            receive_block_samples: 137,
            first_sample: 29,
            reference_rms: 0.7,
        };
        let mut sink = Sink {
            events: 0,
            seen_tx: false,
            seen_rx: false,
            stages: Default::default(),
        };
        let baseline =
            run_reference_waveform(&experiment, &mut TestPointProbe::disabled(), &mut sink)
                .unwrap();
        assert_eq!(sink.events, 0);
        assert_eq!(baseline.transmitted_symbols, 256);
        assert_eq!(baseline.received_symbols, 256);
        assert_eq!(baseline.compared_bins, 256 * spacing.fft_samples() as u64);
        assert!(baseline.compared_pilots > 0);
        assert!(baseline.maximum_bin_error < 1e-11, "{baseline:?}");
        assert_eq!(baseline.information_bit_errors, 0);
        assert!(baseline.compared_information_bits > 0);
        assert_eq!(baseline.plc_frames_decoded, 2);
        assert!(baseline.preambles_match);
        assert!(!baseline.ldpc_iterations.is_empty());
        assert_eq!(
            baseline.receiver_configuration,
            ReceiverConfigurationStatus::Ready
        );
        assert_eq!(baseline.drain_samples, 128);
        assert_eq!(baseline.pending_rx_samples, 128);
        let stages = [
            TestPointId::TxIfft,
            TestPointId::TxPrefix,
            TestPointId::TxWindow,
            TestPointId::TxOverlap,
            TestPointId::ChannelInput,
            TestPointId::ChannelOutput,
            TestPointId::RxFramed,
            TestPointId::RxUseful,
            TestPointId::RxFft,
            TestPointId::RxEqualized,
            TestPointId::RxPlc,
            TestPointId::RxPilots,
            TestPointId::RxReduced,
        ];
        let mut probe = TestPointProbe::new(ProbeConfig {
            selections: stages
                .into_iter()
                .map(|stage| ProbeSelection {
                    stage,
                    elements: 1..4,
                    source_symbols: None,
                    channel_symbols: Some(0..1),
                })
                .collect(),
            max_events: 13,
            max_elements_per_event: 3,
            max_total_elements: 39,
        })
        .unwrap();
        let observed = run_reference_waveform(&experiment, &mut probe, &mut sink).unwrap();
        assert_eq!(observed, baseline);
        assert_eq!(probe.stats().dispatched_elements, 39);
        assert!(sink.seen_tx);
        // Longer chunks allow all thirteen stages, including RX, to fit into the same finite budget.
        sink.stages.clear();
        let mut single_block = experiment;
        single_block.receive_block_samples = 65_536;
        let mut probe = TestPointProbe::new(ProbeConfig {
            selections: stages
                .into_iter()
                .map(|stage| ProbeSelection {
                    stage,
                    elements: 1..4,
                    source_symbols: None,
                    channel_symbols: Some(0..1),
                })
                .collect(),
            max_events: 13,
            max_elements_per_event: 3,
            max_total_elements: 39,
        })
        .unwrap();
        let again = run_reference_waveform(&single_block, &mut probe, &mut sink).unwrap();
        assert_eq!(again, baseline);
        assert_eq!(probe.stats().dispatched_events, 13);
        assert_eq!(sink.stages.len(), 13);
        assert!(sink.seen_tx && sink.seen_rx);
    }
}
#[test]
fn invalid_reference_work_limits_are_rejected() {
    let (settings, layout) =
        acquisition_support::acquisition_setup(SubcarrierSpacing::Khz50, false);
    let mut experiment = ReferenceExperiment {
        settings,
        layout,
        frames: 0,
        receive_block_samples: 137,
        first_sample: 0,
        reference_rms: 1.0,
    };
    let mut sink = Sink {
        events: 0,
        seen_tx: false,
        seen_rx: false,
        stages: Default::default(),
    };
    let mut probe = TestPointProbe::disabled();
    assert!(run_reference_waveform(&experiment, &mut probe, &mut sink).is_err());
    experiment.frames = 1;
    experiment.receive_block_samples = 0;
    assert!(run_reference_waveform(&experiment, &mut probe, &mut sink).is_err());
    experiment.receive_block_samples = 137;
    experiment.first_sample = u64::MAX;
    assert!(run_reference_waveform(&experiment, &mut probe, &mut sink).is_err());
}
