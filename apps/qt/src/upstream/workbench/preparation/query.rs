/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Existing controller-slot query transport shared by actual grid/instrument consumers.
use super::*;

type QueryCompletion<T> = (
    ControllerSlot,
    DeliveryReceiver<Result<Retained<T>, WorkerFailure>>,
);

/// Dispatches one passive query while retaining and restoring its original controller.
/// # Arguments
/// * `controller`, `budget` - Existing admitted owner and canonical counter.
/// * `name` - Static application worker label, without scientific/input text.
/// * `query` - Finite public operation executed off the GUI thread under the slot.
/// # Returns
/// Original controller slot and one bounded completion; caller restores the owner
/// before adopting a result. No run lease, source clone or new transport is added.
/// # Errors
/// Admission/spawn failure returns the exact original controller and diagnostic.
pub(super) fn controller_query<T: Send + 'static>(
    controller: UpstreamController,
    budget: RetentionBudget,
    name: &'static str,
    query: impl FnOnce(&UpstreamController, &RetentionBudget) -> Result<Retained<T>, String>
        + Send
        + 'static,
) -> Result<QueryCompletion<T>, (UpstreamController, String)> {
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
                query(
                    slot.as_ref()
                        .expect("Passive query retains the original controller"),
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
        .name(name.into())
        .spawn(move || run_task(task))
    {
        return Err((take_controller(&owner), error_text(error.to_string())));
    }
    Ok((owner, receiver))
}
