/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Small presentation pages over retained TX samples, bits and cells.
use super::capture::WAVE_STAGES;
use crate::downstream::common::constants::{
    MAX_INSTRUMENT_TIME_SAMPLES, MAX_TX_FRAME_SEQUENCE_SAMPLES, TX_BIT_PREVIEW_PAGE_BITS,
    TX_INSPECTOR_BITS_PER_ROW, TX_INSPECTOR_ROWS,
};
use crate::downstream::simulation::closed_loop::latest;
use crate::downstream::workbench::session::Context;
use crate::shared::presentation::constants::MIN_SPECTRUM_DISPLAY_BINS;
use merlin::downstream::{
    ApiResult, ChannelSpectrum, ErrorCode, FullLoopResult, FullLoopSpectrumQuery,
    FullLoopStage as TestPointId,
};
use serde_json::{json, Value};

const STAGE_IDS: [TestPointId; 9] = [
    TestPointId::SourcePayload,
    TestPointId::PayloadCoded,
    TestPointId::PayloadInterleavedBits,
    TestPointId::PayloadLabels,
    TestPointId::PayloadMapped,
    TestPointId::PayloadInterleaved,
    TestPointId::PhysicalBins,
    TestPointId::PlcIq,
    TestPointId::PlcPreamble,
];
const NAMES: [&str; STAGE_IDS.len()] = [
    "Source Bits",
    "LDPC Codeword",
    "Interleaved Bits",
    "Cell Labels",
    "Mapped Cells · Data / NCP",
    "Interleaved Cells · Data / NCP",
    "Physical FFT Bins",
    "PLC Data I/Q",
    "PLC Preamble I/Q",
];
const WAVE_NAMES: [&str; WAVE_STAGES.len()] =
    ["TX IFFT", "TX Cyclic Prefix", "TX Window", "TX Output"];

/// Returns a fixed-size page and scatter view of one genuine TX boundary.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `index` - Source/coding/bits/labels/mapped/interleaved/bins/PLC selector.
/// * `offset` - First retained element, nonnegative and inside available data.
/// # Returns
/// At most64 rows and2048 plotted cells, preserving nulls and original coordinates.
pub(crate) fn stage(context: &Context, index: i32, offset: i32) -> Value {
    let get = || -> Result<Value, String> {
        let index = usize::try_from(index).map_err(|_| "Invalid TX stage.")?;
        let id = *STAGE_IDS.get(index).ok_or("Invalid TX stage.")?;
        let result =
            latest(context).ok_or("Use Start on the main workbench to capture TX stages.")?;
        let snapshot = result
            .observations
            .tx
            .snapshots
            .get(&id)
            .ok_or("No data captured for this stage.")?;
        let values = snapshot.data["values"]
            .as_array()
            .ok_or("No array captured for this stage.")?;
        let offset = usize::try_from(offset).map_err(|_| "Invalid page offset.")?;
        if offset >= values.len() {
            return Err("Page is outside the captured data.".into());
        }
        let bits = matches!(
            id,
            TestPointId::SourcePayload
                | TestPointId::PayloadCoded
                | TestPointId::PayloadInterleavedBits
        );
        let stride = if bits { TX_INSPECTOR_BITS_PER_ROW } else { 1 };
        let page_size = if matches!(
            id,
            TestPointId::SourcePayload | TestPointId::PayloadInterleavedBits
        ) {
            TX_BIT_PREVIEW_PAGE_BITS
        } else {
            TX_INSPECTOR_ROWS * stride
        };
        let end = (offset + page_size).min(values.len());
        let rows: Vec<_> = values[offset..end]
            .chunks(stride)
            .enumerate()
            .map(|(row, chunk)| {
                let first = snapshot.offset + offset + row * stride;
                let text = if bits {
                    chunk.iter().map(|v| v.to_string()).collect::<String>()
                } else {
                    serde_json::to_string(&chunk[0]).unwrap_or_default()
                };
                json!({"index":first,"value":text})
            })
            .collect();
        let roles = super::qam_points::roles(result.backend(), id, snapshot);
        let scatter = super::qam_points::scatter(values, snapshot.offset, roles.as_deref(), id);
        let graph = super::graphs::project(
            id,
            snapshot,
            offset,
            end,
            result
                .observations
                .tx
                .snapshots
                .get(&TestPointId::PayloadMapped),
            result.backend(),
        );
        Ok(
            json!({"graph":graph,"available":true,"name":NAMES[index],"tap":id,"rows":rows,"points":scatter["points"],"offset":offset,"end":end,"retained":values.len(),"total":snapshot.total,"symbol":snapshot.symbol,"phase":snapshot.phase,"extent":scatter["extent"],"iq_count":scatter["iq_count"],"role_counts":scatter["role_counts"],"missing":values.iter().filter(|v|v.is_null()).count(),"reason":""}),
        )
    };
    get().unwrap_or_else(|reason|json!({"available":false,"rows":[],"points":[],"reason":reason,"offset":0,"end":0,"retained":0}))
}

