/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Post-computation observer failure cannot alter retained outputs or subsequent TX state.
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    signal_processing::waveform::WaveformGeometry,
    simulation::{
        test_points::{
            PhysicalProbeGrid, ProbeConfig, ProbeContext, ProbeEvent, ProbeObserver,
            ProbeSelection, RunPhase, SignalReference, TestPointId, TestPointProbe,
        },
        waveform::{observation, WaveformTransmitter},
    },
    transmission::{
        frame::assemble,
        layout::{CarrierLayout, PhysicalCarrierRole},
        SubcarrierSpacing,
    },
    SimulationError,
};
struct FailingSink;
impl ProbeObserver for FailingSink {
    fn observe(&mut self, _event: &ProbeEvent<'_>) -> Result<(), SimulationError> {
        Err(SimulationError::new(
            "TestObserverFailure",
            "Deliberate observer rejection.",
        ))
    }
}
#[test]
fn failed_callback_and_exhausted_budget_preserve_outputs_and_overlap_history() {
    let (settings, raw) = acquisition_support::acquisition_setup(SubcarrierSpacing::Khz50, false);
    let geometry = WaveformGeometry::new(&settings).unwrap();
    let layout = CarrierLayout::new(&settings, &raw).unwrap();
    let mut actual = WaveformTransmitter::new(&geometry, 0, 0);
    let mut baseline = WaveformTransmitter::new(&geometry, 0, 0);
    let mut context =
        ProbeContext::new("waveform-probe", "same-session", RunPhase::Observation).unwrap();
    context.channel_symbol_index = Some(0);
    context.plc_phase = Some(0);
    context.physical_grid = Some(PhysicalProbeGrid {
        spacing: SubcarrierSpacing::Khz50,
        zero_frequency_hz: raw.zero_frequency_hz,
    });
    context.reference = SignalReference::OtherSubcarrierRms { rms: 1.0 };
    let mut probe = TestPointProbe::new(ProbeConfig {
        selections: vec![ProbeSelection {
            stage: TestPointId::TxIfft,
            elements: 0..2,
            source_symbols: None,
            channel_symbols: None,
        }],
        max_events: 1,
        max_elements_per_event: 2,
        max_total_elements: 2,
    })
    .unwrap();
    for index in 0..2 {
        let reduced: Vec<_> = layout
            .symbol_roles(index)
            .iter()
            .filter_map(|role| match role {
                PhysicalCarrierRole::Data => Some(Some([1.0, 0.5])),
                PhysicalCarrierRole::ScatteredPilot => Some(None),
                _ => None,
            })
            .collect();
        let physical = assemble(&layout, index, &reduced, &[[1.0, 0.0]; 8], 1.0).unwrap();
        let result = actual.transmit(&physical).unwrap();
        let expected = baseline.transmit(&physical).unwrap();
        context.channel_symbol_index = Some(index);
        context.plc_phase = Some(index as u8);
        let mismatched =
            WaveformGeometry::new(&docsis_ofdm_fec::transmission::TransmissionSettings {
                subcarrier_spacing: SubcarrierSpacing::Khz50,
                cyclic_prefix_samples: 256,
                ..Default::default()
            })
            .unwrap();
        assert!(observation::observe_transmit(
            &result,
            &mismatched,
            &context,
            &mut probe,
            &mut FailingSink
        )
        .is_err());
        assert_eq!(probe.stats().dispatched_events, index as usize);
        let emitted = observation::observe_transmit(
            &result,
            &geometry,
            &context,
            &mut probe,
            &mut FailingSink,
        );
        if index == 0 {
            assert!(emitted.is_err());
        } else {
            assert!(emitted.is_ok());
        }
        assert_eq!(result.output, expected.output);
        assert_eq!(result.useful, expected.useful);
    }
    assert_eq!(actual.finish().unwrap(), baseline.finish().unwrap());
    assert_eq!(probe.stats().callback_errors, 1);
}
