/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Compact validated profile metadata; original encoded IDs/schemes remain explicit.
use crate::{
    scenario::{FrequencyGrid, ScenarioKey},
    types::modulation::{CarrierType, ModulationAssignment, ModulationParameters},
    SimulationError,
};
use serde::Serialize;
use std::collections::BTreeMap;

/// Exact supported source header, distinct from a simulated control message.
#[derive(Clone, Serialize)]
pub struct ProfileCaptureHeader {
    pub file_type: String,
    pub file_type_version: u8,
    pub major_version: u8,
    pub minor_version: u8,
    /// Original whole capture seconds without an invented playback epoch.
    pub capture_time: u32,
}

/// One original compact range with authoritative coordinates and assignment interpretation.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ProfileSchemeInfo {
    /// Zero-based original compact scheme position, separate from physical carrier indices.
    pub source_index: usize,
    pub first_offset: usize,
    pub count: usize,
    pub first_physical_carrier: usize,
    pub last_physical_carrier: usize,
    pub first_frequency_hz: u32,
    pub last_frequency_hz: u32,
    /// Unmodified source code, including unassigned values.
    pub modulation_order_code: u8,
    pub modulation_order: &'static str,
    pub carrier_type: CarrierType,
    pub bit_loading: Option<u8>,
    pub display_label: &'static str,
    pub constellation: Option<&'static str>,
    /// Ideal Shannon capacity bound; separate from DOCSIS receiver requirements.
    pub ideal_shannon_snr_db: Option<f64>,
    /// PLC preamble metadata stays separate from its data constellation.
    pub preamble: Option<ModulationParameters>,
}

impl ProfileSchemeInfo {
    pub(super) fn new(
        source_index: usize,
        first_offset: usize,
        count: usize,
        code: u8,
        grid: &FrequencyGrid,
    ) -> Result<Self, SimulationError> {
        let assignment = ModulationAssignment::from_code(code);
        let last = first_offset + count - 1;
        Ok(Self {
            source_index,
            first_offset,
            count,
            first_physical_carrier: usize::from(grid.first_subcarrier_index) + first_offset,
            last_physical_carrier: usize::from(grid.first_subcarrier_index) + last,
            first_frequency_hz: grid.frequency(first_offset)?,
            last_frequency_hz: grid.frequency(last)?,
            modulation_order_code: code,
            modulation_order: assignment.modulation_order,
            carrier_type: assignment.carrier_type,
            bit_loading: assignment.bit_loading,
            display_label: assignment.display_label,
            constellation: assignment.constellation(),
            ideal_shannon_snr_db: assignment.shannon_minimum_mer_db(),
            preamble: assignment.preamble(),
        })
    }
}

/// Retained compact source metadata; scientific import callers can omit these arrays.
#[derive(Clone)]
pub struct ProfileCaptureMetadata {
    pub header: ProfileCaptureHeader,
    pub key: ScenarioKey,
    pub schemes: BTreeMap<u8, Vec<ProfileSchemeInfo>>,
}
