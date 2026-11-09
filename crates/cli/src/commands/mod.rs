/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Small command dispatch; each operation owns its adaptation layer.
mod channel;
mod evaluate;
mod import_pnm;
mod simulate;
use crate::arguments::Arguments;
use std::sync::atomic::AtomicBool;
/// Dispatches one validated command request to the shared scientific library.
/// # Arguments
/// * `arguments` - Paths and indices parsed from the command line.
/// * `cancel` - Cooperative signal token shared with the job.
/// # Returns
/// Unit after exporting the actual Rust result.
/// # Errors
/// Reports input, scientific-validation, cancellation and output failures.
pub(crate) fn run(arguments: Arguments, cancel: &AtomicBool) -> Result<(), String> {
    log::info!("command-start command={}", arguments.command);
    match arguments.command.as_str() {
        "import-pnm" => import_pnm::run(arguments),
        "channel" => channel::run(arguments, cancel),
        "simulate" => simulate::run(arguments, cancel),
        "evaluate" => evaluate::run(arguments, cancel),
        _ => Err("Unknown command. Use --help.".into()),
    }
}
