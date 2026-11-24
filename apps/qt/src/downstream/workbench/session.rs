/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Explicit downstream accepted inputs, results and editor transfer ownership.
mod accepted;
use super::store::Store;
use crate::downstream::common::constants::DEFAULT_RESPONSE_ASSOCIATION_SECONDS;
use crate::shell::{ApplicationSessions, RetentionError};
pub(crate) use accepted::{Section, SectionSnapshot, Shared as AcceptedState, Snapshot};
use merlin::downstream::{
    AdditionalLoss, ChannelEstimate, MeasurementMode, PowerConditioning, ProfileCapture,
    RxMerFrame, SourceInput, SourceProfileFrame,
};
use std::sync::Arc;

/// Explicit downstream application owner; queries never follow tab selection.
#[derive(Clone)]
pub(crate) struct Context {
    /// Canonical two-direction identities and global dispatch for this workbench.
    pub(crate) application: crate::shell::ApplicationSessions,
    /// Matching immutable inputs/documents, admitted before their one owner swap.
    accepted: Store<Option<AcceptedState>>,
    /// Latest actual public result, independent of other contexts.
    pub(crate) result: Store<Option<Arc<crate::downstream::simulation::closed_loop::ResultData>>>,
    /// Detached bounded QAM history for this owner only.
    pub(crate) qam: Store<crate::downstream::instruments::receiver::qam_history::Registry>,
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
        let application =
            ApplicationSessions::desktop().expect("Application session identities unavailable");
        Self::for_application(application).expect("Application fixed Store admission unavailable")
    }

    /// Creates independent empty Stores on an existing canonical application host.
    /// # Arguments
    /// * `application` - Shared lifecycle host and exact accepting resource counter.
    /// # Returns
    /// Five separately charged empty Store allocations, without creating a host,
    /// advancing generations, changing readiness or cloning scientific payloads.
    /// Each Store covers its fixed pointee only; dynamic children remain separate.
    /// # Errors
    /// Rejects byte/root admission or accounting overflow. Any earlier Store
    /// allocations and reservations are released before returning the error.
    pub(crate) fn for_application(
        application: ApplicationSessions,
    ) -> Result<Self, RetentionError> {
        let budget = application.budget();
        let accepted = Store::new(None, budget)?;
        let result = Store::new(None, budget)?;
        let qam = Store::new(Default::default(), budget)?;
        let performance = Store::new(Vec::new(), budget)?;
        let prepared = Store::new(Default::default(), budget)?;
        Ok(Self {
            application,
            accepted,
            result,
            qam,
            performance,
            prepared,
        })
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

/// Prepares a coupled accepted input/document update before any editor commit.
///
/// # Arguments
/// * `context` - Exact downstream owner; selected tabs never redirect publication.
/// * `change` - Finite candidate edits; matching editor acceptance happens after success.
///
/// # Returns
/// Unit after the complete immutable owner is admitted and adopted.
///
/// # Errors
/// Preserves current inputs/documents/aliases on validation or aggregate failure.
pub(crate) fn update(
    context: &Context,
    change: impl FnOnce(&mut Inputs, &mut accepted::Sections) -> Result<(), String>,
) -> Result<(), String> {
    accepted::update(context, change)
}

/// Borrows current accepted inputs through their original charged immutable owner.
///
/// # Arguments
/// * `context` - Exact downstream owner.
///
/// # Returns
/// A shared read handle; replacing the accepted state cannot release old aliases.
pub(crate) fn snapshot(context: &Context) -> Snapshot {
    accepted::snapshot(context)
}

/// Reads bounded profile identities without copying assignments or measurements.
///
/// # Arguments
/// * `context` - Exact downstream owner.
///
/// # Returns
/// Sorted unique captured IDs and the currently applied ID, if any.
pub(crate) fn profile_catalog(context: &Context) -> (Vec<u8>, Option<u8>) {
    let inputs = snapshot(context);
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
}

/// Borrows an accepted block while preserving the original charged owner.
///
/// # Arguments
/// * `context` - Exact downstream owner.
/// * `key` - Existing static application block identifier.
///
/// # Returns
/// Immutable section or null before Apply; embedded originals are not cloned.
pub(crate) fn saved(context: &Context, key: &'static str) -> Section {
    accepted::section(context, key)
}

/// Borrows all matching accepted sections through the same charged owner.
///
/// # Arguments
/// * `context` - Exact downstream owner.
///
/// # Returns
/// Read handle; independently copied/queued documents require separate admission.
pub(crate) fn saved_sections(context: &Context) -> SectionSnapshot {
    accepted::sections(context)
}

/// Admits matching validated inputs/documents before target adoption.
///
/// # Arguments
/// * `context` - Exact accepting owner.
/// * `inputs`, `sections` - Finite completed candidates, preserving original bytes.
///
/// # Returns
/// Unit after one immutable owner swap.
///
/// # Errors
/// Metadata or aggregate rejection preserves the old state and aliases.
#[cfg(test)]
pub(crate) fn restore(
    context: &Context,
    inputs: Inputs,
    sections: accepted::Sections,
) -> Result<(), String> {
    let candidate = accepted::admit(inputs, sections, context.application.budget())?;
    *context.accepted.borrow_mut() = Some(candidate);
    *context.prepared.borrow_mut() = Default::default();
    Ok(())
}

/// Admits completed project preparation against its canonical target before queueing.
///
/// # Arguments
/// * `inputs`, `sections` - Matching validated candidates.
/// * `budget` - Actual accepting host counter, shared with accepted/old/queued owners.
///
/// # Returns
/// Charged immutable owner; no target Context is retained by this handle.
///
/// # Errors
/// Metadata or aggregate rejection before publication.
pub(crate) fn prepare_accepted(
    inputs: Inputs,
    sections: accepted::Sections,
    budget: &crate::shell::RetentionBudget,
) -> Result<AcceptedState, String> {
    accepted::admit(inputs, sections, budget)
}

/// Publishes a previously admitted exact-target state without copying its payloads.
///
/// # Arguments
/// * `context` - Exact accepting owner.
/// * `state` - Candidate carrying this host's canonical reservation.
/// * `expected` - Original charged accepted owner captured before loading.
///
/// # Returns
/// Unit after the infallible owner swap.
///
/// # Errors
/// Rejects a foreign budget or changed accepted owner before target mutation.
#[cfg(test)]
pub(crate) fn adopt_accepted(
    context: &Context,
    state: AcceptedState,
    expected: &Snapshot,
) -> Result<(), String> {
    accepted::adopt_with(context, state, expected, || Ok(()))
}

/// Adopts a target-admitted project with its final guarded readiness transition.
///
/// # Arguments
/// * `context`, `state`, `expected` - Exact target, replacement and captured original.
/// * `target` - Captured canonical downstream identity/generation.
/// * `ready` - Receiver readiness after all concrete target-root preflight succeeds.
///
/// # Returns
/// Genuine committed key. No scientific preparation or new data copy occurs here.
///
/// # Errors
/// Ownership/staleness or final metadata/revision failure preserves the old project.
pub(crate) fn adopt_project(
    context: &Context,
    state: AcceptedState,
    expected: &Snapshot,
    target: crate::shell::SessionKey,
    ready: bool,
) -> Result<crate::shell::SessionKey, String> {
    accepted::adopt_with(context, state, expected, || {
        context
            .application
            .configure_downstream_if(target, ready)
            .map_err(|error| match error {
                crate::shell::ApplicationError::Retention(_) => {
                    "The session retained-data limit would be exceeded.".into()
                }
                crate::shell::ApplicationError::Session(crate::shell::SessionError::StaleGuard) => {
                    "The prepared workbench target is stale.".into()
                }
                _ => "The workbench target cannot accept this project.".into(),
            })
    })
}

/// Reads the actual current accepted owner's test ledger, without estimating science.
///
/// # Arguments
/// * `context` - Exact fixture owner.
///
/// # Returns
/// Stored reservation only; capacity arithmetic is validated independently.
#[cfg(test)]
pub(crate) fn fixture_reserved_resources(context: &Context) -> crate::shell::RetainedResources {
    context
        .accepted
        .borrow()
        .as_ref()
        .map_or(Default::default(), |owner| {
            owner.fixture_reserved_resources()
        })
}

/// Measures an actual fixture candidate before moving it into the target budget.
///
/// # Arguments
/// * `inputs`, `sections` - The actual unchanged candidates subsequently moved to restore.
///
/// # Returns
/// Concrete owner quantities, not a clone or serialized-size estimate.
#[cfg(test)]
pub(crate) fn fixture_candidate_resources(
    inputs: &Inputs,
    sections: &accepted::Sections,
) -> crate::shell::RetainedResources {
    accepted::resources(inputs, sections).unwrap()
}

// Existing fixture publication helpers keep historical callsites readable. Runtime
// Apply uses the coupled transaction above; these helpers never accept an editor.
/// Publishes fixture source selection through the runtime transaction.
///
/// # Arguments
/// * `context, source` - Actual fixture target and validated data.
///
/// # Panics
/// Panics when the fixture host cannot admit its complete replacement.
#[cfg(test)]
pub(crate) fn set_source(context: &Context, source: Option<AcceptedSource>) {
    update(context, |inputs, _| {
        inputs.source = source;
        Ok(())
    })
    .unwrap();
}
/// Publishes fixture selected profile and original catalog through the runtime transaction.
///
/// # Arguments
/// * `context, profile, capture` - Actual fixture target and validated data.
///
/// # Panics
/// Panics when the fixture host cannot admit its complete replacement.
/// Publishes fixture saved original section through the runtime transaction.
///
/// # Arguments
/// * `context, key, value` - Actual fixture target and validated data.
///
/// # Panics
/// Panics when the fixture host cannot admit its complete replacement.
#[cfg(test)]
pub(crate) fn put_saved(context: &Context, key: &str, value: serde_json::Value) {
    update(context, |_, sections| {
        sections.insert(key.into(), value);
        Ok(())
    })
    .unwrap();
}

#[cfg(test)]
#[path = "session/fixed_tests.rs"]
mod fixed_tests;

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
        assert_eq!(snapshot(&left).source.as_ref().unwrap().seed, 17);
        assert_eq!(snapshot(&right).source.as_ref().unwrap().seed, 31);
        assert_eq!(saved(&left, "source")["seed"], 17);
        assert_eq!(saved(&right, "source")["seed"], 31);
        let shared = left.clone();
        set_source(&shared, None);
        assert!(snapshot(&left).source.is_none());
        assert_eq!(snapshot(&right).source.as_ref().unwrap().seed, 31);
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
        qam_history::begin(&left, std::slice::from_ref(&profile)).unwrap();
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
            let weak = target.accepted.downgrade();
            let project = serde_json::from_slice(PROJECT).unwrap();
            let prepared = super::super::project_load::PreparedProject::prepare(
                project,
                None,
                &target.application,
                target
                    .application
                    .snapshot(crate::shell::Direction::Downstream)
                    .key,
                snapshot(&target),
            )
            .unwrap();
            prepared.adopt(&target).unwrap();
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
