/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded profile performance retention, separate from the latest detailed instrument snapshot.
use crate::downstream::{
    common::constants::{
        MIN_PROFILE_FREQUENCY_AXIS_SPAN_HZ, PERFORMANCE_AXIS_QUANTUM_DB, PERFORMANCE_CURVE_BUCKETS,
        PERFORMANCE_IDEAL_SERIES_ID, PERFORMANCE_INITIAL_MAXIMUM_DB,
        PERFORMANCE_INITIAL_MINIMUM_DB, PERFORMANCE_METRIC_TICK_INTERVALS,
        PERFORMANCE_OBSERVED_SERIES_ID, PROFILE_FREQUENCY_TICK_INTERVALS,
    },
    simulation::closed_loop::ResultData,
};
use crate::shared::presentation::constants::HERTZ_PER_MEGAHERTZ;
use merlin::downstream::{FecReport, ProfileCarrierRole, SourceProfileFrame};
use serde_json::{json, Value};
use std::cell::RefCell;

pub(crate) use crate::downstream::common::constants::MAX_RETAINED_RECEIVER_PROFILES as MAX_PROFILES;
struct Point {
    hz: u32,
    mer: Option<f64>,
    ideal: Option<f64>,
    samples: u64,
    plc: bool,
}
struct Entry {
    id: u8,
    channel: u8,
    status: String,
    fec: Option<FecReport>,
    ber: Option<f64>,
    pending: u64,
    points: Vec<Point>,
    error: Option<String>,
    fec_details: Option<Value>,
}
thread_local! { static ENTRIES: RefCell<Vec<Entry>> = const {RefCell::new(Vec::new())}; }
/// Clears all transient profile results after new accepted input or Start.
pub(crate) fn clear() {
    super::qam_history::clear();
    ENTRIES.with(|entries| entries.borrow_mut().clear());
}
/// Registers the finite profile catalog so unstarted jobs remain visible after Stop.
/// # Arguments
/// * `profiles` - Prevalidated captured identities, bounded by MAX_PROFILES.
pub(crate) fn begin(profiles: &[SourceProfileFrame]) {
    clear();
    super::qam_history::begin(profiles);
    for profile in profiles.iter().take(MAX_PROFILES) {
        insert(Entry {
            fec_details: None,
            id: profile.profile_id,
            channel: profile.key.channel_id,
            status: "not_started".into(),
            fec: None,
            ber: None,
            pending: 0,
            points: vec![],
            error: None,
        });
    }
}
fn insert(entry: Entry) {
    ENTRIES.with(|entries| {
        let mut entries = entries.borrow_mut();
        if let Some(index) = entries.iter().position(|old| old.id == entry.id) {
            entries[index] = entry;
        } else if entries.len() < MAX_PROFILES {
            entries.push(entry);
        }
    });
}
/// Retains one bounded scalar/curve result while leaving the large latest snapshot elsewhere.
/// # Arguments
/// * `profile` - Exact applied profile for this independent run.
/// * `result` - Actual completed or partial job, including observation power sums.
pub(crate) fn retain(profile: &SourceProfileFrame, result: &ResultData) {
    super::qam_history::retain(profile.profile_id, result);
    let performance = match result.backend().profile_performance() {
        Ok(performance) => performance,
        Err(error) => {
            reject(
                profile.profile_id,
                profile.key.channel_id,
                error.to_string(),
            );
            return;
        }
    };
    let points = performance
        .points
        .iter()
        .map(|point| Point {
            hz: point.frequency_hz.value(),
            mer: point.measurement.mer().map(|mer| mer.value()),
            samples: point.measurement.samples().value(),
            plc: point.role == ProfileCarrierRole::Plc,
            ideal: point.ideal_shannon_mer_db.map(|mer| mer.value()),
        })
        .collect();
    let entry = Entry {
        fec_details: Some(super::fec::snapshot(result)),
        id: performance.profile_id.value(),
        channel: performance.key.channel_id,
        status: serde_json::to_value(performance.status)
            .unwrap()
            .as_str()
            .unwrap()
            .into(),
        fec: Some(performance.fec),
        ber: performance.ber.map(|ber| ber.value()),
        pending: performance.unrecovered_generated_payloads,
        points,
        error: performance.error,
    };
    insert(entry);
}
/// Records an explicit per-profile setup failure without fabricated decoder counters.
/// # Arguments
/// * `id`, `channel` - Captured identities.
/// * `error` - Bounded setup diagnostic.
pub(crate) fn reject(id: u8, channel: u8, error: String) {
    insert(Entry {
        fec_details: None,
        id,
        channel,
        status: "failed".into(),
        fec: None,
        ber: None,
        pending: 0,
        points: vec![],
        error: Some(error),
    });
}
/// Queries one profile on its original frequency grid, independently of other instruments.
/// # Arguments
/// * `id` - Captured ID; -1 selects the most recently completed profile.
/// * `start`, `count` - Source offsets; zero count restores Full Span.
/// # Returns
/// Bounded extrema, exact frequency axes, counters and complete/partial status.
pub(crate) fn view(id: i32, start: i32, count: i32) -> Value {
    ENTRIES.with(|entries| {
        let entries = entries.borrow();
        let profiles = entries
            .iter()
            .map(|entry| json!({"id":entry.id,"status":entry.status}))
            .collect::<Vec<_>>();
        let selected = entries
            .iter()
            .find(|entry| i32::from(entry.id) == id)
            .or_else(|| {
                if id < 0 {
                    entries
                        .iter()
                        .rev()
                        .find(|entry| entry.status != "not_started")
                        .or_else(|| entries.first())
                } else {
                    None
                }
            });
        let Some(entry) = selected else {
            return json!({"available":false,"profiles":profiles,"reason":"Run the simulation to compare profiles."});
        };
        let total = entry.points.len();
        let (start, count) = if count == 0 {
            (0, total)
        } else {
            let (Ok(start), Ok(count)) = (usize::try_from(start), usize::try_from(count)) else {
                return json!({"error":"Invalid profile range."});
            };
            (start, count)
        };
        let Some(end) = start.checked_add(count).filter(|end| *end <= total) else {
            return json!({"error":"Choose a range within this profile."});
        };
        if count == 0 {
            return json!({"available":false,"profiles":profiles,"profile_id":entry.id,"channel":entry.channel,"status":entry.status,"reason":entry.error.clone().unwrap_or_else(||"This profile has not run.".into())});
        }
        let first = entry.points[start].hz;
        let last = entry.points[end - 1].hz;
        let span = f64::from((last - first).max(MIN_PROFILE_FREQUENCY_AXIS_SPAN_HZ));
        let mut maximum = PERFORMANCE_INITIAL_MAXIMUM_DB;
        let mut minimum = PERFORMANCE_INITIAL_MINIMUM_DB;
        for point in &entry.points[start..end] {
            if let Some(value) = point.mer {
                maximum = maximum.max((value / PERFORMANCE_AXIS_QUANTUM_DB).ceil() * PERFORMANCE_AXIS_QUANTUM_DB);
                minimum = minimum.min((value / PERFORMANCE_AXIS_QUANTUM_DB).floor() * PERFORMANCE_AXIS_QUANTUM_DB);
            }
        }
        let mut series = Vec::new();
        for ideal in [false, true] {
            let mut points = Vec::new();
            for bucket in entry.points[start..end].chunks(count.div_ceil(PERFORMANCE_CURVE_BUCKETS)) {
                let values = bucket
                    .iter()
                    .filter_map(|point| {
                        if ideal {
                            point.ideal
                        } else {
                            point.mer
                        }
                    })
                    .collect::<Vec<_>>();
                if values.is_empty() {
                    points.push(json!({"y":null}));
                    continue;
                }
                let low = values.iter().copied().fold(f64::INFINITY, f64::min);
                let high = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                points.push(json!({
                    "x":f64::from(bucket[0].hz - first) / span,
                    "y":(maximum - high) / (maximum - minimum),
                    "minimum_y":(maximum - low) / (maximum - minimum),
                    "gap":values.len() != bucket.len(),
                    "readout":format!("{:.3} MHz · {} · {low:.2}–{high:.2} dB", f64::from(bucket[0].hz) / HERTZ_PER_MEGAHERTZ, if ideal {"Ideal Profile Minimum"} else {"Receiver MER"})
                }));
            }
            series.push(json!({"id":if ideal {PERFORMANCE_IDEAL_SERIES_ID} else {PERFORMANCE_OBSERVED_SERIES_ID},"points":points}));
        }
        json!({"available":true,"profiles":profiles,"profile_id":entry.id,"channel":entry.channel,"status":entry.status,"fec":fec_counters(entry.fec.as_ref()),"ber":entry.ber,"unrecovered":entry.pending,"error":entry.error,
            "total":total,"start":start,"count":count,"points":[{"x":0},{"x":1}],"series":series,"bands":plc_bands(&entry.points,start,end),"lanes":[],"markers":[],"axis_label":"MER (dB)",
            "samples":entry.points.iter().map(|point|point.samples).sum::<u64>(),"scope":"Measured data-region cells · FEC: contained payload words · Each captured profile runs as Profile A (wire ID 0)",
            "summary":format!("Frequency selection: {:.3} – {:.3} MHz",f64::from(first)/HERTZ_PER_MEGAHERTZ,f64::from(last)/HERTZ_PER_MEGAHERTZ),
            "x_ticks":(0..=PROFILE_FREQUENCY_TICK_INTERVALS).map(|i|json!({"x":f64::from(i)/f64::from(PROFILE_FREQUENCY_TICK_INTERVALS),"label":format!("{:.3}",(f64::from(first)+f64::from(i)/f64::from(PROFILE_FREQUENCY_TICK_INTERVALS)*f64::from(last-first))/HERTZ_PER_MEGAHERTZ)})).collect::<Vec<_>>(),
            "y_ticks":(0..=PERFORMANCE_METRIC_TICK_INTERVALS).map(|i|json!({"x":f64::from(i)/f64::from(PERFORMANCE_METRIC_TICK_INTERVALS),"label":format!("{:.1}",maximum-f64::from(i)/f64::from(PERFORMANCE_METRIC_TICK_INTERVALS)*(maximum-minimum))})).collect::<Vec<_>>()})
    })
}

