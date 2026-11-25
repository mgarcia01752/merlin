/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Close decisions retain both exact native guards without shutting down early.
use super::*;
use crate::shell::{ProjectSnapshot, ReplacePermission, Retained, RetainedResources};
use serde_json::{json, Value};

pub(super) struct Intent {
    downstream: ProjectSnapshot,
    upstream: ProjectSnapshot,
    downstream_permission: ReplacePermission,
    upstream_permission: ReplacePermission,
    downstream_resolved: bool,
    upstream_resolved: bool,
}
impl Intent {
    fn pending(&self) -> Option<Direction> {
        if !self.downstream_resolved {
            Some(Direction::Downstream)
        } else if !self.upstream_resolved {
            Some(Direction::Upstream)
        } else {
            None
        }
    }
    fn metadata(&self) -> Value {
        json!({"ready":self.pending().is_none(),"direction":self.pending().map(presentation::direction_tag)})
    }
}
impl DirectionShellRust {
    /// Captures both exact project revisions without cancelling live work.
    /// # Returns
    /// The first dirty direction or ready metadata when both projects are clean.
    /// # Errors
    /// Duplicate close or retention failure keeps execution admission open.
    pub(super) fn begin_close(&mut self) -> Result<Value, String> {
        if self.close.is_some() {
            return Err("A project close decision is already pending.".into());
        }
        let downstream = self.application.snapshot(Direction::Downstream);
        let upstream = self.application.snapshot(Direction::Upstream);
        let reservation = self
            .application
            .budget()
            .reserve_resources(RetainedResources {
                payload_bytes: std::mem::size_of::<Retained<Intent>>(),
                ..Default::default()
            })
            .map_err(|_| "The project close decision exceeds retained-data limits.".to_owned())?;
        let intent = Intent {
            downstream,
            upstream,
            downstream_permission: ReplacePermission::RequireClean,
            upstream_permission: ReplacePermission::RequireClean,
            downstream_resolved: !downstream.dirty,
            upstream_resolved: !upstream.dirty,
        };
        let result = intent.metadata();
        self.close = Some(Box::new(reservation.retain(intent)));
        Ok(result)
    }

    /// Resolves one native direction before advancing to the other dirty owner.
    /// # Arguments
    /// * `discard`, `cancel` - Explicit prompt outcomes; Save must already be clean.
    /// # Returns
    /// Next direction, ready or cancelled metadata without shutting down workers.
    /// # Errors
    /// Missing intent, newer edits or an unsaved project aborts this close attempt.
    pub(super) fn close_decision(&mut self, discard: bool, cancel: bool) -> Result<Value, String> {
        let intent = self
            .close
            .take()
            .ok_or("No project close decision is pending.")?;
        if cancel {
            return Ok(json!({"cancelled":true}));
        }
        let (mut intent, guard) = (*intent).into_parts();
        let direction = intent
            .pending()
            .ok_or("Both project decisions are already resolved.")?;
        let current = self.application.snapshot(direction);
        let (expected, permission, resolved) = match direction {
            Direction::Downstream => (
                &mut intent.downstream,
                &mut intent.downstream_permission,
                &mut intent.downstream_resolved,
            ),
            Direction::Upstream => (
                &mut intent.upstream,
                &mut intent.upstream_permission,
                &mut intent.upstream_resolved,
            ),
        };
        if current.key != expected.key || current.revision != expected.revision {
            return Err("A project changed during the close decision. Retry closing.".into());
        }
        if !discard && current.dirty {
            return Err("Save or discard this project's changes before closing.".into());
        }
        *expected = current;
        *permission = if discard {
            ReplacePermission::DiscardConfirmed
        } else {
            ReplacePermission::RequireClean
        };
        *resolved = true;
        let result = intent.metadata();
        self.close = Some(Box::new(guard.retain(intent)));
        Ok(result)
    }

    /// Rechecks both resolved native snapshots under one lock before shutdown.
    /// # Returns
    /// Ready after irreversible admission shutdown and owned cancel requests.
    /// # Errors
    /// Missing/unresolved intent or stale/dirty guard leaves the application live.
    pub(super) fn finish_close(&mut self) -> Result<Value, String> {
        let intent = self
            .close
            .take()
            .ok_or("No project close decision is pending.")?;
        let intent = intent.value();
        if intent.pending().is_some() {
            return Err("Resolve both project decisions before closing.".into());
        }
        self.application
            .close_projects(
                intent.downstream,
                intent.upstream,
                intent.downstream_permission,
                intent.upstream_permission,
            )
            .map_err(|error| format!("{error:?}"))?;
        Ok(json!({"ready":true}))
    }
}

#[cfg(test)]
#[path = "close_tests.rs"]
mod tests;
