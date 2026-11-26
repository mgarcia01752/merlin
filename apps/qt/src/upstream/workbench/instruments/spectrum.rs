/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Linear PSD display over the public FFT result; no logarithmic science in Qt.
use super::{common, SpectrumRequest};
use crate::shared::presentation::constants::{
    HERTZ_PER_MEGAHERTZ, NORMALIZED_AXIS_CENTER, SPECTRUM_FREQUENCY_TICK_INTERVALS,
    SPECTRUM_POWER_TICK_INTERVALS,
};
use merlin::upstream::{BatchReport, SpectrumSnapshot};
use serde_json::{json, Value};

// Linear-axis extent when the entire supplied PSD is exactly zero; values stay zero.
const ZERO_POWER_DISPLAY_EXTENT: f64 = 1.0;

/// Queries the actual public PSD using the report's configured numerology.
/// # Arguments
/// * `report`, `request` - Immutable retained science and original bin viewport.
/// # Returns
/// An unadmitted linear-PSD envelope preserving original sample/FFT metadata.
/// # Errors
/// Public request failure or inconsistent/nonfinite source/viewport metadata.
pub(super) fn project(report: &BatchReport, request: SpectrumRequest) -> Result<Value, String> {
    let mode = report
        .configuration
        .us_configuration
        .waveform()
        .numerology();
    let mut selected = None;
    let mut choices = Vec::new();
    for tap in report
        .options
        .retained_taps
        .iter()
        .copied()
        .filter(|tap| common::time_tap(*tap))
    {
        let result = report.spectrum(tap, mode);
        // A declared retained tap may still lack a complete contiguous window in
        // a partial report. Query actual availability instead of promising a PSD.
        if let Err(error) = &result {
            if !matches!(error, merlin::upstream::PhyError::Unsupported { .. }) {
                return Err(error.to_string());
            }
        }
        choices.push(json!({"value":tap,"label":format!("{tap:?}"),"available":result.is_ok()}));
        if tap == request.tap {
            selected = Some(result);
        }
    }
    let mut display = common::envelope(
        "spectrum",
        json!({"taps":choices}),
        json!({"tap":request.tap,"first":request.first,"count":request.count}),
    );
    let source = match selected.unwrap_or_else(|| report.spectrum(request.tap, mode)) {
        Ok(source) => source,
        Err(error) => return common::unavailable(display, error),
    };
    display["metadata"] = common::report_metadata(report);
    display["metadata"]["tap"] = json!(source.tap);
    display["metadata"]["identity"] = json!({"run":source.identity.run.to_string(),
        "generation":source.identity.generation.to_string(),"grant":source.identity.grant.to_string()});
    display["metadata"]["first_sample"] = source.first_sample.to_string().into();
    display["metadata"]["sample_rate_hz"] = source.sample_rate_hz.into();
    display["metadata"]["fft_len"] = source.fft_len.into();
    display["metadata"]["analysis_window"] = source.analysis_window.into();
    display["metadata"]["units"] = source.units.into();
    display["metadata"]["reference_plane"] =
        "Actual retained contiguous time samples at the selected tap; centered complex baseband."
            .into();
    display["viewport"] = viewport(&source, request.first, request.count)?;
    Ok(display)
}

fn viewport(source: &SpectrumSnapshot, first: usize, count: usize) -> Result<Value, String> {
    let total = source.power_per_hz.len();
    if total == 0 || source.frequency_hz.len() != total || total != source.fft_len {
        return Err("Inconsistent upstream spectrum coordinates.".into());
    }
    let remaining = total
        .checked_sub(first)
        .filter(|remaining| *remaining > 0)
        .ok_or("Invalid upstream spectrum viewport start.")?;
    let count = if count == 0 { remaining } else { count };
    if count > remaining {
        return Err("Invalid upstream spectrum viewport extent.".into());
    }
    let mut top = 0.0_f64;
    for (&frequency, &power) in source.frequency_hz.iter().zip(&source.power_per_hz) {
        if !frequency.is_finite() || !power.is_finite() || power < 0.0 {
            return Err("Nonfinite or negative upstream spectrum value.".into());
        }
        top = top.max(power);
    }
    if top == 0.0 {
        top = ZERO_POWER_DISPLAY_EXTENT;
    }
    let x = |index: usize| {
        if count == 1 {
            NORMALIZED_AXIS_CENTER
        } else {
            (index - first) as f64 / (count - 1) as f64
        }
    };
    let points: Vec<_> = (first..first + count)
        .map(|index| {
            let power = source.power_per_hz[index];
            let y = 1.0 - power / top;
            json!({"x":x(index),"y":y,"minimum_y":y,"gap":false,"bin":index,
            "frequency_hz":source.frequency_hz[index],"power_per_hz":power,
            "readout":format!("{} Hz · {} {}",source.frequency_hz[index],power,source.units)})
        })
        .collect();
    let intervals = (count - 1).min(SPECTRUM_FREQUENCY_TICK_INTERVALS);
    let x_ticks:Vec<_>=(0..=intervals).map(|tick| {
        let index=if intervals==0 {first}else{first+(count-1)*tick/intervals};
        json!({"x":x(index),"label":format!("{:.6}",source.frequency_hz[index]/HERTZ_PER_MEGAHERTZ)})
    }).collect();
    let y_ticks: Vec<_> = (0..=SPECTRUM_POWER_TICK_INTERVALS)
        .map(|tick| {
            let y = f64::from(tick) / f64::from(SPECTRUM_POWER_TICK_INTERVALS);
            json!({"y":y,"label":format!("{:.4e}",top*(1.0-y))})
        })
        .collect();
    Ok(
        json!({"points":points,"x_ticks":x_ticks,"y_ticks":y_ticks,"bands":[],
        "axis_label":format!("Linear {} · {} window · {} samples",source.units,source.analysis_window,source.fft_len),
        "frequency_label":"Centered baseband frequency (MHz)","start":first,"count":count,"total":total}),
    )
}
