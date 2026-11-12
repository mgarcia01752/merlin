/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Profile-derived PLC overlays, clipped to the exact chart viewport.
use docsis_ofdm_fec::{
    profile::candidate::CapturedProfile, scenario::FrequencyGrid, types::modulation::CarrierType,
};
use serde_json::{json, Value};
/// Adds PLC carrier footprints only when the accepted profile matches the source.
/// # Arguments
/// * `view` - Bounded chart JSON with original start/count coordinates.
/// * `grid` - Original measurement grid.
/// * `profile` - Accepted modulation profile, never inferred from MER values.
/// # Returns
/// Unit; incompatible/missing profiles leave the overlay empty.
pub(crate) fn annotate(view: &mut Value, grid: &FrequencyGrid, profile: Option<&CapturedProfile>) {
    view["bands"] = json!([]);
    let Some(profile) = profile.filter(|profile| profile.key.grid == *grid) else {
        return;
    };
    let start = view["start"].as_u64().unwrap_or(0) as usize;
    let count = view["count"].as_u64().unwrap_or(0) as usize;
    if count == 0 {
        return;
    }
    // A single continuous region spans the first and last PLC carrier centers.
    let mut carriers = profile
        .assignments
        .iter()
        .enumerate()
        .filter(|(_, assignment)| assignment.carrier_type == CarrierType::Plc)
        .map(|(index, _)| index);
    let Some(first) = carriers.next() else { return };
    let last = carriers.next_back().unwrap_or(first);
    if count == 1 {
        if (first..=last).contains(&start) {
            view["bands"] = json!([{"first":0.0,"last":1.0,"label":"PLC"}]);
        }
        return;
    }
    let denominator = (count - 1) as f64;
    let left = ((first as f64 - start as f64) / denominator).clamp(0.0, 1.0);
    let right = ((last as f64 - start as f64) / denominator).clamp(0.0, 1.0);
    if right > left {
        view["bands"] = json!([{"first":left,"last":right,"label":"PLC"}]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plc_region_spans_first_to_last_and_clips_when_zoomed() {
        let mut profile = docsis_ofdm_fec::scenario::pnm_json::import_profiles(
            docsis_ofdm_fec::scenario::pnm_json::JsonCapture {
                name: "profile",
                bytes: include_bytes!(
                    "../../../../../examples/measurements/single-channel/modulation_profile.json"
                ),
            },
        )
        .unwrap()
        .remove(0);
        // Deliberately separate assignments: rendering is one first-to-last region.
        for assignment in &mut profile.assignments {
            assignment.carrier_type = CarrierType::Data;
        }
        profile.assignments[10].carrier_type = CarrierType::Plc;
        profile.assignments[20].carrier_type = CarrierType::Plc;
        let mut view = json!({"start":0,"count":101});
        annotate(&mut view, &profile.key.grid, Some(&profile));
        assert_eq!(
            view["bands"],
            json!([{"first":0.1,"last":0.2,"label":"PLC"}])
        );
        view = json!({"start":15,"count":11});
        annotate(&mut view, &profile.key.grid, Some(&profile));
        assert_eq!(
            view["bands"],
            json!([{"first":0.0,"last":0.5,"label":"PLC"}])
        );
        view = json!({"start":30,"count":11});
        annotate(&mut view, &profile.key.grid, Some(&profile));
        assert_eq!(view["bands"], json!([]));
        view = json!({"start":20,"count":1});
        annotate(&mut view, &profile.key.grid, Some(&profile));
        assert_eq!(
            view["bands"],
            json!([{"first":0.0,"last":1.0,"label":"PLC"}])
        );
        annotate(&mut view, &profile.key.grid, None);
        assert_eq!(view["bands"], json!([]));
    }
}
