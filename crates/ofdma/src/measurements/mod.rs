/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Explicitly synthetic baseband inputs; verified binary captures are separate.
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
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

impl RetainedPayload for SyntheticGrid {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            waveform,
            excluded,
            provenance: _,
        } = self;
        sum([waveform.retained_children(), excluded.retained_children()])
    }
}
