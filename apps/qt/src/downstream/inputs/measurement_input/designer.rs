/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Synthetic RxMER envelopes and immutable designer sessions; input files stay intact.
mod retention;
use super::document::{Capture, Measurement, MAX_INPUT_BYTES, MAX_TOTAL_BYTES};
use crate::downstream::help::AdmissionError;
use crate::downstream::{
    common::constants::{
        DEFAULT_DESIGN_BASELINE_DB, DESIGNER_AXIS_PADDING_FRACTION, DESIGNER_EMPTY_AXIS_EXTENT_DB,
        DESIGNER_METRIC_TICK_INTERVALS, DESIGNER_MIN_AXIS_PADDING_DB,
    },
    workbench::session::profile_source,
};
use crate::shell::{Retained, RetainedResources, RetentionBudget};
use merlin::downstream::{
    inputs::{RxMerDesign, RxMerDesignPosition, RxmerRecipe},
    CaptureTime, MeasurementViewport, MerDb, NamedCapture, ProfileCapture, RxMerFrame, ScenarioKey,
    SourceProfileFrame as CapturedProfile,
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
    design: RxMerDesign,
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
    pub catalog: ProfileCapture,
    origins: Vec<Origin>,
    pub request: Request,
}

/// Measures dynamic children of an existing Designer worker request.
///
/// # Arguments
/// * `request` - Actual parsed request with its current source/edit capacities.
///
/// # Returns
/// Child storage only; the containing Session or worker owner counts Request's
/// fixed layout. No scientific operation, clone or application admission occurs.
///
/// # Errors
/// Checked payload overflow from the existing owner-local request traversal.
pub(crate) fn request_children(request: &Request) -> Result<RetainedResources, String> {
    retention::request_children(request)
}

/// Measures one immutable Designer record/design already retained by a capture.
///
/// # Arguments
/// * `origin` - Actual private owner, including independent original/recipe children.
///
/// # Returns
/// Complete fixed owner plus checked payload capacities and container/root counts.
///
/// # Errors
/// Public metadata or checked resource overflow; no source is materialized or mutated.
pub(crate) fn origin_resources(origin: &Origin) -> Result<crate::shell::RetainedResources, String> {
    retention::origin_resources(origin)
}
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn name(record: &Record) -> String {
    let prefix = if matches!(record.baseline, Baseline::Flat { .. }) {
        "Synthetic"
    } else {
        "Designed"
    };

    // Preserve the existing three-digit minimum-width generated capture label.
    format!("{prefix} RxMER {:03}", record.capture_index)
}

fn capture(record: Record, design: RxMerDesign) -> Result<Capture, AdmissionError> {
    let bytes = design.original_bytes().to_vec();
    if bytes.len() > MAX_INPUT_BYTES {
        return Err("Designed capture exceeds 16 MiB.".into());
    }
    let observation = design.materialize();
    let frame = observation.frame().clone();
    let decoded_hash = hash(&serde_json::to_vec(&frame).map_err(|cause| cause.to_string())?);
    Ok(Capture {
        name: name(&record),
        hash: hash(&bytes),
        original: Arc::new(bytes),
        format: "Designed RxMER",
        decoded_hash,
        measurement: Measurement::Rxmer(Arc::new(frame)),
        designer: Some(Arc::new(Origin { record, design })),
    })
}

fn admit(record: &Record) -> Result<RxMerDesign, AdmissionError> {
    let bytes = serde_json::to_vec(record).map_err(|cause| cause.to_string())?;
    let input = NamedCapture::new(name(record), bytes).map_err(AdmissionError::classified)?;
    RxMerDesign::from_bytes(&input).map_err(AdmissionError::classified)
}

