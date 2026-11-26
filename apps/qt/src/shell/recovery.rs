/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Guarded replacement of both native project identities at restart.
use super::*;

impl<DC, DR, UC, UR> ShellSessions<DC, DR, UC, UR> {
    /// Restores both prepared projects after checking their exact original owners.
    /// # Arguments
    /// * `expected` - Downstream and upstream snapshots captured before preparation.
    /// * `downstream`, `upstream` - Complete same-budget readiness candidates.
    /// * `dirty` - Independent saved dirty flags; recovery is not a normal Save.
    /// * `selected` - Saved presentation selection, applied only after both checks.
    /// # Returns
    /// New downstream/upstream keys after one infallible pair publication.
    /// # Errors
    /// Closing, active work, either stale/wrong snapshot, foreign budget or identity
    /// exhaustion preserves both accepted configurations, results and selection.
    pub(crate) fn restore_projects(
        &mut self,
        expected: (ProjectSnapshot, ProjectSnapshot),
        downstream: Retained<DC>,
        upstream: Retained<UC>,
        dirty: (bool, bool),
        selected: Direction,
    ) -> Result<(SessionKey, SessionKey), SessionError> {
        self.ensure_open()?;
        if let Some(active) = self.gate.active() {
            return Err(SessionError::Dispatch(DispatchError::Busy(active)));
        }
        for (direction, guard) in [
            (Direction::Downstream, expected.0),
            (Direction::Upstream, expected.1),
        ] {
            if guard.key.direction != direction || self.snapshot(direction) != guard {
                return Err(SessionError::StaleGuard);
            }
        }
        self.check_accounting(&downstream)?;
        self.check_accounting(&upstream)?;
        // Allocate BOTH identities before publication. A failure on the second
        // may consume an opaque process ID but cannot replace either project.
        let mut downstream = DirectionSession::new(Direction::Downstream, downstream)?;
        let mut upstream = DirectionSession::new(Direction::Upstream, upstream)?;
        downstream.snapshot.dirty = dirty.0;
        upstream.snapshot.dirty = dirty.1;
        let keys = (downstream.snapshot.key, upstream.snapshot.key);
        self.downstream = downstream;
        self.upstream = upstream;
        self.selected = selected;
        Ok(keys)
    }
}
