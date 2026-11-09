/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Borrowed typed observations and bounded optional dispatch; pilot and PLC interleaving producers.
mod model;
mod observer;
pub use model::{
    CellProbeGrid, PhysicalProbeGrid, PlcProbeGrid, ProbeContext, ProbeData, ProbeEvent,
    ProbeUnits, RunPhase, SignalDirection, SignalDomain, SignalReference, TestPointId,
    WaveformProbeGrid,
};
pub use observer::{
    ProbeConfig, ProbeObserver, ProbeOutcome, ProbeSelection, ProbeStats, TestPointProbe,
};

pub mod export;
pub use export::JsonlProbeExporter;

/// Sink for disabled probes; has no scientific state or output side effects.
pub struct NoopObserver;
impl ProbeObserver for NoopObserver {
    /// Ignores an immutable event without allocating or changing scientific data.
    /// # Arguments
    /// * `_event` - Optional read-only selected event.
    /// # Returns
    /// Unit; observation is deliberately discarded.
    fn observe(&mut self, _event: &ProbeEvent<'_>) -> Result<(), crate::LabError> {
        Ok(())
    }
}
