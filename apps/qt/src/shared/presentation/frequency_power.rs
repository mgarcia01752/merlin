/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Raw FFT power drawing over public values, distinct from PSD or electrical power.
//! Presentation adapted from MERLIN scenario/measurement_view.rs (Apache-2.0);
//! numerical projection, original coordinates and missing states remain backend-owned.
use super::constants::*;
use merlin::downstream::FullLoopPhysicalBins;
use serde_json::{json, Value};

/// Formats the existing raw FFT viewport without calculating any scientific quantity.
/// # Arguments
/// * `observation` - Complete actual raw FFT observation with authoritative bin power/grid.
/// * `start`, `count` - Original source offsets; zero count selects full span from zero.
/// # Returns
/// Bounded presentation groups, original frequency labels and explicit absent values.
/// # Errors
/// Rejects empty/out-of-range viewports or an unrepresentable drawing extent.
pub(crate) fn viewport(
    observation: &FullLoopPhysicalBins,
    start: usize,
    count: usize,
) -> Result<Value, String> {
    let bins = &observation.bins;
    let (start, count) = if count == 0 {
        (0, bins.len())
    } else {
        (start, count)
    };
    let end = start
        .checked_add(count)
        .filter(|end| *end <= bins.len())
        .ok_or("Measurement viewport exceeds the source grid.")?;
    if count == 0 {
        return Err("Empty measurement viewport.".into());
    }
    let minimum = bins[start..end]
        .iter()
        .filter_map(|bin| bin.power_db)
        .fold(f64::INFINITY, f64::min);
    let maximum = bins[start..end]
        .iter()
        .filter_map(|bin| bin.power_db)
        .fold(f64::NEG_INFINITY, f64::max);
    let (bottom, top) = if minimum.is_finite() {
        let pad =
            ((maximum - minimum) * FFT_POWER_VIEW_PADDING_RATIO).max(FFT_POWER_VIEW_MIN_PADDING_DB);
        (minimum - pad, maximum + pad)
    } else {
        (
            -FFT_POWER_VIEW_EMPTY_EXTENT_DB,
            FFT_POWER_VIEW_EMPTY_EXTENT_DB,
        )
    };
    if !bottom.is_finite() || !top.is_finite() || !(top - bottom).is_finite() || top <= bottom {
        return Err("Measurement range cannot be represented for plotting.".into());
    }
    let frequency = |index: usize| {
        f64::from(observation.layout.carriers[index].frequency_hz) / HERTZ_PER_MEGAHERTZ
    };
    let units = "FFT Bin Power · dB re normalized unit²";
    let width = count.div_ceil(FFT_POWER_VIEW_GROUPS);
    let points = bins[start..end].chunks(width).enumerate().map(|(index, group)| {
        let first = start + index * width;
        let last = first + group.len() - 1;
        let low = group.iter().filter_map(|bin| bin.power_db).reduce(f64::min);
        let high = group.iter().filter_map(|bin| bin.power_db).reduce(f64::max);
        let gap = group.iter().any(|bin| bin.power_db.is_none());
        let reading = match (low, high) {
            (Some(low), Some(high)) => format!("{low:.3} … {high:.3} {units}"),
            _ => "Unavailable".into(),
        };
        let x = if count == 1 {
            // A single original carrier occupies the drawing midpoint.
            0.5
        } else {
            ((first + last) as f64 / 2.0 - start as f64) / (count - 1) as f64
        };
        let missing = if gap {
            " · includes missing samples"
        } else {
            ""
        };
        json!({"x":x,"y":high.map(|value|(top-value)/(top-bottom)),"minimum_y":low.map(|value|(top-value)/(top-bottom)),"gap":gap,
            "readout":format!("{:.3}–{:.3} MHz · {reading}{missing}",frequency(first),frequency(last))})
    }).collect::<Vec<_>>();
    let intervals = (count - 1).clamp(1, FFT_POWER_VIEW_FREQUENCY_INTERVALS);
    let x_ticks = (0..=intervals)
        .map(|tick| {
            json!({"x":tick as f64 / intervals as f64,
        "label":format!("{:.3}",frequency(start + (count - 1) * tick / intervals))})
        })
        .collect::<Vec<_>>();
    let y_ticks = (0..=FFT_POWER_VIEW_POWER_INTERVALS)
        .map(|tick| {
            let y = f64::from(tick) / f64::from(FFT_POWER_VIEW_POWER_INTERVALS);
            json!({"y":y,"label":format!("{:.1}",top-y*(top-bottom))})
        })
        .collect::<Vec<_>>();
    Ok(
        json!({"points":points,"x_ticks":x_ticks,"y_ticks":y_ticks,"start":start,"count":count,"total":bins.len(),
        "axis_label":units,"frequency_label":"Frequency (MHz)","summary":format!("{} source carriers · {} Hz spacing · {} unavailable",
            bins.len(),observation.context.physical_grid.as_ref().map_or(0,|grid|grid.spacing.frequency_hz()),bins.iter().filter(|bin|bin.power_db.is_none()).count())}),
    )
}
