/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Native project Open/Save coordinator with validation before adoption.
use crate::downstream::{
    common::constants::{
        MAX_OBSERVATION_OPTIONS_DOCUMENT_BYTES, MAX_RECEIVER_OPTIONS_DOCUMENT_BYTES,
        MAX_SAVE_OPTIONS_PACKET_BYTES,
    },
    workbench::{project_file::Project, session as workbench_session},
};
#[cfg(test)]
use crate::shared::project_path::{append_destination_suffix, WORKBENCH_FILE_SUFFIX};
use crate::{
    shared::project_path::{self, normalize_destination, ProjectPath},
    shell::Retained,
};
use cxx_qt::CxxQtType;
use cxx_qt_lib::{QString, QUrl};
use merlin::downstream::MAX_TRANSMISSION_DOCUMENT_BYTES;
use serde_json::json;
use std::path::PathBuf;
mod intent;
#[cxx_qt::bridge]
pub mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("merlin-qt/src/bridge/upstream_session.cxxqt.h");
        type UpstreamSession = crate::bridge::upstream_session::bridge::UpstreamSession;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type WorkbenchFile = super::WorkbenchFileRust;
        /// Creates a blank WB and clears the prior Save destination and accepted inputs.
        /// # Returns
        /// Reload metadata with default settings and no configured run or inputs.
        #[qinvokable]
        fn new_workbench(self: Pin<&mut WorkbenchFile>) -> QString;
        /// Captures the actual downstream New target before its dirty decision.
        /// # Returns
        /// Native action/dirty metadata; accepted state stays untouched.
        #[qinvokable]
        fn request_new_project(self: Pin<&mut WorkbenchFile>) -> QString;
        /// Reads only the pending downstream file action's bounded prompt metadata.
        /// # Returns
        /// Native action/dirty tags, without paths, source documents or revisions.
        #[qinvokable]
        fn file_decision(self: &WorkbenchFile) -> QString;
        /// Resolves the previously captured downstream file action.
        /// # Arguments
        /// * `discard`, `cancel` - Explicit dirty-dialog outcomes for this target.
        /// # Returns
        /// Existing adopted editor metadata or error/cancellation, preserving failure state.
        #[qinvokable]
        fn resolve_project(self: Pin<&mut WorkbenchFile>, discard: bool, cancel: bool) -> QString;
        /// Starts background project selection, validation and preparation.
        /// # Returns
        /// Loading status or diagnostic; poll_open alone adopts completion.
        #[qinvokable]
        fn open(self: Pin<&mut WorkbenchFile>) -> QString;
        /// Collects loading progress or adopts the fully prepared result.
        /// # Returns
        /// Bounded metadata; this never waits for a worker.
        #[qinvokable]
        fn poll_open(self: Pin<&mut WorkbenchFile>) -> QString;
        /// Moves a direction-detected project to its actual native upstream owner.
        /// # Arguments
        /// * `target` - Live GUI-thread upstream QObject; no document passes QML.
        /// # Returns
        /// Empty after staging its exact target dirty decision; original error otherwise.
        /// # Safety
        /// Qt supplies a live pointer of the declared registered QObject type,
        /// exclusively borrowed for this synchronous GUI-thread invocation.
        #[qinvokable]
        unsafe fn handoff_upstream(
            self: Pin<&mut WorkbenchFile>,
            target: *mut UpstreamSession,
        ) -> QString;
        /// Requests cooperative cancellation while preserving the current WB.
        #[qinvokable]
        fn cancel_open(self: &WorkbenchFile);
        /// Saves accepted settings in a private update-recovery WB.
        /// # Arguments
        /// * `settings`, `receiver` - Current accepted configuration JSON.
        /// * `verified`, `dark` - Accepted comparison and appearance choices.
        /// # Returns
        /// Saved/error status; never changes the user's explicit Save destination.
        #[qinvokable]
        fn save_update_recovery(
            self: Pin<&mut WorkbenchFile>,
            settings: &QString,
            receiver: &QString,
            verified: bool,
            dark: bool,
        ) -> QString;
        /// Consumes an optional CLI workbench after the GUI event loop is ready.
        /// # Returns
        /// Background loading metadata or an inactive result; no file chooser.
        #[qinvokable]
        fn startup(self: Pin<&mut WorkbenchFile>) -> QString;
        /// Starts background recovery after an automatic restart.
        /// # Returns
        /// Loading/error metadata; poll_open adopts only validated completion.
        #[qinvokable]
        fn restore_update(self: Pin<&mut WorkbenchFile>) -> QString;
        /// Saves accepted block state and embedded original input bytes atomically.
        /// # Arguments
        /// * `save_as` - True selects a new destination; first Save also prompts.
        /// * `options` - Bounded accepted OCD/receiver/observation settings packet,
        ///   comparison/appearance choices and native runnable readiness.
        /// # Returns
        /// Project path metadata or a diagnostic; no source file is overwritten.
        #[qinvokable]
        fn save(self: Pin<&mut WorkbenchFile>, save_as: bool, options: &QString) -> QString;
    }
}
/// Current explicit workbench destination; project contents stay in Rust-owned sessions.
#[derive(Default)]
pub struct WorkbenchFileRust {
    pub(super) context: crate::downstream::workbench::session::Context,
    path: Option<ProjectPath>,
    loader: crate::downstream::workbench::project_load::Loader,
    intent: Option<Box<Retained<std::cell::RefCell<intent::Intent>>>>,
}

