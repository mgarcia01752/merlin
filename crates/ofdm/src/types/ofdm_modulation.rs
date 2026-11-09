/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Downstream data constellations, distinct from pilot/PLC roles and device support.
//! PHY3.1 I20 section7.5.5.1, PDF120; PHY4.0 I07 section7.3.5.5, PDF134.
use super::BitsPerSymbol;
use serde::{Deserialize, Serialize};

/// Physical constellation size, not an encoded PNM modulation assignment.
pub type ConstellationOrder = u16;
/// Endpoint support requirement in the pinned downstream modulation section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceSupportRequirement {
    Required,
    Optional,
}
/// Scientific implementation state, separate from normative endpoint capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImplementationState {
    ImplementedExperimental,
    Pending,
}
/// Inspectable modulation capability; pending modes never use a substitute mapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct OfdmModulation {
    pub order: ConstellationOrder,
    pub bits_per_symbol: BitsPerSymbol,
    pub device_support: DeviceSupportRequirement,
    pub mapping: ImplementationState,
    pub coding_simulation: ImplementationState,
}
/// Maximum downstream data constellation in the pinned baseline and D4.0 FDD reuse.
pub const MAX_OFDM_BITS_PER_SYMBOL: BitsPerSymbol = 14;
/// Known downstream data assignments. NCP/pilot/PLC/zero-loaded roles are separate.
pub const OFDM_DATA_MODULATIONS: [OfdmModulation; 10] = [
    modulation(4, DeviceSupportRequirement::Required),
    modulation(6, DeviceSupportRequirement::Required),
    modulation(7, DeviceSupportRequirement::Required),
    modulation(8, DeviceSupportRequirement::Required),
    modulation(9, DeviceSupportRequirement::Required),
    modulation(10, DeviceSupportRequirement::Required),
    modulation(11, DeviceSupportRequirement::Required),
    modulation(12, DeviceSupportRequirement::Required),
    modulation(13, DeviceSupportRequirement::Optional),
    modulation(14, DeviceSupportRequirement::Optional),
];
const fn modulation(
    bits: BitsPerSymbol,
    device_support: DeviceSupportRequirement,
) -> OfdmModulation {
    let state = if bits == 4 {
        ImplementationState::ImplementedExperimental
    } else {
        ImplementationState::Pending
    };
    OfdmModulation {
        order: 1 << bits,
        bits_per_symbol: bits,
        device_support,
        mapping: ImplementationState::ImplementedExperimental,
        coding_simulation: state,
    }
}
impl OfdmModulation {
    /// Looks up a downstream data constellation without guessing unsupported orders.
    /// # Arguments
    /// * `bits` - Data bits per QAM symbol; pilot and PLC roles use other APIs.
    /// # Returns
    /// Normative capability and implementation state, or None for unknown loading.
    pub fn from_bits(bits: BitsPerSymbol) -> Option<Self> {
        OFDM_DATA_MODULATIONS
            .iter()
            .copied()
            .find(|modulation| modulation.bits_per_symbol == bits)
    }
}
