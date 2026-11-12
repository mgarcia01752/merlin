/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Single-flight background Open; only completed owned state crosses threads.
use super::{
    project_file::{Editors, Project, MAX_PROJECT_BYTES},
    session::Inputs,
};
use crate::{instruments::transmitter::preview::Analyzer, simulation::closed_loop::Model};
use serde_json::{json, Value};
use std::{
    cell::RefCell,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        mpsc::{self, Receiver, TryRecvError},
        Arc,
    },
    time::Instant,
};

/// Owned editor/request transfer consumed synchronously during GUI adoption.
#[derive(Default)]
pub(crate) struct PreparedModels {
    pub source: Option<crate::inputs::source_input::model::SourceModel>,
    pub profile: Option<crate::inputs::profile_input::ProfileModel>,
    pub measurements: Option<crate::inputs::measurement_input::model::Model>,
    pub receiver: Option<Model>,
    pub transmitter: Option<Analyzer>,
}
thread_local! {
    static PREPARED: RefCell<PreparedModels> = RefCell::default();
}

/// Moves one prepared model into its GUI adapter without decoding captures again.
/// # Arguments
/// * `take` - Takes the owning adapter's field from the completed transfer.
/// # Returns
/// That field, or None for normal edits and blank workbenches.
pub(crate) fn take<T>(take: impl FnOnce(&mut PreparedModels) -> Option<T>) -> Option<T> {
    PREPARED.with(|prepared| take(&mut prepared.borrow_mut()))
}

pub(crate) struct PreparedProject {
    pub path: Option<PathBuf>,
    project: Project,
    inputs: Inputs,
    models: PreparedModels,
    metadata: Value,
}
impl PreparedProject {
    /// Validates and prepares a project independently of the GUI session.
    /// # Arguments
    /// * `project` - Bounded, decoded file document.
    /// * `path` - Explicit Save destination, None for update recovery.
    /// # Returns
    /// Validated models and bounded readiness metadata; never starts a run.
    /// # Errors
    /// Preserves project validation errors without publishing partial state.
    pub(crate) fn prepare(project: Project, path: Option<PathBuf>) -> Result<Self, String> {
        let (
            inputs,
            Editors {
                source,
                profile,
                measurements,
            },
        ) = project.prepare()?;
        let settings =
            serde_json::to_string(&project.transmission).map_err(|error| error.to_string())?;
        let mut receiver = Model::default();
        receiver.options = project.receiver.clone();
        receiver.verify = project.comparison_enabled;
        receiver.prepare_inputs(&settings, &inputs);
        let mut transmitter = Analyzer::default();
        transmitter.configure(&settings, inputs.clone());
        let metadata = json!({"opened":true,"prepared":true,"path":path.as_ref().map(|p|p.to_string_lossy()),
            "transmission":project.transmission,"receiver":project.receiver,
            "verified":project.comparison_enabled,"dark":project.dark,
            "source_state":source.state(),"profile_state":profile.state(&settings),
            "measurement_state":measurements.state(Ok(project.transmission.clone()), &inputs)});
        Ok(Self {
            path,
            project,
            inputs,
            models: PreparedModels {
                source: Some(source),
                profile: Some(profile),
                measurements: Some(measurements),
                receiver: Some(receiver),
                transmitter: Some(transmitter),
            },
            metadata,
        })
    }

    /// Publishes a completed project on the calling GUI thread only.
    /// # Returns
    /// Small adoption metadata; original bytes and requests remain in Rust.
    pub(crate) fn adopt(self) -> Value {
        super::session::restore(self.inputs, self.project.sections);
        crate::platform::input_file::clear_original_paths();
        PREPARED.with(|prepared| *prepared.borrow_mut() = self.models);
        self.metadata
    }
}

struct Job {
    receiver: Receiver<Result<Option<PreparedProject>, String>>,
    cancelled: Arc<AtomicBool>,
    stage: Arc<AtomicU8>,
    started: Instant,
}

