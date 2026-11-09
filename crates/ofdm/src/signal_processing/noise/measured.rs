/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Residual-noise selection without interpolation or channel-gain double counting.
use crate::{
    error::{invalid, SimulationError},
    profile::candidate::CandidateProfile,
    scenario::RxMerFrame,
    types::{modulation::CarrierType, ProtocolScope},
};
/// Measured-noise selection. Unknown/missing data readings are never imputed.
pub struct MeasuredNoise<'capture> {
    pub frame: &'capture RxMerFrame,
    pub candidate: &'capture CandidateProfile,
    pub first: usize,
    pub count: usize,
}
/// Selects controlled AWGN or exact-frame residual powers without filling gaps.
/// # Arguments
/// * `awgn_mer_db` - Validated reference MER for a controlled baseline.
/// * `measured` - Optional exact-grid measured selection.
/// # Returns
/// Complex noise powers and source offsets for selected data carriers.
/// # Errors
/// Rejects incompatible, missing or unsupported measured assignments.
pub(crate) fn noise_model(
    awgn_mer_db: f64,
    measured: Option<&MeasuredNoise<'_>>,
) -> Result<(Vec<f64>, Vec<usize>), SimulationError> {
    let Some(measured) = measured else {
        return Ok((vec![10.0_f64.powf(-awgn_mer_db / 10.0)], vec![]));
    };
    if measured.candidate.protocol != ProtocolScope::Docsis31 {
        return Err(SimulationError::new("UnsupportedCodingMode", "The first coding experiment is scoped to DOCSIS3.1; DOCSIS4.0 placement is not verified."));
    }
    measured.frame.validate()?;
    measured.candidate.validate()?;
    if measured.frame.key != measured.candidate.key {
        return Err(invalid("Candidate and measured RF grids differ."));
    }
    let end = measured
        .first
        .checked_add(measured.count)
        .filter(|end| measured.count > 0 && *end <= measured.frame.mer_db.len())
        .ok_or_else(|| invalid("Simulation selection is outside the source grid."))?;
    let mut powers = Vec::new();
    let mut offsets = Vec::new();
    for offset in measured.first..end {
        let assignment = &measured.candidate.assignments[offset];
        match assignment.carrier_type {
            CarrierType::Data => {
                if assignment.bit_loading != Some(4) {
                    return Err(SimulationError::new(
                        "UnsupportedCodingMode",
                        "This coding slice supports uniform16-QAM data assignments only.",
                    ));
                }
                let mer = measured.frame.mer_db[offset].ok_or_else(|| {
                    invalid("A selected data carrier has no measured residual-noise model.")
                })?;
                let power = 10.0_f64.powf(-mer / 10.0);
                if !power.is_finite() || power <= 0.0 {
                    return Err(invalid(
                        "Measured MER is outside the supported numeric noise range.",
                    ));
                }
                powers.push(power);
                offsets.push(offset);
            }
            CarrierType::Unknown | CarrierType::Reserved => {
                return Err(invalid(
                    "Resolve unknown/reserved roles before detailed simulation.",
                ));
            }
            _ => {}
        }
    }
    if powers.is_empty() {
        return Err(invalid("No data carriers are selected for simulation."));
    }
    Ok((powers, offsets))
}
