/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Use the public CmSpectrumAnalysis API and export its JSON.
mod common;
use common::ExampleError;
use pnm::{
    common::json::DEFAULT_MAX_JSON_BYTES,
    pnm::parser::{cm_spectrum_analysis::CmSpectrumAnalysis, parse_limits::ParseLimits},
};

const FIXTURE: &[u8] = pnm_fixtures::SPECTRUM;

/// Parses the selected capture through its concrete public API.
///
/// # Returns
/// Success after writing bounded JSON to stdout; logging uses stderr.
/// # Errors
/// Propagates CLI, input, parsing, logging and serialization failures.
fn main() -> Result<(), ExampleError> {
    logging::init()?;
    let bytes = common::capture_bytes(Some(FIXTURE))?;
    let capture = CmSpectrumAnalysis::parse(&bytes, ParseLimits::default())?;
    println!("{}", capture.to_json(DEFAULT_MAX_JSON_BYTES)?);
    Ok(())
}
