/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Export worker snapshots; encoding and filesystem publication belong to pnm-input.
use super::measurement_input::bridge;
use crate::downstream::common::constants::{
    MAX_DESIGNER_EXPORT_SELECTION_BYTES, MAX_DESIGNER_RECIPE_DOCUMENT_BYTES,
};
use crate::downstream::inputs::measurement_input::designer::Request;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use merlin::downstream::inputs::rxmer_export::{
    self, ExportDestination, ExportFormat, RangePolicy,
};
use merlin::downstream::RxMerFrame;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    pin::Pin,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc, Arc,
    },
};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Selection {
    first: usize,
    last: usize,
    clamp: bool,
    collection: bool,
}
pub(super) struct Prepared {
    frames: Vec<RxMerFrame>,
    policy: RangePolicy,
    context: Value,
    summary: Value,
    json_summary: Value,
    pnm_error: Option<String>,
    diagnostic: Value,
    collection: bool,
}
pub(super) enum Outcome {
    Ready(Prepared),
    Saved(Value, Prepared),
}
pub(super) enum State {
    Working {
        receiver: mpsc::Receiver<Result<Outcome, String>>,
        cancel: Arc<AtomicBool>,
        completed: Arc<AtomicUsize>,
    },
    Ready(Prepared),
}
fn reply(value: Value) -> QString {
    QString::from(value.to_string())
}
fn error(message: impl ToString) -> QString {
    reply(json!({"running":false,"can_export":false,"error":message.to_string()}))
}
fn spawn(
    operation: impl FnOnce(Arc<AtomicBool>, Arc<AtomicUsize>) -> Result<Outcome, String>
        + Send
        + 'static,
) -> Result<State, String> {
    let cancel = Arc::new(AtomicBool::new(false));
    let completed = Arc::new(AtomicUsize::new(0));
    let (tx, receiver) = mpsc::channel();
    let worker_cancel = cancel.clone();
    let worker_completed = completed.clone();
    std::thread::Builder::new()
        .name("rxmer-pnm-export".into())
        .spawn(move || {
            let _ = tx.send(operation(worker_cancel, worker_completed));
        })
        .map_err(|error| error.to_string())?;
    Ok(State::Working {
        receiver,
        cancel,
        completed,
    })
}
impl bridge::MeasurementInputs {
    /// Starts bounded full-grid export preflight without changing the Designer draft.
    /// # Arguments
    /// * `document` - Recipe/source snapshot, at most24KiB.
    /// * `selection` - Inclusive capture scope and explicit range policy, at most1KiB.
    /// # Returns
    /// Worker admission; poll supplies authoritative range/metadata diagnostics.
    pub fn designer_export_prepare(
        mut self: Pin<&mut Self>,
        document: &QString,
        selection: &QString,
    ) -> QString {
        let result: Result<(), String> = (|| {
            if matches!(self.rust().designer_export, Some(State::Working { .. })) {
                return Err("Wait for the previous export operation.".into());
            }
            let text = document.to_string();
            let selected = selection.to_string();
            if text.len() > MAX_DESIGNER_RECIPE_DOCUMENT_BYTES
                || selected.len() > MAX_DESIGNER_EXPORT_SELECTION_BYTES
            {
                return Err("Export request exceeds the input limit.".into());
            }
            let request: Request =
                serde_json::from_str(&text).map_err(|error| error.to_string())?;
            let selection: Selection =
                serde_json::from_str(&selected).map_err(|error| error.to_string())?;
            let session = self.rust().designer_copy()?;
            let task = spawn(move |cancel, completed| {
                let frames = session.value().export_frames(
                    &request,
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
                let json_summary =
                    rxmer_export::preflight_format(&frames, policy, ExportFormat::Json)
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
                Ok(Outcome::Ready(Prepared {
                    frames,
                    policy,
                    context,
                    summary,
                    json_summary: json!(json_summary),
                    pnm_error,
                    diagnostic,
                    collection: selection.collection,
                }))
            })?;
            self.as_mut().rust_mut().designer_export = Some(task);
            Ok(())
        })();
        match result {
            Ok(()) => reply(json!({"running":true})),
            Err(message) => error(message),
        }
    }
    /// Polls export validation or save without blocking the Qt thread.
    /// # Returns
    /// Actual completed-capture count, authoritative admission, or save outcome.
    pub fn designer_export_poll(mut self: Pin<&mut Self>) -> QString {
        let Some(state) = self.rust().designer_export.as_ref() else {
            return reply(json!({"running":false}));
        };
        let result = match state {
            State::Ready(prepared) => {
                return reply(
                    json!({"running":false,"can_export":prepared.pnm_error.is_none(),"can_json":true,"summary":prepared.summary,"json_summary":prepared.json_summary,"error":prepared.pnm_error,"diagnostic":prepared.diagnostic}),
                )
            }
            State::Working {
                receiver,
                cancel,
                completed,
            } => match receiver.try_recv() {
                Ok(result) => {
                    if cancel.load(Ordering::Relaxed) {
                        Err("PNM export cancelled.".into())
                    } else {
                        result
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {
                    return reply(
                        json!({"running":true,"completed":completed.load(Ordering::Relaxed)}),
                    )
                }
                Err(_) => Err("Export worker ended unexpectedly.".into()),
            },
        };
        self.as_mut().rust_mut().designer_export.take();
        match result {
            Ok(Outcome::Ready(prepared)) => {
                let value = reply(
                    json!({"running":false,"can_export":prepared.pnm_error.is_none(),"can_json":true,"summary":prepared.summary,"json_summary":prepared.json_summary,"error":prepared.pnm_error,"diagnostic":prepared.diagnostic}),
                );
                self.as_mut().rust_mut().designer_export = Some(State::Ready(prepared));
                value
            }
            Ok(Outcome::Saved(mut value, prepared)) => {
                value["can_json"] = json!(true);
                value["can_export"] = json!(prepared.pnm_error.is_none());
                value["json_summary"] = prepared.json_summary.clone();
                self.as_mut().rust_mut().designer_export = Some(State::Ready(prepared));
                reply(value)
            }
            Err(message) => error(message),
        }
    }
    /// Opens a native destination chooser and saves the already-checked immutable snapshot.
    /// # Arguments
    /// * `format` - Explicit pnm or json representation; no implicit conversion.
    /// # Returns
    /// Worker state; the crate revalidates every frame before writing. No Apply/run.
    pub fn designer_export_save(mut self: Pin<&mut Self>, format: &QString) -> QString {
        let format = match format.to_string().as_str() {
            "pnm" => ExportFormat::Pnm,
            "json" => ExportFormat::Json,
            _ => return error("Choose PNM binary or JSON."),
        };
        if !matches!(self.rust().designer_export, Some(State::Ready(_))) {
            return error("Check the current export selection first.");
        }
        let Some(State::Ready(prepared)) = self.as_mut().rust_mut().designer_export.take() else {
            return error("No checked export.");
        };
        if matches!(format, ExportFormat::Pnm) && prepared.pnm_error.is_some() {
            let message = prepared.pnm_error.clone().unwrap();
            self.as_mut().rust_mut().designer_export = Some(State::Ready(prepared));
            return error(message);
        }
        let task = spawn(move |cancel, completed| {
            let Some(uri) = crate::platform::native_file_picker::export_rxmer(
                prepared.collection,
                matches!(format, ExportFormat::Json),
            )?
            else {
                return Ok(Outcome::Saved(
                    json!({"running":false,"cancelled":true}),
                    prepared,
                ));
            };
            let mut path = url::Url::parse(&uri)
                .map_err(|error| error.to_string())?
                .to_file_path()
                .map_err(|_| "Choose a local export destination.")?;
            if prepared.collection {
                let epoch = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|error| error.to_string())?
                    .as_nanos();
                path = path.join(format!("merlin-rxmer-export-{epoch}"));
            } else if path.extension().is_none() {
                path.set_extension(format.extension());
            }
            let destination = if prepared.collection {
                ExportDestination::Collection(&path)
            } else {
                ExportDestination::Single(&path)
            };
            let result = rxmer_export::save_with_format(
                destination,
                &prepared.frames,
                prepared.policy,
                &prepared.context,
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
            Ok(Outcome::Saved(result, prepared))
        });
        match task {
            Ok(task) => {
                self.as_mut().rust_mut().designer_export = Some(task);
                reply(json!({"running":true}))
            }
            Err(message) => error(message),
        }
    }
    /// Cancels a pending worker or discards only the immutable export snapshot.
    /// # Returns
    /// Unit; accepted measurements, source files and current recipe stay intact.
    pub fn designer_export_cancel(mut self: Pin<&mut Self>) {
        match self.rust().designer_export.as_ref() {
            Some(State::Working { cancel, .. }) => cancel.store(true, Ordering::Relaxed),
            _ => {
                self.as_mut().rust_mut().designer_export.take();
            }
        }
    }
}
