/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Exact final layout of the existing serde-tagged public stage JSON, not a growth factor.
use super::{PlanError, ProbeData};
use crate::shell::{RetainedResources, RetentionError};
use merlin::downstream::FullLoopTransportCell;
use serde_json::Value;
use std::mem::size_of;

fn add(a: usize, b: usize) -> Result<usize, PlanError> {
    a.checked_add(b)
        .ok_or(PlanError::Retention(RetentionError::ArithmeticOverflow))
}
fn product(a: usize, b: usize) -> Result<usize, PlanError> {
    a.checked_mul(b)
        .ok_or(PlanError::Retention(RetentionError::ArithmeticOverflow))
}
fn combine(a: RetainedResources, b: RetainedResources) -> Result<RetainedResources, PlanError> {
    Ok(RetainedResources {
        payload_bytes: add(a.payload_bytes, b.payload_bytes)?,
        ordered_entries: add(a.ordered_entries, b.ordered_entries)?,
        shared_roots: 0,
        ..Default::default()
    })
}
fn object(keys: &[&str]) -> Result<RetainedResources, PlanError> {
    let mut bytes = product(keys.len(), size_of::<(String, Value)>())?;
    for key in keys {
        bytes = add(bytes, key.len())?;
    }
    Ok(RetainedResources {
        payload_bytes: bytes,
        ordered_entries: keys.len(),
        shared_roots: 0,
        ..Default::default()
    })
}
fn array(len: usize) -> Result<RetainedResources, PlanError> {
    Ok(RetainedResources {
        payload_bytes: product(len, size_of::<Value>())?,
        ordered_entries: 0,
        shared_roots: 0,
        ..Default::default()
    })
}

/// Plans only dynamic children of the Value already inline in its Snapshot.
///
/// # Arguments
/// * `data` - Borrowed current public variant; arrays, nulls and tags are unchanged.
///
/// # Returns
/// Exact normalized JSON child bytes and ordered entries; no shared roots.
///
/// # Errors
/// Checked arithmetic or an unaudited future public variant rejects before serialization.
pub(super) fn children(data: &ProbeData) -> Result<RetainedResources, PlanError> {
    let (kind, values) = match data {
        ProbeData::Bits(v) => ("bits", array(v.len())?),
        ProbeData::Bytes(v) => ("bytes", array(v.len())?),
        ProbeData::QamLabels(v) => ("qam_labels", array(v.len())?),
        ProbeData::Llrs(v) => ("llrs", array(v.len())?),
        ProbeData::Iq(values) => {
            let mut result = array(values.len())?;
            for value in values {
                result = combine(result, array(value.len())?)?;
            }
            ("iq", result)
        }
        ProbeData::OptionalIq(values) => {
            let mut result = array(values.len())?;
            for value in values.iter().flatten() {
                result = combine(result, array(value.len())?)?;
            }
            ("optional_iq", result)
        }
        ProbeData::TransportCells(values) => {
            let mut result = array(values.len())?;
            for value in values.iter().flatten() {
                let (tag, mut child) = match value {
                    FullLoopTransportCell::PilotPlaceholder => {
                        ("pilot_placeholder", object(&["kind"])?)
                    }
                    FullLoopTransportCell::Sample(iq) => (
                        "sample",
                        combine(object(&["kind", "iq"])?, array(iq.len())?)?,
                    ),
                };
                child.payload_bytes = add(child.payload_bytes, tag.len())?;
                result = combine(result, child)?;
            }
            ("transport_cells", result)
        }
        ProbeData::Pilots(values) => {
            let mut result = array(values.len())?;
            for merlin::downstream::FullLoopPilotValue {
                bin: _,
                sequence_bit: _,
                iq,
            } in values
            {
                result = combine(
                    result,
                    combine(object(&["bin", "sequence_bit", "iq"])?, array(iq.len())?)?,
                )?;
            }
            ("pilots", result)
        }
        _ => return Err(PlanError::UnsupportedRepresentation),
    };
    let mut result = combine(object(&["kind", "values"])?, values)?;
    result.payload_bytes = add(result.payload_bytes, kind.len())?;
    Ok(result)
}

/// Moves an existing serializer result into the exact layout preplanned from borrowed data.
///
/// # Arguments
/// * `value` - Existing JSON whose scientific numbers, nulls, keys and tags must survive.
///
/// # Returns
/// Unit with exact owned text/array capacities; ordered map population is unchanged.
pub(super) fn normalize(value: &mut Value) {
    match value {
        Value::String(text) => *text = String::from(std::mem::take(text).into_boxed_str()),
        Value::Array(values) => {
            for value in values.iter_mut() {
                normalize(value);
            }
            *values = Vec::from(std::mem::take(values).into_boxed_slice());
        }
        Value::Object(values) => {
            *values = std::mem::take(values)
                .into_iter()
                .map(|(key, mut value)| {
                    normalize(&mut value);
                    (String::from(key.into_boxed_str()), value)
                })
                .collect();
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
}

#[cfg(test)]
mod tests;
