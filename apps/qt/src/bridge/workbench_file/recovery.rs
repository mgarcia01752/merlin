/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! One bounded private restart operation over existing directional project owners.
use super::*;
use crate::{
    downstream::workbench::{
        project_load::{PreparedProject, SaveOptions},
        session::{self, SectionSnapshot, Snapshot},
    },
    shared::{
        update_recovery::{Bundle, OriginalPath, ProjectState, MAX_RECOVERY_BYTES},
        worker_failure::WorkerFailure,
    },
    shell::{
        admit_task, bounded_channel, run_task, ApplicationSessions, DeliveryReceiver, Direction,
        ProjectSnapshot, RetainedResources, RetentionBudget,
    },
    upstream::{workbench::Model, PreparedSession, ProjectSource},
};
use std::{io::Read, mem::size_of, path::Path, sync::mpsc::TryRecvError};

// One recovery operation owns exactly one terminal result, independently of run traffic.
const RECOVERY_COMPLETION_CAPACITY: usize = 1;
type Guards = (ProjectSnapshot, ProjectSnapshot);

pub(super) struct Job {
    receiver: DeliveryReceiver<Result<Product, WorkerFailure>>,
}
enum Product {
    Written {
        expected: Guards,
        selected: Direction,
        originals: Box<Retained<Vec<OriginalPath>>>,
    },
    Prepared {
        expected: Guards,
        downstream: Box<PreparedProject>,
        upstream: Option<PreparedSession>,
        upstream_path: Option<ProjectPath>,
        paths: crate::shell::InputPaths,
        expected_paths: Box<Retained<Vec<OriginalPath>>>,
        dirty: (bool, bool),
        selected: Direction,
    },
}

fn guards(application: &ApplicationSessions) -> Guards {
    (
        application.snapshot(Direction::Downstream),
        application.snapshot(Direction::Upstream),
    )
}
fn direction(direction: merlin::common::ProjectDirection) -> Direction {
    match direction {
        merlin::common::ProjectDirection::Downstream => Direction::Downstream,
        merlin::common::ProjectDirection::Upstream => Direction::Upstream,
    }
}
fn public_direction(direction: Direction) -> merlin::common::ProjectDirection {
    match direction {
        Direction::Downstream => merlin::common::ProjectDirection::Downstream,
        Direction::Upstream => merlin::common::ProjectDirection::Upstream,
    }
}
fn check(application: &ApplicationSessions, expected: Guards) -> Result<(), String> {
    if application.active().is_some() || guards(application) != expected {
        return Err(
            "A project changed or began execution during update recovery. Retry the update.".into(),
        );
    }
    Ok(())
}
fn launch(
    application: &ApplicationSessions,
    work: impl FnOnce() -> Result<Product, String> + Send + 'static,
) -> Result<Box<Retained<Job>>, String> {
    let budget = application.budget();
    let (sender, receiver) = bounded_channel(budget, RECOVERY_COMPLETION_CAPACITY)
        .map_err(|error| format!("{error:?}"))?;
    // The actual delivery frame already includes the inline Product and error
    // enum. Children have their own guards; no second Product wrapper is reserved.
    let delivery = sender.reserve().map_err(|error| format!("{error:?}"))?;
    drop(sender);
    let fixed = budget
        .reserve_resources(RetainedResources {
            payload_bytes: size_of::<Retained<Job>>(),
            ..Default::default()
        })
        .map_err(|error| format!("{error:?}"))?;
    let worker_budget = budget.clone();
    let task = admit_task(budget, move || {
        let result = work().map_err(|error| {
            WorkerFailure::admit_with_fallback(
                crate::shared::worker_failure::bounded_message(error).into(),
                &worker_budget,
                "Update recovery failed; retained-data admission rejected its details.",
            )
        });
        let _ = delivery.send(result);
    })
    .map_err(|error| format!("{error:?}"))?;
    std::thread::Builder::new()
        .name("merlin-update-recovery".into())
        .spawn(move || run_task(task))
        .map_err(|error| error.to_string())?;
    Ok(Box::new(fixed.retain(Job { receiver })))
}
fn admit_options(
    options: Vec<u8>,
    budget: &RetentionBudget,
) -> Result<Box<Retained<Vec<u8>>>, String> {
    intent::validate_save_packet(&options)?;
    let bytes = size_of::<Retained<Vec<u8>>>()
        .checked_add(options.capacity())
        .ok_or("Recovery option capacity overflowed.")?;
    Ok(Box::new(
        budget
            .reserve_resources(RetainedResources {
                payload_bytes: bytes,
                ..Default::default()
            })
            .map_err(|error| format!("{error:?}"))?
            .retain(options),
    ))
}
fn copy_path(path: Option<&Path>, budget: &RetentionBudget) -> Result<Option<ProjectPath>, String> {
    // A restarted installer may choose another working directory. Resolve only
    // this recovery copy; the live destination and ordinary Save stay unchanged.
    path.map(|path| {
        let absolute = std::path::absolute(path).map_err(|error| error.to_string())?;
        project_path::admit(absolute, budget)
    })
    .transpose()
}

