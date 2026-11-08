/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Standalone bounded JSON driver for the shared Rust library.
use docsis_ofdm_fec::profile::{
    document::{verify_captured_roles, CandidateDocument, ScenarioDocument},
    evaluation::{evaluate, generate_candidate, EvaluationSettings},
    simulation::{simulate, MeasuredNoise, SimulationSettings},
};
use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
const MAX_JSON_BYTES: u64 = 16 * 1024 * 1024;
const HELP:&str="Usage: ofdm-fec <simulate|evaluate> --input FILE --output FILE [--scenario FILE --candidate FILE --frame INDEX]\n\nCommands:\n  simulate  Run a seeded short8/9 uniform16-QAM coding experiment.\n  evaluate  Generate/evaluate a candidate from a decoded scenario JSON.\n\nUse --candidate with evaluate to test an existing internal candidate.\nUse --settings FILE and --profile INDEX to generate an explicit candidate.\nUse --scenario and --candidate with simulate for measured per-carrier noise.\nExisting output files are never replaced. Ctrl-C preserves completed FEC trials.";
#[derive(Default)]
struct Arguments {
    command: String,
    input: PathBuf,
    output: PathBuf,
    scenario: Option<PathBuf>,
    candidate: Option<PathBuf>,
    settings: Option<PathBuf>,
    profile: Option<usize>,
    frame: usize,
}
fn arguments() -> Result<Option<Arguments>, String> {
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
fn read_json<Document: serde::de::DeserializeOwned>(path: &Path) -> Result<Document, String> {
    let file = std::fs::File::open(path).map_err(|error| error.to_string())?;
    let mut bytes = vec![];
    file.take(MAX_JSON_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() as u64 > MAX_JSON_BYTES {
        return Err("Input exceeds16MiB.".into());
    }
    serde_json::from_slice(&bytes).map_err(|error| error.to_string())
}
fn write_json(document: &impl serde::Serialize, path: &Path) -> Result<(), String> {
    // Serialization is bounded before opening a destination, preserving transactional failure.
    struct Bounded(Vec<u8>);
    impl Write for Bounded {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if self.0.len() + bytes.len() > MAX_JSON_BYTES as usize {
                return Err(std::io::Error::other("Output exceeds16MiB."));
            }
            self.0.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut bytes = Bounded(vec![]);
    serde_json::to_writer_pretty(&mut bytes, document).map_err(|error| error.to_string())?;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| error.to_string())?;
    if let Err(error) = file.write_all(&bytes.0).and_then(|()| file.sync_all()) {
        drop(file);
        let _ = std::fs::remove_file(path);
        return Err(error.to_string());
    }
    Ok(())
}
fn run(arguments: Arguments, cancel: &AtomicBool) -> Result<(), String> {
    match arguments.command.as_str() {
        "simulate" => {
            if arguments.settings.is_some() || arguments.profile.is_some() {
                return Err("--settings and --profile apply to evaluate only.".into());
            }
            let settings: SimulationSettings = read_json(&arguments.input)?;
            let scenario: Option<ScenarioDocument> =
                arguments.scenario.as_deref().map(read_json).transpose()?;
            let candidate: Option<CandidateDocument> =
                arguments.candidate.as_deref().map(read_json).transpose()?;
            if scenario.is_some() != candidate.is_some() {
                return Err("Measured simulation requires both --scenario and --candidate.".into());
            }
            if let Some(scenario) = &scenario {
                scenario.validate().map_err(|error| error.to_string())?;
            }
            if let Some(candidate) = &candidate {
                candidate.validate().map_err(|error| error.to_string())?;
            }
            let measured = match (&scenario, &candidate) {
                (Some(scenario), Some(candidate)) => {
                    let frame = scenario
                        .frames
                        .get(arguments.frame)
                        .ok_or("Frame index is outside the scenario.")?;
                    Some(MeasuredNoise {
                        frame,
                        candidate: &candidate.candidate,
                        first: 0,
                        count: frame.mer_db.len(),
                    })
                }
                _ => None,
            };
            let result = simulate(&settings, measured.as_ref(), cancel, |progress| {
                eprintln!(
                    "Codewords {}/{} · {} ms",
                    progress.completed_codewords, progress.requested_codewords, progress.elapsed_ms
                )
            })
            .map_err(|error| error.to_string())?;
            write_json(&result, &arguments.output)
        }
        "evaluate" => {
            if arguments.scenario.is_some() {
                return Err("evaluate reads the scenario through --input.".into());
            }
            let scenario: ScenarioDocument = read_json(&arguments.input)?;
            scenario.validate().map_err(|error| error.to_string())?;
            let selected = scenario
                .frames
                .get(arguments.frame)
                .ok_or("Frame index is outside the scenario.")?;
            let frames: Vec<_> = scenario
                .frames
                .iter()
                .filter(|frame| frame.key == selected.key)
                .collect();
            let mut document = if let Some(path) = arguments.candidate {
                let document: CandidateDocument = read_json(&path)?;
                document.validate().map_err(|error| error.to_string())?;
                document
            } else {
                let settings: EvaluationSettings = arguments
                    .settings
                    .as_deref()
                    .map(read_json)
                    .transpose()?
                    .unwrap_or_default();
                let template = arguments
                    .profile
                    .map(|index| {
                        scenario
                            .profiles
                            .get(index)
                            .ok_or("Profile index is outside the scenario.")
                    })
                    .transpose()?;
                if template.is_some_and(|profile| {
                    profile.capture_time.abs_diff(selected.capture_time)
                        > settings.time_tolerance_seconds
                }) {
                    return Err("Profile timestamp exceeds explicit association tolerance.".into());
                }
                let candidate =
                    generate_candidate(&frames, template, &settings, scenario.protocol, cancel)
                        .map_err(|error| error.to_string())?;
                CandidateDocument {
                    document_kind: "docsis_ofdm_fec_candidate".into(),
                    schema_version: 1,
                    candidate,
                    settings,
                    evaluation: None,
                }
            };
            verify_captured_roles(&mut document.candidate, &scenario)
                .map_err(|error| error.to_string())?;
            document.evaluation = Some(
                evaluate(&document.candidate, &frames, &document.settings, cancel)
                    .map_err(|error| error.to_string())?,
            );
            write_json(&document, &arguments.output)
        }
        _ => Err("Unknown command. Use --help.".into()),
    }
}
/// Runs the standalone JSON driver without network access or device operations.
///
/// Errors go to stderr and return status1; Ctrl-C requests bounded cancellation.
fn main() {
    let result = (|| {
        let Some(arguments) = arguments()? else {
            return Ok(());
        };
        let cancel = Arc::new(AtomicBool::new(false));
        let signal = cancel.clone();
        ctrlc::set_handler(move || signal.store(true, Ordering::Relaxed))
            .map_err(|error| error.to_string())?;
        run(arguments, &cancel)
    })();
    if let Err(error) = result {
        eprintln!("ERROR: {error}");
        std::process::exit(1);
    }
}
