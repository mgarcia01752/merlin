/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Finite performance preparation, followed by actual normalized Entry admission.
use super::{Entry, FecReport, Point, ResultData, SourceProfileFrame, Value};
use crate::shell::{RetainedResources, RetentionBudget, RetentionError};
use merlin::downstream::{FullLoopProfilePerformance, ProfileCarrierRole};
use std::mem::size_of;

// This temporary construction owner is never adopted by Context or a worker.
// Public performance points use the validated source grid (MAX_LAB_CARRIERS).
// fec::snapshot uses actual bounded retained_words (MAX_RETAINED_WORDS), the
// MAX_CHANNEL_DECODER_ITERATIONS + 1 histogram, MAX_VERIFICATION_WORD_BARS and
// fixed scalar/enum data. Scientific error text is the admitted result's existing
// bounded engine diagnostic; a setup rejection moves its actual supplied String.
// These finite builders are separate from retained owner admission, not an
// aggregate temporary-byte or preparation-peak guarantee.
pub(super) struct PreparedEntry {
    id: u8,
    channel: u8,
    status: String,
    fec: Option<FecReport>,
    ber: Option<f64>,
    pending: u64,
    points: Vec<Point>,
    error: Option<String>,
    fec_details: Option<Value>,
}

impl PreparedEntry {
    /// Prepares the existing unstarted state without altering either history registry.
    ///
    /// # Arguments
    /// * `profile` - Exact captured identity; the caller bounds the catalog by MAX_PROFILES.
    ///
    /// # Returns
    /// Temporary finite candidate awaiting host admission.
    pub(super) fn not_started(profile: &SourceProfileFrame) -> Self {
        Self {
            id: profile.profile_id,
            channel: profile.key.channel_id,
            status: "not_started".into(),
            fec: None,
            ber: None,
            pending: 0,
            points: vec![],
            error: None,
            fec_details: None,
        }
    }

    /// Preserves the existing public scientific projection and FEC JSON builders.
    ///
    /// # Arguments
    /// * `performance` - Existing finite public query candidate, never a retained query alias.
    /// * `result` - Independently charged immutable source kept alive through preparation.
    ///
    /// # Returns
    /// Temporary copied candidate with unchanged coordinates, nulls, status and error text.
    pub(super) fn from_result(
        performance: FullLoopProfilePerformance,
        result: &ResultData,
    ) -> Self {
        let points = performance
            .points
            .iter()
            .map(|point| Point {
                hz: point.frequency_hz.value(),
                mer: point.measurement.mer().map(|mer| mer.value()),
                samples: point.measurement.samples().value(),
                plc: point.role == ProfileCarrierRole::Plc,
                ideal: point.ideal_shannon_mer_db.map(|mer| mer.value()),
            })
            .collect();
        Self {
            fec_details: Some(super::super::fec::snapshot(result)),
            id: performance.profile_id.value(),
            channel: performance.key.channel_id,
            status: serde_json::to_value(performance.status)
                .unwrap()
                .as_str()
                .unwrap()
                .into(),
            fec: Some(performance.fec),
            ber: performance.ber.map(|ber| ber.value()),
            pending: performance.unrecovered_generated_payloads,
            points,
            error: performance.error,
        }
    }

    /// Prepares the existing failed state without synthetic counters or text truncation.
    ///
    /// # Arguments
    /// * `id`, `channel` - Original captured identifiers.
    /// * `error` - Actual already produced setup/query diagnostic, transferred in full.
    ///
    /// # Returns
    /// Temporary finite failed candidate awaiting host admission.
    pub(super) fn failed(id: u8, channel: u8, error: String) -> Self {
        Self {
            fec_details: None,
            id,
            channel,
            status: "failed".into(),
            fec: None,
            ber: None,
            pending: 0,
            points: vec![],
            error: Some(error),
        }
    }

