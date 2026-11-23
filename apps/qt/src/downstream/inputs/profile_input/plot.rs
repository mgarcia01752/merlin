/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Drawing-only profile views over authoritative public inspection results.
use super::document::Capture;
use crate::{
    downstream::common::constants::*,
    shared::presentation::constants::{BITS_PER_MEGABIT, HERTZ_PER_MEGAHERTZ},
};
use merlin::downstream::{
    inputs::CaptureLoader, MeasurementViewport, NamedCapture, ProfileCapture, ProfileCarrierRole,
    ProfileInspection, ProfileMetric, ProfileViewRequest, MAX_PROFILE_SOURCE_IDS,
    MAX_PROFILE_VIEW_BUCKETS,
};
use serde_json::{json, Value};

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ViewRequest {
    profiles: Vec<u8>,
    roles: Vec<ProfileCarrierRole>,
    metric: u8,
    start: usize,
    count: usize,
}

fn admitted(capture: &Capture) -> Result<ProfileCapture, String> {
    let original = NamedCapture::new(&capture.name, capture.original.as_ref().clone())
        .map_err(|error| error.message().to_owned())?;
    CaptureLoader::profile_capture(&original).map_err(|error| error.message().to_owned())
}

/// Projects a draft profile using public raw extrema and original coordinates.
/// # Arguments
/// * `capture` - Validated original binary/JSON input retained unchanged by the editor.
/// * `profile_id` - Original byte identity, independent of the runnable ProfileId domain.
/// * `start`, `count` - Source viewport; zero count resets to the full original profile.
/// # Returns
/// Bounded normalized points/ticks with authoritative role/loading/frequency metadata.
/// # Errors
/// Preserves capture admission and unavailable/overflow/out-of-bounds viewport errors.
pub(crate) fn viewport(
    capture: &Capture,
    profile_id: u8,
    start: usize,
    count: usize,
) -> Result<Value, String> {
    viewport_document(&admitted(capture)?, profile_id, start, count)
}

fn viewport_document(
    capture: &ProfileCapture,
    profile_id: u8,
    start: usize,
    count: usize,
) -> Result<Value, String> {
    if !capture.profile_ids().contains(&profile_id) {
        return Err("Choose an available profile.".into());
    }
    let total = capture.metadata().key.grid.sample_count;
    let (start, count) = if count == 0 {
        (0, total)
    } else {
        (start, count)
    };
    let end = start.checked_add(count).ok_or("Profile range overflow.")?;
    if count == 0 || end > total {
        return Err("Choose a range inside the captured profile.".into());
    }
    let view = capture
        .inspect(&ProfileViewRequest {
            profiles: Some(vec![profile_id]),
            viewport: MeasurementViewport { start, count },
            max_buckets: PROFILE_SINGLE_TRACE_COLUMNS,
            ..Default::default()
        })
        .map_err(|error| error.message().to_owned())?;
    let first = view.first_frequency_hz;
    let last = view.last_frequency_hz;
    let span = f64::from(
        last.saturating_sub(first)
            .max(MIN_PROFILE_FREQUENCY_AXIS_SPAN_HZ),
    );
    let buckets = &view.series[0].buckets;
    let roles = ProfileCarrierRole::ALL
        .iter()
        .filter(|kind| {
            buckets
                .iter()
                .any(|bucket| bucket.roles.iter().any(|role| role.kind == **kind))
        })
        .map(|kind| json!({"kind":kind,"label":label(*kind)}))
        .collect::<Vec<_>>();
    let mut points = Vec::new();
    for bucket in buckets {
        for role in &bucket.roles {
            let minimum = role.minimum_bit_loading;
            let maximum = role.maximum_bit_loading;
            let frequency = role.first.frequency_hz;
            let physical_bin = role.first.physical_carrier;
            let loading = match (minimum, maximum) {
                (Some(low), Some(high)) if low == high => format!("{low} bits/symbol"),
                (Some(low), Some(high)) => format!("{low}–{high} bits/symbol"),
                _ => "No bit loading · role marker".into(),
            };
            points.push(json!({"x":f64::from(frequency-first)/span,
                "y":1.0-f64::from(maximum.unwrap_or(0))/f64::from(PROFILE_SINGLE_BIT_AXIS_MAX),
                "minimum_y":1.0-f64::from(minimum.unwrap_or(0))/f64::from(PROFILE_SINGLE_BIT_AXIS_MAX),
                "role":role.kind,"gap":count>PROFILE_SINGLE_TRACE_COLUMNS,"bits":maximum,
                "bin":physical_bin,"frequency_hz":frequency,
                "readout":format!("{:.3} MHz · Bin {physical_bin} · {} · {loading} · {} carrier(s)",f64::from(frequency)/HERTZ_PER_MEGAHERTZ,label(role.kind),role.count)}));
        }
    }
    points.sort_by(|left, right| {
        left["x"]
            .as_f64()
            .unwrap()
            .total_cmp(&right["x"].as_f64().unwrap())
    });
    let x_ticks=(0..=PROFILE_FREQUENCY_TICK_INTERVALS).map(|tick| {
        let fraction=f64::from(tick)/f64::from(PROFILE_FREQUENCY_TICK_INTERVALS);
        json!({"x":fraction,"label":format!("{:.3}",(f64::from(first)+fraction*f64::from(last-first))/HERTZ_PER_MEGAHERTZ)})
    }).collect::<Vec<_>>();
    let bit_intervals = PROFILE_SINGLE_BIT_AXIS_MAX / PROFILE_SINGLE_BIT_TICK_STEP;
    let y_ticks=(0..=bit_intervals).map(|tick|json!({"x":1.0-f64::from(tick)/f64::from(bit_intervals),"label":(tick*PROFILE_SINGLE_BIT_TICK_STEP).to_string()})).collect::<Vec<_>>();
    Ok(
        json!({"profile_id":profile_id,"total":total,"start":start,"count":count,
        "points":points,"roles":roles,"x_ticks":x_ticks,"y_ticks":y_ticks,
        "axis_label":"Bit Loading (Bits/Symbol)","frequency_label":"Frequency (MHz)",
        "step":true,"summary":format!("Profile {profile_id} · {count} of {total} carriers"),
        "role_markers":points.iter().any(|point|point["bits"].is_null())}),
    )
}