/// Collects six graphical cards from retained boundaries without starting a run.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// # Returns
/// Six finite graph documents; exact row tables remain behind the full inspector.
/// Bit/label alternatives retain their actual boundary index; absence is explicit.
pub(crate) fn stage_previews(context: &Context) -> Value {
    let mut cards = Vec::new();
    for index in [0, 1, 2, 4, 5, 6] {
        let mut selected = index;
        let mut observation = stage(context, index, 0);
        if index == 2 && observation["available"] != true {
            let labels = stage(context, 3, 0);
            if labels["available"] == true {
                observation = labels;
                selected = 3;
            }
        }
        observation
            .as_object_mut()
            .map(|value| value.remove("rows"));
        if let Some(points) = observation["graph"]["points"].as_array_mut() {
            for point in points {
                if let Some(point) = point.as_object_mut() {
                    point.remove("readout");
                }
            }
        }
        cards.push(json!({"stage_index":selected,"observation":observation}));
    }
    json!(cards)
}

/// Describes one retained TX waveform tap without calculating a spectrum.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `tap` - Existing IFFT/prefix/window/overlap selector, in the shared registration order.
/// # Returns
/// Original SA readiness/labels with local run dispatch disabled; absent history stays absent.
pub(crate) fn spectrum_state(context: &Context, tap: i32) -> Value {
    let index = usize::try_from(tap)
        .ok()
        .filter(|index| *index < WAVE_STAGES.len());
    let result = latest(context);
    let available = index.zip(result.as_ref()).is_some_and(|(index, result)| {
        result
            .backend()
            .stage_inventory()
            .iter()
            .any(|info| info.stage == WAVE_STAGES[index] && info.retained_elements > 0)
    });
    let reason = match available {
        true => "",
        false => "Use Start on the main workbench to capture this TX tap.",
    };
    json!({"has_result":available,"can_generate":false,"busy":false,
        "result_token":format!("{:?}-{tap}",result.as_ref().map(|result|result.job.elapsed_ms)),
        "tap_name":index.map(|index|WAVE_NAMES[index]).unwrap_or("Unavailable"),
        "tap_id":index.map(|index|WAVE_STAGES[index]),
        "tap_description":"TX complex baseband I/Q · before channel","reason":reason})
}

fn observed_power(
    result: &FullLoopResult,
    query: FullLoopSpectrumQuery,
) -> ApiResult<ChannelSpectrum> {
    match query.stage {
        TestPointId::TxOverlap => Ok(result.calibrated_spectrum(query)?.power),
        _ => Ok(result.calibrated_spectrum_sequence(query)?.power),
    }
}

/// Formats authoritative calibrated TX spectra within the unchanged retained-sample policies.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `tap` - Shared IFFT/prefix/window/overlap selector.
/// * `start` - Nonnegative first FFT display bin.
/// * `count` - At least two bins, or zero for the existing full-FFT-width convention.
/// * `capture_axis` - Physical capture labels instead of centered baseband coordinates.
/// # Returns
/// Existing bounded SA viewport with unchanged labels, PLC shading and full-band power.
/// Frame taps preserve their ordered-frame prefix; output uses physically continuous samples.
/// All PSD/calibration/window planning comes from public Rust operations; no run is started.
pub(crate) fn spectrum(
    context: &Context,
    tap: i32,
    start: i32,
    count: i32,
    capture_axis: bool,
) -> Value {
    let get = || -> Result<Value, String> {
        let result = latest(context).ok_or("Run the simulation to capture TX samples.")?;
        let stage = *WAVE_STAGES
            .get(usize::try_from(tap).map_err(|_| "Invalid TX tap.")?)
            .ok_or("Invalid TX tap.")?;
        let backend = result.backend();
        let transmission = &backend.inputs().transmission;
        let budget = match stage {
            TestPointId::TxOverlap => MAX_INSTRUMENT_TIME_SAMPLES,
            _ => MAX_TX_FRAME_SEQUENCE_SAMPLES,
        };
        let query = FullLoopSpectrumQuery {
            stage,
            max_averages: transmission
                .spectrum_average_capacity(budget)
                .map_err(|cause| cause.message().to_owned())?,
            ..Default::default()
        };
        // Preserve the old missing-sample presentation and numerical-error ordering.
        // This is an error label, not an independently enforced analysis work limit.
        let full = observed_power(backend, query.clone()).map_err(|cause| match cause.code() {
            ErrorCode::MissingCoverage | ErrorCode::InvalidIdentifier => {
                "Spectrum needs 1..32 complete overlapping FFT windows.".to_owned()
            }
            _ => cause.message().to_owned(),
        })?;
        let start = usize::try_from(start).map_err(|_| "Invalid spectrum start.")?;
        let count = if count == 0 {
            full.fft_samples
        } else {
            usize::try_from(count).map_err(|_| "Invalid spectrum count.")?
        };
        if count < MIN_SPECTRUM_DISPLAY_BINS {
            return Err("Display at least two spectrum bins.".into());
        }
        let selected = if start == 0 && count == full.fft_samples {
            // Full span reuses the original estimate; zoom needs precise original-bin buckets.
            None
        } else {
            Some(
                observed_power(
                    backend,
                    FullLoopSpectrumQuery {
                        start,
                        count,
                        ..query
                    },
                )
                .map_err(|cause| cause.message().to_owned())?,
            )
        };
        // Only the fixed configured PLC interval is drawn; no scattered-role phase is inferred.
        let layout = transmission
            .physical_layout(&backend.inputs().profile, 0, 0, 0)
            .map_err(|cause| cause.message().to_owned())?;
        Ok(crate::shared::presentation::spectrum::viewport(
            selected.as_ref().unwrap_or(&full),
            &full,
            &layout.layout,
            start,
            count,
            capture_axis,
            "TX complex baseband · before channel",
        ))
    };
    get().unwrap_or_else(|error| json!({"error":error,"points":[],"x_ticks":[],"y_ticks":[]}))
}
