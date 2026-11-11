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
