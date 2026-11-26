/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Original RxMER readings in screen coordinates; no measurement reconstruction.
use super::constants::{
    CAPTURE_PLOT_CARRIER_TICK_INTERVALS, CAPTURE_PLOT_DB_TICK_INTERVALS,
    CAPTURE_PLOT_FLAT_PADDING_DB, MAX_CAPTURE_PLOT_POINTS,
};
use crate::shared::presentation::constants::NORMALIZED_AXIS_CENTER;
use merlin::upstream::inputs::RxmerCapture;
use serde_json::{json, Value};

/// Projects a finite original capture rectangle for the shared measurement plot.
/// # Arguments
/// * `capture` - Actual public decoded source retaining original dB/null readings.
/// * `first`, `count` - Original record offset and span; zero chooses the remaining
///   bounded plot prefix, independently from the source inspector table page.
/// # Returns
/// Original physical coordinates and unchanged readings with pure screen scales.
/// Missing readings remain null and break curves; no frequency origin or profile,
/// association, calibrated power or simulated measurement is inferred.
/// # Errors
/// Invalid source/viewport bounds, nonfinite values or exhausted point policy.
pub(super) fn viewport(
    capture: &RxmerCapture,
    first: usize,
    count: usize,
) -> Result<Value, String> {
    let readings = capture.readings_db();
    let total = readings.len();
    if total == 0 {
        if first != 0 || count != 0 {
            return Err("An empty RxMER source accepts only the zero-origin empty plot.".into());
        }
        return Ok(json!({"points":[],"x_ticks":[],"y_ticks":[],"bands":[],
            "start":0,"count":0,"total":0,"maximum_count":MAX_CAPTURE_PLOT_POINTS,
            "axis_label":"Original upstream RxMER (dB)",
            "frequency_label":"Original physical subcarrier index",
            "summary":"No original RxMER records; unassociated inspection only."}));
    }
    if first >= total {
        return Err("RxMER plot origin is outside the original source.".into());
    }
    let remaining = total - first;
    let count = if count == 0 {
        remaining.min(MAX_CAPTURE_PLOT_POINTS)
    } else {
        count
    };
    if count > MAX_CAPTURE_PLOT_POINTS || count > remaining {
        return Err("RxMER plot extent exceeds its original source or display policy.".into());
    }
    // Stable full-source fitting needs only two scalars, not another reading Vec.
    let mut fit: Option<(f64, f64)> = None;
    for value in readings.iter().flatten().copied() {
        if !value.is_finite() {
            return Err("Nonfinite original RxMER reading.".into());
        }
        fit = Some(match fit {
            Some((bottom, top)) => (bottom.min(value), top.max(value)),
            None => (value, value),
        });
    }
    let (mut bottom, mut top) = fit.unwrap_or((0.0, CAPTURE_PLOT_FLAT_PADDING_DB));
    if top == bottom {
        top += CAPTURE_PLOT_FLAT_PADDING_DB;
        bottom -= CAPTURE_PLOT_FLAT_PADDING_DB;
    }
    let physical_first = usize::from(capture.header().first_active_subcarrier);
    let x = |index: usize| {
        if count == 1 {
            NORMALIZED_AXIS_CENTER
        } else {
            (index - first) as f64 / (count - 1) as f64
        }
    };
    let points: Vec<_> = (first..first+count).map(|index| {
        let value = readings[index];
        let y = value.map(|value|(top-value)/(top-bottom));
        json!({"x":x(index),"y":y,"minimum_y":y,"gap":value.is_none(),
            "record":index,"physical_carrier":physical_first+index,"rxmer_db":value,
            "readout":format!("Physical carrier {} · {}",physical_first+index,
                value.map(|value|format!("{value} dB")).unwrap_or_else(||"Unavailable original reading".into()))})
    }).collect();
    let intervals = (count - 1).min(CAPTURE_PLOT_CARRIER_TICK_INTERVALS);
    let x_ticks: Vec<_> = (0..=intervals)
        .map(|tick| {
            let index = if intervals == 0 {
                first
            } else {
                first + (count - 1) * tick / intervals
            };
            json!({"x":x(index),"label":(physical_first+index).to_string()})
        })
        .collect();
    let y_ticks: Vec<_> = (0..=CAPTURE_PLOT_DB_TICK_INTERVALS)
        .map(|tick| {
            let y = f64::from(tick) / f64::from(CAPTURE_PLOT_DB_TICK_INTERVALS);
            json!({"x":y,"label":format!("{:.2}",top-y*(top-bottom))})
        })
        .collect();
    Ok(
        json!({"points":points,"x_ticks":x_ticks,"y_ticks":y_ticks,"bands":[],
        "start":first,"count":count,"total":total,"maximum_count":MAX_CAPTURE_PLOT_POINTS,"axis_label":"Original upstream RxMER (dB)",
        "frequency_label":"Original physical subcarrier index",
        "summary":format!("Original records {}–{} of {total}; unassociated inspection only{}",first+1,first+count,
            if fit.is_none() {"; no finite readings"} else {""})}),
    )
}
