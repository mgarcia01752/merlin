/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Captured native downstream decisions; QML retains only compiled prompt tags.
use super::*;
use crate::{
    downstream::workbench::project_load::PreparedProject,
    shell::{Direction, ProjectSnapshot, ReplacePermission, RetainedResources},
};

pub(super) enum Intent {
    New(ProjectSnapshot),
    Open(Box<PreparedProject>),
}
impl Intent {
    fn guard(&self) -> ProjectSnapshot {
        match self {
            Self::New(guard) => *guard,
            Self::Open(candidate) => candidate.project_guard(),
        }
    }
}

impl WorkbenchFileRust {
    /// Queues the exact accepted sections and revision on the existing file worker.
    /// # Arguments
    /// * `destination` - Explicit Save As target, or the current direction's path.
    /// * `options` - Bounded actual accepted editor and observation settings.
    /// # Returns
    /// Loading metadata; the current destination and dirty state stay unchanged.
    /// # Errors
    /// Busy, invalid options, missing path or retention admission before dispatch.
    pub(super) fn queue_save(
        &mut self,
        destination: Option<PathBuf>,
        options: Vec<u8>,
    ) -> Result<serde_json::Value, String> {
        if self.loader.is_loading() {
            return Err("Wait for the workbench file action to finish.".into());
        }
        validate_save_packet(&options)?;
        let destination = destination
            .or_else(|| self.path.as_ref().map(|owner| owner.value().clone()))
            .ok_or("Choose a project destination.")?;
        let path = project_path::admit(
            normalize_destination(destination)?,
            self.context.application.budget(),
        )?;
        let reservation = self
            .context
            .application
            .budget()
            .reserve_resources(RetainedResources {
                payload_bytes: std::mem::size_of::<Retained<Vec<u8>>>()
                    .checked_add(options.capacity())
                    .ok_or("Save options ownership size overflowed.")?,
                ..Default::default()
            })
            .map_err(|_| "The session retained-data limit would be exceeded.".to_owned())?;
        let request = crate::downstream::workbench::project_load::SaveRequest {
            options: Box::new(reservation.retain(options)),
            sections: workbench_session::saved_sections(&self.context),
            path,
            expected: self.context.application.snapshot(Direction::Downstream),
            application: self.context.application.clone(),
        };
        Ok(self.loader.start_save(request))
    }

    /// Acknowledges a real completed write before adopting its destination.
    /// # Returns
    /// Saved path metadata, refreshing only an exactly matching pending guard.
    /// # Errors
    /// Missing completion or stale revision preserves the previous destination.
    pub(super) fn adopt_saved(&mut self) -> Result<serde_json::Value, String> {
        let saved = self
            .loader
            .take_saved()
            .ok_or("No project write is ready for adoption.")?;
        self.context
            .application
            .acknowledge_save(saved.value().expected)
            .map_err(|error| format!("{error:?}"))?;
        if let Some(intent) = self.intent.as_ref() {
            let expected = saved.value().expected;
            let current = self.context.application.snapshot(Direction::Downstream);
            match &mut *intent.value().borrow_mut() {
                Intent::New(guard) => {
                    if *guard == expected {
                        *guard = current;
                    }
                }
                Intent::Open(candidate) => candidate.refresh_saved_guard(expected, current),
            }
        }
        let (saved, _guard) = (*saved).into_parts();
        let result = json!({"saved":true,"path":saved.path.value().to_string_lossy()});
        self.path = Some(saved.path);
        Ok(result)
    }
    /// Captures the actual downstream revision before a New dirty decision.
    /// # Returns
    /// Compiled prompt metadata without mutating accepted sections or paths.
    /// # Errors
    /// An unresolved action or retention admission preserves the current project.
    pub(super) fn request_new(&mut self) -> Result<serde_json::Value, String> {
        if self.loader.is_loading() || self.intent.is_some() {
            return Err("Finish the pending workbench file action first.".into());
        }
        let guard = self.context.application.snapshot(Direction::Downstream);
        self.stage_intent(Intent::New(guard))
    }

