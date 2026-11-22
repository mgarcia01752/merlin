/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent full-loop public contracts over the existing downstream controlled job.
mod cells;
mod control;
mod instruments;
mod messages;
mod observations;
mod options;
mod physical;
mod request;
mod result;
mod sweep;
mod transmit_cells;
use super::*;
pub use cells::*;
pub use control::*;
pub use instruments::*;
pub use messages::*;
pub use observations::*;
pub use options::*;
pub use physical::*;
pub use request::*;
pub use result::*;
use std::sync::Arc;
pub use sweep::*;
pub use transmit_cells::*;

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

    /// Prepares a request with independent H/profile and RxMER/profile tolerances.
    /// # Arguments
    /// * `inputs` - Owned accepted inputs; the measurement sequence retains its own tolerance.
    /// * `options` - Explicit bounded execution and observation policy.
    /// * `tolerance` - Maximum whole-second original H/profile epoch distance.
    /// # Returns
    /// Owned idle request preserving the separately accepted response tolerance through sweeps.
    /// Existing prepare_full_loop behavior remains unchanged; no source or RNG advances.
    /// # Errors
    /// Preserves scientific, identity, epoch, coverage and resource admission errors.
    pub fn prepare_full_loop_with_response_tolerance(
        inputs: FullLoopInputs,
        options: FullLoopOptions,
        tolerance: AssociationTolerance,
    ) -> ApiResult<FullLoopRequest> {
        FullLoopRequest::prepare_with_response_tolerance(inputs, options, Some(tolerance))
    }
}
