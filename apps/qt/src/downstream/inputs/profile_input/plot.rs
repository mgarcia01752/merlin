/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded profile projections for the shared PNM Viewer-derived chart.
use super::document::Capture;
use docsis_ofdm_fec::types::modulation::CarrierType;
use serde_json::{json, Value};

const DISPLAY_COLUMNS: usize = 512;
const MAX_BITS: f64 = 14.0;

/// Projects a draft profile into role-aware min/max groups on its original grid.
/// # Arguments
/// * `capture` - Validated original input retained by the editor.
/// * `profile_id` - Captured profile identity, never an array position.
/// * `start`, `count` - Source offsets; zero count selects the full profile.
/// # Returns
/// Bounded normalized points, ticks and exact frequency/readout metadata.
/// # Errors
/// Rejects unavailable profiles, empty/out-of-bounds ranges or invalid coordinates.
pub(crate) fn viewport(
    capture: &Capture,
    profile_id: u8,
    start: usize,
    count: usize,
) -> Result<Value, String> {
    let profile = capture.profile(profile_id)?;
    let total = profile.assignments.len();
    let (start, count) = if count == 0 {
        (0, total)
    } else {
        (start, count)
    };
    let end = start.checked_add(count).ok_or("Profile range overflow.")?;
    if count == 0 || end > total {
        return Err("Choose a range inside the captured profile.".into());
    }
    let grid = &profile.key.grid;
    let first = grid.frequency(start).map_err(|error| error.to_string())?;
    let last = grid.frequency(end - 1).map_err(|error| error.to_string())?;
    let span = f64::from(last.saturating_sub(first).max(1));
    let stride = count.div_ceil(DISPLAY_COLUMNS);
    let mut points = Vec::new();
    let mut roles = Vec::new();
    for kind in CarrierType::ALL {
        if profile.assignments[start..end]
            .iter()
            .any(|assignment| assignment.carrier_type == kind)
        {
            roles.push(json!({"kind":kind,"label":label(kind)}));
        }
    }
    for offset in (start..end).step_by(stride) {
        let stop = (offset + stride).min(end);
        for kind in CarrierType::ALL {
            let selected = (offset..stop)
                .filter(|index| profile.assignments[*index].carrier_type == kind)
                .collect::<Vec<_>>();
            let Some(&index) = selected.first() else {
                continue;
            };
            let values = selected
                .iter()
                .filter_map(|index| profile.assignments[*index].bit_loading);
            let minimum = values.clone().min();
            let maximum = values.max();
            let frequency = grid.frequency(index).map_err(|error| error.to_string())?;
            let physical_bin = usize::from(grid.first_subcarrier_index) + index;
            let loading = match (minimum, maximum) {
                (Some(low), Some(high)) if low == high => format!("{low} bits/symbol"),
                (Some(low), Some(high)) => format!("{low}–{high} bits/symbol"),
                _ => "No bit loading · role marker".into(),
            };
            points.push(json!({"x":f64::from(frequency-first)/span,
                "y":1.0-f64::from(maximum.unwrap_or(0))/MAX_BITS,
                "minimum_y":1.0-f64::from(minimum.unwrap_or(0))/MAX_BITS,
                "role":kind,"gap":stride>1,"bits":maximum,
                "bin":physical_bin,"frequency_hz":frequency,
                "readout":format!("{:.3} MHz · Bin {physical_bin} · {} · {loading} · {} carrier(s)", f64::from(frequency)/1e6,label(kind),selected.len())}));
        }
    }
    points.sort_by(|left, right| {
        left["x"]
            .as_f64()
            .unwrap()
            .total_cmp(&right["x"].as_f64().unwrap())
    });
    let x_ticks = (0..=4).map(|tick| {
        let fraction = f64::from(tick)/4.0;
        json!({"x":fraction,"label":format!("{:.3}",(f64::from(first)+fraction*f64::from(last-first))/1e6)})
    }).collect::<Vec<_>>();
    let y_ticks = (0..=7)
        .map(|tick| json!({"x":1.0-f64::from(tick)/7.0,"label":(tick*2).to_string()}))
        .collect::<Vec<_>>();
    Ok(
        json!({"profile_id":profile_id,"total":total,"start":start,"count":count,
        "points":points,"roles":roles,"x_ticks":x_ticks,"y_ticks":y_ticks,
        "axis_label":"Bit Loading (Bits/Symbol)","frequency_label":"Frequency (MHz)",
        "step":true,"summary":format!("Profile {profile_id} · {count} of {total} carriers"),
        "role_markers":points.iter().any(|point|point["bits"].is_null())}),
    )
}

