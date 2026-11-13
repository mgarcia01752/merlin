/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Native project Open/Save coordinator with validation before adoption.
use crate::workbench::{project_file::Project, session as workbench_session};
use cxx_qt::CxxQtType;
use cxx_qt_lib::{QString, QUrl};
use serde_json::json;
use std::path::PathBuf;
#[cxx_qt::bridge]
pub mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
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
        /// * `settings` - Current bounded OCD JSON.
        /// * `receiver` - Accepted receiver options, null if unconfigured.
        /// * `verified` - Independent source-comparison acceptance.
        /// * `dark` - Global appearance choice.
        /// # Returns
        /// Project path metadata or a diagnostic; no source file is overwritten.
        #[qinvokable]
        fn save(
            self: Pin<&mut WorkbenchFile>,
            save_as: bool,
            settings: &QString,
            receiver: &QString,
            verified: bool,
            dark: bool,
        ) -> QString;
    }
}
/// Current explicit workbench destination; project contents stay in Rust-owned sessions.
#[derive(Default)]
pub struct WorkbenchFileRust {
    path: Option<PathBuf>,
    loader: crate::workbench::project_load::Loader,
}
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

#[cfg(not(target_os = "linux"))]
fn recovery_path() -> Result<PathBuf, String> {
    Err("Automatic update recovery is available on Ubuntu only.".into())
}

impl bridge::WorkbenchFile {
    /// Creates an explicit blank WB without saving or overwriting any file.
    /// # Returns
    /// Default settings for editor reload, with no accepted inputs or run options.
    pub fn new_workbench(mut self: core::pin::Pin<&mut Self>) -> QString {
        if self.rust().loader.is_loading() {
            return QString::from("{\"error\":\"Wait for workbench loading to finish.\"}");
        }
        workbench_session::restore(Default::default(), Default::default());
        crate::platform::input_file::clear_original_paths();
        self.as_mut().rust_mut().path = None;
        QString::from(json!({"created":true,"transmission":docsis_ofdm_fec::transmission::TransmissionSettings::default(),
            "receiver":null,"verified":false,"dark":null}).to_string())
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
            if settings.to_string().len() > 4096 || receiver.to_string().len() > 4096 {
                return Err("Recovery configuration exceeds limits.".into());
            }
            let project = Project {
                schema_version: 2,
                application: "MERLIN".into(),
                transmission: serde_json::from_str(&settings.to_string())
                    .map_err(|error| error.to_string())?,
                sections: workbench_session::saved_sections(),
                receiver: serde_json::from_str(&receiver.to_string())
                    .map_err(|error| error.to_string())?,
                comparison_enabled: verified,
                dark,
            };
            project.write(&recovery_path()?)
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
            Some(source) => self.as_mut().rust_mut().loader.start_requested(source),
            None => json!({"loading":false}),
        };
        QString::from(result.to_string())
    }

    /// Starts private recovery without waiting for capture validation.
    /// # Returns
    /// Loading metadata; poll_open preserves the current WB on any failure.
    pub fn restore_update(mut self: core::pin::Pin<&mut Self>) -> QString {
        let result = match recovery_path() {
            Ok(path) => self.as_mut().rust_mut().loader.start(Some(path)),
            Err(error) => json!({"error":error}),
        };
        QString::from(result.to_string())
    }

    /// Starts background Open without blocking Qt's window-event processing.
    /// # Returns
    /// Loading status or a diagnostic; completion is collected by poll_open.
    pub fn open(mut self: core::pin::Pin<&mut Self>) -> QString {
        QString::from(self.as_mut().rust_mut().loader.start(None).to_string())
    }

    /// Collects completed Open and publishes only a fully prepared project.
    /// # Returns
    /// Bounded loading/error/adoption metadata, never capture arrays.
    pub fn poll_open(mut self: core::pin::Pin<&mut Self>) -> QString {
        let (status, prepared) = self.as_mut().rust_mut().loader.poll();
        let result = if let Some(prepared) = prepared {
            self.as_mut().rust_mut().path = prepared.path.clone();
            prepared.adopt()
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
    /// * `settings` - Validated OCD JSON, bounded to 4096 bytes.
    /// * `receiver` - Accepted options or null, bounded to 4096 bytes.
    /// * `verified` - Comparison setup.
    /// * `dark` - Appearance preference.
    /// # Returns
    /// Success/path or explicit error/cancellation, preserving the previous file on failure.
    pub fn save(
        mut self: core::pin::Pin<&mut Self>,
        save_as: bool,
        settings: &QString,
        receiver: &QString,
        verified: bool,
        dark: bool,
    ) -> QString {
        if self.rust().loader.is_loading() {
            return QString::from("{\"error\":\"Wait for workbench loading to finish.\"}");
        }
        let save = || -> Result<Option<PathBuf>, String> {
            if settings.to_string().len() > 4096 || receiver.to_string().len() > 4096 {
                return Err("Project options exceed bounds".into());
            }
            let mut destination = if save_as || self.rust().path.is_none() {
                let Some(uri) = crate::platform::native_file_picker::save("Save MERLIN Workbench")?
                else {
                    return Ok(None);
                };
                path(&uri)?
            } else {
                self.rust()
                    .path
                    .clone()
                    .ok_or("Missing project destination")?
            };
            if !destination.to_string_lossy().ends_with(".merlin.json") {
                destination = PathBuf::from(format!("{}.merlin.json", destination.display()));
                if destination.exists() {
                    return Err("Choose the full .merlin.json filename to confirm replacement of that project.".into());
                }
            }
            let project = Project {
                schema_version: 2,
                application: "MERLIN".into(),
                transmission: serde_json::from_str(&settings.to_string())
                    .map_err(|e| e.to_string())?,
                sections: workbench_session::saved_sections(),
                receiver: serde_json::from_str(&receiver.to_string()).map_err(|e| e.to_string())?,
                comparison_enabled: verified,
                dark,
            };
            project.write(&destination)?;
            Ok(Some(destination))
        };
        let result = match save() {
            Ok(Some(path)) => {
                self.as_mut().rust_mut().path = Some(path.clone());
                json!({"saved":true,"path":path.to_string_lossy()})
            }
            Ok(None) => json!({"cancelled":true}),
            Err(error) => json!({"error":error}),
        };
        QString::from(result.to_string())
    }
}
