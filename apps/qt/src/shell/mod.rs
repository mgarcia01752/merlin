/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Explicit Qt application ownership, independent of either scientific engine.
//! Not registered in the application yet; the integration lead owns that wiring.
mod dispatch;
mod identity;
mod retention;
mod worker;

pub use dispatch::{
    ActiveRun, DispatchError, DispatchGate, DispatchPhase, RunCancellation, RunLease,
};
pub use identity::{
    ConfigurationGeneration, Direction, EditRevision, RunId, RunToken, SessionId, SessionKey,
};
pub use retention::{
    Retained, RetainedBytes, RetainedResources, RetentionBudget, RetentionError,
    RetentionReservation,
};
pub use worker::{
    spawn_worker, WorkerEvent, WorkerSender, WorkerSpawnError, WorkerStatus, WorkerTerminal,
};

use std::sync::Arc;

/// Immutable identity/dirty snapshot for Save completion or guarded Open adoption.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectSnapshot {
    pub key: SessionKey,
    pub revision: EditRevision,
    pub dirty: bool,
}

/// Explicit result of the host's target-direction dirty confirmation policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplacePermission {
    RequireClean,
    DiscardConfirmed,
}

/// Rejected ownership operation; previously accepted state is preserved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionError {
    IdentityExhausted,
    WrongBudget,
    Closing,
    StaleGuard,
    DirtyTarget,
    Dispatch(DispatchError),
}

/// Independent typed accepted configuration and actual retained results.
///
/// `C` belongs to the direction's existing accepted input/configuration owner;
/// `R` belongs to its public actual report/result owner. Neither is merged with
/// the other direction. Access is immutable; changed configuration is adopted
/// only through the coordinator's checked generation transition. Configuration
/// payloads must not expose interior scientific mutation through shared borrows;
/// mutable executing backend sessions belong to the worker, not this snapshot.
pub struct DirectionSession<C, R> {
    snapshot: ProjectSnapshot,
    last_run: RunId,
    configuration: Retained<C>,
    result: Option<(RunToken, Retained<R>)>,
}

/// Application coordinator over separate downstream and upstream payload types.
///
/// `DC`/`DR` are downstream configuration/result types and `UC`/`UR` upstream
/// types. Scientific validation occurs before adoption in their existing public
/// owners. The host must reserve before allocating payloads and dispatch work
/// outside the Qt thread. This module starts no worker and performs no I/O.
pub struct ShellSessions<DC, DR, UC, UR> {
    downstream: DirectionSession<DC, DR>,
    upstream: DirectionSession<UC, UR>,
    selected: Direction,
    budget: RetentionBudget,
    gate: DispatchGate,
    closing: bool,
}

impl<C, R> DirectionSession<C, R> {
    fn new(direction: Direction, configuration: Retained<C>) -> Result<Self, SessionError> {
        Ok(Self {
            snapshot: ProjectSnapshot {
                key: SessionKey {
                    direction,
                    session: SessionId::allocate().map_err(|_| SessionError::IdentityExhausted)?,
                    generation: ConfigurationGeneration::default(),
                },
                revision: EditRevision::default(),
                dirty: false,
            },
            last_run: RunId::default(),
            configuration,
            result: None,
        })
    }

    /// Borrows this direction's accepted scientific state without mutable access.
    ///
    /// # Returns
    /// The original typed payload for the lifetime of this session borrow.
    pub fn configuration(&self) -> &C {
        self.configuration.value()
    }

    /// Borrows the most recently adopted actual report and its original identity.
    ///
    /// # Returns
    /// None before any accepted report; otherwise its token and typed payload.
    pub fn result(&self) -> Option<(RunToken, &R)> {
        self.result
            .as_ref()
            .map(|(token, result)| (*token, result.value()))
    }

    fn reconfigure(&mut self, configuration: Retained<C>) -> Result<(), SessionError> {
        let revision = self.next_configuration_revision()?;
        self.accept_configuration(configuration, revision);
        Ok(())
    }

    fn next_configuration_revision(
        &self,
    ) -> Result<(ConfigurationGeneration, EditRevision), SessionError> {
        let generation = self
            .snapshot
            .key
            .generation
            .0
            .checked_add(1)
            .ok_or(SessionError::IdentityExhausted)?;
        let revision = self
            .snapshot
            .revision
            .0
            .checked_add(1)
            .ok_or(SessionError::IdentityExhausted)?;
        Ok((ConfigurationGeneration(generation), EditRevision(revision)))
    }

