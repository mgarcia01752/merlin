/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Response display from typed backend values, without transfer or inverse calculations.
#[cfg(test)]
#[path = "response_tests.rs"]
mod tests;

use super::common;
use crate::shared::presentation::constants::{
    HERTZ_PER_MEGAHERTZ, MICROSECONDS_PER_SECOND, NORMALIZED_AXIS_CENTER,
    SPECTRUM_FREQUENCY_TICK_INTERVALS, SPECTRUM_POWER_TICK_INTERVALS,
};
use merlin::upstream::{
    ConfiguredResponseSnapshot, ConfiguredResponseSource, ReceiverEstimateIdentity,
    ReceiverEstimateSnapshot, ReceiverObservationSummary, MAX_CONFIGURED_IMPULSE_SAMPLES,
    MAX_CONFIGURED_RESPONSE_POINTS, MAX_RECEIVER_ESTIMATE_FRAMES, MAX_RECEIVER_ESTIMATE_POINTS,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// Display padding in the selected axis units for an exactly constant finite line.
const FLAT_RESPONSE_AXIS_PADDING: f64 = 1.0;

/// Explicit configured transfers and the separately retained actual receiver plane.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Source {
    PhysicalChannel,
    CmPreEq,
    Composite,
    ReceivedEstimate,
}

/// Existing backend scalar outputs; an actual estimate has no inverse operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Metric {
    Amplitude,
    Phase,
    Impulse,
}

/// Thin original-coordinate request; scientific query/error routing stays with the caller.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(in crate::upstream::workbench) struct ResponseRequest {
    /// Actual configured source, or a real retained receiver-estimate frame.
    pub(super) source: Source,
    /// Backend amplitude dB, principal phase radians or configured periodic impulse dB.
    pub(super) metric: Metric,
    /// Original base-grant symbol for configured C/H/composite queries.
    pub(super) grant_symbol: usize,
    /// Original index discovered from receiver_observations, never a schedule ordinal.
    pub(super) retained_index: usize,
    /// First physical carrier, or first original periodic sample for impulse display.
    pub(super) first: usize,
    /// Positive display span, or zero for the remaining original extent.
    pub(super) count: usize,
}

struct Sample {
    index: usize,
    coordinate: f64,
    reading: Option<f64>,
    detail: Value,
    readout: String,
}

