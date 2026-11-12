/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Frozen request admission and one bounded closed-loop worker.
use super::run_timing::RunTiming;
use crate::help::{AdmissionError, Diagnostic};
use crate::workbench::session::Inputs;
use docsis_ofdm_fec::{
    scenario::document::ScenarioDocument,
    signal_processing::receiver::channel_estimation::ReceiverMode,
    simulation::{
        channel::{
            job::run_channel_controlled,
            measurement::{captured_layout, MeasurementSelection},
            ChannelJobLimits, ChannelJobResult, ChannelRunSettings,
        },
        test_points::{
            ProbeConfig, ProbeData, ProbeEvent, ProbeObserver, ProbeSelection, RunPhase,
            TestPointId, TestPointProbe,
        },
    },
    transmission::TransmissionSettings,
    types::{IqPair, ProtocolScope},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    cell::RefCell,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc, Arc,
    },
    time::Duration,
};

/// Upper bound of the editable per-profile observation duration, in milliseconds.
pub(crate) const MAX_DURATION_MS: u64 = 1000;

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Options {
    pub receiver_mode: ReceiverMode,
    #[serde(default = "legacy_all_profiles")]
    pub all_profiles: bool,
    pub selected_profile_ids: Option<Vec<u8>>,
    pub duration_ms: u64,
    pub max_iterations: usize,
    pub max_runtime_ms: u64,
    pub noise_seed: u64,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            receiver_mode: ReceiverMode::SuppliedChannel,
            all_profiles: false,
            selected_profile_ids: None,
            duration_ms: 3,
            max_iterations: 50,
            max_runtime_ms: 300_000,
            noise_seed: 41,
        }
    }
}
fn legacy_all_profiles() -> bool {
    true
}
impl Options {
    /// Checks accepted limits and selected captured identities without requiring all inputs.
    /// # Arguments
    /// * `inputs` - Current accepted profile catalog; empty before DPD import.
    /// # Errors
    /// Rejects invalid limits, repeated or absent captured IDs and excessive selections.
    pub(crate) fn validate(&self, inputs: &Inputs) -> Result<(), String> {
        if !(1..=MAX_DURATION_MS).contains(&self.duration_ms)
            || !(1..=100).contains(&self.max_iterations)
            || !(1..=300000).contains(&self.max_runtime_ms)
        {
            return Err("Invalid workbench run limits.".into());
        }
        if let Some(ids) = &self.selected_profile_ids {
            let mut unique = std::collections::BTreeSet::new();
            for id in ids {
                if *id > 15 || !unique.insert(*id) {
                    return Err("Choose unique captured profile IDs from 0 to 15.".into());
                }
                if !inputs
                    .profiles
                    .iter()
                    .chain(inputs.profile.iter())
                    .any(|profile| profile.profile_id == *id)
                {
                    return Err(format!(
                        "Selected profile {id} is not present in the accepted DPD."
                    ));
                }
            }
        }
        Ok(())
    }
}
#[derive(Default)]
pub(crate) struct Samples {
    pub values: Vec<IqPair>,
    next: Option<u64>,
}
#[derive(Default)]
pub(crate) struct Observations {
    pub tx: crate::transmitter_instruments::capture::Capture,
    pub taps: [Samples; 4],
    pub responses: Vec<(u64, Vec<IqPair>)>,
    pub decoder_llrs: Vec<f64>,
    pub decoder_word: Option<u64>,
}
impl ProbeObserver for Observations {
    /// Retains finite contiguous time windows and one raw FFT without changing science.
    /// # Arguments
    /// * `event` - Selected immutable stage values; acquisition/drain are not retained.
    /// # Returns
    /// Unit; full snapshots stop copying while scientific processing continues.
    /// # Errors
    /// No adapter errors are introduced by exhausted retention.
    fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), docsis_ofdm_fec::SimulationError> {
        self.tx.observe(event);
        if event.context.phase != RunPhase::Observation {
            return Ok(());
        }
        if event.stage == TestPointId::RxChannelEstimate {
            if let ProbeData::Iq(values) = event.data {
                if self.responses.len() < 4 {
                    self.responses.push((
                        event.context.channel_symbol_index.unwrap_or(0),
                        values.to_vec(),
                    ));
                }
            }
            return Ok(());
        }
        if event.stage == TestPointId::PayloadLdpcInput {
            if let ProbeData::Llrs(values) = event.data {
                if self.decoder_llrs.is_empty() {
                    self.decoder_llrs = values.iter().copied().take(16200).collect();
                    self.decoder_word = event.context.codeword_index;
                }
            }
            return Ok(());
        }
        let index = match event.stage {
            TestPointId::ChannelOutput => 0,
            TestPointId::RxFramed => 1,
            TestPointId::RxUseful => 2,
            TestPointId::RxFft => 3,
            _ => return Ok(()),
        };
        let ProbeData::Iq(values) = event.data else {
            return Ok(());
        };
        let target = &mut self.taps[index];
        if index >= 2 && !target.values.is_empty() {
            return Ok(());
        }
        if target.next.is_some() && target.next != event.context.sample_start_index {
            return Ok(());
        }
        let length = values.len().min(32768 - target.values.len());
        target.values.extend_from_slice(&values[..length]);
        target.next = event
            .context
            .sample_start_index
            .map(|start| start + values.len() as u64);
        Ok(())
    }
}
pub(crate) struct ResultData {
    pub job: ChannelJobResult,
    pub observations: Observations,
}
thread_local! { static RESULT: RefCell<Option<Arc<ResultData>>> = const { RefCell::new(None) }; }
/// Borrows the immutable current result through a shared owner on the GUI thread.
/// # Returns
/// None before completion or after accepted input changes.
pub(crate) fn latest() -> Option<Arc<ResultData>> {
    RESULT.with(|result| result.borrow().clone())
}
fn publish(result: Option<Arc<ResultData>>) {
    RESULT.with(|stored| *stored.borrow_mut() = result);
}