/// Encodes one bounded native recovery response for the thin Qt adapter.
/// # Arguments
/// * `result` - Existing scalar/editor metadata or the original native error.
/// # Returns
/// JSON metadata with errors bounded by the shared diagnostic-character policy.
pub(super) fn response(result: Result<serde_json::Value, String>) -> QString {
    QString::from(
        result
            .unwrap_or_else(
                |error| json!({"error":crate::shared::worker_failure::bounded_message(error)}),
            )
            .to_string(),
    )
}

/// Borrows a live Qt-owned upstream target for one synchronous native transaction.
/// # Arguments
/// * `file`, `target` - Exclusive same-thread QObject model owners.
/// * `operation` - Completes before either GUI borrow is released.
/// # Returns
/// Original operation result; no QObject or model pointer is retained by a worker.
/// # Errors
/// Null target or model admission failure before invoking the operation.
/// # Safety
/// `target` must identify a live registered upstream QObject on its owning thread,
/// with no other outstanding mutable borrow for the duration of this call.
pub(super) unsafe fn with_target<T>(
    file: &mut WorkbenchFileRust,
    target: *mut bridge::UpstreamSession,
    operation: impl FnOnce(&mut WorkbenchFileRust, &mut Model) -> Result<T, String>,
) -> Result<T, String> {
    // SAFETY: Caller guarantees live same-thread exclusive ownership; null rejects.
    let target = unsafe { target.as_mut() }.ok_or("Upstream recovery target is unavailable.")?;
    // SAFETY: Qt owns a pinned QObject; this temporary Pin never moves it.
    let mut target = unsafe { core::pin::Pin::new_unchecked(target) };
    let mut native = target.as_mut().rust_mut();
    let model = native
        .ensure_model()
        .ok_or("Upstream recovery model admission failed.")?;
    operation(file, model)
}

impl WorkbenchFileRust {
    fn recovery_idle(&self, upstream: &Model) -> Result<(), String> {
        if self.recovery.is_some() || self.loader.is_loading() || self.intent.is_some() {
            return Err("Finish the pending project file action before update recovery.".into());
        }
        upstream.ensure_recovery_target(&self.context.application)?;
        upstream.recovery_source()?;
        if self.context.application.active().is_some() {
            return Err("Stop the simulation before update recovery.".into());
        }
        Ok(())
    }

