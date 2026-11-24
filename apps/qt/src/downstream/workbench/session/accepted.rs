/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! One immutable accepted input/document transaction, shared with bounded preparation.
use super::{Context, Inputs};
use crate::downstream::inputs::retention as storage;
use crate::shell::{Retained, RetainedResources, RetentionBudget};
use merlin::downstream::{ChannelEstimate, ProfileCapture, SourceInput};
use serde_json::Value;
use std::{borrow::Borrow, collections::BTreeMap, mem::size_of, ops::Deref, sync::Arc};

pub(crate) type Sections = BTreeMap<String, Value>;
pub(crate) type Shared = Arc<Retained<State>>;

// One actual allocation owns the admitted immutable state and its last reservation.
const ACCEPTED_SHARED_ROOTS: usize = 1;

#[derive(Clone, Default)]
pub(crate) struct State {
    /// Matching scientific/editor selections, preserving source originals.
    pub inputs: Inputs,
    /// Matching saved documents accepted in the same transaction.
    pub sections: Sections,
}

/// Read handle preserving its original accepted owner across replacement.
pub(crate) struct Snapshot {
    owner: Option<Shared>,

    // Untouched state has no allocated data; defaults remain inline in this temporary view.
    empty: State,
}

/// Section-map borrow sharing the same accepted owner as its input snapshot.
pub(crate) struct SectionSnapshot(Snapshot);

/// Single-section borrow; the identifier is an existing fixed application block name.
pub(crate) struct Section {
    snapshot: Snapshot,
    key: &'static str,
}

impl Snapshot {
    /// Borrows matching original-byte documents without cloning their payloads.
    ///
    /// # Returns
    /// The same immutable owner's section map, empty before the first acceptance.
    pub(crate) fn sections(&self) -> &Sections {
        &self.state().sections
    }

    fn state(&self) -> &State {
        self.owner
            .as_ref()
            .map_or(&self.empty, |owner| owner.value())
    }
}

impl Clone for Snapshot {
    /// Shares the original admitted owner without copying scientific inputs or JSON.
    ///
    /// # Returns
    /// Another handle; an untouched snapshot remains allocation-free.
    fn clone(&self) -> Self {
        Self {
            owner: self.owner.as_ref().map(Arc::clone),
            empty: State::default(),
        }
    }
}

impl Deref for Snapshot {
    type Target = Inputs;

    /// Borrows accepted inputs while keeping their original reservation alive.
    ///
    /// # Returns
    /// Immutable inputs; mutation requires a separately admitted replacement.
    fn deref(&self) -> &Inputs {
        &self.state().inputs
    }
}

impl Deref for SectionSnapshot {
    type Target = Sections;

    /// Borrows the matching saved map without producing an independent copy.
    ///
    /// # Returns
    /// Immutable original-byte sections tied to this shared snapshot.
    fn deref(&self) -> &Sections {
        self.0.sections()
    }
}

impl Borrow<Sections> for SectionSnapshot {
    /// Borrows the original saved sections through their admitted owner.
    ///
    /// # Returns
    /// The same immutable map retained by this handle.
    fn borrow(&self) -> &Sections {
        self
    }
}

impl serde::Serialize for SectionSnapshot {
    /// Serializes the unchanged map while preserving its source reservation.
    ///
    /// # Arguments
    /// * `serializer` - The existing project serializer; no wrapper fields are emitted.
    ///
    /// # Returns
    /// The map serialization result with the legacy project schema unchanged.
    ///
    /// # Errors
    /// Propagates the serializer's original errors.
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&**self, serializer)
    }
}

impl Deref for Section {
    type Target = Value;

    /// Borrows one exact accepted section, retaining its complete source owner.
    ///
    /// # Returns
    /// Existing section or the static null value for an unapplied block.
    fn deref(&self) -> &Value {
        self.snapshot
            .sections()
            .get(self.key)
            .unwrap_or(&Value::Null)
    }
}

/// Reads the exact current owner without cloning its scientific or document data.
///
/// # Arguments
/// * `context` - Explicit downstream storage, independent of selected tabs.
///
/// # Returns
/// Charged shared data or allocation-free untouched defaults.
pub(super) fn snapshot(context: &Context) -> Snapshot {
    Snapshot {
        owner: context.accepted.borrow().as_ref().map(Arc::clone),
        empty: State::default(),
    }
}

