/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded display projections of retained soft decisions; never decoder inputs.
use merlin::downstream::Simulation;
use serde_json::{json, Value};
/// Formats only backend-prepared likelihood distributions and original-index extrema.
/// # Arguments
/// * `view` - Authoritative selected-cell or decoder soft report; reference planes stay separate.
/// # Returns
/// Existing chart fields; only bar geometry/percentage labels use presentation arithmetic.
/// # Errors
/// Preserves unexpected public report serialization failures without panicking.
pub(super) fn confidence(
    view: &merlin::downstream::FullLoopSoftValueView,
) -> Result<Value, String> {
    let report = serde_json::to_value(view).map_err(|cause| cause.to_string())?;
    Ok(format_confidence(&report))
}
fn format_confidence(view: &Value) -> Value {
    let histogram = |name: &str| {
        let data = &view[name];
        let bins = data["bins"].as_array().map(Vec::as_slice).unwrap_or(&[]);
        // Empty chart height uses a unit denominator; no measurement is substituted.
        let maximum = bins
            .iter()
            .filter_map(|bin| bin["count"].as_u64())
            .max()
            .unwrap_or(0)
            .max(1);
        let total = data["total_values"].as_u64().unwrap_or(0).max(1);
        let bars:Vec<_>=bins.iter().map(|bin|json!({"low":bin["low"],"high":bin["high"],"count":bin["count"],"height":bin["count"].as_u64().unwrap_or(0) as f64/maximum as f64,"percent":crate::shared::presentation::constants::PERCENT_PER_UNIT*bin["count"].as_u64().unwrap_or(0) as f64/total as f64})).collect();
        json!({"bars":bars,"limit":data["half_width"],"maximum":maximum,"shown":data["shown_values"]})
    };
    let sequence:Vec<_>=view["sequence"].as_array().map(Vec::as_slice).unwrap_or(&[]).iter().map(|group|json!({"first":group["first"],"last":group["last"],"low":group["minimum"],"high":group["maximum"]})).collect();
    json!({"full":histogram("full"),"near":histogram("near_zero"),"negative_limit":view["negative_limit_count"],"positive_limit":view["positive_limit_count"],"count":view["statistics"]["count"],"sequence":sequence,"sequence_limit":view["sequence_abs_limit"]})
}
/// Queries the backend's selected-symbol region against its complete reference.
/// # Arguments
/// * `order`, `label` - Existing physical constellation and nearest label.
/// # Returns
/// Normalized local reference points, exact decision polygon and viewport radius;
/// unsupported or out-of-range selectors return an empty object.
pub(super) fn neighborhood(order: usize, label: usize) -> Value {
    let Ok(order) = u16::try_from(order) else {
        return json!({});
    };
    let Ok(label) = u16::try_from(label) else {
        return json!({});
    };
    // This view uses the existing unit-energy reference plane, not received gain.
    let Ok(region) = Simulation::decision_region(order, label, 1.0) else {
        return json!({});
    };
    let references: Vec<_> = region
        .nearby_references
        .iter()
        .map(|reference| reference.relative_iq)
        .collect();
    json!({
        "center": region.center_iq,
        "radius": region.viewport_half_extent,
        "references": references,
        "polygon": region.vertices_relative,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn histogram_conserves_limits_and_zero_and_sequence_extrema() {
        // Original seven-value endpoints/zero and32768→1024 extrema vectors remain
        // independently evaluated in the Rust soft owner. This fixture verifies the
        // adapter preserves those authoritative values and original coding indices.
        let fixture = json!({"statistics":{"count":7},"full":{"bins":[{"low":-100,"high":-95,"count":1}],"half_width":100,"total_values":7,"shown_values":5},"near_zero":{"bins":[],"half_width":10,"total_values":7,"shown_values":3},"negative_limit_count":1,"positive_limit_count":1,"sequence_abs_limit":100,"sequence":[{"first":0,"last":6,"minimum":-100,"maximum":100}]});
        let result = format_confidence(&fixture);
        assert_eq!(result["full"]["shown"], 5);
        assert_eq!(result["negative_limit"], 1);
        assert_eq!(result["positive_limit"], 1);
        assert_eq!(result["near"]["shown"], 3);
        let empty = format_confidence(&json!({"statistics":{"count":0},"sequence":[]}));
        assert_eq!(empty["count"], 0);
        let mut large = fixture;
        large["statistics"]["count"] = json!(32768);
        large["sequence"]=json!((0..1024).map(|index|json!({"first":index*32,"last":index*32+31,"minimum":if index==0 {-100.0} else {0.0},"maximum":if index==0 {100.0} else {0.0}})).collect::<Vec<_>>());
        let result = format_confidence(&large);
        assert_eq!(result["sequence"].as_array().unwrap().len(), 1024);
        assert_eq!(result["sequence"][0]["low"], -100.0);
        assert_eq!(result["sequence"][0]["high"], 100.0);
    }
    #[test]
    fn invalid_reference_selectors_return_empty_geometry() {
        // Unsupported order three; label sixteen is just outside 16-QAM.
        for (order, label) in [(3, 0), (16, 16), (usize::MAX, 0), (16, usize::MAX)] {
            assert_eq!(neighborhood(order, label), json!({}));
        }
    }

    #[test]
    fn decision_region_contains_only_nearest_reference_for_square_and_cross_qam() {
        for order in [16, 128, 4096] {
            // Independent coverage anchors: square 16-QAM, cross 128-QAM and
            // the previously covered 4096-QAM reference; label zero, unit energy.
            let reference = Simulation::constellation_reference(order as u16, 1.0, 0, 0).unwrap();
            let result = neighborhood(order, 0);
            for vertex in result["polygon"].as_array().unwrap() {
                let x = vertex[0].as_f64().unwrap();
                let y = vertex[1].as_f64().unwrap();
                let distance = x * x + y * y;
                for point in &reference.points {
                    let dx = x + reference.points[0].iq[0] - point.iq[0];
                    let dy = y + reference.points[0].iq[1] - point.iq[1];
                    // Independent squared-distance floating-point tolerance.
                    assert!(distance <= dx * dx + dy * dy + 1e-10);
                }
            }
        }
    }
}