    /// Retains a fully prepared native Open product before its dirty decision.
    /// # Arguments
    /// * `candidate` - Validated scientific/editor owners with their native guard.
    /// # Returns
    /// Compiled prompt metadata without adopting any partial state.
    /// # Errors
    /// Existing intent or retention rejection leaves accepted data unchanged.
    pub(super) fn stage_open(
        &mut self,
        candidate: PreparedProject,
    ) -> Result<serde_json::Value, String> {
        if self.intent.is_some() {
            return Err("Finish the pending downstream project decision first.".into());
        }
        self.stage_intent(Intent::Open(Box::new(candidate)))
    }

    fn stage_intent(&mut self, intent: Intent) -> Result<serde_json::Value, String> {
        let reservation = self
            .context
            .application
            .budget()
            .reserve_resources(RetainedResources {
                payload_bytes: std::mem::size_of::<Retained<std::cell::RefCell<Intent>>>(),
                ..Default::default()
            })
            .map_err(|_| "The session retained-data limit would be exceeded.".to_owned())?;
        self.intent = Some(Box::new(
            reservation.retain(std::cell::RefCell::new(intent)),
        ));
        Ok(self.decision())
    }

    /// Projects only compiled action and current dirty metadata for one prompt.
    /// # Returns
    /// A null decision when no native intent is pending; no native guard is copied.
    pub(super) fn decision(&self) -> serde_json::Value {
        let Some(intent) = self.intent.as_ref() else {
            return json!({"decision":null});
        };
        let current = self.context.application.snapshot(Direction::Downstream);
        json!({"downstream_decision":true,"decision":match &*intent.value().borrow() {
            Intent::New(_)=>"new",Intent::Open(_)=>"open",
        },"dirty":current.dirty})
    }

    /// Resolves the captured native target after the actual prompt closes.
    /// # Arguments
    /// * `discard`, `cancel` - Explicit outcomes; Cancel drops only the intent.
    /// # Returns
    /// Atomic adopted editor metadata, or cancellation without mutation.
    /// # Errors
    /// Stale target, unresolved dirty state or failed preflight preserves both tabs.
    pub(super) fn resolve_intent(
        &mut self,
        discard: bool,
        cancel: bool,
    ) -> Result<serde_json::Value, String> {
        let intent = self
            .intent
            .take()
            .ok_or("No downstream file decision is pending.")?;
        if cancel {
            return Ok(json!({"cancelled":true}));
        }
        let expected = intent.value().borrow().guard();
        let current = self.context.application.snapshot(Direction::Downstream);
        if current.key != expected.key || current.revision != expected.revision {
            return Err("The downstream project changed. Retry the file action.".into());
        }
        let permission = if discard {
            ReplacePermission::DiscardConfirmed
        } else {
            ReplacePermission::RequireClean
        };
        if current.dirty && permission == ReplacePermission::RequireClean {
            return Err("Save or discard the downstream project's changes first.".into());
        }
        let (intent, _guard) = (*intent).into_parts();
        let candidate = match intent.into_inner() {
            Intent::New(_) => PreparedProject::blank(&self.context)?,
            Intent::Open(candidate) => *candidate,
        };
        let adopted = candidate.adopt_with_permission(&self.context, permission)?;
        self.path = adopted.path;
        Ok(adopted.metadata.value().clone())
    }
}

/// Validates the bounded native options packet before retaining it for a worker.
/// # Arguments
/// * `bytes` - Exact UTF-8 packet containing independently bounded option strings.
/// # Returns
/// Unit without changing scientific settings or accepted state.
/// # Errors
/// Packet/field size violations or malformed/missing/unknown fields.
pub(super) fn validate_save_packet(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() > MAX_SAVE_OPTIONS_PACKET_BYTES {
        return Err("Project Save options exceed the packet byte limit.".into());
    }
    let options: crate::downstream::workbench::project_load::SaveOptions =
        serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    validate_project_options(&options.settings, &options.receiver)?;
    if options.observations.len() > MAX_OBSERVATION_OPTIONS_DOCUMENT_BYTES {
        return Err("Project observation options exceed the byte limit.".into());
    }
    Ok(())
}

#[cfg(test)]
#[path = "intent_tests.rs"]
mod tests;
