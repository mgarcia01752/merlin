/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Bounded composition of the eight upstream transmitter stages.
mod assembly;
mod framer;
mod framing;
pub use assembly::BurstAssembler;
pub use framer::{TransmittedFrame, Transmitter};
pub use framing::encode_grant;
