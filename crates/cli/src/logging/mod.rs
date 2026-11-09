/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Process-owned stderr logging. Scientific libraries never install a global logger.
/// Installs the standard log backend once at CLI startup.
/// # Returns
/// Unit with info default and RUST_LOG module/level overrides; stdout is untouched.
/// Optional regex/color/time features are disabled to keep the backend lean.
/// # Errors
/// Reports an already installed logger instead of panicking or replacing it.
pub(crate) fn initialize() -> Result<(), log::SetLoggerError> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .target(env_logger::Target::Stderr)
        .format_timestamp(None)
        .try_init()
}
