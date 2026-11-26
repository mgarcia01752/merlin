/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Configured-grid query routing and scalar presentation; the public API owns cells.
#[cfg(test)]
mod tests;
use super::{Model, ObservationStatus, Pending};
use crate::shell::{presentation::identity, Direction};
use merlin::upstream::{ResourceGridRequest, MAX_RESOURCE_GRID_CELLS};
use serde::Deserialize;
use serde_json::{json, Value};

pub(crate) use crate::shared::presentation::constants::MAX_VIEW_REQUEST_JSON_BYTES as MAX_GRID_REQUEST_BYTES;
/// Qt viewport population fits simultaneous old/new host and instrument copies.
/// This presentation policy is smaller than the public scientific query ceiling;
/// native 25 kHz full-span admission rejects the larger 16384-cell viewport.
const MAX_GRID_VIEW_CELLS: usize = 4096;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    frame_index: usize,
    first_symbol: usize,
    symbol_count: usize,
    first_carrier: usize,
    carrier_count: usize,
    max_cells: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Context {
    session_token: String,
    configuration_generation: String,
    target_session_token: Option<String>,
    target_grant_id: Option<String>,
}

impl Model {
    /// Requests the public default rectangle of the current configured base grant.
    /// # Returns
    /// Unit after dispatch; only Poll establishes an accepted owned observation.
    /// # Errors
    /// Missing, stale, busy or retention rejection preserves configuration/results.
    pub(crate) fn begin_resource_grid(&mut self) -> Result<(), String> {
        let result = self.queue_grid(ResourceGridRequest {
            max_cells: MAX_GRID_VIEW_CELLS,
            ..Default::default()
        });
        self.grid_request_result(result)
    }

    /// Forwards a bounded original-coordinate request with exact view context.
    /// # Arguments
    /// * `request`, `context` - Thin JSON transport; no resource roles are accepted.
    /// # Returns
    /// Unit after native context validation and finite worker dispatch.
    /// # Errors
    /// Invalid fields, stale/foreign context, work bounds or admission failure.
    /// Rejected requests cannot replace the original session or previous grid.
    pub(crate) fn request_resource_grid(
        &mut self,
        request: &str,
        context: &str,
    ) -> Result<(), String> {
        let result = (|| {
            if request.len() > MAX_GRID_REQUEST_BYTES || context.len() > MAX_GRID_REQUEST_BYTES {
                return Err("Resource grid request exceeds its transport byte limit.".into());
            }
            let request: Request =
                serde_json::from_str(request).map_err(|error| error.to_string())?;
            let context: Context =
                serde_json::from_str(context).map_err(|error| error.to_string())?;
            self.validate_grid_context(&context)?;
            self.queue_grid(ResourceGridRequest {
                frame_index: request.frame_index,
                first_symbol: request.first_symbol,
                symbol_count: request.symbol_count,
                first_carrier: request.first_carrier,
                carrier_count: request.carrier_count,
                max_cells: request.max_cells,
            })
        })();
        self.grid_request_result(result)
    }

    /// Records a thin-transport rejection so pending viewport actions can settle.
    /// # Arguments
    /// * `message` - Existing bounded diagnostic, without scientific payload data.
    /// # Returns
    /// Unit after retaining the diagnostic under the existing error owner.
    pub(crate) fn reject_grid_request(&mut self, message: &str) {
        self.grid_status = ObservationStatus::Rejected;
        self.error = self.admit_error(message.to_owned());
    }

    /// Projects current configured cells separately from actual waveform results.
    /// # Returns
    /// One bounded owned envelope. Canonical and backend generations stay distinct.
    pub(super) fn grid_state(&self) -> Value {
        let key = self.application.snapshot(Direction::Upstream).key;
        let token = identity(key);
        let current = self
            .grid
            .as_ref()
            .filter(|(owner, _)| *owner == key && self.current());
        let snapshot = current.map(|(_, display)| &display.value().snapshot);
        let generation = snapshot
            .and_then(|value| value["configuration_generation"].as_str())
            .unwrap_or("");
        let (status, reason) = match self.grid_status {
            ObservationStatus::Rejected => ("error", self.error.message()),
            ObservationStatus::Cancelled => (
                "unavailable",
                "Resource grid request cancelled; previous view retained.",
            ),
            ObservationStatus::Loading => ("loading", "Preparing configured resource cells."),
            _ if self.pending.is_some()
                || self.is_running()
                || self.application.active().is_some() =>
            {
                (
                    "unavailable",
                    "Finish active work before querying configured resources.",
                )
            }
            _ if snapshot.is_some() => (
                "ready",
                "Configured transmit allocation; not an observed waveform.",
            ),
            _ => (
                "unavailable",
                "Configure upstream and open its resource grid.",
            ),
        };
        json!({"status":status,"reason":reason,"query_sequence":self.grid_sequence.to_string(),
            "session_token":token,"snapshot_session_token":current.map(|(owner,_)|identity(*owner)),
            "configuration_generation":generation,"max_cells":MAX_GRID_VIEW_CELLS.min(MAX_RESOURCE_GRID_CELLS),
            "snapshot":snapshot})
    }

    fn validate_grid_context(&self, context: &Context) -> Result<(), String> {
        let key = self.application.snapshot(Direction::Upstream).key;
        let (owner, display) = self
            .grid
            .as_ref()
            .ok_or("Open a configured resource grid first.")?;
        if !self.current()
            || *owner != key
            || context.session_token != identity(key)
            || display.value().snapshot["configuration_generation"].as_str()
                != Some(context.configuration_generation.as_str())
        {
            return Err(
                "Resource grid context changed; reopen the current configured view.".into(),
            );
        }
        if context
            .target_session_token
            .as_ref()
            .is_some_and(|target| target != &context.session_token)
            || context.target_grant_id.as_ref().is_some_and(|target| {
                display.value().snapshot["grid"]["grant_id"].as_str() != Some(target.as_str())
            })
        {
            return Err("Only this session's configured base grant is available.".into());
        }
        Ok(())
    }

    fn queue_grid(&mut self, request: ResourceGridRequest) -> Result<(), String> {
        self.ensure_configuration_idle()?;
        if request.max_cells > MAX_GRID_VIEW_CELLS {
            return Err("Resource grid request exceeds the Qt viewport cell limit.".into());
        }
        if !self.current() {
            return Err("Apply an upstream configuration before inspecting resources.".into());
        }
        let next = self
            .grid_sequence
            .checked_add(1)
            .ok_or("Resource grid presentation sequence exhausted.")?;
        let key = self.key.ok_or("Upstream configuration changed.")?;
        let controller = self
            .controller
            .take()
            .ok_or("Upstream configuration is unavailable.")?;
        match Pending::grid(controller, self.application.budget().clone(), key, request) {
            Ok(pending) => self.pending = Some(pending),
            Err((controller, error)) => {
                self.controller = Some(controller);
                return Err(error);
            }
        }
        self.grid_sequence = next;
        self.grid_status = ObservationStatus::Loading;
        self.preparation_cancelled = false;
        self.error = Default::default();
        Ok(())
    }

    fn grid_request_result(&mut self, result: Result<(), String>) -> Result<(), String> {
        if let Err(error) = &result {
            self.reject_grid_request(error);
        }
        result
    }
}
