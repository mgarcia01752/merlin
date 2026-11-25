/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Use the public CmLatencyRpt API and export its JSON.
//! Requires an LLD01 file path; the decoder has synthetic-test coverage only.
mod common;
use common::ExampleError;
use pnm::{
    common::json::DEFAULT_MAX_JSON_BYTES,
    pnm::parser::{cm_latency_rpt::CmLatencyRpt, parse_limits::ParseLimits},
};

/// Parses the selected capture through its concrete public API.
///
/// # Returns
/// Success after writing bounded JSON to stdout; logging uses stderr.
/// # Errors
/// Propagates CLI, input, parsing, logging and serialization failures.
fn main() -> Result<(), ExampleError> {
    logging::init()?;
    let bytes = common::capture_bytes(None)?;
    let capture = CmLatencyRpt::parse(&bytes, ParseLimits::default())?;
    println!("{}", capture.to_json(DEFAULT_MAX_JSON_BYTES)?);
    Ok(())
}
