/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Generate synthetic CCAP RxMER binary and provenance in an exclusive new folder.
use pnm_input::upstream::{SyntheticRxmerCapture, MAX_SYNTHETIC_RXMER_REQUEST_BYTES};
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

const BINARY_NAME: &str = "synthetic-cmts-rxmer.bin";
const SIDECAR_NAME: &str = "synthetic-cmts-rxmer.bin.merlin.json";

/// Generates one file without running simulation or overwriting existing paths.
///
/// # Returns
/// Unit after writing binary/provenance and printing the provenance JSON.
///
/// # Errors
/// Rejects missing/extra arguments, oversized/invalid input, existing output
/// directories and I/O failures. A failed write may leave a new incomplete folder;
/// no success is reported and the original request remains unchanged.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args_os().skip(1);
    let request_path = PathBuf::from(
        arguments
            .next()
            .ok_or("Expected REQUEST_JSON OUTPUT_DIRECTORY")?,
    );
    let output = PathBuf::from(
        arguments
            .next()
            .ok_or("Expected REQUEST_JSON OUTPUT_DIRECTORY")?,
    );
    if arguments.next().is_some() {
        return Err("Unexpected extra argument".into());
    }
    let mut request = String::new();
    fs::File::open(request_path)?
        .take(MAX_SYNTHETIC_RXMER_REQUEST_BYTES as u64 + 1)
        .read_to_string(&mut request)?;
    let generated = SyntheticRxmerCapture::from_json(&request)?;
    let provenance = generated.to_json()?;

    // Exclusive directory creation preserves every existing destination, including
    // empty directories. Write provenance last so it only describes a complete pair.
    fs::create_dir(&output)?;
    let mut binary = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output.join(BINARY_NAME))?;
    binary.write_all(generated.capture().original_bytes())?;
    binary.sync_all()?;
    let mut sidecar = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output.join(SIDECAR_NAME))?;
    sidecar.write_all(provenance.as_bytes())?;
    sidecar.sync_all()?;
    println!("{provenance}");
    Ok(())
}
