/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Direction-owned project destination and exact-revision adoption.
use super::{preparation::Pending, Model, ObservationStatus};
use crate::{
    shared::{
        project_path::{self, ProjectPath},
        worker_failure::WorkerFailure,
    },
    shell::{Direction, ProjectSnapshot, ReplacePermission, Retained},
    upstream::{PreparedSession, UpstreamController},
};
use std::path::{Path, PathBuf};

#[derive(Default)]
pub(super) struct Files {
    path: Option<ProjectPath>,
    status: Status,
    intent: Option<Intent>,
}

struct Intent {
    expected: ProjectSnapshot,
    kind: IntentKind,
}
enum IntentKind {
    New,
    Open(ProjectPath),
    Prepared(Box<Retained<Opened>>),
}

#[derive(Clone, Copy, Default)]
enum Status {
    #[default]
    Idle,
    Saving,
    Saved,
    Opening,
    Opened,
    Created,
    Cancelled,
    Rejected,
}

pub(super) struct Written {
    pub(super) path: ProjectPath,
}

pub(crate) struct Opened {
    pub(crate) session: PreparedSession,
    pub(crate) path: Option<ProjectPath>,
}

impl Model {
    /// Stages a native direction-detected project before its target dirty decision.
    /// # Arguments
    /// * `candidate` - Complete same-host public scientific/path owner from Open.
    /// # Returns
    /// Unit after capturing the actual upstream revision, independent of selection.
    /// No scientific document, original path or native revision crosses QML.
    /// # Errors
    /// Busy, another intent or foreign retention ownership preserves accepted state.
    pub(crate) fn stage_prepared_project(
        &mut self,
        candidate: Box<Retained<Opened>>,
    ) -> Result<(), String> {
        self.ensure_project_intent_available()?;
        if !self.application.budget().owns(&candidate) {
            return Err("Prepared upstream project belongs to another application host.".into());
        }
        self.preparation_cancelled = false;
        self.project.intent = Some(Intent {
            expected: self.application.snapshot(Direction::Upstream),
            kind: IntentKind::Prepared(candidate),
        });
        Ok(())
    }
    /// Captures a native New target before displaying its dirty prompt.
    /// # Returns
    /// Unit after capturing exact identity/revision, without changing science.
    /// # Errors
    /// Busy or an existing unresolved decision preserves the original intent.
    pub(crate) fn request_project_new(&mut self) -> Result<(), String> {
        self.ensure_project_intent_available()?;
        self.project.intent = Some(Intent {
            expected: self.application.snapshot(Direction::Upstream),
            kind: IntentKind::New,
        });
        Ok(())
    }

    /// Captures an admitted Open destination before its target's dirty prompt.
    /// # Arguments
    /// * `path` - Local path detected as upstream by the native file coordinator.
    /// # Returns
    /// Unit with the exact native target retained, without replacing either tab.
    /// # Errors
    /// Busy, existing intent or path admission leaves accepted state untouched.
    pub(crate) fn request_project_open(&mut self, path: PathBuf) -> Result<(), String> {
        self.ensure_project_intent_available()?;
        let path = project_path::admit(path, self.application.budget())?;
        self.project.intent = Some(Intent {
            expected: self.application.snapshot(Direction::Upstream),
            kind: IntentKind::Open(path),
        });
        Ok(())
    }

    /// Resolves only the native intent captured before the displayed decision.
    /// # Arguments
    /// * `discard` - Explicit discard authorization for that exact revision.
    /// * `cancel` - Dismisses the intent without changing accepted data or paths.
    /// # Returns
    /// Unit after cancellation, blank adoption or asynchronous Open dispatch.
    /// # Errors
    /// Missing/stale intent, unresolved dirty state or preparation failure leaves
    /// accepted science/path unchanged. Save success refreshes only its own guard.
    pub(crate) fn resolve_project(&mut self, discard: bool, cancel: bool) -> Result<(), String> {
        let intent = self
            .project
            .intent
            .take()
            .ok_or("No upstream file decision is pending.")?;
        if cancel {
            return Ok(());
        }
        let permission = if discard {
            ReplacePermission::DiscardConfirmed
        } else {
            ReplacePermission::RequireClean
        };
        match intent.kind {
            IntentKind::New => self.new_project(intent.expected, permission),
            IntentKind::Open(path) => {
                self.open_project(path.value().clone(), intent.expected, permission)
            }
            IntentKind::Prepared(candidate) => {
                self.check_project_guard(intent.expected, permission)?;
                self.adopt_project_open(intent.expected, permission, Ok(*candidate))
                    .map_err(|error| error.message().to_owned())
            }
        }
    }

