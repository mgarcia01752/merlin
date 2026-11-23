/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Public-request admission and bounded downstream workbench lifecycle.
mod observations;
mod request;
mod worker;
use super::run_timing::RunTiming;
use crate::downstream::help::{AdmissionError, Diagnostic};
use crate::downstream::workbench::session::{Context, Inputs};
use merlin::downstream::{
    FullLoopReport, FullLoopRequest, FullLoopResult, FullLoopSweepPlan, TransmissionSettings,
};
use request::Sections;
pub(crate) use request::{prepare, prepare_sweep, Options, MAX_DURATION_MS};
use serde_json::{json, Value};
use std::{
    ops::Deref,
    sync::{mpsc, Arc},
};
#[cfg(test)]
use worker::run_one;
use worker::{Worker, WorkerMessage};

/// Read-only compatibility projection of the authoritative public result.
pub(crate) struct JobView(FullLoopResult);
impl Deref for JobView {
    type Target = FullLoopReport;
    /// Borrows the unchanged actual report; no mutable engine state is exposed.
    /// # Returns
    /// Public immutable report owned for the lifetime of this result.
    fn deref(&self) -> &Self::Target {
        self.0.report()
    }
}

pub(crate) struct ResultData {
    pub job: JobView,
    pub observations: observations::Observations,
}
impl ResultData {
    /// Retains the public result and bounded presentation copies together.
    /// # Arguments
    /// * `result` - Actual completed or partial high-level backend result.
    /// # Returns
    /// Immutable scientific ownership independent of future workbench runs.
    pub(crate) fn new(result: FullLoopResult) -> Self {
        let observations = observations::Observations::from_result(&result);
        Self {
            job: JobView(result),
            observations,
        }
    }
    /// Borrows the authoritative public result without reconstructing observations.
    /// # Returns
    /// Read-only data and public instrument operations for this result's lifetime.
    pub(crate) fn backend(&self) -> &FullLoopResult {
        &self.job.0
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

#[derive(Default)]
pub(crate) struct Model {
    pub(crate) context: Context,
    pub options: Option<Options>,
    pub verify: bool,
    request: Option<FullLoopRequest>,
    plan: Option<FullLoopSweepPlan>,
    worker: Option<Worker>,
    timing: Option<Arc<RunTiming>>,
    error: String,
    diagnostic: Option<Box<Diagnostic>>,
    token: u64,
}
impl Model {
    /// Cancels active work and invalidates observations after an accepted configuration change.
    pub(crate) fn invalidate(&mut self) {
        if let Some(worker) = &mut self.worker {
            worker.cancel();
            worker.stale = true;
        }
        self.request = None;
        self.plan = None;
        self.timing = None;
        self.error.clear();
        self.diagnostic = None;
        publish(&self.context, None);
        crate::downstream::instruments::receiver::performance::clear(&self.context);
        self.token = self.token.wrapping_add(1);
    }
    /// Validates current accepted inputs against the saved receiver configuration.
    /// # Arguments
    /// * `settings` - Current OCD document.
    pub(crate) fn configure(&mut self, settings: &str) {
        self.invalidate();
        self.prepare_inputs(
            settings,
            &crate::downstream::workbench::session::snapshot(&self.context),
            &crate::downstream::workbench::session::saved_sections(&self.context),
        );
    }
    /// Prepares requests from independent inputs without publishing GUI results.
    /// # Arguments
    /// * `settings` - Accepted OCD JSON.
    /// * `inputs` - Frozen snapshot owned by the caller.
    /// # Returns
    /// Readiness is stored in this idle model; no simulation is started.
    pub(crate) fn prepare_inputs(&mut self, settings: &str, inputs: &Inputs, sections: &Sections) {
        if let Some(options) = &self.options {
            let request = serde_json::from_str::<TransmissionSettings>(settings)
                .map_err(|e| AdmissionError::from(e.to_string()))
                .and_then(|settings| {
                    let plan = prepare_sweep(&settings, inputs, options, sections)?;
                    let request = plan.request(0).map_err(AdmissionError::classified)?;
                    Ok((plan, request))
                });
            match request {
                Ok((plan, request)) => {
                    self.request = Some(request);
                    self.plan = Some(plan);
                }
                Err(error) => {
                    self.error = error.message;
                    self.diagnostic = error.diagnostic;
                }
            }
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
        self.error.clear();
        self.diagnostic = None;
        self.timing = None;
        publish(&self.context, None);
        self.token = self.token.wrapping_add(1);
        crate::downstream::instruments::receiver::performance::clear(&self.context);
        let plan = self.plan.clone();
        let catalog = if let Some(plan) = &plan {
            let profiles = (0..plan.info().selected_profile_ids.len())
                .map(|index| {
                    let request = plan
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
                .settings()
                .playback
                .as_ref()
                .map(|playback| vec![playback.profile.clone()])
                .unwrap_or_default()
        };
        crate::downstream::instruments::receiver::performance::begin(&self.context, &catalog);
        let timing = Arc::new(RunTiming::new());
        match Worker::spawn(settings, plan, timing.clone()) {
            Ok(worker) => {
                self.timing = Some(timing);
                self.worker = Some(worker);
            }
            Err(error) => self.error = error.to_string(),
        }
    }
    /// Requests cooperative Stop; completed comparisons remain available when the worker finishes.
    pub(crate) fn stop(&self) {
        if let Some(worker) = &self.worker {
            worker.cancel();
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
        if let Some(message) = self
            .worker
            .as_ref()
            .map(|worker| worker.receiver.try_recv())
        {
            match message {
                Ok(WorkerMessage::Profile(result)) => {
                    if !self.worker.as_ref().is_some_and(|w| w.stale) {
                        if let Some(playback) = &result.job.settings.playback {
                            crate::downstream::instruments::receiver::performance::retain(
                                &self.context,
                                &playback.profile,
                                &result,
                            );
                        }
                        self.error = result
                            .backend()
                            .report()
                            .error
                            .as_ref()
                            .map(ToString::to_string)
                            .unwrap_or_default();
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
                Ok(WorkerMessage::Rejected(id, channel, error)) => {
                    if !self.worker.as_ref().is_some_and(|w| w.stale) {
                        crate::downstream::instruments::receiver::performance::reject(
                            &self.context,
                            id,
                            channel,
                            error.clone(),
                        );
                        self.error = error;
                        self.token = self.token.wrapping_add(1);
                    }
                }
                Ok(WorkerMessage::Finished(reason)) => {
                    if !self.worker.as_ref().is_some_and(|w| w.stale) && !reason.is_empty() {
                        self.error = reason;
                    }
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
        let result = latest(&self.context);
        let summary=result.as_ref().map(|r|json!({"profile_id":r.job.settings.playback.as_ref().map(|p|p.profile.profile_id),"fec":r.job.summary.window.as_ref().map(|w|w.fec.snapshot()),"status":r.job.status,"elapsed_ms":r.job.elapsed_ms,
            "bits_tested":r.job.summary.comparison.bits_tested,"bit_errors":r.job.summary.comparison.bit_errors,
            "ber":r.job.summary.payload_ber,"recovered_payloads":r.job.summary.recovered_payloads,
            "ldpc_failures":r.job.summary.payload_ldpc_failures,"bch_failures":r.job.summary.payload_bch_failures,
            "mean_iterations":r.job.summary.payload_mean_ldpc_iterations,"ncp_fields":r.job.summary.accepted_ncp_fields,
            "plc_frames":r.job.summary.plc_frames,"unrecovered":r.job.unrecovered_generated_payloads,"observation_warnings":r.observations.warnings}));
        json!({"receiver_status":if ready {"complete"} else {"attention"},"verify_status":if self.verify {"complete"} else {"attention"},
            "reason":if !self.error.is_empty(){self.error.as_str()} else if ready {"Validated source/channel/receiver configuration ready."} else {"Configure WB and Receiver after accepting OCD, DPD, PRBS-TX and RxMER."},
            "diagnostic":self.diagnostic,"timing":self.timing.as_ref().map(|timing|timing.snapshot()),
            "busy":self.worker.is_some(),"paused":progress.is_some_and(|value| value.0),
            "profile_index":progress.map_or(0, |value| value.1 + 1),
            "profile_id":progress.map(|value| value.2),
            "profile_count":self.worker.as_ref().map_or(0,|w|w.profile_count),
            "completed_symbols":progress.map_or(0, |value| value.3),
            "can_start":ready&&self.verify&&self.worker.is_none(),"has_result":result.is_some(),"result_token":self.token.to_string(),"result":summary,
            "available_profile_ids":profile_ids,"applied_profile_id":applied_profile_id,"options":self.options.clone().unwrap_or_default(),"accepted_options":self.options,"verified":self.verify})
    }
}
impl Drop for Model {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests;
