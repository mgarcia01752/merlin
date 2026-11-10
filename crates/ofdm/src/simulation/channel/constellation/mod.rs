/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Matched payload/NCP receive observations using an independent, bounded provenance stream.
mod collector;
mod model;
mod selection;
pub(crate) use collector::ConstellationCollector;
pub use model::{
    ConstellationPair, ConstellationResult, ConstellationStatus, NcpConstellationPair,
};
pub use selection::{ConstellationSelection, DEFAULT_CONSTELLATION_PAIRS, MAX_CONSTELLATION_PAIRS};
