/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual upstream sessions and reports, without another scientific interface.
mod failure;
mod project;
mod project_restore;
mod source;
use crate::shell::{
    spawn_worker, Direction, Retained, RetainedResources, RetentionBudget, RetentionError,
    RetentionReservation, RunCancellation, RunLease, RunToken, SessionKey, WorkerEvent,
    WorkerReceiver, WorkerTerminal,
};
pub use failure::RunFailure;
use merlin::{
    common::RetainedFootprint,
    upstream::{
        inputs::MeasuredExperiment, ApiError, BatchReport, ConfigurationDocument, ProgressSnapshot,
        ReceiverObservationPolicy, RunControl, Simulation,
    },
};
use source::retain_measured_source;
pub(crate) use source::ProjectSource;
use std::{
    io,
    mem::size_of,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::TryRecvError,
        Arc, Mutex, MutexGuard, TryLockError,
    },
};

/// Actual report and reservation shared by the controller, queue and old views.
///
/// Cloning this Arc shares its charge and payload. Cloning the underlying report
/// or a nested allocation requires a separate host reservation before allocation.
pub type RetainedReport = Arc<Retained<merlin::upstream::BatchReport>>;

type SharedSession = Arc<Retained<Mutex<SessionStorage>>>;

// One real batch report at most; known terminal has its own delivery permit.
const RUN_REPORT_EVENTS: usize = 1;

type Events = WorkerReceiver<RetainedReport, RunFailure>;

struct SessionStorage {
    simulation: Option<Simulation>,
    measured_source: Option<Retained<Box<[u8]>>>,
}

/// Immutable source inventory from the successfully admitted measured plan.
#[derive(Clone, Copy)]
pub(crate) struct MeasuredInputInfo {
    /// Physical source fragments, kept distinct from their derived reassembly.
    pub(crate) ucd_fragments: usize,
    /// Explicitly associated measured RxMER files.
    pub(crate) rxmer_captures: usize,
    /// Whether the plan supplies an original initial applied-bank capture.
    pub(crate) pre_eq_initial: bool,
    /// Original update/snapshot sources with explicit replay context.
    pub(crate) pre_eq_events: usize,
}

/// Off-GUI admitted scientific session, carrying a backend-derived ownership charge.
pub struct PreparedSession {
    session: SharedSession,
    budget: RetentionBudget,
    measured_input: Option<MeasuredInputInfo>,
}

/// Target reservation prepared off-GUI for one exact immutable controller session.
///
/// Not cloneable: Start consumes this report reservation. Dropping an unused or
/// rejected preparation releases its target while keeping other owners charged.
pub struct PreparedRun {
    key: SessionKey,
    owner: SharedSession,
    backend_limit: RetainedFootprint,
    reservation: RetentionReservation,
}

/// Typed off-GUI admission failure; previously accepted ownership is unchanged.
#[derive(Debug)]
pub enum AdmissionError {
    Backend(ApiError),
    Retention(RetentionError),
    Controller(ControllerError),
}

/// Nonblocking public control adapter shared with global dispatch admission.
///
/// The same Arc must be passed to `ShellSessions::begin` and controller `start`.
/// Cancellation is requested once across both owners; backend pause/resume and
/// progress semantics remain unchanged. Use one fresh backend control per run.
pub struct UpstreamControl {
    backend: RunControl,
    cancellation_requested: AtomicBool,

    // All backend state drops before its adapter's one shared reservation.
    guard: Retained<()>,
}

/// Application rejection; accepted configuration and previous report survive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControllerError {
    WrongDirection,
    Busy,
    StaleToken,
    StalePreparation,
    WorkerDisconnected,
    ForeignControl,
}

/// Failure before worker admission, retaining the original unstarted lease.
#[derive(Debug)]
pub enum StartFailure {
    Controller(ControllerError),
    Spawn(io::Error),
}

/// Rejected Start; the host must acknowledge its returned unstarted lease.
pub struct StartError {
    reason: StartFailure,
    lease: RunLease,
}

/// One nonblocking poll result; report and terminal retain their original token.
pub enum ControllerEvent {
    Idle,
    Pending,
    Report {
        token: RunToken,
        report: RetainedReport,
    },
    /// Host reads the original category/bounded text, then consumes or drops the lease.
    Terminal(WorkerTerminal<RunFailure>),
}

