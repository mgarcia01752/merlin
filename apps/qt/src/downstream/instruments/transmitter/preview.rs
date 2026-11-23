/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! GUI-thread lifecycle and viewport formatting over immutable Rust worker results.
use crate::downstream::common::constants::IDLE_PREVIEW_COMPLETION_CAPACITY;
use crate::downstream::inputs::public_inputs::transmitter_inputs;
use crate::downstream::workbench::session::Context;
use crate::shared::presentation::constants::MIN_SPECTRUM_DISPLAY_BINS;
use merlin::downstream::{
    ChannelConfiguration, ChannelConfigurationInput, ConfiguredPhysicalLayout,
    TransmissionConfiguration, TransmitterPreview, TransmitterPreviewControl,
    TransmitterPreviewRequest, MAX_TRANSMISSION_DOCUMENT_BYTES,
};
use serde_json::{json, Value};
use std::sync::mpsc::{self, Receiver, TryRecvError};

struct PreparedPreview {
    request: TransmitterPreviewRequest,
    layout: ConfiguredPhysicalLayout,
}

struct Job {
    revision: u64,
    control: TransmitterPreviewControl,
    receiver: Receiver<Result<Box<TransmitterPreview>, String>>,
}

#[derive(Default)]
pub(crate) struct Analyzer {
    pub(crate) context: Context,
    revision: u64,
    timing_settings: Option<TransmissionConfiguration>,
    result_token: u64,
    request: Option<PreparedPreview>,
    job: Option<Job>,
    result: Option<TransmitterPreview>,
    error: String,
    notice: String,
}

impl Analyzer {
    /// Freezes accepted settings/inputs and invalidates older sample results.
    /// # Arguments
    /// * `document` - Bounded accepted OCD JSON.
    /// * `source` - Accepted original source section, or null before Apply.
    /// * `profile` - Accepted original profile section, or null before Apply.
    /// # Returns
    /// No result; status reports missing prerequisites or core configuration errors.
    pub(crate) fn configure(&mut self, document: &str, source: &Value, profile: &Value) {
        self.cancel();
        self.revision = self.revision.wrapping_add(1);
        self.result = None;
        self.request = None;
        self.timing_settings = None;
        self.error.clear();
        self.notice.clear();
        let prepare = || -> Result<(TransmissionConfiguration, Option<PreparedPreview>), String> {
            if document.len() > MAX_TRANSMISSION_DOCUMENT_BYTES {
                return Err(format!(
                    "OCD request exceeds {MAX_TRANSMISSION_DOCUMENT_BYTES} bytes."
                ));
            }
            let transmission = TransmissionConfiguration::from_json(document.as_bytes())
                .map_err(|cause| cause.message().to_owned())?;
            let request = if let Some(inputs) = transmitter_inputs(source, profile)? {
                let request = TransmitterPreviewRequest::from_capture(
                    &transmission,
                    &inputs.capture,
                    inputs.profile_id,
                    &inputs.source,
                )
                .map_err(|cause| cause.message().to_owned())?;
                let layout = transmission
                    .physical_layout_capture(&inputs.capture, inputs.profile_id, 0, 0, 0)
                    .map_err(|cause| cause.message().to_owned())?;
                Some(PreparedPreview { request, layout })
            } else {
                None
            };
            Ok((transmission, request))
        };
        match prepare() {
            Ok((settings, request)) => {
                self.timing_settings = Some(settings);
                self.request = request;
            }
            Err(error) => self.error = error,
        }
    }

    /// Starts at most one bounded worker; repeated calls while busy are ignored.
    /// # Returns
    /// Status reflects progress, unavailable inputs or thread-spawn diagnostics.
    pub(crate) fn generate(&mut self) {
        self.poll();
        if self.job.is_some() {
            return;
        }
        let Some(request) = self
            .request
            .as_ref()
            .map(|prepared| prepared.request.clone())
        else {
            return;
        };
        self.error.clear();
        self.notice.clear();
        let control = TransmitterPreviewControl::new();
        let worker_control = control.clone();
        let (sender, receiver) = mpsc::sync_channel(IDLE_PREVIEW_COMPLETION_CAPACITY);
        let spawn = std::thread::Builder::new()
            .name("merlin-tx-spectrum".into())
            .spawn(move || {
                let result = request
                    .generate(&worker_control)
                    .map(Box::new)
                    .map_err(|cause| cause.message().to_owned());
                let _ = sender.send(result);
            });
        match spawn {
            Ok(_handle) => {
                self.job = Some(Job {
                    revision: self.revision,
                    control,
                    receiver,
                })
            }
            Err(error) => self.error = format!("Cannot start TX worker: {error}"),
        }
    }