/// One tracked worker, with nonblocking progress/completion and cancellation.
#[derive(Default)]
pub(crate) struct Loader {
    job: Option<Job>,
}
impl Loader {
    /// Reports whether a load is still tracked, including cancellation.
    /// # Returns
    /// True until completion is drained; prevents conflicting file operations.
    pub(crate) fn is_loading(&self) -> bool {
        self.job.is_some()
    }
    /// Starts the native picker or recovery read on a background thread.
    /// # Arguments
    /// * `recovery` - Private recovery path; None selects an explicit project.
    /// # Returns
    /// Loading metadata, or a busy/spawn diagnostic; repeated Open is rejected.
    pub(crate) fn start(&mut self, recovery: Option<PathBuf>) -> Value {
        self.start_source(
            recovery.map(crate::platform::startup::WorkbenchSource::File),
            false,
        )
    }

    /// Starts a command-line file or redirected standard-input workbench.
    /// # Arguments
    /// * `source` - Owned input request; stdin never becomes a Save destination.
    /// # Returns
    /// Immediate loading status; validation and reads run on the single worker.
    pub(crate) fn start_requested(
        &mut self,
        source: crate::platform::startup::WorkbenchSource,
    ) -> Value {
        self.start_source(Some(source), true)
    }

    fn start_source(
        &mut self,
        source: Option<crate::platform::startup::WorkbenchSource>,
        remember_path: bool,
    ) -> Value {
        self.start_with(move |stage, cancelled| {
            use crate::platform::startup::WorkbenchSource;
            use std::io::IsTerminal;
            let (mut reader, destination): (Box<dyn std::io::Read>, Option<PathBuf>) = match source
            {
                Some(WorkbenchSource::Stdin) => {
                    if std::io::stdin().is_terminal() {
                        return Err("Redirect workbench JSON with --workbench - < FILE.".into());
                    }
                    stage.store(1, Ordering::Relaxed);
                    (Box::new(std::io::stdin()), None)
                }
                other => {
                    let (path, remember) = if let Some(WorkbenchSource::File(path)) = other {
                        (path, remember_path)
                    } else {
                        let Some(uri) = crate::platform::native_file_picker::choose_workbench(
                            "Open MERLIN Workbench",
                        )?
                        else {
                            return Ok(None);
                        };
                        let path = cxx_qt_lib::QUrl::from(&cxx_qt_lib::QString::from(uri))
                            .to_local_file()
                            .map(|path| PathBuf::from(path.to_string()))
                            .ok_or("Choose a local project path.")?;
                        (path, true)
                    };
                    if cancelled.load(Ordering::Relaxed) {
                        return Ok(None);
                    }
                    stage.store(1, Ordering::Relaxed);
                    if !std::fs::metadata(&path)
                        .map_err(|error| error.to_string())?
                        .is_file()
                    {
                        return Err("Choose a regular workbench file.".into());
                    }
                    let file = std::fs::File::open(&path).map_err(|error| error.to_string())?;
                    let metadata = file.metadata().map_err(|error| error.to_string())?;
                    if !metadata.is_file() || metadata.len() > MAX_PROJECT_BYTES as u64 {
                        return Err(
                            "Choose a regular workbench file within the input limit.".into()
                        );
                    }
                    (Box::new(file), remember.then_some(path))
                }
            };
            let Some(bytes) = read_bounded(&mut reader, cancelled, MAX_PROJECT_BYTES)? else {
                return Ok(None);
            };
            stage.store(2, Ordering::Relaxed);
            let project = serde_json::from_slice(&bytes)
                .map_err(|error| format!("Invalid workbench JSON: {error}"))?;
            drop(bytes);
            if cancelled.load(Ordering::Relaxed) {
                return Ok(None);
            }
            stage.store(3, Ordering::Relaxed);
            PreparedProject::prepare(project, destination).map(Some)
        })
    }

