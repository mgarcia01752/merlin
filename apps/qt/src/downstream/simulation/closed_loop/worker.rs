/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! One bounded serial dispatcher using only public downstream jobs and controls.
use super::{ResultData, RunTiming};
use crate::downstream::common::constants::PROFILE_COMPLETION_CAPACITY;
use merlin::downstream::{ChannelJobStatus, FullLoopControl, FullLoopRequest, FullLoopSweepPlan};
use std::sync::{mpsc, Arc, Mutex, MutexGuard};

pub(super) enum WorkerMessage {
    Profile(Box<ResultData>),
    Rejected(u8, u8, String),
    Finished(String),
}

#[derive(Default)]
struct Dispatch {
    cancelled: bool,
    paused: bool,
    active: Option<FullLoopControl>,
    index: usize,
    id: u8,
    completed: u64,
}

pub(super) struct Worker {
    state: Arc<Mutex<Dispatch>>,
    pub receiver: mpsc::Receiver<WorkerMessage>,
    pub profile_count: usize,
    pub stale: bool,
}

fn lock(state: &Mutex<Dispatch>) -> MutexGuard<'_, Dispatch> {
    state.lock().unwrap_or_else(|cause| cause.into_inner())
}

/// Executes one already admitted request with a narrowed remaining active-time budget.
/// # Arguments
/// * `request` - Immutable public request including its bounded observation policy.
/// * `runtime_ms` - Remaining sweep budget; may narrow but never widen admission.
/// * `control` - Fresh public one-shot control for this profile.
/// # Returns
/// Actual complete/partial report with backend-controlled retained capacities,
/// plus passive instrument copies. Aggregate application ownership is separate.
/// # Errors
/// Preserves public admission or dispatch failure; completed reports are not discarded
/// when a presentation observation is unavailable.
pub(super) fn run_one(
    request: &FullLoopRequest,
    runtime_ms: u64,
    control: &FullLoopControl,
) -> Result<ResultData, String> {
    request
        .with_runtime_limit(runtime_ms)
        .and_then(|request| {
            let target = request.result_retained_bound()?;
            request.run_bounded(control, target)
        })
        .map(ResultData::new)
        .map_err(|cause| cause.message().to_owned())
}

