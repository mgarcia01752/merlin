/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded views of real retained receiver observations.
use crate::closed_loop::model::latest;
use docsis_ofdm_fec::{
    scenario::{measurement_view::measurement_view, FrequencyGrid},
    signal_processing::waveform::spectrum::{density_db, estimate},
    types::IqPair,
};
use serde_json::{json, Value};

pub(super) fn qam(signal: i32, view: i32, part: i32) -> Option<Value> {
    if !(0..3).contains(&signal) || !(0..3).contains(&view) || !(0..2).contains(&part) {
        return None;
    }
    let result = latest()?;
    let mut pairs: Vec<(IqPair, IqPair)> = Vec::new();
    let mut detail = String::new();
    if signal == 0 {
        if let Some(data) = &result.job.constellation {
            let modulation = data.pairs.first().map(|p| p.modulation_order);
            pairs.extend(
                data.pairs
                    .iter()
                    .filter(|p| Some(p.modulation_order) == modulation)
                    .take(2048)
                    .map(|p| (p.before_equalization, p.after_equalization)),
            );
            detail=format!("Received Profile {} · {} QAM · {} actual matched cells · first compatible modulation group",data.selection.profile_id,modulation.map_or_else(|| "Unavailable".into(), |order| order.to_string()),pairs.len());
        }
    } else if signal == 2 {
        if let Some(protected) = &result.job.protected_constellation {
            let data = if part == 0 {
                &protected.plc_data
            } else {
                &protected.plc_preamble
            };
            pairs.extend(
                data.pairs
                    .iter()
                    .take(2048)
                    .map(|p| (p.before_equalization, p.after_equalization)),
            );
            detail = format!(
                "PLC {} · {} · {} actual matched cells",
                if part == 0 { "data" } else { "preamble" },
                data.modulation,
                pairs.len()
            );
        }
    } else {
        return None;
    }
    let extent = pairs
        .iter()
        .flat_map(|(a, b)| a.iter().chain(b))
        .map(|v| v.abs())
        .fold(1.0_f64, f64::max)
        * 1.1;
    let points: Vec<_> = pairs
        .iter()
        .map(|(raw, eq)| {
            json!({"raw_x":0.5+raw[0]/(2.0*extent),"raw_y":0.5-raw[1]/(2.0*extent),
        "eq_x":0.5+eq[0]/(2.0*extent),"eq_y":0.5-eq[1]/(2.0*extent)})
        })
        .collect();
    Some(
        json!({"available":!points.is_empty(),"reason":if points.is_empty(){"No eligible cells retained in this run"}else{"Actual receiver observations"},
        "detail":format!("{detail} · correction {:?} · shared axes ±{extent:.3}",result.job.settings.receiver_mode),
        "raw_tap":"rx.fft.bins","equalized_tap":"rx.equalized.iq","points":points,"profiles":if signal==0 {vec!["Profile 0"]}else{vec![]},"view":view}),
    )
}
pub(super) fn spectrum(
    tap: i32,
    start: i32,
    count: i32,
    capture_axis: bool,
) -> Result<Value, String> {
    let result = latest().ok_or("Run the closed loop to retain RX samples.")?;
    let tap = usize::try_from(tap).map_err(|_| "Invalid tap")?;
    let samples = &result
        .observations
        .taps
        .get(tap)
        .ok_or("Invalid tap")?
        .values;
    let settings = &result.job.settings;
    if tap < 3 {
        let spectrum = estimate(
            settings.transmission.subcarrier_spacing,
            settings.layout.zero_frequency_hz,
            samples,
        )
        .map_err(|e| e.to_string())?;
        return crate::spectrum_view::viewport(
            &spectrum,
            start,
            count,
            capture_axis,
            "Retained contiguous RX samples from the completed/partial run",
        );
    }
    let values: Vec<_> = samples
        .iter()
        .map(|p| density_db(p[0] * p[0] + p[1] * p[1]))
        .collect();
    let grid = FrequencyGrid {
        zero_frequency_hz: settings.layout.zero_frequency_hz,
        first_subcarrier_index: 0,
        spacing_hz: settings.transmission.subcarrier_spacing.frequency_hz(),
        sample_count: values.len(),
    };
    let view = measurement_view(
        &grid,
        &values,
        usize::try_from(start).map_err(|_| "Invalid start")?,
        usize::try_from(count).map_err(|_| "Invalid count")?,
        "FFT Bin Power · dB re normalized unit²",
    )
    .map_err(|e| e.to_string())?;
    let mut document = serde_json::to_value(view).map_err(|e| e.to_string())?;
    document["y_ticks"] = json!(document["y_ticks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| json!({"y":t["x"],"label":t["label"]}))
        .collect::<Vec<_>>());
    Ok(document)
}
