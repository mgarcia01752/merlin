/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * File: examples/api/rust/run_single_capture_cm_chan_est.rs
 */
//! Apply one CM response to the channel and reuse it for receiver correction.
//! This is supplied-reference correction, not blind pilot or MCE estimation.
//! RxMER still supplies residual noise; correction does not remove it.
use docsis_ofdm_fec::api::*;
use pnm_input::{read_capture, CaptureLoader};

const FIXTURE_DIRECTORY: &str = "../../examples/workbench/downstream/single";
const PROFILE_FILENAME: &str = "modulation-profile.bin";
const RXMER_FILENAME: &str = "rxmer.bin";
const CHANNEL_ESTIMATE_FILENAME: &str = "ds-channel-estimate.bin";
const OBSERVATION_DURATION_MS: u64 = 1;
const PROFILE_ASSOCIATION_TOLERANCE_SECONDS: u32 = 90_000;
const MAX_RUNTIME_MS: u64 = 300_000;

/// Runs a bounded captured-channel experiment using supplied receiver correction.
/// # Returns
/// Unit after printing the owned observation-window report as JSON.
/// # Errors
/// Propagates loading/preflight errors and rejects incomplete fixture outcomes.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Locate shared fixtures relative to the registered example's crate manifest.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_DIRECTORY);

    // Create an idle session, retain the profile collection and select Profile A.
    let mut sim = Simulation::new();
    sim.load_profiles(CaptureLoader::profiles(&read_capture(
        root.join(PROFILE_FILENAME),
    )?)?)?;
    sim.select_profile(ProfileId::PROFILE_A)?;

    // Decode each measurement once, preserving its original bytes and provenance.
    let rxmer = sim.add_rxmer(CaptureLoader::rxmer(&read_capture(
        root.join(RXMER_FILENAME),
    )?)?)?;
    let channel = sim.add_channel_estimate(CaptureLoader::channel_estimate(&read_capture(
        root.join(CHANNEL_ESTIMATE_FILENAME),
    )?)?)?;

    // Configure physical channel impairment separately from receiver correction.
    let options =
        EvaluationOptions::default().with_rxmer_selection(RxMerSelection::Individual(rxmer));

    // Apply the captured complex response to the simulated channel.
    let options = options.with_channel_selection(ChannelSelection::Explicit(channel));

    // Correct with the same supplied response; no MCE training is requested.
    let mut options = options
        .with_receiver_mode(ReceiverMode::CmChannelEstimated)
        .with_profile_tolerance(AssociationTolerance::new(
            PROFILE_ASSOCIATION_TOLERANCE_SECONDS,
        )?);

    // Source observation and host execution limits are independent durations.
    options.observation = ObservationDuration::new(OBSERVATION_DURATION_MS)?;
    options.limits.runtime = RuntimeLimit::new(MAX_RUNTIME_MS)?;
    sim.set_options(options)?;

    // Only run starts the engine; inspect actual FEC results before reporting success.
    for entry in sim.run()?.iter() {
        let report = entry.report().ok_or("Missing channel-estimate report")?;
        if entry.status() != RunStatus::Complete || report.fec().is_none() {
            return Err("Channel-estimate example did not produce complete FEC results".into());
        }

        // The report distinguishes observation-window FEC from whole-run diagnostics.
        println!("{}", report.to_json()?);
    }
    Ok(())
}
