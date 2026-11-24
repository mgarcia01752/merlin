/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! One bounded serial dispatcher using only public downstream jobs and controls.
use super::{
    request_retention::{self, OwnerError, SharedPlan, SharedRequest},
    result_retention::{PreparedResult, ResultError},
    ResultData, RunTiming,
};
use crate::{
    downstream::common::constants::PROFILE_COMPLETION_CAPACITY,
    shared::{
        presentation::constants::WORKER_FAILURE_ADMISSION_MESSAGE, worker_failure::WorkerFailure,
    },
    shell::{
        spawn_worker, Retained, RetainedResources, RetentionBudget, RetentionError,
        RunCancellation, RunLease, RunToken, WorkerEvent, WorkerSpawnError,
    },
};
use merlin::downstream::{ChannelJobStatus, FullLoopControl, FullLoopRequest, FullLoopState};
use std::{
    mem::size_of,
    sync::{mpsc, Arc, Mutex, MutexGuard},
};

pub(super) enum WorkerMessage {
    Profile(Box<ResultData>),
    Rejected(u8, u8, WorkerFailure),
    Finished(&'static str),
}

#[derive(Default)]
struct Dispatch {
    cancelled: bool,
    paused: bool,
    active: Option<Arc<Retained<FullLoopControl>>>,
    index: usize,
    id: u8,
    completed: u64,
}

/// Shared application cancellation adapter; scientific state stays backend-owned.
pub(super) struct DownstreamControl {
    state: Mutex<Dispatch>,

    // Dispatch and its active scientific owner drop before the adapter charge.
    _guard: Retained<()>,
}

pub(super) struct Worker {
    control: Arc<DownstreamControl>,
    token: RunToken,
    pub receiver: mpsc::Receiver<WorkerEvent<WorkerMessage, WorkerFailure>>,
    pub profile_count: usize,
    pub stale: bool,
}

fn lock(state: &Mutex<Dispatch>) -> MutexGuard<'_, Dispatch> {
    state.lock().unwrap_or_else(|cause| cause.into_inner())
}

/// Executes a public profile only after its working input and result are admitted.
///
/// # Arguments
/// * `request` - Owned finite profile preparation candidate; consumed before execution.
/// * `runtime_ms` - Remaining active sweep budget; never widens admission.
/// * `control` - Existing fresh public one-shot profile control.
/// * `budget` - Canonical host budget shared with old, queued and accepted owners.
///
/// # Returns
/// Actual complete/partial result with its pre-execution charge. Existing dynamic
/// observation/capture copies remain explicit separate application work.
///
/// # Errors
/// Preserves public and target-admission failure before scientific control claim;
/// projection warnings never discard completed FEC comparisons.
pub(super) fn run_one_admitted(
    request: FullLoopRequest,
    runtime_ms: u64,
    control: &FullLoopControl,
    budget: &RetentionBudget,
) -> Result<ResultData, String> {
    let request =
        request_retention::admit_run_request(request, runtime_ms, budget).map_err(|error| {
            match error {
                OwnerError::Metadata(error) => error.message().to_owned(),
                OwnerError::Retention(error) => {
                    super::application_error(crate::shell::ApplicationError::Retention(error))
                }
            }
        })?;
    let prepared =
        PreparedResult::new(request.value(), budget).map_err(ResultError::into_message)?;
    let result = prepared.run(control).map_err(ResultError::into_message)?;
    Ok(ResultData::from_retained(result, budget))
}

/// Preserves existing scientific fixture calls through the admitted real path.
///
/// # Arguments
/// * `request`, `runtime_ms`, `control` - Original public fixture execution inputs.
///
/// # Returns
/// Real science using an independently supplied test-only budget, never production policy.
///
/// # Errors
/// Preserves the common admitted execution errors.
#[cfg(test)]
pub(super) fn run_one(
    request: &FullLoopRequest,
    runtime_ms: u64,
    control: &FullLoopControl,
) -> Result<ResultData, String> {
    run_one_admitted(
        request.clone(),
        runtime_ms,
        control,
        &super::result_retention::fixture_budget(),
    )
}