    fn ensure_project_intent_available(&self) -> Result<(), String> {
        self.ensure_project_idle()?;
        if self.project.intent.is_some() {
            return Err("Finish the pending upstream project decision first.".into());
        }
        Ok(())
    }
    /// Borrows this direction's last successfully adopted explicit destination.
    /// # Returns
    /// Original native path, independent of selected tab; no path copy is retained.
    pub(crate) fn project_path(&self) -> Option<&Path> {
        self.project
            .path
            .as_ref()
            .map(|owner| owner.value().as_path())
    }

    /// Queues public project serialization and safe writing away from Qt.
    /// # Arguments
    /// * `destination` - Explicit chosen path, or reuse the last successful one.
    /// # Returns
    /// Unit after dispatch, before writing/clean acknowledgement. The captured
    /// native snapshot alone determines which revision can become clean.
    /// # Errors
    /// Busy, missing configuration/destination, path/transport admission or spawn
    /// rejection leaves the accepted owner, old path and dirty revision intact.
    pub(crate) fn save_project(&mut self, destination: Option<PathBuf>) -> Result<(), String> {
        self.ensure_project_idle()?;
        if !self.current() {
            return Err("Apply an upstream configuration before saving a project.".into());
        }
        let destination = destination
            .or_else(|| self.project_path().map(Path::to_path_buf))
            .ok_or("Choose an upstream project destination.")?;
        let path = project_path::admit(
            project_path::normalize_destination(destination)?,
            self.application.budget(),
        )?;
        let expected = self.application.snapshot(Direction::Upstream);
        let controller = self
            .controller
            .take()
            .ok_or("Upstream session is unavailable.")?;
        match Pending::project_save(controller, self.application.clone(), expected, path) {
            Ok(pending) => self.pending = Some(pending),
            Err((controller, error)) => {
                self.controller = Some(controller);
                return Err(error);
            }
        }
        self.project.status = Status::Saving;
        self.preparation_cancelled = false;
        self.error = Default::default();
        Ok(())
    }

    /// Queues complete public project admission for a captured upstream target.
    /// # Arguments
    /// * `path` - Chosen local project; source inputs are not registered as this path.
    /// * `expected`, `permission` - Native target captured before its dirty decision.
    /// # Returns
    /// Unit after finite background dispatch, with the old controller still owned.
    /// # Errors
    /// Wrong/stale/dirty target, Busy, path/transport admission or spawn failure
    /// leaves both directions and their destinations unchanged.
    pub(crate) fn open_project(
        &mut self,
        path: PathBuf,
        expected: ProjectSnapshot,
        permission: ReplacePermission,
    ) -> Result<(), String> {
        self.ensure_project_idle()?;
        self.check_project_guard(expected, permission)?;
        let path = project_path::admit(path, self.application.budget())?;
        self.pending = Some(Pending::project_open(
            self.application.clone(),
            expected,
            permission,
            path,
        )?);
        self.project.status = Status::Opening;
        self.preparation_cancelled = false;
        self.error = Default::default();
        Ok(())
    }

    /// Creates a genuinely unconfigured direction after its exact dirty decision.
    /// # Arguments
    /// * `expected`, `permission` - Native target snapshot and explicit decision.
    /// # Returns
    /// Unit after adopting a clean identity without configuration or a Save path.
    /// # Errors
    /// Busy, stale/dirty denial or metadata admission preserves the old project.
    pub(crate) fn new_project(
        &mut self,
        expected: ProjectSnapshot,
        permission: ReplacePermission,
    ) -> Result<(), String> {
        self.ensure_project_idle()?;
        self.check_project_guard(expected, permission)?;
        self.application
            .replace_project(expected, permission, false)
            .map_err(|error| format!("{error:?}"))?;
        self.controller = None;
        self.key = None;
        self.clear_project_views();
        self.project.path = None;
        self.project.status = Status::Created;
        self.application.clear_input_paths(Direction::Upstream);
        Ok(())
    }

    fn ensure_project_idle(&self) -> Result<(), String> {
        if self.pending.is_some() || self.is_running() {
            return Err("Finish upstream work before changing project files.".into());
        }
        Ok(())
    }