/// Projects an already-returned configured transfer without recomputing its science.
///
/// # Arguments
/// * `snapshot` - Actual typed configured C, H or composite and optional complete
///   periodic inverse supplied by the existing public query.
/// * `request` - Matching source/symbol; frequency metrics match the public carrier
///   viewport. Impulse first/count instead select original samples from the full
///   supplied period, independently of the public query's frequency viewport.
/// * `summary` - Optional actual report inventory; None is an idle configuration.
///
/// # Returns
/// An unadmitted display candidate with original scalar metadata, exact wide
/// identities and original point details. Null dB/phase stays null with line gaps.
/// No FFT, logarithm, phase extraction, causal-echo inference or source advance occurs.
///
/// # Errors
/// Source/symbol/viewport mismatch, exceeded existing query limits, inconsistent
/// complete-period metadata or nonfinite backend/display values. Missing impulse
/// data returns an explicit unavailable display rather than reconstructing it.
pub(super) fn project_configured(
    snapshot: &ConfiguredResponseSnapshot,
    request: &ResponseRequest,
    summary: Option<&ReceiverObservationSummary>,
) -> Result<Value, String> {
    let matches_source = matches!(
        (request.source, snapshot.source),
        (
            Source::PhysicalChannel,
            ConfiguredResponseSource::PhysicalChannel
        ) | (Source::CmPreEq, ConfiguredResponseSource::CmPreEq)
            | (Source::Composite, ConfiguredResponseSource::Composite)
    );
    if !matches_source || request.grant_symbol != snapshot.grant_symbol {
        return Err("Configured response source or original symbol changed.".into());
    }
    if snapshot.points.len() > MAX_CONFIGURED_RESPONSE_POINTS
        || !snapshot.physical_phase_radians.is_finite()
        || !snapshot.physical_frequency_offset_hz.is_finite()
        || !snapshot.configured_sample_noise_variance.is_finite()
    {
        return Err("Invalid configured response metadata or point limit.".into());
    }
    let mut display = envelope(request, summary)?;
    let mut metadata = serde_json::to_value(snapshot).map_err(|error| error.to_string())?;
    let fields = metadata
        .as_object_mut()
        .ok_or("Invalid configured response metadata.")?;
    fields.remove("points");
    if let Some(impulse) = metadata["impulse"].as_object_mut() {
        impulse.remove("points");
    }
    metadata["configuration_generation"] = optional_identity(snapshot.configuration_generation);
    metadata["run_id"] = optional_identity(snapshot.run_id);
    metadata["grant_id"] = snapshot.grant_id.to_string().into();
    metadata["pre_eq_generation"] = optional_identity(snapshot.pre_eq_generation);
    display["metadata"] = metadata;
    if let Some(summary) = summary {
        display["metadata"]["receiver_observations"] = summary_value(summary)?;
    }
    let (samples, label, frequency_label, first, total) = if request.metric == Metric::Impulse {
        let Some(impulse) = snapshot.impulse.as_ref() else {
            return Ok(unavailable(
                display,
                "The public query did not supply a complete periodic impulse.",
            ));
        };
        if impulse.points.len() != impulse.period_samples
            || impulse.points.len() > MAX_CONFIGURED_IMPULSE_SAMPLES
            || !impulse.peak_amplitude.is_finite()
        {
            return Err("Incomplete configured periodic impulse.".into());
        }
        let end = extent(request.first, request.count, impulse.period_samples)?;
        let mut values = Vec::with_capacity(end - request.first);
        for (index, point) in impulse.points.iter().enumerate() {
            if point.sample_index != index {
                return Err("Configured impulse original sample order changed.".into());
            }
            if !point.value.is_finite()
                || !point.time_seconds.is_finite()
                || point
                    .relative_power_db
                    .is_some_and(|value| !value.is_finite())
            {
                return Err("Nonfinite configured periodic impulse value.".into());
            }
            if (request.first..end).contains(&index) {
                values.push(Sample {
                    index,
                    coordinate: point.time_seconds * MICROSECONDS_PER_SECOND,
                    reading: point.relative_power_db,
                    detail: serde_json::to_value(point).map_err(|error| error.to_string())?,
                    readout: format!(
                        "Periodic sample {} · {} s · I {} · Q {} · {}",
                        point.sample_index,
                        point.time_seconds,
                        point.value.re,
                        point.value.im,
                        optional_readout(
                            point.relative_power_db,
                            "dB relative to full-period peak",
                            "Exact zero: logarithmic magnitude unavailable"
                        )
                    ),
                });
            }
        }
        display["metadata"]["impulse"]["note"] = "Complete periodic inverse supplied by backend; delays may wrap. This is not unaliased causal echo support.".into();
        (
            values,
            "Periodic impulse magnitude (dB relative to full-period peak)",
            "Periodic time (µs)",
            request.first,
            impulse.period_samples,
        )
    } else {
        validate_frequency_window(
            request,
            snapshot.first_carrier,
            snapshot.carrier_count,
            snapshot.fft_samples,
            snapshot.points.len(),
            MAX_CONFIGURED_RESPONSE_POINTS,
        )?;
        let mut values = Vec::with_capacity(snapshot.points.len());
        for (position, point) in snapshot.points.iter().enumerate() {
            if !point.value.is_finite()
                || point.amplitude_db.is_some_and(|value| !value.is_finite())
                || point.phase_radians.is_some_and(|value| !value.is_finite())
            {
                return Err("Nonfinite configured complex response.".into());
            }
            if point.physical_carrier != snapshot.first_carrier + position {
                return Err("Configured response original carrier order changed.".into());
            }
            let reading = match request.metric {
                Metric::Amplitude => point.amplitude_db,
                Metric::Phase => point.phase_radians,
                Metric::Impulse => unreachable!(),
            };
            values.push(Sample {
                index: point.physical_carrier,
                coordinate: point.frequency_hz / HERTZ_PER_MEGAHERTZ,
                reading,
                detail: serde_json::to_value(point).map_err(|error| error.to_string())?,
                readout: format!(
                    "Physical carrier {} · {} Hz · excluded {} · I {} · Q {} · {}",
                    point.physical_carrier,
                    point.frequency_hz,
                    point.excluded,
                    point.value.re,
                    point.value.im,
                    metric_readout(
                        reading,
                        request.metric,
                        "Exact zero transfer: logarithm/phase unavailable"
                    )
                ),
            });
        }
        (
            values,
            metric_label(request.metric),
            "Centered baseband frequency (MHz)",
            snapshot.first_carrier,
            snapshot.fft_samples,
        )
    };
    display["viewport"] = viewport(samples, label, frequency_label, first, total)?;
    Ok(display)
}