fn materialize(record: Record) -> Result<Capture, AdmissionError> {
    let design = admit(&record)?;
    capture(record, design)
}
/// Detects and validates a native designer envelope before the ordinary PNM decoder.
/// # Arguments
/// * `bytes` - Already bounded input JSON; original embedded captures remain unchanged.
/// # Returns
/// None for ordinary files, or a reconstructed designed capture with baseline/recipe.
/// # Errors
/// Rejects unsupported versions, nesting, invalid provenance and unbounded recipes.
pub(crate) fn decode(bytes: &[u8]) -> Result<Option<Capture>, String> {
    let Ok(fields) =
        serde_json::from_slice::<std::collections::BTreeMap<String, serde::de::IgnoredAny>>(bytes)
    else {
        return Ok(None);
    };
    if !fields.contains_key("merlin_rxmer_designer") {
        return Ok(None);
    }
    let record: Record = serde_json::from_slice(bytes).map_err(|cause| cause.to_string())?;
    let input = NamedCapture::new(name(&record), bytes.to_vec())
        .map_err(|cause| cause.message().to_owned())?;
    let design = RxMerDesign::from_bytes(&input).map_err(|cause| cause.message().to_owned())?;
    capture(record, design)
        .map(Some)
        .map_err(|cause| cause.message)
}

