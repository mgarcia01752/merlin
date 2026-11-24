/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Passive public resource queries reuse the existing admitted controller slot.
use super::*;
use merlin::upstream::ResourceGridRequest;

impl Pending {
    /// Queries an owned grid away from Qt while preserving the original controller.
    /// # Arguments
    /// * `controller` - Accepted idle scientific owner, never cloned or executed.
    /// * `budget` - Existing canonical counter for the slot, task and retained DTO.
    /// * `expected` - Actual application key, independent of backend generation.
    /// * `request` - Original-coordinate bounded public viewport request.
    /// # Returns
    /// One finite pending completion without acquiring a simulation run lease.
    /// # Errors
    /// Slot/transport/dispatch failure returns the unchanged original controller.
    pub(in crate::upstream::workbench) fn grid(
        controller: UpstreamController,
        budget: RetentionBudget,
        expected: SessionKey,
        request: ResourceGridRequest,
    ) -> Result<Self, (UpstreamController, String)> {
        let reservation = match budget.reserve_resources(RetainedResources {
            payload_bytes: std::mem::size_of::<Retained<Mutex<Option<UpstreamController>>>>(),
            shared_roots: CONTROLLER_SLOT_SHARED_ROOTS,
            ..Default::default()
        }) {
            Ok(reservation) => reservation,
            Err(error) => return Err((controller, error_text(format!("{error:?}")))),
        };
        let owner = Arc::new(reservation.retain(Mutex::new(Some(controller))));
        let worker_owner = Arc::clone(&owner);
        let transport = (|| {
            let (sender, receiver) = bounded_channel(&budget, PREPARATION_COMPLETION_CAPACITY)?;
            let delivery = sender.reserve()?;
            let task_budget = budget.clone();
            let task = admit_task(&task_budget, move || {
                let result = {
                    let slot = worker_owner
                        .value()
                        .lock()
                        .unwrap_or_else(|error| error.into_inner());
                    super::super::grid::observe(
                        slot.as_ref()
                            .expect("Grid preparation retains the original controller"),
                        request,
                        &budget,
                    )
                    .map_err(error_text)
                    .map_err(|message| {
                        WorkerFailure::admit_with_fallback(
                            message.into(),
                            &budget,
                            WORKER_FAILURE_ADMISSION_MESSAGE,
                        )
                    })
                };
                let _ = delivery.send(result);
            })?;
            Ok::<_, crate::shell::RetentionError>((receiver, task))
        })();
        let (receiver, task) = match transport {
            Ok(prepared) => prepared,
            Err(error) => return Err((take_controller(&owner), error_text(format!("{error:?}")))),
        };
        if let Err(error) = std::thread::Builder::new()
            .name("merlin-upstream-grid".into())
            .spawn(move || run_task(task))
        {
            return Err((take_controller(&owner), error_text(error.to_string())));
        }
        Ok(Self::Grid {
            expected,
            receiver,
            owner,
        })
    }
}
