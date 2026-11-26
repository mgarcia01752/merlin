/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Upstream application ownership over the existing public scientific session.
//!
//! UIU-SESSION-01 preparation for REQ-UIU-03/06/07, reusable by later UIU-US-02
//! and REQ-UIU-12; the registered bridge delegates to the workbench model.
//! The host supplies genuine shell identities, one shared dispatch
//! gate and explicit transport policy. Public metadata/report bounds now admit
//! scientific roots, concrete retained wrappers and scientific control adapters.
//! The canonical host admits its fixed stores/counter/gate; outer controller,
//! preparation, worker/transport and presentation owners remain separate work. This
//! component and its QObject registration do not establish aggregate acceptance.
mod session;
pub(crate) use session::ProjectSource;
pub(crate) mod workbench;

pub use session::{
    AdmissionError, ControllerError, ControllerEvent, PreparedRun, PreparedSession, RetainedReport,
    RunFailure, StartError, StartFailure, UpstreamControl, UpstreamController,
};
