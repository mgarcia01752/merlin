/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shared measurement editor/OS adapter; all interpretation stays in Rust.
use crate::inputs::measurement_input::{document, model};
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
        /// Snapshots original baselines and restores Designer controls.
        #[qinvokable]
        fn designer_begin(self: Pin<&mut MeasurementInputs>) -> QString;
        /// Returns a bounded shared-axis before/after frequency preview.
        #[qinvokable]
        fn designer_preview(self: &MeasurementInputs, document: &QString, position: i32)
            -> QString;
        /// Generates a complete replacement draft on a cancellable Rust worker.
        #[qinvokable]
        fn designer_stage(
            self: Pin<&mut MeasurementInputs>,
            document: &QString,
            options: &QString,
        ) -> QString;
        /// Polls actual batch progress and stages only current complete results.
        #[qinvokable]
        fn designer_poll(self: Pin<&mut MeasurementInputs>) -> QString;
        /// Cancels pending generation without changing the parent draft.
        #[qinvokable]
        fn designer_cancel(self: Pin<&mut MeasurementInputs>);
        /// Queries one selected original for capture-preview animation.
        /// # Arguments
        /// * `options` - Bounded draft selected identities and controls.
        /// * `position` - Selected-frame offset.
        /// * `start`, `count` - Nonnegative viewport; zero count selects full span.
        /// # Returns
        /// At most 1024 buckets and scalar position/identity/time metadata.
        #[qinvokable]
        fn preview_frame(
            self: &MeasurementInputs,
            options: &QString,
            position: i32,
            start: i32,
            count: i32,
        ) -> QString;
        /// Queries selection capacity from the accepted CWB duration and OCD.
        /// # Arguments
        /// * `options` - Bounded selected identities and playback controls.
        /// * `settings` - Accepted OCD.
        /// * `run_options` - Accepted CWB options, or null if unconfigured.
        /// # Returns
        /// Loaded/selected counts, capacity and actionable timing guidance.
        #[qinvokable]
        fn selection_status(
            self: &MeasurementInputs,
            options: &QString,
            settings: &QString,
            run_options: &QString,
        ) -> QString;
        /// Reloads accepted originals after atomic project Open.
        #[qinvokable]
        fn reload(self: Pin<&mut MeasurementInputs>);
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
        /// * `urls` - JSON array with at most 256 local URLs.
        /// # Returns
        /// Draft metadata or structured diagnostic.
        #[qinvokable]
        fn stage(self: Pin<&mut MeasurementInputs>, rxmer: bool, urls: &QString) -> QString;
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
        /// Returns accepted amplifier, attenuation and electrical-reference choices.
        #[qinvokable]
        fn power_current(self: &MeasurementInputs) -> QString;
        /// Validates and accepts bounded conditioning JSON, preserving drafts on error.
        #[qinvokable]
        fn power_apply(self: Pin<&mut MeasurementInputs>, document: &QString) -> QString;
        /// Returns availability and identity of a selected channel TapPoint0..2.
        #[qinvokable]
        fn tap_state(self: &MeasurementInputs, tap: i32) -> QString;
        /// Returns a bounded calibrated/relative spectrum for the selected TapPoint.
        #[qinvokable]
        fn tap_spectrum(
            self: &MeasurementInputs,
            tap: i32,
            start: i32,
            count: i32,
            capture_axis: bool,
        ) -> QString;
        /// Returns bounded raw I/Q from the selected TapPoint and bin window.
        #[qinvokable]
        fn tap_qam(self: &MeasurementInputs, tap: i32, query: &QString) -> QString;
        /// Returns accepted additional-loss parameters; absent legacy settings mean Off.
        /// # Returns
        /// Small typed parameter JSON without capture arrays.
        #[qinvokable]
        fn loss_current(self: &MeasurementInputs) -> QString;
        /// Validates a draft and returns a bounded response preview.
        /// # Arguments
        /// * `document` - At most4096 bytes of additional-loss parameters.
        /// * `settings` - Accepted OCD JSON.
        /// # Returns
        /// Gain chart and endpoint summary, or an explicit diagnostic.
        #[qinvokable]
        fn loss_preview(
            self: &MeasurementInputs,
            document: &QString,
            settings: &QString,
        ) -> QString;
        /// Accepts additional loss atomically without editing measured captures.
        /// # Arguments
        /// * `document` - Draft additional-loss parameters.
        /// * `settings` - Accepted OCD JSON.
        /// # Returns
        /// Empty on success; failed validation preserves previous settings.
        #[qinvokable]
        fn loss_apply(
            self: Pin<&mut MeasurementInputs>,
            document: &QString,
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
    pub(super) model: Model,
    pub(super) designer: Option<crate::inputs::measurement_input::designer::Session>,
    pub(super) designer_work: Option<super::rxmer_designer::Work>,
}