// Preserve the existing UI counter keys using only the scoped public report getters.
fn fec_counters(report: Option<&FecReport>) -> Value {
    json!({
        "total_cw":report.map_or(0, |report|report.total_codewords().value()),
        "ccw":report.map_or(0, |report|report.corrected_codewords().value()),
        "ufec":report.map_or(0, |report|report.uncorrectable_codewords().value()),
        "clean_cw":report.map_or(0, |report|report.clean_codewords().value()),
        "undetected_errors":report.map_or(0, |report|report.undetected_errors().value())
    })
}

// Coordinates use original carrier centers, independently of curve decimation.
fn plc_bands(points: &[Point], start: usize, end: usize) -> Value {
    let mut carriers = points.iter().enumerate().filter(|(_, point)| point.plc);
    let Some((first, _)) = carriers.next() else {
        return json!([]);
    };
    let last = carriers.next_back().map_or(first, |(index, _)| index);
    if start >= end || last < start || first >= end {
        return json!([]);
    }
    let denominator = (end - start - 1).max(1) as f64;
    let left = (first.saturating_sub(start) as f64 / denominator).min(1.0);
    let right = (last.saturating_sub(start) as f64 / denominator).min(1.0);
    json!([{"first":left,"last":right,"label":"PLC"}])
}

