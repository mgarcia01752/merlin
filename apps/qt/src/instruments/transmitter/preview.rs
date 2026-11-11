/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! GUI-thread lifecycle and viewport formatting over immutable Rust worker results.
use crate::workbench::session::Inputs;
use docsis_ofdm_fec::{
    simulation::transmitter_preview::{TransmitterPreview, TransmitterPreviewRequest},
    transmission::TransmissionSettings,
};
use serde_json::{json, Value};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    mpsc::{self, Receiver, TryRecvError},
    Arc,
};

struct Job {
    revision: u64,
    cancel: Arc<AtomicBool>,
    done: Arc<AtomicUsize>,
    total: Arc<AtomicUsize>,
    receiver: Receiver<Result<Box<TransmitterPreview>, String>>,
}

#[derive(Default)]
pub(crate) struct Analyzer {
    revision: u64,
    result_token: u64,
    request: Option<TransmitterPreviewRequest>,
    job: Option<Job>,
    result: Option<TransmitterPreview>,
    error: String,
    notice: String,
}

impl Analyzer {
    /// Freezes accepted settings/inputs and invalidates older sample results.
    /// # Arguments
    /// * `document` - Bounded accepted OCD JSON.
    /// * `inputs` - GUI-thread snapshot of explicitly accepted source and profile.
    /// # Returns
    /// No result; status reports missing prerequisites or core configuration errors.
    pub(crate) fn configure(&mut self, document: &str, inputs: Inputs) {
        self.cancel();
        self.revision = self.revision.wrapping_add(1);
        self.result = None;
        self.request = None;
        self.error.clear();
        self.notice.clear();
        let prepare = || -> Result<Option<TransmitterPreviewRequest>, String> {
            if document.len() > 4096 {
                return Err("OCD request exceeds 4096 bytes.".into());
            }
            let settings: TransmissionSettings =
                serde_json::from_str(document).map_err(|error| error.to_string())?;
            settings.validate().map_err(|error| error.to_string())?;
            match (inputs.source, inputs.profile) {
                (Some(source), Some(profile)) => {
                    TransmitterPreviewRequest::new(&settings, &profile, source.config, source.seed)
                        .map(Some)
                        .map_err(|error| error.to_string())
                }
                _ => Ok(None),
            }
        };
        match prepare() {
            Ok(request) => self.request = request,
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
        let Some(request) = self.request.clone() else {
            return;
        };
        self.error.clear();
        self.notice.clear();
        let cancel = Arc::new(AtomicBool::new(false));
        let done = Arc::new(AtomicUsize::new(0));
        let total = Arc::new(AtomicUsize::new(0));
        let (sender, receiver) = mpsc::sync_channel(1);
        let (stop, completed, planned) =
            (Arc::clone(&cancel), Arc::clone(&done), Arc::clone(&total));
        let spawn = std::thread::Builder::new()
            .name("merlin-tx-spectrum".into())
            .spawn(move || {
                let result = request
                    .generate(&stop, |step, steps| {
                        planned.store(steps, Ordering::Relaxed);
                        completed.store(step, Ordering::Relaxed);
                    })
                    .map(Box::new)
                    .map_err(|error| error.to_string());
                let _ = sender.send(result);
            });
        match spawn {
            Ok(_handle) => {
                self.job = Some(Job {
                    revision: self.revision,
                    cancel,
                    done,
                    total,
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
            job.cancel.store(true, Ordering::Relaxed);
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
                        if job.cancel.load(Ordering::Relaxed) {
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
                (
                    job.done.load(Ordering::Relaxed),
                    job.total.load(Ordering::Relaxed),
                )
            })
            .unwrap_or_default();
        json!({"status":status,"reason":reason,"tap_name":"TX Output","tap_id":"tx.overlap_add.samples","tap_description":"Complex baseband I/Q · post-prefix, window and overlap · before channel","busy":self.job.is_some(),"can_generate":self.request.is_some()&&self.job.is_none(),"has_result":self.result.is_some(),"result_token":self.result_token.to_string(),"progress":if total==0 {0.0} else {done as f64/total as f64},"message":if self.error.is_empty(){self.notice.as_str()} else {self.error.as_str()}})
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

    /// Returns timing/window display data from completed result settings only.
    /// # Arguments
    /// * `velocity` - Nominal cable VF in (0,1]; NaN represents unknown input.
    /// # Returns
    /// Bounded Rust-derived labels/coordinates, or an error with no stale geometry.
    pub(crate) fn timing(&self, velocity: f64) -> Value {
        use docsis_ofdm_fec::signal_processing::waveform::{
            timing::symbol_timing, WaveformGeometry,
        };
        let prepare = || -> Result<Value, String> {
            let completed = crate::simulation::closed_loop::latest();
            let settings = completed
                .as_ref()
                .map(|result| &result.job.settings.transmission)
                .or_else(|| self.result.as_ref().map(|result| &result.settings))
                .ok_or("Run simulation to show symbol timing.")?;
            let geometry = WaveformGeometry::new(settings).map_err(|error| error.to_string())?;
            let report = symbol_timing(
                &geometry,
                if velocity.is_nan() {
                    None
                } else {
                    Some(velocity)
                },
            )
            .map_err(|error| error.to_string())?;
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
                json!({"points":report.points,"prefix_fraction":report.prefix_fraction,"roll_off_fraction":report.roll_off_fraction,
                "ticks":ticks,"overview":format!("CP {} · FFT {} samples / {:.1} µs · symbol {:.3} µs",report.prefix_samples,report.fft_samples,report.useful_us,report.symbol_us),
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
        let layout = docsis_ofdm_fec::transmission::layout::CarrierLayout::new(
            &result.settings,
            &result.layout,
        )
        .map_err(|e| e.to_string())?;
        crate::instruments::channel::channel_viewport(
            result.spectrum.clone(),
            start,
            count,
            capture_axis,
            &format!(
                "{} actual TX samples · first sample {}",
                result.samples.len(),
                result.first_sample
            ),
            &layout,
            &Default::default(),
        )
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
    use crate::workbench::session::AcceptedSource;
    use docsis_ofdm_fec::{
        scenario::pnm_json::{import_profiles, JsonCapture},
        simulation::source::PayloadSourceConfig,
    };
    use std::time::{Duration, Instant};

    fn inputs() -> Inputs {
        let profiles = import_profiles(JsonCapture {
            name: "demo",
            bytes: include_bytes!(
                "../../../../../examples/measurements/single-channel/modulation_profile.json"
            ),
        })
        .unwrap();
        Inputs {
            source: Some(AcceptedSource {
                config: PayloadSourceConfig::Prbs15,
                seed: 32767,
            }),
            profile: profiles.into_iter().find(|profile| profile.profile_id == 0),
            ..Default::default()
        }
    }

    fn wait(model: &mut Analyzer) {
        let deadline = Instant::now() + Duration::from_secs(30);
        while model.poll()["busy"] == true {
            assert!(Instant::now() < deadline, "TX worker did not terminate");
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    #[test]
    fn readiness_cancellation_and_stale_generation_preserve_accepted_inputs() {
        let settings = serde_json::to_string(&TransmissionSettings::default()).unwrap();
        let mut model = Analyzer::default();
        model.configure(&settings, Inputs::default());
        assert_eq!(model.status()["status"], "attention");
        model.configure(&settings, inputs());
        assert_eq!(model.status()["status"], "complete");
        model.generate();
        model.generate();
        assert!(model.job.is_some());
        model.configure(&settings, Inputs::default());
        wait(&mut model);
        assert!(model.result.is_none());
        assert!(model.timing(0.82).get("error").is_some());
        assert_eq!(model.status()["status"], "attention");
        let mut unsupported = inputs();
        unsupported.profile.as_mut().unwrap().profile_id = 16;
        model.configure(&settings, unsupported);
        assert_eq!(model.status()["status"], "error");
    }

    #[test]
    fn generated_baseband_and_capture_axes_share_one_immutable_spectrum() {
        let settings = serde_json::to_string(&TransmissionSettings::default()).unwrap();
        let mut model = Analyzer::default();
        model.configure(&settings, inputs());
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
        let baseband = model.view(0, 0, false);
        let capture = model.view(0, 0, true);
        assert_eq!(baseband["x_ticks"][0]["label"], "-102.400");
        assert_eq!(baseband["x_ticks"][2]["label"], "0.000");
        assert_eq!(capture["x_ticks"][0]["label"], "631.100");
        assert_eq!(baseband["points"].as_array().unwrap().len(), 2048);
        assert_eq!(baseband["points"][0]["y"], capture["points"][0]["y"]);
        assert!(baseband.get("samples").is_none());
        assert!(model.view(-1, 20, false).get("error").is_some());
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