impl Session {
    /// Snapshots original baselines and restores a common recipe without compounding.
    /// # Arguments
    /// * `captures` - Current parent draft, including optional designed copies.
    /// * `profile`, `catalog` - Selected source DTO and its exact original, for public admission and PLC.
    /// # Returns
    /// Immutable session and bounded controls; differing recipes reset to original data.
    /// # Errors
    /// Rejects non-RxMER inputs or profile/grid mismatches.
    pub(crate) fn begin(
        captures: &[Capture],
        profile: Arc<CapturedProfile>,
        catalog: ProfileCapture,
    ) -> Result<Self, String> {
        profile_source(Some(profile.as_ref()), Some(&catalog))?;
        let mut origins = Vec::new();
        for (index, capture) in captures.iter().enumerate() {
            let origin = if let Some(origin) = &capture.designer {
                (**origin).clone()
            } else {
                let input = NamedCapture::new(&capture.name, capture.original.as_ref().clone())
                    .map_err(|cause| cause.message().to_owned())?;
                let design = RxMerDesign::imported_capture(
                    &catalog,
                    profile.profile_id,
                    &input,
                    RxmerRecipe::default(),
                    RxMerDesignPosition::new(index + 1, captures.len())
                        .map_err(|cause| cause.message().to_owned())?,
                )
                .map_err(|cause| cause.message().to_owned())?;
                let record = serde_json::from_slice(design.original_bytes())
                    .map_err(|cause| cause.to_string())?;
                Origin { record, design }
            };
            origin
                .design
                .check_capture(&catalog, profile.profile_id)
                .map_err(|cause| cause.message().to_owned())?;
            origins.push(origin);
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
                .unwrap_or(DEFAULT_DESIGN_BASELINE_DB),
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
            catalog,
            origins,
            request,
        })
    }

    /// Admits this actual immutable Designer session before storing or dispatching it.
    ///
    /// # Arguments
    /// * `budget` - Exact target counter also covering old, candidate and worker owners.
    ///
    /// # Returns
    /// Existing Retained ownership of all Session children and concrete storage layouts.
    /// Keep the returned guard with its data through queues, adoption and final disposal.
    ///
    /// # Errors
    /// Public metadata, checked layout arithmetic or any aggregate dimension rejection.
    pub(crate) fn admit(self, budget: &RetentionBudget) -> Result<Retained<Self>, String> {
        let resources = self.retained_resources()?;
        crate::downstream::inputs::retention::admit_model(self, resources, budget)
    }

    /// Measures this actual Designer session without materializing or exporting captures.
    ///
    /// # Returns
    /// Session fixed layout once, owned capacities and conservative complete shared
    /// profile/catalog/design payloads and roots. No source, recipe or cache is mutated.
    ///
    /// # Errors
    /// Existing public metadata or checked payload/population overflow.
    pub(crate) fn retained_resources(&self) -> Result<RetainedResources, String> {
        retention::session_resources(self)
    }

    /// Prepares and admits one actual immutable session copy for a worker or old owner.
    ///
    /// # Arguments
    /// * `budget` - Canonical target counter; the borrowed source stays independently charged.
    ///
    /// # Returns
    /// A separately charged copy, measured after its finite clone preparation. Clone
    /// capacities need not equal the source's capacities. Immutable shared payloads are
    /// conservatively covered by each owner; this does not bound temporary preparation peaks.
    ///
    /// # Errors
    /// Metadata, checked arithmetic or target rejection drops the candidate and leaves
    /// this source, its original bytes and all existing reservations unchanged.
    pub(crate) fn clone_admitted(
        &self,
        budget: &RetentionBudget,
    ) -> Result<Retained<Self>, String> {
        self.clone().admit(budget)
    }

    fn count(&self, request: &Request) -> Result<usize, AdmissionError> {
        if request.source == "imported" && request.recipe.random_variation.is_some() {
            return Err("Random variation requires a new flat sequence.".into());
        }
        let count = match request.source.as_str() {
            "imported" => self.origins.len(),
            "flat" => request.capture_count,
            _ => return Err("Choose imported or flat baseline.".into()),
        };
        let limits = RxMerDesign::limits();
        if count == 0
            || count > limits.maximum_captures
            || count
                .checked_mul(self.profile.key.grid.sample_count)
                .is_none_or(|values| values > limits.maximum_values)
        {
            return Err("Use 1–256 captures and at most 4,000,000 total values.".into());
        }
        let mut record = self.origin(request, 0)?.record;
        record.recipe = request.recipe.clone();
        record.capture_index = 1;
        record.capture_count = count;
        admit(&record)?;
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
        let design = RxMerDesign::flat_capture(
            &self.catalog,
            self.profile.profile_id,
            MerDb::new(request.baseline_db).map_err(|cause| cause.message().to_owned())?,
            CaptureTime::new(self.profile.capture_time)
                .map_err(|cause| cause.message().to_owned())?,
            RxmerRecipe::default(),
            RxMerDesignPosition::new(index + 1, request.capture_count)
                .map_err(|cause| cause.message().to_owned())?,
        )
        .map_err(|cause| cause.message().to_owned())?;
        let record =
            serde_json::from_slice(design.original_bytes()).map_err(|cause| cause.to_string())?;
        Ok(Origin { record, design })
    }
    /// Materializes only the requested export range on full original grids.
    /// # Arguments
    /// * `request` - Immutable source/recipe snapshot.
    /// * `first`, `last` - Inclusive one-based source positions, independent of zoom.
    /// * `cancel`, `completed` - Worker cancellation and actual capture progress.
    /// # Returns
    /// Owned complete RxMER frames without adopting a draft or running simulation.
    /// # Errors
    /// Rejects invalid ranges, recipes, work bounds or cancellation.
    pub(crate) fn export_frames(
        &self,
        request: &Request,
        first: usize,
        last: usize,
        cancel: &AtomicBool,
        completed: &AtomicUsize,
    ) -> Result<Vec<RxMerFrame>, String> {
        let count = self.count(request).map_err(|error| error.message)?;
        if first == 0 || first > last || last > count {
            return Err("Choose an export range within the capture collection.".into());
        }
        let mut frames = Vec::with_capacity(last - first + 1);
        for index in first..=last {
            if cancel.load(Ordering::Relaxed) {
                return Err("PNM export cancelled.".into());
            }
            let mut origin = self.origin(request, index - 1)?;
            origin.record.recipe = request.recipe.clone();
            origin.record.capture_index = index;
            origin.record.capture_count = count;
            let capture = materialize(origin.record).map_err(|error| error.message)?;
            let Measurement::Rxmer(frame) = capture.measurement else {
                unreachable!()
            };
            frames.push((*frame).clone());
            completed.store(index - first + 1, Ordering::Relaxed);
        }
        Ok(frames)
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
        let baseline = origin
            .design
            .with_recipe(RxmerRecipe::default())
            .map_err(AdmissionError::classified)?
            .materialize();
        let grid = &baseline.frame().key.grid;
        let mut record = origin.record;
        record.recipe = request.recipe.clone();
        record.capture_index = position;
        record.capture_count = count;
        let designed = admit(&record)?.materialize();
        let viewport = MeasurementViewport {
            start,
            count: samples,
        };
        let before = baseline
            .preview(viewport)
            .map_err(AdmissionError::classified)?
            .view;
        let after = designed
            .preview(viewport)
            .map_err(AdmissionError::classified)?
            .view;
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
                return (
                    -DESIGNER_EMPTY_AXIS_EXTENT_DB,
                    DESIGNER_EMPTY_AXIS_EXTENT_DB,
                );
            }
            let padding =
                ((high - low) * DESIGNER_AXIS_PADDING_FRACTION).max(DESIGNER_MIN_AXIS_PADDING_DB);
            (low - padding, high + padding)
        }
        let (before_bottom, before_top) = limits(&baseline.frame().mer_db[window.clone()]);
        let (after_bottom, after_top) = limits(&designed.frame().mer_db[window]);
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
        after["y_ticks"]=json!((0..=DESIGNER_METRIC_TICK_INTERVALS).map(|tick|json!({"x":f64::from(tick)/f64::from(DESIGNER_METRIC_TICK_INTERVALS),"label":format!("{:.1}",high-f64::from(tick)/f64::from(DESIGNER_METRIC_TICK_INTERVALS)*(high-low))})).collect::<Vec<_>>());
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
            let (first,last)=match scope {merlin::downstream::inputs::CaptureScope::All=>(1,count),merlin::downstream::inputs::CaptureScope::Range{first,last}=>(*first,*last)};
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
            let capture = materialize(origin.record)?;
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
mod admission_tests;

