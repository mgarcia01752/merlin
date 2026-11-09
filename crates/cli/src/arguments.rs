/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Command-line syntax and small request fields.
use std::path::PathBuf;
const HELP: &str = r#"Usage: ofdm-fec <simulate|evaluate|channel|import-pnm> --input FILE --output FILE [--scenario FILE --candidate FILE --frame INDEX]

Commands:
  simulate  Run a seeded short8/9 uniform16-QAM coding experiment.
  evaluate  Generate/evaluate a candidate from a decoded scenario JSON.
  channel   Run the duration-based reference or impaired OFDM channel.
  import-pnm Import three decoded PNM JSON files into a scenario.

Use --payload-file FILE with channel and source.kind=file for exact text/binary bytes.
Use --recovered-file FILE with file-source channel jobs for recovered original bytes.
Use --probes-output FILE with channel and request.probes for bounded JSONL export.

Use --candidate with evaluate to test an existing internal candidate.
Use --settings FILE and --profile INDEX to generate an explicit candidate.
Use --scenario and --candidate with simulate for measured per-carrier noise.
Use --scenario with channel and request.measurement for waveform RxMER noise.
Existing output files are never replaced. Ctrl-C preserves completed FEC trials.
RUST_LOG controls stderr logging (default: info)."#;
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
    pub(crate) frame_supplied: bool,
    pub(crate) probes_output: Option<PathBuf>,
    pub(crate) payload_file: Option<PathBuf>,
    pub(crate) recovered_file: Option<PathBuf>,
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
            Some("--payload-file") => arguments.payload_file = Some(value.into()),
            Some("--recovered-file") => arguments.recovered_file = Some(value.into()),
            Some("--probes-output") => arguments.probes_output = Some(value.into()),
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
                arguments.frame_supplied = true;
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
    if arguments.command != "channel"
        && (arguments.payload_file.is_some() || arguments.recovered_file.is_some())
    {
        return Err("--payload-file and --recovered-file apply to channel only.".into());
    }
    if arguments.command != "channel" && arguments.probes_output.is_some() {
        return Err("--probes-output applies to channel only.".into());
    }
    Ok(Some(arguments))
}
