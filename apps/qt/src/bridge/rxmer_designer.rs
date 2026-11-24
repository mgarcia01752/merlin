/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Designer batch lifecycle; the GUI adopts only a complete, current draft.
use super::measurement_input::bridge;
use crate::downstream::common::constants::{
    DESIGNER_CAPTURE_QUEUE_CAPACITY, MAX_DESIGNER_PLAYBACK_DOCUMENT_BYTES,
    MAX_DESIGNER_RECIPE_DOCUMENT_BYTES,
};
use crate::downstream::help::AdmissionError;
use crate::downstream::inputs::measurement_input::document::Capture;
use crate::downstream::inputs::measurement_input::{
    designer::{Request, Session},
    model::{Model, Options},
};
use crate::shell::{Retained, RetentionReservation};
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use serde_json::{json, Value};
use std::{
    pin::Pin,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc, Arc,
    },
};
pub(super) struct Work {
    receiver: mpsc::Receiver<Result<Retained<Vec<Capture>>, AdmissionError>>,
    cancel: Arc<AtomicBool>,
    completed: Arc<AtomicUsize>,
    hashes: Vec<String>,
    profile_hash: String,
    options: Options,
    imported: bool,
}

// Preserve the source charge until the independently admitted editor owns the data.
struct StagedBatch {
    metadata: Value,
    _source: Option<RetentionReservation>,
}
fn request(document: &QString) -> Result<Request, String> {
    let text = document.to_string();
    if text.len() > MAX_DESIGNER_RECIPE_DOCUMENT_BYTES {
        return Err("Designer recipe exceeds 24 KiB.".into());
    }
    serde_json::from_str(&text).map_err(|error| error.to_string())
}
fn response(result: Result<Value, AdmissionError>) -> QString {
    QString::from(
        result
            .unwrap_or_else(|error| json!({"error":error.message,"diagnostic":error.diagnostic}))
            .to_string(),
    )
}
impl bridge::MeasurementInputs {
    /// Snapshots draft baselines and accepted profile without publishing edits.
    /// # Returns
    /// Restored recipe/source controls or a diagnostic; large arrays remain in Rust.
    pub fn designer_begin(mut self: Pin<&mut Self>) -> QString {
        let result = (|| {
            if self.rust().designer_work.is_some() {
                return Err("Wait for the previous Designer operation to stop.".into());
            }
            let inputs = crate::downstream::workbench::session::snapshot(&self.rust().context);
            let profile = inputs
                .profile
                .as_ref()
                .ok_or("Apply a modulation profile before opening RxMER Designer.")?;
            let session = Session::begin(
                &self.rust().model.draft_rx,
                Arc::new(profile.clone()),
                inputs
                    .profile_capture
                    .as_ref()
                    .ok_or("Apply the original modulation-profile capture.")?
                    .clone(),
            )?;
            let grid = &session.profile.key.grid;
            let value = json!({"request":session.request,"imported_count":session.hashes.len(),"first_hz":grid.frequency(0).map_err(|error|error.to_string())?,"last_hz":grid.frequency(grid.sample_count-1).map_err(|error|error.to_string())?,"spacing_hz":grid.spacing_hz,"limits":merlin::downstream::inputs::RxMerDesign::limits()});
            self.as_mut().rust_mut().replace_designer(session)?;
            Ok(value)
        })();
        response(result)
    }
    /// Queries a single before/after preview with bounded curves and PLC shading.
    /// # Arguments
    /// * `document` - Source controls and scientific recipe, at most24KiB.
    /// * `position` - One-based capture number.
    /// * `start`, `count` - Nonnegative source viewport; zero count restores full span.
    /// # Returns
    /// Shared-axis frequency view, never a serialized capture collection.
    pub fn designer_preview(
        &self,
        document: &QString,
        position: i32,
        start: i32,
        count: i32,
    ) -> QString {
        response((|| {
            self.rust()
                .designer
                .as_ref()
                .ok_or("Open Designer first.")?
                .value()
                .preview_window(
                    &request(document)?,
                    usize::try_from(position).map_err(|_| "Invalid capture position.")?,
                    usize::try_from(start).map_err(|_| "Invalid viewport start.")?,
                    usize::try_from(count).map_err(|_| "Invalid viewport count.")?,
                )
        })())
    }
    /// Starts cancellable batch generation on a Rust worker without changing drafts.
    /// # Arguments
    /// * `document` - Bounded recipe and baseline controls.
    /// * `options` - Parent playback options; checked identities are remapped on success.
    /// # Returns
    /// Running state or diagnostic. Completion must be polled before parent Apply.
    pub fn designer_stage(
        mut self: Pin<&mut Self>,
        document: &QString,
        options: &QString,
    ) -> QString {
        response((|| {
            if self.rust().designer_work.is_some() {
                return Err("Designer is already running.".into());
            }
            let request = request(document)?;
            if self.rust().designer.is_none() {
                return Err("Open Designer first.".into());
            }
            let session = self.rust().designer_copy()?;
            let total = session.value().preview(&request, 1)?["capture_count"]
                .as_u64()
                .unwrap_or(0);
            let text = options.to_string();
            if text.len() > MAX_DESIGNER_PLAYBACK_DOCUMENT_BYTES {
                return Err("Selection exceeds 24 KiB.".into());
            }
            let options: Options =
                serde_json::from_str(&text).map_err(|error| error.to_string())?;
            let cancel = Arc::new(AtomicBool::new(false));
            let completed = Arc::new(AtomicUsize::new(0));
            let budget = self.rust().context.application.budget().clone();
            let (sender, receiver) = mpsc::sync_channel(DESIGNER_CAPTURE_QUEUE_CAPACITY);
            let work = Work {
                receiver,
                cancel: cancel.clone(),
                completed: completed.clone(),
                hashes: session.value().hashes.clone(),
                profile_hash: session.value().profile.source.sha256.clone(),
                options,
                imported: request.source == "imported",
            };
            log::debug!(
                "rxmer-designer stage_started captures={total} imported={}",
                work.imported
            );
            std::thread::Builder::new()
                .name("rxmer-designer".into())
                .spawn(move || {
                    let result = session
                        .value()
                        .build(&request, &cancel, &completed)
                        .and_then(|captures| {
                            Model::admit_capture_batch(captures, &budget)
                                .map_err(AdmissionError::from)
                        });
                    let _ = sender.send(result);
                })
                .map_err(|error| error.to_string())?;
            self.as_mut().rust_mut().designer_work = Some(work);
            Ok(json!({"running":true}))
        })())
    }
    /// Polls progress and atomically stages a complete batch if its baseline is current.
    /// # Returns
    /// Running progress, cancelled/error, or new metadata/options for the parent draft.
    pub fn designer_poll(mut self: Pin<&mut Self>) -> QString {
        let Some(work) = self.rust().designer_work.as_ref() else {
            return response(Ok(json!({"running":false})));
        };
        let result = match work.receiver.try_recv() {
            Ok(result) => result,
            Err(mpsc::TryRecvError::Empty) => {
                return response(Ok(
                    json!({"running":true,"completed":work.completed.load(Ordering::Relaxed)}),
                ))
            }
            Err(_) => Err("Designer worker ended unexpectedly.".into()),
        };
        let work = self.as_mut().rust_mut().designer_work.take().unwrap();
        let inputs = crate::downstream::workbench::session::snapshot(&self.rust().context);
        let profile = inputs.profile.as_ref();
        let result = adopt_to_owner(self.as_mut().rust_mut().get_mut(), work, result, profile);
        response(result)
    }
    /// Cancels pending generation; already accepted captures are never modified.
    /// # Returns
    /// Unit; polling reaps the worker and discards cancelled completion.
    pub fn designer_cancel(self: Pin<&mut Self>) {
        if let Some(work) = &self.rust().designer_work {
            work.cancel.store(true, Ordering::Relaxed)
        }
    }
}

