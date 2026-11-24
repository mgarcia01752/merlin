/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Passive original/catalog/presentation ownership; shared Context graph remains separate.
use super::{document::Capture, ProfileModel};
use crate::{downstream::inputs::retention as count, shell::RetainedResources};
use merlin::downstream::{NamedCapture, ProfileCapture};
use serde_json::Value;
use std::mem::size_of;

fn capture_children(value: &Capture) -> Result<RetainedResources, String> {
    let Capture {
        name,
        format,
        original,
        sha256,
        catalog,
        document,
    } = value;
    let mut total = RetainedResources {
        payload_bytes: name.capacity(),
        ..Default::default()
    };
    count::append(&mut total, count::fixed_children(format))?;
    count::append(
        &mut total,
        RetainedResources {
            payload_bytes: sha256.capacity(),
            ..Default::default()
        },
    )?;
    count::append(
        &mut total,
        count::public_children(
            original
                .retained_footprint()
                .map_err(|error| error.message().to_owned())?,
            size_of::<NamedCapture>(),
        )?,
    )?;
    count::append(
        &mut total,
        count::public_children(
            catalog
                .retained_footprint()
                .map_err(|error| error.message().to_owned())?,
            size_of::<ProfileCapture>(),
        )?,
    )?;
    count::append(
        &mut total,
        RetainedResources {
            payload_bytes: size_of::<Value>(),
            shared_roots: 1,
            ..Default::default()
        },
    )?;
    count::append(&mut total, count::json_children(document)?)?;
    Ok(total)
}

/// Measures actual original input, catalog and decoded presentation ownership.
/// # Arguments
/// * `model` - Existing draft/accepted owner; no parsing or cloning occurs.
/// # Returns
/// ProfileModel fixed layout once and every repeated draft/accepted payload occurrence.
/// The fixed Context handle is inline; its reachable graph is separately lead-owned,
/// an unfinished owner under the contract rather than a contract exclusion.
/// # Errors
/// Public metadata or checked resource overflow, with unchanged originals/selection.
pub(super) fn resources(model: &ProfileModel) -> Result<RetainedResources, String> {
    let ProfileModel {
        context: _,
        draft,
        accepted,
    } = model;
    let mut total = RetainedResources {
        payload_bytes: size_of::<ProfileModel>(),
        ..Default::default()
    };
    if let Some(capture) = draft {
        count::append(&mut total, capture_children(capture)?)?;
    }
    if let Some((capture, id)) = accepted {
        count::append(&mut total, count::fixed_children(id))?;
        count::append(&mut total, capture_children(capture)?)?;
    }
    Ok(total)
}

#[cfg(test)]
#[path = "retention/tests.rs"]
mod tests;
