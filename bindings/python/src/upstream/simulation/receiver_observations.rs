/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin session policy access through the established native Busy guard.
use super::super::receiver_observations::PyReceiverObservationPolicy;
use super::*;

#[pymethods]
impl PySimulation {
    /// Atomically replaces optional received-estimate retention through the Rust API.
    /// # Arguments
    /// * `policy` - Validated native aggregate frame/scalar policy.
    /// # Returns
    /// None without changing accepted generation, source, configuration or existing reports.
    /// # Errors
    /// Busy while running; insufficient mode-specific storage preserves the prior policy.
    fn set_receiver_observation_policy(
        &self,
        policy: PyRef<'_, PyReceiverObservationPolicy>,
    ) -> PyResult<()> {
        self.access(|session| session.set_receiver_observation_policy(policy.0))
    }

    /// Reads optional receiver observation policy without executing scientific work.
    /// # Returns
    /// Owned immutable native policy, independent of subsequent session changes.
    /// # Errors
    /// Existing Busy category while another operation owns the session.
    fn receiver_observation_policy(&self) -> PyResult<PyReceiverObservationPolicy> {
        self.access(|session| {
            Ok(PyReceiverObservationPolicy(
                session.receiver_observation_policy(),
            ))
        })
    }
}