#[cfg(test)]
#[path = "rxmer_designer/retention_tests.rs"]
mod retention_tests;

#[cfg(test)]
#[path = "rxmer_designer/capture_batch_tests.rs"]
mod capture_batch_tests;

fn adopt_to_owner(
    owner: &mut super::measurement_input::MeasurementInputsRust,
    work: Work,
    result: Result<Retained<Vec<Capture>>, AdmissionError>,
    profile: Option<&merlin::downstream::SourceProfileFrame>,
) -> Result<Value, AdmissionError> {
    let mut candidate = owner.model.clone();
    let staged = adopt(work, result, &mut candidate, profile)?;
    if staged.metadata["staged"] == true {
        owner.replace_model(candidate)?;
    }
    Ok(staged.metadata)
}

fn adopt(
    work: Work,
    result: Result<Retained<Vec<Capture>>, AdmissionError>,
    model: &mut Model,
    profile: Option<&merlin::downstream::SourceProfileFrame>,
) -> Result<StagedBatch, AdmissionError> {
    if work.cancel.load(Ordering::Relaxed) {
        log::debug!(
            "rxmer-designer stage_cancelled completed={}",
            work.completed.load(Ordering::Relaxed)
        );
        return Ok(StagedBatch {
            metadata: json!({"cancelled":true,"running":false}),
            _source: None,
        });
    }
    let captures = result?;
    let current = model
        .draft_rx
        .iter()
        .map(|capture| capture.hash.clone())
        .collect::<Vec<_>>();
    if current != work.hashes
        || profile
            .as_ref()
            .is_none_or(|profile| profile.source.sha256 != work.profile_hash)
    {
        return Err("Inputs changed during design; reopen Designer and retry.".into());
    }
    let mut options = work.options;
    if work.imported {
        options.selected_captures = options.selected_captures.map(|selected| {
            let selected = selected
                .into_iter()
                .collect::<std::collections::HashSet<_>>();
            work.hashes
                .iter()
                .zip(captures.value())
                .filter(|(hash, _)| selected.contains(*hash))
                .map(|(_, capture)| capture.hash.clone())
                .collect()
        });
    } else {
        options.selected_captures = None;
        if captures.value().len() > 1 {
            options.mode = merlin::downstream::MeasurementMode::Hold;
        }
    }
    let (captures, source) = captures.into_parts();
    let mut checked = Model::default();
    checked.stage(captures, true)?;
    model.draft_rx = checked.draft_rx;
    log::debug!(
        "rxmer-designer stage_complete captures={}",
        model.draft_rx.len()
    );
    let mut metadata = json!({"files":model.files(true,profile),"flat":false});
    metadata["options"] = json!(options);
    Ok(StagedBatch {
        metadata: json!({"running":false,"staged":true,"metadata":metadata}),
        _source: Some(source),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reuses the original bounded two-capture Designer/editor fixture.
    /// # Returns
    /// A validated Session and the existing model with untouched accepted captures.
    /// # Panics
    /// Panics if the checked sample workbench or original association no longer loads.
    pub(super) fn fixture() -> (Session, Model) {
        let project: crate::downstream::workbench::project_file::Project = serde_json::from_slice(
            include_bytes!("../../../../examples/workbench/multi-rxmer-2/workbench.merlin.json"),
        )
        .unwrap();
        let (inputs, editors) = project.prepare().unwrap();
        let mut model = editors.measurements;
        model.draft_rx.truncate(2);
        let session = Session::begin(
            &model.draft_rx,
            Arc::new(inputs.profile.unwrap()),
            inputs.profile_capture.unwrap(),
        )
        .unwrap();
        (session, model)
    }
    /// Creates the original completed-worker fixture without launching a thread.
    /// # Arguments
    /// * `session` - Original baseline identities used by stale-completion checks.
    /// * `options` - Existing playback and selection settings.
    /// # Returns
    /// A disconnected worker fixture with unchanged progress and baseline fields.
    pub(super) fn work(session: &Session, options: Options) -> Work {
        let (_, receiver) = mpsc::channel();
        Work {
            receiver,
            cancel: Arc::new(AtomicBool::new(false)),
            completed: Arc::new(AtomicUsize::new(2)),
            hashes: session.hashes.clone(),
            profile_hash: session.profile.source.sha256.clone(),
            options,
            imported: true,
        }
    }
    fn batch(captures: Vec<Capture>) -> Retained<Vec<Capture>> {
        let budget = crate::shell::RetentionBudget::with_limits(crate::shell::RetainedResources {
            payload_bytes: usize::MAX,
            ordered_entries: usize::MAX,
            shared_roots: usize::MAX,
        });
        Model::admit_capture_batch(captures, &budget).unwrap()
    }
    #[test]
    fn adoption_is_atomic_remaps_selection_and_rejects_stale_completion() {
        let (session, mut model) = fixture();
        let options = Options {
            selected_captures: Some(vec![model.draft_rx[1].hash.clone()]),
            ..Default::default()
        };
        let captures = session
            .build(
                &session.request,
                &AtomicBool::new(false),
                &AtomicUsize::new(0),
            )
            .unwrap();
        let selected = captures[1].hash.clone();
        let result = adopt(
            work(&session, options),
            Ok(batch(captures)),
            &mut model,
            Some(&session.profile),
        )
        .unwrap();
        assert_eq!(
            result.metadata["metadata"]["options"]["selected_captures"],
            json!([selected])
        );
        assert_eq!(model.accepted_rx.len(), 132);
        let previous = model.draft_rx[0].hash.clone();
        let captures = session
            .build(
                &session.request,
                &AtomicBool::new(false),
                &AtomicUsize::new(0),
            )
            .unwrap();
        assert!(adopt(
            work(&session, Options::default()),
            Ok(batch(captures)),
            &mut model,
            Some(&session.profile)
        )
        .is_err());
        assert_eq!(model.draft_rx[0].hash, previous);
    }
    #[test]
    fn cancellation_discards_even_a_completed_success_and_profile_changes_reject() {
        let (session, mut model) = fixture();
        let captures = session
            .build(
                &session.request,
                &AtomicBool::new(false),
                &AtomicUsize::new(0),
            )
            .unwrap();
        let work = work(&session, Options::default());
        work.cancel.store(true, Ordering::Relaxed);
        let result = adopt(
            work,
            Ok(batch(captures.clone())),
            &mut model,
            Some(&session.profile),
        )
        .unwrap();
        assert_eq!(result.metadata["cancelled"], true);
        assert_eq!(model.draft_rx[0].hash, session.hashes[0]);
        let mut profile = (*session.profile).clone();
        profile.source.sha256 = "f".repeat(64);
        assert!(adopt(
            super::tests::work(&session, Options::default()),
            Ok(batch(captures)),
            &mut model,
            Some(&profile)
        )
        .is_err());
        assert_eq!(model.draft_rx[0].hash, session.hashes[0]);
    }
}