    /// Starts a bounded off-GUI dual-project write without acknowledging normal Save.
    /// # Arguments
    /// * `upstream` - Genuine same-host direction owner, exclusively borrowed on GUI.
    /// * `options` - Existing validated downstream Save packet with actual observations.
    /// * `path` - Private recovery destination obtained by the platform adapter.
    /// # Returns
    /// Preparing status; installation must await completed successful polling.
    /// # Errors
    /// Busy/foreign target, packet/path/transport admission or thread creation failure.
    pub(super) fn start_recovery_write(
        &mut self,
        upstream: &Model,
        options: Vec<u8>,
        path: PathBuf,
    ) -> Result<serde_json::Value, String> {
        self.recovery_idle(upstream)?;
        let application = self.context.application.clone();
        let options = admit_options(options, application.budget())?;
        let source = upstream.recovery_source()?;
        let downstream_path = copy_path(
            self.path.as_ref().map(|path| path.value().as_path()),
            application.budget(),
        )?;
        let upstream_path = copy_path(upstream.project_path(), application.budget())?;
        let sections = session::saved_sections(&self.context);
        let expected = guards(&application);
        let selected = application.selected_direction();
        let originals = application.snapshot_input_paths()?;
        let destination = project_path::admit(path, application.budget())?;
        let worker_application = application.clone();
        self.recovery = Some(launch(&application, move || {
            write_bundle(
                worker_application,
                *options,
                sections,
                source,
                downstream_path,
                upstream_path,
                expected,
                selected,
                originals,
                *destination,
            )
        })?);
        Ok(json!({"preparing":true}))
    }

    /// Starts complete dual preparation while preserving every live native owner.
    /// # Arguments
    /// * `upstream` - Idle genuine target; guards remain native throughout preparation.
    /// * `path` - Platform-selected private bundle, never a normal Save destination.
    /// # Returns
    /// Preparing status; only completed polling may publish the pair.
    /// # Errors
    /// Pending work, foreign host or task admission/spawn failure.
    pub(super) fn start_recovery_restore(
        &mut self,
        upstream: &Model,
        path: PathBuf,
    ) -> Result<serde_json::Value, String> {
        self.recovery_idle(upstream)?;
        let application = self.context.application.clone();
        let expected = guards(&application);
        let accepted = session::snapshot(&self.context);
        let expected_paths = application.snapshot_input_paths()?;
        let path = project_path::admit(path, application.budget())?;
        let worker_application = application.clone();
        self.recovery = Some(launch(&application, move || {
            prepare_bundle(
                worker_application,
                *path,
                expected,
                accepted,
                expected_paths,
            )
        })?);
        Ok(json!({"preparing":true}))
    }

    /// Collects a real terminal write or atomically adopts both prepared projects.
    /// # Arguments
    /// * `upstream` - Exact same-host native owner, not the currently selected tab.
    /// # Returns
    /// Preparing, completed saved/restored metadata or the original bounded error.
    /// # Errors
    /// Stale/active target, failed preparation/write or disconnected worker leaves
    /// accepted projects and normal destinations unchanged. No Save is acknowledged.
    pub(super) fn poll_recovery(
        &mut self,
        upstream: &mut Model,
    ) -> Result<serde_json::Value, String> {
        let Some(job) = self.recovery.as_ref() else {
            return Ok(json!({"preparing":false}));
        };
        let product = match job.value().receiver.try_recv() {
            Ok(product) => product,
            Err(TryRecvError::Empty) => return Ok(json!({"preparing":true})),
            Err(TryRecvError::Disconnected) => {
                self.recovery = None;
                return Err("Update recovery worker disconnected.".into());
            }
        };
        self.recovery = None;
        let (product, _delivery) = product.into_parts();
        let product = product.map_err(|error| error.message().to_owned())?;
        match product {
            Product::Written {
                expected,
                selected,
                originals,
            } => {
                let application = &self.context.application;
                check(application, expected)?;
                if application.selected_direction() != selected
                    || !application.input_paths_match(originals.value())
                {
                    return Err("Presentation or original inputs changed during update recovery. Retry the update.".into());
                }
                Ok(json!({"saved":true,"preparing":false}))
            }
            Product::Prepared {
                expected,
                downstream,
                upstream: prepared,
                upstream_path,
                paths,
                expected_paths,
                dirty,
                selected,
            } => {
                upstream.ensure_recovery_target(&self.context.application)?;
                upstream.recovery_source()?;
                check(&self.context.application, expected)?;
                let application = self.context.application.clone();
                let ready = prepared.is_some();
                let adopted = downstream.adopt_transaction(&self.context, |downstream_ready| {
                    let keys = application
                        .restore_projects(
                            expected,
                            (downstream_ready, ready),
                            dirty,
                            selected,
                            paths,
                            expected_paths.value(),
                        )
                        .map_err(|error| format!("{error:?}"))?;
                    upstream.attach_recovery(prepared, upstream_path, keys.1);
                    Ok(keys.0)
                })?;
                self.path = adopted.path;
                // Bounded metadata already admitted by downstream preparation;
                // return only that existing editor projection and scalar route tags.
                let mut metadata = adopted.metadata.value().clone();
                metadata["restored"] = json!(true);
                metadata["selected_direction"] = json!(match selected {
                    Direction::Downstream => "downstream",
                    Direction::Upstream => "upstream",
                });
                metadata["preparing"] = json!(false);
                Ok(metadata)
            }
        }
    }
}

