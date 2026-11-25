/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Application-owned logging setup. Parsers emit events but never install a logger.
#![forbid(unsafe_code)]
use tracing_subscriber::EnvFilter;

/// Default log level when RUST_LOG is unset.
pub const DEFAULT_LOG_FILTER: &str = "info";

/// Installs process-wide text logging to stderr, once at application startup.
///
/// # Returns
/// Success after installing a subscriber using RUST_LOG or the info default.
/// Output includes event levels, module targets and active span context.
/// # Errors
/// Reports invalid/non-Unicode configuration or an existing global subscriber.
/// Never replaces a subscriber installed by the caller.
pub fn init() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let filter = match std::env::var("RUST_LOG") {
        Ok(filter) => filter,
        Err(std::env::VarError::NotPresent) => DEFAULT_LOG_FILTER.to_owned(),
        Err(error) => return Err(error.into()),
    };
    let filter = EnvFilter::try_new(filter)?;
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .try_init()
}