fn label(kind: ProfileCarrierRole) -> &'static str {
    match kind {
        ProfileCarrierRole::Data => "Data",
        ProfileCarrierRole::ContinuousPilot => "Pilots",
        ProfileCarrierRole::Plc => "PLC",
        ProfileCarrierRole::Exclusion => "Excluded",
        ProfileCarrierRole::ZeroBitLoaded => "Zero-loaded",
        ProfileCarrierRole::Reserved => "Reserved",
        ProfileCarrierRole::Unknown => "Unknown",
    }
}

/// Formats independent public profile metrics without recalculating scientific values.
/// # Arguments
/// * `capture` - Retained original binary/JSON draft, never modified by this operation.
/// * `request` - Bounded JSON selection: profiles, roles, legacy metric selector and range.
/// # Returns
/// Existing bounded overlay/role-lane JSON, summaries and original frequency axes.
/// # Errors
/// Rejects malformed/oversized requests, unavailable identities, metrics and viewports.
pub(crate) fn inspect(capture: &Capture, request: &str) -> Result<Value, String> {
    if request.len() > MAX_PROFILE_VIEW_REQUEST_BYTES {
        return Err("Profile view request is too large.".into());
    }
    let mut request: ViewRequest =
        serde_json::from_str(request).map_err(|error| error.to_string())?;
    let metric = match request.metric {
        PROFILE_VIEW_IDEAL_DB_SELECTOR => ProfileMetric::IdealShannonSnr,
        PROFILE_VIEW_BITS_SELECTOR => ProfileMetric::BitLoading,
        _ => return Err("Invalid profile view selection.".into()),
    };
    if request.profiles.len() > MAX_PROFILE_SOURCE_IDS
        || request.roles.len() > ProfileCarrierRole::ALL.len()
    {
        return Err("Invalid profile view selection.".into());
    }
    request.profiles.sort_unstable();
    request.profiles.dedup();
    let public = admitted(capture)?;
    for id in &request.profiles {
        if !public.profile_ids().contains(id) {
            return Err("Choose an available profile.".into());
        }
    }
    let reference = *public
        .profile_ids()
        .first()
        .ok_or("No captured profiles.")?;
    let mut output = viewport_document(&public, reference, request.start, request.count)?;
    let view = public
        .inspect(&ProfileViewRequest {
            profiles: Some(request.profiles),
            roles: request.roles.clone(),
            metric,
            viewport: MeasurementViewport {
                start: request.start,
                count: request.count,
            },
            max_buckets: MAX_PROFILE_VIEW_BUCKETS,
        })
        .map_err(|error| error.message().to_owned())?;
    format_inspection(&mut output, &view, &request.roles, request.metric);
    output["channel"] = capture.document["channel_id"].clone();
    output["mac"] = capture.document["mac_address"].clone();
    Ok(output)
}

