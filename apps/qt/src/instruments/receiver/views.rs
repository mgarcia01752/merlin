/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded views of real retained receiver observations.
use crate::simulation::closed_loop::latest;
use docsis_ofdm_fec::{
    scenario::{measurement_view::measurement_view, FrequencyGrid},
    signal_processing::waveform::spectrum::{density_db, estimate},
};
use serde_json::{json, Value};

pub(crate) fn spectrum(
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
        let layout = docsis_ofdm_fec::transmission::layout::CarrierLayout::new(
            &settings.transmission,
            &settings.layout,
        )
        .map_err(|e| e.to_string())?;
        return crate::instruments::channel::channel_viewport(
            spectrum,
            start,
            count,
            capture_axis,
            "Retained contiguous RX samples from the completed/partial run",
            &layout,
            &settings.impairments.power,
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