impl DownstreamControl {
    /// Admits idle dispatch storage before allocating its shared cancellation adapter.
    ///
    /// # Arguments
    /// * `budget` - Canonical host counter also covering active scientific controls.
    ///
    /// # Returns
    /// One guarded Arc shared by the canonical gate and its exact worker.
    ///
    /// # Errors
    /// Any aggregate dimension before adapter publication or lease acquisition.
    pub(super) fn new(budget: &RetentionBudget) -> Result<Arc<Self>, RetentionError> {
        let guard = budget.reserve_resources(adapter_resources())?.retain(());
        Ok(Arc::new(Self {
            state: Mutex::new(Dispatch::default()),
            _guard: guard,
        }))
    }

    /// Reads actual backend pause acknowledgement without changing execution.
    ///
    /// # Returns
    /// Some(true) only for acknowledged Paused; Some(false) only for Running.
    /// None before/between profiles, after completion or during cancellation.
    pub(super) fn acknowledged_pause(&self) -> Option<bool> {
        let dispatch = lock(&self.state);
        if dispatch.cancelled {
            return None;
        }
        let snapshot = dispatch.active.as_ref()?.value().snapshot();
        if snapshot.cancellation_requested {
            return None;
        }
        match snapshot.state {
            FullLoopState::Paused => Some(true),
            FullLoopState::Running => Some(false),
            FullLoopState::Idle
            | FullLoopState::Completed
            | FullLoopState::Cancelled
            | FullLoopState::RuntimeExceeded
            | FullLoopState::Failed => None,
        }
    }

    /// Cancels the active public control and prevents later profiles from starting.
    fn cancel(&self) {
        let mut dispatch = lock(&self.state);
        if dispatch.cancelled {
            return;
        }
        dispatch.cancelled = true;
        if let Some(control) = &dispatch.active {
            control.value().cancel();
        }
    }

    /// Toggles the requested pause across profile boundaries without resetting state.
    fn pause(&self) {
        let mut dispatch = lock(&self.state);
        dispatch.paused = !dispatch.paused;
        if let Some(control) = &dispatch.active {
            if dispatch.paused {
                // A terminal control can race with this request; no later source is advanced.
                let _ = control.value().pause();
            } else {
                control.value().resume();
            }
        }
    }

    /// Reads bounded UI progress from the current public control.
    /// # Arguments
    /// * `timing` - Presentation clock belonging to this worker's sweep.
    /// # Returns
    /// Requested pause, zero-based ordinal, original profile ID and completed symbols.
    fn progress(&self, timing: &RunTiming) -> (bool, usize, u8, u64) {
        let mut dispatch = lock(&self.state);
        if let Some(control) = &dispatch.active {
            dispatch.completed = timing.observe(control.value());
        }
        (
            dispatch.paused,
            dispatch.index,
            dispatch.id,
            dispatch.completed,
        )
    }
}

impl RunCancellation for DownstreamControl {
    /// Requests cooperative stop without waiting or releasing global admission.
    ///
    /// The public active control is cancelled at most once by this adapter;
    /// unstarted profiles remain skipped until the real worker terminal arrives.
    fn request_cancel(&self) {
        self.cancel();
    }
}

