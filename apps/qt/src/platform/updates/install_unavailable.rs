/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Full MSI installation owns Windows updates; no automatic elevation helper.

/// Reports that automatic installation is unavailable on this platform.
pub(crate) fn supported() -> bool {
    false
}

/// Rejects automatic installation; progress is unused because no work starts.
/// # Arguments
/// * `_progress` - Caller callback, never invoked on this platform.
/// # Errors
/// Always directs the user to the full release installer.
pub(crate) fn update(_progress: impl Fn(String)) -> Result<(), String> {
    Err("Download the newer MSI from the MERLIN Releases page.".into())
}

/// Rejects the internal Linux update entry point without reading its path.
/// # Arguments
/// * `_directory` - Unused update staging path.
/// # Errors
/// Always reports that automatic installation is unavailable.
pub(crate) fn elevated(_directory: &std::path::Path) -> Result<(), String> {
    Err("Use the full installer from the MERLIN Releases page.".into())
}

/// Rejects automatic restart because this platform has no in-app installer.
/// # Errors
/// Always asks the user to launch the installed application normally.
pub(crate) fn restart() -> Result<(), String> {
    Err("Launch MERLIN from the Start menu after installation.".into())
}