fn destination_identity(path: &Path) -> Result<PathBuf, String> {
    if let Ok(path) = std::fs::canonicalize(path) {
        return Ok(path);
    }
    let absolute = std::path::absolute(path).map_err(|error| error.to_string())?;
    if let (Some(parent), Some(name)) = (absolute.parent(), absolute.file_name()) {
        if let Ok(parent) = std::fs::canonicalize(parent) {
            return Ok(parent.join(name));
        }
    }
    Ok(absolute)
}

fn reject_private_destinations(
    downstream: Option<&Path>,
    upstream: Option<&Path>,
    private: &Path,
) -> Result<(), String> {
    let private = destination_identity(private)?;
    for destination in [downstream, upstream].into_iter().flatten() {
        let destination = destination_identity(destination)?;
        // Windows private state uses the ordinary case-insensitive namespace;
        // conservative case matching never permits an internal file overwrite.
        let same = if cfg!(target_os = "windows") {
            destination
                .to_string_lossy()
                .eq_ignore_ascii_case(&private.to_string_lossy())
        } else {
            destination == private
        };
        if same {
            return Err("A project Save destination points to private update recovery. Choose Save As outside that directory before updating.".into());
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn write_bundle(
    application: ApplicationSessions,
    options: Retained<Vec<u8>>,
    sections: SectionSnapshot,
    source: Option<ProjectSource>,
    downstream_path: Option<ProjectPath>,
    upstream_path: Option<ProjectPath>,
    expected: Guards,
    selected: Direction,
    originals: Box<Retained<Vec<OriginalPath>>>,
    destination: Retained<PathBuf>,
) -> Result<Product, String> {
    reject_private_destinations(
        downstream_path.as_ref().map(|path| path.value().as_path()),
        upstream_path.as_ref().map(|path| path.value().as_path()),
        destination.value(),
    )?;

    let options: SaveOptions =
        serde_json::from_slice(options.value()).map_err(|error| error.to_string())?;
    let dark = options.dark;
    let downstream = options.encode(sections)?;
    let upstream = source.map(|source| source.document()).transpose()?;
    let bundle = Bundle {
        downstream: ProjectState {
            document: Some(downstream),
            path: downstream_path.as_ref().map(|path| path.value().clone()),
            dirty: expected.0.dirty,
        },
        upstream: ProjectState {
            document: upstream,
            path: upstream_path.as_ref().map(|path| path.value().clone()),
            dirty: expected.1.dirty,
        },
        dark,
        selected: public_direction(selected),
        original_paths: originals.value().clone(),
    };
    let bytes = bundle.encode()?;
    check(&application, expected)?;
    if !application.input_paths_match(originals.value())
        || application.selected_direction() != selected
    {
        return Err(
            "Inputs or presentation changed before recovery writing. Retry the update.".into(),
        );
    }
    if crate::platform::input_file::is_original(&application, destination.value()) {
        return Err("Update recovery cannot overwrite an imported original input.".into());
    }
    crate::shared::project_file::write_recovery_bytes(
        destination.value(),
        &bytes,
        MAX_RECOVERY_BYTES,
    )?;
    Ok(Product::Written {
        expected,
        selected,
        originals,
    })
}

// Previous installed versions wrote a single schema-1/2 downstream editor at
// this same private path. Recognize that exact legacy envelope explicitly; a
// recognized new bundle is never retried as legacy after any decoding failure.
fn format_present<'input, D: serde::Deserializer<'input>>(decoder: D) -> Result<bool, D::Error> {
    let _: serde::de::IgnoredAny = serde::Deserialize::deserialize(decoder)?;
    Ok(true)
}

fn decode_bundle(bytes: &[u8]) -> Result<Bundle, String> {
    if bytes.len() > MAX_RECOVERY_BYTES {
        return Err("Update recovery exceeds its byte limit.".into());
    }
    #[derive(serde::Deserialize)]
    struct LegacyDiscriminator {
        #[serde(rename = "format", default, deserialize_with = "format_present")]
        format_present: bool,
        application: Option<String>,
        schema_version: Option<u32>,
    }
    let header: LegacyDiscriminator =
        serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    if !header.format_present
        && header.application.as_deref() == Some("MERLIN")
        && header.schema_version.is_some_and(|schema| {
            crate::downstream::workbench::project_envelope::LEGACY_EDITOR_SCHEMAS.contains(&schema)
        })
    {
        let decoded = crate::downstream::workbench::project_envelope::decode(bytes)?;
        return Ok(Bundle {
            downstream: ProjectState {
                document: Some(bytes.into()),
                path: None,
                dirty: true,
            },
            upstream: ProjectState {
                document: None,
                path: None,
                dirty: false,
            },
            selected: merlin::common::ProjectDirection::Downstream,
            dark: decoded.project.dark,
            original_paths: Vec::new(),
        });
    }
    Bundle::decode(bytes)
}

fn prepare_bundle(
    application: ApplicationSessions,
    path: Retained<PathBuf>,
    expected: Guards,
    accepted: Snapshot,
    expected_paths: Box<Retained<Vec<OriginalPath>>>,
) -> Result<Product, String> {
    let file = std::fs::File::open(path.value()).map_err(|error| error.to_string())?;
    if !file
        .metadata()
        .map_err(|error| error.to_string())?
        .is_file()
    {
        return Err("Choose a regular recovery bundle.".into());
    }
    let mut bytes = Vec::new();
    // One sentinel byte beyond the ceiling detects oversized streaming input.
    file.take(MAX_RECOVERY_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    let mut bundle = decode_bundle(&bytes)?;
    drop(bytes);
    reject_private_destinations(
        bundle.downstream.path.as_deref(),
        bundle.upstream.path.as_deref(),
        path.value(),
    )?;
    let downstream_path = bundle
        .downstream
        .path
        .take()
        .map(|path| project_path::admit(path, application.budget()))
        .transpose()?;
    let upstream_path = bundle
        .upstream
        .path
        .take()
        .map(|path| project_path::admit(path, application.budget()))
        .transpose()?;
    let upstream = bundle
        .upstream
        .document
        .as_ref()
        .map(|bytes| {
            let project =
                merlin::common::Project::from_bytes(bytes).map_err(|error| error.to_string())?;
            PreparedSession::from_project(&project, application.budget())
        })
        .transpose()?;
    let downstream = PreparedProject::prepare_recovery(
        bundle
            .downstream
            .document
            .as_deref()
            .ok_or("Recovery is missing its downstream editor state.")?,
        downstream_path,
        &application,
        expected.0,
        accepted,
        bundle.dark,
    )?;
    let paths = application.prepare_input_paths(bundle.original_paths)?;
    Ok(Product::Prepared {
        expected,
        downstream: Box::new(downstream),
        upstream,
        upstream_path,
        paths,
        expected_paths,
        dirty: (bundle.downstream.dirty, bundle.upstream.dirty),
        selected: direction(bundle.selected),
    })
}

#[cfg(test)]
#[path = "recovery_tests.rs"]
mod tests;
