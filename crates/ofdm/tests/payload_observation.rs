/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Probe selection, callback failure and exhaustion must not alter queued data or subsequent state.
#![cfg(feature = "simulation")]
mod acquisition_support;
mod payload_support;
use docsis_ofdm_fec::{
    simulation::{
        payload::{observation, PayloadReceiver, PayloadTransmitter},
        source::SeededSource,
        test_points::{
            CellProbeGrid, ProbeConfig, ProbeContext, ProbeEvent, ProbeObserver, ProbeSelection,
            RunPhase, TestPointId, TestPointProbe, TransmitCellRole,
        },
    },
    transmission::{packing::NcpModulation, SubcarrierSpacing},
    SimulationError,
};
use std::collections::BTreeSet;
struct Sink {
    fail: bool,
    stages: BTreeSet<TestPointId>,
    roles: Vec<TransmitCellRole>,
}
impl ProbeObserver for Sink {
    fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), SimulationError> {
        self.stages.insert(event.stage);
        assert!(event.shape[0] <= 2);
        assert!(event.context.cell_grid.is_some());
        if event.stage == TestPointId::PayloadMapped {
            self.roles = event.context.transmit_cell_roles.clone().unwrap();
            assert_eq!(self.roles.len(), event.total_elements);
        } else {
            assert!(event.context.transmit_cell_roles.is_none());
        }
        if self.fail {
            Err(SimulationError::new(
                "TestObserverFailure",
                "Deliberate callback rejection.",
            ))
        } else {
            Ok(())
        }
    }
}
fn run(mode: u8) -> (Vec<u8>, BTreeSet<TestPointId>) {
    let (settings, raw) = acquisition_support::acquisition_setup(SubcarrierSpacing::Khz50, false);
    let state = payload_support::ready(&settings, &raw, NcpModulation::Qpsk);
    let configuration = state.require_ready().unwrap();
    let grid = CellProbeGrid {
        settings: configuration.interleaver_settings().unwrap(),
        origin_symbol: 128,
        ncp_modulation: NcpModulation::Qpsk,
    };
    let stages = [
        TestPointId::PayloadCoded,
        TestPointId::PayloadLabels,
        TestPointId::PayloadMapped,
        TestPointId::PayloadInterleaved,
        TestPointId::PayloadRestored,
        TestPointId::NcpSoft,
        TestPointId::NcpAccepted,
        TestPointId::DataSoft,
        TestPointId::PayloadLdpcInput,
        TestPointId::PayloadDecoded,
    ];
    let mut probe = if mode == 0 {
        TestPointProbe::disabled()
    } else {
        TestPointProbe::new(ProbeConfig {
            selections: stages
                .into_iter()
                .map(|stage| ProbeSelection {
                    stage,
                    elements: 0..2,
                    source_symbols: None,
                    channel_symbols: None,
                })
                .collect(),
            max_events: if mode == 3 { 2 } else { 1000 },
            max_elements_per_event: 2,
            max_total_elements: if mode == 3 { 4 } else { 2000 },
        })
        .unwrap()
    };
    let mut sink = Sink {
        fail: mode == 2,
        stages: BTreeSet::new(),
        roles: Vec::new(),
    };
    let mut tx = PayloadTransmitter::new(&settings, &raw, NcpModulation::Qpsk, 128).unwrap();
    let mut rx = PayloadReceiver::new(&state, 128).unwrap();
    let payload = SeededSource::new(123).next_payload();
    let encoded = tx.enqueue(&payload).unwrap();
    let context =
        ProbeContext::new("payload-probe", "fixed-profile", RunPhase::Observation).unwrap();
    let observed = observation::observe_encoded(&encoded, grid, &context, &mut probe, &mut sink);
    if !sink.fail {
        observed.unwrap();
    }
    let mut recovered = Vec::new();
    for _ in 0..12 {
        let symbol = tx.transmit().unwrap();
        let observed =
            observation::observe_transmit(&symbol, grid, &context, &mut probe, &mut sink);
        if !sink.fail {
            observed.unwrap();
        }
        if mode == 1 {
            for (cell, role) in symbol.packed.cells.iter().zip(&sink.roles) {
                assert_eq!(
                    matches!(
                        cell,
                        docsis_ofdm_fec::transmission::packing::PackedCell::NcpReserved { .. }
                    ),
                    *role == TransmitCellRole::Ncp
                );
                assert_eq!(
                    matches!(
                        cell,
                        docsis_ofdm_fec::transmission::packing::PackedCell::ZeroLoadedFiller
                    ),
                    *role == TransmitCellRole::Filler
                );
            }
        }
        let received = rx
            .receive(
                &state,
                symbol.channel.channel_symbol_index,
                &symbol.channel.reduced,
                0.01,
                50,
            )
            .unwrap();
        let observed =
            observation::observe_receive(&received, grid, &context, &mut probe, &mut sink);
        if !sink.fail {
            observed.unwrap();
        }
        if let Some(decoded) = received.decoded {
            assert!(decoded.error.is_none());
            for word in decoded.recovered {
                recovered.extend_from_slice(&word.decoded.bch.codeword[..14232]);
            }
        }
    }
    while let Some(symbol) = tx.drain().unwrap() {
        let received = rx
            .receive(
                &state,
                symbol.channel_symbol_index,
                &symbol.reduced,
                0.01,
                50,
            )
            .unwrap();
        let observed =
            observation::observe_receive(&received, grid, &context, &mut probe, &mut sink);
        if !sink.fail {
            observed.unwrap();
        }
        if let Some(decoded) = received.decoded {
            assert!(decoded.error.is_none());
            for word in decoded.recovered {
                recovered.extend_from_slice(&word.decoded.bch.codeword[..14232]);
            }
        }
    }
    assert_eq!(recovered, payload);
    assert_eq!(rx.pending_bits(), 0);
    (recovered, sink.stages)
}
#[test]
fn disabled_enabled_failed_and_exhausted_probes_preserve_actual_payload() {
    let (baseline, disabled) = run(0);
    assert!(disabled.is_empty());
    let (observed, stages) = run(1);
    assert_eq!(observed, baseline);
    assert_eq!(stages.len(), 10);
    assert_eq!(run(2).0, baseline);
    let (exhausted, limited) = run(3);
    assert_eq!(exhausted, baseline);
    assert_eq!(limited.len(), 2);
}
