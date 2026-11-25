/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Full-capture decision-directed MER in decoded amplitude coordinates.
use crate::pnm::{data_type::IqPair, parser::cm_ds_const_disp_meas::CmDsConstDispMeas};
use serde::Serialize;

/// Availability and exceptional conditions; JSON never encodes NaN or infinity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MerStatus {
    Available,
    PerfectReference,
    NoSamples,
    InvalidSamples,
    SampleCountMismatch,
    ZeroSignalPower,
    NonFiniteResult,
    UnsupportedModulation,
}

/// Power sums use every received sample and its corresponding reference symbol.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct MerResult {
    pub mer_db: Option<f64>,
    /// Number of supplied soft samples, including invalid entries.
    pub sample_count: usize,
    pub signal_power: Option<f64>,
    pub error_power: Option<f64>,
    pub rms_evm_percent: Option<f64>,
    pub calculation_status: MerStatus,
}
impl MerResult {
    /// Describes an unavailable measurement without inventing finite powers.
    /// # Arguments
    /// * `sample_count` - Original soft-sample count, including invalid entries.
    /// * `status` - Reason the measurement cannot be calculated.
    /// # Returns
    /// A structured result with null numeric measurements.
    pub fn unavailable(sample_count: usize, status: MerStatus) -> Self {
        Self {
            mer_db: None,
            sample_count,
            signal_power: None,
            error_power: None,
            rms_evm_percent: None,
            calculation_status: status,
        }
    }
}

#[derive(Default)]
struct Powers {
    sample_count: usize,
    signal: f64,
    error: f64,
    invalid: bool,
}
impl Powers {
    fn push(&mut self, soft: Option<IqPair>, hard: Option<IqPair>) {
        self.sample_count += 1;
        let (Some(soft), Some(hard)) = (soft, hard) else {
            self.invalid = true;
            return;
        };
        if !soft.into_iter().chain(hard).all(f64::is_finite) {
            self.invalid = true;
            return;
        }
        self.signal += hard[0].powi(2) + hard[1].powi(2);
        self.error += (soft[0] - hard[0]).powi(2) + (soft[1] - hard[1]).powi(2);
    }
    fn finish(self, expected_count: usize) -> MerResult {
        let status = if self.sample_count != expected_count {
            Some(MerStatus::SampleCountMismatch)
        } else if self.sample_count == 0 {
            Some(MerStatus::NoSamples)
        } else if self.invalid {
            Some(MerStatus::InvalidSamples)
        } else if !self.signal.is_finite() || !self.error.is_finite() {
            Some(MerStatus::NonFiniteResult)
        } else {
            None
        };
        if let Some(status) = status {
            return MerResult::unavailable(self.sample_count, status);
        }
        let mut result = MerResult {
            signal_power: Some(self.signal),
            error_power: Some(self.error),
            ..MerResult::unavailable(self.sample_count, MerStatus::ZeroSignalPower)
        };
        if self.signal == 0.0 {
            return result;
        }
        if self.error == 0.0 {
            result.calculation_status = MerStatus::PerfectReference;
            result.rms_evm_percent = Some(0.0);
            return result;
        }
        // Log differences avoid overflowing the intermediate power ratio.
        let mer_db = 10.0 * (self.signal.log10() - self.error.log10());
        let rms_evm_percent = self.error.sqrt() / self.signal.sqrt() * 100.0;
        if !mer_db.is_finite() || !rms_evm_percent.is_finite() {
            result.calculation_status = MerStatus::NonFiniteResult;
            return result;
        }
        result.mer_db = Some(mer_db);
        result.rms_evm_percent = Some(rms_evm_percent);
        result.calculation_status = MerStatus::Available;
        result
    }
}

/// Calculates MER from explicitly paired decoded soft and reference coordinates.
/// # Arguments
/// * `soft` - Received I/Q amplitudes; no rescaling, clipping or rejection.
/// * `hard` - One corresponding ideal I/Q symbol per soft sample, in order.
/// # Returns
/// f64 power sums, MER and RMS EVM, or a structured exceptional status.
/// Zero error is perfect_reference with null MER rather than a finite substitute.
/// Mismatched lengths and invalid samples invalidate the entire measurement.
pub fn calculate_mer(soft: &[IqPair], hard: &[IqPair]) -> MerResult {
    if soft.len() != hard.len() {
        return MerResult::unavailable(soft.len(), MerStatus::SampleCountMismatch);
    }
    let mut powers = Powers::default();
    for (&soft, &hard) in soft.iter().zip(hard) {
        powers.push(Some(soft), Some(hard));
    }
    powers.finish(soft.len())
}

