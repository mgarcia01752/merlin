/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 * File: examples/api/rust/run_single_capture.rs
 */
//! Developer walkthrough: load captures, configure MERLIN, run, inspect results.
//!
//! Run with `scripts/cargo-local.sh run -p pnm-input --example run_single_capture --locked`.
//! This uses the same Rust engine as the native Python example, without Qt or
//! Python dependencies. Loading and configuration never execute simulation;
//! only the explicit `run()` call below starts a batch.
use docsis_ofdm_fec::api::*;
use pnm_input::{read_capture, CaptureLoader};

// Example configuration: adapt these caller choices for your own captures.
// Cargo registers this file as a pnm-input example, so its manifest directory
// is crates/pnm-input even though the source lives under examples/api/rust.
const FIXTURE_DIRECTORY_FROM_MANIFEST: &str = "../../examples/workbench/downstream/single";
const PROFILE_FILENAME: &str = "modulation-profile.bin";
const RXMER_FILENAME: &str = "rxmer.bin";
const CHANNEL_ESTIMATE_FILENAME: &str = "ds-channel-estimate.bin";

// Source observation excludes acquisition, completion and receiver drain.
const OBSERVATION_DURATION_MS: u64 = 1;

// Permit this fixture's profile and RxMER to have different original epochs.
const PROFILE_ASSOCIATION_TOLERANCE_SECONDS: u32 = 90_000;

// Host execution has its own per-experiment bound, independent of source time.
const MAX_RUNTIME_MS: u64 = 300_000;

/// Runs one fixture-backed experiment and displays actual Rust-engine reports.
///
/// # Returns
/// Unit after displaying each entry's status and available serialized report.
/// Input files remain unchanged; loading does not start background simulation.
///
/// # Errors
/// Propagates structured input/configuration/preflight errors through `?`.
/// Started partial or failed experiments retain their status and available report.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Locate shared fixtures independently of the caller's working directory.
    // read_capture performs bounded read-only I/O; it never rewrites the input.
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_DIRECTORY_FROM_MANIFEST);

    // 2. Import the Downstream Profile Descriptor (DPD), selecting PROFILE_A.
    // A capture may contain multiple profiles, so choose one explicitly.
    // CaptureLoader detects binary/JSON by content and calls the native decoder.
    // The immutable DPD retains original bytes, assignments and provenance.
    let mut sim = Simulation::new();
    sim.load_profiles(CaptureLoader::profiles(&read_capture(
        root.join(PROFILE_FILENAME),
    )?)?)?;

    // 3. Select a retained profile without rereading its original capture.
    sim.select_profile(ProfileId::PROFILE_A)?;

    // 4. The controller owns profiles and resolved OCD; no setter is needed.
    // Bandwidth derives uniquely; omitted timing retains documented defaults.

    // 5. Add measurements independently, preserving their original timestamps.
    // RxMER conditions Rust's residual-error model by subcarrier. The channel
    // estimate supplies the complex response H. Matching pairs/equal counts are
    // not required. Each addition returns an opaque ID scoped to this controller;
    // keep these IDs to select the desired original observations later.
    let first = sim.add_rxmer(CaptureLoader::rxmer(&read_capture(
        root.join(RXMER_FILENAME),
    )?)?)?;
    let channel = sim.add_channel_estimate(CaptureLoader::channel_estimate(&read_capture(
        root.join(CHANNEL_ESTIMATE_FILENAME),
    )?)?)?;

    // 6. Select one RxMER and explicitly reuse the selected channel response.
    // Other RxMER modes are Each, AverageAll and InterpolateAt. Channel policies
    // also include explicit unity and nearest-time selection within tolerance.
    // The profile association tolerance is separate from nearest-channel time
    // matching; this fixture-specific allowance never rewrites capture epochs.
    let mut options = EvaluationOptions::default()
        .with_rxmer_selection(RxMerSelection::Individual(first))
        .with_channel_selection(ChannelSelection::Explicit(channel))
        .with_profile_tolerance(AssociationTolerance::new(
            PROFILE_ASSOCIATION_TOLERANCE_SECONDS,
        )?);

    // Source observation and host runtime use milliseconds but measure different
    // things. A host limit can return partial results before observation ends.
    options.observation = ObservationDuration::new(OBSERVATION_DURATION_MS)?;
    options.limits.runtime = RuntimeLimit::new(MAX_RUNTIME_MS)?;
    sim.set_options(options)?;

    // 7. Execute explicitly. Rust preflights the whole batch, then dispatches the
    // existing TX/channel/RX/FEC engine. The returned batch owns its reports;
    // subsequent controller changes do not invalidate those results.
    for entry in sim.run()?.iter() {
        if entry.status() != RunStatus::Complete {
            return Err("Single-capture example did not complete".into());
        }

        // 8. Inspect status before interpreting optional scientific results.
        // Complete means execution finished, not that every codeword decoded.
        // NotStarted entries have no report; early partial runs can lack window
        // FEC/BER. Missing denominators remain absent rather than fabricated zero.
        println!("{:?}", entry.status());
        if let Some(report) = entry.report() {
            // JSON includes actual FEC/profile results, status, metric scopes,
            // resolved settings/assumptions and original source provenance.
            // Window metrics and whole-run diagnostics are separately labeled.
            // Printing does not write a result file or modify any capture.
            println!("{}", report.to_json()?);
        }
    }
    Ok(())
}
