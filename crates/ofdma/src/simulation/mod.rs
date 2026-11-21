/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Bounded frozen requests, lifecycle and source comparison outside normal RX.
mod control;
mod report;
mod request;
mod run;
pub use control::{ProgressSnapshot, RunControl, RunStatus};
pub use report::{BatchReport, GrantReport, RunFailure};
pub use request::{preview, ExperimentConfig, ExperimentOptions, ExperimentPreview};
pub use run::run;
