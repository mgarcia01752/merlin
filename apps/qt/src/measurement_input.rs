/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shared measurement editor/OS adapter; all interpretation stays in Rust.
mod document;
mod model;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use docsis_ofdm_fec::transmission::TransmissionSettings;
use document::{Capture, MAX_INPUT_BYTES};
use model::{ChartRequest, Model, Options};
use serde_json::{json, Value};

#[cxx_qt::bridge]
pub mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type MeasurementInputs = super::MeasurementInputsRust;
        /// Starts a draft from the accepted measurement configuration.
        /// # Arguments
        /// * `rxmer` - Selects RxMER or channel response.
        /// # Returns
        /// Bounded files/options, with full inputs retained in Rust.
        #[qinvokable]
        fn begin(self: Pin<&mut MeasurementInputs>, rxmer: bool) -> QString;
        /// Opens the native chooser; RxMER permits multiple selected files.
        /// # Arguments
        /// * `rxmer` - Expected capture type and selection policy.
        /// # Returns
        /// Draft metadata or failure preserving the previous draft.
        #[qinvokable]
        fn choose_files(self: Pin<&mut MeasurementInputs>, rxmer: bool) -> QString;
        /// Stages a bounded list of local file URLs atomically.
        /// # Arguments
        /// * `rxmer` - Expected capture kind.
        /// * `urls` - JSON array with at most 64 local URLs.
        /// # Returns
        /// Draft metadata or structured diagnostic.
        #[qinvokable]
        fn stage(self: Pin<&mut MeasurementInputs>, rxmer: bool, urls: &QString) -> QString;
        /// Loads an existing sanitized example through the same shared contract.
        /// # Arguments
        /// * `rxmer` - RxMER or response example.
        /// * `variant` - Zero baseline, one RxMER notch alternative.
        /// # Returns
        /// Updated draft metadata; originals remain untouched.
        #[qinvokable]
        fn demo(self: Pin<&mut MeasurementInputs>, rxmer: bool, variant: i32) -> QString;
        /// Edits one draft list or switches the response to explicit flat/unity.
        /// # Arguments
        /// * `rxmer` - Draft kind.
        /// * `index` - Selected source, -1 clears all / selects flat.
        /// * `delta` - Zero removes; -1/+1 reorders RxMER; 2 sorts by original epoch.
        /// # Returns
        /// Updated bounded metadata.
        #[qinvokable]
        fn edit(self: Pin<&mut MeasurementInputs>, rxmer: bool, index: i32, delta: i32) -> QString;
        /// Validates current draft options and reports scientific scheduling metadata.
        /// # Arguments
        /// * `rxmer` - Draft kind.
        /// * `options` - Bounded JSON mode/interval/tolerance.
        /// * `settings` - Accepted OCD JSON.
        /// # Returns
        /// Admission diagnostic and explicit symbol/time schedule.
        #[qinvokable]
        fn describe(
            self: &MeasurementInputs,
            rxmer: bool,
            options: &QString,
            settings: &QString,
        ) -> QString;
        /// Queries one shared bounded original/effective measurement chart.
        /// # Arguments
        /// * `rxmer` - Measurement kind.
        /// * `request` - Bounded viewport/selection/options JSON.
        /// * `settings` - Accepted OCD.
        /// # Returns
        /// At most 1024 min/max buckets with original gaps preserved.
        #[qinvokable]
        fn view(
            self: &MeasurementInputs,
            rxmer: bool,
            request: &QString,
            settings: &QString,
        ) -> QString;
        /// Accepts a fully valid draft atomically, without starting simulation.
        /// # Arguments
        /// * `rxmer` - Measurement kind.
        /// * `options` - Draft mode/schedule/association settings.
        /// * `settings` - Accepted OCD.
        /// # Returns
        /// Empty on success; diagnostic preserves the accepted configuration.
        #[qinvokable]
        fn apply(
            self: Pin<&mut MeasurementInputs>,
            rxmer: bool,
            options: &QString,
            settings: &QString,
        ) -> QString;
        /// Derives joint readiness from accepted measurements and current OCD/DPD.
        /// # Arguments
        /// * `settings` - Accepted OCD JSON.
        /// # Returns
        /// RxMER/response/channel statuses; green means configured, not running.
        #[qinvokable]
        fn state(self: &MeasurementInputs, settings: &QString) -> QString;
    }
}
/// One GUI-thread owner with separate accepted/draft inputs for both shared viewers.
#[derive(Default)]
pub struct MeasurementInputsRust {
    model: Model,
}

