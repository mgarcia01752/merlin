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
    pub playback:
        Option<docsis_ofdm_fec::simulation::channel::measurement_playback::PlaybackConfiguration>,
    pub response: Option<docsis_ofdm_fec::scenario::ChannelFrame>,
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

/// Publishes accepted RxMER playback inputs for independent worker snapshots.
/// # Arguments
/// * `playback` - Original frames and explicit combination/schedule, or removal.
pub(crate) fn set_playback(
    playback: Option<
        docsis_ofdm_fec::simulation::channel::measurement_playback::PlaybackConfiguration,
    >,
) {
    INPUTS.with(|inputs| inputs.borrow_mut().playback = playback);
}
/// Publishes accepted channel response H; None explicitly means synthetic unity.
/// # Arguments
/// * `response` - Original validated response, never reciprocal correction.
pub(crate) fn set_response(response: Option<docsis_ofdm_fec::scenario::ChannelFrame>) {
    INPUTS.with(|inputs| inputs.borrow_mut().response = response);
}

thread_local! { static SAVED: RefCell<std::collections::BTreeMap<String,serde_json::Value>> = RefCell::new(Default::default()); }
/// Stores accepted editor state in Rust for workbench persistence.
/// # Arguments
/// * `key` - Known block identifier.
/// * `value` - Accepted options and encoded original inputs, never a draft.
pub(crate) fn put_saved(key: &str, value: serde_json::Value) {
    SAVED.with(|saved| {
        saved.borrow_mut().insert(key.into(), value);
    });
}
/// Copies one accepted block document without passing original arrays through QML.
/// # Arguments
/// * `key` - Known block identifier.
/// # Returns
/// The saved document, or null before explicit acceptance.
pub(crate) fn saved(key: &str) -> serde_json::Value {
    SAVED.with(|saved| {
        saved
            .borrow()
            .get(key)
            .cloned()
            .unwrap_or(serde_json::Value::Null)
    })
}
/// Copies all accepted block documents for a project file.
/// # Returns
/// Rust-owned section map; inputs remain offline and self-contained.
pub(crate) fn saved_sections() -> std::collections::BTreeMap<String, serde_json::Value> {
    SAVED.with(|saved| saved.borrow().clone())
}
/// Replaces all accepted inputs and sections after complete project validation.
/// # Arguments
/// * `inputs` - Independently validated scientific snapshot.
/// * `sections` - Matching original-byte editor documents.
pub(crate) fn restore(
    inputs: Inputs,
    sections: std::collections::BTreeMap<String, serde_json::Value>,
) {
    INPUTS.with(|current| *current.borrow_mut() = inputs);
    SAVED.with(|saved| *saved.borrow_mut() = sections);
}