/// Reads saved sections through the same charged immutable owner.
///
/// # Arguments
/// * `context` - Exact downstream owner.
///
/// # Returns
/// A borrowed map view preserving old aliases after replacement.
pub(super) fn sections(context: &Context) -> SectionSnapshot {
    SectionSnapshot(snapshot(context))
}

/// Reads one fixed application block without copying embedded originals.
///
/// # Arguments
/// * `context` - Exact downstream owner.
/// * `key` - Existing static source/profile/measurement/conditioning block name.
///
/// # Returns
/// A shared accepted section view or null before Apply.
pub(super) fn section(context: &Context, key: &'static str) -> Section {
    Section {
        snapshot: snapshot(context),
        key,
    }
}

/// Prepares and admits a complete matching input/document replacement.
///
/// # Arguments
/// * `context` - Target owner; old snapshots remain charged throughout preparation.
/// * `change` - Finite candidate edits only; must not mutate accepted editor models
///   or re-enter this Context while its transaction lock is held.
///
/// # Returns
/// Unit after the one infallible owner swap.
///
/// # Errors
/// Preserves all old data and aliases on candidate, metadata or aggregate rejection.
pub(super) fn update(
    context: &Context,
    change: impl FnOnce(&mut Inputs, &mut Sections) -> Result<(), String>,
) -> Result<(), String> {
    let mut current = context.accepted.borrow_mut();
    let mut candidate = current
        .as_ref()
        .map_or_else(State::default, |owner| owner.value().clone());
    change(&mut candidate.inputs, &mut candidate.sections)?;
    let admitted = admit(
        candidate.inputs,
        candidate.sections,
        context.application.budget(),
    )?;
    *current = Some(admitted);
    // A successful edit invalidates unconsumed project models from the previous
    // accepted owner. Rejected candidates leave those transfers untouched.
    *context.prepared.borrow_mut() = Default::default();
    Ok(())
}

/// Admits actual completed preparation before shared allocation or queued retention.
///
/// # Arguments
/// * `inputs`, `sections` - Matching finite candidates, preserving original payloads.
/// * `budget` - Canonical accepting host; accepted and old source owners coexist.
///
/// # Returns
/// One immutable charged allocation ready for infallible target adoption.
///
/// # Errors
/// Rejects checked current-root metadata or aggregate limits without publishing data.
pub(super) fn admit(
    inputs: Inputs,
    sections: Sections,
    budget: &RetentionBudget,
) -> Result<Shared, String> {
    let state = State { inputs, sections };
    let reservation = budget
        .reserve_resources(resources(&state.inputs, &state.sections)?)
        .map_err(|_| "The session retained-data limit would be exceeded.".to_owned())?;
    Ok(Arc::new(reservation.retain(state)))
}

/// Commits the last fallible project operation while its original owner is locked.
///
/// # Arguments
/// * `ResultValue` - Small operation result, normally the genuine readiness key.
/// * `context` - Exact target; selected tabs do not redirect the operation.
/// * `candidate` - Already admitted complete replacement.
/// * `expected` - Charged original owner captured before asynchronous preparation.
/// * `commit` - Final fallible readiness transition. It must not re-enter accepted
///   storage or mutate target project/results/paths before succeeding.
///
/// # Returns
/// Operation result after one infallible accepted-owner swap.
///
/// # Errors
/// Foreign budget, intervening accepted edit or final readiness rejection preserves
/// current inputs/documents and their unconsumed prepared transfer.
pub(super) fn adopt_with<ResultValue>(
    context: &Context,
    candidate: Shared,
    expected: &Snapshot,
    commit: impl FnOnce() -> Result<ResultValue, String>,
) -> Result<ResultValue, String> {
    if !context.application.budget().owns(candidate.as_ref()) {
        return Err("Prepared inputs belong to another application host.".into());
    }
    let mut current = context.accepted.borrow_mut();
    let matches = match (current.as_ref(), expected.owner.as_ref()) {
        (None, None) => true,
        (Some(current), Some(expected)) => Arc::ptr_eq(current, expected),
        _ => false,
    };
    if !matches {
        return Err("The prepared workbench input state is stale.".into());
    }
    let result = commit()?;
    *current = Some(candidate);
    Ok(result)
}

