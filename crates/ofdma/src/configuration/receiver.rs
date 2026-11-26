/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Explicit receiver engineering policies, without injected truth or target noise.
use crate::{error::invalid, PhyResult};
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use serde::{Deserialize, Serialize};

/// Frame-stationary received-pilot estimator policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Estimator {
    /// One LS complex response fitted to the frame's available known pilots.
    FrameFlatLs,
    /// Per-pilot-carrier LS, bracketed within contiguous regions and gap limit.
    FramePilotLinear {
        /// Maximum bracket separation in physical carriers.
        maximum_gap: usize,
    },
}

/// Admitted configured-timing receiver settings, independent of channel models.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReceiverConfig {
    estimator: Estimator,
    variance: f64,
    power_floor: f64,
    iterations: usize,
}

impl ReceiverConfig {
    /// Validates `estimator`, reference complex `variance`, response-power floor
    /// and bounded LDPC `iterations` before any receive input is accepted.
    ///
    /// Returns owned settings; rejects nonfinite/nonpositive or excessive values.
    /// Variance is an explicit confidence assumption, never measured RxMER.
    pub fn new(
        estimator: Estimator,
        variance: f64,
        power_floor: f64,
        iterations: usize,
    ) -> PhyResult<Self> {
        if !variance.is_finite()
            || variance <= 0.0
            || !variance.recip().is_finite()
            || !power_floor.is_finite()
            || power_floor <= 0.0
            || !(1..=100).contains(&iterations)
            || matches!(
                estimator,
                Estimator::FramePilotLinear {
                    maximum_gap: 0 | 4097..
                }
            )
        {
            return Err(invalid("receiver.policy"));
        }
        Ok(Self {
            estimator,
            variance,
            power_floor,
            iterations,
        })
    }
    /// Borrows the accepted estimator policy without changing frame history.
    ///
    /// No automatic fallback or oracle correction is implied.
    pub fn estimator(&self) -> &Estimator {
        &self.estimator
    }
    /// Returns declared complex reference error power after role-gain removal.
    ///
    /// It is not inferred from channel settings or fit residuals.
    pub fn variance(&self) -> f64 {
        self.variance
    }
    /// Returns the positive response-power rejection floor.
    ///
    /// Estimates at/below it remain observed but unavailable for equalization.
    pub fn power_floor(&self) -> f64 {
        self.power_floor
    }
    /// Returns the finite LDPC work budget per indivisible decode call.
    ///
    /// Actual iterations are retained separately in each decoder result.
    pub fn iterations(&self) -> usize {
        self.iterations
    }
}

impl RetainedPayload for ReceiverConfig {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            estimator,
            variance,
            power_floor,
            iterations,
        } = self;
        sum([
            estimator.retained_children(),
            variance.retained_children(),
            power_floor.retained_children(),
            iterations.retained_children(),
        ])
    }
}

impl RetainedPayload for Estimator {
    /// Audits the complete inline estimator policy without running estimation.
    ///
    /// # Returns
    /// Checked dynamic children; current variants own only fixed scalar values.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        match self {
            Self::FrameFlatLs => Some(RetainedFootprint::default()),
            Self::FramePilotLinear { maximum_gap } => maximum_gap.retained_children(),
        }
    }
}
