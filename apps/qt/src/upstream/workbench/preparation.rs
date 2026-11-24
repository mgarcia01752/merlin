/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Finite off-GUI preparation; these threads never hold a simulation run lease.
mod capture;
mod edit;
mod grid;
use super::captures::CaptureDisplay;
use super::drafts::SharedDraft;
use super::grid::Display as GridDisplay;
use super::{error_text, UpstreamController};
use crate::upstream::{PreparedRun, PreparedSession};
use crate::{
    shared::{
        presentation::constants::WORKER_FAILURE_ADMISSION_MESSAGE, worker_failure::WorkerFailure,
    },
    shell::{
        admit_task, bounded_channel, run_task, AdmittedTask, DeliveryReceiver, Retained,
        RetainedResources, RetentionBudget, RetentionReservation, SessionKey,
    },
};
use merlin::upstream::{
    inputs::MeasuredExperiment, ConfigurationDocument, ReceiverObservationPolicy,
};
use serde::Deserialize;
use std::sync::{mpsc::TryRecvError, Arc, Mutex};

// One preparation per model and one completion; unrelated to scientific callbacks.
const PREPARATION_COMPLETION_CAPACITY: usize = 1;

// One additional Arc pointee while the controller lives outside its GUI model.
const CONTROLLER_SLOT_SHARED_ROOTS: usize = 1;

// Content routing selects a public parser; that parser remains authoritative.
const MEASURED_INPUT_FORMAT: &str = "merlin.upstream.measured-experiment";

#[derive(Deserialize)]
struct InputFormat {
    format: String,
}

type ControllerSlot = Arc<Retained<Mutex<Option<UpstreamController>>>>;

pub(super) enum Pending {
    Grid {
        expected: SessionKey,
        receiver: DeliveryReceiver<Result<Retained<GridDisplay>, WorkerFailure>>,
        owner: ControllerSlot,
    },
    Capture {
        expected: SessionKey,
        receiver: DeliveryReceiver<Result<Retained<CaptureDisplay>, WorkerFailure>>,
    },
    Edit {
        draft: SharedDraft,
        receiver: DeliveryReceiver<Result<PreparedSession, WorkerFailure>>,
        owner: ControllerSlot,
    },
    Configuration {
        expected: SessionKey,
        receiver: DeliveryReceiver<Result<PreparedSession, WorkerFailure>>,
    },
    Run {
        receiver: DeliveryReceiver<Result<PreparedRun, WorkerFailure>>,
        owner: ControllerSlot,
    },
}

pub(super) enum Prepared {
    Grid {
        expected: SessionKey,
        controller: UpstreamController,
        candidate: Result<Retained<GridDisplay>, WorkerFailure>,
        delivery: Option<RetentionReservation>,
    },
    Capture {
        expected: SessionKey,
        candidate: Result<Retained<CaptureDisplay>, WorkerFailure>,
        delivery: Option<RetentionReservation>,
    },
    Edit {
        draft: SharedDraft,
        controller: UpstreamController,
        candidate: Result<PreparedSession, WorkerFailure>,
        delivery: Option<RetentionReservation>,
    },
    Configuration {
        expected: SessionKey,
        candidate: Result<PreparedSession, WorkerFailure>,
        delivery: Option<RetentionReservation>,
    },
    Run {
        controller: UpstreamController,
        target: Result<PreparedRun, WorkerFailure>,
        delivery: Option<RetentionReservation>,
    },
}

