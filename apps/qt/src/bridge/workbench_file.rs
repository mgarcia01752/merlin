/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Native project Open/Save coordinator with validation before adoption.
use crate::workbench::{
    project_file::{Project, MAX_PROJECT_BYTES},
    session as workbench_session,
};
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
        /// Selects and completely validates a project before replacing accepted inputs.
        /// # Returns
        /// Bounded reload metadata or a diagnostic; cancellation preserves current state.
        #[qinvokable]
        fn open(self: Pin<&mut WorkbenchFile>) -> QString;
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
        /// Restores the private recovery WB after an automatic restart.
        /// # Returns
        /// Validated reload metadata or an error preserving current state.
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
}
fn path(url: &str) -> Result<PathBuf, String> {
    QUrl::from(&QString::from(url))
        .to_local_file()
        .map(|p| PathBuf::from(p.to_string()))
        .ok_or("Choose a local project path.".into())
}
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

fn adopt_project(project: Project) -> Result<serde_json::Value, String> {
    let inputs = project.validate()?;
    let metadata = json!({"opened":true,"transmission":project.transmission,
        "receiver":project.receiver,"verified":project.comparison_enabled,"dark":project.dark});
    workbench_session::restore(inputs, project.sections);
    crate::platform::input_file::clear_original_paths();
    Ok(metadata)
}

impl bridge::WorkbenchFile {
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
        let save = || -> Result<(), String> {
            if settings.to_string().len() > 4096 || receiver.to_string().len() > 4096 {
                return Err("Recovery configuration exceeds limits.".into());
            }
            let project = Project {
                schema_version: 1,
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

    /// Adopts the private recovery WB after a successful update restart.
    /// # Returns
    /// Validated settings for UI reload; errors leave the current WB untouched.
    pub fn restore_update(self: core::pin::Pin<&mut Self>) -> QString {
        let restore = || -> Result<serde_json::Value, String> {
            let path = recovery_path()?;
            let file = std::fs::File::open(path).map_err(|error| error.to_string())?;
            use std::io::Read;
            let mut bytes = Vec::new();
            file.take(MAX_PROJECT_BYTES as u64 + 1)
                .read_to_end(&mut bytes)
                .map_err(|error| error.to_string())?;
            if bytes.len() > MAX_PROJECT_BYTES {
                return Err("Recovery workbench exceeds limits.".into());
            }
            let project = serde_json::from_slice(&bytes)
                .map_err(|error| format!("Invalid recovery WB: {error}"))?;
            adopt_project(project)
        };
        QString::from(
            restore()
                .unwrap_or_else(|error| json!({"error":error}))
                .to_string(),
        )
    }

    /// Opens original project bytes and adopts only a fully valid decoded document.
    /// # Returns
    /// Small reload metadata; all arrays remain in the Rust session.
    pub fn open(mut self: core::pin::Pin<&mut Self>) -> QString {
        let load =
            || -> Result<Option<(PathBuf, Project, crate::workbench::session::Inputs)>, String> {
                let Some(uri) =
                    crate::platform::native_file_picker::choose_workbench("Open MERLIN Workbench")?
                else {
                    return Ok(None);
                };
                let (_, bytes) =
                    crate::platform::input_file::read_untracked(&uri, MAX_PROJECT_BYTES)?;
                let project: Project = serde_json::from_slice(&bytes)
                    .map_err(|e| format!("Invalid workbench JSON: {e}"))?;
                let inputs = project.validate()?;
                Ok(Some((path(&uri)?, project, inputs)))
            };
        let result = match load() {
            Ok(Some((path, project, inputs))) => {
                let metadata = json!({"opened":true,"path":path.to_string_lossy(),"transmission":project.transmission,
                    "receiver":project.receiver,"verified":project.comparison_enabled,"dark":project.dark});
                workbench_session::restore(inputs, project.sections);
                crate::platform::input_file::clear_original_paths();
                self.as_mut().rust_mut().path = Some(path);
                metadata
            }
            Ok(None) => json!({"cancelled":true}),
            Err(error) => json!({"error":error}),
        };
        QString::from(result.to_string())
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
                schema_version: 1,
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
