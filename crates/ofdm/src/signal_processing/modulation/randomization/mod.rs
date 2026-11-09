/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 section7.5.5.3: data/NCP randomization, separate from PLC randomization.
mod cell;
mod lfsr;
pub use cell::CellRandomizer;
pub(crate) use lfsr::Lfsr;
pub use lfsr::{RandomizerRegister, RandomizerState};
