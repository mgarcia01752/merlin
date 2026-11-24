/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Existing public-API export operations with explicit previous-Ready ownership.
use super::*;
use crate::bridge::measurement_input::MeasurementInputsRust;
use crate::downstream::common::constants::EXPORT_RESULT_QUEUE_CAPACITY;
use std::io;

type Task = Box<dyn FnOnce() + Send>;

/// Starts the existing detached native export worker.
/// # Arguments
/// * `task` - One finite operation carrying its admitted immutable input owners.
/// # Returns
/// Unit after successful OS thread creation; no GUI thread work is performed here.
/// # Errors
/// Preserves the OS spawn diagnostic without consuming any GUI Ready snapshot.
pub(super) fn launch(task: Task) -> io::Result<()> {
    std::thread::Builder::new()
        .name("rxmer-pnm-export".into())
        .spawn(task)
        .map(drop)
}

fn spawn(
    previous: Option<SharedPrepared>,
    budget: RetentionBudget,
    operation: impl FnOnce(Arc<AtomicBool>, Arc<AtomicUsize>) -> Result<Outcome, String>
        + Send
        + 'static,
    start: impl FnOnce(Task) -> io::Result<()>,
) -> Result<State, String> {
    let cancel = Arc::new(AtomicBool::new(false));
    let completed = Arc::new(AtomicUsize::new(0));
    let (tx, receiver) = mpsc::sync_channel(EXPORT_RESULT_QUEUE_CAPACITY);
    let worker_cancel = Arc::clone(&cancel);
    let worker_completed = Arc::clone(&completed);
    start(Box::new(move || {
        let result = operation(worker_cancel, worker_completed)
            .map_err(|message| WorkerFailure::admit(message.into(), &budget));
        let _ = tx.send(result);
    }))
    .map_err(|error| error.to_string())?;
    Ok(State::Working {
        receiver,
        cancel,
        completed,
        previous,
    })
}

fn previous(state: &Option<State>) -> Option<SharedPrepared> {
    match state {
        Some(State::Ready(prepared)) => Some(Arc::clone(prepared)),
        _ => None,
    }
}

/// Prepares the existing export operation before replacing its accepted Ready owner.
/// # Arguments
/// * `owner` - Exact GUI adapter, retaining old Session/export guards on rejection.
/// * `document`, `selected` - Existing bounded UTF-8 recipe and capture selection JSON.
/// * `start` - Native launcher or deterministic failure seam, matching shell workers.
/// # Returns
/// Unit after successful worker creation and one Working-state publication.
/// # Errors
/// Busy, input/Session admission or spawn failure leaves the previous state unchanged.
pub(super) fn prepare(
    owner: &mut MeasurementInputsRust,
    document: &str,
    selected: &str,
    start: impl FnOnce(Task) -> io::Result<()>,
) -> Result<(), String> {
    if matches!(owner.designer_export, Some(State::Working { .. })) {
        return Err("Wait for the previous export operation.".into());
    }
    if document.len() > MAX_DESIGNER_RECIPE_DOCUMENT_BYTES
        || selected.len() > MAX_DESIGNER_EXPORT_SELECTION_BYTES
    {
        return Err("Export request exceeds the input limit.".into());
    }
    let request: Request = serde_json::from_str(document).map_err(|error| error.to_string())?;
    let selection: Selection = serde_json::from_str(selected).map_err(|error| error.to_string())?;
    let session = owner.designer_copy()?;
    let budget = owner.context.application.budget().clone();
    let input = input::Input { request, selection }.admit(&budget)?;
    let task = spawn(
        previous(&owner.designer_export),
        budget.clone(),
        move |cancel, completed| {
            let input::Input { request, selection } = input.value();
            let frames = session.value().export_frames(
                request,
                selection.first,
                selection.last,
                &cancel,
                &completed,
            )?;
            if !selection.collection && frames.len() != 1 {
                return Err("Preview export requires exactly one capture.".into());
            }
            let policy = if selection.clamp {
                RangePolicy::Clamp
            } else {
                RangePolicy::Reject
            };
            let json_summary = rxmer_export::preflight_format(&frames, policy, ExportFormat::Json)
                .map_err(|error| error.to_string())?;
            let pnm_check = rxmer_export::preflight(&frames, policy).map_err(|mut error| {
                if let Some(index) = error.capture {
                    error.capture = Some(index + selection.first - 1);
                }
                error
            });
            let diagnostic = match &pnm_check {
                Err(error) if error.code == "out_of_range" => {
                    crate::downstream::help::static_diagnostic("RxmerPnmExportRange")
                }
                _ => Value::Null,
            };
            let pnm_error = pnm_check.as_ref().err().map(|error| error.to_string());
            let summary = pnm_check.map_or(Value::Null, |summary| json!(summary));
            let context = json!({"designer":request,"first_capture":selection.first,"last_capture":selection.last,"baseline_sha256":session.value().hashes,"profile_sha256":session.value().profile.source.sha256});
            let prepared = Prepared {
                frames,
                policy,
                context,
                summary,
                json_summary: json!(json_summary),
                pnm_error,
                diagnostic,
                collection: selection.collection,
            };
            Ok(Outcome::Ready(prepared.admit(&budget)?))
        },
        start,
    )?;
    owner.designer_export = Some(task);
    Ok(())
}

fn metadata(prepared: &SharedPrepared) -> Value {
    let prepared = prepared.value();
    json!({"running":false,"can_export":prepared.pnm_error.is_none(),"can_json":true,"summary":prepared.summary,"json_summary":prepared.json_summary,"error":prepared.pnm_error,"diagnostic":prepared.diagnostic})
}

