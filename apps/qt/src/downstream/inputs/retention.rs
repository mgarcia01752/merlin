/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Shared passive actual-capacity leaves for the existing input owners.
mod model_children;
use crate::shell::{
    Retained, RetainedResources as Resources, RetentionBudget, RetentionReservation,
};
use merlin::{
    common::RetainedFootprint,
    downstream::{FrequencyGrid, RxMerFrame, ScenarioKey, SourceProfileFrame},
};
pub(crate) use model_children::{recipe_children, response_frame_children};
use serde_json::Value;
use std::mem::size_of;

fn overflow() -> String {
    "Input retained-data metadata overflow.".into()
}
fn product(count: usize, bytes: usize) -> Result<usize, String> {
    count.checked_mul(bytes).ok_or_else(overflow)
}

/// Admits one actual editor candidate for queue storage and later GUI adoption.
///
/// # Arguments
/// * `T`, `model` - Existing concrete editor payload after finite preparation.
/// * `resources` - Complete actual candidate metadata counting sizeof(T) once.
/// * `budget` - Canonical target counter shared with old and accepted owners.
///
/// # Returns
/// The model and live guard. Coverage uses the larger of its two actual layouts:
/// queued Retained<T>, or GUI T followed by Option<RetentionReservation>.
/// No new shared allocation or guessed growth multiplier is introduced.
///
/// # Errors
/// Checked layout/metadata overflow or any aggregate dimension rejection.
pub(crate) fn admit_model<T>(
    model: T,
    mut resources: Resources,
    budget: &RetentionBudget,
) -> Result<Retained<T>, String> {
    let gui_layout = size_of::<(T, Option<RetentionReservation>)>();
    let fixed_layout = size_of::<Retained<T>>().max(gui_layout);
    resources.payload_bytes = resources
        .payload_bytes
        .checked_sub(size_of::<T>())
        .and_then(|children| children.checked_add(fixed_layout))
        .ok_or_else(overflow)?;
    let reservation = budget
        .reserve_resources(resources)
        .map_err(|_| "The session retained-data limit would be exceeded.".to_owned())?;
    Ok(reservation.retain(model))
}

/// Adds actual child quantities without counting either containing fixed root again.
/// # Arguments
/// * `total`, `children` - Audited concrete quantities, including repeated shared payloads.
/// # Returns
/// Unit after checked addition of every application resource dimension.
/// # Errors
/// Checked payload/population overflow; this is passive metadata, not admission.
pub(crate) fn append(total: &mut Resources, children: Resources) -> Result<(), String> {
    let value = total.checked_add(children).map_err(|_| overflow())?;
    *total = value;
    Ok(())
}

/// Measures an actual Vec buffer; callers separately visit dynamic element children.
/// # Arguments
/// * `values` - Actual borrowed container, preserving spare requested capacity and inferred T.
/// # Returns
/// Buffer bytes only; no fixed Vec handle, entries or shared roots are added.
/// # Errors
/// Checked capacity times actual element layout overflow.
pub(crate) fn vector_storage<T>(values: &Vec<T>) -> Result<Resources, String> {
    Ok(Resources {
        payload_bytes: product(values.capacity(), size_of::<T>())?,
        ..Default::default()
    })
}

/// Audits a fixed Copy leaf without adding its already-containing inline bytes.
/// # Arguments
/// * `_value` - Actual field; future owned heap fields fail this Copy constraint.
/// # Returns
/// Zero children. Borrowed/static pointees are non-owning.
pub(crate) fn fixed_children<T: Copy>(_value: &T) -> Resources {
    Resources::default()
}

/// Converts complete public root metadata into children of its already-inline value.
/// # Arguments
/// * `metadata` - Actual matching owner's existing complete public metadata.
/// * `inline_bytes` - Exactly size_of of that same public owner, already counted by its parent.
/// # Returns
/// Public dynamic/cache/Arc quantities with the fixed root removed once.
/// # Errors
/// Root underflow; this never substitutes clone or serialized sizes for actual ownership.
pub(crate) fn public_children(
    metadata: RetainedFootprint,
    inline_bytes: usize,
) -> Result<Resources, String> {
    Ok(Resources {
        payload_bytes: metadata
            .payload_bytes
            .checked_sub(inline_bytes)
            .ok_or_else(overflow)?,
        ordered_entries: metadata.ordered_entries,
        shared_roots: metadata.shared_roots,
        ..Default::default()
    })
}