#[cfg(test)]
mod worker_input_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::downstream::inputs::measurement_input::model::{Model, Options};
    fn fixture() -> (Session, Model) {
        let project: crate::downstream::workbench::project_file::Project =
            serde_json::from_slice(include_bytes!(
                "../../../../../../examples/workbench/downstream/multi-rxmer-2/workbench.merlin.json"
            ))
            .unwrap();
        let (inputs, editors) = project.prepare().unwrap();
        let session = Session::begin(
            &editors.measurements.draft_rx,
            Arc::new(inputs.profile.unwrap()),
            inputs.profile_capture.unwrap(),
        )
        .unwrap();
        (session, editors.measurements)
    }
    #[test]
    fn random_variation_requires_flat_and_survives_envelope_roundtrip() {
        let (session, _) = fixture();
        let mut request = session.request.clone();
        request.recipe.random_variation = Some(merlin::downstream::inputs::RandomVariation {
            standard_deviation_db: 1.5,
            seed: 41,
        });
        request.source = "imported".into();
        assert!(session.count(&request).is_err());
        request.source = "flat".into();
        request.capture_count = 1;
        assert_eq!(session.count(&request).unwrap(), 1);
        let origin = session.origin(&request, 0).unwrap();
        let capture = materialize(origin.record).unwrap();
        let restored = decode(&capture.original).unwrap().unwrap();
        assert_eq!(capture.decoded_hash, restored.decoded_hash);
    }
    #[test]
    fn envelope_roundtrip_reopens_originals_without_compounding() {
        let (mut session, model) = fixture();
        // A small original collection exercises provenance without inflating test data.
        session.origins.truncate(2);
        session.hashes.truncate(2);
        let mut request = session.request.clone();
        request.source = "imported".into();
        request.recipe.tilt = Some(merlin::downstream::inputs::Tilt {
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
        let reopened =
            Session::begin(&result, session.profile.clone(), session.catalog.clone()).unwrap();
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
                mode: merlin::downstream::MeasurementMode::Hold,
                ..Default::default()
            },
            ..Default::default()
        };
        let restored = Model::from_saved(&accepted.export()).unwrap();
        assert_eq!(restored.accepted_rx[0].hash, accepted.accepted_rx[0].hash);
        assert_eq!(model.accepted_rx.len(), 132);
    }
    #[test]
    fn repeated_flat_edits_recover_the_immutable_baseline() {
        let (session, model) = fixture();
        let original_bytes = model.draft_rx[0].original.clone();
        let mut request = Request {
            source: "flat".into(),
            capture_count: 1,
            baseline_db: 48.0,
            recipe: Default::default(),
        };
        let mut current = session;

        // Independent edge anchors: a 48-dB flat baseline plus -6, -3 and zero tilt.
        for (tilt_db, expected_last_db) in [(-6.0, 42.0), (-3.0, 45.0), (0.0, 48.0)] {
            request.recipe.tilt = Some(merlin::downstream::inputs::Tilt {
                scope: Default::default(),
                tilt_db,
            });
            let result = current
                .build(&request, &AtomicBool::new(false), &AtomicUsize::new(0))
                .unwrap();
            let Measurement::Rxmer(frame) = &result[0].measurement else {
                panic!("Expected a public Designer observation");
            };
            assert_eq!(frame.mer_db.iter().flatten().next(), Some(&48.0));
            assert_eq!(
                frame.mer_db.iter().flatten().next_back(),
                Some(&expected_last_db)
            );
            current =
                Session::begin(&result, current.profile.clone(), current.catalog.clone()).unwrap();
        }
        assert_eq!(*model.draft_rx[0].original, *original_bytes);
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
        request.recipe.notch = Some(merlin::downstream::inputs::Notch {
            scope: merlin::downstream::inputs::CaptureScope::Range {
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
        let project: crate::downstream::workbench::project_file::Project =
            serde_json::from_slice(include_bytes!(
                "../../../../../../examples/workbench/downstream/multi-rxmer-2/workbench.merlin.json"
            ))
            .unwrap();
        let (mut inputs, editors) = project.prepare().unwrap();
        let profile = Arc::new(inputs.profile.as_ref().unwrap().clone());
        let session = Session::begin(
            &editors.measurements.draft_rx,
            profile.clone(),
            inputs.profile_capture.clone().unwrap(),
        )
        .unwrap();
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
            inputs.profile_capture.as_ref(),
        )
        .unwrap();
        let present = plan
            .at(0)
            .normalized_power
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
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
        let mut sections = project.sections.clone();
        sections.insert("measurements".into(), model.export());
        let settings = crate::downstream::simulation::closed_loop::prepare(
            &project.transmission,
            &inputs,
            &crate::downstream::simulation::closed_loop::Options {
                duration_ms: 1,
                max_runtime_ms: 60_000,
                ..Default::default()
            },
            &sections,
        )
        .unwrap();
        let actual = settings
            .run(&merlin::downstream::FullLoopControl::new())
            .unwrap();
        let result = actual.report();
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
    #[test]
    fn pnm_export_range_uses_full_grid_and_guards_recipe_results() {
        let (session, model) = fixture();
        let mut request = session.request.clone();
        request.source = "flat".into();
        request.capture_count = 3;
        request.baseline_db = 48.13;
        let cancel = AtomicBool::new(false);
        let completed = AtomicUsize::new(0);
        let frames = session
            .export_frames(&request, 2, 3, &cancel, &completed)
            .unwrap();
        assert_eq!(frames.len(), 2);
        assert_eq!(
            frames[0].key.grid.sample_count,
            session.profile.key.grid.sample_count
        );
        assert_eq!(frames[0].capture_time, session.profile.capture_time);
        let summary = merlin::downstream::inputs::rxmer_export::preflight(
            &frames,
            merlin::downstream::inputs::rxmer_export::RangePolicy::Reject,
        )
        .unwrap();
        assert_eq!(summary.captures, 2);
        assert!(summary.rounded > 0);
        request.baseline_db = 70.0;
        let frames = session
            .export_frames(&request, 1, 1, &cancel, &completed)
            .unwrap();
        assert!(merlin::downstream::inputs::rxmer_export::preflight(
            &frames,
            merlin::downstream::inputs::rxmer_export::RangePolicy::Reject
        )
        .is_err());
        assert!(
            merlin::downstream::inputs::rxmer_export::preflight(
                &frames,
                merlin::downstream::inputs::rxmer_export::RangePolicy::Clamp
            )
            .unwrap()
            .clipped
                > 0
        );
        assert!(session
            .export_frames(&request, 0, 1, &cancel, &completed)
            .is_err());
        assert!(session
            .export_frames(&request, 1, 4, &cancel, &completed)
            .is_err());
        assert!(session
            .export_frames(&request, 1, 1, &AtomicBool::new(true), &completed)
            .is_err());
        assert!(!model.draft_rx.is_empty());
    }
}