/// Measures the actual complete input/document candidate without copying it.
///
/// # Arguments
/// * `inputs`, `sections` - Actual candidates, moved unchanged into the admitted root.
///
/// # Returns
/// Full wrapper bytes plus actual child capacity, map entries and shared roots.
///
/// # Errors
/// Checked arithmetic or public metadata failure before admission.
pub(super) fn resources(inputs: &Inputs, sections: &Sections) -> Result<RetainedResources, String> {
    let mut result = RetainedResources {
        payload_bytes: size_of::<Retained<State>>(),
        ordered_entries: 0,
        shared_roots: ACCEPTED_SHARED_ROOTS,
        ..Default::default()
    };
    let Inputs {
        power,
        source,
        profile,
        profiles,
        profile_capture,
        playback,
        response,
        response_tolerance,
        additional_loss,
    } = inputs;
    // Exhaustive public field/variant audits: these owners currently have only
    // fixed Copy leaves, already included in State's inline layout.
    let merlin::downstream::PowerConditioning {
        amplifier_gain_db,
        cpa_attenuation_db,
        input_data_level_dbmv_per_6mhz,
        amplifier_target_dbmv_per_6mhz,
        receiver_noise_dbmv_hz,
    } = power;
    storage::append(
        &mut result,
        storage::fixed_children(&(
            *amplifier_gain_db,
            *cpa_attenuation_db,
            *input_data_level_dbmv_per_6mhz,
            *amplifier_target_dbmv_per_6mhz,
            *receiver_noise_dbmv_hz,
        )),
    )?;
    match additional_loss {
        merlin::downstream::AdditionalLoss::Off => {}
        merlin::downstream::AdditionalLoss::Manual { tilt_db } => {
            storage::append(&mut result, storage::fixed_children(tilt_db))?;
        }
        merlin::downstream::AdditionalLoss::Cable {
            cable,
            length,
            unit,
        } => {
            storage::append(
                &mut result,
                storage::fixed_children(&(*cable, *length, *unit)),
            )?;
        }
    }
    storage::append(&mut result, storage::fixed_children(response_tolerance))?;
    if let Some(super::AcceptedSource { config, seed }) = source {
        storage::append(&mut result, storage::fixed_children(seed))?;
        storage::append(
            &mut result,
            storage::public_children(
                config
                    .retained_footprint()
                    .map_err(|error| error.to_string())?,
                size_of::<SourceInput>(),
            )?,
        )?;
    }
    if let Some(profile) = profile {
        storage::append(&mut result, storage::profile_children(profile)?)?;
    }
    storage::append(&mut result, storage::vector_storage(profiles)?)?;
    for profile in profiles {
        storage::append(&mut result, storage::profile_children(profile)?)?;
    }
    if let Some(capture) = profile_capture {
        storage::append(
            &mut result,
            storage::public_children(
                capture
                    .retained_footprint()
                    .map_err(|error| error.to_string())?,
                size_of::<ProfileCapture>(),
            )?,
        )?;
    }
    if let Some(playback) = playback {
        let super::PlaybackInputs {
            frames,
            mode,
            interval_symbols,
            tolerance_seconds,
        } = playback;
        storage::append(
            &mut result,
            storage::fixed_children(&(*mode, *interval_symbols, *tolerance_seconds)),
        )?;
        storage::append(&mut result, storage::vector_storage(frames)?)?;
        for frame in &playback.frames {
            storage::append(&mut result, storage::rxmer_frame_children(frame)?)?;
        }
    }
    if let Some(response) = response {
        storage::append(
            &mut result,
            storage::public_children(
                response
                    .retained_footprint()
                    .map_err(|error| error.to_string())?,
                size_of::<ChannelEstimate>(),
            )?,
        )?;
    }
    let map_storage = sections
        .len()
        .checked_mul(size_of::<(String, Value)>())
        .ok_or("Accepted section storage arithmetic overflow.")?;
    storage::append(
        &mut result,
        RetainedResources {
            payload_bytes: map_storage,
            ordered_entries: sections.len(),
            shared_roots: 0,
            ..Default::default()
        },
    )?;
    for (key, value) in sections {
        storage::append(
            &mut result,
            RetainedResources {
                payload_bytes: key.capacity(),
                ..Default::default()
            },
        )?;
        storage::append(&mut result, storage::json_children(value)?)?;
    }
    Ok(result)
}

#[cfg(test)]
#[path = "accepted/tests.rs"]
mod tests;
