/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded windows retain grid meaning, and failed observers cannot mutate stages.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    simulation::{
        plc::PlcInterleavingStages,
        test_points::{
            ProbeConfig, ProbeContext, ProbeData, ProbeEvent, ProbeObserver, ProbeOutcome,
            ProbeSelection, RunPhase, SignalDomain, TestPointId, TestPointProbe,
        },
    },
    transmission::SubcarrierSpacing,
    SimulationError,
};
#[derive(Default)]
struct Sink {
    events: Vec<serde_json::Value>,
    fail: bool,
}
impl ProbeObserver for Sink {
    /// Copies a bounded selected event for assertions or injects an observer error.
    /// # Arguments
    /// * `event` - Immutable label window with its original full-grid metadata.
    /// # Returns
    /// Unit after copying; no scientific state is exposed mutably.
    /// # Errors
    /// Returns TestObserverFailure when requested by this test.
    fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), SimulationError> {
        self.events.push(serde_json::to_value(event).unwrap());
        if self.fail {
            return Err(SimulationError::new(
                "TestObserverFailure",
                "Injected observer error.",
            ));
        }
        Ok(())
    }
}
fn probe() -> TestPointProbe {
    TestPointProbe::new(ProbeConfig {
        selections: [TestPointId::PlcCodedNibbles, TestPointId::PlcInterleaved]
            .map(|stage| ProbeSelection {
                stage,
                elements: 14..18,
                source_symbols: None,
                channel_symbols: Some(0..1),
            })
            .to_vec(),
        max_events: 2,
        max_elements_per_event: 4,
        max_total_elements: 8,
    })
    .unwrap()
}
fn context() -> ProbeContext {
    let mut context = ProbeContext::new("test", "fixed25k", RunPhase::Observation).unwrap();
    context.channel_symbol_index = Some(0);
    context.plc_phase = Some(0);
    context
}
#[test]
fn selected_windows_keep_grid_metadata_and_leave_scientific_results_unchanged() {
    let bits = std::array::from_fn::<_, 384, _>(|index| u8::from(index % 3 == 0));
    let stages = PlcInterleavingStages::generate(SubcarrierSpacing::Khz25, 0, &bits).unwrap();
    let original = stages.clone();
    let mut sink = Sink::default();
    assert_eq!(
        stages
            .observe(&context(), &mut TestPointProbe::disabled(), &mut sink)
            .unwrap(),
        [ProbeOutcome::Disabled; 2]
    );
    assert!(sink.events.is_empty());
    let mut probe = probe();
    assert_eq!(
        stages.observe(&context(), &mut probe, &mut sink).unwrap(),
        [ProbeOutcome::Dispatched { elements: 4 }; 2]
    );
    assert_eq!(sink.events[0]["domain"], "codeword_cell");
    assert_eq!(sink.events[1]["domain"], "plc_cell");
    assert_eq!(sink.events[1]["units"], "qam_label");
    assert_eq!(sink.events[1]["shape"], serde_json::json!([4]));
    assert_eq!(sink.events[1]["element_start"], 14);
    assert_eq!(sink.events[1]["total_elements"], 96);
    assert_eq!(sink.events[1]["context"]["plc_grid"]["symbols"], 6);
    assert_eq!(
        sink.events[1]["context"]["plc_grid"]["subcarriers_per_symbol"],
        16
    );
    assert_eq!(sink.events[1]["context"]["plc_grid"]["spacing"], "25000");
    assert_eq!(
        sink.events[1]["data"]["values"],
        serde_json::json!(&stages.interleaved().values()[14..18])
    );
    assert_eq!(
        stages.observe(&context(), &mut probe, &mut sink).unwrap(),
        [ProbeOutcome::BudgetExhausted; 2]
    );
    assert_eq!(stages, original);
}
#[test]
fn bad_phase_context_payload_or_callback_never_changes_completed_science() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        for phase in [1, 120, 127] {
            assert!(PlcInterleavingStages::generate(spacing, phase, &[0; 384]).is_err());
        }
        assert!(PlcInterleavingStages::generate(spacing, 128, &[0; 384]).is_ok());
    }
    let stages = PlcInterleavingStages::generate(SubcarrierSpacing::Khz25, 0, &[1; 384]).unwrap();
    let before = stages.clone();
    let mut sink = Sink {
        fail: true,
        ..Default::default()
    };
    let mut probe = probe();
    assert_eq!(
        stages
            .observe(&context(), &mut probe, &mut sink)
            .unwrap_err()
            .code,
        "TestObserverFailure"
    );
    assert_eq!(probe.stats().callback_errors, 1);
    assert_eq!(sink.events.len(), 1);
    let mut mismatched = context();
    mismatched.plc_phase = Some(1);
    assert!(stages.observe(&mismatched, &mut probe, &mut sink).is_err());
    let mut raw = probe_for_one();
    // The generic dispatcher also rejects label events without full grid metadata.
    assert!(raw
        .emit(
            TestPointId::PlcInterleaved,
            &context(),
            SignalDomain::PlcCell,
            ProbeData::Labels(&[0; 96]),
            &mut sink
        )
        .is_err());
    let mut grid_context = context();
    grid_context.plc_grid = Some(docsis_ofdm_fec::simulation::test_points::PlcProbeGrid::new(
        SubcarrierSpacing::Khz25,
    ));
    assert!(raw
        .emit(
            TestPointId::PlcInterleaved,
            &grid_context,
            SignalDomain::PlcCell,
            ProbeData::Labels(&[16; 96]),
            &mut sink
        )
        .is_err());
    assert!(raw
        .emit(
            TestPointId::PlcInterleaved,
            &grid_context,
            SignalDomain::CodewordCell,
            ProbeData::Labels(&[0; 96]),
            &mut sink
        )
        .is_err());
    assert_eq!(raw.stats().dispatched_events, 0);
    assert_eq!(sink.events.len(), 1);
    assert_eq!(stages, before);
}
fn probe_for_one() -> TestPointProbe {
    TestPointProbe::new(ProbeConfig {
        selections: vec![ProbeSelection {
            stage: TestPointId::PlcInterleaved,
            elements: 0..1,
            source_symbols: None,
            channel_symbols: None,
        }],
        max_events: 1,
        max_elements_per_event: 1,
        max_total_elements: 1,
    })
    .unwrap()
}
