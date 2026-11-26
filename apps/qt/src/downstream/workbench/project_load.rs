/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Single-flight background Open; only completed owned state crosses threads.
mod save;
use super::{
    project_file::{Editors, Project, MAX_PROJECT_BYTES},
    session::{AcceptedState, Context, Snapshot},
};
use crate::downstream::{
    instruments::transmitter::preview::Analyzer, simulation::closed_loop::Model,
};
use crate::{
    shared::{
        presentation::constants::{
            WORKER_FAILURE_ADMISSION_MESSAGE, WORKER_TRANSPORT_ADMISSION_MESSAGE,
        },
        project_path::{self, ProjectPath},
        worker_controls,
        worker_failure::WorkerFailure,
    },
    shell::{
        admit_task, bounded_channel, run_task, DeliveryReceiver, Retained, RetainedResources,
        RetentionBudget,
    },
};
pub(crate) use save::{SaveOptions, SaveRequest};
use serde_json::{json, Value};
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        mpsc::TryRecvError,
        Arc,
    },
    time::Instant,
};

// Existing atomic progress tags; values are private UI transport, not PHY stages.
const PROJECT_CHOOSER_PHASE: u8 = 0;
const PROJECT_FILE_READ_PHASE: u8 = 1;
const PROJECT_CONFIGURATION_PHASE: u8 = 2;
const PROJECT_PREPARATION_PHASE: u8 = 3;
// Independent streaming chunk policy: 64 KiB, separate from total input ceilings.
const PROJECT_READ_CHUNK_BYTES: usize = 64 * 1024;

/// Owned editor/request transfer consumed synchronously during GUI adoption.
#[derive(Default)]
pub(crate) struct PreparedModels {
    pub source: Option<
        Box<crate::shell::Retained<crate::downstream::inputs::source_input::model::SourceModel>>,
    >,
    pub profile:
        Option<Box<crate::shell::Retained<crate::downstream::inputs::profile_input::ProfileModel>>>,
    pub measurements: Option<
        Box<crate::shell::Retained<crate::downstream::inputs::measurement_input::model::Model>>,
    >,
    pub receiver: Option<Box<Model>>,
    pub transmitter: Option<Box<Analyzer>>,
}

/// Moves one prepared model into its GUI adapter without decoding captures again.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `take` - Takes the owning adapter's field from the completed transfer.
/// # Returns
/// That field, or None for normal edits and blank workbenches.
pub(crate) fn take<T>(
    context: &Context,
    take: impl FnOnce(&mut PreparedModels) -> Option<T>,
) -> Option<T> {
    context
        .prepared
        .with(|prepared| take(&mut prepared.borrow_mut()))
}

/// Guarded project products transferred to the file adapter after atomic adoption.
#[derive(Debug)]
pub(crate) struct AdoptedProject {
    pub(crate) path: Option<ProjectPath>,
    pub(crate) metadata: Box<Retained<Value>>,
}

enum MetadataKind {
    Opened,
    Created,
}

pub(crate) struct PreparedProject {
    pub path: Option<ProjectPath>,
    accepted: AcceptedState,
    target: crate::shell::SessionKey,
    project_guard: crate::shell::ProjectSnapshot,
    expected: Snapshot,
    models: PreparedModels,
    metadata: Box<Retained<Value>>,

    // All owned data and independently guarded children drop before this layout.
    _fixed_guard: Retained<()>,
}
impl PreparedProject {
    /// Preflights a blank workbench and all editor resets against the current target.
    ///
    /// # Arguments
    /// * `context` - Exact downstream owner; no inputs, paths or GUI models change here.
    ///
    /// # Returns
    /// Complete admitted blank candidate, using the same guarded adoption as Open.
    ///
    /// # Errors
    /// Any editor/scientific/input admission or validation failure preserves old state.
    pub(crate) fn blank(context: &Context) -> Result<Self, String> {
        Self::prepare_guarded(
            Project {
                schema_version: crate::downstream::common::constants::WORKBENCH_SCHEMA_VERSION,
                application: "MERLIN".into(),
                transmission: Default::default(),
                sections: Default::default(),
                receiver: None,
                comparison_enabled: false,
                dark: true,
            },
            None,
            &context.application,
            context
                .application
                .snapshot(crate::shell::Direction::Downstream)
                .key,
            super::session::snapshot(context),
            MetadataKind::Created,
        )
    }

