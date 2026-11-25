/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Reusable spectrum bandwidth calculations without file-format dependencies.

/// Bandwidth in Hz, retaining fractional-Hz precision.
pub type BandwidthHz = f64;
/// Equivalent noise bandwidth divided by bin spacing (dimensionless).
pub type NoiseBandwidthFactor = f64;

/// Calculates the equivalent noise bandwidth of a spectrum resolution filter.
///
/// # Arguments
/// * `bin_spacing_hz` - Positive, finite frequency separation between bins.
/// * `noise_bandwidth_factor` - Positive, finite normalized ENBW multiplier.
/// # Returns
/// RBW in Hz, or None for invalid inputs, overflow or underflow to zero.
/// The caller owns conversion from any protocol-specific encoded factor.
pub fn resolution_bandwidth_hz(
    bin_spacing_hz: BandwidthHz,
    noise_bandwidth_factor: NoiseBandwidthFactor,
) -> Option<BandwidthHz> {
    if !bin_spacing_hz.is_finite()
        || bin_spacing_hz <= 0.0
        || !noise_bandwidth_factor.is_finite()
        || noise_bandwidth_factor <= 0.0
    {
        return None;
    }
    let bandwidth = bin_spacing_hz * noise_bandwidth_factor;
    if bandwidth.is_finite() && bandwidth > 0.0 {
        Some(bandwidth)
    } else {
        None
    }
}

/// Averages spectrum bins in linear power and returns the same-length dBmV trace.
/// # Arguments
/// * `levels_dbmv` - Original bin levels; absent/non-finite values are gaps.
/// * `window_points` - Positive centered reflected window length in source bins.
/// # Returns
/// `10 log10(mean(10^(level/10)))` per bin, or missing values where a window
/// contains invalid/overflowing/underflowing input. No zero-power substitution.
/// None indicates an invalid window size or unsupported indexing. This is
/// averaging across frequency, not channel-power integration or an RBW change.
pub fn window_average_dbmv(
    levels_dbmv: &[Option<f64>],
    window_points: super::moving_average::WindowPointCount,
) -> Option<Vec<Option<f64>>> {
    let powers: Vec<_> = levels_dbmv
        .iter()
        .map(|level| {
            level.filter(|level| level.is_finite()).and_then(|level| {
                let power = 10.0_f64.powf(level / 10.0);
                (power.is_finite() && power > 0.0).then_some(power)
            })
        })
        .collect();
    Some(
        super::moving_average::reflected_window_average(&powers, window_points)?
            .into_iter()
            .map(|power| {
                power.and_then(|power| {
                    let level = 10.0 * power.log10();
                    (power > 0.0 && level.is_finite()).then_some(level)
                })
            })
            .collect(),
    )
}

/// Averages compatible levels in linear mean-square voltage, returning dBmV.
/// # Arguments
/// * `levels` - Finite dBmV readings sharing one acquisition/reference convention.
/// # Returns
/// Equal-weight 10 log10(mean(10^(L/10))), or None for empty/nonfinite input.
/// Maximum-relative summation avoids overflow for extreme levels. Impedance
/// cancels for a shared reference; this does not change acquisition RBW.
pub fn mean_level_dbmv(levels: &[f64]) -> Option<f64> {
    if levels.is_empty() || levels.iter().any(|level| !level.is_finite()) {
        return None;
    }
    let maximum = levels.iter().copied().max_by(f64::total_cmp)?;
    let mean = levels
        .iter()
        .map(|level| 10.0_f64.powf((level - maximum) / 10.0))
        .sum::<f64>()
        / levels.len() as f64;
    let result = maximum + 10.0 * mean.log10();
    result.is_finite().then_some(result)
}
