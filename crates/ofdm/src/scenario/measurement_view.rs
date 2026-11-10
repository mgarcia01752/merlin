/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded frequency plots shared by measurement adapters; all conversions are Rust.
use crate::{error::invalid, scenario::FrequencyGrid, types::IqPair, SimulationError};
use serde::Serialize;

/// Maximum min/max groups per viewport; original arrays remain with their owner.
pub const MAX_MEASUREMENT_POINTS: usize = 1024;
/// Selected projection of a complex channel response, never receiver equalizer gain.
#[derive(Clone, Copy)]
pub enum ResponseMetric {
    GainDb,
    PhaseDegrees,
    Real,
    Imaginary,
}

/// Projects H into a display value while retaining unavailable/zero magnitude gaps.
/// # Arguments
/// * `sample` - Original optional complex response.
/// * `metric` - Explicit gain, wrapped phase, real or imaginary selection.
/// # Returns
/// Finite value or None; no alteration or implicit unity substitution.
pub fn response_value(sample: Option<IqPair>, metric: ResponseMetric) -> Option<f64> {
    let [real, imaginary] = sample?;
    let value = match metric {
        ResponseMetric::GainDb => 20.0 * real.hypot(imaginary).log10(),
        ResponseMetric::PhaseDegrees => imaginary.atan2(real).to_degrees(),
        ResponseMetric::Real => real,
        ResponseMetric::Imaginary => imaginary,
    };
    value.is_finite().then_some(value)
}

/// One min/max bucket with explicit gaps and original-frequency readout.
#[derive(Serialize)]
pub struct MeasurementPoint {
    pub x: f64,
    pub y: Option<f64>,
    pub minimum_y: Option<f64>,
    pub gap: bool,
    pub readout: String,
}
/// Rust-formatted axis annotation; normalized position is presentation-ready.
#[derive(Serialize)]
pub struct AxisTick {
    pub x: f64,
    pub label: String,
}
/// Bounded view; counts and units refer to the original source grid.
#[derive(Serialize)]
pub struct MeasurementView {
    pub points: Vec<MeasurementPoint>,
    pub x_ticks: Vec<AxisTick>,
    pub y_ticks: Vec<AxisTick>,
    pub start: usize,
    pub count: usize,
    pub total: usize,
    pub axis_label: String,
    pub frequency_label: &'static str,
    pub summary: String,
}

/// Builds min/max groups without resampling, dropping nulls or changing source data.
/// # Arguments
/// * `grid` - Exact source frequency coordinates.
/// * `values` - Same-length finite/absent projection.
/// * `start` - First original sample index.
/// * `count` - Number of original samples; zero requests full span from zero.
/// * `units` - Display axis unit supplied by the scientific caller.
/// # Returns
/// At most 1024 points, explicit gaps, labels and bounded readouts.
/// # Errors
/// Rejects inconsistent/nonfinite arrays and out-of-range view requests.
pub fn measurement_view(
    grid: &FrequencyGrid,
    values: &[Option<f64>],
    start: usize,
    count: usize,
    units: &str,
) -> Result<MeasurementView, SimulationError> {
    grid.validate()?;
    if values.len() != grid.sample_count
        || values.iter().flatten().any(|value| !value.is_finite())
        || units.len() > 128
    {
        return Err(invalid("Invalid measurement chart values/units."));
    }
    let (start, count) = if count == 0 {
        (0, values.len())
    } else {
        (start, count)
    };
    let end = start
        .checked_add(count)
        .filter(|end| *end <= values.len())
        .ok_or_else(|| invalid("Measurement viewport exceeds the source grid."))?;
    if count == 0 {
        return Err(invalid("Empty measurement viewport."));
    }
    let present = values[start..end].iter().flatten();
    let minimum = present.clone().copied().fold(f64::INFINITY, f64::min);
    let maximum = present.copied().fold(f64::NEG_INFINITY, f64::max);
    let (bottom, top) = if minimum.is_finite() {
        let padding = ((maximum - minimum) * 0.1).max(1.0);
        (minimum - padding, maximum + padding)
    } else {
        (-1.0, 1.0)
    };
    if !bottom.is_finite() || !top.is_finite() || !(top - bottom).is_finite() || top <= bottom {
        return Err(invalid(
            "Measurement range cannot be represented for plotting.",
        ));
    }
    let normalize = |value: f64| (top - value) / (top - bottom);
    let width = count.div_ceil(MAX_MEASUREMENT_POINTS);
    let points = values[start..end]
        .chunks(width)
        .enumerate()
        .map(|(index, samples)| {
            let first = start + index * width;
            let last = first + samples.len() - 1;
            let low = samples.iter().flatten().copied().reduce(f64::min);
            let high = samples.iter().flatten().copied().reduce(f64::max);
            let reading = match (low, high) {
                (Some(low), Some(high)) => format!("{low:.3} … {high:.3} {units}"),
                _ => "Unavailable".into(),
            };
            MeasurementPoint {
                x: if count == 1 {
                    0.5
                } else {
                    ((first + last) as f64 / 2.0 - start as f64) / (count - 1) as f64
                },
                y: high.map(normalize),
                minimum_y: low.map(normalize),
                gap: samples.iter().any(Option::is_none),
                readout: format!(
                    "{:.3}–{:.3} MHz · {reading}{}",
                    grid.frequency(first).unwrap_or(0) as f64 / 1e6,
                    grid.frequency(last).unwrap_or(0) as f64 / 1e6,
                    if samples.iter().any(Option::is_none) {
                        " · includes missing samples"
                    } else {
                        ""
                    }
                ),
            }
        })
        .collect();
    let intervals = (count - 1).clamp(1, 4);
    let x_ticks = (0..=intervals)
        .map(|tick| {
            let offset = (count - 1) * tick / intervals;
            Ok(AxisTick {
                x: tick as f64 / intervals as f64,
                label: format!("{:.3}", f64::from(grid.frequency(start + offset)?) / 1e6),
            })
        })
        .collect::<Result<Vec<_>, SimulationError>>()?;
    let y_ticks = (0..=4)
        .map(|tick| AxisTick {
            x: f64::from(tick) / 4.0,
            label: format!("{:.1}", top - f64::from(tick) / 4.0 * (top - bottom)),
        })
        .collect();
    Ok(MeasurementView {
        points,
        x_ticks,
        y_ticks,
        start,
        count,
        total: values.len(),
        axis_label: units.into(),
        frequency_label: "Frequency (MHz)",
        summary: format!(
            "{} source carriers · {} Hz spacing · {} unavailable",
            values.len(),
            grid.spacing_hz,
            values.iter().filter(|value| value.is_none()).count()
        ),
    })
}