    /// Validates and prepares a project independently of the GUI session.
    /// # Arguments
    /// * `project` - Bounded, decoded file document.
    /// * `path` - Explicit Save destination, None for update recovery.
    /// * `application` - Canonical accepting host, covering old and queued owners.
    /// * `target` - Exact downstream identity/generation captured when Open started.
    /// * `expected` - Original accepted owner captured at the same request boundary.
    /// # Returns
    /// Validated models and guarded path/readiness metadata; never starts a run.
    /// # Errors
    /// Preserves validation or ownership-admission errors without publishing partial state.
    #[cfg(test)]
    pub(crate) fn prepare(
        project: Project,
        path: Option<PathBuf>,
        application: &crate::shell::ApplicationSessions,
        target: crate::shell::SessionKey,
        expected: Snapshot,
    ) -> Result<Self, String> {
        let path = path
            .map(|path| project_path::admit(path, application.budget()))
            .transpose()?;
        Self::prepare_guarded(
            project,
            path,
            application,
            target,
            expected,
            MetadataKind::Opened,
        )
    }

    fn prepare_guarded(
        project: Project,
        path: Option<ProjectPath>,
        application: &crate::shell::ApplicationSessions,
        target: crate::shell::SessionKey,
        expected: Snapshot,
        kind: MetadataKind,
    ) -> Result<Self, String> {
        Self::prepare_with_plan(project, path, application, target, expected, kind, None)
    }

    fn prepare_with_plan(
        project: Project,
        path: Option<ProjectPath>,
        application: &crate::shell::ApplicationSessions,
        target: crate::shell::SessionKey,
        expected: Snapshot,
        kind: MetadataKind,
        public_plan: Option<merlin::downstream::FullLoopSweepPlan>,
    ) -> Result<Self, String> {
        let budget = application.budget();
        if path.as_ref().is_some_and(|path| !budget.owns(path)) {
            return Err("Prepared project path belongs to another application host.".into());
        }
        let candidate_context = Context::for_application(application.clone())
            .map_err(|_| "The session retained-data limit would be exceeded.".to_owned())?;
        let (
            inputs,
            Editors {
                source,
                profile,
                measurements,
            },
        ) = project.prepare_for_context(candidate_context.clone())?;
        let settings =
            serde_json::to_string(&project.transmission).map_err(|error| error.to_string())?;
        let mut receiver = Model::for_context(candidate_context.clone()).map_err(|_| {
            crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE.to_owned()
        })?;
        receiver.prepare_options_for_budget(
            project.receiver.clone(),
            project.comparison_enabled,
            budget,
        )?;
        if let Some(plan) = public_plan {
            receiver.prepare_public_plan_for_budget(plan, budget)?;
        } else {
            receiver.prepare_inputs_for_budget(&settings, &inputs, &project.sections, budget)?;
        }
        let mut transmitter = Analyzer::for_context(candidate_context).map_err(|_| {
            crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE.to_owned()
        })?;
        transmitter.configure_for_budget(
            &settings,
            project.sections.get("source").unwrap_or(&Value::Null),
            project.sections.get("profile").unwrap_or(&Value::Null),
            budget,
        )?;
        let mut metadata = json!({"opened":true,"prepared":true,"path":path.as_ref().map(|p|p.value().to_string_lossy()),
            "transmission":project.transmission,"receiver":project.receiver,
            "verified":project.comparison_enabled,"dark":project.dark,
            "source_state":source.state(),"profile_state":profile.state(&settings),
            "measurement_state":measurements.state(Ok(project.transmission.clone()), &inputs)});
        if matches!(kind, MetadataKind::Created) {
            metadata["created"] = json!(true);
            metadata["opened"] = json!(false);
            metadata["dark"] = Value::Null;
        }
        let metadata = admit_metadata(metadata, budget)?;
        let fixed_guard = budget
            .reserve_resources(RetainedResources {
                payload_bytes: std::mem::size_of::<Self>(),
                ..Default::default()
            })
            .map_err(|_| "The session retained-data limit would be exceeded.".to_owned())?
            .retain(());
        let accepted = super::session::prepare_accepted(inputs, project.sections, budget)?;
        let source = source.admit(budget)?;
        let profile = profile.admit(budget)?;
        let measurements = measurements.admit(budget)?;
        Ok(Self {
            path,
            accepted,
            target,
            project_guard: application.snapshot(crate::shell::Direction::Downstream),
            expected,
            models: PreparedModels {
                source: Some(Box::new(source)),
                profile: Some(Box::new(profile)),
                measurements: Some(Box::new(measurements)),
                receiver: Some(receiver),
                transmitter: Some(transmitter),
            },
            metadata,
            _fixed_guard: fixed_guard,
        })
    }

