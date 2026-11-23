/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! PLC bounds from the frozen physical layout, projected onto the PSD viewport.
use merlin::downstream::PhysicalLayoutView;
use serde_json::{json, Value};

/// Annotates a validated spectrum viewport with one first-to-last PLC region.
///
/// # Arguments
/// * `view` - Spectrum JSON with validated physical-bin start/count coordinates.
/// * `layout` - Frozen carrier roles used to generate the displayed samples.
///
/// # Returns
/// Unit; bands remain empty when no PLC or no intersection is present. Axis
/// label selection never shifts the region because both axes share bin order.
pub(crate) fn annotate(view: &mut Value, layout: &PhysicalLayoutView) {
    view["bands"] = json!([]);
    view["plc_summary"] = json!("");
    let first = usize::from(layout.plc_first_bin);
    let last = usize::from(layout.plc_end_exclusive) - 1;
    let start = view["start"].as_u64().unwrap_or(0) as f64;
    let count = view["count"].as_u64().unwrap_or(0);
    if count < 2 || (last as f64) < start || first as f64 > start + (count - 1) as f64 {
        return;
    }
    let denominator = (count - 1) as f64;
    let left = ((first as f64 - start) / denominator).clamp(0.0, 1.0);
    let right = ((last as f64 - start) / denominator).clamp(0.0, 1.0);
    view["bands"] = json!([{"first":left,"last":right,"label":"PLC"}]);
    view["plc_summary"] = json!(format!("PLC shaded · first–last bins {first}–{last}"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use merlin::downstream::{
        Dpd, NamedCapture, ProfileId, SubcarrierSpacing, TransmissionConfiguration,
        TransmissionSettings,
    };

    fn layout(spacing: SubcarrierSpacing, divisor: u16, first: u16) -> PhysicalLayoutView {
        let transmission = TransmissionConfiguration::new(TransmissionSettings {
            subcarrier_spacing: spacing,
            ..Default::default()
        })
        .unwrap();
        let neighborhood = first - 112 / divisor..first + 128 / divisor;
        let additional: Vec<_> = (0..60)
            .map(|offset| (480 + 120 * offset) / divisor)
            .filter(|bin| !neighborhood.contains(bin))
            .take(48)
            .collect();
        let mut source: Value = serde_json::from_slice(include_bytes!(
            "../../../../../../examples/measurements/single-channel/modulation_profile.json"
        ))
        .unwrap();
        let templates = source["profiles"][0]["schemes"].as_array().unwrap().clone();
        let mut schemes: Vec<Value> = vec![];
        for bin in 296 / divisor..7896 / divisor {
            let role = if (first..first + 16 / divisor).contains(&bin) {
                "plc"
            } else if additional.contains(&bin) {
                "continuous_pilot"
            } else {
                "data"
            };
            if let Some(previous) = schemes
                .last_mut()
                .filter(|previous| previous["carrier_type"] == role)
            {
                previous["num_subcarriers"] =
                    json!(previous["num_subcarriers"].as_u64().unwrap() + 1);
            } else {
                let mut scheme = templates
                    .iter()
                    .find(|scheme| scheme["carrier_type"] == role)
                    .unwrap()
                    .clone();
                scheme["num_subcarriers"] = json!(1);
                schemes.push(scheme);
            }
        }
        source["subcarrier_zero_frequency"] = json!(100_000_000);
        source["first_active_subcarrier_index"] = json!(296 / divisor);
        source["subcarrier_spacing"] = json!(spacing.frequency_hz());
        source["num_profiles"] = json!(1);
        source["profiles"] = json!([{"profile_id":0,"schemes":schemes}]);
        let profile = Dpd::from_bytes(
            &NamedCapture::new("original-grid", serde_json::to_vec(&source).unwrap()).unwrap(),
            ProfileId::new(0).unwrap(),
        )
        .unwrap();
        transmission
            .physical_layout(&profile, 0, 0, 0)
            .unwrap()
            .layout
    }

    #[test]
    fn plc_bounds_follow_spacing_layout_and_clipped_viewports() {
        for (spacing, divisor) in [(SubcarrierSpacing::Khz25, 1), (SubcarrierSpacing::Khz50, 2)] {
            for first in [4112 / divisor, 4352 / divisor] {
                // Same independent original PLC/pilot/grid fixture, through public admission.
                let layout = layout(spacing, divisor, first);
                let last = first + 16 / divisor - 1;
                let count = layout.fft_bins;
                let mut full = json!({"start":0,"count":count});
                annotate(&mut full, &layout);
                assert_eq!(
                    full["bands"][0]["first"],
                    json!(f64::from(first) / (count - 1) as f64)
                );
                assert_eq!(
                    full["bands"][0]["last"],
                    json!(f64::from(last) / (count - 1) as f64)
                );
                let mut partial = json!({"start":first+2,"count":32});
                annotate(&mut partial, &layout);
                assert_eq!(partial["bands"][0]["first"], 0.0);
                assert_eq!(
                    partial["bands"][0]["last"],
                    json!(f64::from(last - first - 2) / 31.0)
                );
                let mut outside = json!({"start":last+1,"count":32});
                annotate(&mut outside, &layout);
                assert_eq!(outside["bands"], json!([]));
            }
        }
    }
}