    fn accept_configuration(
        &mut self,
        configuration: Retained<C>,
        revision: (ConfigurationGeneration, EditRevision),
    ) {
        self.configuration = configuration;
        self.result = None;
        self.snapshot.key.generation = revision.0;
        self.snapshot.revision = revision.1;
        self.snapshot.dirty = true;
    }
}

impl<DC, DR, UC, UR> ShellSessions<DC, DR, UC, UR> {
    /// Creates two independent accepted sessions sharing application admission.
    ///
    /// # Arguments
    /// * `budget` - Shared accounting for all accepted/queued/old payloads.
    /// * `gate` - Application-wide gate, also retained by active worker leases.
    /// * `downstream` - Prevalidated, reserved downstream accepted configuration.
    /// * `upstream` - Prevalidated, reserved upstream accepted configuration.
    ///
    /// # Returns
    /// An idle coordinator initially selecting downstream; no worker is started.
    ///
    /// # Errors
    /// Rejects foreign budgets or exhausted process identity space.
    pub fn new(
        budget: RetentionBudget,
        gate: DispatchGate,
        downstream: Retained<DC>,
        upstream: Retained<UC>,
    ) -> Result<Self, SessionError> {
        if !budget.owns(&downstream) || !budget.owns(&upstream) {
            return Err(SessionError::WrongBudget);
        }
        Ok(Self {
            downstream: DirectionSession::new(Direction::Downstream, downstream)?,
            upstream: DirectionSession::new(Direction::Upstream, upstream)?,
            selected: Direction::Downstream,
            budget,
            gate,
            closing: false,
        })
    }

    /// Borrows the downstream owner without implicit selected-tab routing.
    ///
    /// # Returns
    /// The independent downstream configuration/result owner.
    pub fn downstream(&self) -> &DirectionSession<DC, DR> {
        &self.downstream
    }

    /// Borrows the upstream owner without implicit selected-tab routing.
    ///
    /// # Returns
    /// The independent upstream configuration/result owner.
    pub fn upstream(&self) -> &DirectionSession<UC, UR> {
        &self.upstream
    }

    /// Changes presentation selection without stopping or redirecting execution.
    ///
    /// # Arguments
    /// * `direction` - The tab whose own actions the host should present.
    pub fn select(&mut self, direction: Direction) {
        self.selected = direction;
    }

    /// Snapshots the selected direction for an asynchronous Save operation.
    ///
    /// # Returns
    /// Identity/dirty state; actual serialization and safe writes remain host-owned.
    pub fn save_target(&self) -> ProjectSnapshot {
        self.snapshot(self.selected)
    }

    /// Snapshots the detected Open target before the host's dirty confirmation.
    ///
    /// # Arguments
    /// * `direction` - Direction established by validated public project admission.
    ///
    /// # Returns
    /// A guard rechecked by replacement, independently of the selected tab.
    pub fn open_target(&self, direction: Direction) -> ProjectSnapshot {
        self.snapshot(direction)
    }

    /// Marks a completed Save clean only while its original revision is current.
    ///
    /// # Arguments
    /// * `saved` - Snapshot captured before the successfully completed write.
    ///
    /// # Errors
    /// Rejects closing or changed session/generation/revision; later edits stay dirty.
    pub fn acknowledge_save(&mut self, saved: ProjectSnapshot) -> Result<(), SessionError> {
        self.ensure_open()?;
        let current = self.snapshot(saved.key.direction);
        if current.key != saved.key || current.revision != saved.revision {
            return Err(SessionError::StaleGuard);
        }
        self.snapshot_mut(saved.key.direction).dirty = false;
        Ok(())
    }

    /// Records an accepted project metadata edit without mutating scientific data.
    ///
    /// # Arguments
    /// * `direction` - Explicit owner of the changed project metadata.
    ///
    /// # Errors
    /// Rejects closing or revision overflow without changing dirty state.
    pub fn mark_metadata_dirty(&mut self, direction: Direction) -> Result<(), SessionError> {
        self.ensure_open()?;
        let snapshot = self.snapshot_mut(direction);
        let next = snapshot
            .revision
            .0
            .checked_add(1)
            .ok_or(SessionError::IdentityExhausted)?;
        snapshot.revision = EditRevision(next);
        snapshot.dirty = true;
        Ok(())
    }

