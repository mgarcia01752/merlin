/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Passive source draft/accepted payload metadata, without source generation or cloning.
use super::{Accepted, Options, SourceFile, SourceModel};
use crate::{downstream::inputs::retention as count, shell::RetainedResources};
use merlin::downstream::SourceInput;
use std::mem::size_of;

fn file_children(file: &SourceFile) -> Result<RetainedResources, String> {
    let SourceFile { payload } = file;
    count::public_children(
        payload
            .retained_footprint()
            .map_err(|error| error.message().to_owned())?,
        size_of::<SourceInput>(),
    )
}

/// Measures this actual source model, including draft and accepted repeated payloads.
/// # Arguments
/// * `model` - Existing owner; no source bits, export JSON or clone is produced.
/// # Returns
/// Full SourceModel fixed layout once plus every actually present dynamic child/root.
/// # Errors
/// Public metadata or checked resource overflow, leaving the owner unchanged.
pub(super) fn resources(model: &SourceModel) -> Result<RetainedResources, String> {
    let SourceModel { file, accepted } = model;
    let mut total = RetainedResources {
        payload_bytes: size_of::<SourceModel>(),
        ..Default::default()
    };
    if let Some(file) = file {
        count::append(&mut total, file_children(file)?)?;
    }
    if let Some(Accepted {
        options,
        file,
        config,
    }) = accepted
    {
        let Options {
            mode,
            seed,
            append,
            target_bits,
            fill_seed,
        } = options;
        count::append(
            &mut total,
            RetainedResources {
                payload_bytes: mode.capacity(),
                ..Default::default()
            },
        )?;
        count::append(
            &mut total,
            count::fixed_children(&(*seed, *append, *target_bits, *fill_seed)),
        )?;
        if let Some(file) = file {
            count::append(&mut total, file_children(file)?)?;
        }
        count::append(
            &mut total,
            count::public_children(
                config
                    .retained_footprint()
                    .map_err(|error| error.message().to_owned())?,
                size_of::<SourceInput>(),
            )?,
        )?;
    }
    Ok(total)
}

#[cfg(test)]
#[path = "retention/tests.rs"]
mod tests;
