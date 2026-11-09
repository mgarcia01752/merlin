/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Observation cannot change deterministic pilot stage values or retain engine state.
#![cfg(feature = "simulation")]
mod support;
use docsis_ofdm_fec::{
    simulation::{
        pilots::PilotGenerator,
        test_points::{
            ProbeConfig, ProbeContext, ProbeEvent, ProbeObserver, ProbeOutcome, ProbeSelection,
            RunPhase, TestPointId, TestPointProbe,
        },
    },
    transmission::SubcarrierSpacing,
    LabError,
};
#[derive(Default)]
struct Sink {
    events: Vec<serde_json::Value>,
    fail: bool,
}
impl ProbeObserver for Sink {
    /// Captures a small owned JSON snapshot for comparison in this test.
    /// # Arguments
    /// * `event` - Borrowed selected window; no scientific object can be modified.
    /// # Returns
    /// Unit after copying the selected values, or the injected failure.
    /// # Errors
    /// Returns TestObserverFailure when this test requests a failing sink.
    fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), LabError> {
        self.events.push(serde_json::to_value(event).unwrap());
        if self.fail {
            Err(LabError::new(
                "TestObserverFailure",
                "Injected sink failure.",
            ))
        } else {
            Ok(())
        }
    }
}
fn config(max_events: usize) -> ProbeConfig {
    ProbeConfig {
        selections: [
            TestPointId::PilotSequence,
            TestPointId::PilotPower,
            TestPointId::ContinuousPilots,
            TestPointId::ScatteredPilots,
        ]
        .map(|stage| ProbeSelection {
            stage,
            elements: 0..8,
            source_symbols: None,
            channel_symbols: Some(0..128),
        })
        .to_vec(),
        max_events,
        max_elements_per_event: 8,
        max_total_elements: 64,
    }
}
#[test]
fn enabled_disabled_and_exhausted_observers_preserve_scientific_results() {
    let generator = PilotGenerator::new(SubcarrierSpacing::Khz25);
    let layout = support::layout(SubcarrierSpacing::Khz25);
    let before = generator.generate(&layout, 0, 1.0).unwrap();
    let mut context = ProbeContext::new("demo", "fixed-grid", RunPhase::Observation).unwrap();
    context.channel_symbol_index = Some(0);
    context.plc_phase = Some(0);
    let mut sink = Sink::default();
    assert_eq!(
        generator
            .observe(
                &before,
                &context,
                &mut TestPointProbe::disabled(),
                &mut sink
            )
            .unwrap(),
        [ProbeOutcome::Disabled; 4]
    );
    assert!(sink.events.is_empty());
    let mut probe = TestPointProbe::new(config(4)).unwrap();
    let outcomes = generator
        .observe(&before, &context, &mut probe, &mut sink)
        .unwrap();
    assert!(outcomes
        .iter()
        .all(|outcome| matches!(outcome, ProbeOutcome::Dispatched { .. })));
    assert_eq!(sink.events.len(), 4);
    assert_eq!(sink.events[0]["stage"], "tx.pilots.sequence");
    assert_eq!(sink.events[1]["stage"], "tx.pilots.power");
    assert_eq!(sink.events[1]["data"]["values"]["pilot_amplitude"], 2.0);
    assert_eq!(sink.events[2]["context"]["reference"]["rms"], 1.0);
    assert_eq!(
        generator
            .observe(&before, &context, &mut probe, &mut sink)
            .unwrap(),
        [ProbeOutcome::BudgetExhausted; 4]
    );
    assert_eq!(sink.events.len(), 4);
    assert_eq!(generator.generate(&layout, 0, 1.0).unwrap(), before);
    let sequence = generator.sequence().bits().to_vec();
    for symbol in 1..128 {
        generator.generate(&layout, symbol, 1.0).unwrap();
    }
    assert_eq!(generator.sequence().bits(), sequence);
}
#[test]
fn callback_or_context_errors_leave_completed_outputs_available() {
    let generator = PilotGenerator::new(SubcarrierSpacing::Khz50);
    let layout = support::layout(SubcarrierSpacing::Khz50);
    let result = generator.generate(&layout, 3, 0.5).unwrap();
    let mut context = ProbeContext::new("demo", "fixed-grid", RunPhase::Warmup).unwrap();
    context.channel_symbol_index = Some(3);
    context.plc_phase = Some(3);
    let mut sink = Sink {
        fail: true,
        ..Default::default()
    };
    let mut probe = TestPointProbe::new(config(4)).unwrap();
    assert_eq!(
        generator
            .observe(&result, &context, &mut probe, &mut sink)
            .unwrap_err()
            .code,
        "TestObserverFailure"
    );
    assert_eq!(sink.events.len(), 1);
    assert_eq!(probe.stats().callback_errors, 1);
    assert_eq!(generator.generate(&layout, 3, 0.5).unwrap(), result);
    context.plc_phase = Some(4);
    assert!(generator
        .observe(&result, &context, &mut probe, &mut sink)
        .is_err());
    assert_eq!(sink.events.len(), 1);
    assert!(generator
        .generate(&support::layout(SubcarrierSpacing::Khz25), 3, 1.0)
        .is_err());
    context.plc_phase = Some(3);
    context.run_id = "x".repeat(129);
    assert!(generator
        .observe(
            &result,
            &context,
            &mut TestPointProbe::disabled(),
            &mut sink
        )
        .is_err());
    assert_eq!(sink.events.len(), 1);
}