    /// Normalizes and admits actual final target storage before Context adoption.
    ///
    /// # Arguments
    /// * `budget` - Exact canonical host also covering source, old and candidate owners.
    ///
    /// # Returns
    /// One immutable-by-convention stored Entry with its last field owning the charge.
    /// No Vec/String clone or future collect capacity is used as an admission estimate.
    ///
    /// # Errors
    /// Checked payload/entry arithmetic or host rejection; previous entries stay unchanged.
    pub(super) fn admit(mut self, budget: &RetentionBudget) -> Result<Entry, RetentionError> {
        self.status = string(self.status);
        self.error = self.error.map(string);
        self.points = self.points.into_boxed_slice().into_vec();
        self.fec_details = self.fec_details.map(normalize_json);
        let reservation = budget.reserve_resources(self.resources()?)?;
        let Self {
            id,
            channel,
            status,
            fec,
            ber,
            pending,
            points,
            error,
            fec_details,
        } = self;
        Ok(Entry {
            id,
            channel,
            status,
            fec,
            ber,
            pending,
            points,
            error,
            fec_details,
            _retained: reservation.retain(()),
        })
    }

    /// Measures this actual candidate using the final Entry's full fixed layout once.
    ///
    /// # Returns
    /// Actual capacities and JSON map population, with zero shared roots: no Arc is stored.
    /// FecReport is an audited fixed MetricScope plus five-u64 FecCounters at its owner;
    /// its private representation is not reconstructed through getters or serialized sizes.
    ///
    /// # Errors
    /// Checked payload multiplication, byte addition or nested ordered-entry overflow.
    pub(super) fn resources(&self) -> Result<RetainedResources, RetentionError> {
        let Self {
            id: _,
            channel: _,
            status,
            fec: _,
            ber: _,
            pending: _,
            points,
            error,
            fec_details,
        } = self;
        for point in points {
            let Point {
                hz,
                mer,
                ideal,
                samples,
                plc,
            } = point;
            let _: (u32, Option<f64>, Option<f64>, u64, bool) = (*hz, *mer, *ideal, *samples, *plc);
        }
        let bytes = add(size_of::<Entry>(), status.capacity())?;
        let bytes = add(bytes, product(points.capacity(), size_of::<Point>())?)?;
        let bytes = add(bytes, error.as_ref().map_or(0, String::capacity))?;
        let mut resources = RetainedResources {
            payload_bytes: bytes,
            ordered_entries: 0,
            shared_roots: 0,
            ..Default::default()
        };
        if let Some(value) = fec_details {
            append(&mut resources, json_children(value)?)?;
        }
        Ok(resources)
    }
}

fn string(value: String) -> String {
    value.into_boxed_str().into_string()
}

fn normalize_json(value: Value) -> Value {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) => value,
        Value::String(value) => Value::String(string(value)),
        Value::Array(values) => Value::Array(
            values
                .into_iter()
                .map(normalize_json)
                .collect::<Box<[_]>>()
                .into_vec(),
        ),
        Value::Object(values) => Value::Object(
            values
                .into_iter()
                .map(|(key, value)| (string(key), normalize_json(value)))
                .collect(),
        ),
    }
}

fn add(left: usize, right: usize) -> Result<usize, RetentionError> {
    left.checked_add(right)
        .ok_or(RetentionError::ArithmeticOverflow)
}
fn product(count: usize, fixed_bytes: usize) -> Result<usize, RetentionError> {
    count
        .checked_mul(fixed_bytes)
        .ok_or(RetentionError::ArithmeticOverflow)
}
fn append(
    total: &mut RetainedResources,
    children: RetainedResources,
) -> Result<(), RetentionError> {
    total.payload_bytes = add(total.payload_bytes, children.payload_bytes)?;
    total.ordered_entries = add(total.ordered_entries, children.ordered_entries)?;
    Ok(())
}

fn json_children(value: &Value) -> Result<RetainedResources, RetentionError> {
    crate::downstream::inputs::retention::json_children(value)
        .map_err(|_| RetentionError::ArithmeticOverflow)
}

#[cfg(test)]
#[path = "retention_tests.rs"]
mod tests;
