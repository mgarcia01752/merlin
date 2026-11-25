/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Native project guards over the existing canonical direction sessions.
use super::{
    lock, metadata, ApplicationError, ApplicationSessions, Direction, DownstreamReadiness,
    ProjectSnapshot, SessionKey, UpstreamReadiness,
};
use crate::shell::ReplacePermission;

impl ApplicationSessions {
    /// Restores both native identities and original-source protection atomically.
    /// # Arguments
    /// * `expected` - Exact downstream/upstream guards captured before preparation.
    /// * `ready`, `dirty` - Independent prepared readiness and saved edit flags.
    /// * `selected` - Saved direction, applied after both owner checks pass.
    /// * `paths` - Fully admitted same-host original-path registry candidate.
    /// * `expected_paths` - Original live registry snapshot, checked under its lock.
    /// # Returns
    /// Genuine new keys; caller attaches already prepared scientific owners without
    /// allocation or failure after this commit. No Save acknowledgement is implied.
    /// # Errors
    /// Foreign registry, admission, active work, stale guard, closing or exhausted
    /// identity space preserves both projects, selection and all protected paths.
    pub(crate) fn restore_projects(
        &self,
        expected: (ProjectSnapshot, ProjectSnapshot),
        ready: (bool, bool),
        dirty: (bool, bool),
        selected: Direction,
        paths: super::InputPaths,
        expected_paths: &[crate::shared::update_recovery::OriginalPath],
    ) -> Result<(SessionKey, SessionKey), ApplicationError> {
        if !paths.belongs_to(&self.budget) {
            return Err(ApplicationError::Session(
                crate::shell::SessionError::WrongBudget,
            ));
        }
        let downstream = metadata(&self.budget, DownstreamReadiness(ready.0))?;
        let upstream = metadata(&self.budget, UpstreamReadiness(ready.1))?;
        let mut sessions = lock(self.sessions.value());
        self.input_paths.replace_with(paths, expected_paths, || {
            sessions
                .restore_projects(expected, downstream, upstream, dirty, selected)
                .map_err(ApplicationError::Session)
        })
    }

    /// Closes only after both exact project guards and dirty decisions pass atomically.
    /// # Arguments
    /// * `downstream`, `upstream` - Native snapshots captured before close decisions.
    /// * `downstream_permission`, `upstream_permission` - Explicit per-owner outcomes.
    /// # Returns
    /// Unit after irreversible admission shutdown and owned cancellation requests.
    /// # Errors
    /// Stale/foreign guards or an unresolved dirty project preserve the live host;
    /// no worker is cancelled while either direction still needs a decision.
    pub fn close_projects(
        &self,
        downstream: ProjectSnapshot,
        upstream: ProjectSnapshot,
        downstream_permission: ReplacePermission,
        upstream_permission: ReplacePermission,
    ) -> Result<(), ApplicationError> {
        let mut sessions = lock(self.sessions.value());
        for (direction, guard, permission) in [
            (Direction::Downstream, downstream, downstream_permission),
            (Direction::Upstream, upstream, upstream_permission),
        ] {
            if guard.key.direction != direction || sessions.open_target(direction) != guard {
                return Err(ApplicationError::Session(
                    crate::shell::SessionError::StaleGuard,
                ));
            }
            if guard.dirty && permission == ReplacePermission::RequireClean {
                return Err(ApplicationError::Session(
                    crate::shell::SessionError::DirtyTarget,
                ));
            }
        }
        sessions.shutdown();
        Ok(())
    }
    /// Acknowledges a completed Save only while its captured revision is current.
    ///
    /// # Arguments
    /// * `saved` - Exact native direction, session, generation and revision captured
    ///   before the successful write; current tab selection has no effect.
    ///
    /// # Returns
    /// Unit after marking only the matching direction clean. No destination,
    /// scientific owner, run lease or configuration is changed.
    ///
    /// # Errors
    /// Rejects shutdown or stale identity/revision without cleaning newer edits.
    pub fn acknowledge_save(&self, saved: ProjectSnapshot) -> Result<(), ApplicationError> {
        lock(self.sessions.value())
            .acknowledge_save(saved)
            .map_err(ApplicationError::Session)
    }

    /// Replaces one project readiness owner after its exact dirty guard passes.
    ///
    /// # Arguments
    /// * `expected` - Full target snapshot captured before confirmation/preparation.
    /// * `permission` - Explicit existing clean-or-discard decision for that snapshot.
    /// * `ready` - Whether the caller's fully prepared project can run; false also
    ///   represents a valid blank or incomplete native editor project.
    ///
    /// # Returns
    /// New clean session key for the captured direction, independent of selected
    /// tab. Scientific inputs and destinations remain the caller's responsibility.
    /// The other direction and any foreign-direction run lease are unchanged.
    ///
    /// # Errors
    /// Metadata admission, shutdown, stale snapshot, unconfirmed dirty target,
    /// target-owned active execution or identity exhaustion before replacement.
    pub fn replace_project(
        &self,
        expected: ProjectSnapshot,
        permission: ReplacePermission,
        ready: bool,
    ) -> Result<SessionKey, ApplicationError> {
        match expected.key.direction {
            Direction::Downstream => {
                let candidate = metadata(&self.budget, DownstreamReadiness(ready))?;
                let mut sessions = lock(self.sessions.value());
                sessions
                    .replace_downstream(expected, permission, candidate)
                    .map_err(ApplicationError::Session)?;
                Ok(sessions.open_target(Direction::Downstream).key)
            }
            Direction::Upstream => {
                let candidate = metadata(&self.budget, UpstreamReadiness(ready))?;
                let mut sessions = lock(self.sessions.value());
                sessions
                    .replace_upstream(expected, permission, candidate)
                    .map_err(ApplicationError::Session)?;
                Ok(sessions.open_target(Direction::Upstream).key)
            }
        }
    }

    /// Records an accepted metadata edit in its explicit direction project.
    ///
    /// # Arguments
    /// * `direction` - Actual metadata owner, independent of current tab selection.
    ///
    /// # Returns
    /// Unit after advancing the existing edit revision and marking that project
    /// dirty; scientific generation, configuration and execution are unchanged.
    ///
    /// # Errors
    /// Shutdown or revision exhaustion preserves both direction snapshots.
    pub fn mark_project_dirty(&self, direction: Direction) -> Result<(), ApplicationError> {
        lock(self.sessions.value())
            .mark_metadata_dirty(direction)
            .map_err(ApplicationError::Session)
    }

    /// Closes admission after the caller has resolved both dirty-project decisions.
    ///
    /// # Returns
    /// Unit after requesting cancellation of owned execution without blocking.
    /// Existing accepted data remains available, and an active lease stays held
    /// until actual termination is acknowledged. Repeated calls do not reopen it.
    pub fn shutdown(&self) {
        lock(self.sessions.value()).shutdown();
    }
}

#[cfg(test)]
#[path = "project_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "recovery_tests.rs"]
mod recovery_tests;