    /// Prepares one nested recovery document through the normal native owners.
    /// # Arguments
    /// * `bytes`, `path` - Exact project bytes and its saved normal destination.
    /// * `application`, `target`, `expected` - Original canonical and accepted guards.
    /// * `dark` - Saved application appearance, applied before metadata admission.
    /// # Returns
    /// Complete admitted models, including partial legacy editors where supported.
    /// # Errors
    /// Existing public/editor/observation or retained admission errors, without mutation.
    pub(crate) fn prepare_recovery(
        bytes: &[u8],
        path: Option<ProjectPath>,
        application: &crate::shell::ApplicationSessions,
        target: crate::shell::ProjectSnapshot,
        expected: Snapshot,
        dark: bool,
    ) -> Result<Self, String> {
        let mut decoded = super::project_envelope::decode(bytes)?;
        decoded.project.dark = dark;
        let mut candidate = Self::prepare_with_plan(
            decoded.project,
            path,
            application,
            target.key,
            expected,
            MetadataKind::Opened,
            decoded.public_plan,
        )?;
        candidate.project_guard = target;
        Ok(candidate)
    }

    /// Publishes a completed project on the calling GUI thread only.
    /// # Arguments
    /// * `context` - Validated target owner; other contexts and transfers stay untouched.
    /// # Returns
    /// Guarded original path and metadata; original bytes and requests remain in Rust.
    /// # Errors
    /// Rejects a stale generation or foreign host before any target state changes.
    /// Scientific roots and final readiness are checked before the owner swap;
    /// subsequent GUI attachment needs no new reservation or generation advance.
    pub(crate) fn adopt(self, context: &Context) -> Result<AdoptedProject, String> {
        self.adopt_with_permission(context, crate::shell::ReplacePermission::DiscardConfirmed)
    }

    /// Borrows the native exact target captured before its dirty-state decision.
    /// # Returns
    /// Direction/key/revision guard, retained in Rust rather than passed through QML.
    pub(crate) fn project_guard(&self) -> crate::shell::ProjectSnapshot {
        self.project_guard
    }

    /// Refreshes only this intent's guard after its exact successful native Save.
    /// # Arguments
    /// * `saved`, `current` - Acknowledged pre-write and resulting clean snapshots.
    /// # Returns
    /// Unit; unrelated/newer intent guards are never changed or implicitly confirmed.
    pub(crate) fn refresh_saved_guard(
        &mut self,
        saved: crate::shell::ProjectSnapshot,
        current: crate::shell::ProjectSnapshot,
    ) {
        if self.project_guard == saved {
            self.project_guard = current;
        }
    }

    /// Adopts a completely prepared direction project after its exact dirty decision.
    /// # Arguments
    /// * `context` - Actual downstream owner and accepted-input store.
    /// * `permission` - Explicit clean/discard authorization for the captured guard.
    /// # Returns
    /// New clean native identity and already-admitted editor/path metadata.
    /// # Errors
    /// Every stale/dirty/ownership/Busy/admission failure precedes target mutation.
    pub(crate) fn adopt_with_permission(
        self,
        context: &Context,
        permission: crate::shell::ReplacePermission,
    ) -> Result<AdoptedProject, String> {
        let guard = self.project_guard;
        let adopted = self.adopt_transaction(context, |ready| {
            context
                .application
                .replace_project(guard, permission, ready)
                .map_err(|error| match error {
                    crate::shell::ApplicationError::Retention(_) => {
                        "The session retained-data limit would be exceeded.".into()
                    }
                    crate::shell::ApplicationError::Session(
                        crate::shell::SessionError::StaleGuard,
                    ) => "The prepared workbench target is stale.".into(),
                    _ => "The workbench target cannot accept this project.".into(),
                })
        })?;
        context
            .application
            .clear_input_paths(crate::shell::Direction::Downstream);
        Ok(adopted)
    }

