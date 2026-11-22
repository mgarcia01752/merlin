/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Physical-bin power envelope and exact core-layout role bands for one symbol.
use docsis_ofdm_fec::{
    simulation::channel::ChannelRunSettings,
    transmission::layout::{CarrierLayout, PhysicalCarrierRole},
};
use serde_json::{json, Value};

/// Reduces real physical-bin power and preserves exact layout-role boundaries.
/// # Arguments
/// * `values`, `offset` - Retained bins and their physical starting index.
/// * `symbol` - Actual captured channel symbol; required for scattered roles.
/// * `settings` - Frozen run transmission and layout.
/// # Returns
/// At most2048 power points and1024 role bands, or explicit missing geometry.
pub(super) fn project(
    values: &[Value],
    offset: usize,
    symbol: Option<u64>,
    settings: &ChannelRunSettings,
) -> Value {
    let Some(symbol) = symbol else {
        return json!({"kind":"empty","reason":"Captured symbol identity is unavailable."});
    };
    let Ok(layout) = CarrierLayout::new(&settings.transmission, &settings.layout) else {
        return json!({"kind":"empty","reason":"Captured physical layout is unavailable."});
    };
    let roles = layout.symbol_roles(symbol);
    if values.is_empty() || offset.saturating_add(values.len()) > roles.len() {
        return json!({"kind":"empty","reason":"Captured bins are outside the frozen layout."});
    }
    let power = values
        .iter()
        .map(|value| {
            super::iq(value)
                .map(|(i, q)| i * i + q * q)
                .filter(|power| power.is_finite())
        })
        .collect::<Vec<_>>();
    let maximum = power.iter().flatten().copied().fold(1.0f64, f64::max);
    let group = values.len().div_ceil(1024).max(1);
    let mut points = Vec::new();
    for (bucket, chunk) in power.chunks(group).enumerate() {
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
                let bin = offset + index;
                points.push(json!({"x":index as f64/values.len().saturating_sub(1).max(1) as f64,"y":1.0-power/maximum,"gap":chunk.iter().any(Option::is_none),"readout":format!("Bin {bin} · {:.3} MHz · power {power:.6} · {}",layout.frequency_hz(bin as u16).unwrap_or(0) as f64/1e6,role_name(roles[bin]))}));
            }
        } else {
            points.push(json!({"x":(bucket*group) as f64/values.len().saturating_sub(1).max(1) as f64,"y":null,"readout":"Unavailable captured bins"}));
        }
    }
    let mut bands = Vec::new();
    let mut start = 0;
    let mut omitted = 0;
    for end in 1..=values.len() {
        if end == values.len() || roles[offset + end] != roles[offset + start] {
            if bands.len() < 1024 {
                bands.push(json!({"x":start as f64/values.len() as f64,"end_x":end as f64/values.len() as f64,"role":role_name(roles[offset+start])}));
            } else {
                omitted += 1;
            }
            start = end;
        }
    }
    json!({"kind":"physical","points":points,"bands":bands,"omitted_bands":omitted,"maximum":maximum,"x_label":"Frequency (MHz)","y_label":"Bin power · relative","first":format!("{:.3}",layout.frequency_hz(offset as u16).unwrap_or(0) as f64/1e6),"last":format!("{:.3}",layout.frequency_hz((offset+values.len()-1) as u16).unwrap_or(0) as f64/1e6),"count":values.len(),"symbol":symbol})
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
    #[test]
    fn bins_preserve_peak_missing_power_and_layout_roles_within_bounds() {
        let transmission = docsis_ofdm_fec::transmission::TransmissionSettings::default();
        let layout = docsis_ofdm_fec::transmission::layout::CarrierLayoutSettings {
            zero_frequency_hz: 100_000_000,
            first_active_bin: 296,
            active_end_exclusive: 7896,
            plc_first_bin: 4112,
            excluded_bins: vec![6000],
            additional_continuous_pilots: (0..60)
                .map(|offset| 480 + 120 * offset)
                .filter(|bin| !(4000..4240).contains(bin) && *bin != 6000)
                .take(48)
                .collect(),
        };
        let settings = ChannelRunSettings::new(transmission, layout);
        let values = vec![
            json!([0.0, 0.0]);
            settings.transmission.subcarrier_spacing.fft_samples() as usize
        ];
        let mut values = values;
        values[17] = json!([3.0, 4.0]);
        values[18] = Value::Null;
        let view = project(&values, 0, Some(0), &settings);
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
        assert_eq!(project(&values, 0, None, &settings)["kind"], "empty");
    }
}