struct Active {
    token: RunToken,
    control: Arc<UpstreamControl>,
    events: Events,
}

/// One immutable accepted upstream configuration with independently owned results.
///
/// Replacement is host-owned: prepare a new controller off-GUI and adopt only
/// after the shell rechecks the generation/Open guard. This owner never changes
/// accepted science under an unchanged key. Its shared budget admits actual
/// scientific root quantities and fixed retained-wrapper storage before adoption.
/// This is a partial application proof: outer controller/preparation,
/// transport and DTO owners need separate host charges. The canonical host already
/// admits its fixed session, gate and accounting pointees.
pub struct UpstreamController {
    key: SessionKey,
    // Immutable admission route; available even while the scientific owner runs.
    measured_input: Option<MeasuredInputInfo>,

    // Drop the receiver before this owner's session/result aliases. A disconnected
    // worker finishes through the shared transport's terminal acknowledgement.
    active: Option<Active>,
    session: SharedSession,
    budget: RetentionBudget,
    report: Option<(RunToken, RetainedReport)>,
}

struct RunningSession {
    simulation: Option<Simulation>,
    owner: SharedSession,
}

impl PreparedSession {
    /// Builds a real public session off-GUI, then admits its final retained owner.
    ///
    /// # Arguments
    /// * `document` - Owned source document; admission preserves its exact provenance.
    /// * `observations` - Explicit validated receiver retention policy.
    /// * `budget` - Shared application policy enforcing all five dimensions.
    ///
    /// # Returns
    /// An idle session reserved from actual Simulation metadata plus checked
    /// fixed Mutex/Option/Retained storage and its outer shared root. The incoming
    /// document owner remains the caller's separately charged input responsibility.
    /// Temporary backend builders retain their existing finite execution limits;
    /// this does not claim an aggregate preparation-peak or scratch-byte bound.
    /// No source, run, backend control or output file is started.
    ///
    /// # Errors
    /// Preserves typed public admission errors and budget/checked-layout failures.
    /// Failure drops only the candidate; accepted controllers/reports survive.
    pub fn from_document(
        document: ConfigurationDocument,
        observations: ReceiverObservationPolicy,
        budget: &RetentionBudget,
    ) -> Result<Self, AdmissionError> {
        let (_, options) = document.prepare().map_err(AdmissionError::Backend)?;
        let mut simulation = Simulation::new(options).map_err(AdmissionError::Backend)?;
        simulation
            .configure_document(document)
            .map_err(AdmissionError::Backend)?;
        simulation
            .set_receiver_observation_policy(observations)
            .map_err(AdmissionError::Backend)?;
        Self::retain_simulation(simulation, budget, None, None)
    }

    /// Admits a complete measured plan through the existing public native owners.
    ///
    /// # Arguments
    /// * `plan` - Original complete capture/association/replay context, never inferred.
    /// * `observations` - Explicit existing receiver retention policy.
    /// * `budget` - Same canonical application policy as synthetic sessions.
    ///
    /// # Returns
    /// An idle admitted session retaining original plan/capture provenance and the
    /// actual projected scientific configuration. No simulation or source advances.
    ///
    /// # Errors
    /// Public measured admission, observation or existing retention failures.
    /// Previously accepted controllers and reports are never modified.
    pub fn from_measured(
        plan: MeasuredExperiment,
        observations: ReceiverObservationPolicy,
        budget: &RetentionBudget,
    ) -> Result<Self, AdmissionError> {
        let measured = plan.admit().map_err(AdmissionError::Backend)?;
        let measured_source = retain_measured_source(&plan, budget)?;
        let inventory = MeasuredInputInfo {
            ucd_fragments: plan.ucd_fragments.len(),
            rxmer_captures: plan.rxmer.len(),
            pre_eq_initial: plan.pre_eq_initial.is_some(),
            pre_eq_events: plan.pre_eq_events.len(),
        };
        // Only an empty construction policy: configure_measured adopts the plan's
        // validated options atomically before this candidate becomes observable.
        let mut simulation =
            Simulation::new(Default::default()).map_err(AdmissionError::Backend)?;
        simulation
            .configure_measured(measured)
            .map_err(AdmissionError::Backend)?;
        simulation
            .set_receiver_observation_policy(observations)
            .map_err(AdmissionError::Backend)?;
        Self::retain_simulation(simulation, budget, Some(inventory), Some(measured_source))
    }