/// Reads one bounded retained FEC summary without selecting other instruments.
/// # Arguments
/// * `id` - Captured profile identity; -1 follows the most recently finished run.
/// # Returns
/// Actual per-profile graphs and records, or explicit unstarted/failed state.
pub(crate) fn fec_view(id: i32) -> Value {
    ENTRIES.with(|entries| {
        let entries = entries.borrow();
        let profiles = entries
            .iter()
            .map(|entry| json!({"id":entry.id,"status":entry.status}))
            .collect::<Vec<_>>();
        let selected = entries
            .iter()
            .find(|entry| i32::from(entry.id) == id)
            .or_else(|| {
                if id < 0 {
                    entries
                        .iter()
                        .rev()
                        .find(|entry| entry.status != "not_started")
                        .or_else(|| entries.first())
                } else {
                    None
                }
            });
        let mut result = selected.map_or_else(
            || {
                if id < 0 && entries.is_empty() {
                    super::diagnostics::fec()
                } else {
                    json!({"available":false,"reason":"This profile has no retained FEC result."})
                }
            },
            |entry| {
                entry.fec_details.clone().unwrap_or_else(|| json!({
                "available":false,"profile_id":entry.id,"status":entry.status,
                "reason":entry.error.clone().unwrap_or_else(||"This profile has not run.".into())
            }))
            },
        );
        if let Some(entry) = selected {
            result["profile_id"] = json!(entry.id);
        }
        result["profiles"] = json!(profiles);
        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plc_band_uses_selected_profile_centers_and_clips_without_decimation() {
        let mut points: Vec<_> = (0..101)
            .map(|index| Point {
                hz: 100_000_000 + index * 25_000,
                mer: None,
                ideal: None,
                samples: 0,
                plc: index == 10 || index == 20,
            })
            .collect();
        assert_eq!(
            plc_bands(&points, 0, 101),
            json!([{"first":0.1,"last":0.2,"label":"PLC"}])
        );
        assert_eq!(
            plc_bands(&points, 15, 26),
            json!([{"first":0.0,"last":0.5,"label":"PLC"}])
        );
        assert_eq!(
            plc_bands(&points, 11, 20),
            json!([{"first":0.0,"last":1.0,"label":"PLC"}])
        );
        assert_eq!(plc_bands(&points, 30, 40), json!([]));
        assert_eq!(
            plc_bands(&points, 20, 21),
            json!([{"first":0.0,"last":0.0,"label":"PLC"}])
        );
        for point in &mut points {
            point.plc = false;
        }
        assert_eq!(plc_bands(&points, 0, 101), json!([]));
        points[50].plc = true;
        assert_eq!(
            plc_bands(&points, 0, 101),
            json!([{"first":0.5,"last":0.5,"label":"PLC"}])
        );
    }
}
