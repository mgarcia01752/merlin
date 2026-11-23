/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Explicit downstream accepted inputs, results and editor transfer ownership.
use super::store::Store;
use crate::downstream::common::constants::DEFAULT_RESPONSE_ASSOCIATION_SECONDS;
use merlin::downstream::{
    AdditionalLoss, ChannelEstimate, MeasurementMode, PowerConditioning, ProfileCapture,
    RxMerFrame, SourceInput, SourceProfileFrame,
};
use std::{collections::BTreeMap, sync::Arc};

/// Explicit downstream application owner; queries never follow tab selection.
#[derive(Clone)]
pub(crate) struct Context {
    /// Accepted scientific input snapshots for this downstream owner.
    pub(crate) inputs: Store<Inputs>,
    /// Exact accepted editor sections and original embedded bytes.
    pub(crate) saved: Store<BTreeMap<String, serde_json::Value>>,
    /// Latest actual public result, independent of other contexts.
    pub(crate) result: Store<Option<Arc<crate::downstream::simulation::closed_loop::ResultData>>>,
    /// Detached bounded QAM history for this owner only.
    pub(crate) qam: Store<Vec<crate::downstream::instruments::receiver::qam_history::Entry>>,
    /// Bounded per-profile performance history for this owner only.
    pub(crate) performance:
        Store<Vec<crate::downstream::instruments::receiver::performance::Entry>>,
    /// One independent pending editor transfer, consumed by this owner.
    pub(crate) prepared: Store<super::project_load::PreparedModels>,
}

thread_local! {
    // Fixed-direction construction only. Data operations always receive their context.
    static BOOTSTRAP_DOWNSTREAM: Context = Context::isolated();
}

impl Default for Context {
    /// Binds newly constructed legacy downstream adapters to their fixed owner.
    /// # Returns
    /// The GUI thread's downstream construction handle, never the selected tab.
    fn default() -> Self {
        BOOTSTRAP_DOWNSTREAM.with(Clone::clone)
    }
}

impl Context {
    /// Creates independent accepted storage and empty result/editor histories.
    /// # Returns
    /// A new downstream owner, suitable for isolated preparation and tests.
    pub(crate) fn isolated() -> Self {
        Self {
            inputs: Store::default(),
            saved: Store::default(),
            result: Store::default(),
            qam: Store::default(),
            performance: Store::default(),
            prepared: Store::default(),
        }
    }
}

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

/// Publishes an accepted immutable source on the Qt GUI thread.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `source` - Validated configuration and seed; shared file bytes stay Rust-owned.
pub(crate) fn set_source(context: &Context, source: Option<AcceptedSource>) {
    context
        .inputs
        .with(|inputs| inputs.borrow_mut().source = source);
}

/// Publishes accepted profile selection/removal, never a draft.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `profile`, `capture` - Selected assignments and exact original catalog, or removal.
pub(crate) fn set_profile(
    context: &Context,
    profile: Option<SourceProfileFrame>,
    capture: Option<ProfileCapture>,
) {
    context.inputs.with(|inputs| {
        let mut inputs = inputs.borrow_mut();
        inputs.profiles = profile.iter().cloned().collect();
        inputs.profile = profile;
        inputs.profile_capture = capture;
    });
}

/// Publishes all profiles from the accepted original capture, never editor drafts.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `profiles` - Validated captured profiles in source identity order.
pub(crate) fn set_profiles(context: &Context, profiles: Vec<SourceProfileFrame>) {
    context
        .inputs
        .with(|inputs| inputs.borrow_mut().profiles = profiles);
}

/// Copies the current GUI-thread inputs for an independent immutable worker request.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// # Returns
/// Typed snapshot; QML never receives source bytes or full assignment arrays.
pub(crate) fn snapshot(context: &Context) -> Inputs {
    context.inputs.with(|inputs| inputs.borrow().clone())
}

