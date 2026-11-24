/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Scalar shell projection; scientific inputs and outcomes stay direction-owned.
use super::{ApplicationSessions, Direction, DispatchPhase, SessionKey};
use serde_json::{json, Value};

/// Supplies the compiled direction tag used by native and QML shell consumers.
///
/// # Arguments
/// * `direction` - Actual application owner, independent of presentation selection.
/// # Returns
/// A static tag; no scientific state is inspected or changed.
pub(crate) fn direction_tag(direction: Direction) -> &'static str {
    match direction {
        Direction::Downstream => "downstream",
        Direction::Upstream => "upstream",
    }
}

/// Formats one canonical key without passing integer identities through JavaScript.
///
/// # Arguments
/// * `key` - Current direction/session/configuration key issued by the native host.
/// # Returns
/// Bounded opaque text. Consumers retain it unchanged and never parse its fields.
pub(crate) fn identity(key: SessionKey) -> String {
    format!(
        "{}/{}/{}",
        direction_tag(key.direction),
        key.session,
        key.generation
    )
}

/// Projects existing canonical metadata without producing scientific observations.
///
/// # Arguments
/// * `application` - Host shared by both actual native direction adapters.
/// # Returns
/// Fixed-field scalar JSON with independent identities/dirty flags and active phase.
/// Serialization is transient; QML must admit its persistent copy before publication.
pub(crate) fn state(application: &ApplicationSessions) -> Value {
    let downstream = application.snapshot(Direction::Downstream);
    let upstream = application.snapshot(Direction::Upstream);
    let active = application.active();
    let status = active.map(|run| {
        let owner = match run.token.key.direction {
            Direction::Downstream => "Downstream",
            Direction::Upstream => "Upstream",
        };
        let phase = match run.phase {
            DispatchPhase::Running => "running",
            DispatchPhase::Paused => "paused",
            DispatchPhase::Cancelling => "cancelling",
        };
        format!("{owner} simulation {phase}")
    });
    json!({
        "selectedDirection": direction_tag(application.selected_direction()),
        "downstreamIdentity": identity(downstream.key),
        "upstreamIdentity": identity(upstream.key),
        "downstreamDirty": downstream.dirty,
        "upstreamDirty": upstream.dirty,
        "executionDirection": active.map(|run| direction_tag(run.token.key.direction)),
        "executionStatus": status.unwrap_or_default(),
        "applicationBusy": active.is_some(),
    })
}

#[cfg(test)]
mod tests;
