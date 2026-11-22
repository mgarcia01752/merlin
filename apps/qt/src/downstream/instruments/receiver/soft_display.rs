/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded display projections of retained soft decisions; never decoder inputs.
use docsis_ofdm_fec::signal_processing::modulation::reference_for_order;
use serde_json::{json, Value};
const DISPLAY_LIMIT: f64 = 100.0;
const BIN_COUNT: usize = 40;

fn histogram(values: &[f64], limit: f64) -> Value {
    let mut bins = [0_usize; BIN_COUNT];
    for &value in values {
        if value > -limit && value < limit {
            let index =
                (((value + limit) / (2.0 * limit) * BIN_COUNT as f64) as usize).min(BIN_COUNT - 1);
            bins[index] += 1;
        }
    }
    let maximum = bins.iter().copied().max().unwrap_or(0).max(1);
    let bars: Vec<_> = bins.iter().enumerate().map(|(index, count)| {
        let low = -limit + index as f64 * 2.0 * limit / BIN_COUNT as f64;
        json!({"low":low,"high":low+2.0*limit/BIN_COUNT as f64,"count":count,
            "height":*count as f64/maximum as f64,"percent":100.0 * *count as f64/values.len().max(1) as f64})
    }).collect();
    json!({"bars":bars,"limit":limit,"maximum":maximum,"shown":bins.iter().sum::<usize>()})
}
/// Builds bounded bars and a decoder-index envelope from finite retained LLRs.
/// # Arguments
/// * `values` - Validated retained values, at most 32768.
/// # Returns
/// Full/near-zero histograms, endpoint counts and at most 1024 min/max groups.
pub(super) fn confidence(values: &[f64]) -> Value {
    let sequence: Vec<_> = values
        .chunks(values.len().div_ceil(1024).max(1))
        .enumerate()
        .map(|(index, group)| {
            let size = values.len().div_ceil(1024).max(1);
            json!({"first":index*size,"last":index*size+group.len()-1,
            "low":group.iter().copied().fold(f64::INFINITY,f64::min),
            "high":group.iter().copied().fold(f64::NEG_INFINITY,f64::max)})
        })
        .collect();
    json!({"full":histogram(values,DISPLAY_LIMIT),"near":histogram(values,10.0),
        "negative_limit":values.iter().filter(|&&v|v<=-DISPLAY_LIMIT).count(),
        "positive_limit":values.iter().filter(|&&v|v>=DISPLAY_LIMIT).count(),
        "count":values.len(),"sequence":sequence,
        "sequence_limit":values.iter().map(|v|v.abs()).fold(1.0_f64,f64::max)})
}
/// Clips the selected symbol's Voronoi region against the complete reference.
/// # Arguments
/// * `order`, `label` - Existing physical constellation and nearest label.
/// # Returns
/// Normalized local reference points, exact decision polygon and viewport radius.
pub(super) fn neighborhood(order: usize, label: usize) -> Value {
    let Some(reference) = reference_for_order(order) else {
        return json!({});
    };
    let center = reference.iq[label];
    let mut neighbors: Vec<_> = reference
        .iq
        .iter()
        .map(|p| [p[0] - center[0], p[1] - center[1]])
        .collect();
    neighbors.sort_by(|a, b| (a[0] * a[0] + a[1] * a[1]).total_cmp(&(b[0] * b[0] + b[1] * b[1])));
    let spacing = neighbors
        .iter()
        .map(|p| p[0].hypot(p[1]))
        .find(|d| *d > 0.0)
        .unwrap_or(1.0);
    let radius = spacing * 2.5;
    let mut polygon = vec![
        [-radius, -radius],
        [radius, -radius],
        [radius, radius],
        [-radius, radius],
    ];
    for point in neighbors.iter().skip(1) {
        let bound = (point[0] * point[0] + point[1] * point[1]) / 2.0;
        let mut clipped = Vec::new();
        for index in 0..polygon.len() {
            let a = polygon[index];
            let b = polygon[(index + 1) % polygon.len()];
            let da = a[0] * point[0] + a[1] * point[1] - bound;
            let db = b[0] * point[0] + b[1] * point[1] - bound;
            if da <= 0.0 {
                clipped.push(a);
            }
            if (da <= 0.0) != (db <= 0.0) {
                let fraction = da / (da - db);
                clipped.push([
                    a[0] + fraction * (b[0] - a[0]),
                    a[1] + fraction * (b[1] - a[1]),
                ]);
            }
        }
        polygon = clipped;
    }
    neighbors.truncate(49);
    json!({"center":center,"radius":radius,"references":neighbors,"polygon":polygon})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn histogram_conserves_limits_and_zero_and_sequence_extrema() {
        let result = confidence(&[-100.0, -99.0, -1.0, 0.0, 1.0, 99.0, 100.0]);
        assert_eq!(result["full"]["shown"], 5);
        assert_eq!(result["negative_limit"], 1);
        assert_eq!(result["positive_limit"], 1);
        assert_eq!(result["near"]["shown"], 3);
        assert_eq!(confidence(&[])["count"], 0);
        let mut values = vec![0.0; 32768];
        values[15] = -100.0;
        values[16] = 100.0;
        let result = confidence(&values);
        assert_eq!(result["sequence"].as_array().unwrap().len(), 1024);
        assert_eq!(result["sequence"][0]["low"], -100.0);
        assert_eq!(result["sequence"][0]["high"], 100.0);
    }
    #[test]
    fn decision_region_contains_only_nearest_reference_for_square_and_cross_qam() {
        for order in [16, 128, 4096] {
            let reference = reference_for_order(order).unwrap();
            let result = neighborhood(order, 0);
            for vertex in result["polygon"].as_array().unwrap() {
                let x = vertex[0].as_f64().unwrap();
                let y = vertex[1].as_f64().unwrap();
                let distance = x * x + y * y;
                for point in reference.iq {
                    let dx = x + reference.iq[0][0] - point[0];
                    let dy = y + reference.iq[0][1] - point[1];
                    assert!(distance <= dx * dx + dy * dy + 1e-10);
                }
            }
        }
    }
}