/// Reads bounded profile identities without cloning captured assignments or RxMER arrays.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// # Returns
/// Sorted unique captured IDs and the currently applied ID, if any.
pub(crate) fn profile_catalog(context: &Context) -> (Vec<u8>, Option<u8>) {
    context.inputs.with(|current| {
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
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `playback` - Original frames and explicit combination/schedule, or removal.
pub(crate) fn set_playback(context: &Context, playback: Option<PlaybackInputs>) {
    context
        .inputs
        .with(|inputs| inputs.borrow_mut().playback = playback);
}
/// Publishes accepted channel response H; None explicitly means synthetic unity.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `response` - Original validated response, never reciprocal correction.
/// * `tolerance` - Independent accepted response-association seconds.
pub(crate) fn set_response(context: &Context, response: Option<ChannelEstimate>, tolerance: u32) {
    context.inputs.with(|inputs| {
        let mut inputs = inputs.borrow_mut();
        inputs.response = response;
        inputs.response_tolerance = tolerance;
    });
}

/// Stores accepted editor state in Rust for workbench persistence.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `key` - Known block identifier.
/// * `value` - Accepted options and encoded original inputs, never a draft.
pub(crate) fn put_saved(context: &Context, key: &str, value: serde_json::Value) {
    context.saved.with(|saved| {
        saved.borrow_mut().insert(key.into(), value);
    });
}
/// Copies one accepted block document without passing original arrays through QML.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `key` - Known block identifier.
/// # Returns
/// The saved document, or null before explicit acceptance.
pub(crate) fn saved(context: &Context, key: &str) -> serde_json::Value {
    context.saved.with(|saved| {
        saved
            .borrow()
            .get(key)
            .cloned()
            .unwrap_or(serde_json::Value::Null)
    })
}
/// Copies all accepted block documents for a project file.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// # Returns
/// Rust-owned section map; inputs remain offline and self-contained.
pub(crate) fn saved_sections(
    context: &Context,
) -> std::collections::BTreeMap<String, serde_json::Value> {
    context.saved.with(|saved| saved.borrow().clone())
}
/// Replaces all accepted inputs and sections after complete project validation.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `inputs` - Independently validated scientific snapshot.
/// * `sections` - Matching original-byte editor documents.
pub(crate) fn restore(
    context: &Context,
    inputs: Inputs,
    sections: std::collections::BTreeMap<String, serde_json::Value>,
) {
    context
        .inputs
        .with(|current| *current.borrow_mut() = inputs);
    context.saved.with(|saved| *saved.borrow_mut() = sections);
}

/// Publishes the accepted synthetic magnitude response for the next frozen run.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `loss` - Validated additional loss; original measurements remain unchanged.
pub(crate) fn set_additional_loss(context: &Context, loss: AdditionalLoss) {
    context
        .inputs
        .with(|inputs| inputs.borrow_mut().additional_loss = loss);
}

/// Publishes validated amplifier, attenuation and electrical-reference settings.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `power` - Accepted configuration, independent of measurement originals.
pub(crate) fn set_power(context: &Context, power: PowerConditioning) {
    context
        .inputs
        .with(|inputs| inputs.borrow_mut().power = power);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::downstream::instruments::receiver::{qam_history, selection};
    use crate::downstream::simulation::closed_loop::{latest, ResultData};
    use merlin::downstream::{FullLoopControl, PrbsSeed};
    use serde_json::{json, Value};

    // Existing self-contained input fixture; no alternate scientific implementation.
    const PROJECT: &[u8] =
        include_bytes!("../../../../../examples/workbench/single/workbench.merlin.json");

    #[test]
    fn explicit_contexts_isolate_accepted_sources_sections_and_clone_ownership() {
        let left = Context::isolated();
        let right = Context::isolated();

        // Two distinct valid PRBS identities are independent isolation anchors.
        for (context, seed) in [(&left, 17), (&right, 31)] {
            set_source(
                context,
                Some(AcceptedSource {
                    config: SourceInput::prbs15(PrbsSeed::new(seed).unwrap()),
                    seed,
                }),
            );
            put_saved(context, "source", json!({"seed":seed}));
        }
        assert_eq!(snapshot(&left).source.unwrap().seed, 17);
        assert_eq!(snapshot(&right).source.unwrap().seed, 31);
        assert_eq!(saved(&left, "source")["seed"], 17);
        assert_eq!(saved(&right, "source")["seed"], 31);
        let shared = left.clone();
        set_source(&shared, None);
        assert!(snapshot(&left).source.is_none());
        assert_eq!(snapshot(&right).source.unwrap().seed, 31);
        assert!(saved_sections(&Context::isolated()).is_empty());
    }

    #[test]
    fn actual_results_and_archives_never_fall_back_to_another_context() {
        let left = Context::isolated();
        let right = Context::isolated();
        let mut project: Value = serde_json::from_slice(PROJECT).unwrap();
        // Existing one-millisecond receiver smoke window, not a runtime limit.
        project["receiver"]["duration_ms"] = json!(1);
        let plan = merlin::downstream::inputs::Workbench::prepare_full_loop(
            &serde_json::to_vec(&project).unwrap(),
        )
        .unwrap();
        let outcome = Arc::new(ResultData::new(
            plan.request(0)
                .unwrap()
                .run(&FullLoopControl::new())
                .unwrap(),
        ));
        let profile = outcome
            .job
            .settings
            .playback
            .as_ref()
            .unwrap()
            .profile
            .clone();
        qam_history::begin(&left, std::slice::from_ref(&profile));
        qam_history::retain(&left, profile.profile_id, &outcome).unwrap();
        *left.result.borrow_mut() = Some(outcome.clone());
        assert!(Arc::ptr_eq(&latest(&left).unwrap(), &outcome));
        assert!(latest(&right).is_none());
        assert!(qam_history::get(&left, i32::from(profile.profile_id)).is_ok());
        assert!(qam_history::get(&right, i32::from(profile.profile_id)).is_err());
        assert_eq!(
            selection::qam(&right, &Default::default())["available"],
            false
        );
        qam_history::clear(&right);
        assert!(qam_history::get(&left, i32::from(profile.profile_id)).is_ok());
    }

    #[test]
    fn prepared_adoption_is_targeted_and_unconsumed_models_never_form_owner_cycles() {
        // Prepare on the same thread as its bootstrap owner to expose accidental
        // target → prepared model → target cycles when that thread exits.
        let released = std::thread::spawn(|| {
            let target = Context::default();
            let other = Context::isolated();
            let weak = target.inputs.downgrade();
            let project = serde_json::from_slice(PROJECT).unwrap();
            let prepared =
                super::super::project_load::PreparedProject::prepare(project, None).unwrap();
            prepared.adopt(&target);
            assert!(snapshot(&target).source.is_some());
            assert!(snapshot(&other).source.is_none());
            assert!(
                super::super::project_load::take(&other, |models| models.source.take()).is_none()
            );
            assert!(
                super::super::project_load::take(&target, |models| models.source.take()).is_some()
            );

            // Leave the profile/receiver/transmitter transfer unconsumed deliberately.
            weak
        })
        .join()
        .unwrap();
        assert!(released.upgrade().is_none());
    }

    #[test]
    fn context_handles_transfer_owned_application_state_between_threads() {
        let context = Context::isolated();
        let worker = context.clone();
        std::thread::spawn(move || {
            put_saved(&worker, "source", json!({"name":"owned transfer"}));
        })
        .join()
        .unwrap();
        assert_eq!(saved(&context, "source")["name"], "owned transfer");
        assert!(saved(&Context::isolated(), "source").is_null());
    }
}
