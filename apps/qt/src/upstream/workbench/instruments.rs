/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Passive bounded instrument DTOs; public upstream owners retain all science.
mod common;
mod fec;
#[cfg(test)]
mod fec_tests;
mod qam;
mod request;
mod response;
mod soft;
mod spectrum;
#[cfg(test)]
mod tests;

use crate::{
    downstream::inputs::retention::{admit_model, json_children},
    shell::{Retained, RetentionBudget},
};
use fec::FecRequest;
use merlin::upstream::{BatchReport, QamConstellationRequest, QamPairRequest, Tap};
pub(super) use request::Request;
use response::{Metric as ResponseMetric, ResponseRequest, Source as ResponseSource};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use std::mem::size_of;

/// One independent final instrument presentation; no report or source is cloned.
pub(super) struct Display {
    pub(super) snapshot: Value,
}

/// Explicit public QAM operation, preserving incompatible raw/reference planes.
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum QamRequest {
    /// Original actual stage values, optionally filtered by original loading.
    Raw {
        tap: Tap,
        loading: Option<u8>,
        maximum_points: usize,
    },
    /// Existing backend role-gain normalization and canonical labelled references.
    Constellation(QamConstellationRequest),
    /// Existing exact original-coordinate pairing, without channel correction.
    Pairing(QamPairRequest),
}

/// Original retained sample tap and frequency-bin display window.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SpectrumRequest {
    /// Existing actual time-domain tap; frequency-grid taps reject explicitly.
    pub(super) tap: Tap,
    /// First original bin inside the public complete centered spectrum.
    pub(super) first: usize,
    /// Positive span, or zero for the remaining original bins.
    pub(super) count: usize,
}

impl Serialize for QamRequest {
    /// Serializes the exact flat application request echoed by the selection DTO.
    /// # Arguments
    /// * `serializer` - Caller-selected serde transport; no scientific values change.
    /// # Returns
    /// The mode and complete matching public request fields.
    /// # Errors
    /// Propagates the serializer's original transport failure.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        qam::selection(*self).serialize(serializer)
    }
}

/// Queries actual raw, normalized or exactly paired retained QAM observations.
///
/// # Arguments
/// * `report` - Immutable actual complete/partial report; no replay occurs.
/// * `request` - One existing public QAM operation and its independent bounds.
/// * `budget` - Canonical counter shared with old, queued and source owners.
///
/// # Returns
/// One admitted display with actual tap/loading choices, unchanged bounded point
/// details and screen-only coordinates. Unsupported/unretained planes return an
/// explicit admitted unavailable envelope, without fabricated references or data.
///
/// # Errors
/// Invalid public requests, nonfinite display data, checked metadata or retention
/// rejection. No report/source mutation or scientific state advance occurs.
pub(super) fn qam(
    report: &BatchReport,
    request: QamRequest,
    budget: &RetentionBudget,
) -> Result<Retained<Display>, String> {
    admit(qam::project(report, request)?, budget)
}

/// Queries the backend PSD over actual retained samples and projects a viewport.
///
/// # Arguments
/// * `report` - Immutable report supplying the actual configured FFT numerology.
/// * `request` - Retained time tap and original complete-spectrum bin viewport.
/// * `budget` - Canonical counter covering simultaneous old/new presentation.
///
/// # Returns
/// An admitted SpectrumPlot-compatible linear normalized power/Hz viewport. All
/// FFT/PSD/window/frequency values come from the public operation; no logarithm,
/// electrical calibration or downstream PLC overlay is introduced in the adapter.
/// Unsupported or insufficient history stays explicitly unavailable.
///
/// # Errors
/// Invalid request/extents, nonfinite/inconsistent supplied data, checked metadata
/// or actual retention admission rejection; accepted scientific owners survive.
pub(super) fn spectrum(
    report: &BatchReport,
    request: SpectrumRequest,
    budget: &RetentionBudget,
) -> Result<Retained<Display>, String> {
    admit(spectrum::project(report, request)?, budget)
}

fn admit(mut snapshot: Value, budget: &RetentionBudget) -> Result<Retained<Display>, String> {
    snapshot["summary"] = snapshot["metadata"]["reference_plane"]
        .as_str()
        .unwrap_or("Original retained upstream observations; missing history is unavailable.")
        .into();
    if snapshot.get("metadata_rows").is_none() {
        snapshot["metadata_rows"] = snapshot["metadata"]
            .as_object()
            .map(|metadata| {
                metadata
                    .iter()
                    .map(|(name, value)| {
                        serde_json::json!({"label":name.replace('_', " "),
                    "value":value.as_str().map(str::to_owned).unwrap_or_else(||value.to_string())})
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
            .into();
    }
    let mut resources = json_children(&snapshot)?;
    resources.payload_bytes = resources
        .payload_bytes
        .checked_add(size_of::<Display>())
        .ok_or("Upstream instrument retained size overflowed.")?;
    admit_model(Display { snapshot }, resources, budget)
}
