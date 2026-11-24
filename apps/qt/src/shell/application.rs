/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Canonical application identities and dispatch over existing direction owners.
use super::{
    constants::{MAX_SESSION_ORDERED_ENTRIES, MAX_SESSION_PAYLOAD_BYTES, MAX_SESSION_SHARED_ROOTS},
    ActiveRun, Direction, DispatchError, DispatchGate, ProjectSnapshot, Retained,
    RetainedResources, RetentionBudget, RetentionError, RunCancellation, RunLease, RunToken,
    SessionError, SessionKey, ShellSessions,
};
use std::{
    mem::size_of,
    sync::{Arc, Mutex, MutexGuard},
};

// These are direction-specific lifecycle metadata. Scientific configurations and
// actual reports stay in the existing Context/Model and upstream controller;
// readiness flags never establish scientific or aggregate-memory acceptance.
struct DownstreamReadiness(bool);
struct UpstreamReadiness(bool);
type Sessions = ShellSessions<DownstreamReadiness, (), UpstreamReadiness, ()>;

/// Application-host failure preserving previously accepted ownership.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplicationError {
    Retention(RetentionError),
    Session(SessionError),
    NotConfigured(Direction),
}

/// One shared canonical lifecycle host for independent native direction adapters.
///
/// Clone handles only; no scientific input or report is cloned. Retained science,
/// presentation copies, controls and transport still require their actual owner
/// charges. This host's fixed metadata charges alone do not prove that closure.
#[derive(Clone)]
pub struct ApplicationSessions {
    sessions: Arc<Mutex<Sessions>>,
    budget: RetentionBudget,
    gate: DispatchGate,
}

impl ApplicationSessions {
    /// Creates two unconfigured direction identities with explicit shared limits.
    ///
    /// # Arguments
    /// * `maximum` - Independent application payload/entry/root ceilings.
    ///
    /// # Returns
    /// An idle host initially selecting downstream; no backend or worker starts.
    ///
    /// # Errors
    /// Rejects insufficient fixed metadata admission or exhausted identity space.
    pub fn new(maximum: RetainedResources) -> Result<Self, ApplicationError> {
        let budget = RetentionBudget::with_limits(maximum);
        let gate = DispatchGate::default();
        let downstream = metadata(&budget, DownstreamReadiness(false))?;
        let upstream = metadata(&budget, UpstreamReadiness(false))?;
        let sessions = ShellSessions::new(budget.clone(), gate.clone(), downstream, upstream)
            .map_err(ApplicationError::Session)?;
        Ok(Self {
            sessions: Arc::new(Mutex::new(sessions)),
            budget,
            gate,
        })
    }

    /// Creates the normal desktop host using named application safety policies.
    ///
    /// # Returns
    /// An idle host with independent limits, not a scientific memory guarantee.
    ///
    /// # Errors
    /// Preserves fixed-admission or checked process-identity exhaustion failures.
    pub fn desktop() -> Result<Self, ApplicationError> {
        Self::new(RetainedResources {
            payload_bytes: MAX_SESSION_PAYLOAD_BYTES,
            ordered_entries: MAX_SESSION_ORDERED_ENTRIES,
            shared_roots: MAX_SESSION_SHARED_ROOTS,
        })
    }

    /// Borrows the same accounting policy used by actual admitted owner wrappers.
    ///
    /// # Returns
    /// Shared counters; callers must account their own complete retained payloads.
    pub fn budget(&self) -> &RetentionBudget {
        &self.budget
    }

    /// Reads the exact current direction identity, independent of tab selection.
    ///
    /// # Arguments
    /// * `direction` - Owner to inspect; no scientific readiness is inferred.
    ///
    /// # Returns
    /// Current configuration generation and dirty revision for this direction.
    pub fn snapshot(&self, direction: Direction) -> ProjectSnapshot {
        lock(&self.sessions).open_target(direction)
    }

    /// Adopts readiness after the downstream owner accepts a configuration change.
    ///
    /// # Arguments
    /// * `ready` - Whether that exact owner holds a validated public request.
    ///
    /// # Returns
    /// The new checked key; old work retains global admission until termination.
    ///
    /// # Errors
    /// Rejects metadata limits, shutdown or revision exhaustion before cancellation.
    pub fn configure_downstream(&self, ready: bool) -> Result<SessionKey, ApplicationError> {
        let candidate = metadata(&self.budget, DownstreamReadiness(ready))?;
        let mut sessions = lock(&self.sessions);
        sessions
            .configure_downstream_cancelling(candidate)
            .map_err(ApplicationError::Session)?;
        Ok(sessions.open_target(Direction::Downstream).key)
    }

