/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared reference metadata; measured s2.13 samples must not be normalized twice.
use crate::{
    common::signal_processing::qam::{reference_for_modulation_type, QamReference},
    pnm::data_type::ConstellationModulationOrder,
};
use serde::Serialize;
pub mod mer;

/// Analysis serialized identically by the parser, CLI and desktop exports.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConstellationAnalysis {
    pub reference: Option<&'static QamReference>,
    pub decision_rule: &'static str,
    pub normalization: &'static str,
    pub soft_samples_rescaled: bool,
    pub mer: mer::MerResult,
}
impl ConstellationAnalysis {
    /// Resolves reference geometry while preserving original measured samples.
    /// # Arguments
    /// * `modulation_type` - The header's DsOfdmModulationType code.
    /// # Returns
    /// Embedded reference or null for unsupported modulation; no guessed geometry.
    /// MER is unavailable until the parser analyzes the received samples.
    pub fn from_modulation_type(modulation_type: ConstellationModulationOrder) -> Self {
        Self {
            reference: reference_for_modulation_type(modulation_type),
            decision_rule: "nearest_reference_euclidean",
            normalization: "unit_average_symbol_power",
            soft_samples_rescaled: false,
            mer: mer::MerResult::unavailable(0, mer::MerStatus::NoSamples),
        }
    }
}