    /// Requests cooperative cancellation without blocking the Qt event loop.
    /// # Returns
    /// A live worker remains tracked until completion; prior results stay intact.
    pub(crate) fn cancel(&mut self) {
        if let Some(job) = &self.job {
            job.control.cancel();
        }
    }

    /// Drains one bounded completion message and rejects stale generations.
    /// # Returns
    /// Updated scalar status; callers query a viewport only after result-token changes.
    pub(crate) fn poll(&mut self) -> Value {
        let message = self.job.as_ref().map(|job| job.receiver.try_recv());
        if let Some(message) = message {
            match message {
                Ok(result) => {
                    let job = self
                        .job
                        .take()
                        .expect("Completion requires a tracked worker");
                    if job.revision == self.revision {
                        if job.control.snapshot().cancellation_requested {
                            self.notice =
                                "Preview cancelled; accepted inputs are unchanged.".into();
                        } else {
                            match result {
                                Ok(result) => {
                                    self.result = Some(*result);
                                    self.result_token = self.result_token.wrapping_add(1);
                                    self.notice =
                                        "Completed TX sample snapshot before channel impairments."
                                            .into();
                                }
                                Err(error) => self.error = error,
                            }
                        }
                    }
                }
                Err(TryRecvError::Disconnected) => {
                    self.job = None;
                    self.error = "TX spectrum worker ended without a result.".into();
                }
                Err(TryRecvError::Empty) => {}
            }
        }
        self.status()
    }

    /// Queries scalar readiness/progress without serializing samples or FFT arrays.
    /// # Returns
    /// Stable status/reason and a result token for bounded viewport refresh.
    pub(crate) fn status(&self) -> Value {
        let (status, reason) = if !self.error.is_empty() {
            ("error", self.error.as_str())
        } else if self.request.is_some() {
            (
                "complete",
                "Accepted OCD, DPD and PRBS-TX form a supported transmitter configuration.",
            )
        } else {
            (
                "attention",
                "Accept compatible OCD, DPD and PRBS-TX inputs to configure the transmitter.",
            )
        };
        let (done, total) = self
            .job
            .as_ref()
            .map(|job| {
                let progress = job.control.snapshot();
                (progress.completed_symbols, progress.total_symbols)
            })
            .unwrap_or_default();
        json!({"status":status,"reason":reason,"tap_name":"TX Output","tap_id":"tx.overlap_add.samples","tap_description":"Complex baseband I/Q · post-prefix, window and overlap · before channel","busy":self.job.is_some(),"can_generate":self.request.is_some()&&self.job.is_none(),"has_result":self.result.is_some(),"result_token":self.result_token.to_string(),"configuration_token":self.revision.to_string(),"progress":if total==0 {
            0.0
        } else {
            done as f64/total as f64
        },"message":if self.error.is_empty() {
            self.notice.as_str()
        } else {
            self.error.as_str()
        }})
    }

    /// Formats a min/max spectrum viewport; large sample/FFT storage stays in Rust.
    /// # Arguments
    /// * `start` - First FFT bin; nonnegative and inside the spectrum.
    /// * `count` - Requested bin count; zero selects full span.
    /// * `capture_axis` - True uses captured bin frequency labels; false centers DC.
    /// # Returns
    /// At most 2048 points, normalized drawing coordinates and formatted readouts.
    /// Invalid requests return an error document without changing scientific results.
    pub(crate) fn view(&self, start: i32, count: i32, capture_axis: bool) -> Value {
        match self.viewport(start, count, capture_axis) {
            Ok(view) => view,
            Err(error) => json!({"error":error,"points":[],"x_ticks":[],"y_ticks":[]}),
        }
    }