    /// Adopts readiness after the upstream owner accepts its public configuration.
    ///
    /// # Arguments
    /// * `ready` - Whether a real prepared upstream session is available.
    ///
    /// # Returns
    /// The new checked key; downstream configuration/control is untouched.
    ///
    /// # Errors
    /// Rejects metadata limits, shutdown or revision exhaustion before cancellation.
    pub fn configure_upstream(&self, ready: bool) -> Result<SessionKey, ApplicationError> {
        let candidate = metadata(&self.budget, UpstreamReadiness(ready))?;
        let mut sessions = lock(&self.sessions);
        sessions
            .configure_upstream_cancelling(candidate)
            .map_err(ApplicationError::Session)?;
        Ok(sessions.open_target(Direction::Upstream).key)
    }

    /// Acquires the one global run lease for an exact already prepared owner.
    ///
    /// # Arguments
    /// * `key` - Immutable direction/session/generation accepted by the caller.
    /// * `control` - That run's actual nonblocking public-control adapter.
    ///
    /// # Returns
    /// Sole genuine lease; no scientific operation or thread starts here.
    ///
    /// # Errors
    /// Rejects stale keys, unconfigured owners, shutdown, exhaustion or global Busy.
    pub fn begin(
        &self,
        key: SessionKey,
        control: Arc<dyn RunCancellation>,
    ) -> Result<RunLease, ApplicationError> {
        let mut sessions = lock(&self.sessions);
        if sessions.open_target(key.direction).key != key {
            return Err(ApplicationError::Session(SessionError::StaleGuard));
        }
        let ready = match key.direction {
            Direction::Downstream => sessions.downstream().configuration().0,
            Direction::Upstream => sessions.upstream().configuration().0,
        };
        if !ready {
            return Err(ApplicationError::NotConfigured(key.direction));
        }
        sessions
            .begin(key.direction, control)
            .map_err(ApplicationError::Session)
    }

    /// Revalidates a callback immediately before its actual direction-store adoption.
    ///
    /// # Arguments
    /// * `token` - Genuine worker identity carried by the common transport.
    ///
    /// # Errors
    /// Rejects shutdown and any stale session, generation, run or direction.
    pub fn validate_callback(&self, token: RunToken) -> Result<(), ApplicationError> {
        lock(&self.sessions)
            .validate_callback(token)
            .map_err(ApplicationError::Session)
    }

    /// Consumes a genuine lease after known termination or unstarted spawn failure.
    ///
    /// # Arguments
    /// * `lease` - Sole ownership received from the common terminal/spawn result.
    ///
    /// # Errors
    /// Reports stale/shutdown callbacks while still acknowledging actual termination.
    pub fn finish(&self, lease: RunLease) -> Result<(), ApplicationError> {
        let mut sessions = lock(&self.sessions);
        match lease.token().key.direction {
            Direction::Downstream => sessions.finish_downstream(lease, None),
            Direction::Upstream => sessions.finish_upstream(lease, None),
        }
        .map_err(ApplicationError::Session)
    }

    /// Reads the one active owner, including pause and cancellation until termination.
    ///
    /// # Returns
    /// None only while no global lease is held.
    pub fn active(&self) -> Option<ActiveRun> {
        self.gate.active()
    }

    /// Requests cancellation of only the exact active owner without releasing it.
    ///
    /// # Arguments
    /// * `token` - Original genuine worker identity, never the selected tab.
    ///
    /// # Errors
    /// Rejects stale identities without changing another run's public control.
    pub fn cancel(&self, token: RunToken) -> Result<(), DispatchError> {
        self.gate.request_cancel(token)
    }

    /// Records pause/resume already acknowledged by the actual backend control.
    ///
    /// # Arguments
    /// * `token` - Exact active worker identity.
    /// * `paused` - Observed backend state; a pending request is insufficient.
    ///
    /// # Errors
    /// Rejects stale or cancelling runs without releasing the global slot.
    pub fn acknowledge_pause(&self, token: RunToken, paused: bool) -> Result<(), DispatchError> {
        self.gate.acknowledge_pause(token, paused)
    }
}

fn metadata<T>(budget: &RetentionBudget, value: T) -> Result<Retained<T>, ApplicationError> {
    budget
        .reserve_resources(RetainedResources {
            payload_bytes: size_of::<Retained<T>>(),
            ..Default::default()
        })
        .map(|reservation| reservation.retain(value))
        .map_err(ApplicationError::Retention)
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|cause| cause.into_inner())
}
