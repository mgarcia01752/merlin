/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Stateful cell packing in the pre-interleaving domain; no NCP wire encoding.
mod model;
mod packer;
mod planner;
pub use model::{
    CodewordId, DataPosition, NcpModulation, NcpPlanEntry, PackedCell, PackedSymbol,
    PackingSettings, PackingSummary,
};
pub use packer::SymbolPacker;
