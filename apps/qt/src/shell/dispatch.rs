/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! One application run slot; a cancellation request is not terminal acknowledgement.
use super::{
    identity::{RunToken, SessionId},
    RetainedResources, RetentionBudget, RetentionError, RetentionReservation,
};
use std::{
    mem::size_of,
    sync::{Arc, Mutex, MutexGuard},
};

/// Thin application adapter around the owning direction's existing public control.
pub trait RunCancellation: Send + Sync {
    /// Requests cooperative cancellation without joining or waiting for the worker.
    ///
    /// Implementations must be nonblocking and must not panic. The worker retains
    /// its execution state until termination; a request does not release admission.
    fn request_cancel(&self);
}

/// Presentation dispatch phase; scientific pause semantics remain backend-owned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DispatchPhase {
    Running,
    Paused,
    Cancelling,
}

/// Snapshot of the single occupied execution slot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActiveRun {
    pub token: RunToken,
    pub phase: DispatchPhase,
}

/// Rejected application dispatch transition, preserving the current lease.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DispatchError {
    Busy(ActiveRun),
    StaleCallback,
    Cancelling,
}

struct Occupied {
    active: ActiveRun,
    cancellation: Arc<dyn RunCancellation>,
    cancellation_requested: bool,
}

#[derive(Default)]
struct GateStorage {
    state: Mutex<Option<Occupied>>,

    // Destroy the slot and its control before releasing the fixed gate charge.
    _reservation: Option<RetentionReservation>,
}

// Every admitted gate has one shared pointee, reused by every worker lease.
const APPLICATION_GATE_ROOTS: usize = 1;

/// Application-owned dispatch gate, shared with workers and successor UI owners.
///
/// Every coordinator in the application must share the same gate. A fresh gate
/// is not a way to replace an owner whose worker has not terminated. Generic
/// Default construction carries no admission; application hosts must use the guarded
/// constructor so leases keep its fixed allocation charged after host destruction.
#[derive(Clone, Default)]
pub struct DispatchGate(Arc<GateStorage>);

/// Unique worker lease, consumed only after the associated worker has terminated.
///
/// Dropping without acknowledgement requests cancellation but deliberately keeps
/// the gate Busy. This conservative state requires the owning worker's terminal
/// acknowledgement; normal errors and panics must reach a host terminal envelope.
pub struct RunLease {
    gate: DispatchGate,
    token: RunToken,
    acknowledged: bool,
}

impl DispatchGate {
    /// Admits an application's fixed dispatch allocation before allocating its Arc.
    ///
    /// # Arguments
    /// * `budget` - Canonical host counter, also retained by the gate reservation.
    ///
    /// # Returns
    /// One guarded slot charging its exact GateStorage layout and one shared root,
    /// with no ordered entries. Clones and worker leases share the last guard.
    /// The control pointee requires its own separate concrete owner admission.
    ///
    /// # Errors
    /// Rejects checked accounting overflow or byte/root admission before allocation,
    /// preserving existing accounting, controls and execution ownership.
    pub(super) fn for_application(budget: &RetentionBudget) -> Result<Self, RetentionError> {
        let reservation = budget.reserve_resources(RetainedResources {
            payload_bytes: size_of::<GateStorage>(),
            ordered_entries: 0,
            shared_roots: APPLICATION_GATE_ROOTS,
            ..Default::default()
        })?;
        Ok(Self(Arc::new(GateStorage {
            state: Mutex::new(None),
            _reservation: Some(reservation),
        })))
    }

    /// Reads global admission state without changing execution or selection.
    ///
    /// # Returns
    /// The current running/paused/cancelling owner, or None when idle.
    pub fn active(&self) -> Option<ActiveRun> {
        lock(&self.0.state).as_ref().map(|occupied| occupied.active)
    }

    /// Occupies the shared slot before the host dispatches its worker.
    ///
    /// # Arguments
    /// * `token` - Fresh checked direction/session/generation/run identity.
    /// * `cancellation` - Nonblocking adapter retained until actual termination.
    ///
    /// # Returns
    /// Sole terminal lease for the worker envelope.
    ///
    /// # Errors
    /// Returns Busy with the existing owner, including paused/cancelling work.
    pub(super) fn admit(
        &self,
        token: RunToken,
        cancellation: Arc<dyn RunCancellation>,
    ) -> Result<RunLease, DispatchError> {
        let mut slot = lock(&self.0.state);
        if let Some(occupied) = slot.as_ref() {
            return Err(DispatchError::Busy(occupied.active));
        }
        *slot = Some(Occupied {
            active: ActiveRun {
                token,
                phase: DispatchPhase::Running,
            },
            cancellation,
            cancellation_requested: false,
        });
        Ok(RunLease {
            gate: self.clone(),
            token,
            acknowledged: false,
        })
    }

