/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 §7.5.15.2.2, pp.169–171: additional continuous-pilot placement.
use super::{CarrierLayout, CarrierLayoutSettings, PhysicalCarrierRole};
use crate::{
    error::invalid, transmission::TransmissionSettings, types::SubcarrierIdx, SimulationError,
};
use std::collections::BTreeSet;

const REFERENCE_SPAN_HZ: u64 = 190_000_000;
const EDGE_GUARD_HZ: u32 = 1_000_000;
const MAX_PILOTS: usize = 120;

/// Fixed CMTS pilot-placement controls; poor bins are not converted to exclusions.
#[derive(Debug, Clone)]
pub struct ContinuousPilotSettings {
    /// PHY density parameter M,48..120; the typical setting is48.
    pub density: u8,
    /// Known interference/poor physical bins which must not carry generated pilots.
    pub avoid_bins: Vec<SubcarrierIdx>,
    /// Reproducible placement perturbation; does not affect signal/noise RNG state.
    pub seed: u64,
}
impl Default for ContinuousPilotSettings {
    fn default() -> Self {
        Self {
            density: 48,
            avoid_bins: Vec::new(),
            seed: 1,
        }
    }
}

/// Generates an owned layout with additional continuous pilots, preserving inputs.
/// # Arguments
/// * `transmission` - Fixed validated channel numerology and occupied bandwidth.
/// * `layout` - Physical geometry; existing additional-pilot assignments are replaced.
/// * `settings` - Density, known poor bins and deterministic perturbation seed.
/// # Returns
/// A layout accepted by CarrierLayout, with8..120 additional pilots and isolated
/// active-region coverage. The caller must transmit its positions through OCD/PLC.
/// # Errors
/// Rejects invalid geometry/density/avoid bins or spectrum unable to meet pilot
/// count, edge-distance and region-coverage constraints simultaneously.
pub fn generate_continuous_pilots(
    transmission: &TransmissionSettings,
    layout: &CarrierLayoutSettings,
    settings: &ContinuousPilotSettings,
) -> Result<CarrierLayoutSettings, SimulationError> {
    if !(48..=120).contains(&settings.density) || settings.avoid_bins.len() > 8192 {
        return Err(invalid(
            "Pilot density must be48..120 and avoid bins bounded.",
        ));
    }
    let base = CarrierLayout::planning_base(transmission, layout)?;
    let first = usize::from(layout.first_active_bin);
    let end = usize::from(layout.active_end_exclusive);
    let avoid: BTreeSet<_> = settings.avoid_bins.iter().copied().collect();
    if avoid.len() != settings.avoid_bins.len()
        || avoid
            .iter()
            .any(|bin| !(first..end).contains(&usize::from(*bin)))
    {
        return Err(invalid(
            "Poor pilot bins must be unique and within the active envelope.",
        ));
    }
    let plc = base.plc_neighborhood_hz()?;
    let eligible: Vec<u16> = (first..end)
        .filter(|&bin| {
            base.static_roles()[bin] == PhysicalCarrierRole::Data
                && !avoid.contains(&(bin as u16))
                && !plc.contains(&base.frequency_hz(bin as u16).expect("validated FFT bin"))
        })
        .map(|bin| bin as u16)
        .collect();
    if eligible.len() < 8 {
        return Err(invalid("Too few eligible continuous-pilot locations."));
    }
    // Spectral regions are separated by actual exclusions, not by the occupied PLC.
    let mut regions = Vec::new();
    let mut start = first;
    while start < end {
        if base.static_roles()[start] == PhysicalCarrierRole::Excluded {
            start += 1;
            continue;
        }
        let mut stop = start + 1;
        while stop < end && base.static_roles()[stop] != PhysicalCarrierRole::Excluded {
            stop += 1;
        }
        regions.push((start as u16, (stop - 1) as u16));
        start = stop;
    }
    let guard = (EDGE_GUARD_HZ / transmission.subcarrier_spacing.frequency_hz()) as u16;
    let mut allowed_regions = Vec::new();
    for &(low, high) in &regions {
        let predefined_coverage = base.static_roles()[usize::from(low)..=usize::from(high)]
            .contains(&PhysicalCarrierRole::ContinuousPilot);
        if high - low < 2 * guard {
            // A narrow isolated region requires its center, without jitter.
            let center = low + (high - low) / 2;
            if !eligible.contains(&center) {
                return Err(invalid(
                    "Narrow region center is unavailable for a continuous pilot.",
                ));
            }
            allowed_regions.push(vec![center]);
        } else {
            let allowed: Vec<_> = eligible
                .iter()
                .copied()
                .filter(|bin| *bin >= low + guard && *bin <= high - guard)
                .collect();
            if allowed.is_empty() && !predefined_coverage {
                return Err(invalid(
                    "Active region has no edge-safe continuous-pilot location.",
                ));
            }
            allowed_regions.push(allowed);
        }
    }
    let span_hz = u64::from(regions.last().expect("validated active spectrum").1 - regions[0].0)
        * u64::from(transmission.subcarrier_spacing.frequency_hz());
    let count = (u64::from(settings.density) * span_hz)
        .div_ceil(REFERENCE_SPAN_HZ)
        .clamp(8, 120) as usize;
    for initial_count in (1..=count).rev() {
        let mut selected = BTreeSet::new();
        for index in 0..initial_count {
            let rank =
                ((2 * index + 1) * (eligible.len() - 1) + initial_count) / (2 * initial_count);
            let desired = eligible[rank];
            let region = regions
                .iter()
                .position(|&(low, high)| (low..=high).contains(&desired))
                .expect("eligible region");
            if let Some(&bin) = allowed_regions[region]
                .iter()
                .filter(|bin| !selected.contains(*bin))
                .min_by_key(|bin| bin.abs_diff(desired))
            {
                selected.insert(bin);
            }
        }
        for (region, &(low, high)) in allowed_regions.iter().zip(&regions) {
            let predefined_coverage = base.static_roles()[usize::from(low)..=usize::from(high)]
                .contains(&PhysicalCarrierRole::ContinuousPilot);
            if !predefined_coverage && !selected.iter().any(|bin| (low..=high).contains(bin)) {
                let center = low + (high - low) / 2;
                selected.insert(
                    *region
                        .iter()
                        .min_by_key(|bin| bin.abs_diff(center))
                        .expect("nonempty region"),
                );
            }
        }
        if selected.len() > MAX_PILOTS {
            continue;
        }
        // If edge relocation merged candidates, refill at the largest uncovered gap.
        let allowed: Vec<_> = allowed_regions.iter().flatten().copied().collect();
        while selected.len() < initial_count.max(8) {
            let Some(&bin) = allowed
                .iter()
                .filter(|bin| !selected.contains(*bin))
                .max_by_key(|bin| {
                    selected
                        .iter()
                        .map(|pilot| pilot.abs_diff(**bin))
                        .min()
                        .unwrap_or(u16::MAX)
                })
            else {
                return Err(invalid("Insufficient distinct edge-safe pilot locations."));
            };
            selected.insert(bin);
        }
        // Step7's allowed ±5-bin disturbance. Seeded integer mixing is a placement
        // choice, not a prescribed PHY random sequence or an impairment generator.
        let original: Vec<_> = selected.iter().copied().collect();
        for (index, &bin) in original.iter().enumerate() {
            let region = regions
                .iter()
                .position(|&(low, high)| (low..=high).contains(&bin))
                .expect("pilot region");
            let mixed = settings
                .seed
                .wrapping_add(index as u64)
                .wrapping_add(0x9e3779b97f4a7c15);
            let mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            let offset = ((mixed ^ (mixed >> 27)) % 11) as i32 - 5;
            let target = i32::from(bin) + offset;
            if let Some(&candidate) = allowed_regions[region]
                .iter()
                .filter(|candidate| {
                    candidate.abs_diff(bin) <= 5
                        && (!selected.contains(*candidate) || **candidate == bin)
                })
                .min_by_key(|candidate| (i32::from(**candidate) - target).abs())
            {
                selected.remove(&bin);
                selected.insert(candidate);
            }
        }
        if repeating_gaps(&selected) {
            let mut disturbed = None;
            'search: for &bin in &selected {
                for &candidate in &allowed {
                    if candidate.abs_diff(bin) > 5 || selected.contains(&candidate) {
                        continue;
                    }
                    let region = regions
                        .iter()
                        .position(|&(low, high)| (low..=high).contains(&bin))
                        .expect("pilot region");
                    if !allowed_regions[region].contains(&candidate) {
                        continue;
                    }
                    let mut alternative = selected.clone();
                    alternative.remove(&bin);
                    alternative.insert(candidate);
                    if !repeating_gaps(&alternative) {
                        disturbed = Some(alternative);
                        break 'search;
                    }
                }
            }
            selected = disturbed.ok_or_else(|| {
                invalid("Eligible spectrum cannot disturb the periodic continuous-pilot pattern.")
            })?;
        }
        let mut generated = layout.clone();
        generated.additional_continuous_pilots = selected.into_iter().collect();
        CarrierLayout::new(transmission, &generated)?;
        return Ok(generated);
    }
    Err(invalid(
        "Isolated regions require more than120 continuous pilots.",
    ))
}

// A finite exact repeating gap pattern requires at least two cycles. Seeded
// disturbance is applied first even to patterns made nonuniform by exclusions.
fn repeating_gaps(pilots: &BTreeSet<u16>) -> bool {
    let bins: Vec<_> = pilots.iter().copied().collect();
    let gaps: Vec<_> = bins.windows(2).map(|pair| pair[1] - pair[0]).collect();
    (1..=gaps.len() / 2).any(|period| {
        gaps.iter()
            .enumerate()
            .all(|(index, gap)| *gap == gaps[index % period])
    })
}