    /// Attaches all admitted downstream roots around a final canonical transaction.
    /// # Arguments
    /// * `context` - Exact accepting downstream owner, independent of selected tab.
    /// * `commit` - Last fallible readiness operation; must publish no partial state
    ///   on error and may attach the already prepared upstream owner on success.
    /// # Returns
    /// Already admitted editor metadata/path after infallible native model attachment.
    /// # Errors
    /// Target/accepted staleness, missing/foreign root or commit rejection precedes
    /// accepted-input publication. Original-path cleanup remains caller-owned.
    pub(crate) fn adopt_transaction(
        mut self,
        context: &Context,
        commit: impl FnOnce(bool) -> Result<crate::shell::SessionKey, String>,
    ) -> Result<AdoptedProject, String> {
        if context
            .application
            .snapshot(crate::shell::Direction::Downstream)
            .key
            != self.target
        {
            return Err("The prepared workbench target is stale.".into());
        }
        // All scientific target roots already participate in the accepting host.
        // Check them before the last fallible readiness transition or data mutation.
        let budget = context.application.budget();
        if !budget.owns(&self._fixed_guard)
            || !budget.owns(&self.metadata)
            || self.path.as_ref().is_some_and(|path| !budget.owns(path))
        {
            return Err(
                "Prepared project metadata or path belongs to another application host.".into(),
            );
        }
        if !self
            .models
            .source
            .as_ref()
            .is_some_and(|owner| budget.owns(owner))
            || !self
                .models
                .profile
                .as_ref()
                .is_some_and(|owner| budget.owns(owner))
            || !self
                .models
                .measurements
                .as_ref()
                .is_some_and(|owner| budget.owns(owner))
        {
            return Err(
                "Prepared editors are missing or belong to another application host.".into(),
            );
        }
        let receiver = self
            .models
            .receiver
            .as_ref()
            .ok_or("Prepared receiver is missing.")?;
        let ready = receiver.check_prepared_target(context.application.budget())?;
        self.models
            .transmitter
            .as_ref()
            .ok_or("Prepared transmitter is missing.")?
            .check_prepared_target(context.application.budget())
            .map_err(|error| error.into_message())?;
        let key = super::session::adopt_prepared_transaction(
            context,
            self.accepted,
            &self.expected,
            || commit(ready),
        )?;
        // Detached models still hold isolated Contexts. Only the genuine key is
        // stamped here; the GUI attaches its Context after removing each model.
        self.models
            .receiver
            .as_mut()
            .expect("Receiver was checked above")
            .commit_prepared_generation(key);
        *context.result.borrow_mut() = None;
        crate::downstream::instruments::receiver::performance::clear(context);
        context
            .prepared
            .with(|prepared| *prepared.borrow_mut() = self.models);
        Ok(AdoptedProject {
            path: self.path,
            metadata: self.metadata,
        })
    }
}

fn admit_metadata(value: Value, budget: &RetentionBudget) -> Result<Box<Retained<Value>>, String> {
    let mut resources = crate::downstream::inputs::retention::json_children(&value)?;
    resources.payload_bytes = resources
        .payload_bytes
        .checked_add(std::mem::size_of::<Retained<Value>>())
        .ok_or("Project metadata ownership size overflowed.")?;
    let reservation = budget
        .reserve_resources(resources)
        .map_err(|_| "The session retained-data limit would be exceeded.".to_owned())?;
    Ok(Box::new(reservation.retain(value)))
}

const LOADER_BUSY_MESSAGE: &str = "A workbench is already loading.";

// One loader completion; independent of scientific progress callback capacity.
const LOADER_COMPLETION_CAPACITY: usize = 1;

enum LoadCandidate {
    Downstream(Box<PreparedProject>),
    Upstream(Box<Retained<crate::upstream::workbench::PreparedProject>>),
    Saved(Box<Retained<save::Saved>>),
}

struct Job {
    receiver: DeliveryReceiver<Result<Option<LoadCandidate>, WorkerFailure>>,
    cancelled: Arc<Retained<AtomicBool>>,
    stage: Arc<Retained<AtomicU8>>,
    started: Instant,
    _fixed_guard: Retained<()>,
    saving: bool,
}

/// One tracked worker, with nonblocking progress/completion and cancellation.
#[derive(Default)]
pub(crate) struct Loader {
    job: Option<Job>,
    upstream: Option<Box<Retained<crate::upstream::workbench::PreparedProject>>>,
    saved: Option<Box<Retained<save::Saved>>>,
}
impl Loader {
    /// Reports whether a load is still tracked, including cancellation.
    /// # Returns
    /// True until completion is drained; prevents conflicting file operations.
    pub(crate) fn is_loading(&self) -> bool {
        self.job.is_some() || self.upstream.is_some() || self.saved.is_some()
    }

    /// Transfers a direction-detected candidate directly to its native adapter.
    /// # Returns
    /// Original same-host scientific/path owner, without copying through QML.
    pub(crate) fn take_upstream(
        &mut self,
    ) -> Option<Box<Retained<crate::upstream::workbench::PreparedProject>>> {
        self.upstream.take()
    }

    /// Takes a completed write before exact native revision acknowledgement.
    /// # Returns
    /// Admitted destination/guard, without cleaning or changing either session.
    pub(crate) fn take_saved(&mut self) -> Option<Box<Retained<save::Saved>>> {
        self.saved.take()
    }
    /// Starts the native picker or recovery read on a background thread.
    /// # Arguments
    /// * `context` - Exact target captured before background work.
    /// * `recovery` - Raw recovery path admitted before capture; None selects a project.
    /// # Returns
    /// Loading metadata, or a busy/spawn diagnostic; repeated Open is rejected.
    pub(crate) fn start(&mut self, context: &Context, recovery: Option<PathBuf>) -> Value {
        if self.is_loading() {
            return json!({"error": LOADER_BUSY_MESSAGE});
        }
        let source = match recovery
            .map(|path| project_path::admit(path, context.application.budget()))
            .transpose()
        {
            Ok(source) => source.map(crate::platform::startup::AdmittedSource::File),
            Err(error) => return json!({"error": error}),
        };
        self.start_source(context, source, false)
    }

