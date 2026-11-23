/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Upstream application ownership over the existing public scientific session.
//!
//! UIU-SESSION-01 preparation for REQ-UIU-03/06/07, reusable by later UIU-US-02
//! and REQ-UIU-12; no task acceptance or Qt registration is established here.
//! The host supplies genuine shell identities, one shared dispatch
//! gate, explicit transport policy, and independently audited reservations.
//! Backend public counts do not yet establish a deep reachable-allocation bound;
//! this module preserves charges but does not establish aggregate acceptance.
mod session;

pub use session::{
    ControllerError, ControllerEvent, PreparedSession, RetainedReport, StartError, StartFailure,
    UpstreamControl, UpstreamController,
};