fn parse<T: serde::de::DeserializeOwned>(document: &str) -> Result<T, String> {
    if document.len() > 4096 {
        return Err("Configuration exceeds 4096 bytes.".into());
    }
    serde_json::from_str(document).map_err(|error| error.to_string())
}

fn parse_selection<T: serde::de::DeserializeOwned>(document: &str) -> Result<T, String> {
    if document.len() > 24 * 1024 {
        return Err("RxMER selection exceeds 24 KiB.".into());
    }
    serde_json::from_str(document).map_err(|error| error.to_string())
}
impl MeasurementInputsRust {
    pub(super) fn metadata(&self, rxmer: bool) -> Value {
        let profile = crate::workbench::session::snapshot().profile;
        let mut options = self.model.options.clone();
        if !rxmer {
            options.tolerance_seconds = self.model.response_tolerance;
        }
        json!({"files":self.model.files(rxmer,profile.as_ref()),"options":options,"flat":!rxmer&&self.model.draft_response.is_none()})
    }
    fn stage_urls(&mut self, rxmer: bool, urls: Vec<String>) -> Result<(), String> {
        if urls.len()
            > docsis_ofdm_fec::simulation::channel::measurement_playback::MAX_PLAYBACK_FRAMES
        {
            return Err("Select at most 256 files.".into());
        }
        if urls.is_empty() {
            return Ok(());
        }
        let mut captures = Vec::new();
        let mut total = 0usize;
        for url in urls {
            let (name, bytes) = crate::platform::input_file::read_local(&url, MAX_INPUT_BYTES)?;
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
        let profile = crate::workbench::session::snapshot().profile;
        if rxmer {
            let plan = Model::playback(&self.model.draft_rx, options, settings, profile.as_ref())?;
            let (symbols, ms) = plan.span();
            let summary = match options.mode {
                docsis_ofdm_fec::simulation::channel::measurement_playback::PlaybackMode::Single => "Static per-carrier RxMER · one capture held for the run".into(),
                docsis_ofdm_fec::simulation::channel::measurement_playback::PlaybackMode::Average => format!("Static equal-weight error-power average · {} captures · no temporal playback",plan.frames().len()),
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
    /// Queries one original-capture animation frame without preparing a full replay.
    /// # Arguments
    /// * `options` - Draft selection, bounded to 24 KiB.
    /// * `position` - Zero-based selected-frame position.
    /// * `start`, `count` - Nonnegative carrier range; zero count is full span.
    /// # Returns
    /// Bounded chart and frame metadata, or a diagnostic preserving all inputs.
    pub fn preview_frame(
        &self,
        options: &QString,
        position: i32,
        start: i32,
        count: i32,
    ) -> QString {
        let result = (|| {
            let options: Options = parse_selection(&options.to_string())?;
            let position = usize::try_from(position).map_err(|_| "Invalid capture position.")?;
            let start = usize::try_from(start).map_err(|_| "Invalid viewport start.")?;
            let count = usize::try_from(count).map_err(|_| "Invalid viewport count.")?;
            let mut value = self
                .rust()
                .model
                .preview_frame(&options, position, start, count)?;
            let profile = crate::workbench::session::snapshot().profile;
            if let Some(capture) = value["source_index"]
                .as_u64()
                .and_then(|index| self.rust().model.draft_rx.get(index as usize))
            {
                super::super::inputs::measurement_input::plc::annotate(
                    &mut value,
                    &capture.key().grid,
                    profile
                        .as_ref()
                        .filter(|profile| &profile.key == capture.key()),
                );
            }
            Ok::<_, String>(value)
        })();
        QString::from(
            result
                .unwrap_or_else(|error| json!({"error":error,"points":[],"frame_count":0}))
                .to_string(),
        )
    }

    /// Queries how many checked captures fit the accepted CWB observation window.
    /// # Arguments
    /// * `options` - Draft identities and controls, at most 24 KiB.
    /// * `settings`, `run_options` - Accepted bounded OCD/CWB documents.
    /// # Returns
    /// Scalar counts, capacity and timing advice without publishing any state.
    pub fn selection_status(
        &self,
        options: &QString,
        settings: &QString,
        run_options: &QString,
    ) -> QString {
        let result = (|| -> Result<Value, String> {
            let options: Options = parse_selection(&options.to_string())?;
            let selection = options.selected(&self.rust().model.draft_rx);
            let selection_error = selection.as_ref().err().cloned();
            let selected = selection
                .as_ref()
                .map(|captures| captures.len())
                .unwrap_or(0);
            let loaded = self.rust().model.draft_rx.len();
            let run: Option<crate::simulation::closed_loop::Options> =
                parse(&run_options.to_string())?;
            let Some(run) = run else {
                return Ok(
                    json!({"loaded":loaded,"selected":selected,"capacity":null,"error":selection_error,"fits":selected>0 && (options.mode != docsis_ofdm_fec::simulation::channel::measurement_playback::PlaybackMode::Single || selected==1),"summary":format!("{selected} selected / {loaded} loaded · Configure WorkBench to see replay capacity.")}),
                );
            };
            let settings = parse::<TransmissionSettings>(&settings.to_string())?;
            let capacity = options.capacity(&settings, run.duration_ms)?;
            let symbols = settings
                .duration_timing(std::time::Duration::from_millis(run.duration_ms))
                .map_err(|error| error.to_string())?
                .complete_symbols;
            let fits = selected > 0 && selected <= capacity;
            let temporal = matches!(options.mode, docsis_ofdm_fec::simulation::channel::measurement_playback::PlaybackMode::Hold | docsis_ofdm_fec::simulation::channel::measurement_playback::PlaybackMode::Interpolate);
            let summary = if temporal {
                format!("{selected} selected / {loaded} loaded · CWB: {} ms per profile ({symbols} symbols) · Up to {capacity} captures at {} symbols apart.{}",run.duration_ms,options.interval_symbols,if fits {""} else {" Select fewer captures, reduce the interval or increase CWB duration."})
            } else {
                format!("{selected} selected / {loaded} loaded · {}",if options.mode == docsis_ofdm_fec::simulation::channel::measurement_playback::PlaybackMode::Single {"Single Shot requires exactly one capture."} else {"Average combines selected captures; no temporal replay."})
            };
            Ok(
                json!({"error":selection_error,"loaded":loaded,"selected":selected,"capacity":capacity,"fits":fits,"observation_symbols":symbols,"duration_ms":run.duration_ms,"summary":summary}),
            )
        })();
        QString::from(
            result
                .unwrap_or_else(|error| json!({"error":error,"fits":false}))
                .to_string(),
        )
    }

    /// Adopts original captures/options after complete project validation.
    pub fn reload(mut self: core::pin::Pin<&mut Self>) {
        if let Some(model) =
            crate::workbench::project_load::take(|prepared| prepared.measurements.take())
        {
            self.as_mut().rust_mut().model = model;
        } else if let Ok(model) =
            Model::from_saved(&crate::workbench::session::saved("measurements"))
        {
            self.as_mut().rust_mut().model = model;
        }
    }

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
            crate::platform::native_file_picker::choose_many("Open RxMER Captures")
        } else {
            crate::platform::native_file_picker::choose("Open Channel Response Capture")
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
        let result = parse_selection::<Options>(&options.to_string()).and_then(|options| {
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
        let profile = crate::workbench::session::snapshot().profile;
        let result = parse_selection::<ChartRequest>(&request.to_string()).and_then(|request| {
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
        let result = parse_selection::<Options>(&options.to_string()).and_then(|options| {
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
            if rxmer {
                let profile = crate::workbench::session::snapshot().profile;
                let published = model.published(profile.as_ref())?;
                crate::workbench::session::set_playback(published.playback);
            } else {
                crate::workbench::session::set_response(model.accepted_response.as_ref().and_then(
                    |capture| match &capture.measurement {
                        document::Measurement::Response(frame) => Some(frame.as_ref().clone()),
                        _ => None,
                    },
                ));
            }
            Ok(())
        });
        if result.is_ok() {
            crate::workbench::session::put_saved("measurements", self.rust().model.export());
        }
        QString::from(result.err().unwrap_or_default())
    }
    /// Returns accepted additional-loss parameters for a new editor draft.
    /// # Returns
    /// Small JSON configuration without original measurement arrays.
    pub fn loss_current(&self) -> QString {
        QString::from(
            serde_json::to_string(&crate::workbench::session::snapshot().additional_loss)
                .unwrap_or_default(),
        )
    }
    /// Validates and previews a draft through the shared Rust channel planner.
    /// # Arguments
    /// * `document` - Bounded model JSON.
    /// * `settings` - Accepted OCD JSON.
    /// # Returns
    /// Bounded gain chart or error, without changing accepted state.
    pub fn loss_preview(&self, document: &QString, settings: &QString) -> QString {
        let result = parse(&document.to_string()).and_then(|loss| {
            crate::inputs::channel_model::preview(&loss, &parse(&settings.to_string())?)
        });
        QString::from(
            result
                .unwrap_or_else(|error| json!({"error":error,"points":[]}))
                .to_string(),
        )
    }
    /// Commits a valid additional response and its versioned persistence section.
    /// # Arguments
    /// * `document` - Bounded model JSON.
    /// * `settings` - Accepted OCD JSON.
    /// # Returns
    /// Empty on success, diagnostic preserving accepted state otherwise.
    pub fn loss_apply(
        self: core::pin::Pin<&mut Self>,
        document: &QString,
        settings: &QString,
    ) -> QString {
        let result = parse(&document.to_string()).and_then(|loss| {
            crate::inputs::channel_model::preview(&loss, &parse(&settings.to_string())?)?;
            crate::workbench::session::put_saved(
                "channel_model",
                crate::inputs::channel_model::saved(&loss),
            );
            crate::workbench::session::set_additional_loss(loss);
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
        let profile = crate::workbench::session::snapshot().profile;
        let settings = parse::<TransmissionSettings>(&settings.to_string());
        let model = &self.rust().model;
        if let Ok(inputs) = model.published(profile.as_ref()) {
            crate::workbench::session::set_playback(inputs.playback);
            crate::workbench::session::set_response(inputs.response);
        }
        QString::from(
            model
                .state(settings, &crate::workbench::session::snapshot())
                .to_string(),
        )
    }
}

impl bridge::MeasurementInputs {
    /// Returns accepted electrical settings without capture arrays.
    /// # Returns
    /// Typed scalar JSON; missing legacy settings use core defaults.
    pub fn power_current(&self) -> QString {
        QString::from(
            serde_json::to_string(&crate::workbench::session::snapshot().power).unwrap_or_default(),
        )
    }
    /// Commits a bounded validated power draft and its saved section atomically.
    /// # Arguments
    /// * `document` - At most4096 bytes of scalar configuration JSON.
    /// # Returns
    /// Empty on success, diagnostic on failure without changing accepted state.
    pub fn power_apply(self: core::pin::Pin<&mut Self>, document: &QString) -> QString {
        let result =
            parse::<docsis_ofdm_fec::signal_processing::channel::power::PowerConditioning>(
                &document.to_string(),
            )
            .and_then(|power| {
                power.validate().map_err(|e| e.to_string())?;
                crate::workbench::session::put_saved(
                    "channel_power",
                    crate::inputs::channel_model::saved_power(&power),
                );
                crate::workbench::session::set_power(power);
                Ok(())
            });
        QString::from(result.err().unwrap_or_default())
    }
    /// Returns immutable completed-run availability for one channel TapPoint.
    /// # Arguments
    /// * `tap` - Before amp0, after amp1 or receiver input2.
    /// # Returns
    /// Bounded identity/readiness JSON with no implicit simulation dispatch.
    pub fn tap_state(&self, tap: i32) -> QString {
        QString::from(crate::instruments::channel::state(tap).to_string())
    }
    /// Queries the shared spectrum estimator at a frozen channel boundary.
    /// # Arguments
    /// * `tap` - Channel boundary0..2.
    /// * `start` - First display bin.
    /// * `count` - Display width; zero means full span.
    /// * `capture_axis` - Select physical frequency labels.
    /// # Returns
    /// Bounded viewport; Channel Power is integrated independently of zoom.
    pub fn tap_spectrum(&self, tap: i32, start: i32, count: i32, capture_axis: bool) -> QString {
        QString::from(
            crate::instruments::channel::spectrum(tap, start, count, capture_axis).to_string(),
        )
    }
    /// Queries raw channel I/Q through the common QAM presentation provider.
    /// # Arguments
    /// * `tap` - Channel boundary0..2.
    /// * `query` - Bounded bin/role selection JSON.
    /// # Returns
    /// Bounded points with original coordinates, or explicit unavailable state.
    pub fn tap_qam(&self, tap: i32, query: &QString) -> QString {
        QString::from(crate::instruments::channel::qam(tap, &query.to_string()).to_string())
    }
}
