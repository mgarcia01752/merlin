/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Scalar projection of authoritative upstream lifecycle and recovery outcomes.
use super::Model;
use crate::shell::Direction;
use serde_json::{json, Value};

impl Model {
    /// Reads current accepted state without copying source or observation buffers.
    /// # Returns
    /// Scalars only; missing BER remains null and application identities are strings.
    pub(crate) fn state(&self) -> Value {
        let active = self.application.active();
        let progress = self
            .controller
            .as_ref()
            .and_then(|controller| controller.progress());
        let result = self.report.as_ref().map(|(token, retained)| {
            let report = retained.value();
            json!({"run":token.run.to_string(),"status":report.status,
                "bits_tested":report.compared_source_bits,"bit_errors":report.bit_errors,
                "ber":report.ber,"reference_recovered":report.reference_recovered,
                "completed_grants":report.grants.iter().filter(|grant| grant.completed).count(),
                "unverified_source_bits":report.unverified_source_bits})
        });
        json!({"configured":self.current(),"preparing":self.pending.is_some(),
            "busy":self.is_running()||self.pending.is_some(),"application_busy":active.is_some(),
            "active_direction":active.map(|run|match run.token.key.direction{Direction::Downstream=>"downstream",Direction::Upstream=>"upstream"}),
            "can_start":self.current()&&self.pending.is_none()&&!self.is_running()&&active.is_none(),
            "session":self.key.map(|key|key.session.to_string()),
            "generation":self.key.map(|key|key.generation.to_string()),
            "progress":progress.map(|(token,snapshot)|json!({"run":token.run.to_string(),"status":snapshot.status,
                "completed_grants":snapshot.completed_grants,"emitted_samples":snapshot.emitted_samples,
                "compared_bits":snapshot.compared_bits,"elapsed_ms":snapshot.elapsed_ms})),
            "has_result":self.report.is_some(),"result":result,"error":self.error.message()})
    }
}
