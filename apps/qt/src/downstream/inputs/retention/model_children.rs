/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Concrete response and recipe children for retained measurement editor owners.
use super::{append, fixed_children, key_children, provenance_strings, vector_storage};
use crate::shell::RetainedResources as Resources;
use merlin::downstream::{
    inputs::{
        CaptureScope, Notch, RandomVariation, Ripple, RippleSpacing, RxmerRecipe, Tilt, ValueEdit,
    },
    ChannelEstimateFrame,
};

/// Counts every current public response-frame child without producing inverse H.
/// # Arguments
/// * `value` - Existing full-resolution original-grid response.
/// # Returns
/// Actual IQ capacity and provenance Strings beyond the containing fixed frame.
/// # Errors
/// Checked byte/population overflow; inferred SourceProvenance has the same audit
/// limitation documented for rxmer_frame_children.
pub(crate) fn response_frame_children(value: &ChannelEstimateFrame) -> Result<Resources, String> {
    let ChannelEstimateFrame {
        key,
        source,
        capture_time,
        iq,
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
    append(&mut total, vector_storage(iq)?)?;
    for value in iq {
        append(&mut total, fixed_children(value))?;
    }
    Ok(total)
}

fn scope(value: &CaptureScope) -> Resources {
    match value {
        CaptureScope::All => Resources::default(),
        CaptureScope::Range { first, last } => fixed_children(&(*first, *last)),
    }
}

/// Counts the actual recipe edit buffer and audits all other public component fields.
/// # Arguments
/// * `value` - Existing recipe; no designer materialization or record serialization occurs.
/// # Returns
/// Dynamic children only, retaining spare edit capacity and fixed fields once in the parent.
/// # Errors
/// Checked edit-buffer arithmetic overflow.
pub(crate) fn recipe_children(value: &RxmerRecipe) -> Result<Resources, String> {
    let RxmerRecipe {
        random_variation,
        tilt,
        ripple,
        notch,
        edits,
    } = value;
    let mut total = vector_storage(edits)?;
    if let Some(RandomVariation {
        standard_deviation_db,
        seed,
    }) = random_variation
    {
        append(&mut total, fixed_children(&(*standard_deviation_db, *seed)))?;
    }
    if let Some(Tilt {
        scope: selected,
        tilt_db,
    }) = tilt
    {
        append(&mut total, scope(selected))?;
        append(&mut total, fixed_children(tilt_db))?;
    }
    if let Some(Ripple {
        scope: selected,
        peak_to_peak_db,
        spacing,
    }) = ripple
    {
        append(&mut total, scope(selected))?;
        append(&mut total, fixed_children(peak_to_peak_db))?;
        match spacing {
            RippleSpacing::Frequency { spacing_hz } => {
                append(&mut total, fixed_children(spacing_hz))?
            }
            RippleSpacing::EchoDistance {
                distance_metres,
                velocity_factor,
            } => append(
                &mut total,
                fixed_children(&(*distance_metres, *velocity_factor)),
            )?,
        }
    }
    if let Some(Notch {
        scope: selected,
        center_hz,
        bandwidth_hz,
        depth_db,
        edge_hz,
    }) = notch
    {
        append(&mut total, scope(selected))?;
        append(
            &mut total,
            fixed_children(&(*center_hz, *bandwidth_hz, *depth_db, *edge_hz)),
        )?;
    }
    for ValueEdit {
        scope: selected,
        first_hz,
        last_hz,
        value_db,
    } in edits
    {
        append(&mut total, scope(selected))?;
        append(
            &mut total,
            fixed_children(&(*first_hz, *last_hz, *value_db)),
        )?;
    }
    Ok(total)
}
