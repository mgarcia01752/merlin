/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Export a local PNM capture through the public centralized dispatch API.
mod common;
use common::ExampleError;
use pnm::{
    common::json::DEFAULT_MAX_JSON_BYTES,
    pnm::parser::{parse_limits::ParseLimits, pnm_file::PnmFile},
};

/// Identifies and decodes one supplied file and prints bounded JSON.
///
/// # Returns
/// Success after writing the concrete capture JSON to stdout.
/// # Errors
/// Reports CLI, input, unsupported formats, decoding or serialization failures.
fn main() -> Result<(), ExampleError> {
    logging::init()?;
    let bytes = common::capture_bytes(None)?;
    let file = PnmFile::from_bytes(&bytes);
    let capture = file.decode(ParseLimits::default())?;
    println!("{}", capture.to_json(DEFAULT_MAX_JSON_BYTES)?);
    Ok(())
}
