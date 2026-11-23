/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual upstream sessions and reports, without another scientific interface.
use crate::shell::{
    spawn_worker, Direction, Retained, RetentionReservation, RunCancellation, RunLease, RunToken,
    SessionKey, WorkerEvent, WorkerTerminal,
};
use merlin::upstream::{
    ApiError, ConfigurationDocument, ProgressSnapshot, ReceiverObservationPolicy, RunControl,
    Simulation,
};
use std::{
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, TryRecvError},
        Arc, Mutex, MutexGuard, TryLockError,
    },
};

/// Actual report and reservation shared by the controller, queue and old views.
///
/// Cloning this Arc shares its charge and payload. Cloning the underlying report
/// or a nested allocation requires a separate host reservation before allocation.
pub type RetainedReport = Arc<Retained<merlin::upstream::BatchReport>>;

type SharedSession = Arc<Retained<Mutex<SessionStorage>>>;
type Events = Receiver<WorkerEvent<RetainedReport, ApiError>>;

struct SessionStorage {
    // The reservation owns the original document even during failed preparation.
    preparing: Option<ConfigurationDocument>,
    simulation: Option<Simulation>,
}

/// Off-GUI admitted scientific session, carrying its host's ownership charge.
pub struct PreparedSession {
    session: SharedSession,
}

/// Nonblocking public control adapter shared with global dispatch admission.
///
/// The same Arc must be passed to `ShellSessions::begin` and controller `start`.
/// Cancellation is requested once across both owners; backend pause/resume and
/// progress semantics remain unchanged. Use one fresh backend control per run.
pub struct UpstreamControl {
    backend: RunControl,
    cancellation_requested: AtomicBool,
}

/// Application rejection; accepted configuration and previous report survive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControllerError {
    WrongDirection,
    Busy,
    StaleToken,
    WorkerDisconnected,
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
    /// Host inspects the original typed status, then consumes or drops the lease.
    Terminal(WorkerTerminal<ApiError>),
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
/// accepted science under an unchanged key. It owns no budget; reservations are
/// trusted host inputs, not estimates or proof of a backend memory ceiling.
pub struct UpstreamController {
    key: SessionKey,

    // Drop the receiver before this owner's session/result aliases. A disconnected
    // worker finishes through the shared transport's terminal acknowledgement.
    active: Option<Active>,
    session: SharedSession,
    report: Option<(RunToken, RetainedReport)>,
}

struct RunningSession {
    simulation: Option<Simulation>,
    owner: SharedSession,
}

impl PreparedSession {
    /// Admits an actual configuration document through public upstream operations.
    ///
    /// # Arguments
    /// * `document` - Owned source document; admission retains its exact provenance.
    /// * `observations` - Explicit validated receiver retention policy.
    /// * `reservation` - Host charge reserved before document/preparation allocation;
    ///   it must cover reachable session data and preparation peak ownership.
    ///
    /// # Returns
    /// An idle scientific session; call off-GUI because public admission allocates.
    /// No source, run, worker or output file is started. External input aliases
    /// remain the host's separately charged responsibility.
    ///
    /// # Errors
    /// Preserves the original public validation error; no existing owner changes.
    pub fn from_document(
        document: ConfigurationDocument,
        observations: ReceiverObservationPolicy,
        reservation: RetentionReservation,
    ) -> Result<Self, ApiError> {
        let session = Arc::new(reservation.retain(Mutex::new(SessionStorage {
            preparing: Some(document),
            simulation: None,
        })));
        let document = lock(session.value())
            .preparing
            .take()
            .expect("Preparation owns its source document");
        let (_, options) = document.prepare()?;
        let mut simulation = Simulation::new(options)?;
        simulation.configure_document(document)?;
        simulation.set_receiver_observation_policy(observations)?;
        lock(session.value()).simulation = Some(simulation);
        Ok(Self { session })
    }
}

impl UpstreamControl {
    /// Wraps the caller's real public lifecycle handle without starting a run.
    ///
    /// # Arguments
    /// * `backend` - Fresh per-run public control, optionally pre-paused/cancelled.
    ///
    /// # Returns
    /// Adapter to share via Arc with the shell gate and upstream controller.
    pub fn new(backend: RunControl) -> Self {
        Self {
            backend,
            cancellation_requested: AtomicBool::new(false),
        }
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
            active: None,
            session: prepared.session,
            report: None,
        })
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

    /// Dispatches only the existing public `Simulation::run_with_control` operation.
    ///
    /// # Arguments
    /// * `lease` - Genuine global admission for this exact key; never a tab-derived ID.
    /// * `control` - Same adapter Arc admitted by the shell gate for this run.
    /// * `result_reservation` - Independently audited host charge reserved before
    ///   running; it must cover all actual result allocations and queued ownership.
    /// * `capacity` - Host-validated bounded callback-count policy; no default here.
    ///
    /// # Returns
    /// Immediately after shared transport dispatch; polling never joins the worker.
    /// The configuration charge survives all worker aliases. Previous reports stay
    /// accepted until an identified actual report is received.
    ///
    /// # Errors
    /// Busy, mismatched identity or OS spawn failure returns the original lease.
    /// The host must acknowledge unstarted failure; no active lease is abandoned.
    pub fn start(
        &mut self,
        lease: RunLease,
        control: Arc<UpstreamControl>,
        result_reservation: RetentionReservation,
        capacity: usize,
    ) -> Result<(), StartError> {
        let token = lease.token();
        let rejection = if self.active.is_some() {
            Some(ControllerError::Busy)
        } else if token.key != self.key {
            Some(ControllerError::StaleToken)
        } else {
            None
        };
        if let Some(reason) = rejection {
            return Err(StartError {
                reason: StartFailure::Controller(reason),
                lease,
            });
        }
        let owner = Arc::clone(&self.session);
        let backend = Arc::clone(&control);
        let events = spawn_worker("merlin-upstream", capacity, lease, move |publisher| {
            let mut session = RunningSession::take(owner)?;
            let report = session
                .simulation
                .as_mut()
                .expect("Running guard owns its session")
                .run_with_control(&backend.backend)?;

            // Restore accepted configuration before publishing independent results.
            drop(session);
            let report = Arc::new(result_reservation.retain(report));

            // Receiver destruction discards the actual report with its charge.
            let _ = publisher.send(report);
            Ok(())
        })
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

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|cause| cause.into_inner())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