    /// Adopts a prevalidated downstream configuration and invalidates its old results.
    ///
    /// # Arguments
    /// * `configuration` - Reserved candidate from the existing public admission owner.
    ///
    /// # Errors
    /// Rejects closing, foreign accounting, own active run or identity overflow.
    /// Existing configuration/results and the upstream owner remain unchanged.
    pub fn configure_downstream(
        &mut self,
        configuration: Retained<DC>,
    ) -> Result<(), SessionError> {
        self.check_candidate(Direction::Downstream, &configuration)?;
        self.downstream.reconfigure(configuration)
    }

    /// Adopts a prevalidated upstream configuration and invalidates its old results.
    ///
    /// # Arguments
    /// * `configuration` - Reserved candidate from the existing public admission owner.
    ///
    /// # Errors
    /// Rejects closing, foreign accounting, own active run or identity overflow.
    /// Existing configuration/results and the downstream owner remain unchanged.
    pub fn configure_upstream(&mut self, configuration: Retained<UC>) -> Result<(), SessionError> {
        self.check_candidate(Direction::Upstream, &configuration)?;
        self.upstream.reconfigure(configuration)
    }

    /// Accepts a downstream edit while cancelling its old scientific generation.
    ///
    /// # Arguments
    /// * `configuration` - Completely validated, immutable, pre-reserved candidate.
    ///
    /// # Returns
    /// Unit after atomic adoption; old work keeps the global slot until termination.
    ///
    /// # Errors
    /// Rejects closing, foreign accounting or revision exhaustion before cancellation
    /// or mutation. Upstream work and the existing reject-while-Busy API are unchanged.
    pub fn configure_downstream_cancelling(
        &mut self,
        configuration: Retained<DC>,
    ) -> Result<(), SessionError> {
        self.check_accounting(&configuration)?;
        let revision = self.downstream.next_configuration_revision()?;
        self.cancel_direction(Direction::Downstream);
        self.downstream
            .accept_configuration(configuration, revision);
        Ok(())
    }

    /// Accepts an upstream edit while cancelling its old scientific generation.
    ///
    /// # Arguments
    /// * `configuration` - Completely validated, immutable, pre-reserved candidate.
    ///
    /// # Returns
    /// Unit after atomic adoption; old work keeps the global slot until termination.
    ///
    /// # Errors
    /// Rejects closing, foreign accounting or revision exhaustion before cancellation
    /// or mutation. Downstream work and the existing reject-while-Busy API are unchanged.
    pub fn configure_upstream_cancelling(
        &mut self,
        configuration: Retained<UC>,
    ) -> Result<(), SessionError> {
        self.check_accounting(&configuration)?;
        let revision = self.upstream.next_configuration_revision()?;
        self.cancel_direction(Direction::Upstream);
        self.upstream.accept_configuration(configuration, revision);
        Ok(())
    }

    /// Replaces downstream only after rechecking the detected Open target's guard.
    ///
    /// # Arguments
    /// * `guard` - Target snapshot captured before confirmation/async loading.
    /// * `permission` - Host's explicit decision about discarding dirty target state.
    /// * `configuration` - Completely admitted and reserved downstream project state.
    ///
    /// # Errors
    /// Rejects wrong/stale target, dirty denial, Busy, budget mismatch or exhaustion.
    /// Rejection preserves both tabs; success creates a clean new session identity.
    pub fn replace_downstream(
        &mut self,
        guard: ProjectSnapshot,
        permission: ReplacePermission,
        configuration: Retained<DC>,
    ) -> Result<(), SessionError> {
        self.check_replace(Direction::Downstream, guard, permission, &configuration)?;
        self.downstream = DirectionSession::new(Direction::Downstream, configuration)?;
        Ok(())
    }