    /// Starts a command-line file or redirected standard-input workbench.
    /// # Arguments
    /// * `context` - Exact target captured before background work.
    /// * `source` - Same-host admitted request; stdin never becomes a Save destination.
    /// # Returns
    /// Immediate loading status; validation and reads run on the single worker.
    pub(crate) fn start_requested(
        &mut self,
        context: &Context,
        source: crate::platform::startup::AdmittedSource,
    ) -> Value {
        self.start_source(context, Some(source), true)
    }

    fn start_source(
        &mut self,
        context: &Context,
        source: Option<crate::platform::startup::AdmittedSource>,
        remember_path: bool,
    ) -> Value {
        if self.is_loading() {
            return json!({"error": LOADER_BUSY_MESSAGE});
        }
        if let Some(crate::platform::startup::AdmittedSource::File(path)) = &source {
            if !context.application.budget().owns(path) {
                return json!({"error":"Project source path belongs to another application host."});
            }
        }
        // Only the host and immutable identity cross this boundary. Retaining
        // the target Context inside its own prepared-model store would form a cycle.
        let application = context.application.clone();
        let target = context
            .application
            .snapshot(crate::shell::Direction::Downstream)
            .key;
        let expected = super::session::snapshot(context);
        let budget = application.budget().clone();
        self.start_candidate(&budget, move |stage, cancelled| {
            use crate::platform::startup::AdmittedSource;
            use std::io::IsTerminal;
            let (mut reader, destination): (Box<dyn std::io::Read>, Option<ProjectPath>) =
                match source {
                    Some(AdmittedSource::Stdin) => {
                        if std::io::stdin().is_terminal() {
                            return Err("Redirect workbench JSON with --workbench - < FILE.".into());
                        }
                        stage.store(PROJECT_FILE_READ_PHASE, Ordering::Relaxed);
                        (Box::new(std::io::stdin()), None)
                    }
                    other => {
                        let (path, remember) = if let Some(AdmittedSource::File(path)) = other {
                            (path, remember_path)
                        } else {
                            let Some(uri) = crate::platform::native_file_picker::choose_workbench(
                                "Open MERLIN Workbench",
                            )?
                            else {
                                return Ok(None);
                            };
                            let path = cxx_qt_lib::QUrl::from(&cxx_qt_lib::QString::from(uri))
                                .to_local_file()
                                .map(|path| PathBuf::from(path.to_string()))
                                .ok_or("Choose a local project path.")?;
                            (project_path::admit(path, application.budget())?, true)
                        };
                        if cancelled.load(Ordering::Relaxed) {
                            return Ok(None);
                        }
                        stage.store(PROJECT_FILE_READ_PHASE, Ordering::Relaxed);
                        if !std::fs::metadata(path.value())
                            .map_err(|error| error.to_string())?
                            .is_file()
                        {
                            return Err("Choose a regular workbench file.".into());
                        }
                        let file =
                            std::fs::File::open(path.value()).map_err(|error| error.to_string())?;
                        let metadata = file.metadata().map_err(|error| error.to_string())?;
                        if !metadata.is_file() || metadata.len() > MAX_PROJECT_BYTES as u64 {
                            return Err(
                                "Choose a regular workbench file within the input limit.".into()
                            );
                        }
                        (Box::new(file), remember.then_some(path))
                    }
                };
            let Some(bytes) = read_bounded(&mut reader, cancelled, MAX_PROJECT_BYTES)? else {
                return Ok(None);
            };
            stage.store(PROJECT_CONFIGURATION_PHASE, Ordering::Relaxed);
            #[derive(serde::Deserialize)]
            struct DirectionHeader {
                direction: Option<merlin::common::ProjectDirection>,
            }
            let header: DirectionHeader = serde_json::from_slice(&bytes)
                .map_err(|error| format!("Invalid workbench JSON: {error}"))?;
            if header.direction == Some(merlin::common::ProjectDirection::Upstream) {
                // Public admission verifies schema, direction and complete source;
                // the header merely routes to that authority and never accepts it.
                let public = merlin::common::Project::from_bytes(&bytes)
                    .map_err(|error| error.to_string())?;
                if cancelled.load(Ordering::Relaxed) {
                    return Ok(None);
                }
                let reservation = application
                    .budget()
                    .reserve_resources(RetainedResources {
                        payload_bytes: std::mem::size_of::<
                            Retained<crate::upstream::workbench::PreparedProject>,
                        >(),
                        ..Default::default()
                    })
                    .map_err(|error| format!("{error:?}"))?;
                let session =
                    crate::upstream::PreparedSession::from_project(&public, application.budget())?;
                let prepared = crate::upstream::workbench::PreparedProject {
                    session,
                    path: destination,
                };
                return Ok(Some(LoadCandidate::Upstream(Box::new(
                    reservation.retain(prepared),
                ))));
            }
            let decoded = super::project_envelope::decode(&bytes)
                .map_err(|error| format!("Invalid workbench JSON: {error}"))?;
            drop(bytes);
            if cancelled.load(Ordering::Relaxed) {
                return Ok(None);
            }
            stage.store(PROJECT_PREPARATION_PHASE, Ordering::Relaxed);
            PreparedProject::prepare_with_plan(
                decoded.project,
                destination,
                &application,
                target,
                expected,
                MetadataKind::Opened,
                decoded.public_plan,
            )
            .map(|project| Some(LoadCandidate::Downstream(Box::new(project))))
        })
    }

