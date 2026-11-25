/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared OFDM assignments from CM-OSSI 3.1 I27 / 4.0 I13, D.2.1.7.
use super::{BitsPerSymbol, ModulationOrder};
use crate::common::signal_processing::shannon::{minimum_snr_db, ShannonSnrDb};
use serde::Serialize;

const BPSK_BITS_PER_SYMBOL: BitsPerSymbol = 1;

/// Application classification of the assignment roles identified by OSSI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CarrierType {
    Data,
    ZeroBitLoaded,
    ContinuousPilot,
    Exclusion,
    Plc,
    Reserved,
    Unknown,
}

/// Physical modulation parameters, distinct from the OSSI assignment role.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct ModulationParameters {
    pub constellation: &'static str,
    pub bit_loading: BitsPerSymbol,
    /// Ideal Shannon bound, not measured MER or a receiver operating threshold.
    pub shannon_minimum_mer_db: ShannonSnrDb,
}

impl CarrierType {
    /// All recognized classifications in stable presentation order.
    pub const ALL: [Self; 7] = [
        Self::Data,
        Self::ContinuousPilot,
        Self::Plc,
        Self::Exclusion,
        Self::ZeroBitLoaded,
        Self::Reserved,
        Self::Unknown,
    ];
}

/// Semantic interpretation shared by JSON export and desktop charting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModulationAssignment {
    pub modulation_order: &'static str,
    /// Constellation bits per symbol; PLC describes its data segment, not its preamble.
    /// A nonzero value does not imply user-data capacity (pilots and PLC are overhead).
    pub bit_loading: Option<BitsPerSymbol>,
    pub carrier_type: CarrierType,
    /// Short desktop label, separate from the stable JSON name.
    pub display_label: &'static str,
}

// Each numeric code is declared once in this protocol table. Text names follow
// PyPNM's ModulationOrderType; carrier_type is our classification, not a wire field.
const ASSIGNMENTS: &[(
    ModulationOrder,
    &str,
    Option<BitsPerSymbol>,
    CarrierType,
    &str,
)] = &[
    (
        0,
        "zero_bit_loaded",
        Some(0),
        CarrierType::ZeroBitLoaded,
        "Zero-loaded",
    ),
    (
        1,
        "continuous_pilot",
        Some(BPSK_BITS_PER_SYMBOL),
        CarrierType::ContinuousPilot,
        "Pilots",
    ),
    (2, "qpsk", Some(2), CarrierType::Data, "QPSK"),
    (3, "reserved_3", None, CarrierType::Reserved, "Reserved"),
    (4, "qam_16", Some(4), CarrierType::Data, "16-QAM"),
    (5, "reserved_5", None, CarrierType::Reserved, "Reserved"),
    (6, "qam_64", Some(6), CarrierType::Data, "64-QAM"),
    (7, "qam_128", Some(7), CarrierType::Data, "128-QAM"),
    (8, "qam_256", Some(8), CarrierType::Data, "256-QAM"),
    (9, "qam_512", Some(9), CarrierType::Data, "512-QAM"),
    (10, "qam_1024", Some(10), CarrierType::Data, "1024-QAM"),
    (11, "qam_2048", Some(11), CarrierType::Data, "2048-QAM"),
    (12, "qam_4096", Some(12), CarrierType::Data, "4096-QAM"),
    (13, "qam_8192", Some(13), CarrierType::Data, "8192-QAM"),
    (14, "qam_16384", Some(14), CarrierType::Data, "16384-QAM"),
    (16, "exclusion", None, CarrierType::Exclusion, "Excluded"),
    (20, "plc", Some(4), CarrierType::Plc, "PLC"),
];

impl ModulationAssignment {
    /// Interprets an encoded assignment without rejecting unassigned values.
    ///
    /// # Arguments
    /// * `code` - Original one-byte modulation assignment from a profile scheme.
    /// # Returns
    /// Known modulation name and carrier role, or `unknown` with no bit loading.
    /// PHY 3.1 I20 sections 7.5.13 and 7.5.15.3 specify PLC data as
    /// 16-QAM and continuous pilots as BPSK. Neither becomes a data carrier.
    pub fn from_code(code: ModulationOrder) -> Self {
        match ASSIGNMENTS.iter().find(|entry| entry.0 == code) {
            Some(&(_, modulation_order, bit_loading, carrier_type, display_label)) => Self {
                modulation_order,
                bit_loading,
                carrier_type,
                display_label,
            },
            None => Self {
                modulation_order: "unknown",
                bit_loading: None,
                carrier_type: CarrierType::Unknown,
                display_label: "Reserved",
            },
        }
    }
}

impl ModulationAssignment {
    /// Returns the physical constellation for a known modulated assignment.
    /// # Returns
    /// PLC data is `qam_16`, pilots are `bpsk`, and data uses its QAM/QPSK name.
    /// Unmodulated, zero-loaded and unknown assignments return None.
    pub fn constellation(&self) -> Option<&'static str> {
        match self.carrier_type {
            CarrierType::Data => Some(self.modulation_order),
            CarrierType::ContinuousPilot => Some("bpsk"),
            CarrierType::Plc => Some("qam_16"),
            _ => None,
        }
    }

    /// Calculates the same ideal reference used by JSON and desktop charts.
    /// # Returns
    /// `10 log10(2^bits - 1)` for a known constellation, otherwise None.
    /// This is not a DOCSIS receiver MER requirement or a pilot power adjustment.
    pub fn shannon_minimum_mer_db(&self) -> Option<ShannonSnrDb> {
        self.constellation()?;
        minimum_snr_db(self.bit_loading?)
    }

    /// Describes the PLC preamble separately from its 16-QAM data segment.
    /// # Returns
    /// BPSK parameters for PLC (PHY 3.1 I20 section 7.5.13.3); otherwise None.
    /// No symbol timing or instantaneous preamble position is inferred from a profile.
    pub fn preamble(&self) -> Option<ModulationParameters> {
        if self.carrier_type != CarrierType::Plc {
            return None;
        }
        Some(ModulationParameters {
            constellation: "bpsk",
            bit_loading: BPSK_BITS_PER_SYMBOL,
            shannon_minimum_mer_db: minimum_snr_db(BPSK_BITS_PER_SYMBOL)?,
        })
    }
}