    /// Replaces upstream only after rechecking the detected Open target's guard.
    ///
    /// # Arguments
    /// * `guard` - Target snapshot captured before confirmation/async loading.
    /// * `permission` - Host's explicit decision about discarding dirty target state.
    /// * `configuration` - Completely admitted and reserved upstream project state.
    ///
    /// # Errors
    /// Rejects wrong/stale target, dirty denial, Busy, budget mismatch or exhaustion.
    /// Rejection preserves both tabs; success creates a clean new session identity.
    pub fn replace_upstream(
        &mut self,
        guard: ProjectSnapshot,
        permission: ReplacePermission,
        configuration: Retained<UC>,
    ) -> Result<(), SessionError> {
        self.check_replace(Direction::Upstream, guard, permission, &configuration)?;
        self.upstream = DirectionSession::new(Direction::Upstream, configuration)?;
        Ok(())
    }

    /// Admits one direction's worker before the host starts any execution.
    ///
    /// # Arguments
    /// * `direction` - Explicit session to execute, independent of selection.
    /// * `cancellation` - Nonblocking adapter around its existing public run control.
    ///
    /// # Returns
    /// Sole lease to move into the worker/terminal envelope; attach its token to callbacks.
    ///
    /// # Errors
    /// Rejects closing, identity overflow or an existing run, including paused work.
    pub fn begin(
        &mut self,
        direction: Direction,
        cancellation: Arc<dyn RunCancellation>,
    ) -> Result<RunLease, SessionError> {
        self.ensure_open()?;
        let last = match direction {
            Direction::Downstream => self.downstream.last_run,
            Direction::Upstream => self.upstream.last_run,
        };
        let next = RunId(
            last.0
                .checked_add(1)
                .ok_or(SessionError::IdentityExhausted)?,
        );
        let lease = self
            .gate
            .admit(
                RunToken {
                    key: self.snapshot(direction).key,
                    run: next,
                },
                cancellation,
            )
            .map_err(SessionError::Dispatch)?;
        match direction {
            Direction::Downstream => self.downstream.last_run = next,
            Direction::Upstream => self.upstream.last_run = next,
        }
        Ok(lease)
    }

    /// Validates a progress callback immediately before the host applies its fields.
    ///
    /// # Arguments
    /// * `token` - Explicit callback identity, independent of current tab selection.
    ///
    /// # Errors
    /// Rejects closing or mismatched direction/session/generation/run ownership.
    /// This check does not retain permission across later event-loop transitions.
    pub fn validate_callback(&self, token: RunToken) -> Result<(), SessionError> {
        self.check_token(token.key.direction, token)
    }

    /// Adopts a completed downstream partial/report callback for its exact owner.
    ///
    /// # Arguments
    /// * `token` - Exact worker identity, including direction/generation/run.
    /// * `result` - Actual reserved report from the existing public result owner.
    ///
    /// # Errors
    /// Rejects closing, stale callbacks or foreign budget; old results stay accepted.
    pub fn publish_downstream(
        &mut self,
        token: RunToken,
        result: Retained<DR>,
    ) -> Result<(), SessionError> {
        self.check_result(Direction::Downstream, token, &result)?;
        self.downstream.result = Some((token, result));
        Ok(())
    }

    /// Adopts a completed upstream partial/report callback for its exact owner.
    ///
    /// # Arguments
    /// * `token` - Exact worker identity, including direction/generation/run.
    /// * `result` - Actual reserved report from the existing public result owner.
    ///
    /// # Errors
    /// Rejects closing, stale callbacks or foreign budget; old results stay accepted.
    pub fn publish_upstream(
        &mut self,
        token: RunToken,
        result: Retained<UR>,
    ) -> Result<(), SessionError> {
        self.check_result(Direction::Upstream, token, &result)?;
        self.upstream.result = Some((token, result));
        Ok(())
    }

    /// Handles a terminated downstream worker, releasing its lease on every path.
    ///
    /// # Arguments
    /// * `lease` - Sole lease received after actual worker termination or spawn failure.
    /// * `result` - Actual optional partial/final report; None preserves old results.
    ///
    /// # Errors
    /// Rejects closing/stale/foreign results while still acknowledging termination.
    pub fn finish_downstream(
        &mut self,
        lease: RunLease,
        result: Option<Retained<DR>>,
    ) -> Result<(), SessionError> {
        let accepted = match result {
            Some(result) => self.publish_downstream(lease.token(), result),
            None => self.check_token(Direction::Downstream, lease.token()),
        };
        let terminated = lease
            .acknowledge_termination()
            .map_err(SessionError::Dispatch);
        accepted.and(terminated)
    }

