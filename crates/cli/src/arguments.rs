/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Command-line syntax and small request fields.
use std::path::PathBuf;
const HELP: &str = r#"Usage: ofdm-fec <simulate|evaluate> --input FILE --output FILE [--scenario FILE --candidate FILE --frame INDEX]

Commands:
  simulate  Run a seeded short8/9 uniform16-QAM coding experiment.
  evaluate  Generate/evaluate a candidate from a decoded scenario JSON.

Use --candidate with evaluate to test an existing internal candidate.
Use --settings FILE and --profile INDEX to generate an explicit candidate.
Use --scenario and --candidate with simulate for measured per-carrier noise.
Existing output files are never replaced. Ctrl-C preserves completed FEC trials."#;
#[derive(Default)]
pub(crate) struct Arguments {
    pub(crate) command: String,
    pub(crate) input: PathBuf,
    pub(crate) output: PathBuf,
    pub(crate) scenario: Option<PathBuf>,
    pub(crate) candidate: Option<PathBuf>,
    pub(crate) settings: Option<PathBuf>,
    pub(crate) profile: Option<usize>,
    pub(crate) frame: usize,
}
/// Parses process arguments, printing help/version without starting a job.
/// # Returns
/// A bounded command request, or None after help/version output.
/// # Errors
/// Rejects unknown options, missing values and malformed indices.
pub(crate) fn parse() -> Result<Option<Arguments>, String> {
    let mut values = std::env::args_os().skip(1);
    let Some(command) = values.next() else {
        println!("{HELP}");
        return Ok(None);
    };
    if command == "--help" || command == "-h" {
        println!("{HELP}");
        return Ok(None);
    }
    if command == "--version" {
        println!("ofdm-fec {}", env!("CARGO_PKG_VERSION"));
        return Ok(None);
    }
    let mut arguments = Arguments {
        command: command
            .into_string()
            .map_err(|_| "Invalid command encoding.")?,
        ..Default::default()
    };
    while let Some(option) = values.next() {
        let value = values.next().ok_or("Each option requires a value.")?;
        match option.to_str() {
            Some("--input") => arguments.input = value.into(),
            Some("--output") => arguments.output = value.into(),
            Some("--scenario") => arguments.scenario = Some(value.into()),
            Some("--candidate") => arguments.candidate = Some(value.into()),
            Some("--settings") => arguments.settings = Some(value.into()),
            Some("--profile") => {
                arguments.profile = Some(
                    value
                        .to_str()
                        .ok_or("Invalid profile index.")?
                        .parse()
                        .map_err(|_| "Profile must be a nonnegative index.")?,
                )
            }
            Some("--frame") => {
                arguments.frame = value
                    .to_str()
                    .ok_or("Invalid frame.")?
                    .parse()
                    .map_err(|_| "Frame must be a nonnegative index.")?
            }
            _ => return Err("Unknown command option.".into()),
        }
    }
    if arguments.input.as_os_str().is_empty() || arguments.output.as_os_str().is_empty() {
        return Err("--input and --output are required.".into());
    }
    Ok(Some(arguments))
}
