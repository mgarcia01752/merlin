/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent full-loop public contracts over the existing downstream controlled job.
mod control;
mod instruments;
mod observations;
mod options;
mod request;
mod result;
use super::*;
pub use control::*;
pub use instruments::*;
pub use observations::*;
pub use options::*;
pub use request::*;
pub use result::*;
use std::sync::Arc;

/// Owned accepted directional inputs; cloning preserves immutable original/source ownership.
#[derive(Clone)]
pub struct FullLoopInputs {
    pub transmission: TransmissionConfiguration,
    pub profile: Dpd,
    pub source: SourceInput,
    pub measurements: MeasurementSequence,
    pub channel: ChannelConfiguration,
    /// Explicit None selects synthetic unity independently of receiver correction policy.
    pub response: Option<ChannelEstimate>,
}

impl Simulation {
    /// Prepares an independent immutable full-loop request through existing scientific owners.
    /// # Arguments
    /// * `inputs` - Owned passive accepted source/profile/measurement/channel inputs.
    /// * `options` - Explicit bounded execution and observation policy.
    /// # Returns
    /// Reusable validated request; existing session state and accepted inputs remain unchanged.
    /// # Errors
    /// Preserves typed limits, identity/epoch/grid/coverage and supported coding errors.
    pub fn prepare_full_loop(
        inputs: FullLoopInputs,
        options: FullLoopOptions,
    ) -> ApiResult<FullLoopRequest> {
        FullLoopRequest::prepare(inputs, options)
    }
}
