/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Diagnostic nearest-symbol decisions; the FEC receiver continues to use soft bits.
use crate::{
    error::invalid,
    signal_processing::modulation::{reference_for_order, SymbolLabel},
    types::{ofdm_modulation::ConstellationOrder, IqPair},
    SimulationError,
};
/// A hard decision in the reference's unit-average-energy coordinate system.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct HardDecision {
    pub label: SymbolLabel,
    pub reference_iq: IqPair,
    /// Received minus decided reference, in normalized I/Q units.
    pub error_iq: IqPair,
    pub distance_squared: f64,
}
/// Selects the nearest stored QAM point without changing received coordinates.
/// # Arguments
/// * `order` - Supported physical constellation size, including control references.
/// * `received` - Finite equalized unit-energy I/Q; no normalization is inferred.
/// # Returns
/// Lowest label among exactly equal computed squared distances, plus reference/error.
/// Work is bounded by the selected constellation (at most16384 points).
/// # Errors
/// Rejects unsupported orders, nonfinite samples or overflowing distance metrics.
pub fn slice_symbol(
    order: ConstellationOrder,
    received: IqPair,
) -> Result<HardDecision, SimulationError> {
    let reference = reference_for_order(usize::from(order)).ok_or_else(|| {
        SimulationError::new("UnsupportedModulation", "Unsupported QAM constellation.")
    })?;
    if received.iter().any(|value| !value.is_finite()) {
        return Err(invalid("Hard slicing requires finite I/Q."));
    }
    let mut best = None;
    for (label, point) in reference.iq.iter().enumerate() {
        let error_iq = [received[0] - point[0], received[1] - point[1]];
        let distance_squared = error_iq[0].powi(2) + error_iq[1].powi(2);
        if !distance_squared.is_finite() {
            return Err(invalid("I/Q distance exceeds finite slicing range."));
        }
        if best
            .as_ref()
            .is_none_or(|prior: &HardDecision| distance_squared < prior.distance_squared)
        {
            best = Some(HardDecision {
                label: label as SymbolLabel,
                reference_iq: *point,
                error_iq,
                distance_squared,
            });
        }
    }
    best.ok_or_else(|| invalid("Empty constellation reference."))
}
