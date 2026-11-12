/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shared editor state; accepted measurements are independent of transient drafts.
use super::document::{Capture, Measurement, MAX_TOTAL_BYTES};
use docsis_ofdm_fec::{
    profile::candidate::CapturedProfile,
    scenario::{
        measurement_view::{measurement_view, response_value, ResponseMetric},
        FrequencyGrid,
    },
    simulation::channel::measurement_playback::{
        validate_response, MeasurementPlayback, PlaybackMode, MAX_PLAYBACK_FRAMES,
        MAX_PLAYBACK_VALUES,
    },
    transmission::TransmissionSettings,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Options {
    pub mode: PlaybackMode,
    pub interval_symbols: String,
    pub tolerance_seconds: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_captures: Option<Vec<String>>,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            mode: PlaybackMode::Single,
            interval_symbols: "1000".into(),
            tolerance_seconds: 90000,
            selected_captures: None,
        }
    }
}
impl Options {
    /// Resolves checked original identities in displayed playback order.
    /// # Arguments
    /// * `captures` - Current ordered collection, retaining unselected originals.
    /// # Returns
    /// References to the selected captures; legacy options select all.
    /// # Errors
    /// Rejects empty, duplicate, unknown or over-limit explicit selections.
    pub(crate) fn selected<'input>(
        &self,
        captures: &'input [Capture],
    ) -> Result<Vec<&'input Capture>, String> {
        let Some(selected) = &self.selected_captures else {
            return Ok(captures.iter().collect());
        };
        if selected.is_empty() || selected.len() > MAX_PLAYBACK_FRAMES {
            return Err("Select at least one RxMER capture (maximum 256).".into());
        }
        let identities = selected.iter().collect::<std::collections::HashSet<_>>();
        if identities.len() != selected.len() {
            return Err("Duplicate RxMER selection.".into());
        }
        let captures = captures
            .iter()
            .filter(|capture| identities.contains(&capture.hash))
            .collect::<Vec<_>>();
        if captures.len() != selected.len() {
            return Err("Selected RxMER capture is not in this collection.".into());
        }
        Ok(captures)
    }

    /// Calculates the number of captures reachable under accepted CWB timing.
    /// # Arguments
    /// * `settings` - Accepted OCD geometry.
    /// * `duration_ms` - Accepted duration per profile, not a whole sweep.
    /// # Returns
    /// Temporal replay capacity, or the static-mode resource limit.
    /// # Errors
    /// Rejects invalid duration/timing, zero or overflowing replay intervals.
    pub(crate) fn capacity(
        &self,
        settings: &TransmissionSettings,
        duration_ms: u64,
    ) -> Result<usize, String> {
        if !(1..=crate::simulation::closed_loop::MAX_DURATION_MS).contains(&duration_ms) {
            return Err("Configure a supported duration per profile.".into());
        }
        let symbols = settings
            .duration_timing(std::time::Duration::from_millis(duration_ms))
            .map_err(|error| error.to_string())?
            .complete_symbols;
        match self.mode {
            PlaybackMode::Single => Ok(1),
            PlaybackMode::Average => Ok(MAX_PLAYBACK_FRAMES),
            _ => {
                let interval = self
                    .interval_symbols
                    .parse::<u64>()
                    .map_err(|_| "Enter an integer OFDM-symbol interval.")?;
                if interval == 0 {
                    return Err("Replay interval must be greater than zero.".into());
                }
                Ok(((symbols.saturating_sub(1) / interval).saturating_add(1))
                    .min(MAX_PLAYBACK_FRAMES as u64) as usize)
            }
        }
    }
}
pub(crate) struct Model {
    pub draft_rx: Vec<Capture>,
    pub accepted_rx: Vec<Capture>,
    pub draft_response: Option<Capture>,
    pub accepted_response: Option<Capture>,
    pub options: Options,
    pub response_tolerance: u32,
}
impl Default for Model {
    fn default() -> Self {
        Self {
            draft_rx: Vec::new(),
            accepted_rx: Vec::new(),
            draft_response: None,
            accepted_response: None,
            options: Options::default(),
            response_tolerance: 90000,
        }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ChartRequest {
    pub index: i32,
    pub metric: i32,
    pub symbol: String,
    pub start: usize,
    pub count: usize,
    pub options: Options,
}
impl Model {
    /// Derives scalar readiness from independent accepted inputs.
    /// # Arguments
    /// * `settings` - Parsed OCD or its validation error.
    /// * `inputs` - Frozen accepted inputs; no global publication occurs.
    /// # Returns
    /// Bounded RxMER, response and channel status for the workbench.
    pub(crate) fn state(
        &self,
        settings: Result<TransmissionSettings, String>,
        inputs: &crate::workbench::session::Inputs,
    ) -> Value {
        let profile = inputs.profile.as_ref();
        let rx = settings
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|settings| {
                Model::playback(&self.accepted_rx, &self.options, settings, profile).map(|_| ())
            });
        let response = settings
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|settings| {
                Model::response(
                    self.accepted_response.as_ref(),
                    settings,
                    profile,
                    self.response_tolerance,
                )
            });
        let rx_status = status(rx.clone(), profile.is_none() || self.accepted_rx.is_empty());
        let response_status = status(response.clone(), profile.is_none());
        let loss = settings
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|settings| {
                crate::inputs::channel_model::preview_inputs(
                    &inputs.additional_loss,
                    settings,
                    inputs,
                )
                .map(|_| ())
            });
        let channel = if rx.is_ok() && response.is_ok() && loss.is_ok() {
            json!({"status":"complete","reason":"RxMER, channel response and additional loss are configured."})
        } else {
            json!({"status":if rx_status["status"]=="error"||response_status["status"]=="error"||(profile.is_some() && loss.is_err()) {"error"} else {"attention"},"reason":format!("{} {} {}",rx.err().unwrap_or_default(),response.err().unwrap_or_default(),loss.err().unwrap_or_default())})
        };
        json!({"rxmer":rx_status,"response":response_status,"channel":channel,"flat":self.accepted_response.is_none(),"additional_loss":inputs.additional_loss})
    }

    /// Serializes accepted originals and controls, excluding drafts and result arrays.
    /// # Returns
    /// Self-contained Rust-owned persistence section.
    pub(crate) fn export(&self) -> Value {
        json!({"rxmer":self.accepted_rx.iter().map(|c|crate::workbench::project_file::encode(&c.name,&c.original)).collect::<Vec<_>>(),
            "response":self.accepted_response.as_ref().map(|c|crate::workbench::project_file::encode(&c.name,&c.original)),
            "options":self.options,"response_tolerance":self.response_tolerance})
    }
    /// Reconstructs originals and options from a bounded project section.
    /// # Arguments
    /// * `value` - Persisted accepted state, null for untouched defaults.
    /// # Returns
    /// Independent editor model with exact original capture bytes.
    /// # Errors
    /// Rejects invalid source types, hashes, options or combined allocation limits.
    pub(crate) fn from_saved(value: &Value) -> Result<Self, String> {
        if value.is_null() {
            return Ok(Self::default());
        }
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Saved {
            rxmer: Vec<Value>,
            response: Option<Value>,
            options: Options,
            response_tolerance: u32,
        }
        let saved: Saved = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
        if saved.rxmer.len() > MAX_PLAYBACK_FRAMES {
            return Err("Too many saved RxMER inputs".into());
        }
        saved
            .options
            .interval_symbols
            .parse::<u64>()
            .map_err(|_| "Invalid saved playback interval")?;
        let mut model = Self::default();
        for file in saved.rxmer {
            let (name, bytes) = crate::workbench::project_file::decode(&file)?;
            model.stage(vec![Capture::decode(name, bytes, true)?], true)?;
        }
        if let Some(file) = saved.response {
            let (name, bytes) = crate::workbench::project_file::decode(&file)?;
            model.stage(vec![Capture::decode(name, bytes, false)?], false)?;
        }
        model.accepted_rx = model.draft_rx.clone();
        model.accepted_response = model.draft_response.clone();
        model.options = saved.options;
        model.options.selected(&model.accepted_rx)?;
        model.response_tolerance = saved.response_tolerance;
        Ok(model)
    }
    /// Resolves accepted original captures without hiding cross-input readiness failures.
    /// # Arguments
    /// * `profile` - Current accepted DPD, optional during incomplete project editing.
    /// # Returns
    /// Typed worker inputs; response None explicitly means unity.
    /// # Errors
    /// Rejects an invalid stored interval; cross-input validation occurs on run admission.
    pub(crate) fn published(
        &self,
        profile: Option<&CapturedProfile>,
    ) -> Result<super::RestoredMeasurements, String> {
        let frames = self
            .options
            .selected(&self.accepted_rx)?
            .into_iter()
            .filter_map(|c| match &c.measurement {
                Measurement::Rxmer(frame) => Some(frame.as_ref().clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        let playback = if let Some(profile) = profile.filter(|_| !frames.is_empty()) {
            Some(
                docsis_ofdm_fec::simulation::channel::measurement_playback::PlaybackConfiguration {
                    profile: profile.clone(),
                    frames,
                    mode: self.options.mode,
                    interval_symbols: self
                        .options
                        .interval_symbols
                        .parse()
                        .map_err(|_| "Invalid interval")?,
                    tolerance_seconds: self.options.tolerance_seconds,
                },
            )
        } else {
            None
        };
        let response = self
            .accepted_response
            .as_ref()
            .and_then(|c| match &c.measurement {
                Measurement::Response(frame) => Some(frame.as_ref().clone()),
                _ => None,
            });
        Ok(super::RestoredMeasurements { playback, response })
    }
    /// Starts a fresh draft from accepted immutable input snapshots.
    /// # Arguments
    /// * `rxmer` - Selects RxMER or response editor.
    pub(crate) fn begin(&mut self, rxmer: bool) {
        if rxmer {
            self.draft_rx = self.accepted_rx.clone();
        } else {
            self.draft_response = self.accepted_response.clone();
        }
    }
    /// Atomically stages a decoded batch; failed inputs never partially replace drafts.
    /// # Arguments
    /// * `captures` - Whole selected batch, already decoded.
    /// * `rxmer` - RxMER permits append; channel response replaces one input.
    /// # Returns
    /// Unit after resource/duplicate checks.
    /// # Errors
    /// Rejects duplicate captures, too many files, bytes or decoded samples.
    pub(crate) fn stage(&mut self, captures: Vec<Capture>, rxmer: bool) -> Result<(), String> {
        let mut next = if rxmer {
            self.draft_rx.clone()
        } else {
            Vec::new()
        };
        next.extend(captures);
        if next.len() > MAX_PLAYBACK_FRAMES
            || (!rxmer && next.len() != 1)
            || next
                .iter()
                .map(|capture| capture.original.len())
                .sum::<usize>()
                > MAX_TOTAL_BYTES
            || next
                .iter()
                .map(|capture| capture.key().grid.sample_count)
                .sum::<usize>()
                > MAX_PLAYBACK_VALUES
        {
            return Err("Selection exceeds 256 files, 64 MiB or 4 million carrier values.".into());
        }
        let mut hashes = std::collections::HashSet::new();
        if next
            .iter()
            .any(|capture| !hashes.insert(capture.hash.clone()))
        {
            return Err("Duplicate capture already selected; originals are counted once.".into());
        }
        if rxmer {
            self.draft_rx = next;
        } else {
            self.draft_response = next.pop();
        }
        Ok(())
    }
    /// Validates one exact ordered RxMER playback plan through the shared core.
    /// # Arguments
    /// * `captures` - Original typed inputs.
    /// * `options` - Mode and explicit association/playback controls.
    /// * `settings` - Accepted OCD.
    /// * `profile` - Accepted DPD; absence blocks acceptance.
    /// # Returns
    /// Immutable scientific playback plan without Qt data.
    /// # Errors
    /// Rejects absent DPD, bad settings or any core association/schedule error.
    pub(crate) fn playback(
        captures: &[Capture],
        options: &Options,
        settings: &TransmissionSettings,
        profile: Option<&CapturedProfile>,
    ) -> Result<MeasurementPlayback, String> {
        let profile = profile.ok_or("Apply a compatible DPD before accepting RxMER.")?;
        let frames = options
            .selected(captures)?
            .into_iter()
            .map(|capture| match &capture.measurement {
                Measurement::Rxmer(frame) => Ok(frame.as_ref().clone()),
                _ => Err("Expected RxMER input.".to_string()),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let interval = options
            .interval_symbols
            .parse::<u64>()
            .map_err(|_| "Enter an integer OFDM-symbol interval.".to_string())?;
        MeasurementPlayback::new(
            settings,
            profile,
            frames,
            options.mode,
            interval,
            options.tolerance_seconds,
        )
        .map_err(|error| error.to_string())
    }
    /// Validates flat/unity or imported response against the accepted profile.
    /// # Arguments
    /// * `capture` - None explicitly selects synthetic unity.
    /// * `settings` - Accepted transmission geometry.
    /// * `profile` - Accepted profile identity/coverage.
    /// * `tolerance` - Explicit capture association window in seconds.
    /// # Returns
    /// Unit for supported H, never reciprocal equalizer coefficients.
    /// # Errors
    /// Rejects missing DPD, wrong input type or incompatible response/epoch.
    pub(crate) fn response(
        capture: Option<&Capture>,
        settings: &TransmissionSettings,
        profile: Option<&CapturedProfile>,
        tolerance: u32,
    ) -> Result<(), String> {
        let profile = profile.ok_or("Apply a compatible DPD to define the channel grid.")?;
        let response = match capture.map(|capture| &capture.measurement) {
            Some(Measurement::Response(frame)) => {
                if frame.capture_time.abs_diff(profile.capture_time) > tolerance {
                    return Err(
                        "Channel response capture exceeds association tolerance from DPD.".into(),
                    );
                }
                Some(frame.as_ref())
            }
            Some(_) => return Err("Expected a channel-response capture.".into()),
            None => None,
        };
        validate_response(settings, profile, response).map_err(|error| error.to_string())
    }
    /// Lists bounded original-file metadata and compatibility diagnostics.
    /// # Arguments
    /// * `rxmer` - Which draft list to inspect.
    /// * `profile` - Optional accepted association target.
    /// # Returns
    /// File rows without full samples or original bytes.
    pub(crate) fn files(&self, rxmer: bool, profile: Option<&CapturedProfile>) -> Value {
        let captures = if rxmer {
            self.draft_rx.iter().collect::<Vec<_>>()
        } else {
            self.draft_response.iter().collect()
        };
        json!(captures.iter().enumerate().map(|(index,capture)| {
            let grid=&capture.key().grid;
            let compatible=profile.map(|profile|profile.key==*capture.key());
            json!({"index":index,"name":capture.name,"format":capture.format,"bytes":capture.original.len(),"sha256":capture.hash,"decoded_sha256":capture.decoded_hash,"capture_time":capture.epoch(),"channel":capture.key().channel_id,"mac":capture.key().mac_address.iter().map(|octet|format!("{octet:02x}")).collect::<Vec<_>>().join(":"),"frequency":format!("{:.3}–{:.3} MHz · {} Hz · {} carriers",f64::from(grid.frequency(0).unwrap_or(0))/1e6,f64::from(grid.frequency(grid.sample_count-1).unwrap_or(0))/1e6,grid.spacing_hz,grid.sample_count),"compatible":compatible,"reason":match compatible {Some(true)=>"Exact DPD grid/identity match",Some(false)=>"Device/channel/frequency grid differs from DPD",None=>"Apply DPD to check compatibility"}})
        }).collect::<Vec<_>>())
    }
    /// Produces a shared bounded chart for an original capture or effective playback.
    /// # Arguments
    /// * `rxmer` - RxMER or response projection.
    /// * `request` - Source index/projection, preview symbol, viewport and draft options.
    /// * `settings` - Accepted OCD.
    /// * `profile` - Optional accepted DPD.
    /// # Returns
    /// Compact chart JSON; missing values remain gaps.
    /// # Errors
    /// Reports unavailable inputs, invalid projection/window or core errors.
    pub(crate) fn chart(
        &self,
        rxmer: bool,
        request: &ChartRequest,
        settings: &TransmissionSettings,
        profile: Option<&CapturedProfile>,
    ) -> Result<Value, String> {
        let ChartRequest {
            index,
            metric,
            symbol,
            start,
            count,
            options,
        } = request;
        let (index, metric, start, count) = (*index, *metric, *start, *count);
        let symbol = symbol
            .parse::<u64>()
            .map_err(|_| "Enter an integer preview symbol.")?;
        let (grid, values, units): (FrequencyGrid, Vec<Option<f64>>, &str) = if rxmer {
            if index < 0 {
                let plan = Self::playback(&self.draft_rx, options, settings, profile)?;
                let snapshot = plan.at(symbol);
                (
                    plan.frames()[0].key.grid.clone(),
                    snapshot
                        .power
                        .iter()
                        .map(|power| {
                            power.and_then(|value| {
                                let mer = -10.0 * value.log10();
                                mer.is_finite().then_some(mer)
                            })
                        })
                        .collect(),
                    "Equivalent RxMER (dB)",
                )
            } else {
                let capture = self
                    .draft_rx
                    .get(index as usize)
                    .ok_or("Choose an RxMER capture.")?;
                let Measurement::Rxmer(frame) = &capture.measurement else {
                    return Err("Expected RxMER.".into());
                };
                (frame.key.grid.clone(), frame.mer_db.clone(), "RxMER (dB)")
            }
        } else {
            let metric = match metric {
                0 => ResponseMetric::GainDb,
                1 => ResponseMetric::PhaseDegrees,
                2 => ResponseMetric::Real,
                3 => ResponseMetric::Imaginary,
                _ => return Err("Invalid response projection.".into()),
            };
            let units = match metric {
                ResponseMetric::GainDb => "Channel gain (dB)",
                ResponseMetric::PhaseDegrees => "Wrapped phase (degrees)",
                ResponseMetric::Real => "H real (linear)",
                ResponseMetric::Imaginary => "H imaginary (linear)",
            };
            if let Some(capture) = &self.draft_response {
                let Measurement::Response(frame) = &capture.measurement else {
                    return Err("Expected response.".into());
                };
                (
                    frame.key.grid.clone(),
                    frame
                        .iq
                        .iter()
                        .map(|sample| response_value(*sample, metric))
                        .collect(),
                    units,
                )
            } else {
                let grid = profile
                    .ok_or("Apply DPD to view the flat channel grid.")?
                    .key
                    .grid
                    .clone();
                let values = vec![response_value(Some([1.0, 0.0]), metric); grid.sample_count];
                (grid, values, units)
            }
        };
        let view = measurement_view(&grid, &values, start, count, units)
            .map_err(|error| error.to_string())?;
        serde_json::to_value(view).map_err(|error| error.to_string())
    }
}

fn status(result: Result<(), String>, empty: bool) -> Value {
    match result {
        Ok(()) => json!({"status":"complete","reason":"Validated configuration accepted."}),
        Err(error) => json!({"status":if empty {"attention"} else {"error"},"reason":error}),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use docsis_ofdm_fec::scenario::pnm_json::{import_profiles, JsonCapture};
    fn profile() -> CapturedProfile {
        import_profiles(JsonCapture {
            name: "dpd",
            bytes: include_bytes!(
                "../../../../../examples/measurements/single-channel/modulation_profile.json"
            ),
        })
        .unwrap()
        .remove(0)
    }
    fn baseline() -> Capture {
        Capture::decode(
            "rxmer.bin".into(),
            include_bytes!(
                "../../../../../examples/measurements/rxmer-fec-stress/rxmer-baseline.bin"
            )
            .to_vec(),
            true,
        )
        .unwrap()
    }
    #[test]
    fn selection_keeps_originals_and_round_trips_a_single_shot() {
        let mut model = Model::default();
        let first = baseline();
        let second = Capture::decode(
            "notch.json".into(),
            include_bytes!(
                "../../../../../examples/measurements/single-channel/rxmer-center-notch-12db.json"
            )
            .to_vec(),
            true,
        )
        .unwrap();
        let second_hash = second.hash.clone();
        model.stage(vec![first, second], true).unwrap();
        model.accepted_rx = model.draft_rx.clone();
        model.options.selected_captures = Some(vec![second_hash.clone()]);
        let restored = Model::from_saved(&model.export()).unwrap();
        assert_eq!(restored.accepted_rx.len(), 2);
        assert_eq!(
            restored.options.selected_captures,
            Some(vec![second_hash.clone()])
        );
        let profile = profile();
        let published = restored
            .published(Some(&profile))
            .unwrap()
            .playback
            .unwrap();
        assert_eq!(published.frames.len(), 1);
        assert_eq!(
            restored.options.selected(&restored.accepted_rx).unwrap()[0].hash,
            second_hash
        );
        Model::playback(
            &restored.accepted_rx,
            &restored.options,
            &TransmissionSettings::default(),
            Some(&profile),
        )
        .unwrap();
        let all = Options {
            mode: PlaybackMode::Hold,
            selected_captures: Some(
                restored
                    .accepted_rx
                    .iter()
                    .rev()
                    .map(|capture| capture.hash.clone())
                    .collect(),
            ),
            ..Default::default()
        };
        assert_eq!(
            all.selected(&restored.accepted_rx).unwrap()[0].hash,
            restored.accepted_rx[0].hash
        );
        assert_eq!(
            Options::default()
                .selected(&restored.accepted_rx)
                .unwrap()
                .len(),
            2
        );
        for identities in [
            vec![],
            vec!["missing".into()],
            vec![second_hash.clone(), second_hash],
        ] {
            let options = Options {
                selected_captures: Some(identities),
                ..Default::default()
            };
            assert!(options.selected(&restored.accepted_rx).is_err());
        }
    }

    #[test]
    fn replay_capacity_uses_strict_observation_boundary_and_mode() {
        let settings = TransmissionSettings::default();
        let symbols = settings
            .duration_timing(std::time::Duration::from_millis(1))
            .unwrap()
            .complete_symbols;
        assert!(symbols > 1);
        let mut options = Options {
            mode: PlaybackMode::Hold,
            interval_symbols: (symbols - 1).to_string(),
            ..Default::default()
        };
        assert_eq!(options.capacity(&settings, 1).unwrap(), 2);
        options.interval_symbols = symbols.to_string();
        assert_eq!(options.capacity(&settings, 1).unwrap(), 1);
        options.interval_symbols = "0".into();
        assert!(options.capacity(&settings, 1).is_err());
        options.interval_symbols = u64::MAX.to_string();
        assert_eq!(options.capacity(&settings, 1).unwrap(), 1);
        options.mode = PlaybackMode::Single;
        assert_eq!(options.capacity(&settings, 1).unwrap(), 1);
        options.mode = PlaybackMode::Average;
        assert_eq!(options.capacity(&settings, 1).unwrap(), MAX_PLAYBACK_FRAMES);
        assert!(options.capacity(&settings, 0).is_err());
    }
    #[test]
    fn binary_json_shared_viewer_and_atomic_drafts_preserve_original_inputs() {
        let profile = profile();
        let settings = TransmissionSettings::default();
        let mut model = Model::default();
        let capture = baseline();
        let original = capture.original.clone();
        model.stage(vec![capture.clone()], true).unwrap();
        assert!(model.stage(vec![capture], true).is_err());
        assert_eq!(model.draft_rx.len(), 1);
        let options = Options::default();
        Model::playback(&model.draft_rx, &options, &settings, Some(&profile)).unwrap();
        model.accepted_rx = model.draft_rx.clone();
        model.draft_rx.clear();
        model.begin(true);
        assert_eq!(model.draft_rx[0].original.as_ref(), original.as_ref());
        assert_eq!(model.draft_rx[0].format, "Binary PNM");
        let request = ChartRequest {
            index: -1,
            metric: 0,
            symbol: "0".into(),
            start: 0,
            count: 0,
            options,
        };
        let chart = model
            .chart(true, &request, &settings, Some(&profile))
            .unwrap();
        assert!(chart["points"].as_array().unwrap().len() <= 1024);
        let mut wrong = profile.clone();
        wrong.key.grid.zero_frequency_hz += 25000;
        assert!(
            Model::playback(&model.accepted_rx, &model.options, &settings, Some(&wrong)).is_err()
        );
        Model::response(None, &settings, Some(&profile), 90000).unwrap();
        let flat = model
            .chart(false, &request, &settings, Some(&profile))
            .unwrap();
        assert_eq!(flat["axis_label"], "Channel gain (dB)");
        let response = Capture::decode(
            "channel.json".into(),
            include_bytes!(
                "../../../../../examples/measurements/single-channel/channel_estimation.json"
            )
            .to_vec(),
            false,
        )
        .unwrap();
        model.stage(vec![response], false).unwrap();
        Model::response(
            model.draft_response.as_ref(),
            &settings,
            Some(&profile),
            90000,
        )
        .unwrap();
        assert_eq!(
            model
                .chart(false, &request, &settings, Some(&profile))
                .unwrap()["total"],
            7480
        );
        assert!(Capture::decode("wrong.bin".into(), original.as_ref().clone(), false).is_err());
        assert!(Capture::decode("text.bin".into(), b"not a PNM file".to_vec(), true).is_err());
    }
    #[test]
    fn combined_modes_and_render_requests_reject_invalid_controls() {
        let profile = profile();
        let settings = TransmissionSettings::default();
        let mut model = Model::default();
        model.stage(vec![baseline()], true).unwrap();
        model.stage(vec![Capture::decode("notch.json".into(),include_bytes!("../../../../../examples/measurements/single-channel/rxmer-center-notch-12db.json").to_vec(),true).unwrap()],true).unwrap();
        assert!(
            Model::playback(&model.draft_rx, &model.options, &settings, Some(&profile)).is_err()
        );
        let options = Options {
            mode: PlaybackMode::Interpolate,
            ..Default::default()
        };
        let plan = Model::playback(&model.draft_rx, &options, &settings, Some(&profile)).unwrap();
        assert_eq!(plan.span(), (1000, 42.5));
        let request = ChartRequest {
            index: -1,
            metric: 0,
            symbol: "500".into(),
            start: 0,
            count: 0,
            options,
        };
        assert!(
            model
                .chart(true, &request, &settings, Some(&profile))
                .unwrap()["points"]
                .as_array()
                .unwrap()
                .len()
                <= 1024
        );
        let bad = Options {
            interval_symbols: "0".into(),
            mode: PlaybackMode::Interpolate,
            ..Default::default()
        };
        assert!(Model::playback(&model.draft_rx, &bad, &settings, Some(&profile)).is_err());
    }
}
