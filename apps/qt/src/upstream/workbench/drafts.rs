/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Correlated configuration edits over the existing immutable upstream owner.
mod constants;
mod fields;
pub(super) use constants::MEASURED_READ_ONLY_MESSAGE;
use constants::{DRAFT_SHARED_ROOTS, FIRST_DRAFT_ID};
pub(crate) use constants::{MAX_DRAFT_CHANGE_BYTES, MAX_DRAFT_ID_CHARACTERS};

use super::{Model, Pending};
use crate::shell::{Direction, Retained, RetentionBudget, SessionKey};
use crate::upstream::{PreparedSession, UpstreamController};
use merlin::upstream::{ReceiverObservationPolicy, Simulation};
use serde_json::{json, Value};
use std::{
    mem::size_of,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

// Zero denotes an unissued draft. Process-wide tokens never wrap or reuse.
static NEXT_DRAFT_ID: AtomicU64 = AtomicU64::new(FIRST_DRAFT_ID);

pub(super) type SharedDraft = Arc<Retained<Draft>>;

pub(super) struct Draft {
    id: u64,
    pub(super) base: SessionKey,
    section: &'static str,
    fields: fields::Fields,
    has_source: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) enum Outcome {
    #[default]
    Editing,
    Pending,
    Accepted,
    Rejected,
    Cancelled,
    Stale,
}

impl Outcome {
    fn name(self) -> &'static str {
        match self {
            Self::Editing => "editing",
            Self::Pending => "pending",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Cancelled => "cancelled",
            Self::Stale => "stale",
        }
    }
}

impl Draft {
    fn read_only(&self) -> bool {
        self.fields.presentation["options"]["read_only"] == true
    }

    /// Formats this exact native draft as lossless opaque text.
    /// # Returns
    /// A process-unique token; QML must retain it without parsing or alteration.
    pub(super) fn token(&self) -> String {
        format!("upstream-draft/{}", self.id)
    }

    /// Merges changed fields into a temporary typed candidate off the GUI thread.
    /// # Arguments
    /// * `simulation` - Borrowed accepted source, held by its original controller guard.
    /// * `changes` - Separately admitted changed strings/ordered selections.
    /// * `budget` - Existing aggregate counter shared by old and candidate owners.
    /// # Returns
    /// A fully public-API-validated, admitted immutable candidate; no run is started.
    /// # Errors
    /// Missing source, bad field/number/choice, scientific validation or admission.
    /// Imported original bytes remain shared and unchanged through the typed clone.
    pub(super) fn prepare(
        &self,
        simulation: &Simulation,
        changes: &Value,
        budget: &RetentionBudget,
    ) -> Result<PreparedSession, String> {
        let mut document = simulation
            .document()
            .ok_or("Load a source configuration before applying edits.")?
            .clone();
        fields::merge(&mut document, &self.fields, changes)?;
        PreparedSession::from_document(document, ReceiverObservationPolicy::default(), budget)
            .map_err(|error| format!("{error:?}"))
    }
}

