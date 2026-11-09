/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Real pilot values and selected typed observations, without PLC or waveform.
use docsis_ofdm_fec::{
    simulation::{
        pilots::PilotGenerator,
        test_points::{
            ProbeConfig, ProbeContext, ProbeEvent, ProbeObserver, ProbeSelection, RunPhase,
            TestPointId, TestPointProbe,
        },
    },
    transmission::{
        layout::{CarrierLayout, CarrierLayoutSettings},
        TransmissionSettings,
    },
    LabError,
};
struct JsonSink {
    events: Vec<serde_json::Value>,
}
impl ProbeObserver for JsonSink {
    /// Copies selected borrowed values into bounded example-owned JSON events.
    /// # Arguments
    /// * `event` - A selected stage window valid only during this call.
    /// # Returns
    /// Unit after serializing the event; the probe limits the number and size.
    /// # Errors
    /// Reports serialization failure without changing completed pilot results.
    fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), LabError> {
        self.events.push(
            serde_json::to_value(event)
                .map_err(|error| LabError::new("ExampleSerialization", error.to_string()))?,
        );
        Ok(())
    }
}
/// Prints pilot outputs and four bounded test-point windows for an explicit grid.
/// # Returns
/// Unit after writing JSON to stdout. Additional continuous pilots are supplied
/// manually; no claim of automatic pilot placement or full frame assembly.
/// # Errors
/// Propagates layout, signal, observation and serialization errors.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transmission = TransmissionSettings {
        occupied_bandwidth_hz: 24_000_000,
        ..Default::default()
    };
    let layout = CarrierLayout::new(
        &transmission,
        &CarrierLayoutSettings {
            zero_frequency_hz: 100_000_000,
            first_active_bin: 0,
            active_end_exclusive: 960,
            plc_first_bin: 432,
            excluded_bins: vec![800],
            additional_continuous_pilots: vec![32, 128, 224, 600, 688, 760, 856, 936],
        },
    )?;
    let generator = PilotGenerator::new(transmission.subcarrier_spacing);
    let result = generator.generate(&layout, 0, 1.0)?;
    let mut context =
        ProbeContext::new("pilot-demo", "explicit-24mhz-grid", RunPhase::Observation)?;
    context.channel_symbol_index = Some(0);
    context.plc_phase = Some(0);
    let mut probe = TestPointProbe::new(ProbeConfig {
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
            channel_symbols: Some(0..1),
        })
        .to_vec(),
        max_events: 4,
        max_elements_per_event: 8,
        max_total_elements: 32,
    })?;
    let mut sink = JsonSink { events: Vec::new() };
    let outcomes = generator.observe(&result, &context, &mut probe, &mut sink)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({"experiment":"physical_pilot_stages",
        "pilot_stages":result,"test_points":sink.events,"observation_outcomes":outcomes,"probe_stats":probe.stats()}))?
    );
    Ok(())
}
