/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Frozen request admission and one bounded closed-loop worker.
use crate::workbench_session::Inputs;
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

#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Options {
    pub receiver_mode: ReceiverMode,
    pub duration_ms: u64,
    pub max_iterations: usize,
    pub max_runtime_ms: u64,
    pub noise_seed: u64,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            receiver_mode: ReceiverMode::SuppliedChannel,
            duration_ms: 3,
            max_iterations: 50,
            max_runtime_ms: 300_000,
            noise_seed: 41,
        }
    }
}
#[derive(Default)]
pub(crate) struct Samples {
    pub values: Vec<IqPair>,
    next: Option<u64>,
}
#[derive(Default)]
pub(crate) struct Observations {
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

struct Worker {
    cancel: Arc<AtomicBool>,
    paused: Arc<AtomicBool>,
    done: Arc<AtomicU64>,
    receiver: mpsc::Receiver<Result<ResultData, String>>,
    stale: bool,
}
#[derive(Default)]
pub(crate) struct Model {
    pub options: Option<Options>,
    pub verify: bool,
    request: Option<ChannelRunSettings>,
    worker: Option<Worker>,
    error: String,
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
) -> Result<ChannelRunSettings, String> {
    if !(1..=1000).contains(&options.duration_ms) {
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
    playback.prepare(transmission).map_err(|e| e.to_string())?;
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
    settings.validate().map_err(|e| e.to_string())?;
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
        self.error.clear();
        publish(None);
        self.token = self.token.wrapping_add(1);
    }
    /// Validates current accepted inputs against the saved receiver configuration.
    /// # Arguments
    /// * `settings` - Current OCD document.
    pub(crate) fn configure(&mut self, settings: &str) {
        self.invalidate();
        if let Some(options) = &self.options {
            let request = serde_json::from_str::<TransmissionSettings>(settings)
                .map_err(|e| e.to_string())
                .and_then(|settings| {
                    prepare(&settings, &crate::workbench_session::snapshot(), options)
                });
            match request {
                Ok(request) => self.request = Some(request),
                Err(error) => self.error = error,
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
        publish(None);
        self.token = self.token.wrapping_add(1);
        let options = self.options.clone().unwrap_or_default();
        let cancel = Arc::new(AtomicBool::new(false));
        let paused = Arc::new(AtomicBool::new(false));
        let done = Arc::new(AtomicU64::new(0));
        let (sender, receiver) = mpsc::sync_channel(1);
        let (stop, gate, progress) = (cancel.clone(), paused.clone(), done.clone());
        let spawned = std::thread::Builder::new()
            .name("merlin-closed-loop".into())
            .spawn(move || {
                let run = || -> Result<ResultData, String> {
                    let stages = [
                        TestPointId::ChannelOutput,
                        TestPointId::RxFramed,
                        TestPointId::RxUseful,
                        TestPointId::RxFft,
                        TestPointId::RxChannelEstimate,
                        TestPointId::PayloadLdpcInput,
                    ];
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
                        &settings,
                        &ChannelJobLimits {
                            max_runtime_ms: options.max_runtime_ms,
                            max_retained_words: 64,
                        },
                        &stop,
                        &gate,
                        &mut probe,
                        &mut observations,
                        |p| {
                            progress.store(p.completed_source_symbols, Ordering::Relaxed);
                        },
                    )
                    .map_err(|e| e.to_string())?;
                    Ok(ResultData { job, observations })
                };
                let _ = sender.send(run());
            });
        match spawned {
            Ok(_) => {
                self.worker = Some(Worker {
                    cancel,
                    paused,
                    done,
                    receiver,
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
                Ok(result) => {
                    let worker = self.worker.take().expect("Tracked worker");
                    if !worker.stale {
                        match result {
                            Ok(result) => {
                                self.error = result
                                    .job
                                    .error
                                    .as_ref()
                                    .map(|e| e.to_string())
                                    .unwrap_or_default();
                                publish(Some(Arc::new(result)));
                            }
                            Err(error) => self.error = error,
                        }
                        self.token = self.token.wrapping_add(1);
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
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
        let ready = self.request.is_some();
        let result = latest();
        let summary=result.as_ref().map(|r|json!({"status":r.job.status,"elapsed_ms":r.job.elapsed_ms,
            "bits_tested":r.job.summary.comparison.bits_tested,"bit_errors":r.job.summary.comparison.bit_errors,
            "ber":r.job.summary.payload_ber,"recovered_payloads":r.job.summary.recovered_payloads,
            "ldpc_failures":r.job.summary.payload_ldpc_failures,"bch_failures":r.job.summary.payload_bch_failures,
            "mean_iterations":r.job.summary.payload_mean_ldpc_iterations,"ncp_fields":r.job.summary.accepted_ncp_fields,
            "plc_frames":r.job.summary.plc_frames,"unrecovered":r.job.unrecovered_generated_payloads}));
        json!({"receiver_status":if ready {"complete"} else {"attention"},"verify_status":if self.verify {"complete"} else {"attention"},
            "reason":if !self.error.is_empty(){self.error.as_str()} else if ready {"Validated source/channel/receiver configuration ready."} else {"Configure Receiver after accepting OCD, DPD, PRBS-TX and RxMER."},
            "busy":self.worker.is_some(),"paused":self.worker.as_ref().is_some_and(|w|w.paused.load(Ordering::Relaxed)),
            "completed_symbols":self.worker.as_ref().map_or(0,|w|w.done.load(Ordering::Relaxed)),
            "can_start":ready&&self.verify&&self.worker.is_none(),"has_result":result.is_some(),"result_token":self.token.to_string(),"result":summary,
            "options":self.options.clone().unwrap_or_default(),"accepted_options":self.options,"verified":self.verify})
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
    #[test]
    fn accepted_measurements_run_real_closed_loop_and_invalidate_views() {
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
            source: Some(crate::workbench_session::AcceptedSource {
                config: PayloadSourceConfig::Prbs15,
                seed: 32767,
            }),
            profile: Some(profile),
            playback: Some(playback),
            response: None,
        };
        let transmission = TransmissionSettings::default();
        let options = Options {
            duration_ms: 1,
            ..Default::default()
        };
        let settings = prepare(&transmission, &inputs, &options).unwrap();
        let mut model = Model {
            options: Some(options),
            verify: true,
            request: Some(settings),
            worker: None,
            error: String::new(),
            token: 0,
        };
        model.start();
        let deadline = std::time::Instant::now() + Duration::from_secs(60);
        while model.poll()["busy"] == true {
            assert!(std::time::Instant::now() < deadline);
            std::thread::sleep(Duration::from_millis(10));
        }
        let result = latest().expect("Actual result must be retained");
        assert!(
            result.job.summary.comparison.bits_tested > 0,
            "{}",
            model.state()
        );
        assert_eq!(result.job.summary.comparison.bit_errors, 0);
        assert!(!result.observations.taps[0].values.is_empty());
        assert!(result.job.constellation.as_ref().unwrap().collected_pairs > 0);
        let query = crate::receiver_instruments::selection::Query::default();
        let scatter = crate::receiver_instruments::selection::qam(&query);
        assert_eq!(scatter["available"], true);
        let control = result
            .job
            .constellation
            .as_ref()
            .unwrap()
            .ncp_pairs
            .first()
            .expect("Actual CRC-classified NCP");
        let ncp_query = crate::receiver_instruments::selection::Query {
            signal: 1,
            first_bin: i32::from(control.physical_bin),
            last_bin: i32::from(control.physical_bin),
            first_symbol: control.channel_symbol_index as i64,
            last_symbol: control.channel_symbol_index as i64,
            ..Default::default()
        };
        let scatter = crate::receiver_instruments::selection::qam(&ncp_query);
        assert_eq!(scatter["available"], true);
        assert_eq!(scatter["points"][0]["bin"], control.physical_bin);
        let empty_query = crate::receiver_instruments::selection::Query {
            first_bin: 8191,
            last_bin: 8191,
            ..Default::default()
        };
        assert_eq!(
            crate::receiver_instruments::selection::qam(&empty_query)["points"],
            serde_json::json!([])
        );
        let decisions = crate::receiver_instruments::diagnostics::slicer("{}", false).unwrap();
        assert!(!decisions["rows"].as_array().unwrap().is_empty());
        assert!(decisions["llr"]["count"].as_u64().unwrap() > 0);
        let coding = crate::receiver_instruments::diagnostics::slicer("{}", true).unwrap();
        assert_eq!(coding["llr"]["count"], 16200);
        assert_eq!(
            crate::receiver_instruments::diagnostics::response("{}").unwrap()["available"],
            true
        );
        assert_eq!(
            crate::receiver_instruments::diagnostics::response(r#"{"source":1}"#).unwrap()
                ["available"],
            false
        );
        assert_eq!(
            crate::receiver_instruments::diagnostics::fec()["compared_bits"],
            result.job.summary.comparison.bits_tested
        );
        let charts = crate::receiver_instruments::diagnostics::fec()["verification_charts"].clone();
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
        assert!(latest().is_none());
        assert_eq!(
            crate::receiver_instruments::selection::qam(&query)["available"],
            false
        );
    }
}