impl WorkbenchFileRust {
    fn create_blank(&mut self) -> Result<Box<Retained<serde_json::Value>>, String> {
        if self.loader.is_loading() || self.intent.is_some() {
            return Err("Wait for workbench loading to finish.".into());
        }
        let candidate =
            crate::downstream::workbench::project_load::PreparedProject::blank(&self.context)?;
        let adopted = candidate.adopt(&self.context)?;
        self.path = adopted.path;
        Ok(adopted.metadata)
    }

    #[cfg(test)]
    fn save_to(
        &mut self,
        destination: Option<PathBuf>,
        settings: &str,
        receiver: &str,
        verified: bool,
        dark: bool,
    ) -> Result<serde_json::Value, String> {
        validate_project_options(settings, receiver)?;
        let expected = self
            .context
            .application
            .snapshot(crate::shell::Direction::Downstream);
        let candidate = match destination {
            Some(destination) => Some(project_path::admit(
                normalize_destination(destination)?,
                self.context.application.budget(),
            )?),
            None => {
                let current = self.path.as_ref().ok_or("Missing project destination")?;
                if current
                    .value()
                    .to_string_lossy()
                    .ends_with(WORKBENCH_FILE_SUFFIX)
                {
                    None
                } else {
                    Some(project_path::admit(
                        append_destination_suffix(current.value())?,
                        self.context.application.budget(),
                    )?)
                }
            }
        };
        let target = candidate
            .as_ref()
            .or(self.path.as_ref())
            .ok_or("Missing project destination")?;
        let project = Project {
            schema_version: crate::downstream::common::constants::WORKBENCH_SCHEMA_VERSION,
            application: "MERLIN".into(),
            transmission: serde_json::from_str(settings).map_err(|error| error.to_string())?,
            sections: workbench_session::saved_sections(&self.context),
            receiver: serde_json::from_str(receiver).map_err(|error| error.to_string())?,
            comparison_enabled: verified,
            dark,
        };
        project.write(target.value(), &self.context.application)?;
        self.context
            .application
            .acknowledge_save(expected)
            .map_err(|error| format!("{error:?}"))?;
        let result = json!({"saved":true,"path":target.value().to_string_lossy()});
        if let Some(candidate) = candidate {
            self.path = Some(candidate);
        }
        Ok(result)
    }
}

fn validate_project_options(settings: &str, receiver: &str) -> Result<(), String> {
    if settings.len() > MAX_TRANSMISSION_DOCUMENT_BYTES
        || receiver.len() > MAX_RECEIVER_OPTIONS_DOCUMENT_BYTES
    {
        return Err("Project options exceed bounds".into());
    }
    Ok(())
}

#[cfg(test)]
#[path = "workbench_file/editor_tests.rs"]
mod editor_tests;
#[cfg(test)]
#[path = "workbench_file/path_tests.rs"]
mod path_tests;
#[cfg(test)]
#[path = "workbench_file/project_consumer_tests.rs"]
mod project_consumer_tests;
fn path(url: &str) -> Result<PathBuf, String> {
    QUrl::from(&QString::from(url))
        .to_local_file()
        .map(|p| PathBuf::from(p.to_string()))
        .ok_or("Choose a local project path.".into())
}
#[cfg(target_os = "linux")]
fn recovery_path() -> Result<PathBuf, String> {
    let base = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/state")))
        .ok_or("No private application state directory.")?;
    let directory = base.join("merlin/update-recovery");
    use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
    std::fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(&directory)
        .map_err(|error| error.to_string())?;
    let metadata = std::fs::symlink_metadata(&directory).map_err(|error| error.to_string())?;
    if !metadata.is_dir() || metadata.permissions().mode() & 0o077 != 0 {
        return Err("Update recovery directory must be private.".into());
    }
    Ok(directory.join("workbench.merlin.json"))
}

