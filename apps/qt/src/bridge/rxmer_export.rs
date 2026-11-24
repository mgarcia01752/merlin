/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Export worker snapshots; encoding and filesystem publication belong to pnm-input.
mod input;
mod lifecycle;
mod retention;
use super::measurement_input::bridge;
use crate::downstream::common::constants::{
    MAX_DESIGNER_EXPORT_SELECTION_BYTES, MAX_DESIGNER_RECIPE_DOCUMENT_BYTES,
};
use crate::downstream::inputs::measurement_input::designer::Request;
use crate::shell::{Retained, RetentionBudget};
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
type SharedPrepared = Arc<Retained<Prepared>>;

impl Prepared {
    fn admit(self, budget: &RetentionBudget) -> Result<SharedPrepared, String> {
        let mut resources = retention::resources(&self)?;
        resources.shared_roots = resources
            .shared_roots
            .checked_add(1)
            .ok_or("Export product retained-root count overflow.")?;
        let retained = crate::downstream::inputs::retention::admit_model(self, resources, budget)?;
        Ok(Arc::new(retained))
    }
}

pub(super) enum Outcome {
    Ready(SharedPrepared),
    Saved(Value, SharedPrepared),
}
pub(super) enum State {
    Working {
        receiver: mpsc::Receiver<Result<Outcome, String>>,
        cancel: Arc<AtomicBool>,
        completed: Arc<AtomicUsize>,
        previous: Option<SharedPrepared>,
    },
    Ready(SharedPrepared),
}
fn reply(value: Value) -> QString {
    QString::from(value.to_string())
}
fn error(message: impl ToString) -> QString {
    reply(json!({"running":false,"can_export":false,"error":message.to_string()}))
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
        let result = lifecycle::prepare(
            self.as_mut().rust_mut().get_mut(),
            &document.to_string(),
            &selection.to_string(),
            lifecycle::launch,
        );
        match result {
            Ok(()) => reply(json!({"running":true})),
            Err(message) => error(message),
        }
    }
    /// Polls export validation or save without blocking the Qt thread.
    /// # Returns
    /// Actual completed-capture count, authoritative admission, or save outcome.
    pub fn designer_export_poll(mut self: Pin<&mut Self>) -> QString {
        reply(lifecycle::poll(
            &mut self.as_mut().rust_mut().designer_export,
        ))
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
        match lifecycle::save(
            self.as_mut().rust_mut().get_mut(),
            format,
            lifecycle::launch,
        ) {
            Ok(()) => reply(json!({"running":true})),
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

#[cfg(test)]
#[path = "rxmer_export/admission_tests.rs"]
mod admission_tests;
