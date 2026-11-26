/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Typed owned retention and bounded queries over actual borrowed core test-point events.
mod data;
mod policy;
mod query_storage;
mod store;
use super::*;
pub use crate::signal_processing::pilots::PilotValue as FullLoopPilotValue;
pub use crate::simulation::test_points::{
    CellProbeGrid as FullLoopCellGrid, PhysicalProbeGrid as FullLoopPhysicalGrid,
    PlcProbeGrid as FullLoopPlcGrid, RunPhase as FullLoopStagePhase,
    SignalReference as FullLoopStageReference, TransmitCellRole as FullLoopCellRole,
    WaveformProbeGrid as FullLoopWaveformGrid,
};
pub use crate::simulation::test_points::{
    ProbeContext as FullLoopStageContext, ProbeUnits, SignalDirection, SignalDomain,
    TestPointId as FullLoopStage,
};
pub use crate::transmission::symbols::TransportCell as FullLoopTransportCell;
pub use data::*;
pub use policy::*;
pub(super) use store::RetainedStages;
pub use store::{
    FullLoopObservationStats, FullLoopStageInfo, FullLoopStageSnapshot, FullLoopStageViewport,
};

impl FullLoopRequest {
    /// Creates a fresh request with an explicit validated passive observation policy.
    /// # Arguments
    /// * `policy` - Immutable bounded retention; original request/default options remain unchanged.
    /// # Returns
    /// Independent request snapshot; attaching never executes source/DSP/FEC work.
    /// # Errors
    /// Preserves original probe selection validation before accepting the new request.
    pub fn with_observations(&self, policy: FullLoopObservationPolicy) -> ApiResult<Self> {
        policy.probe()?;
        let mut request = self.clone();
        request.observation_policy = Some(policy);
        Ok(request)
    }
    /// Borrows accepted opt-in policy independently of existing execution options.
    /// # Returns
    /// None for default requests; original accepted stage policy otherwise.
    pub fn observation_policy(&self) -> Option<&FullLoopObservationPolicy> {
        self.observation_policy.as_ref()
    }
}
impl FullLoopResult {
    /// Borrows the exact accepted retention policy, independent of request disposal or later runs.
    /// # Returns
    /// None for capture-disabled results; original stage/phase/budget selections otherwise.
    pub fn observation_policy(&self) -> Option<&FullLoopObservationPolicy> {
        self.observation_policy.as_ref()
    }

    /// Copies a bounded per-stage inventory in registered stage order.
    /// # Returns
    /// Registered IDs and actual event/element counts; no data is reconstructed or rerun.
    /// Final vector capacity equals length. Its retained payload fits within this
    /// result's retained_footprint payload bytes, with zero ordered entries/shared
    /// roots. Reserve each simultaneous copy separately; Python/JSON and consumers'
    /// derived containers have independent ownership and are outside this envelope.
    pub fn stage_inventory(&self) -> Vec<FullLoopStageInfo> {
        self.stages.inventory()
    }
    /// Reads accounted retention and dropped-copy counts separately from core probe statistics.
    /// # Returns
    /// Scalar owned statistics; exhaustion never changes scientific values or completed trials.
    pub fn observation_stats(&self) -> FullLoopObservationStats {
        self.stages.stats.clone()
    }
    /// Copies a bounded actual stage window with original metadata and element coordinates.
    /// # Arguments
    /// * `stage` - Registered stage identity.
    /// * `snapshot` - Zero-based actual retained event for that stage.
    /// * `viewport` - Contiguous original retained-element window; at most 4096 copied elements.
    /// # Returns
    /// Owned typed values, exact reference/phase/grid/context and original totals/offsets.
    /// Final text/vector capacities equal lengths. This individual returned owner's
    /// complete retained payload is no larger than this immutable result's
    /// retained_footprint payload bytes, with zero ordered entries/shared roots.
    /// This conservative copy envelope is specific to stage/stage_inventory;
    /// it is not a general future-output or allocator/scratch/RSS bound. Reserve
    /// each simultaneous query, concatenation and consumer copy independently.
    /// # Errors
    /// MissingCoverage for absent retained stages, InvalidIdentifier for absent snapshots;
    /// invalid/overflow/outside windows and query work budgets reject without mutation.
    pub fn stage(
        &self,
        stage: FullLoopStage,
        snapshot: usize,
        viewport: FullLoopStageViewport,
    ) -> ApiResult<FullLoopStageSnapshot> {
        self.stages.query(stage, snapshot, viewport)
    }
}
