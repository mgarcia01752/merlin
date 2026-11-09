/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
mod fixtures;
use std::{error::Error, fs::OpenOptions, io::Write, path::PathBuf};
fn main() -> Result<(), Box<dyn Error>> {
    let destination = std::env::args_os()
        .nth(1)
        .ok_or("Supply a new output directory")?;
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/measurements");
    let source = serde_json::from_slice(&std::fs::read(
        root.join("single-channel/modulation_profile.json"),
    )?)?;
    let destination = PathBuf::from(destination);
    // Refuse an existing root before any output mutation.
    std::fs::create_dir(&destination)?;
    for width in [24_000_000, 90_000_000] {
        let directory = destination.join(format!("{}mhz", width / 1_000_000));
        std::fs::create_dir(&directory)?;
        for (name, document) in fixtures::documents(width, &source)? {
            let mut output = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(directory.join(name))?;
            serde_json::to_writer_pretty(&mut output, &document)?;
            writeln!(output)?;
        }
    }
    Ok(())
}
