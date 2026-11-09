/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded reference demonstration using existing raw/equalized receiver probes.
use docsis_ofdm_fec::{
    scenario::document::ScenarioDocument,
    signal_processing::channel::impairments::NoiseModel,
    simulation::{
        channel::{document::ChannelDocument, ChannelRun},
        test_points::{
            ProbeConfig, ProbeData, ProbeEvent, ProbeObserver, ProbeSelection, TestPointId,
            TestPointProbe,
        },
    },
    transmission::{
        layout::{CarrierLayout, PhysicalCarrierRole},
        plc::{geometry::PLC_DATA_SYMBOLS, PlcFrameGeometry},
    },
    types::{FrequencyHz, IqPair, SubcarrierIdx, PLC_FRAME_SYMBOLS},
    SimulationError,
};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    fs::OpenOptions,
    io::{Read, Write},
};
const MAX_INPUT_BYTES: u64 = 16 * 1024 * 1024;
const CONTEXT_BINS: usize = 256;
const NUMERICAL_TOLERANCE: f64 = 1e-8;
#[derive(Serialize)]
struct Pair {
    physical_bin: SubcarrierIdx,
    frequency_hz: FrequencyHz,
    channel_symbol_index: u64,
    before_equalization: IqPair,
    after_equalization: IqPair,
}
struct Collector {
    layout: CarrierLayout,
    pending: Option<(u64, usize, Vec<IqPair>)>,
    response: Vec<IqPair>,
    streams: BTreeMap<&'static str, Vec<Pair>>,
}
impl ProbeObserver for Collector {
    /// Copies one selected raw window or pairs its matching corrected window.
    /// # Arguments
    /// * `event` - Borrowed selected FFT or equalized physical-bin window.
    /// # Returns
    /// Unit after retaining only the four protected-signal classes.
    /// # Errors
    /// Rejects unmatched windows, missing coordinates or invalid noiseless symbols.
    fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), SimulationError> {
        let ProbeData::Iq(points) = event.data else {
            return Err(failure("Expected receiver I/Q window."));
        };
        let symbol = event
            .context
            .channel_symbol_index
            .ok_or_else(|| failure("Missing symbol."))?;
        if event.stage == TestPointId::RxFft {
            if self.pending.is_some() {
                return Err(failure("Unpaired raw FFT window."));
            }
            self.pending = Some((symbol, event.element_start, points.to_vec()));
            return Ok(());
        }
        let (raw_symbol, start, raw) = self
            .pending
            .take()
            .ok_or_else(|| failure("Missing raw FFT window."))?;
        if raw_symbol != symbol || start != event.element_start || raw.len() != points.len() {
            return Err(failure("Receiver windows do not match."));
        }
        let roles = self.layout.symbol_roles(symbol);
        for (offset, (&before, &after)) in raw.iter().zip(points).enumerate() {
            let bin = start + offset;
            let stream = match roles[bin] {
                PhysicalCarrierRole::ContinuousPilot => "continuous_pilots",
                PhysicalCarrierRole::ScatteredPilot => "scattered_pilots",
                PhysicalCarrierRole::Plc
                    if (symbol % PLC_FRAME_SYMBOLS) < PLC_DATA_SYMBOLS as u64 =>
                {
                    "plc_data"
                }
                PhysicalCarrierRole::Plc => "plc_preamble",
                _ => continue,
            };
            validate_symbol(stream, after)?;
            let response = self.response[bin];
            let expected = [
                response[0] * after[0] - response[1] * after[1],
                response[0] * after[1] + response[1] * after[0],
            ];
            if before.iter().zip(expected).any(|(actual, expected)| {
                !actual.is_finite() || (actual - expected).abs() > NUMERICAL_TOLERANCE
            }) {
                return Err(failure(
                    "Raw point does not satisfy Y = H times corrected point.",
                ));
            }
            let physical_bin =
                SubcarrierIdx::try_from(bin).map_err(|_| failure("Bin overflow."))?;
            self.streams.entry(stream).or_default().push(Pair {
                physical_bin,
                frequency_hz: self
                    .layout
                    .frequency_hz(physical_bin)
                    .ok_or_else(|| failure("Missing frequency."))?,
                channel_symbol_index: symbol,
                before_equalization: before,
                after_equalization: after,
            });
        }
        Ok(())
    }
}
fn failure(message: &str) -> SimulationError {
    SimulationError::new("InvalidProtectedScatter", message)
}
fn validate_symbol(stream: &str, point: IqPair) -> Result<(), SimulationError> {
    let valid = if stream == "plc_data" {
        point.iter().all(|component| {
            let scaled = component.abs() * 10_f64.sqrt();
            (scaled - 1.0).abs().min((scaled - 3.0).abs()) < NUMERICAL_TOLERANCE
        })
    } else {
        let amplitude = if stream == "plc_preamble" { 1.0 } else { 2.0 };
        (point[0].abs() - amplitude).abs() < NUMERICAL_TOLERANCE
            && point[1].abs() < NUMERICAL_TOLERANCE
    };
    if !valid {
        return Err(failure(
            "Corrected point does not match its independent BPSK/16-QAM reference.",
        ));
    }
    Ok(())
}
/// Runs the existing noiseless supplied-H receiver and exports protected pairs.
/// # Returns
/// A new JSON file with four independently validated, physically paired streams.
/// # Errors
/// Rejects oversized input, missing H, incomplete probes and existing output.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    if !(2..=3).contains(&arguments.len()) {
        return Err("Usage: protected_scatter REQUEST.json NEW_OUTPUT.json [SCENARIO.json]".into());
    }
    let document: ChannelDocument = serde_json::from_slice(&read_bounded(&arguments[0])?)?;
    let scenario: Option<ScenarioDocument> = arguments
        .get(2)
        .map(|path| -> Result<_, Box<dyn std::error::Error>> {
            Ok(serde_json::from_slice(&read_bounded(path)?)?)
        })
        .transpose()?;
    let mut settings = document.settings_with_scenario(scenario.as_ref())?;
    // Explicit CE-only experiment: preserve measured H, disable every noise source.
    settings.impairments.noise = NoiseModel::Noiseless;
    if settings.impairments.response.is_none() {
        return Err("This demonstration requires supplied H.".into());
    }
    settings.constellation = None;
    let layout = CarrierLayout::new(&settings.transmission, &settings.layout)?;
    let response = settings
        .impairments
        .plan(&settings.transmission, &layout, settings.llr_variance)?
        .response;
    let plc_width = PlcFrameGeometry::new(settings.transmission.subcarrier_spacing).plc_subcarriers;
    let start = usize::from(settings.layout.plc_first_bin)
        .saturating_sub(CONTEXT_BINS)
        .max(usize::from(settings.layout.first_active_bin));
    let end = (usize::from(settings.layout.plc_first_bin) + plc_width + CONTEXT_BINS)
        .min(usize::from(settings.layout.active_end_exclusive));
    let events = 2 * PLC_FRAME_SYMBOLS as usize;
    let mut probe = TestPointProbe::new(ProbeConfig {
        selections: [TestPointId::RxFft, TestPointId::RxEqualized]
            .into_iter()
            .map(|stage| ProbeSelection {
                stage,
                elements: start..end,
                source_symbols: None,
                channel_symbols: Some(0..PLC_FRAME_SYMBOLS),
            })
            .collect(),
        max_events: events,
        max_elements_per_event: end - start,
        max_total_elements: (end - start) * events,
    })?;
    let mut collector = Collector {
        layout,
        response,
        pending: None,
        streams: BTreeMap::new(),
    };
    let mut run = ChannelRun::new(&settings)?;
    // Stop after one complete acquisition PLC frame; no payload readiness filter.
    while probe.stats().dispatched_events < events {
        if run.advance(&mut probe, &mut collector)?.is_none() {
            return Err("Run ended before the selected PLC frame.".into());
        }
    }
    if collector.pending.is_some() || collector.streams.len() != 4 {
        return Err("Missing protected stream or unmatched receiver pair.".into());
    }
    if run.summary().plc_frames != 1 || run.summary().plc_bit_errors != 0 {
        return Err("Expected one successfully decoded noiseless PLC frame.".into());
    }
    let counts: BTreeMap<_, _> = collector
        .streams
        .iter()
        .map(|(name, pairs)| (*name, pairs.len()))
        .collect();
    let output = serde_json::json!({
        "schema_version": 1,
        "receiver": "supplied_h_known_timing",
        "noise": "noiseless",
        "channel_symbols": {"start": 0, "end_exclusive": PLC_FRAME_SYMBOLS},
        "physical_bins": {"start": start, "end_exclusive": end},
        "settings": settings,
        "counts": counts,
        "probe_stats": probe.stats(),
        "summary": run.summary(),
        "streams": collector.streams,
    });
    let serialized = serde_json::to_vec_pretty(&output)?;
    if serialized.len() as u64 > MAX_INPUT_BYTES {
        return Err("Output exceeds 16 MiB.".into());
    }
    let mut destination = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&arguments[1])?;
    destination.write_all(&serialized)?;
    destination.write_all(b"\n")?;
    Ok(())
}

fn read_bounded(path: &std::ffi::OsStr) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(MAX_INPUT_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_INPUT_BYTES {
        return Err("Input exceeds 16 MiB.".into());
    }
    Ok(bytes)
}