/// Counts actual JSON children, excluding the containing Value fixed layout once.
/// # Arguments
/// * `value` - Existing retained JSON; no serialization or normalization occurs.
/// # Returns
/// Actual String/Array capacity and map key/value payload/entry population.
/// # Errors
/// Checked nested byte/population overflow. Pinned serde_json Number is fixed;
/// arbitrary_precision and preserve_order are absent from the current manifests.
pub(crate) fn json_children(value: &Value) -> Result<Resources, String> {
    let mut total = Resources::default();
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
        Value::String(value) => total.payload_bytes = value.capacity(),
        Value::Array(values) => {
            append(&mut total, vector_storage(values)?)?;
            for value in values {
                append(&mut total, json_children(value)?)?;
            }
        }
        Value::Object(values) => {
            total.payload_bytes = product(values.len(), size_of::<(String, Value)>())?;
            total.ordered_entries = values.len();
            for (key, value) in values {
                append(
                    &mut total,
                    Resources {
                        payload_bytes: key.capacity(),
                        ..Default::default()
                    },
                )?;
                append(&mut total, json_children(value)?)?;
            }
        }
    }
    Ok(total)
}

/// Audits every current public fixed association/grid field.
/// # Arguments
/// * `value` - Actual key borrowed from the owning DTO.
/// # Returns
/// Zero dynamic children; all scalar fields are checked through Copy leaves.
pub(crate) fn key_children(value: &ScenarioKey) -> Resources {
    let ScenarioKey {
        mac_address,
        channel_id,
        grid,
    } = value;
    let FrequencyGrid {
        zero_frequency_hz,
        first_subcarrier_index,
        spacing_hz,
        sample_count,
    } = grid;
    fixed_children(&(
        *mac_address,
        *channel_id,
        *zero_frequency_hz,
        *first_subcarrier_index,
        *spacing_hz,
        *sample_count,
    ))
}

/// Counts current inferred public provenance Strings without creating a nominal API alias.
/// # Arguments
/// * `name`, `hash`, `kind` - Actual SourceProvenance owned fields, never temporary getters.
/// # Returns
/// Requested String capacities only; enclosing DTO counts the fixed handles.
/// # Errors
/// Checked byte addition overflow. Callers audit all remaining current Copy fields.
pub(crate) fn provenance_strings(
    name: &String,
    hash: &String,
    kind: &String,
) -> Result<Resources, String> {
    let mut total = Resources::default();
    for value in [name, hash, kind] {
        append(
            &mut total,
            Resources {
                payload_bytes: value.capacity(),
                ..Default::default()
            },
        )?;
    }
    Ok(total)
}

/// Counts every current public measured-frame child without advancing science.
/// # Arguments
/// * `value` - Existing full-resolution original-grid frame.
/// # Returns
/// Actual sample capacity and provenance Strings, with no additional fixed frame/root.
/// # Errors
/// Checked byte/population overflow. SourceProvenance has no nominal facade path;
/// all current public fields are explicitly audited here and require re-audit if extended.
pub(crate) fn rxmer_frame_children(value: &RxMerFrame) -> Result<Resources, String> {
    let RxMerFrame {
        key,
        source,
        capture_time,
        mer_db,
    } = value;
    let mut total = key_children(key);
    append(&mut total, fixed_children(capture_time))?;
    append(
        &mut total,
        provenance_strings(&source.name, &source.sha256, &source.file_kind)?,
    )?;
    append(
        &mut total,
        fixed_children(&(
            source.byte_length,
            source.capture_time,
            source.format_major,
            source.format_minor,
        )),
    )?;
    append(&mut total, vector_storage(mer_db)?)?;
    for value in mer_db {
        append(&mut total, fixed_children(value))?;
    }
    Ok(total)
}

/// Counts all current public captured-profile children on their original grid.
/// # Arguments
/// * `value` - Existing source profile DTO; no original/catalog clone is made.
/// # Returns
/// Actual assignment capacity and provenance Strings, excluding the fixed profile.
/// # Errors
/// Checked byte/population overflow. SourceProvenance/CarrierAssignment lack nominal
/// facade paths; every current public field is audited and future fields require re-audit.
pub(crate) fn profile_children(value: &SourceProfileFrame) -> Result<Resources, String> {
    let SourceProfileFrame {
        key,
        source,
        capture_time,
        profile_id,
        assignments,
    } = value;
    let mut total = key_children(key);
    append(&mut total, fixed_children(&(*capture_time, *profile_id)))?;
    append(
        &mut total,
        provenance_strings(&source.name, &source.sha256, &source.file_kind)?,
    )?;
    append(
        &mut total,
        fixed_children(&(
            source.byte_length,
            source.capture_time,
            source.format_major,
            source.format_minor,
        )),
    )?;
    append(&mut total, vector_storage(assignments)?)?;
    for value in assignments {
        append(
            &mut total,
            fixed_children(&(value.carrier_type, value.bit_loading, value.origin)),
        )?;
    }
    Ok(total)
}

#[cfg(test)]
#[path = "retention/tests.rs"]
mod tests;