/// Projects an actual received-pilot estimate and its original retained-frame identity.
///
/// # Arguments
/// * `snapshot` - Existing public optional effective C×H estimates, never configured truth.
/// * `request` - Matching retained index and physical carrier window.
/// * `summary` - Actual receiver_observations inventory used for frame choices.
///
/// # Returns
/// An unadmitted amplitude/phase candidate preserving Some(zero) versus None and
/// every original carrier. Impulse requests return explicit unavailability: no
/// public actual-estimate inverse exists, and configured H never substitutes.
///
/// # Errors
/// Source/frame/inventory mismatch, invalid bounds/order or nonfinite backend values.
pub(super) fn project_received(
    snapshot: &ReceiverEstimateSnapshot,
    request: &ResponseRequest,
    summary: &ReceiverObservationSummary,
) -> Result<Value, String> {
    if request.source != Source::ReceivedEstimate
        || request.retained_index != snapshot.frame.retained_index
        || !summary.frames.iter().any(|frame| frame == &snapshot.frame)
    {
        return Err("Received response original retained frame changed.".into());
    }
    if snapshot.points.len() > MAX_RECEIVER_ESTIMATE_POINTS {
        return Err("Received response exceeds its public point limit.".into());
    }
    let mut display = envelope(request, Some(summary))?;
    let mut metadata = serde_json::to_value(snapshot).map_err(|error| error.to_string())?;
    metadata
        .as_object_mut()
        .ok_or("Invalid receiver response metadata.")?
        .remove("points");
    metadata["frame"] = frame_value(&snapshot.frame)?;
    metadata["receiver_observations"] = summary_value(summary)?;
    display["metadata"] = metadata;
    if request.metric == Metric::Impulse {
        return Ok(unavailable(display,"Actual received-estimate periodic impulse is unavailable; no public inverse is supplied."));
    }
    validate_frequency_window(
        request,
        snapshot.first_carrier,
        snapshot.carrier_count,
        snapshot.fft_samples,
        snapshot.points.len(),
        MAX_RECEIVER_ESTIMATE_POINTS,
    )?;
    let mut samples = Vec::with_capacity(snapshot.points.len());
    for (position, point) in snapshot.points.iter().enumerate() {
        if point.value.is_some_and(|value| !value.is_finite())
            || point.amplitude_db.is_some_and(|value| !value.is_finite())
            || point.phase_radians.is_some_and(|value| !value.is_finite())
        {
            return Err("Nonfinite actual received complex response.".into());
        }
        if point.physical_carrier != snapshot.first_carrier + position {
            return Err("Received response original carrier order changed.".into());
        }
        let reading = match request.metric {
            Metric::Amplitude => point.amplitude_db,
            Metric::Phase => point.phase_radians,
            Metric::Impulse => unreachable!(),
        };
        let complex = point.value.map_or_else(
            || "Missing actual estimate".to_owned(),
            |value| format!("Actual I {} · Q {}", value.re, value.im),
        );
        let absent = if point.value.is_none() {
            "Unavailable: no actual estimate"
        } else {
            "Actual exact-zero estimate: logarithm/phase unavailable"
        };
        samples.push(Sample {
            index: point.physical_carrier,
            coordinate: point.frequency_hz / HERTZ_PER_MEGAHERTZ,
            reading,
            detail: serde_json::to_value(point).map_err(|error| error.to_string())?,
            readout: format!(
                "Physical carrier {} · {} Hz · {} · {}",
                point.physical_carrier,
                point.frequency_hz,
                complex,
                metric_readout(reading, request.metric, absent)
            ),
        });
    }
    display["viewport"] = viewport(
        samples,
        metric_label(request.metric),
        "Centered baseband frequency (MHz)",
        snapshot.first_carrier,
        snapshot.fft_samples,
    )?;
    Ok(display)
}

