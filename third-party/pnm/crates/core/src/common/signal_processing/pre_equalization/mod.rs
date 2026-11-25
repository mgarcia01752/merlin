/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Ideal complex correction math; never simulates modem clipping/normalization.
use crate::pnm::data_type::IqPair;
use serde::{Deserialize, Serialize};

/// RNG-RSP intent must be supplied explicitly; PNM6/7 do not encode it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorrectionMode {
    Adjust,
    Set,
}
impl CorrectionMode {
    /// Resolves an explicit user-selected update interpretation.
    /// # Arguments
    /// * `name` - Stable lowercase mode identifier.
    /// # Returns
    /// Known mode or None, never an inferred default.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "adjust" => Some(Self::Adjust),
            "set" => Some(Self::Set),
            _ => None,
        }
    }
}
/// Applies an ideal correction in the complex domain before computing differences.
/// # Arguments
/// * `original`, `correction` - Decoded I/Q, not raw Q-format integers.
/// * `mode` - Explicit Adjust (multiply) or Set (replace) interpretation.
/// # Returns
/// Finite predicted I/Q or None for missing/nonfinite input. Zero is valid.
pub fn apply(
    original: Option<IqPair>,
    correction: Option<IqPair>,
    mode: CorrectionMode,
) -> Option<IqPair> {
    let [in_phase, quadrature] = original?;
    let [adjust_in_phase, adjust_quadrature] = correction?;
    if ![in_phase, quadrature, adjust_in_phase, adjust_quadrature]
        .iter()
        .all(|value| value.is_finite())
    {
        return None;
    }
    let predicted = match mode {
        CorrectionMode::Adjust => [
            in_phase * adjust_in_phase - quadrature * adjust_quadrature,
            in_phase * adjust_quadrature + quadrature * adjust_in_phase,
        ],
        CorrectionMode::Set => [adjust_in_phase, adjust_quadrature],
    };
    predicted
        .iter()
        .all(|value| value.is_finite())
        .then_some(predicted)
}
/// Computes the coefficient-power reference used only for labeled display alignment.
/// # Arguments
/// * `samples` - Matched decoded samples; zero contributes zero power.
/// # Returns
/// 10log10(mean(I²+Q²)), or None for empty, nonfinite or zero mean power.
pub fn reference_db(samples: &[Option<IqPair>]) -> Option<f64> {
    const POWER_DB_FACTOR: f64 = 10.0;
    let mut count = 0;
    let mut power = 0.0;
    for [in_phase, quadrature] in samples.iter().flatten() {
        power += in_phase * in_phase + quadrature * quadrature;
        count += 1;
    }
    if count == 0 {
        return None;
    }
    let reference = POWER_DB_FACTOR * (power / f64::from(count)).log10();
    reference.is_finite().then_some(reference)
}
