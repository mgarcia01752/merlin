/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Owned symbol plans and fixed uniform-modulation packing configuration.
use crate::{
    error::invalid,
    signal_processing::modulation::SymbolLabel,
    transmission::SubcarrierSpacing,
    types::{ofdm_modulation::ConstellationOrder, ProfileId},
    LabError,
};
use serde::Serialize;

/// Job-local identity, for diagnostics/truth tracking; not a transmitted NCP field.
pub type CodewordId = u64;
/// Rank among pre-interleaving data positions; does not count scattered pilots.
pub type DataPosition = u16;
/// Fixed cell count of the currently supported short8/9 uniform256-QAM word.
pub const QAM256_CODEWORD_CELLS: usize = 2025;
pub(super) const MAX_QUEUED_CODEWORDS: usize = 16;
pub(super) use crate::transmission::ncp::message::MAX_CODEWORD_STARTS as MAX_ACTIVE_STARTS;
pub(super) const MAX_NCP_ENTRIES: usize = MAX_ACTIVE_STARTS + 1;
const NCP_ENCODED_BITS: usize = 48;
const MAX_PROFILE_ID: ProfileId = 15;

/// Uniform positive-bit-loaded NCP profile; no zero-loaded NCP cells in this mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NcpModulation {
    Qpsk,
    Qam16,
    Qam64,
}
impl NcpModulation {
    /// Returns physical constellation size for the selected NCP profile.
    /// # Returns
    ///4,16or64; this is not a PNM enumeration.
    pub fn order(self) -> ConstellationOrder {
        match self {
            Self::Qpsk => 4,
            Self::Qam16 => 16,
            Self::Qam64 => 64,
        }
    }
    /// Returns cells needed for one48-bit FEC-encoded NCP or CRC block.
    /// # Returns
    ///24,12or8; CRC reserves one additional block of the same size.
    pub fn cells_per_block(self) -> usize {
        NCP_ENCODED_BITS / self.order().ilog2() as usize
    }
}

/// Fixed packer setup; all cell indices belong to the reduced pre-interleaving domain.
#[derive(Debug, Clone, Copy)]
pub struct PackingSettings {
    pub subcarrier_spacing: SubcarrierSpacing,
    /// Constant NI after excluding PLC, continuous pilots and excluded bins.
    pub interleaver_cells: usize,
    pub ncp_modulation: NcpModulation,
    /// Validated field width only; profile contents remain a caller responsibility.
    pub data_profile_id: ProfileId,
}
impl PackingSettings {
    /// Validates bounded NI and space for the minimum NCP plusCRC reservation.
    /// # Returns
    /// Unit for supported uniform256-QAM packing geometry.
    /// # Errors
    /// Rejects invalid profile width or NI outside spacing-specific PHY limits.
    pub fn validate(&self) -> Result<(), LabError> {
        let maximum = self.subcarrier_spacing.max_interleaver_cells();
        if self.interleaver_cells > maximum
            || self.interleaver_cells < 2 * self.ncp_modulation.cells_per_block()
            || self.data_profile_id > MAX_PROFILE_ID
        {
            return Err(invalid(
                "Unsupported packing NI/profile or insufficient NCP space.",
            ));
        }
        Ok(())
    }
}

/// Planned NCP meaning; transmission::ncp encodes its control bits, CRC and FEC.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum NcpPlanEntry {
    CodewordStart {
        codeword_id: CodewordId,
        profile_id: ProfileId,
        pointer: DataPosition,
    },
    /// Z-bit indication; pointer counts data positions, excluding pilot placeholders.
    ZeroLoaded { pointer: DataPosition },
    /// Future wire encoding uses the specified null-pointer value.
    Null,
}

/// Pre-interleaving cell; data provenance is a test oracle, not receiver signaling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PackedCell {
    Data {
        codeword_id: CodewordId,
        codeword_cell: usize,
        label: SymbolLabel,
    },
    ScatteredPilotPlaceholder,
    ZeroLoadedFiller,
    /// Reserved position, indexed in high-to-low NCP field order; no encoded label.
    NcpReserved {
        field_cell: usize,
    },
}

/// Owned plan for one symbol, including starts and the pending queue size afterward.
#[derive(Debug, Clone, Serialize)]
pub struct PackedSymbol {
    pub symbol_index: u64,
    pub cells: Vec<PackedCell>,
    /// Data/terminal entries only; encoded CRC occupies an additional reserved block.
    pub ncp_entries: Vec<NcpPlanEntry>,
    pub ncp_modulation: NcpModulation,
    pub ncp_reserved_cells: usize,
    pub queued_codewords_after: usize,
}

/// Compact serializable view; NCP meanings remain plans rather than encoded messages.
#[derive(Debug, Clone, Serialize)]
pub struct PackingSummary {
    pub symbol_index: u64,
    pub data_cells: usize,
    pub scattered_pilot_placeholders: usize,
    pub zero_loaded_filler_cells: usize,
    pub ncp_reserved_cells: usize,
    pub ncp_modulation: NcpModulation,
    pub ncp_entries: Vec<NcpPlanEntry>,
    pub queued_codewords_after: usize,
    pub ncp_wire_encoded: bool,
}
impl PackedSymbol {
    /// Counts cell categories without exporting every label or changing the plan.
    /// # Returns
    /// Owned summary with copied NCP meanings and explicit wire-encoding status.
    pub fn summary(&self) -> PackingSummary {
        let mut result = PackingSummary {
            symbol_index: self.symbol_index,
            data_cells: 0,
            scattered_pilot_placeholders: 0,
            zero_loaded_filler_cells: 0,
            ncp_reserved_cells: 0,
            ncp_modulation: self.ncp_modulation,
            ncp_entries: self.ncp_entries.clone(),
            queued_codewords_after: self.queued_codewords_after,
            ncp_wire_encoded: false,
        };
        for cell in &self.cells {
            match cell {
                PackedCell::Data { .. } => result.data_cells += 1,
                PackedCell::ScatteredPilotPlaceholder => result.scattered_pilot_placeholders += 1,
                PackedCell::ZeroLoadedFiller => result.zero_loaded_filler_cells += 1,
                PackedCell::NcpReserved { .. } => result.ncp_reserved_cells += 1,
            }
        }
        result
    }
}
