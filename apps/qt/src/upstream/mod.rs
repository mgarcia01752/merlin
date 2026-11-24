/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Upstream application ownership over the existing public scientific session.
//!
//! UIU-SESSION-01 preparation for REQ-UIU-03/06/07, reusable by later UIU-US-02
//! and REQ-UIU-12; no task acceptance or Qt registration is established here.
//! The host supplies genuine shell identities, one shared dispatch
//! gate and explicit transport policy. Public metadata/report bounds now admit
//! scientific roots and concrete retained wrappers; outer application, control,
//! transport, DTO and budget owners still need separate host charges. This
//! unregistered component does not establish aggregate session acceptance.
mod session;

pub use session::{
    AdmissionError, ControllerError, ControllerEvent, PreparedRun, PreparedSession, RetainedReport,
    StartError, StartFailure, UpstreamControl, UpstreamController,
};