    #[cfg(test)]
    fn start_with(
        &mut self,
        budget: &RetentionBudget,
        work: impl FnOnce(&AtomicU8, &AtomicBool) -> Result<Option<PreparedProject>, String>
            + Send
            + 'static,
    ) -> Value {
        self.start_candidate(budget, move |stage, cancelled| {
            work(stage, cancelled).map(|candidate| {
                candidate.map(|candidate| LoadCandidate::Downstream(Box::new(candidate)))
            })
        })
    }

    fn start_candidate(
        &mut self,
        budget: &RetentionBudget,
        work: impl FnOnce(&AtomicU8, &AtomicBool) -> Result<Option<LoadCandidate>, String>
            + Send
            + 'static,
    ) -> Value {
        if self.is_loading() {
            return json!({"error": LOADER_BUSY_MESSAGE});
        }
        let prepare = || {
            let (sender, receiver) = bounded_channel(budget, LOADER_COMPLETION_CAPACITY)?;
            let delivery = sender.reserve()?;
            drop(sender);
            let cancelled = worker_controls::cancellation(budget)?;
            let stage = worker_controls::load_stage(budget)?;
            let fixed = budget.reserve_resources(RetainedResources {
                payload_bytes: std::mem::size_of::<Job>(),
                ..Default::default()
            })?;
            let stop = cancelled.clone();
            let progress = stage.clone();
            let worker_budget = budget.clone();
            let task = admit_task(budget, move || {
                let result = work(progress.value(), stop.value()).map_err(|message| {
                    WorkerFailure::admit_with_fallback(
                        message.into(),
                        &worker_budget,
                        WORKER_FAILURE_ADMISSION_MESSAGE,
                    )
                });
                let _ = delivery.send(result);
            })?;
            Ok::<_, crate::shell::RetentionError>((receiver, cancelled, stage, fixed, task))
        };
        let (receiver, cancelled, stage, fixed, task) = match prepare() {
            Ok(prepared) => prepared,
            Err(_) => return json!({"error":WORKER_TRANSPORT_ADMISSION_MESSAGE}),
        };
        match std::thread::Builder::new()
            .name("merlin-workbench-open".into())
            .spawn(move || run_task(task))
        {
            Ok(_) => {
                log::debug!("workbench_open_started");
                self.job = Some(Job {
                    receiver,
                    cancelled,
                    stage,
                    started: Instant::now(),
                    _fixed_guard: fixed.retain(()),
                    saving: false,
                });
                self.status()
            }
            Err(error) => json!({"error":format!("Cannot start workbench loader: {error}")}),
        }
    }

    /// Requests cancellation; an in-progress validation stage finishes off-thread.
    /// # Returns
    /// No result; one-worker guard remains until completion is drained.
    pub(crate) fn cancel(&self) {
        if let Some(job) = &self.job {
            if job.saving {
                return;
            }
            job.cancelled.value().store(true, Ordering::Relaxed);
        }
    }