impl Pending {
    /// Parses and admits a candidate away from the Qt thread.
    /// # Arguments
    /// * `bytes` - Bounded original public configuration document.
    /// * `budget` - Same application retention policy as accepted owners.
    /// * `expected` - Canonical key captured before preparation; checked at adoption.
    /// # Returns
    /// One pending completion; discarded receivers cannot adopt into any model.
    /// # Errors
    /// Thread creation failure leaves accepted configuration untouched.
    pub(super) fn configuration(
        bytes: Retained<Box<[u8]>>,
        budget: RetentionBudget,
        expected: SessionKey,
    ) -> Result<Self, String> {
        let (sender, receiver) = bounded_channel(&budget, PREPARATION_COMPLETION_CAPACITY)
            .map_err(|error| error_text(format!("{error:?}")))?;
        let delivery = sender
            .reserve()
            .map_err(|error| error_text(format!("{error:?}")))?;
        let bytes = Box::new(bytes);
        let task_budget = budget.clone();
        let task = admit_task(&task_budget, move || {
            let prepared = prepare_input(bytes.value(), &budget)
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
            .name("merlin-upstream-prepare".into())
            .spawn(move || run_task(task))
            .map_err(|error| error_text(error.to_string()))?;
        Ok(Self::Configuration { expected, receiver })
    }

    /// Preflights the actual report target while preserving the idle controller.
    /// # Arguments
    /// * `controller` - Accepted idle owner; never cloned or scientifically run here.
    /// * `budget` - Canonical counter for the prepared target and queued failure.
    /// # Returns
    /// Pending result retaining the original owner even if preflight unwinds.
    /// # Errors
    /// Slot admission or spawn failure returns the original controller unchanged.
    /// Fixed slot storage is reserved before Arc/channel/spawn; worker aliases
    /// retain it through cancellation, disconnect or GUI-owner destruction.
    pub(super) fn run(
        controller: UpstreamController,
        budget: RetentionBudget,
    ) -> Result<Self, (UpstreamController, String)> {
        Self::run_with(controller, budget, |task| {
            std::thread::Builder::new()
                .name("merlin-upstream-target".into())
                .spawn(move || run_task(task))
                .map(drop)
        })
    }

    /// Dispatches the same admitted task through the selected OS starter.
    ///
    /// # Arguments
    /// * `controller`, `budget` - Original accepted owner and canonical counter.
    /// * `start` - Called only after actual slot, transport and closure admission.
    ///
    /// # Returns
    /// The ordinary pending target operation without changing its scientific call.
    ///
    /// # Errors
    /// Admission or OS dispatch rejection returns the original idle controller.
    pub(super) fn run_with(
        controller: UpstreamController,
        budget: RetentionBudget,
        start: impl FnOnce(AdmittedTask) -> std::io::Result<()>,
    ) -> Result<Self, (UpstreamController, String)> {
        let reservation = match budget.reserve_resources(RetainedResources {
            payload_bytes: std::mem::size_of::<Retained<Mutex<Option<UpstreamController>>>>(),
            ordered_entries: 0,
            shared_roots: CONTROLLER_SLOT_SHARED_ROOTS,
            ..RetainedResources::default()
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
                let target = {
                    let slot = worker_owner
                        .value()
                        .lock()
                        .unwrap_or_else(|error| error.into_inner());
                    slot.as_ref()
                        .expect("Preparation owns its accepted controller")
                        .prepare_run()
                        .map_err(|error| error_text(format!("{error:?}")))
                        .map_err(|message| {
                            WorkerFailure::admit_with_fallback(
                                message.into(),
                                &budget,
                                WORKER_FAILURE_ADMISSION_MESSAGE,
                            )
                        })
                };
                let _ = delivery.send(target);
            })?;
            Ok::<_, crate::shell::RetentionError>((receiver, task))
        })();
        let (receiver, task) = match transport {
            Ok(prepared) => prepared,
            Err(error) => return Err((take_controller(&owner), error_text(format!("{error:?}")))),
        };
        let spawned = start(task);
        if let Err(error) = spawned {
            let controller = take_controller(&owner);
            return Err((controller, error_text(error.to_string())));
        }
        Ok(Self::Run { receiver, owner })
    }

    /// Polls at most one completion without waiting for unfinished preparation.
    /// # Returns
    /// None while pending; original owner and an explicit error after disconnection.
    pub(super) fn poll(&self) -> Option<Prepared> {
        match self {
            Self::Grid {
                expected,
                receiver,
                owner,
            } => receive(receiver).map(|(delivery, candidate)| Prepared::Grid {
                expected: *expected,
                controller: take_controller(owner),
                candidate,
                delivery,
            }),
            Self::Capture { expected, receiver } => {
                receive(receiver).map(|(delivery, candidate)| Prepared::Capture {
                    expected: *expected,
                    delivery,
                    candidate,
                })
            }
            Self::Edit {
                draft,
                receiver,
                owner,
            } => receive(receiver).map(|(delivery, candidate)| Prepared::Edit {
                draft: Arc::clone(draft),
                controller: take_controller(owner),
                candidate,
                delivery,
            }),
            Self::Configuration { expected, receiver } => {
                receive(receiver).map(|(delivery, candidate)| Prepared::Configuration {
                    expected: *expected,
                    delivery,
                    candidate,
                })
            }
            Self::Run { receiver, owner } => {
                receive(receiver).map(|(delivery, target)| Prepared::Run {
                    controller: take_controller(owner),
                    delivery,
                    target,
                })
            }
        }
    }
}

fn prepare_input(bytes: &[u8], budget: &RetentionBudget) -> Result<PreparedSession, String> {
    let measured = serde_json::from_slice::<InputFormat>(bytes)
        .is_ok_and(|input| input.format == MEASURED_INPUT_FORMAT);
    if measured {
        let plan = MeasuredExperiment::from_json(bytes).map_err(|error| error.to_string())?;
        PreparedSession::from_measured(plan, ReceiverObservationPolicy::default(), budget)
            .map_err(|error| format!("{error:?}"))
    } else {
        let document =
            ConfigurationDocument::from_json(bytes).map_err(|error| error.to_string())?;
        PreparedSession::from_document(document, ReceiverObservationPolicy::default(), budget)
            .map_err(|error| format!("{error:?}"))
    }
}

fn receive<T>(
    receiver: &DeliveryReceiver<Result<T, WorkerFailure>>,
) -> Option<(Option<RetentionReservation>, Result<T, WorkerFailure>)> {
    match receiver.try_recv() {
        Ok(value) => {
            let (value, guard) = value.into_parts();
            Some((Some(guard), value))
        }
        Err(TryRecvError::Empty) => None,
        Err(TryRecvError::Disconnected) => Some((
            None,
            Err(WorkerFailure::Static(
                "Upstream preparation stopped unexpectedly.",
            )),
        )),
    }
}

fn take_controller(owner: &ControllerSlot) -> UpstreamController {
    owner
        .value()
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .take()
        .expect("A preparation completion is consumed exactly once")
}

#[cfg(test)]
mod transport_retention_tests;
