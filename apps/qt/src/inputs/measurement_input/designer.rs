/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Synthetic RxMER envelopes and immutable designer sessions; input files stay intact.
use super::document::{Capture, Measurement, MAX_INPUT_BYTES, MAX_TOTAL_BYTES};
use crate::help::AdmissionError;
use docsis_ofdm_fec::{
    profile::candidate::CapturedProfile,
    scenario::{measurement_view::measurement_view, RxMerFrame, ScenarioKey, SourceProvenance},
    signal_processing::rxmer_designer::RxmerRecipe,
    types::modulation::CarrierType,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum Baseline {
    Imported {
        original: Value,
    },
    Flat {
        key: ScenarioKey,
        anchor_epoch: u32,
        valid: Vec<bool>,
        baseline_db: f64,
    },
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Record {
    merlin_rxmer_designer: u32,
    baseline: Baseline,
    recipe: RxmerRecipe,
    capture_index: usize,
    capture_count: usize,
}
#[derive(Clone)]
pub(crate) struct Origin {
    record: Record,
    frame: Arc<RxMerFrame>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Request {
    pub source: String,
    pub capture_count: usize,
    pub baseline_db: f64,
    pub recipe: RxmerRecipe,
}
#[derive(Clone)]
pub(crate) struct Session {
    pub hashes: Vec<String>,
    pub profile: Arc<CapturedProfile>,
    origins: Vec<Origin>,
    pub request: Request,
}
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn frame_from_flat(
    key: &ScenarioKey,
    epoch: u32,
    valid: &[bool],
    db: f64,
) -> Result<Arc<RxMerFrame>, String> {
    key.grid.validate().map_err(|error| error.to_string())?;
    if valid.len() != key.grid.sample_count || !db.is_finite() || !(-30.0..=120.0).contains(&db) {
        return Err("Invalid synthetic baseline/grid.".into());
    }
    let bytes = serde_json::to_vec(&(key, epoch, valid, db)).map_err(|error| error.to_string())?;
    Ok(Arc::new(RxMerFrame {
        key: key.clone(),
        capture_time: epoch,
        mer_db: valid.iter().map(|valid| valid.then_some(db)).collect(),
        source: SourceProvenance {
            name: "Synthetic flat RxMER baseline".into(),
            sha256: hash(&bytes),
            byte_length: bytes.len(),
            capture_time: Some(epoch),
            file_kind: "merlin_synthetic_rxmer_baseline".into(),
            format_major: 1,
            format_minor: 0,
        },
    }))
}
fn materialize(record: Record, frame: Arc<RxMerFrame>) -> Result<Capture, AdmissionError> {
    let values = record
        .recipe
        .apply(
            &frame.key.grid,
            &frame.mer_db,
            record.capture_index,
            record.capture_count,
        )
        .map_err(AdmissionError::classified)?;
    let bytes = serde_json::to_vec(&record).map_err(|error| error.to_string())?;
    if bytes.len() > MAX_INPUT_BYTES {
        return Err("Designed capture exceeds 16 MiB.".into());
    }
    let synthetic = matches!(record.baseline, Baseline::Flat { .. });
    let name = format!(
        "{} RxMER {:03}",
        if synthetic { "Synthetic" } else { "Designed" },
        record.capture_index
    );
    let digest = hash(&bytes);
    let mut designed = (*frame).clone();
    designed.mer_db = values;
    designed.source = SourceProvenance {
        name: name.clone(),
        sha256: digest.clone(),
        byte_length: bytes.len(),
        capture_time: Some(frame.capture_time),
        file_kind: if synthetic {
            "merlin_synthetic_rxmer"
        } else {
            "merlin_designed_rxmer"
        }
        .into(),
        format_major: 1,
        format_minor: 0,
    };
    designed.validate().map_err(|error| error.to_string())?;
    let decoded_hash = hash(&serde_json::to_vec(&designed).map_err(|error| error.to_string())?);
    Ok(Capture {
        name,
        original: Arc::new(bytes),
        format: "Designed RxMER",
        hash: digest,
        decoded_hash,
        measurement: Measurement::Rxmer(Arc::new(designed)),
        designer: Some(Arc::new(Origin { record, frame })),
    })
}
/// Detects and validates a native designer envelope before the ordinary PNM decoder.
/// # Arguments
/// * `bytes` - Already bounded input JSON; original embedded captures remain unchanged.
/// # Returns
/// None for ordinary files, or a reconstructed designed capture with baseline/recipe.
/// # Errors
/// Rejects unsupported versions, nesting, invalid provenance and unbounded recipes.
pub(crate) fn decode(bytes: &[u8]) -> Result<Option<Capture>, String> {
    #[derive(Deserialize)]
    struct Header {
        merlin_rxmer_designer: Option<u32>,
    }
    let Ok(header) = serde_json::from_slice::<Header>(bytes) else {
        return Ok(None);
    };
    let Some(version) = header.merlin_rxmer_designer else {
        return Ok(None);
    };
    if version != 1 {
        return Err("Unsupported RxMER Designer format version.".into());
    }
    let record: Record = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let frame = match &record.baseline {
        Baseline::Imported { original } => {
            let (name, bytes) = crate::workbench::project_file::decode(original)?;
            if serde_json::from_slice::<Header>(&bytes)
                .ok()
                .is_some_and(|header| header.merlin_rxmer_designer.is_some())
            {
                return Err("Nested designer baselines are not supported.".into());
            }
            let capture = Capture::decode(name, bytes, true)?;
            let Measurement::Rxmer(frame) = capture.measurement else {
                return Err("Expected RxMER baseline.".into());
            };
            frame
        }
        Baseline::Flat {
            key,
            anchor_epoch,
            valid,
            baseline_db,
        } => frame_from_flat(key, *anchor_epoch, valid, *baseline_db)?,
    };
    let mut capture = materialize(record, frame).map_err(|error| error.to_string())?;
    // Keep the actual imported envelope bytes and hash, including whitespace.
    capture.original = Arc::new(bytes.to_vec());
    capture.hash = hash(bytes);
    if let Measurement::Rxmer(frame) = &mut capture.measurement {
        let frame = Arc::make_mut(frame);
        frame.source.sha256 = capture.hash.clone();
        frame.source.byte_length = bytes.len();
        capture.decoded_hash = hash(&serde_json::to_vec(frame).map_err(|error| error.to_string())?);
    }
    Ok(Some(capture))
}
impl Session {
    /// Snapshots original baselines and restores a common recipe without compounding.
    /// # Arguments
    /// * `captures` - Current parent draft, including optional designed copies.
    /// * `profile` - Accepted modulation profile used for synthetic grids and PLC.
    /// # Returns
    /// Immutable session and bounded controls; differing recipes reset to original data.
    /// # Errors
    /// Rejects non-RxMER inputs or profile/grid mismatches.
    pub(crate) fn begin(
        captures: &[Capture],
        profile: Arc<CapturedProfile>,
    ) -> Result<Self, String> {
        let mut origins = Vec::new();
        for (index, capture) in captures.iter().enumerate() {
            if capture.key() != &profile.key {
                return Err("RxMER Designer requires the exact accepted modulation-profile grid and identity.".into());
            }
            if let Some(origin) = &capture.designer {
                origins.push((**origin).clone());
            } else {
                let Measurement::Rxmer(frame) = &capture.measurement else {
                    return Err("Expected RxMER.".into());
                };
                origins.push(Origin {
                    frame: frame.clone(),
                    record: Record {
                        merlin_rxmer_designer: 1,
                        baseline: Baseline::Imported {
                            original: crate::workbench::project_file::encode(
                                &capture.name,
                                &capture.original,
                            ),
                        },
                        recipe: RxmerRecipe::default(),
                        capture_index: index + 1,
                        capture_count: captures.len(),
                    },
                });
            }
        }
        let common = origins.first().filter(|first| {
            origins.iter().enumerate().all(|(index, origin)| {
                origin.record.recipe == first.record.recipe
                    && match (&origin.record.baseline, &first.record.baseline) {
                        (Baseline::Imported { .. }, Baseline::Imported { .. }) => true,
                        (
                            Baseline::Flat {
                                baseline_db: left, ..
                            },
                            Baseline::Flat {
                                baseline_db: right, ..
                            },
                        ) => left == right,
                        _ => false,
                    }
                    && origin.record.capture_index == index + 1
                    && origin.record.capture_count == origins.len()
            })
        });
        let request = Request {
            source: if common
                .is_some_and(|origin| matches!(origin.record.baseline, Baseline::Flat { .. }))
                || origins.is_empty()
            {
                "flat"
            } else {
                "imported"
            }
            .into(),
            capture_count: origins.len().max(1),
            baseline_db: common
                .and_then(|origin| match origin.record.baseline {
                    Baseline::Flat { baseline_db, .. } => Some(baseline_db),
                    _ => None,
                })
                .unwrap_or(48.0),
            recipe: common
                .map(|origin| origin.record.recipe.clone())
                .unwrap_or_default(),
        };
        Ok(Self {
            hashes: captures
                .iter()
                .map(|capture| capture.hash.clone())
                .collect(),
            profile,
            origins,
            request,
        })
    }
    fn count(&self, request: &Request) -> Result<usize, AdmissionError> {
        let count = match request.source.as_str() {
            "imported" => self.origins.len(),
            "flat" => request.capture_count,
            _ => return Err("Choose imported or flat baseline.".into()),
        };
        if !(1..=256).contains(&count)
            || count
                .checked_mul(self.profile.key.grid.sample_count)
                .is_none_or(|values| values > 4_000_000)
        {
            return Err("Use 1–256 captures and at most 4,000,000 total values.".into());
        }
        request
            .recipe
            .validate(&self.profile.key.grid, count)
            .map_err(AdmissionError::classified)?;
        Ok(count)
    }
    fn origin(&self, request: &Request, index: usize) -> Result<Origin, String> {
        if request.source == "imported" {
            return self
                .origins
                .get(index)
                .cloned()
                .ok_or_else(|| "Choose an available original capture.".into());
        }
        let valid = self
            .profile
            .assignments
            .iter()
            .map(|assignment| {
                !matches!(
                    assignment.carrier_type,
                    CarrierType::Exclusion | CarrierType::Unknown | CarrierType::Reserved
                )
            })
            .collect::<Vec<_>>();
        let baseline = Baseline::Flat {
            key: self.profile.key.clone(),
            anchor_epoch: self.profile.capture_time,
            valid: valid.clone(),
            baseline_db: request.baseline_db,
        };
        let frame = frame_from_flat(
            &self.profile.key,
            self.profile.capture_time,
            &valid,
            request.baseline_db,
        )?;
        Ok(Origin {
            record: Record {
                merlin_rxmer_designer: 1,
                baseline,
                recipe: request.recipe.clone(),
                capture_index: index + 1,
                capture_count: request.capture_count,
            },
            frame,
        })
    }
    /// Produces a shared-scale before/after viewport and profile-derived PLC region.
    /// # Arguments
    /// * `request` - Bounded source controls and recipe.
    /// * `position` - One-based preview capture index.
    /// # Returns
    /// Two bounded curves on the same axis plus scalar collection metadata.
    /// # Errors
    /// Rejects invalid recipes, sources and capture positions without changing drafts.
    pub(crate) fn preview(
        &self,
        request: &Request,
        position: usize,
    ) -> Result<Value, AdmissionError> {
        self.preview_window(request, position, 0, 0)
    }
    /// Produces bounded original/designed curves for one exact source window.
    /// # Arguments
    /// * `request`, `position` - Immutable recipe and one-based capture position.
    /// * `start`, `samples` - Source offsets; zero samples restores full span.
    /// # Returns
    /// Shared-axis curves, clipped PLC band and original-coordinate navigation.
    /// # Errors
    /// Rejects invalid recipes, positions and out-of-grid viewports atomically.
    pub(crate) fn preview_window(
        &self,
        request: &Request,
        position: usize,
        start: usize,
        samples: usize,
    ) -> Result<Value, AdmissionError> {
        let count = self.count(request)?;
        if position == 0 || position > count {
            return Err("Preview capture outside the collection.".into());
        }
        let origin = self.origin(request, position - 1)?;
        let grid = &origin.frame.key.grid;
        let designed = request
            .recipe
            .apply(grid, &origin.frame.mer_db, position, count)
            .map_err(AdmissionError::classified)?;
        let before = measurement_view(grid, &origin.frame.mer_db, start, samples, "RxMER (dB)")
            .map_err(|error| error.to_string())?;
        let after = measurement_view(grid, &designed, start, samples, "RxMER (dB)")
            .map_err(|error| error.to_string())?;
        let window = after.start..after.start + after.count;
        // Reproject both normalized curves onto a common range from original dB values.
        let mut before = serde_json::to_value(before).map_err(|error| error.to_string())?;
        let mut after = serde_json::to_value(after).map_err(|error| error.to_string())?;
        fn limits(values: &[Option<f64>]) -> (f64, f64) {
            let low = values
                .iter()
                .flatten()
                .copied()
                .fold(f64::INFINITY, f64::min);
            let high = values
                .iter()
                .flatten()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max);
            if !low.is_finite() {
                return (-1.0, 1.0);
            }
            let padding = ((high - low) * 0.1).max(1.0);
            (low - padding, high + padding)
        }
        let (before_bottom, before_top) = limits(&origin.frame.mer_db[window.clone()]);
        let (after_bottom, after_top) = limits(&designed[window]);
        let low = before_bottom.min(after_bottom);
        let high = before_top.max(after_top);
        for (view, (bottom, top)) in [
            (&mut before, (before_bottom, before_top)),
            (&mut after, (after_bottom, after_top)),
        ] {
            for point in view["points"].as_array_mut().unwrap() {
                for key in ["y", "minimum_y"] {
                    if let Some(value) = point[key].as_f64() {
                        point[key] = json!((high - (top - value * (top - bottom))) / (high - low));
                    }
                }
            }
        }
        after["y_ticks"]=json!((0..=4).map(|tick|json!({"x":f64::from(tick)/4.0,"label":format!("{:.1}",high-f64::from(tick)/4.0*(high-low))})).collect::<Vec<_>>());
        after["comparison"] = before["points"].clone();
        super::plc::annotate(&mut after, grid, Some(&self.profile));
        let rules = [
            ("Tilt", request.recipe.tilt.as_ref().map(|rule| &rule.scope)),
            (
                "Ripple",
                request.recipe.ripple.as_ref().map(|rule| &rule.scope),
            ),
            (
                "Notch",
                request.recipe.notch.as_ref().map(|rule| &rule.scope),
            ),
        ];
        after["timeline"]=json!(rules.into_iter().filter_map(|(label,scope)|scope.map(|scope| {
            let (first,last)=match scope {docsis_ofdm_fec::signal_processing::rxmer_designer::CaptureScope::All=>(1,count),docsis_ofdm_fec::signal_processing::rxmer_designer::CaptureScope::Range{first,last}=>(*first,*last)};
            json!({"label":label,"first":(first-1) as f64/count as f64,"last":last as f64/count as f64,"summary":format!("{first}–{last} / {count}")})
        })).collect::<Vec<_>>());
        after["capture_count"] = json!(count);
        after["position"] = json!(position);
        after["summary"]=json!(format!("Capture {position}/{count} · dashed original / solid designed · shaded PLC · synthetic MER edits, no RF phase"));
        Ok(after)
    }
    /// Derives a complete batch off the GUI thread with bounded work and cancellation.
    /// # Arguments
    /// * `request` - Validated immutable recipe/source choices.
    /// * `cancel` - Cooperative stop flag, checked between captures.
    /// * `completed` - Actual completed capture count for UI progress.
    /// # Returns
    /// Complete replacement draft, never a partially adopted batch.
    /// # Errors
    /// Rejects resource/recipe errors or cancellation; originals remain unchanged.
    pub(crate) fn build(
        &self,
        request: &Request,
        cancel: &AtomicBool,
        completed: &AtomicUsize,
    ) -> Result<Vec<Capture>, AdmissionError> {
        let count = self.count(request)?;
        let mut captures = Vec::with_capacity(count);
        let mut bytes = 0;
        for index in 0..count {
            if cancel.load(Ordering::Relaxed) {
                return Err("Designer cancelled.".into());
            }
            let mut origin = self.origin(request, index)?;
            origin.record.recipe = request.recipe.clone();
            origin.record.capture_index = index + 1;
            origin.record.capture_count = count;
            let capture = materialize(origin.record, origin.frame)?;
            bytes += capture.original.len();
            if bytes > MAX_TOTAL_BYTES {
                return Err("Designed collection exceeds 64 MiB.".into());
            }
            captures.push(capture);
            completed.store(index + 1, Ordering::Relaxed);
        }
        Ok(captures)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::measurement_input::model::{Model, Options};
    fn fixture() -> (Session, Model) {
        let project: crate::workbench::project_file::Project = serde_json::from_slice(
            include_bytes!("../../../../../examples/workbench/multi-rxmer-2/workbench.merlin.json"),
        )
        .unwrap();
        let (inputs, editors) = project.prepare().unwrap();
        let session = Session::begin(
            &editors.measurements.draft_rx,
            Arc::new(inputs.profile.unwrap()),
        )
        .unwrap();
        (session, editors.measurements)
    }
    #[test]
    fn envelope_roundtrip_reopens_originals_without_compounding() {
        let (mut session, model) = fixture();
        // A small original collection exercises provenance without inflating test data.
        session.origins.truncate(2);
        session.hashes.truncate(2);
        let mut request = session.request.clone();
        request.source = "imported".into();
        request.recipe.tilt = Some(docsis_ofdm_fec::signal_processing::rxmer_designer::Tilt {
            scope: Default::default(),
            tilt_db: -4.0,
        });
        let result = session
            .build(&request, &AtomicBool::new(false), &AtomicUsize::new(0))
            .unwrap();
        assert_ne!(result[0].hash, model.draft_rx[0].hash);
        assert_eq!(result[0].key(), model.draft_rx[0].key());
        assert_eq!(result[0].epoch(), model.draft_rx[0].epoch());
        let imported = Capture::decode(
            result[0].name.clone(),
            result[0].original.as_ref().clone(),
            true,
        )
        .unwrap();
        assert_eq!(imported.hash, result[0].hash);
        assert_eq!(imported.decoded_hash, result[0].decoded_hash);
        let reopened = Session::begin(&result, session.profile.clone()).unwrap();
        assert_eq!(reopened.request.recipe, request.recipe);
        let repeated = reopened
            .build(
                &reopened.request,
                &AtomicBool::new(false),
                &AtomicUsize::new(0),
            )
            .unwrap();
        assert_eq!(repeated[0].hash, result[0].hash);
        let accepted = Model {
            accepted_rx: result.clone(),
            draft_rx: result,
            options: Options {
                mode:
                    docsis_ofdm_fec::simulation::channel::measurement_playback::PlaybackMode::Hold,
                ..Default::default()
            },
            ..Default::default()
        };
        let restored = Model::from_saved(&accepted.export()).unwrap();
        assert_eq!(restored.accepted_rx[0].hash, accepted.accepted_rx[0].hash);
        assert_eq!(model.accepted_rx.len(), 132);
    }
    #[test]
    fn flat_scope_preview_plc_and_cancellation_preserve_inputs() {
        let (session, _) = fixture();
        let mut request = Request {
            source: "flat".into(),
            capture_count: 50,
            baseline_db: 48.0,
            recipe: Default::default(),
        };
        let grid = &session.profile.key.grid;
        request.recipe.notch = Some(docsis_ofdm_fec::signal_processing::rxmer_designer::Notch {
            scope: docsis_ofdm_fec::signal_processing::rxmer_designer::CaptureScope::Range {
                first: 10,
                last: 20,
            },
            center_hz: grid.frequency(grid.sample_count / 2).unwrap(),
            bandwidth_hz: 1_000_000,
            depth_db: 18.0,
            edge_hz: 0,
        });
        let view = session.preview(&request, 15).unwrap();
        assert_eq!(view["bands"].as_array().unwrap().len(), 1);
        assert_eq!(view["capture_count"], 50);
        assert!(view["points"].as_array().unwrap().len() <= 1024);
        assert_eq!(view["timeline"][0]["summary"], "10–20 / 50");
        let cancelled = AtomicBool::new(true);
        let completed = AtomicUsize::new(0);
        assert!(session.build(&request, &cancelled, &completed).is_err());
        assert_eq!(completed.load(Ordering::Relaxed), 0);
        request.capture_count = 257;
        assert!(session.preview(&request, 1).is_err());
        request.capture_count = 1;
        request.recipe = Default::default();
        let capture = session
            .build(&request, &AtomicBool::new(false), &completed)
            .unwrap()
            .remove(0);
        assert_eq!(capture.key(), &session.profile.key);
        let Measurement::Rxmer(frame) = &capture.measurement else {
            panic!()
        };
        assert_eq!(frame.source.file_kind, "merlin_synthetic_rxmer");
        let restored =
            Capture::decode(capture.name, capture.original.as_ref().clone(), true).unwrap();
        assert_eq!(restored.hash, capture.hash);
    }
    #[test]
    fn designed_flat_mer_drives_residual_power_and_real_fec() {
        use docsis_ofdm_fec::simulation::{
            channel::{run_channel, ChannelJobLimits},
            test_points::{NoopObserver, TestPointProbe},
        };
        let project: crate::workbench::project_file::Project = serde_json::from_slice(
            include_bytes!("../../../../../examples/workbench/multi-rxmer-2/workbench.merlin.json"),
        )
        .unwrap();
        let (mut inputs, editors) = project.prepare().unwrap();
        let profile = Arc::new(inputs.profile.as_ref().unwrap().clone());
        let session = Session::begin(&editors.measurements.draft_rx, profile.clone()).unwrap();
        let request = Request {
            source: "flat".into(),
            capture_count: 1,
            baseline_db: 48.0,
            recipe: Default::default(),
        };
        let captures = session
            .build(&request, &AtomicBool::new(false), &AtomicUsize::new(0))
            .unwrap();
        let plan = Model::playback(
            &captures,
            &Options::default(),
            &project.transmission,
            Some(&profile),
        )
        .unwrap();
        let present = plan.at(0).power.into_iter().flatten().collect::<Vec<_>>();
        assert!(!present.is_empty());
        for power in present {
            assert!((power - 10f64.powf(-4.8)).abs() < 1e-16);
        }
        let model = Model {
            accepted_rx: captures,
            options: Options::default(),
            ..Default::default()
        };
        inputs.playback = model.published(Some(&profile)).unwrap().playback;
        let settings = crate::simulation::closed_loop::prepare(
            &project.transmission,
            &inputs,
            &crate::simulation::closed_loop::Options {
                duration_ms: 1,
                ..Default::default()
            },
        )
        .unwrap();
        let result = run_channel(
            &settings,
            &ChannelJobLimits {
                max_runtime_ms: 60_000,
                max_retained_words: 1,
            },
            &AtomicBool::new(false),
            &mut TestPointProbe::disabled(),
            &mut NoopObserver,
            |_| {},
        )
        .unwrap();
        assert!(
            result.summary.comparison.bits_tested > 0,
            "{:?}",
            result.error
        );
        assert_eq!(result.summary.comparison.bit_errors, 0);
    }
    #[test]
    fn preview_zoom_uses_original_offsets_shared_axes_and_bounded_plc_window() {
        let (session, _) = fixture();
        let request = Request {
            source: "flat".into(),
            capture_count: 1,
            baseline_db: 48.0,
            recipe: Default::default(),
        };
        let full = session.preview(&request, 1).unwrap();
        let start = full["plc_view"]["start"].as_u64().unwrap() as usize;
        let count = full["plc_view"]["count"].as_u64().unwrap() as usize;
        let zoom = session.preview_window(&request, 1, start, count).unwrap();
        assert_eq!(zoom["start"], start);
        assert_eq!(zoom["count"], count);
        assert!(count < session.profile.key.grid.sample_count);
        assert!(start + count <= session.profile.key.grid.sample_count);
        let width = |view: &Value| {
            view["bands"][0]["last"].as_f64().unwrap() - view["bands"][0]["first"].as_f64().unwrap()
        };
        assert!(width(&zoom) > width(&full) * 10.0);
        assert_eq!(zoom["points"], zoom["comparison"]);
        assert!(zoom["points"].as_array().unwrap().len() <= 1024);
        assert_eq!(
            zoom["x_ticks"][0]["label"],
            format!(
                "{:.3}",
                f64::from(session.profile.key.grid.frequency(start).unwrap()) / 1e6
            )
        );
        assert!(session.preview_window(&request, 1, usize::MAX, 2).is_err());
        assert_eq!(
            session.preview_window(&request, 1, 0, 0).unwrap()["count"],
            full["count"]
        );
        assert!(full["plc_summary"]
            .as_str()
            .unwrap()
            .contains("16 carriers"));
    }
}