/// Publishes an unavailable public query with its real retained-frame inventory.
/// # Arguments
/// * `request`, `summary` - Original selector and actual optional report inventory.
/// * `message` - Original public unavailable diagnostic, without invented data.
/// # Returns
/// An unadmitted empty display with unchanged selection and explicit availability.
/// # Errors
/// Propagates finite inventory serialization failure.
pub(super) fn unavailable_request(
    request: &ResponseRequest,
    summary: Option<&ReceiverObservationSummary>,
    message: &str,
) -> Result<Value, String> {
    Ok(unavailable(envelope(request, summary)?, message))
}

fn envelope(
    request: &ResponseRequest,
    summary: Option<&ReceiverObservationSummary>,
) -> Result<Value, String> {
    if summary.is_some_and(|value| value.frames.len() > MAX_RECEIVER_ESTIMATE_FRAMES) {
        return Err("Received frame inventory exceeds its public limit.".into());
    }
    let frames=summary.map(|summary|summary.frames.iter().map(|frame|Ok(json!({
        "value":frame.retained_index,"label":format!("Retained {} · frame {} · grant {}",frame.retained_index,frame.frame,frame.identity.grant),
        "original":frame_value(frame)?,
    }))).collect::<Result<Vec<_>,String>>()).transpose()?.unwrap_or_default();
    let choices = json!({"sources":[
        {"value":ConfiguredResponseSource::PhysicalChannel,"label":"Configured physical H"},
        {"value":ConfiguredResponseSource::CmPreEq,"label":"Configured transmit correction C"},
        {"value":ConfiguredResponseSource::Composite,"label":"Configured deterministic C×H"},
        {"value":"received_estimate","label":"Actual received-pilot effective estimate","available":!frames.is_empty()}],
        "metrics":[{"value":"amplitude","label":"Amplitude (dB)"},{"value":"phase","label":"Principal phase (rad)"},
            {"value":"impulse","label":"Periodic impulse (relative dB)","available":request.source!=Source::ReceivedEstimate}],
        "frames":frames});
    let selection = serde_json::to_value(request).map_err(|error| error.to_string())?;
    let mut display = common::envelope("response", choices, selection);
    display["limits"] = json!({"maximum_points":MAX_CONFIGURED_RESPONSE_POINTS,
        "maximum_impulse_samples":MAX_CONFIGURED_IMPULSE_SAMPLES,"maximum_received_points":MAX_RECEIVER_ESTIMATE_POINTS});
    Ok(display)
}

fn optional_identity(value: Option<u64>) -> Value {
    value.map_or(Value::Null, |value| value.to_string().into())
}

fn frame_value(frame: &ReceiverEstimateIdentity) -> Result<Value, String> {
    let mut value = serde_json::to_value(frame).map_err(|error| error.to_string())?;
    value["identity"] = json!({"run":frame.identity.run.to_string(),"generation":frame.identity.generation.to_string(),"grant":frame.identity.grant.to_string()});
    value["frame"] = frame.frame.to_string().into();
    Ok(value)
}

fn summary_value(summary: &ReceiverObservationSummary) -> Result<Value, String> {
    let mut value = serde_json::to_value(summary).map_err(|error| error.to_string())?;
    value["frames"] = Value::Array(
        summary
            .frames
            .iter()
            .map(frame_value)
            .collect::<Result<Vec<_>, _>>()?,
    );
    Ok(value)
}

fn unavailable(mut display: Value, message: &str) -> Value {
    display["status"] = "unavailable".into();
    display["message"] = message.into();
    display
}

