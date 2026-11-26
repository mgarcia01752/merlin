/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Compatible unit-reference observations paired with canonical upstream geometry.
use super::{QamPoint, Tap, MAX_QAM_QUERY_POINTS};
use crate::{
    error::{invalid, limit},
    signal_processing::{map_symbol, reference_energy},
    simulation::BatchReport,
    types::Iq,
    PhyError, PhyResult,
};
use serde::{Deserialize, Serialize};

/// Ideal-reference application ceiling, independent of actual-point copying.
pub const MAX_QAM_CONSTELLATION_REFERENCES: usize = 4096;
/// Initial payload selector; not a recommended profile or IUC-to-modulation mapping.
const DEFAULT_QAM_PAYLOAD_LOADING: u8 = 4;

/// Actual stage/loading and independent observation/reference budgets.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QamConstellationRequest {
    /// TxGrid or RxEqualized; other raw planes reject a unit-reference overlay.
    pub tap: Tap,
    /// Original payload bits; zero selects known pilots with BPSK references.
    pub loading: u8,
    /// Positive actual-point copy limit, at most MAX_QAM_QUERY_POINTS.
    pub maximum_points: usize,
    /// Positive reference-point limit, at most MAX_QAM_CONSTELLATION_REFERENCES.
    pub maximum_references: usize,
}

/// Rust-owned normalization, separate from the unchanged original raw stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum QamNormalization {
    /// Original prescribed TX role gain removed using actual recorded Cell.gain.
    TxRoleGainRemoved,
    /// Receiver already removed channel estimate and role gain; values stay unchanged.
    ReceiverUnitReference,
}

/// Actual original point paired with its backend unit-reference value.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct QamReferenceSample {
    /// Unchanged actual point, including physical and run identities.
    pub observation: QamPoint,
    /// Actual divisor applied by this view; one for already equalized RX points.
    pub normalization_divisor: f64,
    /// Observed value divided by that divisor, never replaced by an ideal point.
    pub unit_value: Iq,
}

/// One original source-labelled unit-energy constellation point.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct QamReferencePoint {
    /// Original MSB-first label under upstream mapping conventions.
    pub label: u16,
    /// Exact upstream map_symbol result; no downstream/generic geometry substitution.
    pub value: Iq,
}

/// Bounded compatible QAM display preserving original observations and identities.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct QamConstellationSnapshot {
    /// Original actual stage.
    pub tap: Tap,
    /// Original payload loading; zero remains a known-pilot selection.
    pub payload_loading: u8,
    /// Reference label width; one for BPSK even with zero pilot payload bits.
    pub reference_bits: u8,
    /// Actual backend normalization, independent of screen transforms.
    pub normalization: QamNormalization,
    /// Explicit dimensionless unit-reference plane.
    pub units: &'static str,
    /// MSB-first label convention.
    pub bit_order: &'static str,
    /// Whole retained available matching count, independent of display cap.
    pub retained_matching_points: usize,
    /// True when further retained matches are omitted by the actual-point cap.
    pub capped: bool,
    /// Original observations paired with backend-derived unit-reference values.
    pub points: Vec<QamReferenceSample>,
    /// Complete canonical labelled geometry at the supported selected width.
    pub references: Vec<QamReferencePoint>,
}

impl Default for QamConstellationRequest {
    /// Selects received 16-QAM with independent finite actual/reference copy budgets.
    /// # Returns
    /// Read-only selection; no source, receiver stage or RNG executes.
    fn default() -> Self {
        Self {
            tap: Tap::RxEqualized,
            loading: DEFAULT_QAM_PAYLOAD_LOADING,
            maximum_points: MAX_QAM_QUERY_POINTS,
            maximum_references: MAX_QAM_CONSTELLATION_REFERENCES,
        }
    }
}

impl BatchReport {
    /// Projects compatible actual QAM to unit reference with canonical upstream labels.
    /// # Arguments
    /// * `request` - Original stage/loading and independent positive copy ceilings.
    /// # Returns
    /// Owned original points, normalized pairs and complete labelled references. TX uses its
    /// recorded role gain; RX equalized values remain unchanged. No science or RNG replays.
    /// # Errors
    /// Rejects unsupported raw planes/orders, invalid/insufficient budgets, unavailable history
    /// and inconsistent/nonfinite retained metadata. Configured C/H never substitutes for a plane.
    pub fn qam_constellation(
        &self,
        request: QamConstellationRequest,
    ) -> PhyResult<QamConstellationSnapshot> {
        let normalization = match request.tap {
            Tap::TxGrid => QamNormalization::TxRoleGainRemoved,
            Tap::RxEqualized => QamNormalization::ReceiverUnitReference,
            _ => {
                return Err(PhyError::Unsupported {
                    field: "qam_constellation.reference_plane",
                });
            }
        };
        if !(1..=MAX_QAM_QUERY_POINTS).contains(&request.maximum_points)
            || !(1..=MAX_QAM_CONSTELLATION_REFERENCES).contains(&request.maximum_references)
        {
            return Err(limit("qam_constellation.query_budget"));
        }
        let bits = match request.loading {
            0 => 1,
            bits => bits,
        };
        reference_energy(bits)?;
        let order = 1_usize << bits;
        if order > request.maximum_references {
            return Err(limit("qam_constellation.reference_points"));
        }
        let trace = self
            .observations
            .trace(request.tap)
            .ok_or(PhyError::Unsupported {
                field: "qam_constellation.unretained_tap",
            })?;
        if trace.role_gains.len() != trace.points.len() {
            return Err(invalid("qam_constellation.role_metadata"));
        }
        let retained_matching_points = trace
            .points
            .iter()
            .filter(|point| point.loading == request.loading)
            .count();
        if retained_matching_points == 0 {
            return Err(PhyError::Unsupported {
                field: "qam_constellation.unavailable",
            });
        }
        let points = trace
            .points
            .iter()
            .zip(&trace.role_gains)
            .filter(|(point, _)| point.loading == request.loading)
            .take(request.maximum_points)
            .map(|(point, role_gain)| {
                let divisor = match normalization {
                    QamNormalization::TxRoleGainRemoved => *role_gain,
                    QamNormalization::ReceiverUnitReference => 1.0,
                };
                if !divisor.is_finite() || divisor <= 0.0 {
                    return Err(invalid("qam_constellation.role_gain"));
                }
                let unit_value = point.value * divisor.recip();
                if !point.value.is_finite() || !unit_value.is_finite() {
                    return Err(invalid("qam_constellation.observation"));
                }
                Ok(QamReferenceSample {
                    observation: point.clone(),
                    normalization_divisor: divisor,
                    unit_value,
                })
            })
            .collect::<PhyResult<Vec<_>>>()?;
        let references = (0..order)
            .map(|label| {
                let label = label as u16;
                Ok(QamReferencePoint {
                    label,
                    value: map_symbol(bits, label)?,
                })
            })
            .collect::<PhyResult<Vec<_>>>()?;
        Ok(QamConstellationSnapshot {
            tap: request.tap,
            payload_loading: request.loading,
            reference_bits: bits,
            normalization,
            units: "dimensionless_unit_reference_constellation",
            bit_order: "msb_first",
            retained_matching_points,
            capped: points.len() < retained_matching_points,
            points,
            references,
        })
    }
}
