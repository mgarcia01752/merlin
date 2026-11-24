/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Passive measurement originals, observed frames and designer-origin ownership.
use super::{Capture, Measurement, Model, Options};
use crate::{downstream::inputs::retention as count, shell::RetainedResources};
use merlin::downstream::{ChannelEstimateFrame, RxMerFrame};
use std::mem::size_of;

fn capture_children(value: &Capture) -> Result<RetainedResources, String> {
    let Capture {
        name,
        original,
        format,
        hash,
        decoded_hash,
        measurement,
        designer,
    } = value;
    let mut total = RetainedResources {
        payload_bytes: name.capacity(),
        ..Default::default()
    };
    count::append(&mut total, count::fixed_children(format))?;
    for text in [hash, decoded_hash] {
        count::append(
            &mut total,
            RetainedResources {
                payload_bytes: text.capacity(),
                ..Default::default()
            },
        )?;
    }
    count::append(
        &mut total,
        RetainedResources {
            payload_bytes: size_of::<Vec<u8>>(),
            shared_roots: 1,
            ..Default::default()
        },
    )?;
    count::append(&mut total, count::vector_storage(original)?)?;
    for value in original.iter() {
        count::append(&mut total, count::fixed_children(value))?;
    }
    match measurement {
        Measurement::Rxmer(frame) => {
            count::append(
                &mut total,
                RetainedResources {
                    payload_bytes: size_of::<RxMerFrame>(),
                    shared_roots: 1,
                    ..Default::default()
                },
            )?;
            count::append(&mut total, count::rxmer_frame_children(frame)?)?;
        }
        Measurement::Response(frame) => {
            count::append(
                &mut total,
                RetainedResources {
                    payload_bytes: size_of::<ChannelEstimateFrame>(),
                    shared_roots: 1,
                    ..Default::default()
                },
            )?;
            count::append(&mut total, count::response_frame_children(frame)?)?;
        }
    }
    if let Some(origin) = designer {
        count::append(
            &mut total,
            RetainedResources {
                shared_roots: 1,
                ..Default::default()
            },
        )?;
        count::append(
            &mut total,
            super::super::designer::origin_resources(origin)?,
        )?;
    }
    Ok(total)
}

/// Measures one actual finite generated collection using the existing Capture traversal.
///
/// # Arguments
/// * `captures` - Complete builder output with requested spare slots preserved.
///
/// # Returns
/// The Vec fixed handle once, its actual slot capacity and every original, typed
/// frame and optional Designer Origin child. Shared payloads are conservatively
/// repeated per capture occurrence, exactly as in the existing Model visitor.
///
/// # Errors
/// Existing public metadata or checked payload/population overflow; no source,
/// scientific value, capacity or alias is mutated.
pub(super) fn capture_batch_resources(
    captures: &Vec<Capture>,
) -> Result<RetainedResources, String> {
    let mut total = RetainedResources {
        payload_bytes: size_of::<Vec<Capture>>(),
        ..Default::default()
    };
    count::append(&mut total, count::vector_storage(captures)?)?;
    for capture in captures {
        count::append(&mut total, capture_children(capture)?)?;
    }
    Ok(total)
}

/// Measures the actual independent draft/accepted measurement payloads and options.
/// # Arguments
/// * `model` - Existing owner; no public input reconstruction/materialization occurs.
/// # Returns
/// Model fixed layout once, actual Vec slots/String capacities, and all repeated Arc
/// pointees/roots including optional independent Designer record/design ownership.
/// # Errors
/// Existing public metadata or checked resource overflow, without mutation.
pub(super) fn resources(model: &Model) -> Result<RetainedResources, String> {
    let Model {
        draft_rx,
        accepted_rx,
        draft_response,
        accepted_response,
        options,
        response_tolerance,
    } = model;
    let mut total = RetainedResources {
        payload_bytes: size_of::<Model>(),
        ..Default::default()
    };
    count::append(&mut total, count::fixed_children(response_tolerance))?;
    for captures in [draft_rx, accepted_rx] {
        count::append(&mut total, count::vector_storage(captures)?)?;
        for capture in captures {
            count::append(&mut total, capture_children(capture)?)?;
        }
    }
    for capture in [draft_response, accepted_response].into_iter().flatten() {
        count::append(&mut total, capture_children(capture)?)?;
    }
    count::append(&mut total, options_children(options)?)?;
    Ok(total)
}

/// Measures every dynamic Options child without counting its containing fixed value.
///
/// # Arguments
/// * `options` - Existing playback choices with their actual owned capacities.
///
/// # Returns
/// Owned interval capacity and selected Vec/String storage only. Borrowed Cow
/// text is non-owning; current options add no ordered entries or shared roots.
///
/// # Errors
/// Checked capacity/byte arithmetic; no selection, value or capacity is changed.
pub(super) fn options_children(options: &Options) -> Result<RetainedResources, String> {
    let mut total = RetainedResources::default();
    let Options {
        mode,
        interval_symbols,
        tolerance_seconds,
        selected_captures,
    } = options;
    count::append(
        &mut total,
        count::fixed_children(&(*mode, *tolerance_seconds)),
    )?;
    count::append(
        &mut total,
        RetainedResources {
            payload_bytes: match interval_symbols {
                std::borrow::Cow::Borrowed(_) => 0,
                std::borrow::Cow::Owned(value) => value.capacity(),
            },
            ..Default::default()
        },
    )?;
    if let Some(captures) = selected_captures {
        count::append(&mut total, count::vector_storage(captures)?)?;
        for capture in captures {
            count::append(
                &mut total,
                RetainedResources {
                    payload_bytes: capture.capacity(),
                    ..Default::default()
                },
            )?;
        }
    }
    Ok(total)
}

#[cfg(test)]
#[path = "retention/tests.rs"]
mod tests;
