/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Direction-specific Qt ownership over the existing public upstream controller.
mod acquisition;
mod configuration_view;
mod drafts;
mod preparation;
mod presentation;
pub(crate) use drafts::{MAX_DRAFT_CHANGE_BYTES, MAX_DRAFT_ID_CHARACTERS};

use crate::shared::{
    presentation::constants::WORKER_FAILURE_ADMISSION_MESSAGE, worker_failure::WorkerFailure,
};
use crate::shell::{
    ApplicationSessions, Direction, Retained, RetainedResources, RetentionError, RunToken,
    SessionKey, WorkerStatus,
};
use crate::upstream::{
    ControllerEvent, PreparedRun, RetainedReport, UpstreamControl, UpstreamController,
};
use merlin::upstream::{RunControl, RunStatus};
use preparation::{Pending, Prepared};

// Application input and UI diagnostic policies; neither is a normative PHY limit.
pub(crate) const MAX_CONFIGURATION_BYTES: usize = 16 * 1024 * 1024;
const MAX_ERROR_CHARACTERS: usize = 4096;

// Report data and one terminal have separate identities and share bounded transport.
const RUN_COMPLETION_CAPACITY: usize = 2;

/// GUI-thread upstream owner; preparations and execution never block Qt polling.
pub(crate) struct Model {
    application: ApplicationSessions,
    controller: Option<UpstreamController>,
    pending: Option<Pending>,
    preparation_cancelled: bool,
    key: Option<SessionKey>,
    report: Option<(RunToken, RetainedReport)>,
    error: WorkerFailure,
    draft: Option<drafts::SharedDraft>,
    draft_outcome: drafts::Outcome,

    // Drop controllers, pending slots and data aliases before this containing charge.
    _fixed_guard: Retained<()>,
}

impl Model {
    /// Admits an idle upstream model's actual layout before final Box allocation.
    /// # Arguments
    /// * `application` - Actual shared host, including its canonical budget.
    /// # Returns
    /// Independent mutable state with a last nonoptional fixed-layout witness.
    /// Inline controller/Pending/Active storage counts once, without scientific copies.
    /// # Errors
    /// Fixed byte admission failure before allocation, readiness or worker changes.
    pub(crate) fn new(application: ApplicationSessions) -> Result<Box<Self>, RetentionError> {
        let fixed_guard = application
            .budget()
            .reserve_resources(RetainedResources {
                payload_bytes: std::mem::size_of::<Self>(),
                ..Default::default()
            })?
            .retain(());
        Ok(Box::new(Self {
            application,
            controller: None,
            pending: None,
            preparation_cancelled: false,
            key: None,
            report: None,
            error: WorkerFailure::default(),
            draft: None,
            draft_outcome: drafts::Outcome::default(),
            _fixed_guard: fixed_guard,
        }))
    }

    /// Stages a bounded public configuration for background parsing/admission.
    /// # Arguments
    /// * `document` - Exact existing merlin.upstream.configuration JSON bytes.
    /// # Returns
    /// Success means preparation started, not that configuration is accepted.
    /// # Errors
    /// Busy, byte policy or spawn rejection preserves accepted configuration/results.
    pub(crate) fn apply(&mut self, document: &[u8]) -> Result<(), String> {
        if self.pending.is_some() || self.is_running() {
            return Err("Stop upstream work before applying a configuration.".into());
        }
        if document.len() > MAX_CONFIGURATION_BYTES {
            return Err("Upstream configuration exceeds the input byte limit.".into());
        }
        let payload_bytes = std::mem::size_of::<Retained<Box<[u8]>>>()
            .checked_add(document.len())
            .ok_or("Upstream configuration ownership size overflowed.")?;
        let reservation = self
            .application
            .budget()
            .reserve_resources(RetainedResources {
                payload_bytes,
                ..Default::default()
            })
            .map_err(|error| format!("{error:?}"))?;
        self.pending = Some(Pending::configuration(
            reservation.retain(document.into()),
            self.application.budget().clone(),
            self.application.snapshot(Direction::Upstream).key,
        )?);
        self.preparation_cancelled = false;
        self.error = WorkerFailure::default();
        Ok(())
    }