impl Worker {
    /// Starts one serial dispatcher with bounded completion backpressure.
    /// # Arguments
    /// * `request` - Admitted immutable owner; no payload clone at worker dispatch.
    /// * `plan` - Optional admitted sweep owner, shared until the closure returns.
    /// * `timing` - Shared presentation clock, always locked after dispatch state.
    /// * `control` - Same cancellation adapter admitted by the canonical gate.
    /// * `lease` - Genuine exact-session admission retained until common termination.
    /// * `retention` - Canonical host budget for every actual profile target.
    /// # Returns
    /// Controls and identified data/terminal receiver; the GUI never joins.
    /// Finished is data only: retain this Worker until its common terminal arrives.
    /// # Errors
    /// Returns OS thread-start failure with the original unstarted lease. The host
    /// must acknowledge that failure; dropping a lease alone keeps the gate Busy.
    pub(super) fn spawn(
        request: SharedRequest,
        plan: Option<SharedPlan>,
        timing: Arc<RunTiming>,
        control: Arc<DownstreamControl>,
        lease: RunLease,
        retention: RetentionBudget,
    ) -> Result<Self, WorkerSpawnError> {
        let ids = plan
            .as_ref()
            .map(|plan| {
                plan.value()
                    .info()
                    .selected_profile_ids
                    .into_iter()
                    .map(|id| id.value())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(|| {
                vec![request
                    .value()
                    .settings()
                    .playback
                    .as_ref()
                    .map_or(0, |playback| playback.profile.profile_id)]
            });
        let profile_count = ids.len();
        let budget = request.value().options().max_runtime_ms;
        let shared = Arc::clone(&control);
        let token = lease.token();
        let receiver = spawn_worker(
            "merlin-profile-sweep",
            PROFILE_COMPLETION_CAPACITY,
            lease,
            move |sender| {
                let mut elapsed = 0_u64;
                for (index, id) in ids.into_iter().enumerate() {
                    if lock(&shared.state).cancelled || elapsed >= budget {
                        break;
                    }
                    // Public selection/narrowing is finite preparation. The final
                    // candidate is separately admitted before control/source execution.
                    let prepared = plan.as_ref().map_or_else(
                        || Ok(request.value().as_ref().clone()),
                        |plan| plan.value().request(index),
                    );
                    let outcome = match admit_profile_control(&retention) {
                        Ok(control) => {
                            {
                                let mut dispatch = lock(&shared.state);
                                if dispatch.cancelled {
                                    break;
                                }
                                dispatch.index = index;
                                dispatch.id = id;
                                dispatch.completed = 0;
                                if dispatch.paused {
                                    // Preserve the public idle pause before any source step.
                                    let _ = control.value().pause();
                                }
                                dispatch.active = Some(Arc::clone(&control));
                                timing.begin_profile();
                            }
                            let outcome = prepared
                                .map_err(|cause| cause.message().to_owned())
                                .and_then(|request| {
                                    run_one_admitted(
                                        request,
                                        budget - elapsed,
                                        control.value(),
                                        &retention,
                                    )
                                });
                            {
                                let mut dispatch = lock(&shared.state);
                                dispatch.completed = timing.observe(control.value());
                                timing.end_profile(id);
                                dispatch.active = None;
                            }
                            outcome
                        }
                        Err(error) => {
                            if lock(&shared.state).cancelled {
                                break;
                            }
                            Err(error)
                        }
                    };
                    match outcome {
                        Ok(result) => {
                            elapsed = elapsed.saturating_add(result.job.elapsed_ms);
                            let halt = matches!(
                                result.job.status,
                                ChannelJobStatus::Cancelled | ChannelJobStatus::RuntimeExceeded
                            );
                            if sender
                                .send(WorkerMessage::Profile(Box::new(result)))
                                .is_err()
                            {
                                timing.finish();
                                return Ok(());
                            }
                            if halt {
                                break;
                            }
                        }
                        Err(error) => {
                            let channel = request
                                .value()
                                .settings()
                                .playback
                                .as_ref()
                                .map_or(0, |playback| playback.profile.key.channel_id);
                            let error = WorkerFailure::admit_with_fallback(
                                error.into(),
                                &retention,
                                WORKER_FAILURE_ADMISSION_MESSAGE,
                            );
                            if sender
                                .send(WorkerMessage::Rejected(id, channel, error))
                                .is_err()
                            {
                                timing.finish();
                                return Ok(());
                            }
                        }
                    }
                }
                let reason = if lock(&shared.state).cancelled {
                    "Profile sweep cancelled; unstarted profiles remain marked Not Started."
                } else if elapsed >= budget {
                    "Profile sweep active runtime budget reached."
                } else {
                    ""
                };
                timing.finish();
                let _ = sender.send(WorkerMessage::Finished(reason));
                Ok(())
            },
        )?;
        Ok(Self {
            control,
            token,
            receiver,
            profile_count,
            stale: false,
        })
    }

    /// Reads this worker's immutable canonical callback identity.
    ///
    /// # Returns
    /// Exact direction/session/generation/run supplied by the genuine lease.
    pub(super) fn token(&self) -> RunToken {
        self.token
    }

    /// Cancels public execution without releasing the terminal-owned lease.
    ///
    /// # Returns
    /// Unit immediately after requesting cooperative stop; never joins.
    pub(super) fn cancel(&self) {
        self.control.request_cancel();
    }

    /// Toggles the requested pause while preserving the same scientific state.
    ///
    /// # Returns
    /// Unit; actual pause acknowledgement is reported independently.
    pub(super) fn pause(&self) {
        self.control.pause();
    }

    /// Reads actual pause acknowledgement for the canonical gate.
    ///
    /// # Returns
    /// Some(true) for Paused, Some(false) for Running, otherwise None.
    pub(super) fn acknowledged_pause(&self) -> Option<bool> {
        self.control.acknowledged_pause()
    }

    /// Reads bounded UI progress from the current public control.
    ///
    /// # Arguments
    /// * `timing` - Presentation clock belonging to this worker's sweep.
    ///
    /// # Returns
    /// Requested pause, zero-based ordinal, original profile ID and completed symbols.
    pub(super) fn progress(&self, timing: &RunTiming) -> (bool, usize, u8, u64) {
        self.control.progress(timing)
    }
}

impl Drop for Worker {
    /// Cancels unfinished public execution when its presentation owner is released.
    /// # Returns
    /// Unit without joining the simulation thread or blocking the GUI.
    fn drop(&mut self) {
        // Covers direct worker ownership as well as the Model's explicit cancellation.
        // Dropping the receiver also releases bounded-send backpressure; never join Qt.
        self.cancel();
    }
}

fn adapter_resources() -> RetainedResources {
    RetainedResources {
        payload_bytes: size_of::<DownstreamControl>(),
        ordered_entries: 0,
        shared_roots: 1,
    }
}

fn profile_control_resources(
    scientific: merlin::common::RetainedFootprint,
) -> Result<RetainedResources, RetentionError> {
    let children = scientific
        .payload_bytes
        .checked_sub(size_of::<FullLoopControl>())
        .ok_or(RetentionError::ArithmeticOverflow)?;
    Ok(RetainedResources {
        payload_bytes: children
            .checked_add(size_of::<Retained<FullLoopControl>>())
            .ok_or(RetentionError::ArithmeticOverflow)?,
        ordered_entries: scientific.ordered_entries,
        shared_roots: scientific
            .shared_roots
            .checked_add(1)
            .ok_or(RetentionError::ArithmeticOverflow)?,
    })
}

fn admit_profile_control(
    budget: &RetentionBudget,
) -> Result<Arc<Retained<FullLoopControl>>, String> {
    let control = FullLoopControl::new();
    let scientific = control
        .retained_footprint()
        .map_err(|error| error.message().to_owned())?;
    let resources = profile_control_resources(scientific).map_err(|error| {
        super::application_error(crate::shell::ApplicationError::Retention(error))
    })?;
    let reservation = budget.reserve_resources(resources).map_err(|error| {
        super::application_error(crate::shell::ApplicationError::Retention(error))
    })?;
    Ok(Arc::new(reservation.retain(control)))
}

/// Measures the two actual control charges for precise fixture pressure only.
///
/// # Returns
/// Adapter and one scientific profile owner quantities; no production defaults.
#[cfg(test)]
pub(super) fn fixture_control_resources() -> (RetainedResources, RetainedResources) {
    let control = FullLoopControl::new();
    (
        adapter_resources(),
        profile_control_resources(control.retained_footprint().unwrap()).unwrap(),
    )
}

#[cfg(test)]
mod control_retention_tests;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod failure_retention_tests;