    fn retain_simulation(
        simulation: Simulation,
        budget: &RetentionBudget,
        measured_input: Option<MeasuredInputInfo>,
        measured_source: Option<Retained<Box<[u8]>>>,
    ) -> Result<Self, AdmissionError> {
        let scientific = simulation
            .retained_footprint()
            .map_err(AdmissionError::Backend)?;
        let resources = session_resources(scientific).map_err(AdmissionError::Retention)?;
        let reservation = budget
            .reserve_resources(resources)
            .map_err(AdmissionError::Retention)?;
        let session = Arc::new(reservation.retain(Mutex::new(SessionStorage {
            simulation: Some(simulation),
            measured_source,
        })));
        Ok(Self {
            session,
            budget: budget.clone(),
            measured_input,
        })
    }
}

impl UpstreamControl {
    /// Admits a real public lifecycle handle before allocating its shared adapter.
    ///
    /// # Arguments
    /// * `backend` - Fresh per-run public control, optionally pre-paused/cancelled.
    /// * `budget` - Exact canonical counter used by both the controller and gate.
    ///
    /// # Returns
    /// One guarded Arc shared by gate, controller and worker; no raw clone escapes.
    ///
    /// # Errors
    /// Public metadata or any checked aggregate dimension before lease acquisition.
    pub fn new(backend: RunControl, budget: &RetentionBudget) -> Result<Arc<Self>, AdmissionError> {
        let scientific = backend
            .retained_footprint()
            .map_err(|error| AdmissionError::Backend(error.into()))?;
        let resources =
            wrapped_resources::<Self, RunControl>(scientific).map_err(AdmissionError::Retention)?;
        let reservation = budget
            .reserve_resources(resources)
            .map_err(AdmissionError::Retention)?;
        Ok(Arc::new(Self {
            backend,
            cancellation_requested: AtomicBool::new(false),
            guard: reservation.retain(()),
        }))
    }
}

impl RunCancellation for UpstreamControl {
    /// Requests cooperative cancellation once without joining or releasing a lease.
    ///
    /// Repeated gate/controller/destruction requests share this same adapter.
    fn request_cancel(&self) {
        if !self.cancellation_requested.swap(true, Ordering::AcqRel) {
            self.backend.cancel();
        }
    }
}

impl StartError {
    /// Recovers a rejected Start without abandoning its genuine dispatch lease.
    ///
    /// # Returns
    /// Typed cause and original unstarted lease; acknowledge before another Start.
    pub fn into_parts(self) -> (StartFailure, RunLease) {
        (self.reason, self.lease)
    }
}

impl UpstreamController {
    /// Binds a prevalidated session to its exact host-issued upstream identity.
    ///
    /// # Arguments
    /// * `key` - Accepted upstream session/generation from the shell coordinator.
    /// * `prepared` - Independently admitted, reserved actual scientific session.
    ///
    /// # Returns
    /// Idle owner with no result; neither configuration nor identity is synthesized.
    ///
    /// # Errors
    /// Rejects a downstream key without changing any existing controller.
    pub fn new(key: SessionKey, prepared: PreparedSession) -> Result<Self, ControllerError> {
        if key.direction != Direction::Upstream {
            return Err(ControllerError::WrongDirection);
        }
        Ok(Self {
            key,
            measured_input: prepared.measured_input,
            active: None,
            session: prepared.session,
            budget: prepared.budget,
            report: None,
        })
    }

    /// Reports which public admission operation created this immutable owner.
    ///
    /// # Returns
    /// True only after successful measured-plan admission. This adapter tag stays
    /// available during execution; it never infers capture science from results.
    pub(crate) fn has_measured_input(&self) -> bool {
        self.measured_input.is_some()
    }

    /// Reads the inventory supplied by successful native measured admission.
    ///
    /// # Returns
    /// Fixed source counts, or None for a synthetic input. No scientific buffers
    /// or inferred associations are exposed, and no session lock is required.
    pub(crate) fn measured_input(&self) -> Option<MeasuredInputInfo> {
        self.measured_input
    }