    fn start_with(
        &mut self,
        work: impl FnOnce(&AtomicU8, &AtomicBool) -> Result<Option<PreparedProject>, String>
            + Send
            + 'static,
    ) -> Value {
        if self.job.is_some() {
            return json!({"error":"A workbench is already loading."});
        }
        let (sender, receiver) = mpsc::sync_channel(1);
        let cancelled = Arc::new(AtomicBool::new(false));
        let stage = Arc::new(AtomicU8::new(0));
        let stop = cancelled.clone();
        let progress = stage.clone();
        match std::thread::Builder::new()
            .name("merlin-workbench-open".into())
            .spawn(move || {
                let _ = sender.send(work(&progress, &stop));
            }) {
            Ok(_) => {
                log::debug!("workbench_open_started");
                self.job = Some(Job {
                    receiver,
                    cancelled,
                    stage,
                    started: Instant::now(),
                });
                self.status()
            }
            Err(error) => json!({"error":format!("Cannot start workbench loader: {error}")}),
        }
    }

    /// Requests cancellation; an in-progress validation stage finishes off-thread.
    /// # Returns
    /// No result; one-worker guard remains until completion is drained.
    pub(crate) fn cancel(&self) {
        if let Some(job) = &self.job {
            job.cancelled.store(true, Ordering::Relaxed);
        }
    }

    /// Polls once without waiting for the picker, parsing or validation.
    /// # Returns
    /// A completion document and optional prepared result, or loading status.
    pub(crate) fn poll(&mut self) -> (Value, Option<PreparedProject>) {
        let Some(job) = &self.job else {
            return (json!({"loading":false}), None);
        };
        let message = match job.receiver.try_recv() {
            Err(TryRecvError::Empty) => return (self.status(), None),
            Err(TryRecvError::Disconnected) => {
                Err("Workbench loader ended without a result.".into())
            }
            Ok(result) => result,
        };
        let job = self
            .job
            .take()
            .expect("Completion requires a tracked loader");
        if job.cancelled.load(Ordering::Relaxed) {
            log::debug!(
                "workbench_open_cancelled elapsed_ms={}",
                job.started.elapsed().as_millis()
            );
            return (json!({"cancelled":true}), None);
        }
        match message {
            Ok(Some(prepared)) => {
                log::debug!(
                    "workbench_open_completed elapsed_ms={}",
                    job.started.elapsed().as_millis()
                );
                (json!({"loading":false}), Some(prepared))
            }
            Ok(None) => (json!({"cancelled":true}), None),
            Err(error) => {
                log::error!(
                    "workbench_open_failed elapsed_ms={}; see the loading error dialog",
                    job.started.elapsed().as_millis()
                );
                (json!({"error":error}), None)
            }
        }
    }

    fn status(&self) -> Value {
        let Some(job) = &self.job else {
            return json!({"loading":false});
        };
        let phase = if job.cancelled.load(Ordering::Relaxed) {
            "Cancelling"
        } else {
            match job.stage.load(Ordering::Relaxed) {
                0 => "Choose a workbench",
                1 => "Reading workbench",
                2 => "Reading configuration",
                _ => "Validating captures and preparing workbench",
            }
        };
        json!({"loading":true,"phase":phase,"elapsed_ms":job.started.elapsed().as_millis() as u64,"debug_ui":crate::platform::startup::debug_ui(),"cancelling":job.cancelled.load(Ordering::Relaxed)})
    }
}
impl Drop for Loader {
    fn drop(&mut self) {
        self.cancel();
    }
}