    /// Returns timing from completed samples or the accepted current OCD before a run.
    /// # Arguments
    /// * `velocity` - Nominal cable VF in (0,1]; NaN represents unknown input.
    /// # Returns
    /// Bounded Rust-derived labels/coordinates, or an error with no stale geometry.
    pub(crate) fn timing(&self, velocity: f64) -> Value {
        let prepare = || -> Result<Value, String> {
            let completed = crate::downstream::simulation::closed_loop::latest(&self.context);
            let (configuration, origin) = if let Some(result) = completed.as_ref() {
                (
                    TransmissionConfiguration::new(result.job.settings.transmission.clone())
                        .map_err(|cause| cause.message().to_owned())?,
                    "Completed run",
                )
            } else if let Some(result) = self.result.as_ref() {
                (
                    TransmissionConfiguration::new(result.info().settings)
                        .map_err(|cause| cause.message().to_owned())?,
                    "Transmitter preview",
                )
            } else {
                (
                    self.timing_settings
                        .clone()
                        .ok_or("Apply an OFDM configuration to show symbol timing.")?,
                    "Current configuration",
                )
            };
            let velocity = if velocity.is_nan() {
                None
            } else {
                Some(velocity)
            };
            let report = configuration
                .symbol_timing(velocity)
                .map_err(|cause| cause.message().to_owned())?;
            let window = configuration
                .window()
                .map_err(|cause| cause.message().to_owned())?;
            let format_distance = |distance: Option<f64>| {
                distance
                    .map(|value| format!("{value:.0} ft"))
                    .unwrap_or_else(|| "Unknown VF".into())
            };
            let ticks = vec![
                json!({"x":0.0,"label":"0"}),
                json!({"x":report.prefix_fraction,"label":format!("{:.3}",report.prefix_us)}),
                json!({"x":1.0,"label":format!("{:.3} µs",report.zoom_us)}),
            ];
            Ok(
                json!({"source":origin,"window":crate::downstream::instruments::spectrum::symbol_timing::project(&window),"points":report.points,"prefix_fraction":report.prefix_fraction,"roll_off_fraction":report.roll_off_fraction,
                "ticks":ticks,"overview":format!("CP {} / {:.3} µs · FFT {} samples / {:.1} µs · symbol {:.3} µs",report.prefix_samples,report.prefix_us,report.fft_samples,report.useful_us,report.symbol_us),
                "roll_label":format!("δ {:.3} µs",report.roll_off_us),"guard_label":format!("Flat guard {:.3} µs",report.guard_us),
                "extra_path":format_distance(report.extra_path_feet),"reflection_spacing":format_distance(report.reflection_spacing_feet),
                "budget_label":format!("Ideal guard budget: {:.3} µs",report.guard_us)}),
            )
        };
        prepare().unwrap_or_else(|error| json!({"error":error,"points":[],"ticks":[]}))
    }

    fn viewport(&self, start: i32, count: i32, capture_axis: bool) -> Result<Value, String> {
        let result = self
            .result
            .as_ref()
            .ok_or("Generate a TX sample snapshot first.")?;
        let prepared = self
            .request
            .as_ref()
            .ok_or("No accepted TX configuration.")?;
        let info = result.info();
        let start = usize::try_from(start).map_err(|_| "Invalid spectrum start.")?;
        // Preserve the original UI zero-count convention: full FFT width, not remaining bins.
        let count = if count == 0 {
            info.settings.subcarrier_spacing.fft_samples() as usize
        } else {
            usize::try_from(count).map_err(|_| "Invalid spectrum count.")?
        };
        if count < MIN_SPECTRUM_DISPLAY_BINS {
            return Err("Display at least two spectrum bins.".into());
        }
        let channel = ChannelConfiguration::new(ChannelConfigurationInput::default())
            .map_err(|cause| cause.message().to_owned())?;
        let spectrum = result
            .calibrated_spectrum(&channel, start, count)
            .map_err(|cause| cause.message().to_owned())?;
        let full = result
            .calibrated_spectrum(&channel, 0, 0)
            .map_err(|cause| cause.message().to_owned())?;
        Ok(crate::shared::presentation::spectrum::viewport(
            &spectrum,
            &full,
            &prepared.layout.layout,
            start,
            count,
            capture_axis,
            &format!(
                "{} actual TX samples · first sample {}",
                info.sample_count, info.first_sample
            ),
        ))
    }
}