#[cfg(target_os = "windows")]
fn recovery_path() -> Result<PathBuf, String> {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .ok_or("No private Windows application data directory.")?;
    let directory = base.join("merlin/update-recovery");
    std::fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    let metadata = std::fs::symlink_metadata(&directory).map_err(|error| error.to_string())?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err("Update recovery must use a normal private directory.".into());
    }
    Ok(directory.join("workbench.merlin.json"))
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn recovery_path() -> Result<PathBuf, String> {
    Err("Automatic update recovery requires a supported installed desktop.".into())
}

impl bridge::WorkbenchFile {
    /// Captures an exact New target without replacing accepted editor state.
    /// # Returns
    /// Bounded dirty-decision metadata or original native admission error.
    pub fn request_new_project(mut self: core::pin::Pin<&mut Self>) -> QString {
        let result = self.as_mut().rust_mut().request_new();
        QString::from(
            result
                .unwrap_or_else(|error| json!({"error":error}))
                .to_string(),
        )
    }

    /// Reads the currently captured native prompt without copying a guard.
    /// # Returns
    /// Action/dirty metadata; no path, scientific document or revision crosses QML.
    pub fn file_decision(&self) -> QString {
        QString::from(self.rust().decision().to_string())
    }

    /// Resolves the actual native target captured before the prompt.
    /// # Arguments
    /// * `discard`, `cancel` - Explicit outcome for the captured revision only.
    /// # Returns
    /// Adopted editor metadata or bounded error/cancellation state.
    pub fn resolve_project(
        mut self: core::pin::Pin<&mut Self>,
        discard: bool,
        cancel: bool,
    ) -> QString {
        let result = self.as_mut().rust_mut().resolve_intent(discard, cancel);
        QString::from(
            result
                .unwrap_or_else(|error| json!({"error":error}))
                .to_string(),
        )
    }
    /// Transfers one complete prepared owner between the existing Qt adapters.
    /// # Arguments
    /// * `target` - Declared registered upstream QObject supplied by the main shell.
    /// # Returns
    /// Empty after same-host staging; null, missing or stale/busy admission rejects
    /// without replacing either accepted direction. No scientific data is copied.
    /// # Safety
    /// The nonnull target must be live on this GUI thread and exclusively borrowed
    /// for the call. Qt's typed QObject invocation and main-shell ownership provide
    /// these conditions. Unsafe code is confined to this presentation interop.
    pub unsafe fn handoff_upstream(
        mut self: core::pin::Pin<&mut Self>,
        target: *mut bridge::UpstreamSession,
    ) -> QString {
        let result = (|| {
            // Qt holds the object throughout the synchronous typed invocation.
            let target =
                unsafe { target.as_mut() }.ok_or("Upstream project target is unavailable.")?;
            let mut target = unsafe { core::pin::Pin::new_unchecked(target) };
            let candidate = self
                .as_mut()
                .rust_mut()
                .loader
                .take_upstream()
                .ok_or("No upstream project is ready for handoff.")?;
            target
                .as_mut()
                .rust_mut()
                .ensure_model()
                .ok_or(crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE)?
                .stage_prepared_project(candidate)
        })();
        QString::from(result.err().unwrap_or_default())
    }

    /// Creates an explicit blank WB without saving or overwriting any file.
    /// # Returns
    /// Default settings for editor reload, with no accepted inputs or run options.
    pub fn new_workbench(mut self: core::pin::Pin<&mut Self>) -> QString {
        let result = self.as_mut().rust_mut().create_blank();
        match result {
            Ok(metadata) => QString::from(metadata.value().to_string()),
            Err(error) => QString::from(json!({"error":error}).to_string()),
        }
    }

    /// Stores a private recovery WB without overwriting a user-selected file.
    /// # Arguments
    /// * `settings`, `receiver` - Accepted configuration JSON, each at most 4096 bytes.
    /// * `verified`, `dark` - Accepted comparison and appearance choices.
    /// # Returns
    /// Saved/error JSON; failed validation prevents update installation.
    pub fn save_update_recovery(
        self: core::pin::Pin<&mut Self>,
        settings: &QString,
        receiver: &QString,
        verified: bool,
        dark: bool,
    ) -> QString {
        if self.rust().loader.is_loading() {
            return QString::from("{\"error\":\"Wait for workbench loading to finish.\"}");
        }
        let save = || -> Result<(), String> {
            if settings.to_string().len() > MAX_TRANSMISSION_DOCUMENT_BYTES
                || receiver.to_string().len() > MAX_RECEIVER_OPTIONS_DOCUMENT_BYTES
            {
                return Err("Recovery configuration exceeds limits.".into());
            }
            let project = Project {
                schema_version: crate::downstream::common::constants::WORKBENCH_SCHEMA_VERSION,
                application: "MERLIN".into(),
                transmission: serde_json::from_str(&settings.to_string())
                    .map_err(|error| error.to_string())?,
                sections: workbench_session::saved_sections(&self.rust().context),
                receiver: serde_json::from_str(&receiver.to_string())
                    .map_err(|error| error.to_string())?,
                comparison_enabled: verified,
                dark,
            };
            project.write(&recovery_path()?, &self.rust().context.application)
        };
        QString::from(
            match save() {
                Ok(()) => json!({"saved":true}),
                Err(error) => json!({"error":error}),
            }
            .to_string(),
        )
    }