enum WorkerMessage {
    Profile(
        Box<ResultData>,
        Box<docsis_ofdm_fec::profile::candidate::CapturedProfile>,
    ),
    Rejected(u8, u8, String),
    Finished(String),
}
#[derive(Clone)]
pub(crate) struct SweepPlan {
    transmission: TransmissionSettings,
    inputs: Inputs,
    profiles: Vec<docsis_ofdm_fec::profile::candidate::CapturedProfile>,
}
/// Validates every profile before admission without retaining duplicate channel frames.
/// # Arguments
/// * `transmission`, `inputs`, `options` - Accepted immutable run configuration.
/// # Returns
/// A finite sweep plan with common source/channel settings and sorted profile IDs.
/// # Errors
/// Rejects any incompatible profile or a sweep exceeding 32 profiles/262144 assignments.
pub(crate) fn prepare_sweep(
    transmission: &TransmissionSettings,
    inputs: &Inputs,
    options: &Options,
) -> Result<SweepPlan, AdmissionError> {
    options.validate(inputs)?;
    let catalog = if inputs.profiles.is_empty() {
        inputs.profile.iter().cloned().collect::<Vec<_>>()
    } else {
        inputs.profiles.clone()
    };
    let mut profiles = if let Some(ids) = &options.selected_profile_ids {
        catalog
            .into_iter()
            .filter(|profile| ids.contains(&profile.profile_id))
            .collect()
    } else if options.all_profiles {
        catalog
    } else {
        inputs.profile.iter().cloned().collect()
    };
    profiles.sort_by_key(|p| p.profile_id);
    if profiles
        .windows(2)
        .any(|pair| pair[0].profile_id == pair[1].profile_id)
    {
        return Err("Duplicate captured profile identity.".into());
    }
    if profiles.is_empty()
        || profiles.len() > crate::instruments::receiver::performance::MAX_PROFILES
        || profiles.iter().map(|p| p.assignments.len()).sum::<usize>() > 262144
    {
        return Err("Choose 1–32 profiles with at most 262144 assignments for a sweep.".into());
    }
    let mut candidate = inputs.clone();
    for profile in &profiles {
        candidate.profile = Some(profile.clone());
        prepare(transmission, &candidate, options)
            .map_err(|error| error.with_profile(profile.profile_id))?;
    }
    Ok(SweepPlan {
        transmission: transmission.clone(),
        inputs: inputs.clone(),
        profiles,
    })
}
fn run_one(
    settings: &ChannelRunSettings,
    runtime_ms: u64,
    stop: &AtomicBool,
    gate: &AtomicBool,
    progress: &AtomicU64,
    timing: &RunTiming,
) -> Result<ResultData, String> {
    let mut stages = vec![
        TestPointId::ChannelOutput,
        TestPointId::RxFramed,
        TestPointId::RxUseful,
        TestPointId::RxFft,
        TestPointId::RxChannelEstimate,
        TestPointId::PayloadLdpcInput,
    ];
    stages.extend(crate::transmitter_instruments::capture::STAGES);
    let mut probe = TestPointProbe::new(ProbeConfig {
        selections: stages
            .into_iter()
            .map(|stage| ProbeSelection {
                stage,
                elements: 0..16384,
                source_symbols: None,
                channel_symbols: None,
            })
            .collect(),
        max_events: 10000,
        max_elements_per_event: 16384,
        max_total_elements: 1_048_576,
    })
    .map_err(|e| e.to_string())?;
    probe.set_phase_filter(Some(RunPhase::Observation));
    for stage in crate::transmitter_instruments::capture::STAGES {
        probe
            .limit_stage_events(
                stage,
                if stage == TestPointId::PlcMessage {
                    64
                } else if stage == TestPointId::TxOverlap {
                    4
                } else {
                    1
                },
            )
            .map_err(|e| e.to_string())?;
    }
    probe
        .set_stage_phase_filter(TestPointId::PlcMessage, None)
        .map_err(|e| e.to_string())?;
    probe
        .set_stage_phase_filter(TestPointId::PlcIq, None)
        .map_err(|e| e.to_string())?;
    probe
        .set_stage_phase_filter(TestPointId::PlcPreamble, None)
        .map_err(|e| e.to_string())?;
    for (stage, events) in [
        (TestPointId::ChannelOutput, 12),
        (TestPointId::RxFramed, 4),
        (TestPointId::RxUseful, 1),
        (TestPointId::RxFft, 1),
        (TestPointId::RxChannelEstimate, 4),
        (TestPointId::PayloadLdpcInput, 1),
    ] {
        probe
            .limit_stage_events(stage, events)
            .map_err(|e| e.to_string())?;
    }
    let mut observations = Observations::default();
    let job = run_channel_controlled(
        settings,
        &ChannelJobLimits {
            max_runtime_ms: runtime_ms,
            max_retained_words: 64,
        },
        stop,
        gate,
        &mut probe,
        &mut observations,
        |p| {
            progress.store(p.completed_source_symbols, Ordering::Relaxed);
            timing.progress(p.phase, p.recovered_payloads);
        },
    )
    .map_err(|e| e.to_string())?;
    Ok(ResultData { job, observations })
}
struct Worker {
    cancel: Arc<AtomicBool>,
    paused: Arc<AtomicBool>,
    done: Arc<AtomicU64>,
    receiver: mpsc::Receiver<WorkerMessage>,
    profile_index: Arc<AtomicU64>,
    profile_id: Arc<AtomicU64>,
    profile_count: usize,
    stale: bool,
}
#[derive(Default)]
pub(crate) struct Model {
    pub options: Option<Options>,
    pub verify: bool,
    request: Option<ChannelRunSettings>,
    plan: Option<SweepPlan>,
    worker: Option<Worker>,
    timing: Option<Arc<RunTiming>>,
    error: String,
    diagnostic: Option<Box<Diagnostic>>,
    token: u64,
}
/// Prepares one complete supported source-to-compare request with shared core validation.
/// # Arguments
/// * `transmission` - Accepted OCD geometry.
/// * `inputs` - Accepted source/profile/measurement snapshots, never editor drafts.
/// * `options` - Finite receiver/run choices.
/// # Returns
/// Independent request with exact profile loading and continuous residual playback.
/// # Errors
/// Rejects missing, incompatible or unsupported settings without modifying accepted inputs.
pub(crate) fn prepare(
    transmission: &TransmissionSettings,
    inputs: &Inputs,
    options: &Options,
) -> Result<ChannelRunSettings, AdmissionError> {
    if !(1..=MAX_DURATION_MS).contains(&options.duration_ms) {
        return Err("UI duration must be 1–1000 ms.".into());
    }
    ChannelJobLimits {
        max_runtime_ms: options.max_runtime_ms,
        max_retained_words: 64,
    }
    .validate()
    .map_err(|e| e.to_string())?;
    let profile = inputs
        .profile
        .as_ref()
        .ok_or("Apply DPD before configuring Receiver.")?;
    let source = inputs
        .source
        .as_ref()
        .ok_or("Apply PRBS-TX source before configuring Receiver.")?;
    let playback = inputs
        .playback
        .as_ref()
        .ok_or("Apply RxMER before configuring Receiver.")?;
    let mut playback = playback.clone();
    playback.profile = profile.clone();
    let playback_span = playback
        .prepare(transmission)
        .map_err(|e| e.to_string())?
        .span();
    let scenario = ScenarioDocument {
        schema_version: 1,
        protocol: ProtocolScope::Docsis31,
        profiles: vec![profile.clone()],
        frames: playback.frames.clone(),
        channels: inputs.response.iter().cloned().collect(),
    };
    let mut settings = ChannelRunSettings::new(
        transmission.clone(),
        captured_layout(profile, transmission).map_err(|e| e.to_string())?,
    );
    settings.source = source.config.clone();
    settings.prbs_seed = source.seed;
    settings.duration = Duration::from_millis(options.duration_ms);
    settings.max_iterations = options.max_iterations;
    settings.receiver_mode = options.receiver_mode;
    settings.impairments.additional_loss = inputs.additional_loss.clone();
    settings.impairments.power = inputs.power.clone();
    MeasurementSelection {
        use_rxmer_noise: true,
        transitions: vec![],
        frame_index: 0,
        profile_id: profile.profile_id,
        channel_index: inputs.response.as_ref().map(|_| 0),
        tolerance_seconds: playback.tolerance_seconds,
        noise_seed: options.noise_seed,
    }
    .apply(&scenario, &mut settings)
    .map_err(|e| e.to_string())?;
    settings.playback = Some(playback);
    settings.constellation = Some(
        docsis_ofdm_fec::simulation::channel::constellation::ConstellationSelection {
            requested_pairs: 65536,
            capture_ncp: true,
            ..Default::default()
        },
    );
    settings.protected_constellation = Some(
        docsis_ofdm_fec::simulation::channel::protected_constellation::ProtectedCaptureSelection {
            pairs_per_stream: 8192,
            ..Default::default()
        },
    );
    settings
        .validate()
        .map_err(|error| AdmissionError::from_run(error, &settings, playback_span))?;
    Ok(settings)
}
impl Model {
    /// Cancels active work and invalidates observations after an accepted configuration change.
    pub(crate) fn invalidate(&mut self) {
        if let Some(worker) = &mut self.worker {
            worker.cancel.store(true, Ordering::Relaxed);
            worker.stale = true;
        }
        self.request = None;
        self.plan = None;
        self.timing = None;
        self.error.clear();
        self.diagnostic = None;
        publish(None);
        crate::instruments::receiver::performance::clear();
        self.token = self.token.wrapping_add(1);
    }
    /// Validates current accepted inputs against the saved receiver configuration.
    /// # Arguments
    /// * `settings` - Current OCD document.
    pub(crate) fn configure(&mut self, settings: &str) {
        self.invalidate();
        self.prepare_inputs(settings, &crate::workbench::session::snapshot());
    }
    /// Prepares requests from independent inputs without publishing GUI results.
    /// # Arguments
    /// * `settings` - Accepted OCD JSON.
    /// * `inputs` - Frozen snapshot owned by the caller.
    /// # Returns
    /// Readiness is stored in this idle model; no simulation is started.
    pub(crate) fn prepare_inputs(&mut self, settings: &str, inputs: &Inputs) {
        if let Some(options) = &self.options {
            let request = serde_json::from_str::<TransmissionSettings>(settings)
                .map_err(|e| AdmissionError::from(e.to_string()))
                .and_then(|settings| {
                    let plan = prepare_sweep(&settings, inputs, options)?;
                    let mut selected_inputs = inputs.clone();
                    selected_inputs.profile = plan.profiles.first().cloned();
                    let request = prepare(&settings, &selected_inputs, options)?;
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
        publish(None);
        self.token = self.token.wrapping_add(1);
        let options = self.options.clone().unwrap_or_default();
        let cancel = Arc::new(AtomicBool::new(false));
        let paused = Arc::new(AtomicBool::new(false));
        let done = Arc::new(AtomicU64::new(0));
        crate::instruments::receiver::performance::clear();
        let plan = self.plan.clone();
        let profile_count = plan.as_ref().map_or(1, |p| p.profiles.len());
        let catalog = plan
            .as_ref()
            .map(|p| p.profiles.clone())
            .unwrap_or_else(|| {
                settings
                    .playback
                    .as_ref()
                    .map(|p| vec![p.profile.clone()])
                    .unwrap_or_default()
            });
        crate::instruments::receiver::performance::begin(&catalog);
        let profile_index = Arc::new(AtomicU64::new(0));
        let profile_id = Arc::new(AtomicU64::new(0));
        let (ordinal, identity) = (profile_index.clone(), profile_id.clone());
        let (sender, receiver) = mpsc::sync_channel(1);
        let (stop, gate, progress) = (cancel.clone(), paused.clone(), done.clone());
        let timing = Arc::new(RunTiming::new());
        let worker_timing = timing.clone();
        let spawned = std::thread::Builder::new().name("merlin-profile-sweep".into()).spawn(move || {
            let mut elapsed=0_u64;
            for index in 0..profile_count {
                if stop.load(Ordering::Relaxed) || elapsed>=options.max_runtime_ms {break;}
                worker_timing.begin_profile();
                progress.store(0,Ordering::Relaxed);ordinal.store(index as u64,Ordering::Relaxed);
                let prepared=if let Some(plan)=&plan {
                    let profile=plan.profiles[index].clone();
                    let mut inputs=plan.inputs.clone();inputs.profile=Some(profile.clone());
                    (profile,prepare(&plan.transmission,&inputs,&options))
                } else {
                    let Some(profile)=settings.playback.as_ref().map(|p|p.profile.clone()) else {break;};
                    (profile,Ok(settings.clone()))
                };
                let (profile,prepared)=prepared;identity.store(u64::from(profile.profile_id),Ordering::Relaxed);
                let outcome = prepared.map_err(|error|error.to_string()).and_then(|request|run_one(&request,options.max_runtime_ms-elapsed,&stop,&gate,&progress,&worker_timing));
                worker_timing.end_profile(profile.profile_id);
                match outcome {
                    Ok(result)=>{
                        elapsed=elapsed.saturating_add(result.job.elapsed_ms);
                        let halt=matches!(result.job.status,docsis_ofdm_fec::simulation::channel::ChannelJobStatus::Cancelled|docsis_ofdm_fec::simulation::channel::ChannelJobStatus::RuntimeExceeded);
                        if sender.send(WorkerMessage::Profile(Box::new(result),Box::new(profile))).is_err() {worker_timing.finish();return;}
                        if halt {break;}
                    }
                    Err(error)=>{if sender.send(WorkerMessage::Rejected(profile.profile_id,profile.key.channel_id,error)).is_err() {worker_timing.finish();return;}}
                }
            }
            let reason=if stop.load(Ordering::Relaxed) {"Profile sweep cancelled; unstarted profiles remain marked Not Started."}
                else if elapsed>=options.max_runtime_ms {"Profile sweep active runtime budget reached."} else {""};
            worker_timing.finish();
            let _=sender.send(WorkerMessage::Finished(reason.into()));
        });
        match spawned {
            Ok(_) => {
                self.timing = Some(timing);
                self.worker = Some(Worker {
                    cancel,
                    paused,
                    done,
                    receiver,
                    profile_index,
                    profile_id,
                    profile_count,
                    stale: false,
                })
            }
            Err(error) => self.error = error.to_string(),
        }
    }
    /// Requests cooperative Stop; completed comparisons remain available when the worker finishes.
    pub(crate) fn stop(&self) {
        if let Some(worker) = &self.worker {
            worker.cancel.store(true, Ordering::Relaxed);
        }
    }
    /// Toggles pause without replacing source, RNG, interleaver or receiver state.
    pub(crate) fn pause(&self) {
        if let Some(worker) = &self.worker {
            worker.paused.fetch_xor(true, Ordering::Relaxed);
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
                Ok(WorkerMessage::Profile(result, profile)) => {
                    if !self.worker.as_ref().is_some_and(|w| w.stale) {
                        crate::instruments::receiver::performance::retain(&profile, &result);
                        self.error = result
                            .job
                            .error
                            .as_ref()
                            .map(ToString::to_string)
                            .unwrap_or_default();
                        publish(Some(Arc::new(*result)));
                        self.token = self.token.wrapping_add(1);
                    }
                }
                Ok(WorkerMessage::Rejected(id, channel, error)) => {
                    if !self.worker.as_ref().is_some_and(|w| w.stale) {
                        crate::instruments::receiver::performance::reject(
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
        let (profile_ids, applied_profile_id) = crate::workbench::session::profile_catalog();
        let ready = self.request.is_some();
        let result = latest();
        let summary=result.as_ref().map(|r|json!({"profile_id":r.job.settings.playback.as_ref().map(|p|p.profile.profile_id),"fec":r.job.summary.window.as_ref().map(|w|w.fec.snapshot()),"status":r.job.status,"elapsed_ms":r.job.elapsed_ms,
            "bits_tested":r.job.summary.comparison.bits_tested,"bit_errors":r.job.summary.comparison.bit_errors,
            "ber":r.job.summary.payload_ber,"recovered_payloads":r.job.summary.recovered_payloads,
            "ldpc_failures":r.job.summary.payload_ldpc_failures,"bch_failures":r.job.summary.payload_bch_failures,
            "mean_iterations":r.job.summary.payload_mean_ldpc_iterations,"ncp_fields":r.job.summary.accepted_ncp_fields,
            "plc_frames":r.job.summary.plc_frames,"unrecovered":r.job.unrecovered_generated_payloads}));
        json!({"receiver_status":if ready {"complete"} else {"attention"},"verify_status":if self.verify {"complete"} else {"attention"},
            "reason":if !self.error.is_empty(){self.error.as_str()} else if ready {"Validated source/channel/receiver configuration ready."} else {"Configure WB and Receiver after accepting OCD, DPD, PRBS-TX and RxMER."},
            "diagnostic":self.diagnostic,"timing":self.timing.as_ref().map(|timing|timing.snapshot()),
            "busy":self.worker.is_some(),"paused":self.worker.as_ref().is_some_and(|w|w.paused.load(Ordering::Relaxed)),
            "profile_index":self.worker.as_ref().map_or(0,|w|w.profile_index.load(Ordering::Relaxed)+1),
            "profile_id":self.worker.as_ref().map(|w|w.profile_id.load(Ordering::Relaxed)),
            "profile_count":self.worker.as_ref().map_or(0,|w|w.profile_count),
            "completed_symbols":self.worker.as_ref().map_or(0,|w|w.done.load(Ordering::Relaxed)),
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
mod tests {
    use super::*;
    fn accepted_inputs() -> (Inputs, TransmissionSettings) {
        use docsis_ofdm_fec::scenario::pnm_json::{import_profiles, import_rxmer, JsonCapture};
        use docsis_ofdm_fec::simulation::channel::measurement_playback::{
            PlaybackConfiguration, PlaybackMode,
        };
        use docsis_ofdm_fec::simulation::source::PayloadSourceConfig;
        let profile = import_profiles(JsonCapture {
            name: "dpd",
            bytes: include_bytes!(
                "../../../../examples/measurements/single-channel/modulation_profile.json"
            ),
        })
        .unwrap()
        .into_iter()
        .find(|profile| profile.profile_id == 0)
        .unwrap();
        let frame = import_rxmer(JsonCapture {
            name: "rxmer",
            bytes: include_bytes!("../../../../examples/measurements/single-channel/rxmer.json"),
        })
        .unwrap();
        let playback = PlaybackConfiguration {
            profile: profile.clone(),
            frames: vec![frame],
            mode: PlaybackMode::Single,
            interval_symbols: 1000,
            tolerance_seconds: 90000,
        };
        let inputs = Inputs {
            source: Some(crate::workbench::session::AcceptedSource {
                config: PayloadSourceConfig::Prbs15,
                seed: 32767,
            }),
            profiles: vec![profile.clone()],
            profile: Some(profile),
            playback: Some(playback),
            response: None,
            additional_loss: Default::default(),
            power: docsis_ofdm_fec::signal_processing::channel::power::PowerConditioning {
                amplifier_gain_db: 6.0,
                attenuation_db: 6.0,
                input_data_level_dbmv_per_6mhz: Some(0.0),
                ..Default::default()
            },
        };
        let transmission = TransmissionSettings::default();
        (inputs, transmission)
    }
    #[test]
    fn accepted_measurements_run_real_closed_loop_and_invalidate_views() {
        let (inputs, transmission) = accepted_inputs();
        let options = Options {
            duration_ms: 1,
            ..Default::default()
        };
        let settings = prepare(&transmission, &inputs, &options).unwrap();
        let mut model = Model {
            options: Some(options),
            verify: true,
            request: Some(settings),
            plan: None,
            worker: None,
            timing: None,
            error: String::new(),
            diagnostic: None,
            token: 0,
        };
        model.start();
        let deadline = std::time::Instant::now() + Duration::from_secs(60);
        while model.poll()["busy"] == true {
            assert!(std::time::Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(10));
        }
        let timing = model.state()["timing"].clone();
        assert!(timing["elapsed_ms"].as_u64().unwrap() > 0);
        assert_eq!(
            model.state()["timing"],
            timing,
            "Completed elapsed time must be frozen"
        );
        assert_eq!(timing["profiles"].as_array().unwrap().len(), 1);
        let result = latest().expect("Actual result must be retained");
        assert!(
            result.job.summary.comparison.bits_tested > 0,
            "{}",
            model.state()
        );
        assert_eq!(result.job.summary.comparison.bit_errors, 0);
        let source = crate::transmitter_instruments::stage(0, 0);
        assert_eq!(source["available"], true, "{source}");
        assert_eq!(source["rows"][0]["value"].as_str().unwrap().len(), 32);
        let coded = crate::transmitter_instruments::stage(1, 0);
        assert_eq!(coded["total"], 16200);
        let page = crate::transmitter_instruments::stage(1, 2048);
        assert_eq!(page["rows"][0]["index"], 2048);
        assert_eq!(
            crate::transmitter_instruments::stage(1, -1)["available"],
            false
        );
        assert_eq!(
            crate::transmitter_instruments::stage(20, 0)["available"],
            false
        );
        let mapped = crate::transmitter_instruments::stage(4, 0);
        assert_eq!(mapped["available"], true);
        assert!(!mapped["points"].as_array().unwrap().is_empty());
        assert!(mapped["points"].as_array().unwrap().len() <= 2048);
        let bins = crate::transmitter_instruments::stage(6, 0);
        assert_eq!(bins["total"], 8192);
        assert_eq!(source["graph"]["kind"], "bits");
        assert_eq!(coded["graph"]["kind"], "raster");
        assert_eq!(coded["graph"]["points"].as_array().unwrap().len(), 2048);
        assert_eq!(page["graph"]["points"][0]["index"], 2048);
        assert_eq!(mapped["graph"]["kind"], "constellation");
        assert_eq!(bins["graph"]["kind"], "physical");
        assert!(bins["graph"]["points"].as_array().unwrap().len() <= 2048);
        let previews = crate::transmitter_instruments::stage_previews();
        assert_eq!(previews.as_array().unwrap().len(), 6);
        assert_eq!(previews[0]["observation"]["graph"]["kind"], "bits");
        assert!(previews.to_string().len() < 1_500_000);
        assert!(previews[0]["observation"]["rows"].is_null());
        let input = crate::transmitter_instruments::channel_input::qam(-1, -1);
        assert_eq!(input["available"], true, "{input}");
        let point = &input["points"][0];
        let bin = point["index"].as_i64().unwrap() as i32;
        let selected = crate::transmitter_instruments::channel_input::qam(bin, bin);
        assert_eq!(selected["points"].as_array().unwrap().len(), 1);
        assert_eq!(selected["points"][0]["i"], point["i"]);
        assert!(crate::transmitter_instruments::channel_input::qam(4, 3)["available"] == false);
        let before = crate::instruments::channel::spectrum(0, 0, 0, false);
        let after = crate::instruments::channel::spectrum(1, 0, 0, false);
        let zoom = crate::instruments::channel::spectrum(1, 100, 32, true);
        assert!(before["channel_power"]["dbmv"].is_number(), "{before}");
        assert!(
            (after["channel_power"]["dbmv"].as_f64().unwrap()
                - before["channel_power"]["dbmv"].as_f64().unwrap()
                - 6.0)
                .abs()
                < 1e-8
        );
        assert_eq!(after["channel_power"], zoom["channel_power"]);
        let pre = crate::instruments::channel::qam(0, "{}");
        let post = crate::instruments::channel::qam(1, "{}");
        assert_eq!(
            pre["points"].as_array().unwrap().len(),
            post["points"].as_array().unwrap().len()
        );
        let gain = 10.0_f64.powf(6.0 / 20.0);
        assert!(
            (post["points"][0]["i"].as_f64().unwrap()
                - pre["points"][0]["i"].as_f64().unwrap() * gain)
                .abs()
                < 1e-10
        );

        let messages = crate::transmitter_instruments::messages();
        let packets = messages["messages"].as_array().unwrap();
        assert!(packets.iter().any(|p| p["type"] == 49), "{messages}");
        assert!(packets.iter().any(|p| p["profile"] == 0), "{messages}");
        assert!(packets.iter().any(|p| p["profile"] == 255), "{messages}");
        for packet in packets {
            assert_eq!(packet["channel"], 34);
            assert_eq!(packet["completed_frame"], true);
            let bytes: Vec<u8> = packet["hex_rows"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|row| {
                    row["hex"]
                        .as_str()
                        .unwrap()
                        .split_whitespace()
                        .map(|b| u8::from_str_radix(b, 16).unwrap())
                })
                .collect();
            let decoded =
                docsis_ofdm_fec::transmission::plc::mac_packets::ManagementPacket::from_bytes(
                    &bytes,
                )
                .unwrap();
            let parsed = docsis_ofdm_fec::transmission::plc::descriptors::parse(&decoded).unwrap();
            assert_eq!(parsed.channel_id, 34);
            for tlv in packet["tlvs"].as_array().unwrap() {
                let offset = tlv["offset"].as_u64().unwrap() as usize;
                assert_eq!(bytes[offset], tlv["kind"].as_u64().unwrap() as u8);
                assert_eq!(bytes[offset + 1], tlv["length"].as_u64().unwrap() as u8);
            }
        }
        for tap in 0..4 {
            assert_eq!(
                crate::transmitter_instruments::spectrum_state(tap)["has_result"],
                true
            );
            let view = crate::transmitter_instruments::spectrum(tap, 0, 0, false);
            assert!(view.get("error").is_none(), "{view}");
            assert!(view["points"].as_array().unwrap().len() <= 2048);
        }
        assert!(!result.observations.taps[0].values.is_empty());
        assert!(result.job.constellation.as_ref().unwrap().collected_pairs > 0);
        let query = crate::instruments::receiver::selection::Query::default();
        let scatter = crate::instruments::receiver::selection::qam(&query);
        assert_eq!(scatter["available"], true);
        assert_eq!(
            scatter["raw_mer"]["result"]["sample_count"],
            scatter["selected"]
        );
        assert_eq!(
            scatter["equalized_mer"]["result"]["sample_count"],
            scatter["selected"]
        );
        let channel_qam = crate::instruments::channel::qam(2, "{}");
        assert_eq!(channel_qam["mer"], scatter["raw_mer"]);
        let control = result
            .job
            .constellation
            .as_ref()
            .unwrap()
            .ncp_pairs
            .first()
            .expect("Actual CRC-classified NCP");
        let ncp_query = crate::instruments::receiver::selection::Query {
            signal: 1,
            first_bin: i32::from(control.physical_bin),
            last_bin: i32::from(control.physical_bin),
            first_symbol: control.channel_symbol_index as i64,
            last_symbol: control.channel_symbol_index as i64,
            ..Default::default()
        };
        let scatter = crate::instruments::receiver::selection::qam(&ncp_query);
        assert_eq!(scatter["available"], true);
        assert_eq!(scatter["points"][0]["bin"], control.physical_bin);
        let empty_query = crate::instruments::receiver::selection::Query {
            first_bin: 8191,
            last_bin: 8191,
            ..Default::default()
        };
        assert_eq!(
            crate::instruments::receiver::selection::qam(&empty_query)["points"],
            serde_json::json!([])
        );
        let decisions = crate::instruments::receiver::diagnostics::slicer("{}", false).unwrap();
        assert!(!decisions["rows"].as_array().unwrap().is_empty());
        assert!(decisions["llr"]["count"].as_u64().unwrap() > 0);
        let coding = crate::instruments::receiver::diagnostics::slicer("{}", true).unwrap();
        assert_eq!(coding["llr"]["count"], 16200);
        assert_eq!(
            crate::instruments::receiver::diagnostics::response("{}").unwrap()["available"],
            true
        );
        assert_eq!(
            crate::instruments::receiver::diagnostics::response(r#"{"source":1}"#).unwrap()
                ["available"],
            false
        );
        assert_eq!(
            crate::instruments::receiver::diagnostics::fec()["compared_bits"],
            result.job.summary.comparison.bits_tested
        );
        let charts =
            crate::instruments::receiver::diagnostics::fec()["verification_charts"].clone();
        assert_eq!(
            charts["recovery"]["points"][0]["count"],
            result.job.summary.recovered_payloads.to_string()
        );
        assert!(charts["word_errors"]["points"].as_array().unwrap().len() <= 64);
        assert!(charts["word_errors"]["points"]
            .as_array()
            .unwrap()
            .iter()
            .all(|point| point["count"] == "0"));
        model.invalidate();
        assert!(model.state()["timing"].is_null());
        assert!(latest().is_none());
        assert_eq!(
            crate::transmitter_instruments::stage(0, 0)["available"],
            false
        );
        assert!(crate::transmitter_instruments::messages()["messages"]
            .as_array()
            .unwrap()
            .is_empty());
        assert_eq!(
            crate::transmitter_instruments::spectrum_state(3)["has_result"],
            false
        );
        assert_eq!(
            crate::instruments::receiver::selection::qam(&query)["available"],
            false
        );
    }
    #[test]
    fn independent_sweep_preserves_each_profile_and_legacy_run_options() {
        let (mut inputs, transmission) = accepted_inputs();
        inputs.profiles = docsis_ofdm_fec::scenario::pnm_json::import_profiles(
            docsis_ofdm_fec::scenario::pnm_json::JsonCapture {
                name: "profiles",
                bytes: include_bytes!(
                    "../../../../examples/measurements/single-channel/modulation_profile.json"
                ),
            },
        )
        .unwrap();
        // Narrow the decoded test grid explicitly; production captures remain unchanged.
        // This exercises every original profile ID/loading without a wide-channel decoder soak.
        for profile in &mut inputs.profiles {
            profile.assignments.truncate(1920);
            profile.key.grid.sample_count = 1920;
        }
        inputs.profile = Some(
            inputs
                .profiles
                .iter()
                .find(|p| p.profile_id == 0)
                .unwrap()
                .clone(),
        );
        let playback = inputs.playback.as_mut().unwrap();
        playback.profile = inputs.profile.clone().unwrap();
        for frame in &mut playback.frames {
            frame.key.grid.sample_count = 1920;
            frame.mer_db.truncate(1920);
        }
        let options = Options {
            all_profiles: true,
            duration_ms: 1,
            max_iterations: 5,
            ..Default::default()
        };
        let plan = prepare_sweep(&transmission, &inputs, &options).unwrap();
        assert_eq!(
            plan.profiles
                .iter()
                .map(|p| p.profile_id)
                .collect::<Vec<_>>(),
            vec![0, 1, 2, 3]
        );
        let mut model = Model {
            options: Some(options.clone()),
            verify: true,
            request: Some(prepare(&transmission, &inputs, &options).unwrap()),
            plan: Some(plan),
            worker: None,
            timing: None,
            error: String::new(),
            diagnostic: None,
            token: 0,
        };
        model.start();
        let deadline = std::time::Instant::now() + Duration::from_secs(180);
        while model.poll()["busy"] == true {
            assert!(std::time::Instant::now() < deadline, "{}", model.state());
            std::thread::sleep(Duration::from_millis(10));
        }
        let view = crate::instruments::receiver::performance::view(-1, 0, 0);
        assert_eq!(view["profiles"].as_array().unwrap().len(), 4, "{view}");
        assert_eq!(view["profile_id"], 3);
        let timing = model.state()["timing"].clone();
        assert_eq!(timing["profiles"].as_array().unwrap().len(), 4);
        assert_eq!(timing["profiles"][3]["profile_id"], 3);
        assert_eq!(model.state()["timing"], timing);
        for id in 0..4 {
            let view = crate::instruments::receiver::performance::view(id, 0, 0);
            assert_eq!(view["status"], "complete", "{view}");
            assert!(view["fec"]["total_cw"].as_u64().unwrap() > 0);
            assert_eq!(
                view["fec"]["total_cw"].as_u64().unwrap(),
                view["fec"]["clean_cw"].as_u64().unwrap()
                    + view["fec"]["ccw"].as_u64().unwrap()
                    + view["fec"]["ufec"].as_u64().unwrap()
            );
            let fec = crate::instruments::receiver::performance::fec_view(id);
            assert_eq!(fec["profile_id"], id);
            assert_eq!(fec["fec"], view["fec"]);
            assert!(fec["words"].as_array().unwrap().len() <= 64);
            assert!(fec["iterations"]["points"].as_array().unwrap().len() <= 101);
            let recovery_total: u64 = fec["recovery"]["segments"]
                .as_array()
                .unwrap()
                .iter()
                .map(|segment| segment["count"].as_u64().unwrap())
                .sum();
            assert_eq!(recovery_total, fec["generated"].as_u64().unwrap());
            assert!(view["samples"].as_u64().unwrap() > 0);
            assert!(
                view["series"][1]["points"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|point| point["minimum_y"] == 1.0),
                "Captured BPSK pilot reference must retain its 0 dB dip"
            );
            assert!(view["series"]
                .as_array()
                .unwrap()
                .iter()
                .all(|s| s["points"].as_array().unwrap().len() <= 1024));
            let zoom = crate::instruments::receiver::performance::view(id, 10, 4);
            assert_eq!(zoom["start"], 10);
            assert_eq!(zoom["count"], 4);
        }
        assert_eq!(
            crate::instruments::receiver::performance::fec_view(15)["available"],
            false
        );
        assert_eq!(inputs.profile.as_ref().unwrap().profile_id, 0);
        assert_eq!(
            latest()
                .unwrap()
                .job
                .settings
                .playback
                .as_ref()
                .unwrap()
                .profile
                .profile_id,
            3
        );
        assert!(
            prepare_sweep(
                &transmission,
                &inputs,
                &Options {
                    all_profiles: false,
                    ..options.clone()
                }
            )
            .unwrap()
            .profiles
            .len()
                == 1
        );
        let legacy: Options = serde_json::from_str(r#"{"duration_ms":1}"#).unwrap();
        assert!(legacy.all_profiles);
        let saved = serde_json::to_string(&Options {
            all_profiles: false,
            ..options
        })
        .unwrap();
        assert!(
            !serde_json::from_str::<Options>(&saved)
                .unwrap()
                .all_profiles
        );
        inputs.profiles[1].profile_id = 0;
        assert!(prepare_sweep(&transmission, &inputs, &legacy).is_err());
        model.invalidate();
        assert_eq!(
            crate::instruments::receiver::performance::view(-1, 0, 0)["available"],
            false
        );
    }
    #[test]
    fn selected_profiles_are_ordered_validated_and_persisted_without_running() {
        let (mut inputs, transmission) = accepted_inputs();
        let profile = inputs.profile.as_ref().unwrap().clone();
        inputs.profiles = (0..4)
            .map(|id| {
                let mut value = profile.clone();
                value.profile_id = id;
                value
            })
            .collect();
        let selected = Options {
            selected_profile_ids: Some(vec![3, 1]),
            ..Default::default()
        };
        let plan = prepare_sweep(&transmission, &inputs, &selected).unwrap();
        assert_eq!(
            plan.profiles
                .iter()
                .map(|profile| profile.profile_id)
                .collect::<Vec<_>>(),
            vec![1, 3]
        );
        let restored: Options =
            serde_json::from_slice(&serde_json::to_vec(&selected).unwrap()).unwrap();
        assert_eq!(restored.selected_profile_ids, Some(vec![3, 1]));
        for ids in [vec![], vec![1, 1], vec![9], vec![16]] {
            let options = Options {
                selected_profile_ids: Some(ids),
                ..Default::default()
            };
            assert!(prepare_sweep(&transmission, &inputs, &options).is_err());
        }
        let applied = prepare_sweep(&transmission, &inputs, &Options::default()).unwrap();
        assert_eq!(applied.profiles.len(), 1);
        let legacy: Options = serde_json::from_str(r#"{"duration_ms":3}"#).unwrap();
        assert_eq!(
            prepare_sweep(&transmission, &inputs, &legacy)
                .unwrap()
                .profiles
                .len(),
            4
        );
        let partial = Options {
            selected_profile_ids: Some(vec![]),
            ..Default::default()
        };
        partial.validate(&Inputs::default()).unwrap();
        assert!(prepare_sweep(&transmission, &Inputs::default(), &partial).is_err());
    }
}