fn label(kind: CarrierType) -> &'static str {
    match kind {
        CarrierType::Data => "Data",
        CarrierType::ContinuousPilot => "Pilots",
        CarrierType::Plc => "PLC",
        CarrierType::Exclusion => "Excluded",
        CarrierType::ZeroBitLoaded => "Zero-loaded",
        CarrierType::Reserved => "Reserved",
        CarrierType::Unknown => "Unknown",
    }
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ViewRequest {
    profiles: Vec<u8>,
    roles: Vec<CarrierType>,
    metric: u8,
    start: usize,
    count: usize,
}

/// Projects independent profile overlays using the viewer's metric semantics.
/// # Arguments
/// * `capture` - Retained validated binary or JSON draft.
/// * `request` - Bounded JSON selection, metric (0=dB, 1=bits) and source range.
/// # Returns
/// Metadata, bounded extrema, special-role lanes and stable frequency axes.
/// # Errors
/// Rejects malformed/oversized requests, unknown profiles, metrics and ranges.
pub(crate) fn inspect(capture: &Capture, request: &str) -> Result<Value, String> {
    use docsis_ofdm_fec::signal_processing::shannon::minimum_snr_db;
    if request.len() > 8192 {
        return Err("Profile view request is too large.".into());
    }
    let mut request: ViewRequest = serde_json::from_str(request).map_err(|e| e.to_string())?;
    if request.metric > 1 || request.profiles.len() > 256 || request.roles.len() > 7 {
        return Err("Invalid profile view selection.".into());
    }
    request.profiles.sort_unstable();
    request.profiles.dedup();
    for id in &request.profiles {
        capture.profile(*id)?;
    }
    let reference = capture.profiles.first().ok_or("No captured profiles.")?;
    let mut output = viewport(capture, reference.profile_id, request.start, request.count)?;
    let start = output["start"].as_u64().unwrap() as usize;
    let count = output["count"].as_u64().unwrap() as usize;
    let end = start + count;
    let grid = &reference.key.grid;
    let first = grid.frequency(start).map_err(|e| e.to_string())?;
    let last = grid.frequency(end - 1).map_err(|e| e.to_string())?;
    let span = f64::from(last.saturating_sub(first).max(1));
    let maximum = if request.metric == 0 { 45.0 } else { 15.0 };
    let profiles = capture.profiles.iter().map(|profile| {
        let bits: u64 = profile.assignments.iter().filter(|a| a.carrier_type == CarrierType::Data)
            .filter_map(|a| a.bit_loading).map(u64::from).sum();
        let bps = bits * u64::from(profile.key.grid.spacing_hz);
        json!({"id":profile.profile_id,"bandwidth_bps":bps,"bandwidth_text":format!("{:.3} Mbit/s",bps as f64/1e6)})
    }).collect::<Vec<_>>();
    let roles = CarrierType::ALL
        .iter()
        .map(|kind| {
            let count = capture
                .profiles
                .iter()
                .filter(|p| request.profiles.contains(&p.profile_id))
                .flat_map(|p| &p.assignments)
                .filter(|a| a.carrier_type == *kind)
                .count();
            json!({"kind":kind,"label":label(*kind),"count":count})
        })
        .collect::<Vec<_>>();
    let lanes = CarrierType::ALL
        .iter()
        .filter(|kind| {
            **kind != CarrierType::Data
                && request.roles.contains(kind)
                && roles.iter().any(|role| {
                    role["kind"] == json!(kind) && role["count"].as_u64().unwrap_or(0) > 0
                })
        })
        .copied()
        .collect::<Vec<_>>();
    let mut series = Vec::new();
    let mut markers = Vec::new();
    let budget = (2048 / request.profiles.len().max(1)).max(8);
    let stride = count.div_ceil(budget);
    for id in &request.profiles {
        let profile = capture.profile(*id)?;
        if profile.key.grid != *grid || profile.assignments.len() < end {
            return Err("Profile grids differ.".into());
        }
        let mut points = Vec::new();
        for offset in (start..end).step_by(stride) {
            let stop = (offset + stride).min(end);
            let mut values = Vec::new();
            let mut missing = false;
            for (index, assignment) in profile.assignments[offset..stop].iter().enumerate() {
                let kind = assignment.carrier_type;
                let selected = request.roles.contains(&kind);
                let value = if selected
                    && matches!(
                        kind,
                        CarrierType::Data | CarrierType::ContinuousPilot | CarrierType::Plc
                    ) {
                    assignment.bit_loading.and_then(|bits| {
                        if request.metric == 0 {
                            minimum_snr_db(bits)
                        } else {
                            Some(f64::from(bits))
                        }
                    })
                } else {
                    None
                };
                if let Some(value) = value {
                    values.push((offset + index, value));
                } else {
                    missing = true;
                }
            }
            for kind in &lanes {
                let indices = (offset..stop)
                    .filter(|i| profile.assignments[*i].carrier_type == *kind)
                    .collect::<Vec<_>>();
                if let (Some(a), Some(b)) = (indices.first(), indices.last()) {
                    markers.push(json!({"role":kind,"x":f64::from(grid.frequency(*a).map_err(|e|e.to_string())?-first)/span,"last_x":f64::from(grid.frequency(*b).map_err(|e|e.to_string())?-first)/span,"count":indices.len()}));
                }
            }
            if values.is_empty() {
                points.push(json!({"y":null}));
                continue;
            }
            let low = values.iter().map(|v| v.1).fold(f64::INFINITY, f64::min);
            let high = values.iter().map(|v| v.1).fold(f64::NEG_INFINITY, f64::max);
            let index = values[0].0;
            let hz = grid.frequency(index).map_err(|e| e.to_string())?;
            points.push(json!({"x":f64::from(hz-first)/span,"y":1.0-high/maximum,"minimum_y":1.0-low/maximum,
                "gap":missing,"readout":format!("Profile {id} · {:.3} MHz · {low:.3}–{high:.3} {}",f64::from(hz)/1e6,if request.metric==0 {"dB"} else {"bits/symbol"})}));
        }
        series.push(json!({"id":id,"points":points}));
    }
    output["series"] = json!(series);
    output["markers"] = json!(markers);
    output["lanes"] = json!(lanes);
    output["profiles"] = json!(profiles);
    output["roles"] = json!(roles);
    output["channel"] = capture.document["channel_id"].clone();
    output["mac"] = capture.document["mac_address"].clone();
    output["axis_label"] = json!(if request.metric == 0 {
        "dB"
    } else {
        "Bits/Symbol"
    });
    output["y_ticks"] = json!((0..=4)
        .map(|i| json!({"x":f64::from(i)/4.0,"label":format!("{}",maximum*(1.0-f64::from(i)/4.0))}))
        .collect::<Vec<_>>());
    output["summary"] = json!(format!(
        "Frequency selection: {:.3} – {:.3} MHz",
        f64::from(first) / 1e6,
        f64::from(last) / 1e6
    ));
    // Navigation anchors retain the common grid even with no selected trace.
    output["points"] = json!([{"x":0},{"x":1}]);
    Ok(output)
}
