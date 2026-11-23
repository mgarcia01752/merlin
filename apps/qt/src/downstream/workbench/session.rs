/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! One GUI-thread workbench's accepted scientific inputs, shared by typed snapshots.
use crate::downstream::common::constants::DEFAULT_RESPONSE_ASSOCIATION_SECONDS;
use merlin::downstream::{
    AdditionalLoss, ChannelEstimate, MeasurementMode, PowerConditioning, ProfileCapture,
    RxMerFrame, SourceInput, SourceProfileFrame,
};
use std::cell::RefCell;

#[derive(Clone)]
pub(crate) struct AcceptedSource {
    pub config: SourceInput,
    pub seed: u16,
}

/// Passive accepted editor selection; scientific admission belongs to MeasurementSequence.
/// Incomplete saved projects retain their exact frames without claiming run readiness.
#[derive(Clone)]
pub(crate) struct PlaybackInputs {
    pub frames: Vec<RxMerFrame>,
    pub mode: MeasurementMode,
    pub interval_symbols: u64,
    pub tolerance_seconds: u32,
}

#[derive(Clone)]
pub(crate) struct Inputs {
    pub power: PowerConditioning,
    pub source: Option<AcceptedSource>,
    pub profile: Option<SourceProfileFrame>,
    pub profiles: Vec<SourceProfileFrame>,
    pub profile_capture: Option<ProfileCapture>,
    pub playback: Option<PlaybackInputs>,
    pub response: Option<ChannelEstimate>,
    pub response_tolerance: u32,
    pub additional_loss: AdditionalLoss,
}

impl Default for Inputs {
    /// Creates untouched editor storage without admitting a scientific session.
    /// # Returns
    /// Independent empty inputs and the existing response-association default.
    fn default() -> Self {
        Self {
            power: PowerConditioning::default(),
            source: None,
            profile: None,
            profiles: Vec::new(),
            profile_capture: None,
            playback: None,
            response: None,
            response_tolerance: DEFAULT_RESPONSE_ASSOCIATION_SECONDS,
            additional_loss: AdditionalLoss::default(),
        }
    }
}

/// Checks that the selected editor DTO still corresponds to its exact original catalog.
/// # Arguments
/// * `profile`, `capture` - Selected snapshot and retained admitted original.
/// # Returns
/// Original catalog and byte-sized identity, without simulation-ID coercion.
/// # Errors
/// Rejects missing or stale source snapshots; scientific compatibility stays in the API.
pub(crate) fn profile_source<'input>(
    profile: Option<&SourceProfileFrame>,
    capture: Option<&'input ProfileCapture>,
) -> Result<(&'input ProfileCapture, u8), String> {
    let profile = profile.ok_or("Apply a compatible DPD before accepting RxMER.")?;
    let capture = capture.ok_or("Apply the original modulation-profile capture.")?;
    let original = capture
        .frame(profile.profile_id)
        .map_err(|cause| cause.message().to_owned())?;
    if serde_json::to_value(original).map_err(|cause| cause.to_string())?
        != serde_json::to_value(profile).map_err(|cause| cause.to_string())?
    {
        return Err("Accepted profile differs from its original editor capture.".into());
    }
    Ok((capture, profile.profile_id))
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
/// * `profile`, `capture` - Selected assignments and exact original catalog, or removal.
pub(crate) fn set_profile(profile: Option<SourceProfileFrame>, capture: Option<ProfileCapture>) {
    INPUTS.with(|inputs| {
        let mut inputs = inputs.borrow_mut();
        inputs.profiles = profile.iter().cloned().collect();
        inputs.profile = profile;
        inputs.profile_capture = capture;
    });
}

/// Publishes all profiles from the accepted original capture, never editor drafts.
/// # Arguments
/// * `profiles` - Validated captured profiles in source identity order.
pub(crate) fn set_profiles(profiles: Vec<SourceProfileFrame>) {
    INPUTS.with(|inputs| inputs.borrow_mut().profiles = profiles);
}

/// Copies the current GUI-thread inputs for an independent immutable worker request.
/// # Returns
/// Typed snapshot; QML never receives source bytes or full assignment arrays.
pub(crate) fn snapshot() -> Inputs {
    INPUTS.with(|inputs| inputs.borrow().clone())
}

/// Reads bounded profile identities without cloning captured assignments or RxMER arrays.
/// # Returns
/// Sorted unique captured IDs and the currently applied ID, if any.
pub(crate) fn profile_catalog() -> (Vec<u8>, Option<u8>) {
    INPUTS.with(|current| {
        let inputs = current.borrow();
        let mut ids = inputs
            .profiles
            .iter()
            .chain(inputs.profile.iter())
            .map(|profile| profile.profile_id)
            .collect::<Vec<_>>();
        ids.sort_unstable();
        ids.dedup();
        (
            ids,
            inputs.profile.as_ref().map(|profile| profile.profile_id),
        )
    })
}

/// Publishes accepted RxMER playback inputs for independent worker snapshots.
/// # Arguments
/// * `playback` - Original frames and explicit combination/schedule, or removal.
pub(crate) fn set_playback(playback: Option<PlaybackInputs>) {
    INPUTS.with(|inputs| inputs.borrow_mut().playback = playback);
}
/// Publishes accepted channel response H; None explicitly means synthetic unity.
/// # Arguments
/// * `response` - Original validated response, never reciprocal correction.
/// * `tolerance` - Independent accepted response-association seconds.
pub(crate) fn set_response(response: Option<ChannelEstimate>, tolerance: u32) {
    INPUTS.with(|inputs| {
        let mut inputs = inputs.borrow_mut();
        inputs.response = response;
        inputs.response_tolerance = tolerance;
    });
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

/// Publishes the accepted synthetic magnitude response for the next frozen run.
/// # Arguments
/// * `loss` - Validated additional loss; original measurements remain unchanged.
pub(crate) fn set_additional_loss(loss: AdditionalLoss) {
    INPUTS.with(|inputs| inputs.borrow_mut().additional_loss = loss);
}

/// Publishes validated amplifier, attenuation and electrical-reference settings.
/// # Arguments
/// * `power` - Accepted configuration, independent of measurement originals.
pub(crate) fn set_power(power: PowerConditioning) {
    INPUTS.with(|inputs| inputs.borrow_mut().power = power);
}
