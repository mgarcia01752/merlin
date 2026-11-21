/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Shared pure FEC grant planning; both endpoints derive layout independently.
mod fec_selection;
mod plan;
pub use fec_selection::select;
pub use plan::{CodewordPlan, GrantPlan};