    /// Borrows the actual idle session without exposing mutable scientific state.
    ///
    /// # Arguments
    /// * `R` - Caller-owned inspection result; not a borrowed session reference.
    /// * `inspect` - Read-only inspection; reserve separately before cloning owned
    ///   documents/configurations or allocating public instrument snapshots.
    ///
    /// # Returns
    /// The closure result. The short session lock is released before returning;
    /// expensive public queries remain off-GUI work owned by the caller.
    ///
    /// # Errors
    /// Busy while an operation/terminal is active or another inspector holds the lock.
    pub fn inspect<R>(&self, inspect: impl FnOnce(&Simulation) -> R) -> Result<R, ControllerError> {
        if self.active.is_some() {
            return Err(ControllerError::Busy);
        }
        let storage = match self.session.value().try_lock() {
            Ok(storage) => storage,
            Err(TryLockError::Poisoned(cause)) => cause.into_inner(),
            Err(TryLockError::WouldBlock) => return Err(ControllerError::Busy),
        };
        let simulation = storage.simulation.as_ref().ok_or(ControllerError::Busy)?;
        Ok(inspect(simulation))
    }

    /// Preflights and reserves the actual public report target off-GUI while idle.
    ///
    /// # Returns
    /// A single-use reservation bound to this exact immutable session and key.
    /// The future host moves the idle controller to a preparation worker for this
    /// potentially expensive public query, then returns both owners to the GUI.
    /// Call before acquiring a global lease. Existing accepted reports and source
    /// state are unchanged; independently retained old/queued aliases stay charged.
    /// Temporary public preflight builders have separate finite backend limits.
    ///
    /// # Errors
    /// Busy while any worker/terminal is active or the scientific slot is held.
    /// Preserves typed backend preflight and all checked budget-dimension errors.
    pub fn prepare_run(&self) -> Result<PreparedRun, AdmissionError> {
        if self.active.is_some() {
            return Err(AdmissionError::Controller(ControllerError::Busy));
        }
        let storage = match self.session.value().try_lock() {
            Ok(storage) => storage,
            Err(TryLockError::Poisoned(cause)) => cause.into_inner(),
            Err(TryLockError::WouldBlock) => {
                return Err(AdmissionError::Controller(ControllerError::Busy))
            }
        };
        let simulation = storage
            .simulation
            .as_ref()
            .ok_or(AdmissionError::Controller(ControllerError::Busy))?;
        let backend_limit = simulation
            .batch_report_retained_bound()
            .map_err(AdmissionError::Backend)?;
        let resources = report_resources(backend_limit).map_err(AdmissionError::Retention)?;
        let reservation = self
            .budget
            .reserve_resources(resources)
            .map_err(AdmissionError::Retention)?;
        Ok(PreparedRun {
            key: self.key,
            owner: Arc::clone(&self.session),
            backend_limit,
            reservation,
        })
    }

