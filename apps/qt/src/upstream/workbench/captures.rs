/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Independent source inspection over public native capture owners, never association.
mod constants;
mod inspection;
#[cfg(test)]
mod tests;
use constants::CAPTURE_SOURCE_SHARED_ROOTS;
pub(crate) use constants::MAX_CAPTURE_FAMILY_CHARACTERS;

use super::{Model, Pending, MAX_CONFIGURATION_BYTES};
use crate::{
    downstream::inputs::retention::{admit_model, json_children},
    platform::input_file,
    shell::{Direction, Retained, RetainedResources, RetentionBudget},
};
use serde_json::{json, Value};
use std::{mem::size_of, sync::Arc};

/// Explicit existing capture family; no scientific mode is inferred from a filename.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Family {
    Ucd,
    Rxmer,
    PreEq,
}

pub(super) type SharedSource = Arc<Retained<CaptureSource>>;

/// Exact delivery and decoded binary evidence, separate from executable inputs.
pub(super) struct CaptureSource {
    family: Family,
    delivery: Box<[u8]>,
    // Retain originals independently even when the outer delivery is source JSON.
    original: Box<[u8]>,
}

/// One admitted page; source aliases share its existing original-data reservation.
pub(super) struct CaptureDisplay {
    source: SharedSource,
    view: Value,
}

pub(super) enum Request {
    Load {
        family: Family,
        bytes: Retained<Box<[u8]>>,
    },
    Page {
        source: SharedSource,
        first: usize,
    },
}

impl Family {
    /// Validates the exact UI destination before file acquisition or worker dispatch.
    /// # Arguments
    /// * `tag` - One compiled family tag, never arbitrary scientific configuration.
    /// # Returns
    /// The existing typed destination.
    /// # Errors
    /// Unknown tags reject without changing previous input or accepted sessions.
    pub(crate) fn parse(tag: &str) -> Result<Self, String> {
        match tag {
            "ucd" => Ok(Self::Ucd),
            "rxmer" => Ok(Self::Rxmer),
            "pre_eq" => Ok(Self::PreEq),
            _ => Err("Choose a supported upstream capture family.".into()),
        }
    }

    /// Returns the original compiled destination identity, without allocating.
    /// # Returns
    /// The stable family tag used only for presentation routing.
    pub(super) fn tag(self) -> &'static str {
        match self {
            Self::Ucd => "ucd",
            Self::Rxmer => "rxmer",
            Self::PreEq => "pre_eq",
        }
    }

    /// Names a real native picker independently of its selected file content.
    /// # Returns
    /// A fixed title; scientific format admission always follows in Rust.
    pub(crate) fn picker_title(self) -> &'static str {
        match self {
            Self::Ucd => "Load Upstream UCD Capture",
            Self::Rxmer => "Load Upstream RxMER Capture",
            Self::PreEq => "Load Upstream Pre-Eq Capture",
        }
    }
}

impl Request {
    /// Runs bounded capture decoding and page construction on the existing worker.
    /// # Arguments
    /// * `budget` - Canonical counter shared by queued, accepted and previous data.
    /// # Returns
    /// An admitted page preserving exact source evidence independently of sessions.
    /// # Errors
    /// Public input/page errors or actual candidate retention rejection. Existing
    /// accepted displays and scientific configuration remain untouched.
    pub(super) fn prepare(
        self,
        budget: &RetentionBudget,
    ) -> Result<Retained<CaptureDisplay>, String> {
        let (source, view) = match self {
            Self::Load { family, bytes } => {
                let inspected = inspection::inspect(family, bytes.value(), 0)?;
                let payload_bytes = size_of::<Retained<CaptureSource>>()
                    .checked_add(bytes.value().len())
                    .and_then(|size| size.checked_add(inspected.original.len()))
                    .ok_or("Upstream capture source size overflowed.")?;
                let reservation = budget
                    .reserve_resources(RetainedResources {
                        payload_bytes,
                        shared_roots: CAPTURE_SOURCE_SHARED_ROOTS,
                        ..Default::default()
                    })
                    .map_err(|error| format!("{error:?}"))?;
                let source = Arc::new(reservation.retain(CaptureSource {
                    family,
                    delivery: bytes.value().to_vec().into_boxed_slice(),
                    original: inspected.original,
                }));
                (source, inspected.view)
            }
            Self::Page { source, first } => {
                let inspected =
                    inspection::inspect(source.value().family, &source.value().delivery, first)?;
                if inspected.original.as_ref() != source.value().original.as_ref() {
                    return Err("Upstream capture source changed during inspection.".into());
                }
                (source, inspected.view)
            }
        };
        let mut resources = json_children(&view)?;
        resources.payload_bytes = resources
            .payload_bytes
            .checked_add(size_of::<CaptureDisplay>())
            .ok_or("Upstream capture view size overflowed.")?;
        admit_model(CaptureDisplay { source, view }, resources, budget)
    }
}

