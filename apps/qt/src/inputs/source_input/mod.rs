/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Source editing and independent project restoration.
pub(crate) mod model;
/// Validates an offline project source into an independent scientific snapshot.
/// # Arguments
/// * `value` - Stored original source bytes/options.
/// # Returns
/// None for an unconfigured source, otherwise validated exact bytes/PRBS seed.
/// # Errors
/// Reports malformed source data without changing the current workbench.
pub(crate) fn restore_source(
    value: &serde_json::Value,
) -> Result<Option<crate::workbench::session::AcceptedSource>, String> {
    Ok(model::SourceModel::from_saved(value)?.snapshot())
}
