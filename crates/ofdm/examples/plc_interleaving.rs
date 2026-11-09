/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded coded/interleaved snapshots with explicit grid metadata at both spacings.
use docsis_ofdm_fec::{
    simulation::{
        plc::PlcInterleavingStages,
        test_points::{
            ProbeConfig, ProbeContext, ProbeEvent, ProbeObserver, ProbeSelection, RunPhase,
            TestPointId, TestPointProbe,
        },
    },
    transmission::{
        plc::{ExtendedTimestamp, PlcBlockCodec, PlcMessageFrame},
        SubcarrierSpacing,
    },
    LabError,
};
#[derive(Default)]
struct Sink {
    events: Vec<serde_json::Value>,
}
impl ProbeObserver for Sink {
    /// Copies a selected immutable event into a bounded example-owned JSON value.
    /// # Arguments
    /// * `event` - Window with full grid metadata, not the entire frame.
    /// # Returns
    /// Unit after copying; probe budgets cap output volume.
    /// # Errors
    /// Propagates JSON serialization errors without changing stage outputs.
    fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), LabError> {
        self.events.push(
            serde_json::to_value(event)
                .map_err(|error| LabError::new("ExampleSerialization", error.to_string()))?,
        );
        Ok(())
    }
}
/// Emits two fixed-size PLC interleaving examples after real message FEC encoding.
/// # Returns
/// Unit with coded labels, time-major labels, row boundaries and two probe windows.
/// # Errors
/// Propagates codec, message, stage, observation or JSON failures.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let codec = PlcBlockCodec::new()?;
    let mut results = Vec::new();
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let frame = PlcMessageFrame::idle(spacing, ExtendedTimestamp::new(123, 19, 0)?)?;
        let information = frame.information_blocks()?;
        let encoded = codec.encode(&information[0])?;
        let stages = PlcInterleavingStages::generate(spacing, 0, &encoded.transmitted_bits)?;
        let mut context = ProbeContext::new(
            "plc-demo",
            &format!("plc-{}-hz", spacing.frequency_hz()),
            RunPhase::Observation,
        )?;
        context.channel_symbol_index = Some(0);
        context.plc_phase = Some(0);
        let mut probe = TestPointProbe::new(ProbeConfig {
            selections: [TestPointId::PlcCodedNibbles, TestPointId::PlcInterleaved]
                .map(|stage| ProbeSelection {
                    stage,
                    elements: 0..16,
                    source_symbols: None,
                    channel_symbols: Some(0..1),
                })
                .to_vec(),
            max_events: 2,
            max_elements_per_event: 16,
            max_total_elements: 32,
        })?;
        let mut sink = Sink::default();
        let outcomes = stages.observe(&context, &mut probe, &mut sink)?;
        let symbols: Vec<_> = (0..frame.geometry().data_symbols_per_codeword)
            .map(|symbol| stages.interleaved().symbol(symbol))
            .collect();
        results.push(serde_json::json!({"spacing_hz":spacing.frequency_hz(),"stages":stages,"symbol_rows":symbols,"test_points":sink.events,"observation_outcomes":outcomes,"probe_stats":probe.stats()}));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &serde_json::json!({"experiment":"plc_block_interleaving","blocks":results})
        )?
    );
    Ok(())
}