    fn check_project_guard(
        &self,
        expected: ProjectSnapshot,
        permission: ReplacePermission,
    ) -> Result<(), String> {
        if expected.key.direction != Direction::Upstream
            || self.application.snapshot(Direction::Upstream) != expected
        {
            return Err("The upstream project changed. Retry the file action.".into());
        }
        if expected.dirty && permission == ReplacePermission::RequireClean {
            return Err("Save or discard the upstream project's changes first.".into());
        }
        Ok(())
    }

    /// Acknowledges a real completed write before changing this direction's path.
    /// # Arguments
    /// * `expected` - Native snapshot captured before the dispatched write.
    /// * `candidate` - Admitted completed path, or original worker failure.
    /// # Returns
    /// Unit after exact acknowledgement and matching intent refresh; the original
    /// controller has already been restored by preparation adoption.
    /// # Errors
    /// Stale revision or worker failure preserves the old path and dirty state.
    pub(super) fn adopt_project_save(
        &mut self,
        expected: ProjectSnapshot,
        candidate: Result<Retained<Written>, WorkerFailure>,
    ) -> Result<(), WorkerFailure> {
        self.project.status = Status::Rejected;
        if self.preparation_cancelled {
            self.project.status = Status::Cancelled;
            return Ok(());
        }
        let candidate = candidate?;
        self.application
            .acknowledge_save(expected)
            .map_err(|error| self.admit_error(format!("{error:?}")))?;
        if let Some(intent) = self.project.intent.as_mut() {
            if intent.expected == expected {
                intent.expected = self.application.snapshot(Direction::Upstream);
            }
        }
        let (written, _guard) = candidate.into_parts();
        self.project.path = Some(written.path);
        self.project.status = Status::Saved;
        self.error = Default::default();
        Ok(())
    }

    /// Adopts one fully prepared upstream project after its exact final guard.
    /// # Arguments
    /// * `expected`, `permission` - Native target and explicit dirty decision.
    /// * `candidate` - Admitted public session/path, or original worker error.
    /// # Returns
    /// Unit after replacing only upstream and clearing its old views/protection.
    /// Cancelled preparation keeps the accepted controller and path unchanged.
    /// # Errors
    /// Worker, stale/dirty target or host admission failure before publication.
    pub(super) fn adopt_project_open(
        &mut self,
        expected: ProjectSnapshot,
        permission: ReplacePermission,
        candidate: Result<Retained<Opened>, WorkerFailure>,
    ) -> Result<(), WorkerFailure> {
        self.project.status = Status::Rejected;
        if self.preparation_cancelled {
            self.project.status = Status::Cancelled;
            return Ok(());
        }
        let candidate = candidate?;
        let key = self
            .application
            .replace_project(expected, permission, true)
            .map_err(|error| self.admit_error(format!("{error:?}")))?;
        let (opened, _guard) = candidate.into_parts();
        self.controller = Some(
            UpstreamController::new(key, opened.session)
                .expect("Upstream replacement supplies an upstream key"),
        );
        self.key = Some(key);
        self.clear_project_views();
        self.project.path = opened.path;
        self.project.status = Status::Opened;
        self.application.clear_input_paths(Direction::Upstream);
        Ok(())
    }

    fn clear_project_views(&mut self) {
        self.report = None;
        self.grid = None;
        self.grid_status = ObservationStatus::Unavailable;
        self.instrument = None;
        self.instrument_status = ObservationStatus::Unavailable;
        self.capture = None;
        self.draft = None;
        self.draft_outcome = Default::default();
        self.error = Default::default();
    }

    /// Projects only native path/status, dirty state and compiled action metadata.
    /// # Returns
    /// Bounded presentation without configuration documents, captures or samples.
    pub(super) fn project_state(&self) -> serde_json::Value {
        let snapshot = self.application.snapshot(Direction::Upstream);
        serde_json::json!({"dirty":snapshot.dirty,
        "path":self.project_path().map(|path|path.to_string_lossy()),
            "can_save":self.current()&&self.pending.is_none()&&!self.is_running(),
            "decision":self.project.intent.as_ref().map(|intent|match intent.kind {
                IntentKind::New=>"new", IntentKind::Open(_)|IntentKind::Prepared(_)=>"open",
            }),
        "status":match self.project.status {
            Status::Idle=>"idle", Status::Saving=>"saving", Status::Saved=>"saved",
            Status::Opening=>"opening",Status::Opened=>"opened",Status::Created=>"created",
            Status::Cancelled=>"cancelled", Status::Rejected=>"rejected",
        }})
    }
}

#[cfg(test)]
#[path = "project_tests.rs"]
mod tests;