fn parse<T: serde::de::DeserializeOwned>(document: &str) -> Result<T, String> {
    if document.len() > 4096 {
        return Err("Configuration exceeds 4096 bytes.".into());
    }
    serde_json::from_str(document).map_err(|error| error.to_string())
}
fn status(result: Result<(), String>, empty: bool) -> Value {
    match result {
        Ok(()) => json!({"status":"complete","reason":"Validated configuration accepted."}),
        Err(error) => json!({"status":if empty {"attention"} else {"error"},"reason":error}),
    }
}
impl MeasurementInputsRust {
    fn metadata(&self, rxmer: bool) -> Value {
        let profile = crate::workbench_session::snapshot().profile;
        let mut options = self.model.options.clone();
        if !rxmer {
            options.tolerance_seconds = self.model.response_tolerance;
        }
        json!({"files":self.model.files(rxmer,profile.as_ref()),"options":options,"flat":!rxmer&&self.model.draft_response.is_none()})
    }
    fn stage_urls(&mut self, rxmer: bool, urls: Vec<String>) -> Result<(), String> {
        if urls.len() > 64 {
            return Err("Select at most 64 files.".into());
        }
        if urls.is_empty() {
            return Ok(());
        }
        let mut captures = Vec::new();
        let mut total = 0usize;
        for url in urls {
            let (name, bytes) = crate::input_file::read_local(&url, MAX_INPUT_BYTES)?;
            total = total
                .checked_add(bytes.len())
                .ok_or("Input byte count overflow.")?;
            if total > document::MAX_TOTAL_BYTES {
                return Err("Batch exceeds 64 MiB.".into());
            }
            captures.push(
                Capture::decode(name.clone(), bytes, rxmer)
                    .map_err(|error| format!("{name}: {error}"))?,
            );
        }
        self.model.stage(captures, rxmer)
    }
    fn admission(
        &self,
        rxmer: bool,
        options: &Options,
        settings: &TransmissionSettings,
    ) -> Result<Value, String> {
        let profile = crate::workbench_session::snapshot().profile;
        if rxmer {
            let plan = Model::playback(&self.model.draft_rx, options, settings, profile.as_ref())?;
            let (symbols, ms) = plan.span();
            let summary = match options.mode {
                docsis_ofdm_fec::simulation::channel::measurement_playback::PlaybackMode::Single => "Static per-carrier RxMER · one original capture held for the run".into(),
                docsis_ofdm_fec::simulation::channel::measurement_playback::PlaybackMode::Average => format!("Static equal-weight error-power average · {} original captures · no temporal playback",plan.frames().len()),
                _ => format!("Interval {:.3} ms · final transition at symbol {} ({:.3} ms) · hold final capture",plan.interval_milliseconds(),symbols,ms),
            };
            Ok(json!({"error":"","summary":summary}))
        } else {
            Model::response(
                self.model.draft_response.as_ref(),
                settings,
                profile.as_ref(),
                options.tolerance_seconds,
            )?;
            Ok(
                json!({"error":"","summary":if self.model.draft_response.is_none() {"Synthetic flat H = 1 + 0i per carrier · 0 dB / 0°"} else {"Validated captured channel response H; receiver estimation is separate."}}),
            )
        }
    }
}
impl bridge::MeasurementInputs {
    /// Creates a draft without changing accepted scientific state.
    /// # Arguments
    /// * `rxmer` - Measurement kind.
    /// # Returns
    /// Bounded files/options JSON.
    pub fn begin(mut self: core::pin::Pin<&mut Self>, rxmer: bool) -> QString {
        self.as_mut().rust_mut().model.begin(rxmer);
        QString::from(self.rust().metadata(rxmer).to_string())
    }
    /// Runs a native single/multiple chooser and atomically stages its selection.
    /// # Arguments
    /// * `rxmer` - Expected type; RxMER allows multiple files.
    /// # Returns
    /// Updated metadata, or error preserving draft inputs.
    pub fn choose_files(mut self: core::pin::Pin<&mut Self>, rxmer: bool) -> QString {
        let result = if rxmer {
            crate::native_file_picker::choose_many("Open RxMER Captures")
        } else {
            crate::native_file_picker::choose("Open Channel Response Capture")
                .map(|value| value.into_iter().collect())
        };
        let result = result.and_then(|urls| self.as_mut().rust_mut().stage_urls(rxmer, urls));
        let mut metadata = self.rust().metadata(rxmer);
        if let Err(error) = result {
            metadata["error"] = json!(error);
        }
        QString::from(metadata.to_string())
    }
    /// Stages bounded native/local input URLs through shared decoder validation.
    /// # Arguments
    /// * `rxmer` - Expected type.
    /// * `urls` - JSON array of local URIs.
    /// # Returns
    /// Draft metadata or error preserving previous state.
    pub fn stage(mut self: core::pin::Pin<&mut Self>, rxmer: bool, urls: &QString) -> QString {
        let result = if urls.to_string().len() > 262144 {
            Err("Selection is too large.".into())
        } else {
            serde_json::from_str::<Vec<String>>(&urls.to_string())
                .map_err(|error| error.to_string())
                .and_then(|urls| self.as_mut().rust_mut().stage_urls(rxmer, urls))
        };
        let mut metadata = self.rust().metadata(rxmer);
        if let Err(error) = result {
            metadata["error"] = json!(error);
        }
        QString::from(metadata.to_string())
    }
    /// Stages an immutable bundled demo through the production decoder/importer.
    /// # Arguments
    /// * `rxmer` - Example kind.
    /// * `variant` - Baseline or notch example.
    /// # Returns
    /// Metadata with a diagnostic on failure.
    pub fn demo(mut self: core::pin::Pin<&mut Self>, rxmer: bool, variant: i32) -> QString {
        let (name, bytes): (&str, &[u8]) = if !rxmer {
            (
                "channel_estimation.json",
                include_bytes!(
                    "../../../examples/measurements/single-channel/channel_estimation.json"
                ),
            )
        } else if variant == 1 {
            (
                "rxmer-center-notch-12db.json",
                include_bytes!(
                    "../../../examples/measurements/single-channel/rxmer-center-notch-12db.json"
                ),
            )
        } else {
            (
                "rxmer-baseline.bin",
                include_bytes!(
                    "../../../examples/measurements/rxmer-fec-stress/rxmer-baseline.bin"
                ),
            )
        };
        let result = Capture::decode(name.into(), bytes.to_vec(), rxmer)
            .and_then(|capture| self.as_mut().rust_mut().model.stage(vec![capture], rxmer));
        let mut metadata = self.rust().metadata(rxmer);
        if let Err(error) = result {
            metadata["error"] = json!(error);
        }
        QString::from(metadata.to_string())
    }
    /// Removes, reorders or time-sorts a draft without modifying accepted inputs.
    /// # Arguments
    /// * `rxmer` - Draft kind.
    /// * `index` - Source index; -1 clears inputs/selects unity.
    /// * `delta` - Zero remove, signed one move, two explicit timestamp sort.
    /// # Returns
    /// Updated metadata.
    pub fn edit(
        mut self: core::pin::Pin<&mut Self>,
        rxmer: bool,
        index: i32,
        delta: i32,
    ) -> QString {
        let model = &mut self.as_mut().rust_mut().model;
        if rxmer {
            if delta == 2 {
                model.draft_rx.sort_by_key(Capture::epoch);
            } else if index < 0 {
                model.draft_rx.clear();
            } else if (index as usize) < model.draft_rx.len() {
                if delta == 0 {
                    model.draft_rx.remove(index as usize);
                } else if [-1, 1].contains(&delta) {
                    let target = index + delta;
                    if target >= 0 && (target as usize) < model.draft_rx.len() {
                        model.draft_rx.swap(index as usize, target as usize);
                    }
                }
            }
        } else {
            model.draft_response = None;
        }
        QString::from(self.rust().metadata(rxmer).to_string())
    }
    /// Validates draft combination/schedule options through the scientific core.
    /// # Arguments
    /// * `rxmer` - Measurement kind.
    /// * `options` - Bounded draft JSON.
    /// * `settings` - Accepted OCD JSON.
    /// # Returns
    /// Summary or admission error, without accepting the draft.
    pub fn describe(&self, rxmer: bool, options: &QString, settings: &QString) -> QString {
        let result = parse::<Options>(&options.to_string()).and_then(|options| {
            parse::<TransmissionSettings>(&settings.to_string())
                .and_then(|settings| self.rust().admission(rxmer, &options, &settings))
        });
        QString::from(
            result
                .unwrap_or_else(|error| json!({"error":error}))
                .to_string(),
        )
    }
    /// Formats a bounded measurement chart using shared Rust projections.
    /// # Arguments
    /// * `rxmer` - Chart kind.
    /// * `request` - Bounded selection/window/options JSON.
    /// * `settings` - Accepted OCD.
    /// # Returns
    /// Bounded viewport or diagnostic with no stale points.
    pub fn view(&self, rxmer: bool, request: &QString, settings: &QString) -> QString {
        let profile = crate::workbench_session::snapshot().profile;
        let result = parse::<ChartRequest>(&request.to_string()).and_then(|request| {
            parse::<TransmissionSettings>(&settings.to_string()).and_then(|settings| {
                self.rust()
                    .model
                    .chart(rxmer, &request, &settings, profile.as_ref())
            })
        });
        QString::from(
            result
                .unwrap_or_else(
                    |error| json!({"error":error,"points":[],"x_ticks":[],"y_ticks":[]}),
                )
                .to_string(),
        )
    }
    /// Accepts only a fully validated draft, preserving the previous input on error.
    /// # Arguments
    /// * `rxmer` - Draft kind.
    /// * `options` - Mode/interval/association controls.
    /// * `settings` - Accepted OCD.
    /// # Returns
    /// Empty on success, diagnostic otherwise. No run starts.
    pub fn apply(
        mut self: core::pin::Pin<&mut Self>,
        rxmer: bool,
        options: &QString,
        settings: &QString,
    ) -> QString {
        let result = parse::<Options>(&options.to_string()).and_then(|options| {
            let settings = parse::<TransmissionSettings>(&settings.to_string())?;
            self.rust().admission(rxmer, &options, &settings)?;
            let model = &mut self.as_mut().rust_mut().model;
            if rxmer {
                model.accepted_rx = model.draft_rx.clone();
                model.options = options;
            } else {
                model.accepted_response = model.draft_response.clone();
                model.response_tolerance = options.tolerance_seconds;
            }
            Ok(())
        });
        QString::from(result.err().unwrap_or_default())
    }
    /// Derives joint configuration readiness; run dispatch remains separate.
    /// # Arguments
    /// * `settings` - Accepted OCD JSON.
    /// # Returns
    /// Independent RxMER/response statuses and joint Channel Simulator readiness.
    pub fn state(&self, settings: &QString) -> QString {
        let profile = crate::workbench_session::snapshot().profile;
        let settings = parse::<TransmissionSettings>(&settings.to_string());
        let model = &self.rust().model;
        let rx = settings
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|settings| {
                Model::playback(
                    &model.accepted_rx,
                    &model.options,
                    settings,
                    profile.as_ref(),
                )
                .map(|_| ())
            });
        let response = settings
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|settings| {
                Model::response(
                    model.accepted_response.as_ref(),
                    settings,
                    profile.as_ref(),
                    model.response_tolerance,
                )
            });
        let rx_status = status(
            rx.clone(),
            profile.is_none() || model.accepted_rx.is_empty(),
        );
        let response_status = status(response.clone(), profile.is_none());
        let channel = if rx.is_ok() && response.is_ok() {
            json!({"status":"complete","reason":"RxMER and channel response are configured. Full-loop UI dispatch remains pending."})
        } else {
            json!({"status":if rx_status["status"]=="error"||response_status["status"]=="error" {"error"} else {"attention"},"reason":format!("{} {}",rx.err().unwrap_or_default(),response.err().unwrap_or_default())})
        };
        QString::from(json!({"rxmer":rx_status,"response":response_status,"channel":channel,"flat":model.accepted_response.is_none()}).to_string())
    }
}
