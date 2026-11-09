/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit reference-plane sample channels; impairment models remain separate.
#[cfg(feature = "simulation")]
pub mod awgn;
#[cfg(feature = "simulation")]
pub mod impairments;
pub mod reference;
/// Application bound on normalized complex sample/bin noise power, not a DOCSIS limit.
pub const MAX_NORMALIZED_NOISE_POWER: f64 = 1000.0;
