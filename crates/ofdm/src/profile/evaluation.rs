/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Fast analytical screening, explicitly separate from simulated FEC reliability.
use super::{invalid, model::*, LabError};
use crate::{
    math::{bit_loading::BitLoadingModel, ofdm},
    types::{modulation::CarrierType, BitRateBps, BitsPerSymbol, CaptureTime},
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};

/// Bounds full-resolution temporal work independently of chart decimation.
pub const MAX_EVALUATION_SAMPLES: usize = 4_000_000;
/// Inspectable lower-tail selection, never a promise of whole-frame reliability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateStrategy {
    Conservative,
    Percentile,
    WeakIntervalAllowance,
}
/// Explicit interpretation/settings recorded with every evaluation/export.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvaluationSettings {
    pub model: BitLoadingModel,
    pub margin_db: f64,
    pub max_bit_loading: BitsPerSymbol,
    pub strategy: CandidateStrategy,
    pub lower_percentile: f64,
    pub allowed_weak_fraction: f64,
    pub assume_unknown_data_roles: bool,
    pub time_tolerance_seconds: u32,
}
impl Default for EvaluationSettings {
    /// Selects conservative OSSI targets, zero extra margin and exact-time matches.
    /// # Returns
    /// Session settings; unknown carrier roles require explicit user opt-in.
    fn default() -> Self {
        Self {
            model: BitLoadingModel::OssiCandidateDefaults,
            margin_db: 0.0,
            max_bit_loading: crate::types::ofdm_modulation::MAX_OFDM_BITS_PER_SYMBOL,
            strategy: CandidateStrategy::Conservative,
            lower_percentile: 0.05,
            allowed_weak_fraction: 0.05,
            assume_unknown_data_roles: false,
            time_tolerance_seconds: 0,
        }
    }
}
impl EvaluationSettings {
    /// Validates finite bounded policy settings before any candidate mutation.
    /// # Returns
    /// Unit for supported settings.
    /// # Errors
    /// Rejects nonfinite/out-of-range margins, quantiles and modulation caps.
    pub fn validate(&self) -> Result<(), LabError> {
        if !self.margin_db.is_finite()
            || !(0.0..=60.0).contains(&self.margin_db)
            || !self.lower_percentile.is_finite()
            || !(0.0..=1.0).contains(&self.lower_percentile)
            || !self.allowed_weak_fraction.is_finite()
            || !(0.0..=1.0).contains(&self.allowed_weak_fraction)
            || !crate::math::bit_loading::is_data_bit_loading(self.max_bit_loading)
        {
            return Err(invalid(
                "Invalid evaluation margin, modulation cap or temporal objective.",
            ));
        }
        Ok(())
    }
}
/// One full-resolution source frame's analytical outcome, not measured errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrameEvaluation {
    pub source_sha256: String,
    pub capture_time: CaptureTime,
    pub measured_data_carriers: usize,
    pub missing_data_carriers: usize,
    pub vulnerable_carriers: usize,
    pub unresolved_carriers: usize,
    pub worst_margin_db: Option<f64>,
    pub meets_target: Option<bool>,
}
/// Temporal screening retains every interval; an average cannot hide failures.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvaluationReport {
    pub result_kind: String,
    pub frames: Vec<FrameEvaluation>,
    pub complete_frame_count: usize,
    pub passing_frame_count: usize,
    pub fraction_all_frames_meeting_target: f64,
    pub worst_source_sha256: Option<String>,
    pub worst_interval_epoch: Option<CaptureTime>,
    pub proposed_data_bits_per_useful_symbol: u64,
    pub estimated_gross_rate_bps: Option<BitRateBps>,
    pub roles_verified: bool,
    pub excluded_overheads: Vec<String>,
    pub limitations: Vec<String>,
}

fn check_cancel(cancel: &AtomicBool) -> Result<(), LabError> {
    if cancel.load(Ordering::Relaxed) {
        return Err(LabError::new("Cancelled", "Laboratory analysis cancelled."));
    }
    Ok(())
}
fn validate_frames(frames: &[&RxMerFrame], key: &ScenarioKey) -> Result<(), LabError> {
    if frames.is_empty()
        || frames.len() > 4096
        || frames
            .len()
            .checked_mul(key.grid.sample_count)
            .is_none_or(|count| count > MAX_EVALUATION_SAMPLES)
    {
        return Err(invalid(
            "Temporal analysis requires a bounded nonempty capture set.",
        ));
    }
    for frame in frames {
        frame.validate()?;
        if &frame.key != key {
            return Err(invalid(
                "Capture device, channel or original frequency grid differs.",
            ));
        }
    }
    Ok(())
}

