/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual per-source-carrier performance over existing scoped numerical/report owners.
use super::*;
use crate::signal_processing::shannon::minimum_snr_db;
use serde::Serialize;

/// One original source carrier with actual diagnostic MER and independent ideal capacity.
#[derive(Clone, Serialize)]
#[non_exhaustive]
pub struct FullLoopCarrierPerformance {
    /// Original zero-based source offset, independent of physical FFT-bin identity.
    pub source_offset: usize,
    /// Exact original carrier center in Hz, without resampling or RF upconversion.
    pub frequency_hz: FrequencyHz,
    /// Original admitted carrier role, including null/excluded/reserved distinctions.
    pub role: ProfileCarrierRole,
    /// Actual observation-cell samples/power sums/MER at the physical bin.
    pub measurement: CarrierMer,
    /// Ideal AWGN Shannon capacity MER, not a modem operating requirement or coded margin.
    pub ideal_shannon_mer_db: Option<MerDb>,
}

/// Owned complete-source-grid performance; scientific result and source scopes stay explicit.
#[derive(Clone, Serialize)]
#[non_exhaustive]
pub struct FullLoopProfilePerformance {
    /// Original selected capture identity, distinct from the bootstrap wire Profile A.
    pub profile_id: ProfileId,
    /// Original device/channel/frequency grid without retained input bytes.
    pub key: ScenarioKey,
    /// Actual completed/cancelled/runtime/failed lifecycle, independent of FEC success.
    pub status: ChannelJobStatus,
    /// Observation-window decoder counts; unavailable windows retain an empty scoped report.
    pub fec: FecReport,
    /// Observation-window source BER; None when no valid denominator exists.
    pub ber: Option<BitErrorRatio>,
    /// Whole-run unfinished generated payloads, separate from observation-window FEC.
    pub unrecovered_generated_payloads: u64,
    /// All original source carriers in source order, before presentation grouping.
    pub points: Vec<FullLoopCarrierPerformance>,
    /// Existing scientific error text; absence does not claim zero scientific errors.
    pub error: Option<String>,
    /// Diagnostic matched-TX comparison only; no reference enters normal receiver processing.
    pub reference_plane: &'static str,
}

impl FullLoopResult {
    /// Reads full source-grid performance using existing MER, Shannon and scoped FEC owners.
    /// # Returns
    /// Owned source coordinates/roles, typed observation-cell power/count/MER, ideal capacity,
    /// observation-window FEC/BER and whole-run unfinished totals. Missing/zero-error MER
    /// remains optional and distinguishable through sample/power quantities. No source,
    /// simulation, receiver or decoder advances; retained stage policies do not change scope.
    /// # Errors
    /// Preserves invalid original frequency/bin geometry as structured admission errors.
    pub fn profile_performance(&self) -> ApiResult<FullLoopProfilePerformance> {
        let profile = &self.inputs.profile;
        let grid = profile.grid();
        let points = profile
            .carriers()
            .iter()
            .enumerate()
            .map(|(source_offset, assignment)| {
                let physical = usize::from(grid.first_subcarrier_index)
                    .checked_add(source_offset)
                    .and_then(|value| u16::try_from(value).ok())
                    .ok_or_else(|| {
                        ApiError::new(
                            ErrorCode::IncompatibleGrid,
                            "Performance carrier index overflows",
                        )
                    })?;
                let physical = SubcarrierIndex::new(physical)?;
                let ideal = match assignment.carrier_type {
                    ProfileCarrierRole::Data
                    | ProfileCarrierRole::ContinuousPilot
                    | ProfileCarrierRole::Plc => {
                        assignment.bit_loading.and_then(minimum_snr_db).map(MerDb)
                    }
                    _ => None,
                };
                Ok(FullLoopCarrierPerformance {
                    source_offset,
                    frequency_hz: FrequencyHz(grid.frequency(source_offset)?),
                    role: assignment.carrier_type,
                    measurement: CarrierMer::from_observation(
                        physical,
                        self.job.carrier_mer.get(usize::from(physical.value())),
                    ),
                    ideal_shannon_mer_db: ideal,
                })
            })
            .collect::<ApiResult<Vec<_>>>()?;
        let window = self.job.summary.window.as_ref();
        Ok(FullLoopProfilePerformance {
            profile_id: profile.profile_id(),
            key: profile.key().clone(),
            status: self.job.status,
            fec: FecReport::from_counters(
                window.map_or_else(Default::default, |value| value.fec),
                MetricScope::ObservationWindow,
            ),
            ber: window.and_then(|value| value.ber).map(BitErrorRatio),
            unrecovered_generated_payloads: self.job.unrecovered_generated_payloads,
            points,
            error: self.job.error.as_ref().map(ToString::to_string),
            reference_plane: "post_equalization_against_matched_tx_diagnostic_symbols",
        })
    }
}
