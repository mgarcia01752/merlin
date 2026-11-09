/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Constellation references, bit ordering and soft demapping.
#[cfg(feature = "simulation")]
pub mod interleaving;
pub mod qam;
pub mod randomization;
pub mod soft_bits;
pub use qam::{
    map_qam16, map_symbol, reference_for_order, QamReference, QamScaleFactor, SymbolLabel,
};
pub use soft_bits::{qam16_llrs, qam_llrs};
/// Number of mapped bits in the verified16-QAM symbol.
pub const QAM16_BITS_PER_SYMBOL: usize = 4;
