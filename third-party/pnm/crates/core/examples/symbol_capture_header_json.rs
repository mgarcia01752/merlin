/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Header-only symbol stub; this does not decode symbol measurements.
mod common;
use common::ExampleError;
use pnm::{
    common::json::DEFAULT_MAX_JSON_BYTES,
    pnm::parser::{cm_symbol_capture::CmSymbolCapture, pnm_header::HasPnmHeader},
};

/// Prints common metadata for a supplied symbol capture.
///
/// # Returns
/// Header JSON on stdout and an explicit unsupported-payload notice on stderr.
/// # Errors
/// Reports CLI, input, header validation, logging or serialization failures.
fn main() -> Result<(), ExampleError> {
    logging::init()?;
    let bytes = common::capture_bytes(None)?;
    let capture = CmSymbolCapture::parse(&bytes)?;
    eprintln!("Symbol capture payload decoding is unsupported; output contains only its header.");
    println!(
        "{}",
        capture
            .pnm_header()
            .to_json(false, DEFAULT_MAX_JSON_BYTES)?
    );
    Ok(())
}
