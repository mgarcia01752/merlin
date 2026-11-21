/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Units, coordinates and owned scientific buffers shared by stages.
mod bits;
mod iq;
mod samples;

pub use bits::{BitRange, Identity, SourceChunk};
pub use iq::Iq;
pub use samples::SampleBlock;