    /// Requests report-target preparation before actual shared run admission.
    /// # Returns
    /// Unit after staging; Poll reports eventual admission and scientific progress.
    /// # Errors
    /// Global Busy, stale/unconfigured or preparing states preserve previous results.
    pub(crate) fn start(&mut self) -> Result<(), String> {
        if self.pending.is_some() || self.is_running() || self.application.active().is_some() {
            return Err("Another simulation or preparation is active.".into());
        }
        if !self.current() {
            return Err("Apply an upstream configuration before starting.".into());
        }
        let Some(controller) = self.controller.take() else {
            return Err("Apply an upstream configuration before starting.".into());
        };
        match Pending::run(controller, self.application.budget().clone()) {
            Ok(pending) => self.pending = Some(pending),
            Err((controller, error)) => {
                self.controller = Some(controller);
                return Err(error);
            }
        }
        self.preparation_cancelled = false;
        self.error = WorkerFailure::default();
        Ok(())
    }

    /// Toggles the same acknowledged upstream run without controlling downstream.
    /// # Returns
    /// Unit after a cooperative request; Poll observes actual acknowledgement.
    /// # Errors
    /// No active owned token or stale controls leave every other direction unchanged.
    pub(crate) fn pause(&self) -> Result<(), String> {
        let controller = self
            .controller
            .as_ref()
            .ok_or("No upstream run is active.")?;
        let (token, progress) = controller.progress().ok_or("No upstream run is active.")?;
        self.application
            .validate_callback(token)
            .map_err(|error| format!("{error:?}"))?;
        if progress.status == RunStatus::Paused {
            controller.resume(token)
        } else {
            controller.pause(token)
        }
        .map_err(|error| format!("{error:?}"))
    }

    /// Requests stop for this exact upstream run, retaining admission until terminal.
    /// # Returns
    /// Unit after suppressing preparation adoption or requesting cooperative stop.
    /// # Errors
    /// Missing/stale ownership cannot cancel another direction or replacement session.
    pub(crate) fn stop(&mut self) -> Result<(), String> {
        if self.pending.is_some() {
            self.preparation_cancelled = true;
            return Ok(());
        }
        let (token, _) = self
            .controller
            .as_ref()
            .and_then(UpstreamController::progress)
            .ok_or("No upstream run is active.")?;
        self.application
            .cancel(token)
            .map_err(|error| format!("{error:?}"))
    }

    /// Consumes at most one preparation and one scientific event without waiting.
    /// # Returns
    /// Bounded scalar presentation; no full scientific array crosses to QML.
    pub(crate) fn poll(&mut self) -> serde_json::Value {
        if let Some(prepared) = self.pending.as_ref().and_then(Pending::poll) {
            self.pending = None;
            if let Err(error) = self.adopt_prepared(prepared) {
                self.error = error;
            }
        }
        if let Some(controller) = self.controller.as_ref() {
            if let Some((token, progress)) = controller.progress() {
                match progress.status {
                    RunStatus::Paused => {
                        let _ = self.application.acknowledge_pause(token, true);
                    }
                    RunStatus::Running => {
                        let _ = self.application.acknowledge_pause(token, false);
                    }
                    _ => {}
                }
            }
        }
        let event = self.controller.as_mut().map(UpstreamController::poll);
        match event {
            Some(Ok(ControllerEvent::Report { token, report })) => {
                if self.application.validate_callback(token).is_ok() {
                    self.report = Some((token, report));
                }
            }
            Some(Ok(ControllerEvent::Terminal(terminal))) => {
                if self.application.validate_callback(terminal.token()).is_ok() {
                    match terminal.status() {
                        WorkerStatus::Completed => {}
                        WorkerStatus::Failed(error) => {
                            self.error = self.admit_error(error.to_string())
                        }
                        WorkerStatus::Panicked => {
                            self.error =
                                WorkerFailure::Static("Upstream worker stopped unexpectedly.")
                        }
                    }
                }
                let _ = self.application.finish(terminal.into_lease());
            }
            Some(Err(error)) => self.error = self.admit_error(format!("{error:?}")),
            _ => {}
        }
        self.state()
    }

