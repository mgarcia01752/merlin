/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit MER target policies; none guarantees error-free receiver operation.
use crate::{
    common::signal_processing::shannon::minimum_snr_db,
    pnm::data_type::{
        modulation::{CarrierType, ModulationAssignment},
        units::QUARTER_DB_PER_DB,
        BitsPerSymbol,
    },
};
use serde::{Deserialize, Serialize};

/// Versioned analysis policy, distinct from actual configured modem modulation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BitLoadingModel {
    #[default]
    WebuiMerHeuristic,
    OssiCandidateDefaults,
    IdealShannon,
}
// CM-OSSI 3.1 I27 D.2.11 Table 72, pp.182–183: quarter-dB defaults.
// These targets are configurable on devices; the capture does not contain them.
const OSSI_TARGETS: &[(BitsPerSymbol, u16)] = &[
    (4, 60),
    (6, 84),
    (7, 96),
    (8, 108),
    (9, 122),
    (10, 136),
    (11, 148),
    (12, 164),
    (13, 184),
    (14, 208),
];
// PyPNM-CMTS-WebUI 25f40b, src/lib/rxMerSelectionInsights.ts.
const WEBUI_TARGETS: &[(BitsPerSymbol, f64)] =
    &[(8, 33.0), (9, 36.0), (10, 39.0), (11, 42.0), (12, 45.0)];

impl BitLoadingModel {
    /// Parses a stable CLI/JSON policy name without guessing unknown models.
    /// # Arguments
    /// * `name` - Exact policy identifier.
    /// # Returns
    /// Known policy, or None for unsupported text.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "webui_mer_heuristic" => Some(Self::WebuiMerHeuristic),
            "ossi_candidate_defaults" => Some(Self::OssiCandidateDefaults),
            "ideal_shannon" => Some(Self::IdealShannon),
            _ => None,
        }
    }
    /// Returns a stable identifier suitable for command-line requests and JSON.
    /// # Returns
    /// Policy name independent of display text.
    pub fn name(self) -> &'static str {
        match self {
            Self::WebuiMerHeuristic => "webui_mer_heuristic",
            Self::OssiCandidateDefaults => "ossi_candidate_defaults",
            Self::IdealShannon => "ideal_shannon",
        }
    }
    /// Resolves a required MER target, before any additional user margin.
    /// # Arguments
    /// * `bits` - DS data assignment bit count, not an arbitrary QAM order.
    /// # Returns
    /// Target in dB, or None when this policy does not cover the assignment.
    pub fn target_db(self, bits: BitsPerSymbol) -> Option<f64> {
        if !is_data_bit_loading(bits) {
            return None;
        }
        match self {
            Self::WebuiMerHeuristic => WEBUI_TARGETS
                .iter()
                .find(|entry| entry.0 == bits)
                .map(|entry| entry.1),
            Self::OssiCandidateDefaults => OSSI_TARGETS
                .iter()
                .find(|entry| entry.0 == bits)
                .map(|entry| f64::from(entry.1) / QUARTER_DB_PER_DB),
            Self::IdealShannon => minimum_snr_db(bits),
        }
    }
    /// Finds the highest covered order meeting the supplied MER, cap and margin.
    /// # Arguments
    /// * `mer_db` - Finite measured MER in dB; missing samples must not call this.
    /// * `margin_db` - Additional nonnegative target margin.
    /// * `cap` - Maximum allowed data bit count.
    /// # Returns
    /// Highest matching order, or None below this policy's minimum covered target.
    pub fn supported_bits(
        self,
        mer_db: f64,
        margin_db: f64,
        cap: BitsPerSymbol,
    ) -> Option<BitsPerSymbol> {
        if !mer_db.is_finite() || !margin_db.is_finite() || margin_db < 0.0 {
            return None;
        }
        (2..=cap).rev().find(|bits| {
            self.target_db(*bits)
                .is_some_and(|target| mer_db >= target + margin_db)
        })
    }
}
/// Checks the shared assignment registry instead of duplicating a QAM allowlist.
/// # Arguments
/// * `bits` - Assignment code whose encoded value equals data bits per symbol.
/// # Returns
/// True for a known positive Data assignment no larger than the registry maximum.
pub fn is_data_bit_loading(bits: BitsPerSymbol) -> bool {
    let assignment = ModulationAssignment::from_code(bits);
    assignment.carrier_type == CarrierType::Data && assignment.bit_loading == Some(bits)
}
