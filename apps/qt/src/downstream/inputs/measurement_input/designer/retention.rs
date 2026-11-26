/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Owner-local Designer record/design/session metadata; private fields stay private.
use super::{Baseline, Origin, Record, Request, Session};
use crate::{downstream::inputs::retention as count, shell::RetainedResources};
use merlin::downstream::inputs::RxMerDesign;
use merlin::downstream::{ProfileCapture, SourceProfileFrame};
use std::mem::size_of;

fn record_children(value: &Record) -> Result<RetainedResources, String> {
    let Record {
        merlin_rxmer_designer,
        baseline,
        recipe,
        capture_index,
        capture_count,
    } = value;
    let mut total =
        count::fixed_children(&(*merlin_rxmer_designer, *capture_index, *capture_count));
    match baseline {
        Baseline::Imported { original } => {
            count::append(&mut total, count::json_children(original)?)?
        }
        Baseline::Flat {
            key,
            anchor_epoch,
            valid,
            baseline_db,
        } => {
            count::append(&mut total, count::key_children(key))?;
            count::append(
                &mut total,
                count::fixed_children(&(*anchor_epoch, *baseline_db)),
            )?;
            count::append(&mut total, count::vector_storage(valid)?)?;
            for value in valid {
                count::append(&mut total, count::fixed_children(value))?;
            }
        }
    }
    count::append(&mut total, count::recipe_children(recipe)?)?;
    Ok(total)
}
fn origin_children(value: &Origin) -> Result<RetainedResources, String> {
    let Origin { record, design } = value;
    let mut total = record_children(record)?;
    count::append(
        &mut total,
        count::public_children(
            design
                .retained_footprint()
                .map_err(|error| error.message().to_owned())?,
            size_of::<RxMerDesign>(),
        )?,
    )?;
    Ok(total)
}

/// Measures one actual private Designer Origin and every independent record/design child.
/// # Arguments
/// * `origin` - Existing immutable owner, not a clone or reserialized record.
/// # Returns
/// Full sizeof(Origin) once plus complete original/design/cache/recipe/JSON children.
/// An enclosing Arc caller adds its root edge but not another Origin fixed value.
/// # Errors
/// Public design metadata or checked resource overflow, preserving actual data.
pub(super) fn origin_resources(origin: &Origin) -> Result<RetainedResources, String> {
    let mut total = RetainedResources {
        payload_bytes: size_of::<Origin>(),
        ..Default::default()
    };
    count::append(&mut total, origin_children(origin)?)?;
    Ok(total)
}

/// Measures the actual independent Designer editing session, without materializing captures.
/// # Arguments
/// * `session` - Existing session; Context/bridge/control owners remain separate.
/// # Returns
/// Full Session fixed layout once, actual hashes/origins/request storage, shared original
/// profile pointee/root, and complete public catalog/design quantities repeated per owner.
/// # Errors
/// Public metadata or checked resource overflow, with unchanged source/recipe/selection.
pub(super) fn session_resources(session: &Session) -> Result<RetainedResources, String> {
    let Session {
        hashes,
        profile,
        catalog,
        origins,
        request,
    } = session;
    let mut total = RetainedResources {
        payload_bytes: size_of::<Session>(),
        ..Default::default()
    };
    count::append(&mut total, count::vector_storage(hashes)?)?;
    for hash in hashes {
        count::append(
            &mut total,
            RetainedResources {
                payload_bytes: hash.capacity(),
                ..Default::default()
            },
        )?;
    }
    count::append(
        &mut total,
        RetainedResources {
            payload_bytes: size_of::<SourceProfileFrame>(),
            shared_roots: 1,
            ..Default::default()
        },
    )?;
    count::append(&mut total, count::profile_children(profile)?)?;
    count::append(
        &mut total,
        count::public_children(
            catalog
                .retained_footprint()
                .map_err(|error| error.message().to_owned())?,
            size_of::<ProfileCapture>(),
        )?,
    )?;
    count::append(&mut total, count::vector_storage(origins)?)?;
    for origin in origins {
        count::append(&mut total, origin_children(origin)?)?;
    }
    count::append(&mut total, request_children(request)?)?;
    Ok(total)
}

/// Measures every dynamic Request child without counting its containing fixed value.
///
/// # Arguments
/// * `request` - Actual parsed source/recipe owner; spare capacities are preserved.
///
/// # Returns
/// Source String and recipe child capacities only; no entries or shared roots
/// belong to the current Request fields. Values and capacities are unchanged.
///
/// # Errors
/// Checked byte arithmetic from the existing source and recipe visitors.
pub(super) fn request_children(request: &Request) -> Result<RetainedResources, String> {
    let mut total = RetainedResources::default();
    let Request {
        source,
        capture_count,
        baseline_db,
        recipe,
    } = request;
    count::append(
        &mut total,
        count::fixed_children(&(*capture_count, *baseline_db)),
    )?;
    count::append(
        &mut total,
        RetainedResources {
            payload_bytes: source.capacity(),
            ..Default::default()
        },
    )?;
    count::append(&mut total, count::recipe_children(recipe)?)?;
    Ok(total)
}

#[cfg(test)]
#[path = "retention/tests.rs"]
mod tests;
