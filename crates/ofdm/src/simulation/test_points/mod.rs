/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Borrowed typed observations and bounded optional dispatch; current pilot producers.
mod model;
mod observer;
pub use model::{
    ProbeContext, ProbeData, ProbeEvent, ProbeUnits, RunPhase, SignalDirection, SignalDomain,
    SignalReference, TestPointId,
};
pub use observer::{
    ProbeConfig, ProbeObserver, ProbeOutcome, ProbeSelection, ProbeStats, TestPointProbe,
};
