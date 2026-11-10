/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Small presentation pages over retained TX samples, bits and cells.
use crate::closed_loop::model::latest;
use docsis_ofdm_fec::{
    signal_processing::waveform::spectrum::estimate, simulation::test_points::TestPointId,
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
const NAMES: [&str; 9] = [
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

/// Returns a fixed-size page and scatter view of one genuine TX boundary.
/// # Arguments
/// * `index` - Source/coding/bits/labels/mapped/interleaved/bins/PLC selector.
/// * `offset` - First retained element, nonnegative and inside available data.
/// # Returns
/// At most64 rows and2048 plotted cells, preserving nulls and original coordinates.
pub(crate) fn stage(index: i32, offset: i32) -> Value {
    let get = || -> Result<Value, String> {
        let index = usize::try_from(index).map_err(|_| "Invalid TX stage.")?;
        let id = *STAGE_IDS.get(index).ok_or("Invalid TX stage.")?;
        let result = latest().ok_or("Use Start on the main workbench to capture TX stages.")?;
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
        let stride = if bits { 32 } else { 1 };
        let end = (offset + 64 * stride).min(values.len());
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
        let iq: Vec<(usize, [f64; 2])> = values
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                let raw = if v.is_array() { v } else { &v["iq"] };
                Some((i, [raw.get(0)?.as_f64()?, raw.get(1)?.as_f64()?]))
            })
            .collect();
        let extent = iq
            .iter()
            .flat_map(|(_, v)| v.iter())
            .fold(1.0f64, |a, v| a.max(v.abs()))
            * 1.1;
        let points: Vec<_> = iq.iter().step_by(iq.len().div_ceil(2048).max(1)).take(2048).map(|(i,v)|json!({"index":snapshot.offset+i,"i":v[0],"q":v[1],"x":0.5+v[0]/(2.0*extent),"y":0.5-v[1]/(2.0*extent)})).collect();
        Ok(
            json!({"available":true,"name":NAMES[index],"tap":id,"rows":rows,"points":points,"offset":offset,"end":end,"retained":values.len(),"total":snapshot.total,"symbol":snapshot.symbol,"phase":snapshot.phase,"extent":extent,"iq_count":iq.len(),"missing":values.iter().filter(|v|v.is_null()).count(),"reason":""}),
        )
    };
    get().unwrap_or_else(|reason|json!({"available":false,"rows":[],"points":[],"reason":reason,"offset":0,"end":0,"retained":0}))
}

/// Describes one retained TX waveform tap without calculating a spectrum.
/// # Arguments
/// * `tap` - IFFT, prefix, window or overlap-add index0..3.
/// # Returns
/// SA readiness with local run dispatch disabled and an explicit reference plane.
pub(crate) fn spectrum_state(tap: i32) -> Value {
    let names = ["TX IFFT", "TX Cyclic Prefix", "TX Window", "TX Output"];
    let ids = [
        TestPointId::TxIfft,
        TestPointId::TxPrefix,
        TestPointId::TxWindow,
        TestPointId::TxOverlap,
    ];
    let index = usize::try_from(tap).ok().filter(|i| *i < 4);
    let result = latest();
    let available = index
        .zip(result.as_ref())
        .is_some_and(|(i, r)| !r.observations.tx.waves[i].is_empty());
    json!({"has_result":available,"can_generate":false,"busy":false,"result_token":format!("{:?}-{tap}",result.as_ref().map(|r|r.job.elapsed_ms)),"tap_name":index.map(|i|names[i]).unwrap_or("Unavailable"),"tap_id":index.map(|i|ids[i]),"tap_description":"TX complex baseband I/Q · before channel","reason":if available {""} else {"Use Start on the main workbench to capture this TX tap."}})
}

/// Calculates a viewport from actual TX time samples using the shared estimator.
/// # Arguments
/// * `tap` - Waveform reference plane0..3.
/// * `start` - Nonnegative first FFT display bin.
/// * `count` - Positive bin count, or zero for full span.
/// * `capture_axis` - Map frequency labels to the capture without RF conversion.
/// # Returns
/// Bounded viewport or explicit missing/invalid-data error; no bits receive an FFT.
pub(crate) fn spectrum(tap: i32, start: i32, count: i32, capture_axis: bool) -> Value {
    let get = || -> Result<Value, String> {
        let result = latest().ok_or("Run the simulation to capture TX samples.")?;
        let samples = &result
            .observations
            .tx
            .waves
            .get(usize::try_from(tap).map_err(|_| "Invalid TX tap.")?)
            .ok_or("Invalid TX tap.")?;
        let spectrum = estimate(
            result.job.settings.transmission.subcarrier_spacing,
            result.job.settings.layout.zero_frequency_hz,
            samples,
        )
        .map_err(|e| e.to_string())?;
        crate::spectrum_view::viewport(
            &spectrum,
            start,
            count,
            capture_axis,
            "TX complex baseband · before channel",
        )
    };
    get().unwrap_or_else(|error| json!({"error":error,"points":[],"x_ticks":[],"y_ticks":[]}))
}