/// Generates a proposed per-carrier profile from individual measured frames.
/// # Arguments
/// * `frames` - Compatible full-resolution observations, never an average curve.
/// * `template` - Optional captured roles/profile; special roles are protected.
/// * `settings` - Explicit target, lower-tail policy and role-assumption consent.
/// * `protocol` - User-selected DOCSIS interpretation, not inferred from filename.
/// * `cancel` - Cooperative cancellation checked per carrier.
/// # Returns
/// A candidate retaining source provenance and unverifiable role status.
/// # Errors
/// Rejects incompatible grids, invalid limits/settings or cancellation.
pub fn generate_candidate(
    frames: &[&RxMerFrame],
    template: Option<&CapturedProfile>,
    settings: &EvaluationSettings,
    protocol: ProtocolScope,
    cancel: &AtomicBool,
) -> Result<CandidateProfile, LabError> {
    settings.validate()?;
    let first = frames
        .first()
        .ok_or_else(|| invalid("Load an RxMER capture first."))?;
    validate_frames(frames, &first.key)?;
    if template.is_some_and(|profile| {
        profile.key != first.key || profile.assignments.len() != first.mer_db.len()
    }) {
        return Err(invalid("Captured profile does not match the RxMER grid."));
    }
    let mut assignments = Vec::with_capacity(first.mer_db.len());
    for offset in 0..first.mer_db.len() {
        check_cancel(cancel)?;
        let mut assignment = template
            .map(|profile| profile.assignments[offset].clone())
            .unwrap_or(CarrierAssignment {
                carrier_type: CarrierType::Unknown,
                bit_loading: None,
                origin: AssignmentOrigin::Proposed,
            });
        if assignment.carrier_type == CarrierType::Unknown && settings.assume_unknown_data_roles {
            assignment.carrier_type = CarrierType::Data;
        }
        if assignment.carrier_type == CarrierType::Data {
            let mut observations: Vec<f64> = frames
                .iter()
                .filter_map(|frame| frame.mer_db[offset])
                .collect();
            observations.sort_by(f64::total_cmp);
            let quantile = match settings.strategy {
                CandidateStrategy::Conservative => 0.0,
                CandidateStrategy::Percentile => settings.lower_percentile,
                CandidateStrategy::WeakIntervalAllowance => settings.allowed_weak_fraction,
            };
            let selection =
                (quantile * observations.len().saturating_sub(1) as f64).floor() as usize;
            assignment.bit_loading = observations.get(selection).and_then(|mer| {
                settings
                    .model
                    .supported_bits(*mer, settings.margin_db, settings.max_bit_loading)
            });
            assignment.origin = AssignmentOrigin::Proposed;
        }
        assignments.push(assignment);
    }
    let mut sources: Vec<_> = frames.iter().map(|frame| frame.source.clone()).collect();
    if let Some(profile) = template {
        if !sources
            .iter()
            .any(|source| source.sha256 == profile.source.sha256)
        {
            sources.push(profile.source.clone());
        }
    }
    let roles_verified = template.is_some_and(|profile| {
        profile.assignments.iter().all(|assignment| {
            !matches!(
                assignment.carrier_type,
                CarrierType::Unknown | CarrierType::Reserved
            )
        })
    });
    let candidate = CandidateProfile {
        schema_version: CANDIDATE_SCHEMA_VERSION,
        name: "Proposed downstream profile".into(),
        protocol,
        key: first.key.clone(),
        profile_id: template.map(|profile| profile.profile_id),
        assignments,
        sources,
        roles_verified,
    };
    candidate.validate()?;
    Ok(candidate)
}