    /// Handles a terminated upstream worker, releasing its lease on every path.
    ///
    /// # Arguments
    /// * `lease` - Sole lease received after actual worker termination or spawn failure.
    /// * `result` - Actual optional partial/final report; None preserves old results.
    ///
    /// # Errors
    /// Rejects closing/stale/foreign results while still acknowledging termination.
    pub fn finish_upstream(
        &mut self,
        lease: RunLease,
        result: Option<Retained<UR>>,
    ) -> Result<(), SessionError> {
        let accepted = match result {
            Some(result) => self.publish_upstream(lease.token(), result),
            None => self.check_token(Direction::Upstream, lease.token()),
        };
        let terminated = lease
            .acknowledge_termination()
            .map_err(SessionError::Dispatch);
        accepted.and(terminated)
    }

    /// Closes admission and requests owned cancellation once without waiting.
    ///
    /// Active leases remain Busy until the worker's terminal acknowledgement.
    /// Existing accepted state stays available for host shutdown/recovery handling.
    pub fn shutdown(&mut self) {
        self.closing = true;
        self.gate.cancel_owned([
            self.downstream.snapshot.key.session,
            self.upstream.snapshot.key.session,
        ]);
    }

    fn ensure_open(&self) -> Result<(), SessionError> {
        if self.closing {
            return Err(SessionError::Closing);
        }
        Ok(())
    }

    fn snapshot(&self, direction: Direction) -> ProjectSnapshot {
        match direction {
            Direction::Downstream => self.downstream.snapshot,
            Direction::Upstream => self.upstream.snapshot,
        }
    }

    fn snapshot_mut(&mut self, direction: Direction) -> &mut ProjectSnapshot {
        match direction {
            Direction::Downstream => &mut self.downstream.snapshot,
            Direction::Upstream => &mut self.upstream.snapshot,
        }
    }

    fn check_candidate<T>(
        &self,
        direction: Direction,
        candidate: &Retained<T>,
    ) -> Result<(), SessionError> {
        self.check_accounting(candidate)?;
        if let Some(active) = self.gate.active() {
            if active.token.key.session == self.snapshot(direction).key.session {
                return Err(SessionError::Dispatch(DispatchError::Busy(active)));
            }
        }
        Ok(())
    }

    fn check_accounting<T>(&self, candidate: &Retained<T>) -> Result<(), SessionError> {
        self.ensure_open()?;
        if !self.budget.owns(candidate) {
            return Err(SessionError::WrongBudget);
        }
        Ok(())
    }

    fn cancel_direction(&self, direction: Direction) {
        if let Some(active) = self.gate.active() {
            if active.token.key.session == self.snapshot(direction).key.session {
                // Actual termination can race with cancellation. A stale token
                // already relinquished this slot; never touch a successor's control.
                let _ = self.gate.request_cancel(active.token);
            }
        }
    }

    fn check_replace<T>(
        &self,
        direction: Direction,
        guard: ProjectSnapshot,
        permission: ReplacePermission,
        candidate: &Retained<T>,
    ) -> Result<(), SessionError> {
        self.check_candidate(direction, candidate)?;
        if self.snapshot(direction) != guard {
            return Err(SessionError::StaleGuard);
        }
        if guard.dirty && permission != ReplacePermission::DiscardConfirmed {
            return Err(SessionError::DirtyTarget);
        }
        Ok(())
    }

    fn check_token(&self, direction: Direction, token: RunToken) -> Result<(), SessionError> {
        self.ensure_open()?;
        if self.snapshot(direction).key != token.key
            || self.gate.active().map(|active| active.token) != Some(token)
        {
            return Err(SessionError::Dispatch(DispatchError::StaleCallback));
        }
        Ok(())
    }

    fn check_result<T>(
        &self,
        direction: Direction,
        token: RunToken,
        result: &Retained<T>,
    ) -> Result<(), SessionError> {
        self.check_token(direction, token)?;
        if !self.budget.owns(result) {
            return Err(SessionError::WrongBudget);
        }
        Ok(())
    }
}

impl<DC, DR, UC, UR> Drop for ShellSessions<DC, DR, UC, UR> {
    /// Requests owned cancellation once without joining or freeing an active lease.
    ///
    /// Worker inputs and terminal acknowledgement must outlive this presentation owner.
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests;
