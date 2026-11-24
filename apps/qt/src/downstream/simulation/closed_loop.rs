/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Public-request admission and bounded downstream workbench lifecycle.
mod lifecycle;
mod observations;
mod request;
mod request_retention;
mod result_projection;
mod result_retention;
mod worker;
use super::run_timing::RunTiming;
use crate::downstream::help::{AdmissionError, Diagnostic};
use crate::downstream::workbench::session::{Context, Inputs};
use crate::shell::{
    ApplicationError, Direction, DispatchError, Retained, SessionError, SessionKey, WorkerEvent,
    WorkerStatus,
};
use merlin::downstream::{FullLoopReport, FullLoopResult, TransmissionSettings};
use request::Sections;
pub(crate) use request::{prepare, prepare_sweep, Options, MAX_DURATION_MS};
use request_retention::{SharedPlan, SharedRequest};
use serde_json::{json, Value};
use std::{
    ops::Deref,
    sync::{mpsc, Arc},
};
#[cfg(test)]
use worker::run_one;
use worker::{DownstreamControl, Worker, WorkerMessage};

/// Read-only compatibility projection of the authoritative public result.
pub(crate) struct JobView(Retained<FullLoopResult>);
impl Deref for JobView {
    type Target = FullLoopReport;
    /// Borrows the unchanged actual report; no mutable engine state is exposed.
    /// # Returns
    /// Public immutable report owned for the lifetime of this result.
    fn deref(&self) -> &Self::Target {
        self.0.value().report()
    }
}

pub(crate) struct ResultData {
    pub observations: observations::Observations,
    observation_error: Option<&'static str>,
    // Keep the backend/fixed ResultData reservation until all display fields drop.
    pub job: JobView,
}
impl ResultData {
    /// Preserves legacy already-completed scientific query fixtures only.
    ///
    /// # Arguments
    /// * `result` - Actual fixture output, admitted from its current metadata here.
    ///
    /// # Returns
    /// Query-compatible fixture owner; production reserves before executing instead.
    ///
    /// # Panics
    /// Panics if the small fixture's checked metadata or wrapper arithmetic fails.
    #[cfg(test)]
    pub(crate) fn new(result: FullLoopResult) -> Self {
        let budget = result_retention::fixture_budget();
        Self::from_retained(result_retention::retain_fixture(result, &budget), &budget)
    }

    /// Moves an already pre-admitted backend result into its fixed presentation owner.
    ///
    /// # Arguments
    /// * `result` - Actual complete/partial science with its pre-execution reservation.
    /// * `budget` - Same canonical policy for the source and all stored display copies.
    ///
    /// # Returns
    /// Immutable science and separately admitted observation/capture copies. Admission
    /// failure retains science with an empty projection and a borrowed static diagnostic.
    fn from_retained(
        result: Retained<FullLoopResult>,
        budget: &crate::shell::RetentionBudget,
    ) -> Self {
        let (observations, observation_error) = result_projection::admit(result.value(), budget);
        Self {
            observations,
            observation_error,
            job: JobView(result),
        }
    }

    /// Projects existing diagnostics into the bounded UI response without retaining a copy.
    ///
    /// # Returns
    /// Original warning strings, or one static admission diagnostic; science is unchanged.
    fn observation_warnings(&self) -> Value {
        match self.observation_error {
            Some(message) => json!([message]),
            None => json!(self.observations.warnings),
        }
    }

    /// Borrows the authoritative public result without reconstructing observations.
    /// # Returns
    /// Read-only data and public instrument operations for this result's lifetime.
    pub(crate) fn backend(&self) -> &FullLoopResult {
        self.job.0.value()
    }
}
/// Borrows the current result through a GUI-thread shared owner.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// # Returns
/// None before completion or after accepted input changes.
pub(crate) fn latest(context: &Context) -> Option<Arc<ResultData>> {
    context.result.with(|result| result.borrow().clone())
}
fn publish(context: &Context, result: Option<Arc<ResultData>>) {
    context.result.with(|stored| *stored.borrow_mut() = result);
}