impl Worker {
    /// Starts one serial dispatcher with bounded completion backpressure.
    /// # Arguments
    /// * `request` - Current public request; fallback for a single-profile run.
    /// * `plan` - Optional public preflighted sweep.
    /// * `timing` - Shared presentation clock, always locked after dispatch state.
    /// # Returns
    /// Worker controls and its completion receiver; the GUI never joins a running thread.
    /// # Errors
    /// Returns OS thread-start errors before publishing any active worker.
    pub(super) fn spawn(
        request: FullLoopRequest,
        plan: Option<FullLoopSweepPlan>,
        timing: Arc<RunTiming>,
    ) -> std::io::Result<Self> {
        let ids = plan
            .as_ref()
            .map(|plan| {
                plan.info()
                    .selected_profile_ids
                    .into_iter()
                    .map(|id| id.value())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(|| {
                vec![request
                    .settings()
                    .playback
                    .as_ref()
                    .map_or(0, |playback| playback.profile.profile_id)]
            });
        let profile_count = ids.len();
        let budget = request.options().max_runtime_ms;
        let state = Arc::new(Mutex::new(Dispatch::default()));
        let shared = state.clone();
        let (sender, receiver) = mpsc::sync_channel(PROFILE_COMPLETION_CAPACITY);
        std::thread::Builder::new()
            .name("merlin-profile-sweep".into())
            .spawn(move || {
                let mut elapsed = 0_u64;
                for (index, id) in ids.into_iter().enumerate() {
                    if lock(&shared).cancelled || elapsed >= budget {
                        break;
                    }
                    let prepared = plan
                        .as_ref()
                        .map_or_else(|| Ok(request.clone()), |plan| plan.request(index));
                    let control = FullLoopControl::new();
                    {
                        let mut dispatch = lock(&shared);
                        if dispatch.cancelled {
                            break;
                        }
                        dispatch.index = index;
                        dispatch.id = id;
                        dispatch.completed = 0;
                        if dispatch.paused {
                            // Public idle controls support pause before any source step.
                            let _ = control.pause();
                        }
                        dispatch.active = Some(control.clone());
                        timing.begin_profile();
                    }
                    let outcome = prepared
                        .map_err(|cause| cause.message().to_owned())
                        .and_then(|request| run_one(&request, budget - elapsed, &control));
                    {
                        let mut dispatch = lock(&shared);
                        dispatch.completed = timing.observe(&control);
                        timing.end_profile(id);
                        dispatch.active = None;
                    }
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
                                return;
                            }
                            if halt {
                                break;
                            }
                        }
                        Err(error) => {
                            let channel = request
                                .settings()
                                .playback
                                .as_ref()
                                .map_or(0, |playback| playback.profile.key.channel_id);
                            if sender
                                .send(WorkerMessage::Rejected(id, channel, error))
                                .is_err()
                            {
                                timing.finish();
                                return;
                            }
                        }
                    }
                }
                let reason = if lock(&shared).cancelled {
                    "Profile sweep cancelled; unstarted profiles remain marked Not Started."
                } else if elapsed >= budget {
                    "Profile sweep active runtime budget reached."
                } else {
                    ""
                };
                timing.finish();
                let _ = sender.send(WorkerMessage::Finished(reason.into()));
            })?;
        Ok(Self {
            state,
            receiver,
            profile_count,
            stale: false,
        })
    }

    /// Cancels the active public control and prevents later profiles from starting.
    pub(super) fn cancel(&self) {
        let mut dispatch = lock(&self.state);
        dispatch.cancelled = true;
        if let Some(control) = &dispatch.active {
            control.cancel();
        }
    }

    /// Toggles the requested pause across profile boundaries without resetting state.
    pub(super) fn pause(&self) {
        let mut dispatch = lock(&self.state);
        dispatch.paused = !dispatch.paused;
        if let Some(control) = &dispatch.active {
            if dispatch.paused {
                // A terminal control can race with this request; no later source is advanced.
                let _ = control.pause();
            } else {
                control.resume();
            }
        }
    }

    /// Reads bounded UI progress from the current public control.
    /// # Arguments
    /// * `timing` - Presentation clock belonging to this worker's sweep.
    /// # Returns
    /// Requested pause, zero-based ordinal, original profile ID and completed symbols.
    pub(super) fn progress(&self, timing: &RunTiming) -> (bool, usize, u8, u64) {
        let mut dispatch = lock(&self.state);
        if let Some(control) = &dispatch.active {
            dispatch.completed = timing.observe(control);
        }
        (
            dispatch.paused,
            dispatch.index,
            dispatch.id,
            dispatch.completed,
        )
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::downstream::simulation::closed_loop::{prepare, Options};
    use merlin::downstream::FullLoopState;
    use std::time::{Duration, Instant};

    // Scheduling policies for this bounded GUI-control regression, independent
    // of the one-second simulated observation and the 60-second active host budget.
    const CONTROL_ACK_TIMEOUT: Duration = Duration::from_secs(15);
    const CONTROL_POLL_INTERVAL: Duration = Duration::from_millis(5);
    const PAUSED_OBSERVATION_INTERVAL: Duration = Duration::from_millis(30);

    #[test]
    fn paused_public_worker_cancels_and_retains_its_actual_partial_report() {
        let (inputs, transmission, sections) = super::super::tests::accepted_inputs();
        let request = prepare(
            &transmission,
            &inputs,
            &Options {
                duration_ms: 1000,
                max_runtime_ms: 60000,
                ..Default::default()
            },
            &sections,
        )
        .unwrap();
        let timing = Arc::new(RunTiming::new());
        let worker = Worker::spawn(request, None, timing.clone()).unwrap();
        worker.pause();
        let deadline = Instant::now() + CONTROL_ACK_TIMEOUT;
        let control = loop {
            if let Some(control) = lock(&worker.state).active.clone() {
                if control.snapshot().state == FullLoopState::Paused {
                    break control;
                }
            }
            assert!(
                Instant::now() < deadline,
                "Public pause was not acknowledged"
            );
            std::thread::sleep(CONTROL_POLL_INTERVAL);
        };
        let before = control.snapshot().progress;
        let age = control.last_progress_age_ms();
        std::thread::sleep(PAUSED_OBSERVATION_INTERVAL);
        assert_eq!(
            serde_json::to_value(before).unwrap(),
            serde_json::to_value(control.snapshot().progress).unwrap()
        );
        if let (Some(before), Some(after)) = (age, control.last_progress_age_ms()) {
            assert!(after > before);
        }
        assert!(worker.progress(&timing).0);
        worker.cancel();
        let message = worker.receiver.recv_timeout(CONTROL_ACK_TIMEOUT).unwrap();
        let WorkerMessage::Profile(result) = message else {
            panic!("Cancellation must preserve the actual partial public report");
        };
        assert_eq!(result.job.status, ChannelJobStatus::Cancelled);
        assert_eq!(
            result.backend().report().summary.comparison.bits_tested,
            result.job.summary.comparison.bits_tested
        );
        assert_eq!(control.snapshot().state, FullLoopState::Cancelled);
        let WorkerMessage::Finished(reason) =
            worker.receiver.recv_timeout(CONTROL_ACK_TIMEOUT).unwrap()
        else {
            panic!("Cancelled single-profile worker must finish without another run");
        };
        assert!(reason.contains("cancelled"));
        assert_eq!(worker.progress(&timing).1, 0);
    }
}
