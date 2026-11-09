/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded outputs, original frequency offsets and callback/scientific independence.
#![cfg(feature = "simulation")]
mod support;
use docsis_ofdm_fec::{
    simulation::{
        plc::{generate_frame, observation::observe_frequency_symbol},
        test_points::{
            ProbeConfig, ProbeContext, ProbeEvent, ProbeObserver, ProbeSelection, RunPhase,
            TestPointId, TestPointProbe,
        },
    },
    transmission::{
        frame::assemble,
        layout::PhysicalCarrierRole,
        plc::{ExtendedTimestamp, PlcMessageFrame},
        SubcarrierSpacing,
    },
    LabError,
};
#[derive(Default)]
struct Sink {
    events: Vec<serde_json::Value>,
    fail: bool,
}
impl ProbeObserver for Sink {
    /// Copies only the bounded selected event into test-owned storage.
    /// # Arguments
    /// * `event` - Immutable stage window retaining full coordinate metadata.
    /// # Returns
    /// Unit after one copy.
    /// # Errors
    /// Returns the deliberately injected callback error when requested.
    fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), LabError> {
        if self.fail {
            return Err(LabError::new("TestObserver", "Injected sink failure."));
        }
        self.events.push(serde_json::to_value(event).unwrap());
        Ok(())
    }
}
#[test]
fn all_plc_stage_producers_observe_bounded_windows_without_changing_results() {
    let frame = generate_frame(
        PlcMessageFrame::idle(
            SubcarrierSpacing::Khz25,
            ExtendedTimestamp::new(1, 0, 0).unwrap(),
        )
        .unwrap(),
        0,
    )
    .unwrap();
    let before = serde_json::to_value(&frame).unwrap();
    let mut context = ProbeContext::new("plc-test", "known-config", RunPhase::Observation).unwrap();
    context.channel_symbol_index = Some(0);
    context.plc_phase = Some(0);
    let stages = [
        TestPointId::PlcMessage,
        TestPointId::PlcCodedBits,
        TestPointId::PlcCodedNibbles,
        TestPointId::PlcInterleaved,
        TestPointId::PlcRandomized,
        TestPointId::PlcIq,
        TestPointId::PlcPreamble,
    ];
    let config = || ProbeConfig {
        selections: stages
            .iter()
            .map(|stage| ProbeSelection {
                stage: *stage,
                elements: 1..5,
                source_symbols: None,
                channel_symbols: None,
            })
            .collect(),
        max_events: 7,
        max_elements_per_event: 4,
        max_total_elements: 28,
    };
    let mut probe = TestPointProbe::new(config()).unwrap();
    let mut sink = Sink::default();
    frame.observe(&context, &mut probe, &mut sink).unwrap();
    // First six distinct boundaries consume6 events; the next codeword's first
    // stage consumes the final budget. A separate filter selects the preamble.
    assert_eq!(sink.events.len(), 7);
    assert_eq!(probe.stats().dispatched_elements, 28);
    for event in &sink.events {
        assert_eq!(event["element_start"], 1);
        assert_eq!(event["shape"], serde_json::json!([4]));
    }
    let mut preamble = TestPointProbe::new(ProbeConfig {
        selections: vec![ProbeSelection {
            stage: TestPointId::PlcPreamble,
            elements: 0..16,
            source_symbols: None,
            channel_symbols: Some(120..121),
        }],
        max_events: 1,
        max_elements_per_event: 16,
        max_total_elements: 16,
    })
    .unwrap();
    frame.observe(&context, &mut preamble, &mut sink).unwrap();
    let event = sink.events.last().unwrap();
    assert_eq!(event["stage"], "tx.plc.preamble");
    assert_eq!(event["context"]["plc_grid"]["symbols"], 1);
    let mut disabled = TestPointProbe::disabled();
    let mut failed = Sink {
        fail: true,
        ..Default::default()
    };
    frame.observe(&context, &mut disabled, &mut failed).unwrap();
    assert!(failed.events.is_empty());
    let mut enabled = TestPointProbe::new(config()).unwrap();
    assert!(frame.observe(&context, &mut enabled, &mut failed).is_err());
    assert_eq!(enabled.stats().callback_errors, 1);
    assert_eq!(serde_json::to_value(&frame).unwrap(), before);
    context.channel_symbol_index = Some(1);
    assert!(frame.observe(&context, &mut disabled, &mut sink).is_err());
}
#[test]
fn physical_window_retains_full_grid_origin_and_missing_history() {
    let layout = support::layout(SubcarrierSpacing::Khz50);
    let roles = layout.symbol_roles(0);
    let reduced: Vec<_> = roles
        .iter()
        .filter(|role| {
            matches!(
                role,
                PhysicalCarrierRole::Data | PhysicalCarrierRole::ScatteredPilot
            )
        })
        .map(|_| None)
        .collect();
    let symbol = assemble(&layout, 0, &reduced, &[[1.0, 0.0]; 8], 0.5).unwrap();
    let before = symbol.bins.clone();
    let mut context = ProbeContext::new("test", "test", RunPhase::Warmup).unwrap();
    context.channel_symbol_index = Some(0);
    context.plc_phase = Some(0);
    let mut probe = TestPointProbe::new(ProbeConfig {
        selections: vec![ProbeSelection {
            stage: TestPointId::PhysicalBins,
            elements: 1..9,
            source_symbols: None,
            channel_symbols: Some(0..1),
        }],
        max_events: 1,
        max_elements_per_event: 8,
        max_total_elements: 8,
    })
    .unwrap();
    let mut sink = Sink::default();
    observe_frequency_symbol(&symbol, &context, &mut probe, &mut sink).unwrap();
    let event = &sink.events[0];
    assert_eq!(event["element_start"], 1);
    assert_eq!(event["total_elements"], 4096);
    assert_eq!(
        event["context"]["physical_grid"]["zero_frequency_hz"],
        100_000_000
    );
    assert_eq!(event["context"]["physical_grid"]["spacing"], "50000");
    assert_eq!(event["data"]["values"][0], serde_json::Value::Null);
    assert_eq!(symbol.bins, before);
}