    /// Polls once without waiting for the picker, parsing or validation.
    /// # Returns
    /// A completion document and optional prepared result, or loading status.
    pub(crate) fn poll(&mut self) -> (Value, Option<PreparedProject>) {
        let Some(job) = &self.job else {
            return (json!({"loading":false}), None);
        };
        // A cancelled original message must be destroyed before its frame guard.
        let _delivery_guard;
        let message = match job.receiver.try_recv() {
            Err(TryRecvError::Empty) => return (self.status(), None),
            Err(TryRecvError::Disconnected) => {
                _delivery_guard = None;
                Err(WorkerFailure::Static(
                    "Workbench loader ended without a result.",
                ))
            }
            Ok(delivery) => {
                let (message, guard) = delivery.into_parts();
                _delivery_guard = Some(guard);
                message
            }
        };
        let job = self
            .job
            .take()
            .expect("Completion requires a tracked loader");
        if job.cancelled.value().load(Ordering::Relaxed) {
            log::debug!(
                "workbench_open_cancelled elapsed_ms={}",
                job.started.elapsed().as_millis()
            );
            return (json!({"cancelled":true}), None);
        }
        match message {
            Ok(Some(LoadCandidate::Downstream(prepared))) => {
                log::debug!(
                    "workbench_open_completed elapsed_ms={}",
                    job.started.elapsed().as_millis()
                );
                (json!({"loading":false}), Some(*prepared))
            }
            Ok(Some(LoadCandidate::Upstream(prepared))) => {
                self.upstream = Some(prepared);
                (json!({"loading":false,"upstream_detected":true}), None)
            }
            Ok(Some(LoadCandidate::Saved(saved))) => {
                self.saved = Some(saved);
                (json!({"loading":false,"write_completed":true}), None)
            }
            Ok(None) => (json!({"cancelled":true}), None),
            Err(error) => {
                log::error!(
                    "workbench_open_failed elapsed_ms={}; see the loading error dialog",
                    job.started.elapsed().as_millis()
                );
                (json!({"error":error.message()}), None)
            }
        }
    }

    fn status(&self) -> Value {
        let Some(job) = &self.job else {
            return json!({"loading":false});
        };
        let phase = if job.saving {
            "Saving workbench"
        } else if job.cancelled.value().load(Ordering::Relaxed) {
            "Cancelling"
        } else {
            match job.stage.value().load(Ordering::Relaxed) {
                PROJECT_CHOOSER_PHASE => "Choose a workbench",
                PROJECT_FILE_READ_PHASE => "Reading workbench",
                PROJECT_CONFIGURATION_PHASE => "Reading configuration",
                _ => "Validating captures and preparing workbench",
            }
        };
        json!({"loading":true,"saving":job.saving,"phase":phase,"elapsed_ms":job.started.elapsed().as_millis() as u64,"debug_ui":crate::platform::startup::debug_ui(),"cancelling":job.cancelled.value().load(Ordering::Relaxed)})
    }
}
impl Drop for Loader {
    fn drop(&mut self) {
        self.cancel();
    }
}

fn read_bounded(
    reader: &mut impl std::io::Read,
    cancelled: &AtomicBool,
    limit: usize,
) -> Result<Option<Vec<u8>>, String> {
    let mut bytes = Vec::new();
    let mut chunk = [0u8; PROJECT_READ_CHUNK_BYTES];
    loop {
        if cancelled.load(Ordering::Relaxed) {
            return Ok(None);
        }
        let capacity = chunk
            .len()
            .min(limit.saturating_sub(bytes.len()).saturating_add(1));
        let count = reader
            .read(&mut chunk[..capacity])
            .map_err(|error| error.to_string())?;
        if count == 0 {
            return Ok(Some(bytes));
        }
        if bytes.len().saturating_add(count) > limit {
            return Err("Workbench exceeds input limits.".into());
        }
        bytes.extend_from_slice(&chunk[..count]);
    }
}

