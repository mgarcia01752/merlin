/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    simulation::test_points::{
        ProbeConfig, ProbeContext, ProbeData, ProbeEvent, ProbeObserver, ProbeOutcome,
        ProbeSelection, RunPhase, SignalDomain, SignalReference, TestPointId, TestPointProbe,
    },
    LabError,
};
#[derive(Default)]
struct Sink {
    events: Vec<serde_json::Value>,
    fail: bool,
}
impl ProbeObserver for Sink {
    /// Copies a selected event or returns an injected observation failure.
    /// # Arguments
    /// * `event` - Immutable selected data window.
    /// # Returns
    /// Unit when the test sink accepts the copied event.
    /// # Errors
    /// Returns TestObserverFailure when failure injection is enabled.
    fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), LabError> {
        self.events.push(serde_json::to_value(event).unwrap());
        if self.fail {
            Err(LabError::new(
                "TestObserverFailure",
                "Selected sink failed.",
            ))
        } else {
            Ok(())
        }
    }
}
fn config() -> ProbeConfig {
    ProbeConfig {
        selections: vec![ProbeSelection {
            stage: TestPointId::PilotSequence,
            elements: 1..5,
            source_symbols: None,
            channel_symbols: Some(2..4),
        }],
        max_events: 2,
        max_elements_per_event: 8,
        max_total_elements: 8,
    }
}
fn context() -> ProbeContext {
    let mut context = ProbeContext::new("run1", "configuration1", RunPhase::Observation).unwrap();
    context.channel_symbol_index = Some(2);
    context.plc_phase = Some(2);
    context
}
#[test]
fn selection_clips_available_elements_preserves_metadata_and_never_invents_time() {
    let mut probe = TestPointProbe::new(config()).unwrap();
    let mut sink = Sink::default();
    let mut context = context();
    let data = [0, 1, 0, 1];
    let original = data;
    let result = probe
        .emit(
            TestPointId::PilotSequence,
            &context,
            SignalDomain::PhysicalFftBin,
            ProbeData::Bits(&data),
            &mut sink,
        )
        .unwrap();
    assert_eq!(result, ProbeOutcome::Dispatched { elements: 3 });
    assert_eq!(sink.events[0]["element_start"], 1);
    assert_eq!(sink.events[0]["total_elements"], 4);
    assert_eq!(sink.events[0]["shape"], serde_json::json!([3]));
    assert_eq!(
        sink.events[0]["data"]["values"],
        serde_json::json!([1, 0, 1])
    );
    assert_eq!(
        sink.events[0]["context"]["source_symbol_index"],
        serde_json::Value::Null
    );
    context.channel_symbol_index = None;
    assert_eq!(
        probe
            .emit(
                TestPointId::PilotSequence,
                &context,
                SignalDomain::PhysicalFftBin,
                ProbeData::Bits(&data),
                &mut sink
            )
            .unwrap(),
        ProbeOutcome::Filtered
    );
    context.channel_symbol_index = Some(4);
    assert_eq!(
        probe
            .emit(
                TestPointId::PilotSequence,
                &context,
                SignalDomain::PhysicalFftBin,
                ProbeData::Bits(&data),
                &mut sink
            )
            .unwrap(),
        ProbeOutcome::Filtered
    );
    assert_eq!(data, original);
    assert_eq!(sink.events.len(), 1);
}
#[test]
fn whole_event_budgets_and_callback_failures_are_explicit_and_bounded() {
    let mut configuration = config();
    configuration.max_total_elements = 3;
    configuration.max_events = 1;
    let mut probe = TestPointProbe::new(configuration).unwrap();
    let mut sink = Sink {
        fail: true,
        ..Default::default()
    };
    let context = context();
    let data = [0, 1, 0, 1];
    let result = probe.emit(
        TestPointId::PilotSequence,
        &context,
        SignalDomain::PhysicalFftBin,
        ProbeData::Bits(&data),
        &mut sink,
    );
    assert_eq!(result.unwrap_err().code, "TestObserverFailure");
    assert_eq!(probe.stats().dispatched_events, 1);
    assert_eq!(probe.stats().callback_errors, 1);
    assert_eq!(
        probe
            .emit(
                TestPointId::PilotSequence,
                &context,
                SignalDomain::PhysicalFftBin,
                ProbeData::Bits(&data),
                &mut sink
            )
            .unwrap(),
        ProbeOutcome::BudgetExhausted
    );
    assert_eq!(sink.events.len(), 1);
    let mut configuration = config();
    configuration.max_total_elements = 2;
    let mut probe = TestPointProbe::new(configuration).unwrap();
    assert_eq!(
        probe
            .emit(
                TestPointId::PilotSequence,
                &context,
                SignalDomain::PhysicalFftBin,
                ProbeData::Bits(&data),
                &mut sink
            )
            .unwrap(),
        ProbeOutcome::BudgetExhausted
    );
    assert_eq!(probe.stats().dispatched_events, 0);
}
#[test]
fn disabled_or_filtered_paths_do_not_call_observers_or_retain_data() {
    let mut context = context();
    context.run_id.clear();
    let mut probe = TestPointProbe::disabled();
    let mut sink = Sink::default();
    assert_eq!(
        probe
            .emit(
                TestPointId::PilotSequence,
                &context,
                SignalDomain::PhysicalFftBin,
                ProbeData::Bits(&[9]),
                &mut sink
            )
            .unwrap(),
        ProbeOutcome::Disabled
    );
    let mut enabled = TestPointProbe::new(config()).unwrap();
    assert_eq!(
        enabled
            .emit(
                TestPointId::ScatteredPilots,
                &context,
                SignalDomain::PhysicalFftBin,
                ProbeData::Bits(&[9]),
                &mut sink
            )
            .unwrap(),
        ProbeOutcome::Filtered
    );
    assert!(sink.events.is_empty());
    assert_eq!(probe.stats().dispatched_elements, 0);
}
#[test]
fn invalid_limits_windows_context_and_payload_are_rejected_before_dispatch() {
    for failure in 0..8 {
        let mut configuration = config();
        match failure {
            0 => configuration.max_events = 0,
            1 => configuration.max_events = 100001,
            2 => configuration.max_elements_per_event = 65537,
            3 => configuration.max_total_elements = 1048577,
            4 => configuration.selections[0].elements = 3..3,
            5 => configuration.selections[0].source_symbols = Some(3..3),
            6 => configuration
                .selections
                .push(configuration.selections[0].clone()),
            _ => configuration.selections.clear(),
        }
        assert!(TestPointProbe::new(configuration).is_err());
    }
    assert!(ProbeContext::new("", "cfg", RunPhase::Observation).is_err());
    assert!(ProbeContext::new(&"x".repeat(129), "cfg", RunPhase::Observation).is_err());
    let mut probe = TestPointProbe::new(config()).unwrap();
    let mut sink = Sink::default();
    let mut context = context();
    context.plc_phase = Some(128);
    assert!(probe
        .emit(
            TestPointId::PilotSequence,
            &context,
            SignalDomain::PhysicalFftBin,
            ProbeData::Bits(&[0, 1]),
            &mut sink
        )
        .is_err());
    context.plc_phase = Some(2);
    context.reference = SignalReference::OtherSubcarrierRms { rms: f64::NAN };
    assert!(probe
        .emit(
            TestPointId::PilotSequence,
            &context,
            SignalDomain::PhysicalFftBin,
            ProbeData::Bits(&[0, 1]),
            &mut sink
        )
        .is_err());
    context.reference = SignalReference::NotApplicable;
    assert!(probe
        .emit(
            TestPointId::PilotSequence,
            &context,
            SignalDomain::Parameter,
            ProbeData::Bits(&[0, 1]),
            &mut sink
        )
        .is_err());
    assert!(probe
        .emit(
            TestPointId::PilotSequence,
            &context,
            SignalDomain::PhysicalFftBin,
            ProbeData::Bits(&[0, 9]),
            &mut sink
        )
        .is_err());
    assert_eq!(probe.stats().dispatched_events, 0);
    assert!(sink.events.is_empty());
}
