/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! PLC bounds from the frozen physical layout, projected onto the PSD viewport.
use docsis_ofdm_fec::transmission::layout::{CarrierLayout, PhysicalCarrierRole};
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
pub(crate) fn annotate(view: &mut Value, layout: &CarrierLayout) {
    view["bands"] = json!([]);
    view["plc_summary"] = json!("");
    let mut carriers = layout
        .static_roles()
        .iter()
        .enumerate()
        .filter(|(_, role)| **role == PhysicalCarrierRole::Plc)
        .map(|(bin, _)| bin);
    let Some(first) = carriers.next() else { return };
    let last = carriers.next_back().unwrap_or(first);
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
    use docsis_ofdm_fec::transmission::{
        layout::CarrierLayoutSettings, SubcarrierSpacing, TransmissionSettings,
    };

    #[test]
    fn plc_bounds_follow_spacing_layout_and_clipped_viewports() {
        for (spacing, divisor) in [(SubcarrierSpacing::Khz25, 1), (SubcarrierSpacing::Khz50, 2)] {
            for first in [4112 / divisor, 4352 / divisor] {
                let transmission = TransmissionSettings {
                    subcarrier_spacing: spacing,
                    ..Default::default()
                };
                let neighborhood = first - 112 / divisor..first + 128 / divisor;
                let settings = CarrierLayoutSettings {
                    zero_frequency_hz: 100_000_000,
                    first_active_bin: 296 / divisor,
                    active_end_exclusive: 7896 / divisor,
                    plc_first_bin: first,
                    excluded_bins: vec![],
                    additional_continuous_pilots: (0..60)
                        .map(|offset| (480 + 120 * offset) / divisor)
                        .filter(|bin| !neighborhood.contains(bin))
                        .take(48)
                        .collect(),
                };
                let layout = CarrierLayout::new(&transmission, &settings).unwrap();
                let last = first + 16 / divisor - 1;
                let count = layout.static_roles().len();
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