fn extent(first: usize, count: usize, total: usize) -> Result<usize, String> {
    let remaining = total
        .checked_sub(first)
        .filter(|remaining| *remaining > 0)
        .ok_or("Invalid response viewport start.")?;
    let count = if count == 0 { remaining } else { count };
    if count > remaining {
        return Err("Invalid response viewport extent.".into());
    }
    first
        .checked_add(count)
        .ok_or_else(|| "Response viewport overflowed.".into())
}

fn validate_frequency_window(
    request: &ResponseRequest,
    first: usize,
    count: usize,
    total: usize,
    points: usize,
    limit: usize,
) -> Result<(), String> {
    let end = extent(request.first, request.count, total)?;
    if first != request.first || count != end - request.first || points != count || points > limit {
        return Err("Response query does not match the requested original carrier window.".into());
    }
    Ok(())
}

fn metric_label(metric: Metric) -> &'static str {
    match metric {
        Metric::Amplitude => "Complex-transfer amplitude ratio (dB)",
        Metric::Phase => "Principal complex-transfer phase (rad)",
        Metric::Impulse => "Periodic impulse magnitude (relative dB)",
    }
}

fn optional_readout(value: Option<f64>, units: &str, absent: &str) -> String {
    value.map_or_else(|| absent.to_owned(), |value| format!("{value} {units}"))
}

fn metric_readout(value: Option<f64>, metric: Metric, absent: &str) -> String {
    optional_readout(
        value,
        if metric == Metric::Amplitude {
            "dB"
        } else {
            "rad"
        },
        absent,
    )
}

fn viewport(
    samples: Vec<Sample>,
    axis: &str,
    frequency: &str,
    start: usize,
    total: usize,
) -> Result<Value, String> {
    if samples.is_empty() {
        return Err("Response viewport has no original coordinates.".into());
    }
    let mut low = f64::INFINITY;
    let mut high = f64::NEG_INFINITY;
    for sample in &samples {
        if !sample.coordinate.is_finite() || sample.reading.is_some_and(|value| !value.is_finite())
        {
            return Err("Nonfinite response display value.".into());
        }
        if let Some(value) = sample.reading {
            low = low.min(value);
            high = high.max(value);
        }
    }
    let finite = low.is_finite() && high.is_finite();
    if finite && low == high {
        low -= FLAT_RESPONSE_AXIS_PADDING;
        high += FLAT_RESPONSE_AXIS_PADDING;
    }
    let span = high - low;
    if finite && (!span.is_finite() || span <= 0.0) {
        return Err("Response display axis overflowed.".into());
    }
    let count = samples.len();
    let x = |position: usize| {
        if count == 1 {
            NORMALIZED_AXIS_CENTER
        } else {
            position as f64 / (count - 1) as f64
        }
    };
    let intervals = (count - 1).min(SPECTRUM_FREQUENCY_TICK_INTERVALS);
    let x_ticks: Vec<_> = (0..=intervals)
        .map(|tick| {
            let position = if intervals == 0 {
                0
            } else {
                (count - 1) * tick / intervals
            };
            json!({"x":x(position),"label":format!("{:.6}",samples[position].coordinate)})
        })
        .collect();
    let y_ticks = if finite {
        (0..=SPECTRUM_POWER_TICK_INTERVALS)
            .map(|tick| {
                let y = f64::from(tick) / f64::from(SPECTRUM_POWER_TICK_INTERVALS);
                json!({"y":y,"label":format!("{:.3}",high-y*span)})
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let mut previous_gap = true;
    let points:Vec<_>=samples.into_iter().enumerate().map(|(position,sample)| {
        let y=sample.reading.map(|value|(high-value)/span);
        let gap=previous_gap||sample.reading.is_none();previous_gap=sample.reading.is_none();
        json!({"x":x(position),"y":y,"minimum_y":y,"gap":gap,"index":sample.index,
            "coordinate":sample.coordinate,"value":sample.reading,"detail":sample.detail,"readout":sample.readout})
    }).collect();
    Ok(
        json!({"points":points,"x_ticks":x_ticks,"y_ticks":y_ticks,"bands":[],
        "axis_label":axis,"frequency_label":frequency,"start":start,"count":count,"total":total,
        "error":if finite {""}else{"No finite readings: exact zero and missing values remain distinct in the point details."}}),
    )
}