impl Drop for Analyzer {
    fn drop(&mut self) {
        self.cancel();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::downstream::workbench::project_file::encode;
    use merlin::downstream::TransmissionSettings;
    use std::time::{Duration, Instant};

    // Existing host-test watchdog and poll cadence; neither is a simulation/display policy.
    const WORKER_TEST_TIMEOUT_SECONDS: u64 = 30;
    const WORKER_TEST_POLL_MILLISECONDS: u64 = 10;

    fn inputs() -> (Value, Value) {
        // Independent original all-ones PRBS15 source fixture; dormant fill stays disabled.
        // Its equally valued fill seed is a separate inactive fixture choice.
        let source = json!({"options":{"mode":"prbs15","seed":32767,"append":false,
            "target_bits":0,"fill_seed":32767},"file":null});
        let profile = json!({"profile_id":0,"file":encode("demo",include_bytes!(
            "../../../../../../examples/measurements/single-channel/modulation_profile.json"))});
        (source, profile)
    }

    fn configure(model: &mut Analyzer, settings: &str, inputs: (Value, Value)) {
        model.configure(settings, &inputs.0, &inputs.1);
    }

    fn wait(model: &mut Analyzer) {
        let deadline = Instant::now() + Duration::from_secs(WORKER_TEST_TIMEOUT_SECONDS);
        while model.poll()["busy"] == true {
            assert!(Instant::now() < deadline, "TX worker did not terminate");
            std::thread::sleep(Duration::from_millis(WORKER_TEST_POLL_MILLISECONDS));
        }
    }

    #[test]
    fn readiness_cancellation_and_stale_generation_preserve_accepted_inputs() {
        let settings = serde_json::to_string(&TransmissionSettings::default()).unwrap();
        let mut model = Analyzer::default();
        configure(&mut model, &settings, Default::default());
        assert_eq!(model.status()["status"], "attention");
        configure(&mut model, &settings, inputs());
        assert_eq!(model.status()["status"], "complete");
        model.generate();
        model.generate();
        assert!(model.job.is_some());
        configure(&mut model, &settings, Default::default());
        wait(&mut model);
        assert!(model.result.is_none());
        assert_eq!(model.timing(0.82)["source"], "Current configuration");
        assert_eq!(model.status()["status"], "attention");
        let mut unsupported = inputs();
        // Original encoded source ID16 is the first identity unsupported by the idle TX engine.
        let mut original: Value = serde_json::from_slice(include_bytes!(
            "../../../../../../examples/measurements/single-channel/modulation_profile.json"
        ))
        .unwrap();
        for profile in original["profiles"].as_array_mut().unwrap() {
            if profile["profile_id"] == 0 {
                profile["profile_id"] = json!(16);
            }
        }
        unsupported.1 = json!({"profile_id":16,"file":encode("unsupported.json",&serde_json::to_vec(&original).unwrap())});
        configure(&mut model, &settings, unsupported);
        assert_eq!(model.status()["status"], "error");
    }

    #[test]
    fn accepted_timing_refreshes_without_simulation_and_vf_changes_only_distance() {
        let mut model = Analyzer::default();
        let settings = TransmissionSettings::default();
        configure(
            &mut model,
            &serde_json::to_string(&settings).unwrap(),
            Default::default(),
        );
        let initial = model.timing(0.82);
        assert_eq!(initial["source"], "Current configuration");
        assert_eq!(initial["window"], model.timing(0.87)["window"]);
        // Independent alternate geometry: 1024-sample prefix/256-sample taper -> 45/1.25 us.
        let changed = TransmissionSettings {
            cyclic_prefix_samples: 1024,
            roll_off_samples: 256,
            ..settings
        };
        configure(
            &mut model,
            &serde_json::to_string(&changed).unwrap(),
            Default::default(),
        );
        let updated = model.timing(0.82);
        assert_eq!(updated["window"]["symbol_us"], 45.0);
        assert_eq!(updated["window"]["roll_us"], 1.25);
        assert_ne!(
            initial["window"]["overview"]["points"],
            updated["window"]["overview"]["points"]
        );
        assert!(!model.status()["has_result"].as_bool().unwrap());
    }

    #[test]
    fn generated_baseband_and_capture_axes_share_one_immutable_spectrum() {
        let settings = serde_json::to_string(&TransmissionSettings::default()).unwrap();
        let mut model = Analyzer::default();
        configure(&mut model, &settings, inputs());
        model.generate();
        wait(&mut model);
        assert_eq!(model.status()["status"], "complete");
        assert!(model.result.is_some(), "{}", model.status());
        let timing = model.timing(0.82);
        assert_eq!(timing["guard_label"], "Flat guard 1.875 µs");
        assert_eq!(timing["reflection_spacing"], "756 ft");
        assert_eq!(model.timing(0.87)["reflection_spacing"], "802 ft");
        assert_eq!(model.timing(f64::NAN)["reflection_spacing"], "Unknown VF");
        assert!(model.timing(0.0).get("error").is_some());
        // Original scientific/drawing anchors: FFT covers +/-102.4 MHz, source origin631.1 MHz,
        // at most2048 backend extrema buckets and unchanged physical PLC shading.
        let baseband = model.view(0, 0, false);
        let capture = model.view(0, 0, true);
        assert_eq!(baseband["x_ticks"][0]["label"], "-102.400");
        assert_eq!(baseband["x_ticks"][2]["label"], "0.000");
        assert_eq!(capture["x_ticks"][0]["label"], "631.100");
        assert_eq!(baseband["points"].as_array().unwrap().len(), 2048);
        assert_eq!(baseband["points"][0]["y"], capture["points"][0]["y"]);
        assert_eq!(baseband["bands"].as_array().unwrap().len(), 1);
        assert_eq!(baseband["bands"], capture["bands"]);
        assert!(baseband.get("samples").is_none());
        assert!(model.view(-1, 20, false).get("error").is_some());
        assert!(model.view(0, 1, false).get("error").is_some());
        assert!(model.view(1, 0, false).get("error").is_some());
        assert!(baseband["channel_power_text"]
            .as_str()
            .unwrap()
            .contains("dB re unit²"));
        let zoom = model.view(300, 40, false);
        assert_eq!(zoom["points"].as_array().unwrap().len(), 40);
        let token = model.status()["result_token"].clone();
        model.generate();
        model.cancel();
        wait(&mut model);
        assert_eq!(model.status()["result_token"], token);
        assert!(model.result.is_some());
    }
}
