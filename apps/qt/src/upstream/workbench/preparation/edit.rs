/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Typed candidate preparation using the existing controller slot and transport.
use super::*;
use serde_json::Value;

impl Pending {
    /// Queues a correlated typed edit while retaining its original source owner.
    /// # Arguments
    /// * `controller` - Exact accepted idle owner; restored on every failure path.
    /// * `budget` - Existing canonical counter for slot, task and completed candidate.
    /// * `draft` - Admitted immutable field destinations and original base identity.
    /// * `changes` - Separately admitted exact changed values, never a whole replacement.
    /// # Returns
    /// One bounded pending edit without claiming a simulation execution lease.
    /// # Errors
    /// Slot/transport/task admission or spawn rejection returns the old controller.
    pub(in crate::upstream::workbench) fn edit(
        controller: UpstreamController,
        budget: RetentionBudget,
        draft: SharedDraft,
        changes: Box<Retained<Value>>,
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
        let worker_draft = Arc::clone(&draft);
        let transport = (|| {
            let (sender, receiver) = bounded_channel(&budget, PREPARATION_COMPLETION_CAPACITY)?;
            let delivery = sender.reserve()?;
            let task_budget = budget.clone();
            let task = admit_task(&task_budget, move || {
                let candidate = {
                    let controller = worker_owner
                        .value()
                        .lock()
                        .unwrap_or_else(|error| error.into_inner());
                    controller
                        .as_ref()
                        .expect("Pending edit retains its accepted controller")
                        .inspect(|simulation| {
                            worker_draft
                                .value()
                                .prepare(simulation, changes.value(), &budget)
                        })
                        .map_err(|error| format!("{error:?}"))
                        .and_then(|result| result)
                        .map_err(error_text)
                        .map_err(|message| {
                            WorkerFailure::admit_with_fallback(
                                message.into(),
                                &budget,
                                WORKER_FAILURE_ADMISSION_MESSAGE,
                            )
                        })
                };
                let _ = delivery.send(candidate);
            })?;
            Ok::<_, crate::shell::RetentionError>((receiver, task))
        })();
        let (receiver, task) = match transport {
            Ok(value) => value,
            Err(error) => return Err((take_controller(&owner), error_text(format!("{error:?}")))),
        };
        if let Err(error) = std::thread::Builder::new()
            .name("merlin-upstream-edit".into())
            .spawn(move || run_task(task))
        {
            return Err((take_controller(&owner), error_text(error.to_string())));
        }
        Ok(Self::Edit {
            draft,
            receiver,
            owner,
        })
    }

    /// Identifies only the configuration preparation associated with this draft.
    /// # Arguments
    /// * `draft` - Original immutable owner, compared by allocation identity.
    /// # Returns
    /// False for raw imports, execution preparation or another draft; no mutation.
    pub(in crate::upstream::workbench) fn is_edit(&self, draft: &SharedDraft) -> bool {
        matches!(self, Self::Edit { draft: pending, .. } if Arc::ptr_eq(pending, draft))
    }
}