/// Polls one terminal product, restoring the old Ready snapshot after failed work.
/// # Arguments
/// * `state` - Exact adapter state; old/new/queued products retain their existing guards.
/// # Returns
/// Existing bounded progress/result/error JSON. Rejection/cancellation/disconnection
/// restores a prior Ready product only after the worker's terminal state is known.
pub(super) fn poll(state: &mut Option<State>) -> Value {
    let Some(current) = state.as_ref() else {
        return json!({"running":false});
    };
    let result = match current {
        State::Ready(prepared) => return metadata(prepared),
        State::Working {
            receiver,
            cancel,
            completed,
            ..
        } => match receiver.try_recv() {
            Ok(result) => {
                if cancel.load(Ordering::Relaxed) {
                    Err(WorkerFailure::Static("PNM export cancelled."))
                } else {
                    result
                }
            }
            Err(mpsc::TryRecvError::Empty) => {
                return json!({"running":true,"completed":completed.load(Ordering::Relaxed)})
            }
            Err(_) => Err(WorkerFailure::Static("Export worker ended unexpectedly.")),
        },
    };
    let previous = match state.take() {
        Some(State::Working { previous, .. }) => previous,
        _ => None,
    };
    match result {
        Ok(Outcome::Ready(prepared)) => {
            let value = metadata(&prepared);
            *state = Some(State::Ready(prepared));
            value
        }
        Ok(Outcome::Saved(saved, prepared)) => {
            let mut value = saved.reply();
            value["can_json"] = json!(true);
            value["can_export"] = json!(prepared.value().pnm_error.is_none());
            value["json_summary"] = prepared.value().json_summary.clone();
            *state = Some(State::Ready(prepared));
            value
        }
        Err(failure) => {
            *state = previous.map(State::Ready);
            json!({"running":false,"can_export":false,"error":failure.message()})
        }
    }
}

/// Starts the existing public Rust save operation while retaining accepted Ready data.
/// # Arguments
/// * `owner` - Exact adapter, retaining its previous snapshot until known completion.
/// * `format` - Existing explicit JSON or PNM binary choice.
/// * `start` - Native worker launcher or deterministic failure seam.
/// # Returns
/// Unit after worker creation; native picker and publication remain off the Qt thread.
/// # Errors
/// Missing Ready data, original PNM range rejection or spawn failure preserves it.
pub(super) fn save(
    owner: &mut MeasurementInputsRust,
    format: ExportFormat,
    start: impl FnOnce(Task) -> io::Result<()>,
) -> Result<(), String> {
    save_with_picker(
        owner,
        format,
        start,
        crate::platform::native_file_picker::export_rxmer,
    )
}

/// Starts the unchanged save operation with an explicit destination-picker seam.
/// # Arguments
/// * `owner` - Exact GUI state retaining its current Ready product.
/// * `format` - Existing public Rust JSON/PNM format choice.
/// * `start` - Native launcher or deterministic test launcher.
/// * `picker` - Existing native chooser, or bounded test URI/cancellation result.
/// # Returns
/// Unit after worker publication; selection and filesystem work remain off the GUI.
/// # Errors
/// Same Ready/range/spawn errors as save, preserving old state before publication.
pub(super) fn save_with_picker(
    owner: &mut MeasurementInputsRust,
    format: ExportFormat,
    start: impl FnOnce(Task) -> io::Result<()>,
    picker: impl FnOnce(bool, bool) -> Result<Option<String>, String> + Send + 'static,
) -> Result<(), String> {
    let Some(State::Ready(prepared)) = owner.designer_export.as_ref() else {
        return Err("Check the current export selection first.".into());
    };
    if matches!(format, ExportFormat::Pnm) {
        if let Some(message) = &prepared.value().pnm_error {
            return Err(message.clone());
        }
    }
    let prepared = Arc::clone(prepared);
    let budget = owner.context.application.budget().clone();
    let task = spawn(
        Some(Arc::clone(&prepared)),
        budget.clone(),
        move |cancel, completed| {
            let Some(uri) = picker(
                prepared.value().collection,
                matches!(format, ExportFormat::Json),
            )?
            else {
                return Ok(Outcome::Saved(
                    saved::Saved::admit(json!({"running":false,"cancelled":true}), &budget),
                    prepared,
                ));
            };
            let mut path = url::Url::parse(&uri)
                .map_err(|error| error.to_string())?
                .to_file_path()
                .map_err(|_| "Choose a local export destination.")?;
            if prepared.value().collection {
                let epoch = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|error| error.to_string())?
                    .as_nanos();
                path = path.join(format!("merlin-rxmer-export-{epoch}"));
            } else if path.extension().is_none() {
                path.set_extension(format.extension());
            }
            let destination = if prepared.value().collection {
                ExportDestination::Collection(&path)
            } else {
                ExportDestination::Single(&path)
            };
            let result = rxmer_export::save_with_format(
                destination,
                &prepared.value().frames,
                prepared.value().policy,
                &prepared.value().context,
                &cancel,
                &completed,
                format,
            );
            let result = match result {
                Ok(summary) => {
                    json!({"running":false,"saved":true,"path":path.to_string_lossy(),"summary":summary})
                }
                Err(error) => json!({"running":false,"error":error.to_string()}),
            };
            Ok(Outcome::Saved(
                saved::Saved::admit(result, &budget),
                prepared,
            ))
        },
        start,
    )?;
    owner.designer_export = Some(task);
    Ok(())
}
