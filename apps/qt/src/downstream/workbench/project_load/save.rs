/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Save reuses the existing single-flight file transport and accepted roots.
use super::*;
use crate::shell::{ApplicationSessions, Direction, ProjectSnapshot};

/// Bounded accepted editor settings packet, never a second scientific model.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SaveOptions {
    pub(crate) settings: String,
    pub(crate) receiver: String,
    pub(crate) observations: String,
    pub(crate) verified: bool,
    pub(crate) dark: bool,
    pub(crate) runnable: bool,
}

/// Actual accepted section owner and original native revision held by the worker.
pub(crate) struct SaveRequest {
    pub(crate) options: Box<Retained<Vec<u8>>>,
    pub(crate) sections: super::super::session::SectionSnapshot,
    pub(crate) path: ProjectPath,
    pub(crate) expected: ProjectSnapshot,
    pub(crate) application: ApplicationSessions,
}

/// Successful write awaiting exact GUI-thread acknowledgement and path adoption.
pub(crate) struct Saved {
    pub(crate) path: ProjectPath,
    pub(crate) expected: ProjectSnapshot,
}

impl Loader {
    /// Queues one exact revision's validation, serialization and atomic write.
    /// # Arguments
    /// * `request` - Admitted options/path and immutable actual accepted sections.
    /// # Returns
    /// Existing file-worker progress; only later GUI acknowledgement marks clean.
    /// Dispatched atomic Save is not cancelled by the Open cancellation control.
    /// # Errors
    /// Transport/spawn or queued validation/write error leaves old accepted data,
    /// destination and dirty state unchanged; original sources remain protected.
    pub(crate) fn start_save(&mut self, request: SaveRequest) -> Value {
        let budget = request.application.budget().clone();
        let status = self.start_candidate(&budget, move |_, _| {
            let SaveOptions {
                settings,
                receiver,
                observations,
                verified,
                dark,
                runnable,
            } = serde_json::from_slice(request.options.value())
                .map_err(|error| error.to_string())?;
            let project = Project {
                schema_version: crate::downstream::common::constants::WORKBENCH_SCHEMA_VERSION,
                application: "MERLIN".into(),
                transmission: serde_json::from_str(&settings).map_err(|error| error.to_string())?,
                sections: request.sections,
                receiver: serde_json::from_str(&receiver).map_err(|error| error.to_string())?,
                comparison_enabled: verified,
                dark,
            };
            project.validate()?;
            let bytes = if runnable {
                let observations: Option<merlin::downstream::FullLoopObservationInput> =
                    serde_json::from_str(&observations).map_err(|error| error.to_string())?;
                crate::downstream::workbench::project_envelope::encode_runnable(
                    &project,
                    observations.as_ref(),
                )?
            } else {
                serde_json::to_vec_pretty(&project).map_err(|error| error.to_string())?
            };
            if request.application.snapshot(Direction::Downstream) != request.expected {
                return Err("The downstream project changed before writing.".into());
            }
            if crate::platform::input_file::is_original(&request.application, request.path.value())
            {
                return Err("Project Save cannot overwrite an imported original input.".into());
            }
            let reservation = request
                .application
                .budget()
                .reserve_resources(RetainedResources {
                    payload_bytes: std::mem::size_of::<Retained<Saved>>(),
                    ..Default::default()
                })
                .map_err(|_| "The session retained-data limit would be exceeded.".to_owned())?;
            crate::shared::project_file::write_bytes(request.path.value(), &bytes)?;
            Ok(Some(LoadCandidate::Saved(Box::new(reservation.retain(
                Saved {
                    path: request.path,
                    expected: request.expected,
                },
            )))))
        });
        if status["loading"] == true {
            if let Some(job) = self.job.as_mut() {
                job.saving = true;
            }
            return self.status();
        }
        status
    }
}