    /// Starts the one-shot command-line workbench through the normal loader.
    /// # Returns
    /// Loading status, or an inactive document for ordinary desktop startup.
    pub fn startup(mut self: core::pin::Pin<&mut Self>) -> QString {
        let result = match crate::platform::startup::take_workbench() {
            Some(source) => {
                let context = self.rust().context.clone();
                self.as_mut()
                    .rust_mut()
                    .loader
                    .start_requested(&context, source)
            }
            None => json!({"loading":false}),
        };
        QString::from(result.to_string())
    }

    /// Starts private recovery without waiting for capture validation.
    /// # Returns
    /// Loading metadata; poll_open preserves the current WB on any failure.
    pub fn restore_update(mut self: core::pin::Pin<&mut Self>) -> QString {
        let result = match recovery_path() {
            Ok(path) => {
                let context = self.rust().context.clone();
                self.as_mut().rust_mut().loader.start(&context, Some(path))
            }
            Err(error) => json!({"error":error}),
        };
        QString::from(result.to_string())
    }

    /// Starts background Open without blocking Qt's window-event processing.
    /// # Returns
    /// Loading status or a diagnostic; completion is collected by poll_open.
    pub fn open(mut self: core::pin::Pin<&mut Self>) -> QString {
        if self.rust().intent.is_some() {
            return QString::from(
                json!({"error":"Finish the pending project decision first."}).to_string(),
            );
        }
        let context = self.rust().context.clone();
        QString::from(
            self.as_mut()
                .rust_mut()
                .loader
                .start(&context, None)
                .to_string(),
        )
    }

    /// Collects completed Open and publishes only a fully prepared project.
    /// # Returns
    /// Bounded loading/error/adoption metadata, never capture arrays.
    pub fn poll_open(mut self: core::pin::Pin<&mut Self>) -> QString {
        let (status, prepared) = self.as_mut().rust_mut().loader.poll();
        let result = if let Some(prepared) = prepared {
            self.as_mut()
                .rust_mut()
                .stage_open(prepared)
                .unwrap_or_else(|error| json!({"error":error}))
        } else if status["write_completed"] == true {
            self.as_mut()
                .rust_mut()
                .adopt_saved()
                .unwrap_or_else(|error| json!({"error":error}))
        } else {
            status
        };
        QString::from(result.to_string())
    }

    /// Cancels pending Open without waiting or replacing accepted inputs.
    pub fn cancel_open(&self) {
        self.rust().loader.cancel();
    }

    /// Saves current accepted state without converting samples or depending on source paths.
    /// # Arguments
    /// * `save_as` - Prompt for a new destination even if Save already has one.
    /// * `options` - Bounded accepted OCD/receiver/observation settings packet,
    ///   comparison/appearance choices and native runnable readiness. Scientific
    ///   validation, serialization and I/O execute on the existing file worker.
    /// # Returns
    /// Success/path or explicit error/cancellation, preserving the previous file on failure.
    pub fn save(mut self: core::pin::Pin<&mut Self>, save_as: bool, options: &QString) -> QString {
        if self.rust().loader.is_loading() {
            return QString::from("{\"error\":\"Wait for workbench loading to finish.\"}");
        }
        if options.len() > MAX_SAVE_OPTIONS_PACKET_BYTES as isize {
            return QString::from(
                json!({"error":"Project Save options exceed the packet byte limit."}).to_string(),
            );
        }
        let options = options.to_string().into_bytes();
        if let Err(error) = intent::validate_save_packet(&options) {
            return QString::from(json!({"error":error}).to_string());
        }
        let destination = if save_as || self.rust().path.is_none() {
            match crate::platform::native_file_picker::save("Save MERLIN Workbench") {
                Ok(Some(uri)) => match path(&uri) {
                    Ok(path) => Some(path),
                    Err(error) => return QString::from(json!({"error":error}).to_string()),
                },
                Ok(None) => return QString::from("{\"cancelled\":true}"),
                Err(error) => return QString::from(json!({"error":error}).to_string()),
            }
        } else {
            None
        };
        let result = self
            .as_mut()
            .rust_mut()
            .queue_save(destination, options)
            .unwrap_or_else(|error| json!({"error":error}));
        QString::from(result.to_string())
    }
}
