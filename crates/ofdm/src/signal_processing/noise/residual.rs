/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Equivalent MER from averaged residual error power.
/// Converts an already normalized residual error power to equivalent MER.
///
/// # Arguments
/// * `power` - Dimensionless error/reference power ratio, independent of original epochs.
///
/// # Returns
/// Finite dB value, or None for zero, negative, nonfinite or unrepresentable power.
pub fn residual_power_mer(power: f64) -> Option<f64> {
    let mer = -10.0 * power.log10();
    mer.is_finite().then_some(mer)
}

/// Computes equivalent residual-noise MER by averaging linear error powers.
/// # Arguments
/// * `values_db` - Finite per-observation MER values at the same reference point.
/// # Returns
/// Equivalent MER in dB, or None for empty/nonfinite/extreme numeric inputs.
pub fn equivalent_noise_mer(values_db: &[f64]) -> Option<f64> {
    if values_db.is_empty() || values_db.iter().any(|value| !value.is_finite()) {
        return None;
    }
    let noise_power = values_db
        .iter()
        .map(|mer| 10.0_f64.powf(-mer / 10.0))
        .sum::<f64>()
        / values_db.len() as f64;
    let mer = -10.0 * noise_power.log10();
    mer.is_finite().then_some(mer)
}
