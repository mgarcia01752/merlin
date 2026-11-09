/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Starts the standalone driver and owns process signal/exit handling.
mod arguments;
mod commands;
mod json_io;
mod logging;
mod source_io;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
/// Runs the standalone JSON driver without network access or device operations.
///
/// Errors go to stderr and return status1; Ctrl-C requests bounded cancellation.
fn main() {
    if let Err(error) = logging::initialize() {
        // The backend is unavailable here; this is the only bootstrap fallback.
        eprintln!("ERROR: logging-initialization-failed error={error}");
        std::process::exit(1);
    }
    let result = (|| {
        let Some(arguments) = arguments::parse()? else {
            return Ok(());
        };
        let cancel = Arc::new(AtomicBool::new(false));
        let signal = cancel.clone();
        ctrlc::set_handler(move || signal.store(true, Ordering::Relaxed))
            .map_err(|error| error.to_string())?;
        commands::run(arguments, &cancel)
    })();
    if let Err(error) = result {
        log::error!("command-failed error={error}");
        std::process::exit(1);
    }
}