    fn adopt_prepared(&mut self, prepared: Prepared) -> Result<(), WorkerFailure> {
        match prepared {
            Prepared::Edit {
                delivery: _delivery_guard,
                controller,
                draft,
                candidate,
            } => {
                // Restore the unchanged old owner before handling every terminal path.
                self.controller = Some(controller);
                if !self
                    .draft
                    .as_ref()
                    .is_some_and(|current| std::sync::Arc::ptr_eq(current, &draft))
                {
                    return Ok(());
                }
                if self.preparation_cancelled {
                    self.draft_outcome = drafts::Outcome::Cancelled;
                    return Ok(());
                }
                if self.application.snapshot(Direction::Upstream).key != draft.value().base {
                    self.draft_outcome = drafts::Outcome::Stale;
                    return Err(WorkerFailure::Static(
                        "Upstream configuration changed during draft preparation.",
                    ));
                }
                let candidate = match candidate {
                    Ok(candidate) => candidate,
                    Err(error) => {
                        self.draft_outcome = drafts::Outcome::Rejected;
                        return Err(error);
                    }
                };
                let key = match self
                    .application
                    .configure_upstream_if(draft.value().base, true)
                {
                    Ok(key) => key,
                    Err(error) => {
                        self.draft_outcome = drafts::Outcome::Rejected;
                        return Err(self.admit_error(format!("{error:?}")));
                    }
                };
                // This key is issued by the upstream host; the sole constructor
                // error is a downstream direction, which cannot occur here.
                self.controller = Some(
                    UpstreamController::new(key, candidate)
                        .expect("Canonical upstream configuration supplies an upstream key"),
                );
                self.key = Some(key);
                self.report = None;
                self.error = WorkerFailure::default();
                self.draft_outcome = drafts::Outcome::Accepted;
            }
            Prepared::Configuration {
                delivery: _delivery_guard,
                expected,
                candidate,
            } => {
                if self.preparation_cancelled {
                    return Ok(());
                }
                if self.application.snapshot(Direction::Upstream).key != expected {
                    return Err(WorkerFailure::Static(
                        "Upstream configuration changed during preparation.",
                    ));
                }
                let candidate = candidate?;
                let key = self
                    .application
                    .configure_upstream(true)
                    .map_err(|error| self.admit_error(format!("{error:?}")))?;
                let controller = UpstreamController::new(key, candidate)
                    .map_err(|error| self.admit_error(format!("{error:?}")))?;
                self.controller = Some(controller);
                self.key = Some(key);
                self.report = None;
                if self.draft.is_some() {
                    self.draft_outcome = drafts::Outcome::Stale;
                }
            }
            Prepared::Run {
                delivery: _delivery_guard,
                controller,
                target,
            } => {
                self.controller = Some(controller);
                if self.preparation_cancelled {
                    return Ok(());
                }
                self.dispatch(target?)
                    .map_err(|error| self.admit_error(error))?;
            }
        }
        Ok(())
    }

    fn dispatch(&mut self, target: PreparedRun) -> Result<(), String> {
        let key = self.key.ok_or("Upstream configuration changed.")?;
        let control = UpstreamControl::new(RunControl::new(), self.application.budget())
            .map_err(|error| format!("{error:?}"))?;
        let lease = self
            .application
            .begin(key, control.clone())
            .map_err(|error| format!("{error:?}"))?;
        let controller = self
            .controller
            .as_mut()
            .expect("Prepared run restores its accepted owner");
        if let Err(error) = controller.start(lease, control, target, RUN_COMPLETION_CAPACITY) {
            let (reason, lease) = error.into_parts();
            let _ = self.application.finish(lease);
            return Err(format!("{reason:?}"));
        }
        Ok(())
    }

    fn admit_error(&self, message: String) -> WorkerFailure {
        WorkerFailure::admit_with_fallback(
            error_text(message).into(),
            self.application.budget(),
            WORKER_FAILURE_ADMISSION_MESSAGE,
        )
    }

    fn current(&self) -> bool {
        self.key
            .is_some_and(|key| self.application.snapshot(Direction::Upstream).key == key)
    }

    fn is_running(&self) -> bool {
        self.controller
            .as_ref()
            .and_then(UpstreamController::progress)
            .is_some()
    }
}

/// Preserves the existing Qt diagnostic character limit for every upstream path.
///
/// # Arguments
/// * `error` - Temporary rendered text from the original application/public error.
///
/// # Returns
/// The unchanged bounded prefix. Retained worker text is separately admitted before
/// enqueue; independent GUI copies use the same admitted error owner.
pub(super) fn error_text(error: String) -> String {
    error.chars().take(MAX_ERROR_CHARACTERS).collect()
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod fixed_retention_tests;
