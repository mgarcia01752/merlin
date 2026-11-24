/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Finite off-GUI preparation; these threads never hold a simulation run lease.
use super::{error_text, UpstreamController};
use crate::upstream::{PreparedRun, PreparedSession};
use crate::{
    shared::{
        presentation::constants::WORKER_FAILURE_ADMISSION_MESSAGE, worker_failure::WorkerFailure,
    },
    shell::{Retained, RetentionBudget, SessionKey},
};
use merlin::upstream::{ConfigurationDocument, ReceiverObservationPolicy};
use std::sync::{
    mpsc::{self, Receiver, TryRecvError},
    Arc, Mutex,
};

// One preparation per model and one completion; unrelated to scientific callbacks.
const PREPARATION_COMPLETION_CAPACITY: usize = 1;

type ControllerSlot = Arc<Mutex<Option<UpstreamController>>>;

pub(super) enum Pending {
    Configuration {
        expected: SessionKey,
        receiver: Receiver<Result<PreparedSession, WorkerFailure>>,
    },
    Run {
        receiver: Receiver<Result<PreparedRun, WorkerFailure>>,
        owner: ControllerSlot,
    },
}

pub(super) enum Prepared {
    Configuration {
        expected: SessionKey,
        candidate: Result<PreparedSession, WorkerFailure>,
    },
    Run {
        controller: UpstreamController,
        target: Result<PreparedRun, WorkerFailure>,
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
        let (sender, receiver) = mpsc::sync_channel(PREPARATION_COMPLETION_CAPACITY);
        std::thread::Builder::new()
            .name("merlin-upstream-prepare".into())
            .spawn(move || {
                let prepared = ConfigurationDocument::from_json(bytes.value())
                    .map_err(|error| error.to_string())
                    .and_then(|document| {
                        PreparedSession::from_document(
                            document,
                            ReceiverObservationPolicy::default(),
                            &budget,
                        )
                        .map_err(|error| format!("{error:?}"))
                    })
                    .map_err(error_text)
                    .map_err(|message| {
                        WorkerFailure::admit_with_fallback(
                            message.into(),
                            &budget,
                            WORKER_FAILURE_ADMISSION_MESSAGE,
                        )
                    });
                let _ = sender.send(prepared);
            })
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
    /// Spawn failure returns the original controller for immediate restoration.
    pub(super) fn run(
        controller: UpstreamController,
        budget: RetentionBudget,
    ) -> Result<Self, (UpstreamController, String)> {
        let owner = Arc::new(Mutex::new(Some(controller)));
        let worker_owner = Arc::clone(&owner);
        let (sender, receiver) = mpsc::sync_channel(PREPARATION_COMPLETION_CAPACITY);
        let spawned = std::thread::Builder::new()
            .name("merlin-upstream-target".into())
            .spawn(move || {
                let target = {
                    let slot = worker_owner
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
                let _ = sender.send(target);
            });
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
            Self::Configuration { expected, receiver } => {
                receive(receiver).map(|candidate| Prepared::Configuration {
                    expected: *expected,
                    candidate,
                })
            }
            Self::Run { receiver, owner } => receive(receiver).map(|target| Prepared::Run {
                controller: take_controller(owner),
                target,
            }),
        }
    }
}

fn receive<T>(receiver: &Receiver<Result<T, WorkerFailure>>) -> Option<Result<T, WorkerFailure>> {
    match receiver.try_recv() {
        Ok(value) => Some(value),
        Err(TryRecvError::Empty) => None,
        Err(TryRecvError::Disconnected) => Some(Err(WorkerFailure::Static(
            "Upstream preparation stopped unexpectedly.",
        ))),
    }
}

fn take_controller(owner: &ControllerSlot) -> UpstreamController {
    owner
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .take()
        .expect("A preparation completion is consumed exactly once")
}
