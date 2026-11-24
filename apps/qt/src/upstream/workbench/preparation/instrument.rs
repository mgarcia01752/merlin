/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Instruments reuse the existing passive controller-slot completion transport.
use super::*;
use crate::shell::RunToken;

impl Pending {
    /// Queries one typed public instrument away from Qt with exact owner context.
    /// # Arguments
    /// * `controller`, `budget` - Accepted original owner and canonical admission.
    /// * `expected`, `run` - Captured application identity, not a guessed backend key.
    /// * `request` - Finite already-parsed selector; no scientific data is supplied.
    /// # Returns
    /// One passive pending completion, without acquiring a simulation run lease.
    /// # Errors
    /// Admission/spawn failure returns the exact unmodified original controller.
    pub(in crate::upstream::workbench) fn instrument(
        controller: UpstreamController,
        budget: RetentionBudget,
        expected: SessionKey,
        run: Option<RunToken>,
        request: super::super::instruments::Request,
    ) -> Result<Self, (UpstreamController, String)> {
        let (owner, receiver) = super::query::controller_query(
            controller,
            budget,
            "merlin-upstream-instrument",
            move |controller, budget| request.observe(controller, budget),
        )?;
        Ok(Self::Instrument {
            expected,
            run,
            receiver,
            owner,
        })
    }
}
