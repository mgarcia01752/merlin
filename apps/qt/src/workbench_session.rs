/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! One GUI-thread workbench's accepted scientific inputs, shared by typed snapshots.
use docsis_ofdm_fec::{
    profile::candidate::CapturedProfile, simulation::source::PayloadSourceConfig,
};
use std::cell::RefCell;

#[derive(Clone)]
pub(crate) struct AcceptedSource {
    pub config: PayloadSourceConfig,
    pub seed: u16,
}

#[derive(Default, Clone)]
pub(crate) struct Inputs {
    pub source: Option<AcceptedSource>,
    pub profile: Option<CapturedProfile>,
}

thread_local! { static INPUTS: RefCell<Inputs> = RefCell::new(Inputs::default()); }

/// Publishes an accepted immutable source on the Qt GUI thread.
/// # Arguments
/// * `source` - Validated configuration and seed; shared file bytes stay Rust-owned.
pub(crate) fn set_source(source: Option<AcceptedSource>) {
    INPUTS.with(|inputs| inputs.borrow_mut().source = source);
}

/// Publishes accepted profile selection/removal, never a draft.
/// # Arguments
/// * `profile` - Validated captured assignments, or None after applied removal.
pub(crate) fn set_profile(profile: Option<CapturedProfile>) {
    INPUTS.with(|inputs| inputs.borrow_mut().profile = profile);
}

/// Copies the current GUI-thread inputs for an independent immutable worker request.
/// # Returns
/// Typed snapshot; QML never receives source bytes or full assignment arrays.
pub(crate) fn snapshot() -> Inputs {
    INPUTS.with(|inputs| inputs.borrow().clone())
}