#[cfg(test)]
mod project_retention_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    // Test-host scheduling bounds, independent of capture and scientific limits.
    const LOAD_TEST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);
    const LOAD_TEST_POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(1);

    /// Drains a real loader under the existing bounded test-host watchdog.
    /// # Arguments
    /// * `loader` - Tracked fixture worker; polling does not block on completion.
    /// # Returns
    /// Terminal status and the optional completed project.
    /// # Panics
    /// Panics if the fixture does not terminate before the watchdog.
    pub(super) fn drain(loader: &mut Loader) -> (Value, Option<PreparedProject>) {
        let deadline = Instant::now() + LOAD_TEST_TIMEOUT;
        loop {
            let result = loader.poll();
            if result.0["loading"] != true {
                return result;
            }
            assert!(Instant::now() < deadline, "Loader did not complete");
            std::thread::sleep(LOAD_TEST_POLL_INTERVAL);
        }
    }

    #[test]
    fn bounded_stream_preserves_bytes_limits_and_cancellation() {
        let cancelled = AtomicBool::new(false);
        assert_eq!(
            read_bounded(&mut std::io::Cursor::new(b"{}"), &cancelled, 2)
                .unwrap()
                .unwrap(),
            b"{}"
        );
        assert!(read_bounded(&mut std::io::Cursor::new(b"{} "), &cancelled, 2).is_err());
        cancelled.store(true, Ordering::Relaxed);
        assert!(
            read_bounded(&mut std::io::Cursor::new(b"{}"), &cancelled, 2)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn command_line_file_preserves_destination_and_rejects_invalid_json() {
        let mut loader = Loader::default();
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/workbench/downstream/single/workbench.merlin.json");
        let context = Context::isolated();
        loader.start_requested(
            &context,
            crate::platform::startup::AdmittedSource::admit(
                crate::platform::startup::WorkbenchSource::File(path.clone()),
                context.application.budget(),
            )
            .unwrap(),
        );
        let (status, prepared) = drain(&mut loader);
        assert!(prepared.is_some(), "{status}");
        assert_eq!(prepared.unwrap().path.unwrap().value(), &path);
        loader.start_requested(
            &context,
            crate::platform::startup::AdmittedSource::admit(
                crate::platform::startup::WorkbenchSource::File(
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
                ),
                context.application.budget(),
            )
            .unwrap(),
        );
        assert!(drain(&mut loader).0["error"]
            .as_str()
            .unwrap()
            .starts_with("Invalid workbench JSON"));
    }

    #[test]
    fn polling_does_not_wait_and_cancel_retains_single_worker_guard() {
        let (release, wait) = mpsc::sync_channel(1);
        let mut loader = Loader::default();
        let context = Context::isolated();
        assert_eq!(
            loader.start_with(context.application.budget(), move |_, _| {
                wait.recv().unwrap();
                Ok(None)
            })["loading"],
            true
        );
        for _ in 0..20 {
            assert_eq!(loader.poll().0["loading"], true);
        }
        loader.cancel();
        assert_eq!(loader.poll().0["cancelling"], true);
        assert!(
            loader.start_with(context.application.budget(), |_, _| panic!(
                "Must not start another worker"
            ))["error"]
                .is_string()
        );
        release.send(()).unwrap();
        assert_eq!(drain(&mut loader).0["cancelled"], true);
        assert!(!loader.is_loading());
    }

    #[test]
    fn errors_and_cancelled_success_do_not_publish_partial_inputs() {
        let context = &crate::downstream::workbench::session::Context::default();
        let previous = super::super::session::saved_sections(context);
        let mut loader = Loader::default();
        loader.start_with(context.application.budget(), |_, _| {
            Err("Invalid capture hash".into())
        });
        assert_eq!(drain(&mut loader).0["error"], "Invalid capture hash");
        let (release, wait) = mpsc::sync_channel(1);
        loader.start_with(context.application.budget(), move |_, _| {
            wait.recv().unwrap();
            let project = Project {
                schema_version: 2,
                application: "MERLIN".into(),
                transmission: Default::default(),
                sections: Default::default(),
                receiver: None,
                comparison_enabled: false,
                dark: true,
            };
            {
                let context = Context::isolated();
                PreparedProject::prepare(
                    project,
                    None,
                    &context.application,
                    context
                        .application
                        .snapshot(crate::shell::Direction::Downstream)
                        .key,
                    super::super::session::snapshot(&context),
                )
                .map(Some)
            }
        });
        loader.cancel();
        release.send(()).unwrap();
        let (status, result) = drain(&mut loader);
        assert_eq!(status["cancelled"], true);
        assert!(result.is_none());
        assert_eq!(&*super::super::session::saved_sections(context), &*previous);
    }

    #[test]
    fn real_multi_capture_workbench_prepares_editors_off_thread() {
        let mut loader = Loader::default();
        let context = Context::isolated();
        loader.start_with(context.application.budget(), |_, _| {
            let project = serde_json::from_slice(include_bytes!(
                "../../../../../examples/workbench/downstream/multi-rxmer-2/workbench.merlin.json"
            ))
            .unwrap();
            {
                let context = Context::isolated();
                PreparedProject::prepare(
                    project,
                    None,
                    &context.application,
                    context
                        .application
                        .snapshot(crate::shell::Direction::Downstream)
                        .key,
                    super::super::session::snapshot(&context),
                )
                .map(Some)
            }
        });
        let (_, result) = drain(&mut loader);
        let prepared = result.expect("Real curated workbench must load");
        assert!(
            prepared
                .models
                .measurements
                .as_ref()
                .unwrap()
                .value()
                .accepted_rx
                .len()
                > 100
        );
        assert!(prepared.models.receiver.is_some());
        assert_eq!(prepared.metadata.value()["prepared"], true);
        assert_eq!(prepared.metadata.value()["measurement_state"]["channel"]["status"],"complete","Background readiness must use restored inputs, not the worker's empty thread-local session");
        assert!(prepared.metadata.value().to_string().len() < 8192);
    }
}

#[cfg(test)]
#[path = "project_load/admission_tests.rs"]
mod admission_tests;

#[cfg(test)]
#[path = "project_load/target_tests.rs"]
mod target_tests;

#[cfg(test)]
#[path = "project_load/metadata_tests.rs"]
mod metadata_tests;

#[cfg(test)]
#[path = "project_load/transport_tests.rs"]
mod transport_tests;
