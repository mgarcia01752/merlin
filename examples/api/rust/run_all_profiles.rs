/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * File: examples/api/rust/run_all_profiles.rs
 */
//! Load a profile collection once, then explicitly run every retained profile.
use docsis_ofdm_fec::api::*;
use pnm_input::{read_capture, CaptureLoader};
const FIXTURE_DIRECTORY: &str = "../../examples/workbench/single";
const OBSERVATION_DURATION_MS: u64 = 1;
const PROFILE_TOLERANCE_SECONDS: u32 = 90_000;
const MAX_RUNTIME_MS: u64 = 300_000;

/// Run all retained profiles through the existing Rust simulation engine.
/// # Errors
/// Propagates capture, configuration and simulation preflight errors.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_DIRECTORY);

    // Loading retains the complete inventory without starting simulation.
    let mut sim = Simulation::new();
    sim.load_profiles(CaptureLoader::profiles(&read_capture(
        root.join("modulation-profile.bin"),
    )?)?)?;
    println!("Loaded profiles: {:?}", sim.profile_ids());
    sim.add_rxmer(CaptureLoader::rxmer(&read_capture(
        root.join("rxmer.bin"),
    )?)?)?;
    let mut options = EvaluationOptions::default()
        .with_channel_selection(ChannelSelection::Unity)
        .with_profile_tolerance(AssociationTolerance::new(PROFILE_TOLERANCE_SECONDS)?);
    options.observation = ObservationDuration::new(OBSERVATION_DURATION_MS)?;
    options.limits.runtime = RuntimeLimit::new(MAX_RUNTIME_MS)?;
    sim.set_options(options)?;

    // Returned entries identify their profile, including not-started requests.
    let batch = sim.run_all_profiles()?;
    if batch
        .iter()
        .any(|entry| entry.status() != RunStatus::Complete)
    {
        return Err("All-profile example did not complete".into());
    }
    println!("{}", batch.to_json()?);
    Ok(())
}