    /// Dispatches the existing public bounded run using its pre-reserved target.
    ///
    /// # Arguments
    /// * `lease` - Genuine global admission for this exact host session/generation.
    /// * `control` - Same control adapter Arc admitted by the shell for this run.
    /// * `prepared` - Off-GUI target admitted before acquiring the global lease.
    /// * `capacity` - Host-validated bounded callback-count policy; no default here.
    ///
    /// # Returns
    /// Immediately after shared transport dispatch; polling never joins workers.
    /// Source/configuration and target charges survive every worker/queued/old
    /// alias. No oversized-result rejection occurs after scientific execution.
    /// Shared channel and other host owners require separate application accounting.
    ///
    /// # Errors
    /// Busy, stale identity/preparation, foreign control or OS spawn failure returns
    /// the original unstarted lease. The consumed target releases on rejection; old reports
    /// survive. The host acknowledges unstarted failure before another Start.
    pub fn start(
        &mut self,
        lease: RunLease,
        control: Arc<UpstreamControl>,
        prepared: PreparedRun,
        capacity: usize,
    ) -> Result<(), StartError> {
        let token = lease.token();
        let rejection = if self.active.is_some() {
            Some(ControllerError::Busy)
        } else if token.key != self.key {
            Some(ControllerError::StaleToken)
        } else if prepared.key != self.key || !Arc::ptr_eq(&prepared.owner, &self.session) {
            Some(ControllerError::StalePreparation)
        } else if !self.budget.owns(&control.guard) {
            Some(ControllerError::ForeignControl)
        } else {
            None
        };
        if let Some(reason) = rejection {
            return Err(StartError {
                reason: StartFailure::Controller(reason),
                lease,
            });
        }
        let backend = Arc::clone(&control);
        let budget = self.budget.clone();
        let events = spawn_worker(
            "merlin-upstream",
            capacity,
            RUN_REPORT_EVENTS,
            &self.budget,
            lease,
            move |mut publisher| {
                let PreparedRun {
                    key: _,
                    owner,
                    backend_limit,
                    reservation,
                } = prepared;
                let mut session = RunningSession::take(owner)
                    .map_err(|error| RunFailure::admit(error, &budget))?;
                let report = session
                    .simulation
                    .as_mut()
                    .expect("Running guard owns its session")
                    .run_with_control_bounded(&backend.backend, backend_limit)
                    .map_err(|error| RunFailure::admit(error, &budget))?;

                // Restore accepted configuration before publishing independent results.
                drop(session);
                let report = Arc::new(reservation.retain(report));

                // Receiver destruction discards the actual report with its charge.
                let _ = publisher.send(report);
                Ok(())
            },
        )
        .map_err(|failure| {
            let (cause, lease) = failure.into_parts();
            StartError {
                reason: StartFailure::Spawn(cause),
                lease,
            }
        })?;
        self.active = Some(Active {
            token,
            control,
            events,
        });
        Ok(())
    }

    /// Reads one bounded scalar snapshot from the actual public lifecycle handle.
    ///
    /// # Returns
    /// Active token and backend snapshot, or None after terminal delivery. Paused
    /// acknowledgement is the backend's status, not the earlier pause request.
    pub fn progress(&self) -> Option<(RunToken, ProgressSnapshot)> {
        self.active
            .as_ref()
            .map(|active| (active.token, active.control.backend.progress()))
    }

    /// Requests public pause for the exact active run without releasing admission.
    ///
    /// # Arguments
    /// * `token` - Original direction/session/generation/run identity.
    ///
    /// # Returns
    /// Unit after the immediate request; backend acknowledgement is in progress.
    ///
    /// # Errors
    /// Rejects absent/stale runs without redirecting to another control.
    pub fn pause(&self, token: RunToken) -> Result<(), ControllerError> {
        self.active_for(token)?.control.backend.pause();
        Ok(())
    }

    /// Requests public resume for the exact active run without changing selection.
    ///
    /// # Arguments
    /// * `token` - Original direction/session/generation/run identity.
    ///
    /// # Returns
    /// Unit after the immediate request; backend acknowledgement is in progress.
    ///
    /// # Errors
    /// Rejects absent/stale runs without redirecting to another control.
    pub fn resume(&self, token: RunToken) -> Result<(), ControllerError> {
        self.active_for(token)?.control.backend.resume();
        Ok(())
    }

    /// Requests cancellation once; only actual terminal acknowledgement frees admission.
    ///
    /// # Arguments
    /// * `token` - Original direction/session/generation/run identity.
    ///
    /// # Returns
    /// Unit after the immediate request; backend acknowledgement is in progress.
    ///
    /// # Errors
    /// Rejects absent/stale runs without touching another run's control.
    pub fn cancel(&self, token: RunToken) -> Result<(), ControllerError> {
        self.active_for(token)?.control.request_cancel();
        Ok(())
    }

    /// Shares the last actual report without copying its scientific allocations.
    ///
    /// # Returns
    /// Original callback identity and charged Arc, or None before accepted results.
    /// Old views keep their reservation after replacement/controller destruction.
    pub fn report(&self) -> Option<(RunToken, RetainedReport)> {
        self.report
            .as_ref()
            .map(|(token, report)| (*token, Arc::clone(report)))
    }

