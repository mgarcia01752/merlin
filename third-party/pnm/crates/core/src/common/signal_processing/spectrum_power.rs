/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! ENBW-corrected integration of mean-square voltage, expressed in dBmV.
use super::spectrum::BandwidthHz;

/// Requested interval in Hz; endpoints may lie between source bin centers.
#[derive(Clone, Copy, Debug)]
pub struct SpectrumRange {
    pub start_hz: BandwidthHz,
    pub stop_hz: BandwidthHz,
}

/// Integrates original spectrum bins using their fractional overlap with a band.
///
/// # Arguments
/// * `levels_dbmv` - Uniformly spaced source bins; missing/nonfinite bins in the
///   selected band invalidate the result rather than contributing zero power.
/// * `first_center_hz` - Exact center of the first source bin, in Hz.
/// * `bin_spacing_hz` - Positive finite width of each integration cell, in Hz.
/// * `noise_bandwidth_hz` - Positive finite acquisition ENBW, in Hz.
/// * `range` - Ordered, finite band inside the source bin-cell coverage.
/// # Returns
/// Integrated dBmV: 10 log10(sum(10^(L/10) * overlap_hz / ENBW_hz)),
/// or None for invalid bounds, gaps, unsupported grids or nonfinite results.
/// This sums mean-square voltage, not coherent voltage amplitudes. Fixed
/// impedance cancels; no 50/75-ohm conversion is applied to dBmV.
pub fn integrated_level_dbmv(
    levels_dbmv: &[Option<f64>],
    first_center_hz: BandwidthHz,
    bin_spacing_hz: BandwidthHz,
    noise_bandwidth_hz: BandwidthHz,
    range: SpectrumRange,
) -> Option<f64> {
    if levels_dbmv.len() < 2
        || !first_center_hz.is_finite()
        || first_center_hz < 0.0
        || !bin_spacing_hz.is_finite()
        || bin_spacing_hz <= 0.0
        || !noise_bandwidth_hz.is_finite()
        || noise_bandwidth_hz <= 0.0
        || !range.start_hz.is_finite()
        || !range.stop_hz.is_finite()
    {
        return None;
    }
    let last_center_hz = first_center_hz + (levels_dbmv.len() - 1) as f64 * bin_spacing_hz;
    if !last_center_hz.is_finite()
        || range.start_hz < first_center_hz - bin_spacing_hz / 2.0
        || range.stop_hz > last_center_hz + bin_spacing_hz / 2.0
        || range.start_hz >= range.stop_hz
    {
        return None;
    }
    let mut maximum = f64::NEG_INFINITY;
    let mut weighted = Vec::new();
    for (index, level) in levels_dbmv.iter().enumerate() {
        let center = first_center_hz + index as f64 * bin_spacing_hz;
        let overlap = (range.stop_hz.min(center + bin_spacing_hz / 2.0)
            - range.start_hz.max(center - bin_spacing_hz / 2.0))
        .max(0.0);
        if overlap == 0.0 {
            continue;
        }
        let level = level.filter(|level| level.is_finite())?;
        // Add logarithms instead of forming extreme linear powers or ratios.
        let contribution = level + 10.0 * (overlap.log10() - noise_bandwidth_hz.log10());
        if !contribution.is_finite() {
            return None;
        }
        maximum = maximum.max(contribution);
        weighted.push(contribution);
    }
    if weighted.is_empty() {
        return None;
    }
    let sum = weighted
        .iter()
        .map(|level| 10.0_f64.powf((level - maximum) / 10.0))
        .sum::<f64>();
    let result = maximum + 10.0 * sum.log10();
    result.is_finite().then_some(result)
}
