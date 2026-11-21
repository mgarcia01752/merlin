/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Explicitly synthetic baseband inputs; verified binary captures are separate.
mod pre_eq;
mod rxmer;
use crate::{
    configuration::{UsConfiguration, Waveform},
    error::invalid,
    PhyResult,
};
pub use pre_eq::{PreEqEvent, PreEqOperation, PreEqPlayback, ProbeReference};
pub use rxmer::{PowerSnapshot, RxmerPlayback, RxmerPolicy, SyntheticRxmer};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
struct SyntheticGrid {
    waveform: Waveform,
    excluded: Vec<usize>,
    provenance: &'static str,
}
impl SyntheticGrid {
    fn new(config: &UsConfiguration) -> Self {
        Self {
            waveform: config.waveform().clone(),
            excluded: config.excluded().to_vec(),
            provenance: "synthetic-normalized-baseband",
        }
    }
    fn validate(&self, config: &UsConfiguration) -> PhyResult<()> {
        if self != &Self::new(config) {
            return Err(invalid("measurement.synthetic_grid"));
        }
        Ok(())
    }
}