fn read_bounded(
    reader: &mut impl std::io::Read,
    cancelled: &AtomicBool,
    limit: usize,
) -> Result<Option<Vec<u8>>, String> {
    let mut bytes = Vec::new();
    let mut chunk = [0u8; 64 * 1024];
    loop {
        if cancelled.load(Ordering::Relaxed) {
            return Ok(None);
        }
        let capacity = chunk
            .len()
            .min(limit.saturating_sub(bytes.len()).saturating_add(1));
        let count = reader
            .read(&mut chunk[..capacity])
            .map_err(|error| error.to_string())?;
        if count == 0 {
            return Ok(Some(bytes));
        }
        if bytes.len().saturating_add(count) > limit {
            return Err("Workbench exceeds input limits.".into());
        }
        bytes.extend_from_slice(&chunk[..count]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drain(loader: &mut Loader) -> (Value, Option<PreparedProject>) {
        let deadline = Instant::now() + std::time::Duration::from_secs(60);
        loop {
            let result = loader.poll();
            if result.0["loading"] != true {
                return result;
            }
            assert!(Instant::now() < deadline, "Loader did not complete");
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    #[test]
    fn bounded_stream_preserves_bytes_limits_and_cancellation() {
        let cancelled = AtomicBool::new(false);
        assert_eq!(
            read_bounded(&mut std::io::Cursor::new(b"{}"), &cancelled, 2)
                .unwrap()
                .unwrap(),
            b"{}"
        );
        assert!(read_bounded(&mut std::io::Cursor::new(b"{} "), &cancelled, 2).is_err());
        cancelled.store(true, Ordering::Relaxed);
        assert!(
            read_bounded(&mut std::io::Cursor::new(b"{}"), &cancelled, 2)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn command_line_file_preserves_destination_and_rejects_invalid_json() {
        let mut loader = Loader::default();
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/workbench/single/workbench.merlin.json");
        loader.start_requested(crate::platform::startup::WorkbenchSource::File(
            path.clone(),
        ));
        let (status, prepared) = drain(&mut loader);
        assert!(prepared.is_some(), "{status}");
        assert_eq!(prepared.unwrap().path, Some(path));
        loader.start_requested(crate::platform::startup::WorkbenchSource::File(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
        ));
        assert!(drain(&mut loader).0["error"]
            .as_str()
            .unwrap()
            .starts_with("Invalid workbench JSON"));
    }

    #[test]
    fn polling_does_not_wait_and_cancel_retains_single_worker_guard() {
        let (release, wait) = mpsc::sync_channel(1);
        let mut loader = Loader::default();
        assert_eq!(
            loader.start_with(move |_, _| {
                wait.recv().unwrap();
                Ok(None)
            })["loading"],
            true
        );
        for _ in 0..20 {
            assert_eq!(loader.poll().0["loading"], true);
        }
        loader.cancel();
        assert_eq!(loader.poll().0["cancelling"], true);
        assert!(
            loader.start_with(|_, _| panic!("Must not start another worker"))["error"].is_string()
        );
        release.send(()).unwrap();
        assert_eq!(drain(&mut loader).0["cancelled"], true);
        assert!(!loader.is_loading());
    }

    #[test]
    fn errors_and_cancelled_success_do_not_publish_partial_inputs() {
        let previous = super::super::session::saved_sections();
        let mut loader = Loader::default();
        loader.start_with(|_, _| Err("Invalid capture hash".into()));
        assert_eq!(drain(&mut loader).0["error"], "Invalid capture hash");
        let (release, wait) = mpsc::sync_channel(1);
        loader.start_with(move |_, _| {
            wait.recv().unwrap();
            let project = Project {
                schema_version: 2,
                application: "MERLIN".into(),
                transmission: Default::default(),
                sections: Default::default(),
                receiver: None,
                comparison_enabled: false,
                dark: true,
            };
            PreparedProject::prepare(project, None).map(Some)
        });
        loader.cancel();
        release.send(()).unwrap();
        let (status, result) = drain(&mut loader);
        assert_eq!(status["cancelled"], true);
        assert!(result.is_none());
        assert_eq!(super::super::session::saved_sections(), previous);
    }

    #[test]
    fn real_multi_capture_workbench_prepares_editors_off_thread() {
        let mut loader = Loader::default();
        loader.start_with(|_, _| {
            let project = serde_json::from_slice(include_bytes!(
                "../../../../examples/workbench/multi-rxmer-2/workbench.merlin.json"
            ))
            .unwrap();
            PreparedProject::prepare(project, None).map(Some)
        });
        let (_, result) = drain(&mut loader);
        let prepared = result.expect("Real curated workbench must load");
        assert!(
            prepared
                .models
                .measurements
                .as_ref()
                .unwrap()
                .accepted_rx
                .len()
                > 100
        );
        assert!(prepared.models.receiver.is_some());
        assert_eq!(prepared.metadata["prepared"], true);
        assert_eq!(prepared.metadata["measurement_state"]["channel"]["status"],"complete","Background readiness must use restored inputs, not the worker's empty thread-local session");
        assert!(prepared.metadata.to_string().len() < 8192);
    }
}