    /// Consumes at most one callback without waiting for work or terminal delivery.
    ///
    /// # Returns
    /// Idle/Pending, an adopted actual report, or the original typed terminal.
    /// Host checks terminal status and consumes its lease or drops it; every known
    /// terminal is acknowledged even if discarded. Failure/panic retains old data.
    /// Checks use this immutable owner's key; the host must also validate callbacks
    /// against canonical ShellSessions before applying any fields to current UI.
    ///
    /// # Errors
    /// Rejects mismatched tokens without adoption. Disconnection without terminal
    /// retains Busy conservatively; host must establish actual worker termination.
    pub fn poll(&mut self) -> Result<ControllerEvent, ControllerError> {
        let Some(active) = self.active.as_ref() else {
            return Ok(ControllerEvent::Idle);
        };
        let event = match active.events.try_recv() {
            Ok(event) => event,
            Err(TryRecvError::Empty) => return Ok(ControllerEvent::Pending),
            Err(TryRecvError::Disconnected) => return Err(ControllerError::WorkerDisconnected),
        };
        match event {
            WorkerEvent::Message { token, value } => {
                let _delivery_guard;
                let (value, guard) = value.into_parts();
                _delivery_guard = guard;
                self.active_for(token)?;
                self.report = Some((token, Arc::clone(&value)));
                Ok(ControllerEvent::Report {
                    token,
                    report: value,
                })
            }
            WorkerEvent::Terminal(terminal) => {
                self.active_for(terminal.token())?;
                self.active = None;
                Ok(ControllerEvent::Terminal(terminal))
            }
        }
    }

    fn active_for(&self, token: RunToken) -> Result<&Active, ControllerError> {
        self.active
            .as_ref()
            .filter(|active| active.token == token && token.key == self.key)
            .ok_or(ControllerError::StaleToken)
    }
}

impl RunningSession {
    fn take(owner: SharedSession) -> Result<Self, ApiError> {
        let simulation = lock(owner.value())
            .simulation
            .take()
            .ok_or(ApiError::AdapterFailure(
                "upstream.application.session_busy",
            ))?;
        Ok(Self {
            simulation: Some(simulation),
            owner,
        })
    }
}

impl Drop for RunningSession {
    /// Restores the actual accepted session after normal return, API error or unwind.
    ///
    /// Scientific execution holds no slot lock. The reservation remains in the
    /// shared owner until both worker and controller aliases have been destroyed.
    fn drop(&mut self) {
        lock(self.owner.value()).simulation = self.simulation.take();
    }
}

impl Drop for UpstreamController {
    /// Requests cooperative cancellation once and drops receivers without joining.
    ///
    /// Worker-owned configuration/result charges survive this owner; shared
    /// terminal transport acknowledges only after actual return or unwind.
    fn drop(&mut self) {
        if let Some(active) = &self.active {
            active.control.request_cancel();
        }
    }
}

// Scientific root metadata already includes its fixed value. The concrete target
// wrapper's fixed layout replaces that root exactly once; no guessed multiplier.
// The outer Arc is one new shared allocation root. Budget-counter pointees and
// all other application Arc handles remain separately owned by the host.
fn wrapped_resources<Wrapper, Scientific>(
    scientific: RetainedFootprint,
) -> Result<RetainedResources, RetentionError> {
    let children = scientific
        .payload_bytes
        .checked_sub(size_of::<Scientific>())
        .ok_or(RetentionError::ArithmeticOverflow)?;
    Ok(RetainedResources {
        payload_bytes: size_of::<Wrapper>()
            .checked_add(children)
            .ok_or(RetentionError::ArithmeticOverflow)?,
        ordered_entries: scientific.ordered_entries,
        shared_roots: scientific
            .shared_roots
            .checked_add(1)
            .ok_or(RetentionError::ArithmeticOverflow)?,
        ..RetainedResources::default()
    })
}

fn session_resources(scientific: RetainedFootprint) -> Result<RetainedResources, RetentionError> {
    wrapped_resources::<Retained<Mutex<SessionStorage>>, Simulation>(scientific)
}

fn report_resources(scientific: RetainedFootprint) -> Result<RetainedResources, RetentionError> {
    wrapped_resources::<Retained<BatchReport>, BatchReport>(scientific)
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|cause| cause.into_inner())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

#[cfg(test)]
#[path = "session/control_retention_tests.rs"]
mod control_retention_tests;

#[cfg(test)]
#[path = "session/failure_tests.rs"]
mod failure_tests;

#[cfg(test)]
#[path = "session/transport_retention_tests.rs"]
mod transport_retention_tests;