pub(crate) struct Model {
    pub(crate) context: Context,
    pub options: Option<Options>,
    pub verify: bool,
    request: Option<SharedRequest>,
    plan: Option<SharedPlan>,
    worker: Option<Worker>,
    timing: Option<Arc<RunTiming>>,
    error: String,
    diagnostic: Option<Box<Diagnostic>>,
    token: u64,
    generation: Option<SessionKey>,
}

impl Default for Model {
    /// Creates the existing GUI model on its fixed bootstrap downstream Context.
    /// # Returns
    /// The same empty controls/request/result state as the original derived default.
    fn default() -> Self {
        Self::for_context(Context::default())
    }
}

impl Model {
    /// Creates an empty receiver model on an explicitly prepared downstream owner.
    /// # Arguments
    /// * `context` - Already constructed canonical-budget Stores; no selected-tab lookup.
    /// # Returns
    /// Original default fields without constructing another bootstrap host or run.
    pub(crate) fn for_context(context: Context) -> Self {
        Self {
            context,
            options: None,
            verify: false,
            request: None,
            plan: None,
            worker: None,
            timing: None,
            error: String::new(),
            diagnostic: None,
            token: Default::default(),
            generation: None,
        }
    }

    /// Starts one Rust job; inputs are frozen and repeated Start while busy is ignored.
    pub(crate) fn start(&mut self) {
        self.poll();
        if self.worker.is_some() || !self.verify {
            return;
        }
        let Some(settings) = self.request.clone() else {
            return;
        };
        let Some(key) = self.generation else {
            self.error = "Configure the receiver before starting this session.".into();
            return;
        };
        let plan = self.plan.clone();
        let catalog = if let Some(plan) = &plan {
            let profiles = (0..plan.value().info().selected_profile_ids.len())
                .map(|index| {
                    let request = plan
                        .value()
                        .request(index)
                        .map_err(|cause| cause.message().to_owned())?;
                    request
                        .settings()
                        .playback
                        .as_ref()
                        .map(|playback| playback.profile.clone())
                        .ok_or_else(|| {
                            "Public request has no accepted playback profile.".to_owned()
                        })
                })
                .collect::<Result<Vec<_>, String>>();
            match profiles {
                Ok(profiles) => profiles,
                Err(error) => {
                    self.error = error;
                    return;
                }
            }
        } else {
            settings
                .value()
                .settings()
                .playback
                .as_ref()
                .map(|playback| vec![playback.profile.clone()])
                .unwrap_or_default()
        };
        let control = Arc::new(DownstreamControl::new());
        let lease = match self.context.application.begin(key, control.clone()) {
            Ok(lease) => lease,
            Err(error) => {
                self.error = application_error(error);
                return;
            }
        };
        if let Err(error) =
            crate::downstream::instruments::receiver::performance::begin(&self.context, &catalog)
        {
            let _ = self.context.application.finish(lease);
            self.error = application_error(ApplicationError::Retention(error));
            return;
        }
        self.error.clear();
        self.diagnostic = None;
        self.timing = None;

        // Retain accepted science until a new identified Profile replaces it.
        self.token = self.token.wrapping_add(1);
        let timing = Arc::new(RunTiming::new());
        match Worker::spawn(
            settings,
            plan,
            timing.clone(),
            control,
            lease,
            self.context.application.budget().clone(),
        ) {
            Ok(worker) => {
                self.timing = Some(timing);
                self.worker = Some(worker);
            }
            Err(error) => {
                let (cause, lease) = error.into_parts();
                let _ = self.context.application.finish(lease);
                self.error = cause.to_string();
            }
        }
    }
    /// Requests cooperative Stop; completed comparisons remain available when the worker finishes.
    pub(crate) fn stop(&self) {
        if let Some(worker) = &self.worker {
            let _ = self.context.application.cancel(worker.token());
        }
    }
    /// Toggles pause without replacing source, RNG, interleaver or receiver state.
    pub(crate) fn pause(&self) {
        if let Some(worker) = &self.worker {
            worker.pause();
        }
    }
    /// Collects completion and reports bounded progress and real BER/FEC outcomes.
    /// # Returns
    /// Scalar JSON state; full retained samples remain behind Rust ownership.
    pub(crate) fn poll(&mut self) -> Value {
        if let Some(worker) = &self.worker {
            if let Some(paused) = worker.acknowledged_pause() {
                let _ = self
                    .context
                    .application
                    .acknowledge_pause(worker.token(), paused);
            }
        }
        if let Some(message) = self
            .worker
            .as_ref()
            .map(|worker| worker.receiver.try_recv())
        {
            match message {
                Ok(WorkerEvent::Message {
                    token,
                    value: WorkerMessage::Profile(result),
                }) => {
                    if self.accepts_callback(token) {
                        let retention_error = if let Some(playback) = &result.job.settings.playback
                        {
                            crate::downstream::instruments::receiver::performance::retain(
                                &self.context,
                                &playback.profile,
                                &result,
                            )
                            .err()
                        } else {
                            None
                        };
                        self.error = result
                            .backend()
                            .report()
                            .error
                            .as_ref()
                            .map(ToString::to_string)
                            .unwrap_or_else(|| {
                                retention_error
                                    .map(|error| {
                                        application_error(ApplicationError::Retention(error))
                                    })
                                    .unwrap_or_default()
                            });
                        self.diagnostic = result
                            .job
                            .error
                            .as_ref()
                            .and_then(|error| {
                                crate::downstream::help::catalog_diagnostic(error.code)
                            })
                            .map(Box::new);
                        publish(&self.context, Some(Arc::new(*result)));
                        self.token = self.token.wrapping_add(1);
                    }
                }
                Ok(WorkerEvent::Message {
                    token,
                    value: WorkerMessage::Rejected(id, channel, error),
                }) => {
                    if self.accepts_callback(token) {
                        if let Err(retention_error) =
                            crate::downstream::instruments::receiver::performance::reject(
                                &self.context,
                                id,
                                channel,
                                error.clone(),
                            )
                        {
                            log::warn!(
                                "Failed profile display unavailable: {}",
                                application_error(ApplicationError::Retention(retention_error))
                            );
                        }
                        self.error = error;
                        self.token = self.token.wrapping_add(1);
                    }
                }
                Ok(WorkerEvent::Message {
                    token,
                    value: WorkerMessage::Finished(reason),
                }) => {
                    if self.accepts_callback(token) && !reason.is_empty() {
                        self.error = reason;
                    }
                    // This is sweep status, not proof that its closure has returned.
                    self.token = self.token.wrapping_add(1);
                }
                Ok(WorkerEvent::Terminal(terminal)) => {
                    if self.accepts_callback(terminal.token()) {
                        match terminal.status() {
                            WorkerStatus::Completed => {}
                            WorkerStatus::Failed(error) => self.error = error.clone(),
                            WorkerStatus::Panicked => {
                                self.error = "Receiver worker stopped unexpectedly.".into()
                            }
                        }
                    }
                    if let Some(timing) = &self.timing {
                        timing.finish();
                    }
                    let _ = self.context.application.finish(terminal.into_lease());
                    self.worker = None;
                    self.token = self.token.wrapping_add(1);
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    if let Some(timing) = &self.timing {
                        timing.finish();
                    }
                    self.worker = None;
                    self.error = "Receiver worker disconnected.".into();
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        self.state()
    }
    /// Reports next-run readiness separately from outcomes of the previous run.
    /// # Returns
    /// Receiver/comparison configuration, busy state and bounded scientific summary.
    pub(crate) fn state(&self) -> Value {
        let (profile_ids, applied_profile_id) =
            crate::downstream::workbench::session::profile_catalog(&self.context);
        let progress = self
            .worker
            .as_ref()
            .zip(self.timing.as_ref())
            .map(|(worker, timing)| worker.progress(timing));
        let ready = self.request.is_some();
        let active = self.context.application.active();
        let current =
            self.generation == Some(self.context.application.snapshot(Direction::Downstream).key);
        let result = latest(&self.context);
        let summary=result.as_ref().map(|r|json!({"profile_id":r.job.settings.playback.as_ref().map(|p|p.profile.profile_id),"fec":r.job.summary.window.as_ref().map(|w|w.fec.snapshot()),"status":r.job.status,"elapsed_ms":r.job.elapsed_ms,
            "bits_tested":r.job.summary.comparison.bits_tested,"bit_errors":r.job.summary.comparison.bit_errors,
            "ber":r.job.summary.payload_ber,"recovered_payloads":r.job.summary.recovered_payloads,
            "ldpc_failures":r.job.summary.payload_ldpc_failures,"bch_failures":r.job.summary.payload_bch_failures,
            "mean_iterations":r.job.summary.payload_mean_ldpc_iterations,"ncp_fields":r.job.summary.accepted_ncp_fields,
            "plc_frames":r.job.summary.plc_frames,"unrecovered":r.job.unrecovered_generated_payloads,"observation_warnings":r.observation_warnings()}));
        json!({"receiver_status":if ready {"complete"} else {"attention"},"verify_status":if self.verify {"complete"} else {"attention"},
            "reason":if !self.error.is_empty(){self.error.as_str()} else if let Some(message) = result.as_ref().and_then(|result|result.observation_error) {message} else if ready {"Validated source/channel/receiver configuration ready."} else {"Configure WB and Receiver after accepting OCD, DPD, PRBS-TX and RxMER."},
            "diagnostic":self.diagnostic,"timing":self.timing.as_ref().map(|timing|timing.snapshot()),
            "busy":self.worker.is_some(),"paused":progress.is_some_and(|value| value.0),
            "profile_index":progress.map_or(0, |value| value.1 + 1),
            "profile_id":progress.map(|value| value.2),
            "profile_count":self.worker.as_ref().map_or(0,|w|w.profile_count),
            "completed_symbols":progress.map_or(0, |value| value.3),
            "can_start":ready&&self.verify&&self.worker.is_none()&&active.is_none()&&current,"has_result":result.is_some(),"result_token":self.token.to_string(),"result":summary,
            "application_busy":active.is_some(),"active_direction":active.map(|run| match run.token.key.direction {Direction::Downstream=>"downstream",Direction::Upstream=>"upstream"}),
            "available_profile_ids":profile_ids,"applied_profile_id":applied_profile_id,"options":self.options.clone().unwrap_or_default(),"accepted_options":self.options,"verified":self.verify})
    }
    fn accepts_callback(&self, token: crate::shell::RunToken) -> bool {
        self.worker
            .as_ref()
            .is_some_and(|worker| !worker.stale && worker.token() == token)
            && self.context.application.validate_callback(token).is_ok()
    }
}
fn application_error(error: ApplicationError) -> String {
    match error {
        ApplicationError::Session(SessionError::Dispatch(DispatchError::Busy(active))) => match active.token.key.direction {
            Direction::Downstream => "A downstream simulation is already active. Stop it before starting another.".into(),
            Direction::Upstream => "An upstream simulation is already active. Stop it before starting downstream.".into(),
        },
        ApplicationError::NotConfigured(_) | ApplicationError::Session(SessionError::StaleGuard) => "Configuration changed. Configure the receiver before starting this session.".into(),
        ApplicationError::Retention(_) => "The session retained-data limit would be exceeded.".into(),
        ApplicationError::Session(_) => "The session cannot accept this operation. Preserve the project and reopen the Workbench.".into(),
    }
}
impl Drop for Model {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "closed_loop/session_tests.rs"]
mod session_tests;
