/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Passive private ownership metadata for an actual retained worker diagnostic.
use super::{AdmissionError, Diagnostic};
use crate::{downstream::inputs::retention as count, shell::RetainedResources};
use std::mem::size_of;

/// Counts an AdmissionError and its complete optional private Diagnostic payload.
///
/// # Arguments
/// * `error` - Actual failure whose original text and guidance are borrowed unchanged.
///
/// # Returns
/// One fixed AdmissionError, message capacity, and optional Diagnostic fixed value
/// plus its four owned String capacities. The inline Box handle is already in the
/// error root. Exclusive Box ownership introduces no shared roots or ordered entries.
///
/// # Errors
/// Checked payload overflow using the existing resource append helper; no values,
/// capacities, pointers, scientific state or application reservations are changed.
pub(super) fn resources(error: &AdmissionError) -> Result<RetainedResources, String> {
    let AdmissionError {
        message,
        diagnostic,
    } = error;
    let mut total = RetainedResources {
        payload_bytes: size_of::<AdmissionError>(),
        ..Default::default()
    };
    count::append(
        &mut total,
        RetainedResources {
            payload_bytes: message.capacity(),
            ..Default::default()
        },
    )?;
    if let Some(diagnostic) = diagnostic {
        let Diagnostic {
            code,
            title,
            tooltip,
            faq_anchor,
            current_duration_ms,
            final_transition_symbol,
            final_transition_ms,
            minimum_duration_ms,
            maximum_duration_ms,
        } = diagnostic.as_ref();
        count::append(
            &mut total,
            RetainedResources {
                payload_bytes: size_of::<Diagnostic>(),
                ..Default::default()
            },
        )?;
        for text in [code, title, tooltip, faq_anchor] {
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
            count::fixed_children(&(
                *current_duration_ms,
                *final_transition_symbol,
                *final_transition_ms,
                *minimum_duration_ms,
                *maximum_duration_ms,
            )),
        )?;
    }
    Ok(total)
}
