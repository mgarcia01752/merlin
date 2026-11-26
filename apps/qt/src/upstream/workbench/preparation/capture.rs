/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Capture decoding shares the existing finite preparation transport.
use super::{
    admit_task, bounded_channel, error_text, run_task, Pending, RetentionBudget, SessionKey,
    WorkerFailure, PREPARATION_COMPLETION_CAPACITY, WORKER_FAILURE_ADMISSION_MESSAGE,
};
use crate::upstream::workbench::captures::Request;

impl Pending {
    /// Dispatches one original capture or page request away from the GUI thread.
    ///
    /// # Arguments
    /// * `request` - Bounded original delivery or an existing admitted source alias.
    /// * `budget` - Same canonical counter as the prior display and live sessions.
    /// * `expected` - Current upstream identity, checked again before adoption.
    ///
    /// # Returns
    /// One pending completion; inspection never acquires a simulation run lease.
    ///
    /// # Errors
    /// Admission or thread creation failure leaves accepted state unchanged.
    pub(in crate::upstream::workbench) fn capture(
        request: Request,
        budget: RetentionBudget,
        expected: SessionKey,
    ) -> Result<Self, String> {
        let (sender, receiver) = bounded_channel(&budget, PREPARATION_COMPLETION_CAPACITY)
            .map_err(|error| error_text(format!("{error:?}")))?;
        let delivery = sender
            .reserve()
            .map_err(|error| error_text(format!("{error:?}")))?;
        let task_budget = budget.clone();
        let task = admit_task(&task_budget, move || {
            let prepared = request
                .prepare(&budget)
                .map_err(error_text)
                .map_err(|message| {
                    WorkerFailure::admit_with_fallback(
                        message.into(),
                        &budget,
                        WORKER_FAILURE_ADMISSION_MESSAGE,
                    )
                });
            let _ = delivery.send(prepared);
        })
        .map_err(|error| error_text(format!("{error:?}")))?;
        std::thread::Builder::new()
            .name("merlin-upstream-capture".into())
            .spawn(move || run_task(task))
            .map_err(|error| error_text(error.to_string()))?;
        Ok(Self::Capture { expected, receiver })
    }
}