impl Model {
    /// Loads an actual local binary/source-JSON capture without applying science.
    /// # Arguments
    /// * `family` - Explicit typed inspection destination.
    /// * `uri` - Native chooser result; None preserves all state on Cancel.
    /// # Returns
    /// True when finite off-GUI admission is queued; false on Cancel.
    /// # Errors
    /// Busy, file/path/input bound or worker admission failure leaves prior state.
    pub(crate) fn load_capture_uri(
        &mut self,
        family: Family,
        uri: Option<&str>,
    ) -> Result<bool, String> {
        let Some(uri) = uri else {
            return Ok(false);
        };
        self.ensure_configuration_idle()?;
        let (_, bytes) = input_file::read_local(
            &self.application,
            Direction::Upstream,
            uri,
            MAX_CONFIGURATION_BYTES,
        )?;
        let payload_bytes = size_of::<Retained<Box<[u8]>>>()
            .checked_add(bytes.len())
            .ok_or("Upstream capture delivery size overflowed.")?;
        let reservation = self
            .application
            .budget()
            .reserve_resources(RetainedResources {
                payload_bytes,
                ..Default::default()
            })
            .map_err(|error| format!("{error:?}"))?;
        let request = Request::Load {
            family,
            bytes: reservation.retain(bytes.into_boxed_slice()),
        };
        self.queue_capture(request)?;
        Ok(true)
    }

    /// Requests another bounded page of the same immutable source family.
    /// # Arguments
    /// * `family` - Original displayed destination.
    /// * `first` - Original record index; source validation stays in the worker.
    /// # Returns
    /// Unit after staging; failed ranges never replace previous displayed values.
    /// # Errors
    /// Busy, missing/foreign source or worker admission rejects without mutation.
    pub(crate) fn capture_page(&mut self, family: Family, first: usize) -> Result<(), String> {
        self.ensure_configuration_idle()?;
        let source = self
            .capture
            .as_ref()
            .ok_or("Load an upstream capture first.")?;
        if source.value().source.value().family != family {
            return Err("This capture belongs to another input family.".into());
        }
        self.queue_capture(Request::Page {
            source: Arc::clone(&source.value().source),
            first,
        })
    }

    /// Reads a bounded page without exposing original capture arrays to QML.
    /// # Returns
    /// The original family and display strings, or null before successful admission.
    pub(super) fn capture_state(&self) -> Value {
        let Some(display) = self.capture.as_ref().map(Retained::value) else {
            return Value::Null;
        };
        let mut view = display.view.clone();
        view["error"] = self.error.message().into();
        json!({"family":display.source.value().family.tag(),"view":view})
    }

    fn queue_capture(&mut self, request: Request) -> Result<(), String> {
        self.pending = Some(Pending::capture(
            request,
            self.application.budget().clone(),
            self.application.snapshot(Direction::Upstream).key,
        )?);
        self.preparation_cancelled = false;
        self.error = Default::default();
        Ok(())
    }
}
