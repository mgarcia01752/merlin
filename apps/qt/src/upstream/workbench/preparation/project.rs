/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Project file work over existing finite preparation and controller-slot transport.
use super::*;
use crate::{
    shared::{project_file, project_path::ProjectPath},
    shell::{ApplicationSessions, Direction, ProjectSnapshot, ReplacePermission},
    upstream::workbench::project::{Opened, Written},
};

impl Pending {
    /// Writes one exact captured revision using its accepted public controller.
    /// # Arguments
    /// * `controller`, `application` - Actual accepted owner and canonical host.
    /// * `expected`, `path` - Original revision and separately admitted destination.
    /// # Returns
    /// Existing passive-query transport, restoring the original controller first.
    /// # Errors
    /// Spawn/admission returns the original controller; worker errors preserve
    /// accepted science and never publish a destination or clean acknowledgement.
    pub(in crate::upstream::workbench) fn project_save(
        controller: UpstreamController,
        application: ApplicationSessions,
        expected: ProjectSnapshot,
        path: ProjectPath,
    ) -> Result<Self, (UpstreamController, String)> {
        let (owner, receiver) = query::controller_query(
            controller,
            application.budget().clone(),
            "merlin-upstream-project-save",
            move |controller, budget| {
                let reservation = budget
                    .reserve_resources(RetainedResources {
                        payload_bytes: std::mem::size_of::<Retained<Written>>(),
                        ..Default::default()
                    })
                    .map_err(|error| format!("{error:?}"))?;
                let bytes = controller.project_document()?;
                if application.snapshot(Direction::Upstream) != expected {
                    return Err("The upstream project changed before writing.".into());
                }
                if crate::platform::input_file::is_original(&application, path.value()) {
                    return Err("Project Save cannot overwrite an imported original input.".into());
                }
                project_file::write_bytes(path.value(), &bytes)?;
                Ok(reservation.retain(Written { path }))
            },
        )?;
        Ok(Self::ProjectSaved {
            expected,
            receiver,
            owner,
        })
    }

    /// Admits a chosen direction project without touching its accepted target.
    /// # Arguments
    /// * `application` - Canonical host accounting for old and prepared owners.
    /// * `expected`, `permission`, `path` - Captured native target/decision/path.
    /// # Returns
    /// One finite prepared candidate; only GUI adoption can replace the target.
    /// # Errors
    /// Admission/spawn rejects immediately; read/public scientific errors cross
    /// the existing bounded failure channel and preserve old state on adoption.
    pub(in crate::upstream::workbench) fn project_open(
        application: ApplicationSessions,
        expected: ProjectSnapshot,
        permission: ReplacePermission,
        path: ProjectPath,
    ) -> Result<Self, String> {
        let budget = application.budget().clone();
        let (sender, receiver) = bounded_channel(&budget, PREPARATION_COMPLETION_CAPACITY)
            .map_err(|error| format!("{error:?}"))?;
        let delivery = sender.reserve().map_err(|error| format!("{error:?}"))?;
        let task_budget = budget.clone();
        let task = admit_task(&task_budget, move || {
            let prepared = (|| {
                let reservation = budget
                    .reserve_resources(RetainedResources {
                        payload_bytes: std::mem::size_of::<Retained<Opened>>(),
                        ..Default::default()
                    })
                    .map_err(|error| format!("{error:?}"))?;
                let project = merlin::common::Project::from_file(path.value())
                    .map_err(|error| error.to_string())?;
                let session = PreparedSession::from_project(&project, &budget)?;
                Ok::<_, String>(reservation.retain(Opened {
                    session,
                    path: Some(path),
                }))
            })()
            .map_err(|message| {
                WorkerFailure::admit_with_fallback(
                    error_text(message).into(),
                    &budget,
                    WORKER_FAILURE_ADMISSION_MESSAGE,
                )
            });
            let _ = delivery.send(prepared);
        })
        .map_err(|error| format!("{error:?}"))?;
        std::thread::Builder::new()
            .name("merlin-upstream-project-open".into())
            .spawn(move || run_task(task))
            .map_err(|error| error.to_string())?;
        Ok(Self::ProjectOpened {
            expected,
            permission,
            receiver,
        })
    }
}
