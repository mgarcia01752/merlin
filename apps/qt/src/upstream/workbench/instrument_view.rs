/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Exact native observation context and finite worker routing, without science.
#[cfg(test)]
mod tests;
use super::{instruments, Model, ObservationStatus, Pending};
use crate::{
    shared::presentation::constants::MAX_VIEW_REQUEST_JSON_BYTES,
    shell::{presentation::identity, Direction, Retained, RunToken, SessionKey},
};
use serde::Deserialize;
use serde_json::{json, Value};

/// Bound a compiled destination before copying Qt UTF-16 text.
pub(crate) const MAX_INSTRUMENT_KIND_CHARACTERS: isize = 16;

pub(super) struct Observation {
    pub(super) key: SessionKey,
    pub(super) run: Option<RunToken>,
    pub(super) display: Retained<instruments::Display>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Context {
    query_sequence: String,
    session_token: String,
    configuration_generation: String,
    run: Option<String>,
}

impl Model {
    /// Opens one existing public observation with an actual default selection.
    /// # Arguments
    /// * `kind` - Compiled instrument destination, unrelated to scientific modes.
    /// # Returns
    /// Unit after finite background dispatch; Poll establishes accepted data.
    /// # Errors
    /// Missing/busy/stale owners, unknown destination or admission failure.
    pub(crate) fn begin_instrument(&mut self, kind: &str) -> Result<(), String> {
        let result = (|| {
            self.ensure_configuration_idle()?;
            let controller = self
                .controller
                .as_ref()
                .ok_or("Configure upstream first.")?;
            let request = instruments::Request::initial(kind, controller)?;
            self.queue_instrument(request)
        })();
        self.instrument_request_result(result)
    }

    /// Forwards a strict original selector with its exact admitted view context.
    /// # Arguments
    /// * `kind` - Compiled destination, never an input file or scientific object.
    /// * `selection`, `context` - Finite JSON; context retains lossless identities.
    /// # Returns
    /// Unit after background dispatch; old data survives any rejected replacement.
    /// # Errors
    /// Transport, selector, stale/foreign context, Busy or admission failures.
    pub(crate) fn request_instrument(
        &mut self,
        kind: &str,
        selection: &str,
        context: &str,
    ) -> Result<(), String> {
        let result = (|| {
            if selection.len() > MAX_VIEW_REQUEST_JSON_BYTES
                || context.len() > MAX_VIEW_REQUEST_JSON_BYTES
            {
                return Err("Instrument request exceeds its transport byte limit.".into());
            }
            let request = instruments::Request::parse(kind, selection)?;
            let context: Context =
                serde_json::from_str(context).map_err(|error| error.to_string())?;
            let current = self
                .current_observation()
                .ok_or("Open a current upstream instrument first.")?;
            if context.session_token != identity(current.key)
                || context.query_sequence != self.instrument_sequence.to_string()
                || context.run != current.run.map(|token| token.run.to_string())
                || context.configuration_generation != self.instrument_generation(current)
            {
                return Err("Upstream instrument context changed; reopen the current view.".into());
            }
            self.queue_instrument(request)
        })();
        self.instrument_request_result(result)
    }

    /// Records bounded transport feedback to settle a pending presentation action.
    /// # Arguments
    /// * `message` - Existing application diagnostic without scientific payloads.
    /// # Returns
    /// Unit; accepted data and controller ownership remain unchanged.
    pub(crate) fn reject_instrument_request(&mut self, message: &str) {
        self.instrument_status = ObservationStatus::Rejected;
        self.error = self.admit_error(message.to_owned());
    }

    /// Publishes one bounded instrument independently from progress/configuration.
    /// # Returns
    /// Exact application/run context plus the admitted public-data projection.
    /// Missing, stale, active and rejected observations remain explicit.
    pub(super) fn instrument_state(&self) -> Value {
        let key = self.application.snapshot(Direction::Upstream).key;
        let observation = self.current_observation();
        let (status, reason) = match self.instrument_status {
            ObservationStatus::Rejected => ("error", self.error.message()),
            ObservationStatus::Cancelled => (
                "unavailable",
                "Instrument query cancelled; previous observation retained.",
            ),
            ObservationStatus::Loading => ("loading", "Preparing actual upstream observations."),
            _ if self.pending.is_some()
                || self.is_running()
                || self.application.active().is_some() =>
            {
                (
                    "unavailable",
                    "Finish active work before requesting observations.",
                )
            }
            _ if observation.is_some() => ("ready", ""),
            _ => (
                "unavailable",
                "Open an upstream instrument for the current configuration/report.",
            ),
        };
        json!({"status":status,"reason":reason,"query_sequence":self.instrument_sequence.to_string(),
            "session_token":identity(key),"run":observation.and_then(|value|value.run.map(|token|token.run.to_string())),
            "configuration_generation":observation.map(|value|self.instrument_generation(value)).unwrap_or_default(),
            "snapshot":observation.map(|value|&value.display.value().snapshot)})
    }

    fn current_observation(&self) -> Option<&Observation> {
        let key = self.application.snapshot(Direction::Upstream).key;
        self.instrument.as_ref().filter(|value| {
            self.current()
                && value.key == key
                && value.run == self.report.as_ref().map(|(token, _)| *token)
        })
    }

    fn instrument_generation(&self, value: &Observation) -> String {
        value.display.value().snapshot["metadata"]["configuration_generation"]
            .as_str()
            .map(str::to_owned)
            .or_else(|| {
                self.report
                    .as_ref()
                    .and_then(|(_, report)| report.value().grants.first())
                    .map(|grant| grant.identity.generation.to_string())
            })
            .unwrap_or_default()
    }

    fn queue_instrument(&mut self, request: instruments::Request) -> Result<(), String> {
        self.ensure_configuration_idle()?;
        if !self.current() {
            return Err("Apply a current upstream configuration first.".into());
        }
        let sequence = self
            .instrument_sequence
            .checked_add(1)
            .ok_or("Instrument query sequence exhausted.")?;
        let key = self.key.ok_or("Upstream configuration changed.")?;
        let run = self.report.as_ref().map(|(token, _)| *token);
        let controller = self
            .controller
            .take()
            .ok_or("Upstream configuration is unavailable.")?;
        match Pending::instrument(
            controller,
            self.application.budget().clone(),
            key,
            run,
            request,
        ) {
            Ok(pending) => self.pending = Some(pending),
            Err((controller, error)) => {
                self.controller = Some(controller);
                return Err(error);
            }
        }
        self.instrument_sequence = sequence;
        self.instrument_status = ObservationStatus::Loading;
        self.preparation_cancelled = false;
        self.error = Default::default();
        Ok(())
    }

    fn instrument_request_result(&mut self, result: Result<(), String>) -> Result<(), String> {
        if let Err(error) = &result {
            self.reject_instrument_request(error);
        }
        result
    }
}
