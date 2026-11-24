/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Passive actual storage metadata for one prepared immutable export product.
use super::{Prepared, RangePolicy};
use crate::{downstream::inputs::retention as count, shell::RetainedResources};
use std::mem::size_of;

/// Measures a prepared export product without encoding frames or changing state.
///
/// # Arguments
/// * `prepared` - Actual owner, including requested spare frame/text/JSON capacity.
///
/// # Returns
/// The full Prepared fixed layout once, frame buffer plus dynamic frame children,
/// all four JSON children and optional PNM error capacity. Containing frame/Value
/// fixed fields are counted once by their Vec slots or Prepared layout. No new
/// shared owner, admission reservation, serialization or scientific query is made.
///
/// # Errors
/// Checked payload or ordered-entry overflow from the existing counting leaves;
/// the product's values, capacities and original-grid identities stay unchanged.
pub(super) fn resources(prepared: &Prepared) -> Result<RetainedResources, String> {
    let Prepared {
        frames,
        policy,
        context,
        summary,
        json_summary,
        pnm_error,
        diagnostic,
        collection,
    } = prepared;
    let mut total = RetainedResources {
        payload_bytes: size_of::<Prepared>(),
        ..Default::default()
    };
    count::append(&mut total, count::vector_storage(frames)?)?;
    for frame in frames {
        count::append(&mut total, count::rxmer_frame_children(frame)?)?;
    }
    for value in [context, summary, json_summary, diagnostic] {
        count::append(&mut total, count::json_children(value)?)?;
    }
    count::append(
        &mut total,
        RetainedResources {
            payload_bytes: pnm_error.as_ref().map_or(0, String::capacity),
            ..Default::default()
        },
    )?;
    match policy {
        RangePolicy::Reject | RangePolicy::Clamp => {}
    }
    count::append(&mut total, count::fixed_children(&(*policy, *collection)))?;
    Ok(total)
}

#[cfg(test)]
mod tests;