fn format_inspection(
    output: &mut Value,
    view: &ProfileInspection,
    selected_roles: &[ProfileCarrierRole],
    metric: u8,
) {
    let first = view.first_frequency_hz;
    let last = view.last_frequency_hz;
    let span = f64::from(
        last.saturating_sub(first)
            .max(MIN_PROFILE_FREQUENCY_AXIS_SPAN_HZ),
    );
    let (maximum, units) = if metric == PROFILE_VIEW_IDEAL_DB_SELECTOR {
        (PROFILE_OVERLAY_SHANNON_AXIS_MAX_DB, "dB")
    } else {
        (PROFILE_OVERLAY_BIT_AXIS_MAX, "bits/symbol")
    };
    let profiles=view.profiles.iter().map(|profile| {
        let bps=profile.uncoded_data_capacity_bps;
        json!({"id":profile.profile_id,"bandwidth_bps":bps,"bandwidth_text":format!("{:.3} Mbit/s",bps as f64/BITS_PER_MEGABIT)})
    }).collect::<Vec<_>>();
    let roles = view
        .selected_role_counts
        .iter()
        .map(|role| json!({"kind":role.kind,"label":label(role.kind),"count":role.count}))
        .collect::<Vec<_>>();
    let lanes = view
        .selected_role_counts
        .iter()
        .filter(|role| {
            role.kind != ProfileCarrierRole::Data
                && selected_roles.contains(&role.kind)
                && role.count > 0
        })
        .map(|role| role.kind)
        .collect::<Vec<_>>();
    let mut series = Vec::new();
    let mut markers = Vec::new();
    for source in &view.series {
        let mut points = Vec::new();
        for bucket in &source.buckets {
            for role in &bucket.roles {
                if lanes.contains(&role.kind) {
                    markers.push(json!({"role":role.kind,"x":f64::from(role.first.frequency_hz-first)/span,"last_x":f64::from(role.last.frequency_hz-first)/span,"count":role.count}));
                }
            }
            let (Some(low), Some(high), Some(coordinate)) =
                (bucket.minimum, bucket.maximum, bucket.first_value)
            else {
                points.push(json!({"y":null}));
                continue;
            };
            let hz = coordinate.frequency_hz;
            points.push(json!({"x":f64::from(hz-first)/span,"y":1.0-high/maximum,"minimum_y":1.0-low/maximum,
                "gap":bucket.gap,"readout":format!("Profile {} · {:.3} MHz · {low:.3}–{high:.3} {units}",source.profile_id,f64::from(hz)/HERTZ_PER_MEGAHERTZ)}));
        }
        series.push(json!({"id":source.profile_id,"points":points}));
    }
    output["series"] = json!(series);
    output["markers"] = json!(markers);
    output["lanes"] = json!(lanes);
    output["profiles"] = json!(profiles);
    output["roles"] = json!(roles);
    output["axis_label"] = json!(if metric == PROFILE_VIEW_IDEAL_DB_SELECTOR {
        "dB"
    } else {
        "Bits/Symbol"
    });
    output["y_ticks"]=json!((0..=PROFILE_OVERLAY_METRIC_TICK_INTERVALS).map(|tick|json!({"x":f64::from(tick)/f64::from(PROFILE_OVERLAY_METRIC_TICK_INTERVALS),"label":format!("{}",maximum*(1.0-f64::from(tick)/f64::from(PROFILE_OVERLAY_METRIC_TICK_INTERVALS)))})).collect::<Vec<_>>());
    output["summary"] = json!(format!(
        "Frequency selection: {:.3} – {:.3} MHz",
        f64::from(first) / HERTZ_PER_MEGAHERTZ,
        f64::from(last) / HERTZ_PER_MEGAHERTZ
    ));

    // Keep navigation anchors even when the selected trace set is empty.
    output["points"] = json!([{"x":0},{"x":1}]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_source_ids_request_bounds_and_capture_immutability_survive_public_inspection() {
        let mut document: Value = serde_json::from_slice(include_bytes!(
            "../../../../../../examples/measurements/single-channel/modulation_profile.json"
        ))
        .unwrap();

        // Independent encoded IDs include the byte ceiling and an ID outside 0..15 runs.
        for (profile, id) in document["profiles"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .zip([255, 17, 0, 1])
        {
            profile["profile_id"] = json!(id);
        }
        let capture = Capture::decode(
            "raw-ids.json".into(),
            serde_json::to_vec(&document).unwrap(),
        )
        .unwrap();
        let original = capture.original.clone();
        let hash = capture.sha256.clone();
        let decoded = capture.document.clone();
        let assignments = capture.profile(255).unwrap().assignments.clone();
        let request =
            json!({"profiles":[255,17,255],"roles":["data"],"metric":1,"start":20,"count":4});
        let view = inspect(&capture, &request.to_string()).unwrap();
        assert_eq!(view["series"][0]["id"], 17);
        assert_eq!(view["series"][1]["id"], 255);
        assert_eq!(view["series"].as_array().unwrap().len(), 2);
        assert_eq!(viewport(&capture, 255, 0, 1).unwrap()["profile_id"], 255);
        assert_eq!(viewport(&capture, 255, usize::MAX, 0).unwrap()["start"], 0);

        // Independent one-above boundaries: 8192 bytes, 256 raw IDs and seven roles.
        assert!(inspect(&capture, &" ".repeat(8193)).is_err());
        let mut invalid = request.clone();
        invalid["profiles"] = json!(vec![255; 257]);
        assert!(inspect(&capture, &invalid.to_string()).is_err());
        invalid = request.clone();
        invalid["roles"] = json!(vec!["data"; 8]);
        assert!(inspect(&capture, &invalid.to_string()).is_err());
        invalid = request;

        // Metric selector two is outside the existing dB/bit pair.
        invalid["metric"] = json!(2);
        assert!(inspect(&capture, &invalid.to_string()).is_err());
        assert_eq!(capture.original.as_ref(), original.as_ref());
        assert_eq!(capture.sha256, hash);
        assert_eq!(capture.document.as_ref(), decoded.as_ref());
        assert_eq!(capture.profile(255).unwrap().assignments, assignments);
    }
}
