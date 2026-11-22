/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit known-amplitude diagnostic normalization delegates to the original physical slicer.
use crate::{
    error::invalid,
    signal_processing::modulation::{slice_symbol, slicing::HardDecision},
    types::IqPair,
    SimulationError,
};
use serde::Serialize;

/// Original-coordinate nearest decision plus its explicit unit-energy reference coordinates.
#[derive(Serialize)]
#[non_exhaustive]
pub struct ScaledDecision {
    pub normalized_iq: IqPair,
    pub normalized: HardDecision,
    pub reference_iq: IqPair,
    pub error_iq: IqPair,
    pub distance_squared: f64,
}

/// Normalizes using a known positive reference amplitude and applies the existing slicer.
/// # Arguments
/// * `order` - Existing physical reference order, including control references.
/// * `amplitude` - Explicit known reference multiplier; two for boosted pilots, otherwise one.
/// * `received` - Actual finite corrected I/Q; no fitted gain, phase or fabricated soft values.
/// # Returns
/// Owned normalized/physical decision diagnostics, never production FEC inputs.
/// # Errors
/// Rejects invalid amplitude, unsupported references, nonfinite values or numerical overflow.
pub(crate) fn decide(
    order: u16,
    amplitude: f64,
    received: IqPair,
) -> Result<ScaledDecision, SimulationError> {
    if !amplitude.is_finite() || amplitude <= 0.0 {
        return Err(invalid(
            "Diagnostic decision requires a finite positive reference amplitude",
        ));
    }
    let normalized_iq = received.map(|value| value / amplitude);
    let normalized = slice_symbol(order, normalized_iq)?;
    let reference_iq = normalized.reference_iq.map(|value| value * amplitude);
    let error_iq = [received[0] - reference_iq[0], received[1] - reference_iq[1]];
    let distance_squared = error_iq[0].mul_add(error_iq[0], error_iq[1] * error_iq[1]);
    if reference_iq
        .iter()
        .chain(&error_iq)
        .any(|value| !value.is_finite())
        || !distance_squared.is_finite()
    {
        return Err(invalid("Physical decision exceeds finite diagnostic range"));
    }
    Ok(ScaledDecision {
        normalized_iq,
        normalized,
        reference_iq,
        error_iq,
        distance_squared,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boosted_bpsk_decision_preserves_known_physical_and_unit_reference_coordinates() {
        // PHY Annex A BPSK label zero is +1; an explicit pilot multiplier is two.
        let decision = decide(2, 2.0, [2.1, 0.1]).unwrap();
        assert_eq!(decision.normalized.label, 0);
        assert_eq!(decision.normalized.reference_iq, [1.0, 0.0]);
        assert_eq!(decision.reference_iq, [2.0, 0.0]);
        assert_eq!(decision.normalized_iq, [1.05, 0.05]);
        assert!((decision.distance_squared - 0.02).abs() < 1e-14);
        assert!((decision.normalized.distance_squared - 0.005).abs() < 1e-14);
        assert!((decision.error_iq[0] - 0.1).abs() < 1e-14);
        assert_eq!(decision.error_iq[1], 0.1);
    }
    #[test]
    fn invalid_normalization_or_samples_never_become_finite_diagnostic_decisions() {
        for amplitude in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(decide(2, amplitude, [1.0, 0.0]).is_err());
        }
        assert!(decide(8, 1.0, [1.0, 0.0]).is_err());
        assert!(decide(2, 1.0, [f64::NAN, 0.0]).is_err());
        assert!(decide(2, 1.0, [f64::MAX, 0.0]).is_err());
    }
}
