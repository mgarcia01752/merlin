/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Continuous reproducible payload source and independent contiguous verifier.
mod prbs15;
mod verifier;
pub use prbs15::Prbs15;
pub use verifier::{Comparison, PrbsVerifier};