/// Evaluates every supplied frame against the candidate's actual assignments.
/// # Arguments
/// * `candidate` - Validated captured/proposed assignments on the exact grid.
/// * `frames` - Individual compatible source frames; no interpolation/averaging.
/// * `settings` - Named analytical target model and additional margin.
/// * `cancel` - Cooperative cancellation checked per frame and carrier block.
/// # Returns
/// Complete per-frame coverage and worst intervals plus gross useful-symbol rate.
/// # Errors
/// Rejects invalid candidate/settings, incompatible frames, limits or cancellation.
pub fn evaluate(
    candidate: &CandidateProfile,
    frames: &[&RxMerFrame],
    settings: &EvaluationSettings,
    cancel: &AtomicBool,
) -> Result<EvaluationReport, LabError> {
    candidate.validate()?;
    settings.validate()?;
    validate_frames(frames, &candidate.key)?;
    let mut results = Vec::with_capacity(frames.len());
    let mut data_bits = 0_u64;
    let mut capacity_complete = true;
    for assignment in &candidate.assignments {
        match assignment.carrier_type {
            CarrierType::Data => match assignment.bit_loading {
                Some(bits) => data_bits += u64::from(bits),
                None => capacity_complete = false,
            },
            CarrierType::Unknown | CarrierType::Reserved => capacity_complete = false,
            _ => {}
        }
    }
    for frame in frames {
        check_cancel(cancel)?;
        let mut result = FrameEvaluation {
            source_sha256: frame.source.sha256.clone(),
            capture_time: frame.capture_time,
            measured_data_carriers: 0,
            missing_data_carriers: 0,
            vulnerable_carriers: 0,
            unresolved_carriers: 0,
            worst_margin_db: None,
            meets_target: None,
        };
        for (offset, assignment) in candidate.assignments.iter().enumerate() {
            if offset % 256 == 0 {
                check_cancel(cancel)?;
            }
            if matches!(
                assignment.carrier_type,
                CarrierType::Unknown | CarrierType::Reserved
            ) {
                result.unresolved_carriers += 1;
                continue;
            }
            if assignment.carrier_type != CarrierType::Data {
                continue;
            }
            let Some(mer) = frame.mer_db[offset] else {
                result.missing_data_carriers += 1;
                continue;
            };
            let Some(target) = assignment
                .bit_loading
                .and_then(|bits| settings.model.target_db(bits))
            else {
                result.unresolved_carriers += 1;
                continue;
            };
            let margin = mer - target - settings.margin_db;
            result.measured_data_carriers += 1;
            result.vulnerable_carriers += usize::from(margin < 0.0);
            result.worst_margin_db = Some(
                result
                    .worst_margin_db
                    .map_or(margin, |previous| previous.min(margin)),
            );
        }
        if result.measured_data_carriers > 0
            && result.missing_data_carriers == 0
            && result.unresolved_carriers == 0
        {
            result.meets_target = Some(result.vulnerable_carriers == 0);
        }
        results.push(result);
    }
    results.sort_by_key(|result| (result.capture_time, result.source_sha256.clone()));
    let worst = results
        .iter()
        .filter(|frame| frame.worst_margin_db.is_some())
        .min_by(|first, second| {
            first
                .worst_margin_db
                .unwrap()
                .total_cmp(&second.worst_margin_db.unwrap())
        });
    let worst_source_sha256 = worst.map(|frame| frame.source_sha256.clone());
    let worst_interval_epoch = worst.map(|frame| frame.capture_time);
    let complete_frame_count = results
        .iter()
        .filter(|frame| frame.meets_target.is_some())
        .count();
    let passing_frame_count = results
        .iter()
        .filter(|frame| frame.meets_target == Some(true))
        .count();
    let mut limitations = vec![
        "Analytical target screening is not FEC simulation or observed modem reliability.".into(),
        "Per-carrier temporal objectives do not guarantee the same whole-frame passing fraction."
            .into(),
    ];
    if !candidate.roles_verified {
        limitations.push("Carrier roles are proposed/unverified; capacity is conditional on the proposed data assignments.".into());
    }
    if complete_frame_count != results.len() {
        limitations.push(
            "Missing readings, unknown roles or uncovered modulations leave incomplete intervals."
                .into(),
        );
    }
    Ok(EvaluationReport {
        result_kind: "analytical_estimate".into(),
        complete_frame_count,
        passing_frame_count,
        fraction_all_frames_meeting_target: passing_frame_count as f64 / results.len() as f64,
        frames: results,
        worst_source_sha256,
        worst_interval_epoch,
        proposed_data_bits_per_useful_symbol: data_bits,
        estimated_gross_rate_bps: if capacity_complete && data_bits > 0 {
            ofdm::useful_symbol_rate(data_bits, candidate.key.grid.spacing_hz)
        } else {
            None
        },
        roles_verified: candidate.roles_verified,
        excluded_overheads: [
            "cyclic_prefix",
            "fec",
            "pilots_plc_and_exclusions",
            "mac_and_other_phy_overhead",
        ]
        .map(String::from)
        .to_vec(),
        limitations,
    })
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
