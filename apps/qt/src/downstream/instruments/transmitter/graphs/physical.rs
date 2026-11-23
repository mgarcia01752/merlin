/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Display reduction over actual public bin powers and authoritative physical-role spans.
use crate::downstream::common::constants::{
    TX_PHYSICAL_POWER_BUCKETS, TX_PHYSICAL_POWER_MIN_SCALE,
};
use crate::shared::presentation::constants::HERTZ_PER_MEGAHERTZ;
use merlin::downstream::{FullLoopResult, FullLoopStage, PhysicalCarrierRole, PhysicalLayoutView};
use serde_json::{json, Value};

/// Formats the first actual physical-bin observation without reconstructing its science.
/// # Arguments
/// * `result` - Immutable completed or partial public result with bounded observations.
/// # Returns
/// Existing min/max graph and role bands from backend power/frequency/layout metadata.
/// Absent context or retained samples remain explicitly unavailable.
pub(super) fn project(result: &FullLoopResult) -> Value {
    match result.physical_bins(FullLoopStage::PhysicalBins, 0, 0, 0) {
        Ok(observation) => {
            let powers = observation
                .bins
                .iter()
                .map(|bin| bin.linear_power)
                .collect::<Vec<_>>();
            draw(
                &powers,
                &observation.layout,
                observation.context.channel_symbol_index,
            )
        }
        Err(cause) => json!({"kind":"empty","reason":cause.message()}),
    }
}

fn draw(powers: &[Option<f64>], layout: &PhysicalLayoutView, symbol: Option<u64>) -> Value {
    let Some(symbol) = symbol else {
        return json!({"kind":"empty","reason":"Captured symbol identity is unavailable."});
    };
    if powers.is_empty() || powers.len() != layout.carriers.len() {
        return json!({"kind":"empty","reason":"Captured bins are outside the frozen layout."});
    }
    let maximum = powers
        .iter()
        .flatten()
        .copied()
        .fold(TX_PHYSICAL_POWER_MIN_SCALE, f64::max);
    let group = powers.len().div_ceil(TX_PHYSICAL_POWER_BUCKETS).max(1);
    let denominator = powers.len().saturating_sub(1).max(1) as f64;
    let mut points = Vec::new();
    for (bucket, chunk) in powers.chunks(group).enumerate() {
        let mut samples = chunk
            .iter()
            .enumerate()
            .filter_map(|(index, power)| power.map(|power| (bucket * group + index, power)))
            .collect::<Vec<_>>();
        samples.sort_by(|left, right| left.1.total_cmp(&right.1));
        if let (Some(first), Some(last)) = (samples.first(), samples.last()) {
            let mut extremes = vec![*first];
            if first.0 != last.0 {
                extremes.push(*last);
            }
            extremes.sort_by_key(|sample| sample.0);
            for (index, power) in extremes {
                let carrier = &layout.carriers[index];
                points.push(json!({"x":index as f64/denominator,"y":1.0-power/maximum,
                    "gap":chunk.iter().any(Option::is_none),"readout":format!(
                        "Bin {} · {:.3} MHz · power {power:.6} · {}",carrier.physical_bin,
                        f64::from(carrier.frequency_hz)/HERTZ_PER_MEGAHERTZ,role_name(carrier.role))}));
            }
        } else {
            points.push(json!({"x":(bucket*group) as f64/denominator,"y":null,
                "readout":"Unavailable captured bins"}));
        }
    }
    let bands = layout.role_spans.iter().map(|span| json!({
        "x":(usize::from(span.first_bin)-layout.first_bin) as f64/powers.len() as f64,
        "end_x":(usize::from(span.end_exclusive)-layout.first_bin) as f64/powers.len() as f64,
        "role":role_name(span.role)
    })).collect::<Vec<_>>();
    json!({"kind":"physical","points":points,"bands":bands,
        "omitted_bands":layout.omitted_role_spans,"maximum":maximum,
        "x_label":"Frequency (MHz)","y_label":"Bin power · relative",
        "first":format!("{:.3}",f64::from(layout.carriers[0].frequency_hz)/HERTZ_PER_MEGAHERTZ),
        "last":format!("{:.3}",f64::from(layout.carriers[powers.len()-1].frequency_hz)/HERTZ_PER_MEGAHERTZ),
        "count":powers.len(),"symbol":symbol})
}

fn role_name(role: PhysicalCarrierRole) -> &'static str {
    match role {
        PhysicalCarrierRole::Data => "data_location",
        PhysicalCarrierRole::Excluded => "excluded",
        PhysicalCarrierRole::Plc => "plc",
        PhysicalCarrierRole::ContinuousPilot => "continuous_pilot",
        PhysicalCarrierRole::ScatteredPilot => "scattered_pilot",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use merlin::downstream::{
        Dpd, NamedCapture, ProfileId, TransmissionConfiguration, TransmissionSettings,
    };

    #[test]
    fn bins_preserve_peak_missing_power_and_layout_roles_within_bounds() {
        let profile = Dpd::from_bytes(
            &NamedCapture::new(
                "dpd.json",
                include_bytes!(
            "../../../../../../../examples/measurements/single-channel/modulation_profile.json"
        )
                .to_vec(),
            )
            .unwrap(),
            ProfileId::new(0).unwrap(),
        )
        .unwrap();
        let transmission = TransmissionConfiguration::new(TransmissionSettings::default()).unwrap();
        let configured = transmission.physical_layout(&profile, 0, 0, 0).unwrap();
        // Supplied formatting fixture: bin 17 has power 25, bin 18 is missing.
        // The original 3+4i->25 scientific oracle lives in waveform::bin_power tests.
        let mut powers = vec![Some(0.0); 8192];
        powers[17] = Some(25.0);
        powers[18] = None;
        let view = draw(&powers, &configured.layout, Some(0));
        assert_eq!(view["maximum"], 25.0);
        assert!(view["points"].as_array().unwrap().len() <= 2048);
        assert!(view["points"]
            .as_array()
            .unwrap()
            .iter()
            .any(|point| point["y"] == 0.0));
        assert!(view["points"]
            .as_array()
            .unwrap()
            .iter()
            .any(|point| point["gap"] == true));
        assert!(view["bands"]
            .as_array()
            .unwrap()
            .iter()
            .any(|band| band["role"] == "plc"));
        assert_eq!(draw(&powers, &configured.layout, None)["kind"], "empty");
        assert_eq!(view["omitted_bands"], configured.layout.omitted_role_spans);
        assert_eq!(
            view["bands"].as_array().unwrap().len(),
            configured.layout.role_spans.len()
        );
    }
}