impl Model {
    /// Opens an isolated display draft without changing either accepted direction.
    /// # Arguments
    /// * `section` - One existing upstream configuration window.
    /// # Returns
    /// Unit after publishing a fresh token/base and actual native values.
    /// An unconfigured owner supplies no invented scientific defaults.
    /// # Errors
    /// Unknown section, owned preparation/run, exhausted identity or DTO admission.
    pub(crate) fn begin_draft(&mut self, section: &str) -> Result<(), String> {
        let section = section_tag(section).ok_or("Unknown upstream configuration section.")?;
        if self.pending.is_some() || self.is_running() {
            return Err(
                "Wait for upstream work before opening another configuration draft.".into(),
            );
        }
        let measured = self.current()
            && self
                .controller
                .as_ref()
                .is_some_and(UpstreamController::has_measured_input);
        let fields = if measured {
            fields::Fields {
                bindings: Vec::new(),
                presentation: json!({"section":section,"options":{"fields":[],"read_only":true},
                    "values":{},"summary":MEASURED_READ_ONLY_MESSAGE}),
            }
        } else if self.current() {
            self.controller
                .as_ref()
                .ok_or("Upstream source is unavailable.")?
                .inspect(|simulation| fields::build(simulation, section))
                .map_err(|error| format!("{error:?}"))??
        } else {
            fields::Fields {
                bindings: Vec::new(),
                presentation: json!({"options":{"fields":[]},"values":{}}),
            }
        };
        let id = NEXT_DRAFT_ID
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next| {
                next.checked_add(1)
            })
            .map_err(|_| "Upstream draft identity space is exhausted.")?;
        let data = Draft {
            id,
            base: self.application.snapshot(Direction::Upstream).key,
            section,
            fields,
            has_source: self.current(),
        };
        let mut resources =
            crate::downstream::inputs::retention::json_children(&data.fields.presentation)?;
        resources.payload_bytes = resources
            .payload_bytes
            .checked_add(size_of::<Retained<Draft>>())
            .and_then(|bytes| {
                data.fields
                    .bindings
                    .capacity()
                    .checked_mul(size_of::<fields::Binding>())
                    .and_then(|bindings| bytes.checked_add(bindings))
            })
            .ok_or("Upstream draft ownership size overflowed.")?;
        resources.shared_roots = resources
            .shared_roots
            .checked_add(DRAFT_SHARED_ROOTS)
            .ok_or("Upstream draft root count overflowed.")?;
        let reservation = self
            .application
            .budget()
            .reserve_resources(resources)
            .map_err(|error| format!("{error:?}"))?;
        self.draft = Some(Arc::new(reservation.retain(data)));
        self.draft_outcome = Outcome::Editing;
        self.error = Default::default();
        Ok(())
    }

    /// Queues a changed-only edit for its exact original draft/base.
    /// # Arguments
    /// * `section`, `token` - Original captured identifiers; never retargeted.
    /// * `changes` - Bounded JSON object of native field keys to exact strings/arrays.
    /// # Returns
    /// Unit means queued only; Poll reports accepted after genuine final adoption.
    /// # Errors
    /// Wrong/stale identity, Busy, malformed changes, resource or spawn failure.
    /// Existing configuration, PNM lineage and results survive every rejection.
    pub(crate) fn apply_draft(
        &mut self,
        section: &str,
        token: &str,
        changes: &str,
    ) -> Result<(), String> {
        let matches = self.draft.as_ref().is_some_and(|draft| {
            draft.value().section == section && draft.value().token() == token
        });
        if !matches {
            return Err("This upstream configuration draft is no longer current.".into());
        }
        let result = self.queue_draft(changes);
        if let Err(error) = &result {
            self.draft_outcome = if self.draft.as_ref().unwrap().value().base
                != self.application.snapshot(Direction::Upstream).key
            {
                Outcome::Stale
            } else {
                Outcome::Rejected
            };
            self.error = self.admit_error(error.clone());
        }
        result
    }

    /// Records a transport refusal only for the exact currently submitted draft.
    /// # Arguments
    /// * `section`, `token` - Bounded original identifiers checked before copying input.
    /// * `message` - Static adapter diagnostic; no unadmitted request text is retained.
    /// # Returns
    /// Unit without changing a foreign draft, accepted source or pending operation.
    pub(crate) fn reject_draft_request(
        &mut self,
        section: &str,
        token: &str,
        message: &'static str,
    ) {
        if self
            .draft
            .as_ref()
            .is_some_and(|draft| draft.value().section == section && draft.value().token() == token)
            && !matches!(
                self.draft_outcome,
                Outcome::Pending | Outcome::Accepted | Outcome::Cancelled
            )
        {
            self.draft_outcome = Outcome::Rejected;
            self.error = crate::shared::worker_failure::WorkerFailure::Static(message);
        }
    }

    fn queue_draft(&mut self, changes: &str) -> Result<(), String> {
        self.ensure_configuration_idle()?;
        let draft = self
            .draft
            .as_ref()
            .ok_or("Open a configuration draft first.")?;
        if !draft.value().has_source || !self.current() {
            return Err("Load a supported upstream source configuration first.".into());
        }
        if draft.value().read_only() {
            return Err(MEASURED_READ_ONLY_MESSAGE.into());
        }
        if draft.value().base != self.application.snapshot(Direction::Upstream).key {
            return Err("Upstream configuration changed. Close and reopen this draft.".into());
        }
        if !matches!(self.draft_outcome, Outcome::Editing | Outcome::Rejected) {
            return Err("This upstream draft has already ended.".into());
        }
        if changes.len() > MAX_DRAFT_CHANGE_BYTES {
            return Err("Upstream draft changes exceed the input byte limit.".into());
        }
        let changes: Value = serde_json::from_str(changes).map_err(|error| error.to_string())?;
        if changes.as_object().is_none_or(|values| values.is_empty()) {
            return Err("Supply at least one changed upstream field.".into());
        }
        let mut resources = crate::downstream::inputs::retention::json_children(&changes)?;
        resources.payload_bytes = resources
            .payload_bytes
            .checked_add(size_of::<Retained<Value>>())
            .ok_or("Upstream draft change ownership size overflowed.")?;
        let changes = Box::new(
            self.application
                .budget()
                .reserve_resources(resources)
                .map_err(|error| format!("{error:?}"))?
                .retain(changes),
        );
        let draft = Arc::clone(draft);
        let controller = self
            .controller
            .take()
            .ok_or("Upstream source is unavailable.")?;
        match Pending::edit(
            controller,
            self.application.budget().clone(),
            draft,
            changes,
        ) {
            Ok(pending) => self.pending = Some(pending),
            Err((controller, error)) => {
                self.controller = Some(controller);
                return Err(error);
            }
        }
        self.preparation_cancelled = false;
        self.draft_outcome = Outcome::Pending;
        self.error = Default::default();
        Ok(())
    }

    /// Cancels only a matching configuration draft, never a simulation or import.
    /// # Arguments
    /// * `token` - Exact original draft token, independent of selected tab.
    /// # Returns
    /// Unit after discarding local edits or suppressing their queued adoption.
    /// # Errors
    /// Foreign tokens are ignored with an error and cannot change a newer draft.
    pub(crate) fn cancel_draft(&mut self, token: &str) -> Result<(), String> {
        let draft = self
            .draft
            .as_ref()
            .ok_or("No upstream configuration draft is open.")?;
        if draft.value().token() != token {
            return Err("This upstream configuration draft is no longer current.".into());
        }
        if self.draft_outcome == Outcome::Accepted {
            return Ok(());
        }
        if self
            .pending
            .as_ref()
            .is_some_and(|pending| pending.is_edit(draft))
        {
            self.preparation_cancelled = true;
        }
        self.draft_outcome = Outcome::Cancelled;
        Ok(())
    }

    /// Projects one admitted native draft without mutating the accepted source.
    /// # Returns
    /// A finite transient DTO; QML admits its own persistent copy independently.
    pub(super) fn draft_state(&self) -> Value {
        let Some(draft) = self.draft.as_ref().map(|draft| draft.value()) else {
            return Value::Null;
        };
        let mut state = draft.fields.presentation.clone();
        let outcome = if matches!(self.draft_outcome, Outcome::Editing | Outcome::Rejected)
            && draft.base != self.application.snapshot(Direction::Upstream).key
        {
            Outcome::Stale
        } else {
            self.draft_outcome
        };
        let pending = self
            .pending
            .as_ref()
            .is_some_and(|pending| pending.is_edit(self.draft.as_ref().unwrap()));
        let object = state
            .as_object_mut()
            .expect("Native fields supply an object");
        object.extend(json!({"section":draft.section,"token":draft.token(),
            "baseSession":draft.base.session.to_string(),"baseGeneration":draft.base.generation.to_string(),
            "can_apply":draft.has_source && !draft.read_only() && matches!(outcome, Outcome::Editing | Outcome::Rejected)
                && !pending && self.application.active().is_none(),
            "pending":pending,"outcome":outcome.name(),
            "error":if draft.read_only() { MEASURED_READ_ONLY_MESSAGE } else if draft.has_source { self.error.message() } else { "Load a supported upstream configuration document to supply these fields." }
        }).as_object().unwrap().clone());
        state
    }
}

fn section_tag(section: &str) -> Option<&'static str> {
    [
        "ucd",
        "profiles",
        "grants",
        "source",
        "receiver",
        "channel",
        "preEqualization",
    ]
    .into_iter()
    .find(|candidate| *candidate == section)
}

#[cfg(test)]
mod tests;
