/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Process-owned desktop logging; libraries retain their caller-owned facade.
/// Installs a stderr logger without writing capture data or creating log files.
/// # Arguments
/// * `debug_ui` - Enables desktop debug records by default when explicitly set.
/// # Returns
/// No value when initialization succeeds; RUST_LOG overrides the default level.
/// # Errors
/// Reports an existing logger rather than replacing or panicking over it.
pub(crate) fn initialize(debug_ui: bool) -> Result<(), log::SetLoggerError> {
    let default_filter = if debug_ui {
        "warn,merlin_qt=debug"
    } else {
        "warn"
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(default_filter))
        .target(env_logger::Target::Stderr)
        .format_timestamp(None)
        .try_init()
}