    /// Records a pause/resume already accepted by the direction's public control.
    ///
    /// # Arguments
    /// * `token` - Exact active callback identity; never inferred from selected tab.
    /// * `paused` - Accepted backend pause state, not a request to run an algorithm.
    ///
    /// # Returns
    /// Nothing; the same execution slot remains occupied.
    ///
    /// # Errors
    /// Rejects stale tokens and pause/resume after cancellation was requested.
    pub fn acknowledge_pause(&self, token: RunToken, paused: bool) -> Result<(), DispatchError> {
        let mut slot = lock(&self.0.state);
        let occupied = slot.as_mut().ok_or(DispatchError::StaleCallback)?;
        if occupied.active.token != token {
            return Err(DispatchError::StaleCallback);
        }
        if occupied.cancellation_requested {
            return Err(DispatchError::Cancelling);
        }
        occupied.active.phase = if paused {
            DispatchPhase::Paused
        } else {
            DispatchPhase::Running
        };
        Ok(())
    }

    /// Requests cancellation once while retaining the active execution slot.
    ///
    /// # Arguments
    /// * `token` - Exact active worker identity.
    ///
    /// # Returns
    /// Nothing; repeated requests for the same run are idempotent.
    ///
    /// # Errors
    /// Rejects stale callbacks without touching another run's control.
    pub fn request_cancel(&self, token: RunToken) -> Result<(), DispatchError> {
        let control = {
            let mut slot = lock(&self.0.state);
            let occupied = slot.as_mut().ok_or(DispatchError::StaleCallback)?;
            if occupied.active.token != token {
                return Err(DispatchError::StaleCallback);
            }
            if occupied.cancellation_requested {
                return Ok(());
            }
            occupied.cancellation_requested = true;
            occupied.active.phase = DispatchPhase::Cancelling;
            Arc::clone(&occupied.cancellation)
        };
        // Call outside the gate lock so an adapter cannot re-enter a held lock.
        control.request_cancel();
        Ok(())
    }

    /// Requests cancellation only for a coordinator's own accepted sessions.
    ///
    /// # Arguments
    /// * `sessions` - Independent downstream and upstream process identities.
    ///
    /// Stale or already terminated work is ignored; the slot is never released.
    pub(super) fn cancel_owned(&self, sessions: [SessionId; 2]) {
        if let Some(active) = self.active() {
            if sessions.contains(&active.token.key.session) {
                let _ = self.request_cancel(active.token);
            }
        }
    }

    fn acknowledge_termination(&self, token: RunToken) -> Result<(), DispatchError> {
        let mut slot = lock(&self.0.state);
        if slot.as_ref().map(|occupied| occupied.active.token) != Some(token) {
            return Err(DispatchError::StaleCallback);
        }
        let retired = slot.take();
        drop(slot);
        // Control destruction also happens outside the gate lock.
        drop(retired);
        Ok(())
    }
}

impl RunLease {
    /// Copies the identity to attach to progress/result callbacks.
    ///
    /// # Returns
    /// The exact immutable direction/session/generation/run token.
    pub fn token(&self) -> RunToken {
        self.token
    }

    /// Releases admission after actual worker termination, including failure.
    ///
    /// # Returns
    /// Nothing; consumes the sole lease so safe callers cannot acknowledge twice.
    ///
    /// # Errors
    /// Rejects a stale terminal identity without releasing a newer run's slot.
    pub fn acknowledge_termination(mut self) -> Result<(), DispatchError> {
        self.gate.acknowledge_termination(self.token)?;
        self.acknowledged = true;
        Ok(())
    }
}

impl Drop for RunLease {
    /// Requests cancellation for an abandoned lease without freeing admission.
    ///
    /// A normal host terminal path must consume the lease explicitly. No join,
    /// worker wait or automatic replacement occurs here.
    fn drop(&mut self) {
        if !self.acknowledged {
            let _ = self.gate.request_cancel(self.token);
        }
    }
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[cfg(test)]
mod retention_tests;

#[cfg(test)]
mod tests {
    use super::super::{Direction, RunId, SessionKey};
    use super::*;

    #[test]
    fn duplicate_or_stale_terminal_does_not_release_newer_run() {
        struct Cancel;
        impl RunCancellation for Cancel {
            /// A no-op immediate control for identity-only gate tests.
            fn request_cancel(&self) {}
        }
        let gate = DispatchGate::default();
        let key = SessionKey {
            direction: Direction::Downstream,
            session: SessionId::allocate().unwrap(),
            generation: Default::default(),
        };
        let first = RunToken { key, run: RunId(1) };
        let second = RunToken { key, run: RunId(2) };
        gate.admit(first, Arc::new(Cancel))
            .unwrap()
            .acknowledge_termination()
            .unwrap();
        let lease = gate.admit(second, Arc::new(Cancel)).unwrap();
        assert_eq!(
            gate.acknowledge_termination(first),
            Err(DispatchError::StaleCallback)
        );
        assert_eq!(gate.active().unwrap().token, second);
        lease.acknowledge_termination().unwrap();
        assert_eq!(
            gate.acknowledge_termination(second),
            Err(DispatchError::StaleCallback)
        );
    }
}