// Index actual reference rows rather than assuming a square constellation or
// fixed symbol spacing. Cross-QAM rows have different lengths/missing corners.
struct ReferenceRow {
    quadrature: f64,
    in_phase: Vec<f64>,
}
struct Slicer {
    rows: Vec<ReferenceRow>,
}
impl Slicer {
    fn new(reference: &[IqPair]) -> Self {
        let mut points = reference.to_vec();
        points.sort_by(|first, second| {
            first[1]
                .total_cmp(&second[1])
                .then(first[0].total_cmp(&second[0]))
        });
        let mut rows: Vec<ReferenceRow> = Vec::new();
        for point in points {
            if rows.last().is_none_or(|row| row.quadrature != point[1]) {
                rows.push(ReferenceRow {
                    quadrature: point[1],
                    in_phase: Vec::new(),
                });
            }
            rows.last_mut().unwrap().in_phase.push(point[0]);
        }
        Self { rows }
    }
    fn nearest(&self, sample: IqPair) -> IqPair {
        let mut nearest = self
            .rows
            .first()
            .map(|row| [row.in_phase[0], row.quadrature])
            .unwrap();
        let mut best_squared_distance = f64::INFINITY;
        for row in &self.rows {
            let quadrature_error = (sample[1] - row.quadrature).powi(2);
            if quadrature_error > best_squared_distance {
                continue;
            }
            let index = row
                .in_phase
                .partition_point(|coordinate| *coordinate < sample[0]);
            for candidate in [index.saturating_sub(1), index.min(row.in_phase.len() - 1)] {
                let squared_distance =
                    (sample[0] - row.in_phase[candidate]).powi(2) + quadrature_error;
                if squared_distance < best_squared_distance {
                    best_squared_distance = squared_distance;
                    nearest = [row.in_phase[candidate], row.quadrature];
                }
            }
        }
        nearest
    }
}

/// Estimates decision-directed constellation MER using all original samples.
/// # Arguments
/// * `capture` - Parsed signed s2.13 amplitudes, already divided by 8192 once.
/// # Returns
/// A nearest-Euclidean-symbol estimate using the embedded normalized QAM table.
/// Header/payload count disagreement and unknown modulation remain unavailable.
/// No viewport, display sampling, visibility state or modem RxMER is involved.
/// Incorrect decisions under severe noise can make this estimate optimistic.
pub fn from_capture(capture: &CmDsConstDispMeas<'_>) -> MerResult {
    let Some(reference) = capture.constellation_analysis.reference else {
        return MerResult::unavailable(capture.iq().count(), MerStatus::UnsupportedModulation);
    };
    // Public model fields can be modified by library callers; do not let an
    // empty/non-finite replacement reference panic or silently discard symbols.
    if reference.iq.is_empty() || !reference.iq.iter().flatten().all(|value| value.is_finite()) {
        return MerResult::unavailable(capture.iq().count(), MerStatus::InvalidSamples);
    }
    let slicer = Slicer::new(reference.iq);
    let mut powers = Powers::default();
    for soft in capture.iq() {
        let hard = soft
            .filter(|point| point.iter().all(|value| value.is_finite()))
            .map(|point| slicer.nearest(point));
        powers.push(soft, hard);
    }
    powers.finish(usize::from(capture.num_sample_symbols))
}

#[cfg(test)]
mod tests {
    use super::{MerStatus, Powers};

    #[test]
    fn missing_pair_invalidates_the_entire_measurement_without_filtering() {
        let mut powers = Powers::default();
        powers.push(Some([1.0, 0.01]), Some([1.0, 0.0]));
        powers.push(None, Some([1.0, 0.0]));
        let result = powers.finish(2);
        assert_eq!(result.sample_count, 2);
        assert_eq!(result.calculation_status, MerStatus::InvalidSamples);
        assert_eq!(result.mer_db, None);
        assert_eq!(result.signal_power, None);
    }
}
