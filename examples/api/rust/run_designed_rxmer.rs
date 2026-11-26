/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! File: examples/api/rust/run_designed_rxmer.rs
//! Run the custom synthetic designed RxMER against every compatible captured profile.
//! This is designer output, not a modem measurement. The adjacent .bin.merlin.json
//! preserves its recipe/provenance; loading the binary does not reapply the recipe.
use docsis_ofdm_fec::api::*;
use pnm_input::{read_capture, CaptureLoader};
use std::{path::Path, time::Instant};

const RXMER_PATH_FROM_MANIFEST: &str =
    "../../examples/measurements/designed-rxmer/merlin_rxmer_designed.bin";
const PROFILE_PATH_FROM_MANIFEST: &str =
    "../../examples/workbench/downstream/single/modulation-profile.bin";
const OBSERVATION_DURATION_MS: u64 = 1;
const PROFILE_ASSOCIATION_TOLERANCE_SECONDS: u32 = 90_000;
const MAX_RUNTIME_MS: u64 = 300_000;
const RECEIVER_MODE: ReceiverMode = ReceiverMode::CmChannelEstimated;
const NOISE_SEED: u64 = 41;
const PRBS_SEED: u16 = 0x7fff;
const MAX_LDPC_ITERATIONS: usize = 50;
const CYCLIC_PREFIX_SAMPLES: u16 = 512;
const ROLL_OFF_SAMPLES: u16 = 128;
const INTERLEAVER_DEPTH: u8 = 16;
const BER_SIGNIFICANT_DIGITS_AFTER_DECIMAL: usize = 3;

/// Runs all captured profiles against an unchanged designed RxMER original.
///
/// # Returns
/// Unit after printing actual status, FEC, BER and execution time.
/// # Errors
/// Propagates file/API errors; partial results retain their status and optional metrics.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Locate and decode originals without depending on the current directory.
    // The original binary and its sidecar remain unchanged; no simulation yet.
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let observation =
        CaptureLoader::rxmer(&read_capture(manifest.join(RXMER_PATH_FROM_MANIFEST))?)?;
    let profile_input = read_capture(manifest.join(PROFILE_PATH_FROM_MANIFEST))?;

    // 2. Discover the compatible capture's complete catalog through Rust. These
    // exact grids match; the multi-rxmer-2 profile belongs to a different channel.
    let profile_ids = CaptureLoader::profile_ids(&profile_input)?;
    let profiles = profile_ids
        .iter()
        .map(|id| CaptureLoader::profile(&profile_input, *id))
        .collect::<Result<Vec<_>, _>>()?;
    println!("Synthetic designed RxMER; not a modem measurement.");
    println!("Loaded custom RxMER; simulation has not started.");
    println!(
        "Profiles: {:?}",
        profile_ids.iter().map(|id| id.value()).collect::<Vec<_>>()
    );
    println!("Receiver: {RECEIVER_MODE:?}; physical channel: unity");
    let started = Instant::now();

    // 3. Create a separate controller per profile. Match the Python timing,
    // seeds and limits; keep capture epochs unchanged with an explicit tolerance.
    for profile in profiles {
        let profile_id = profile.profile_id();
        let ocd = Ocd::from_profile(
            ProtocolScope::Docsis31,
            &profile,
            OcdOverrides {
                cyclic_prefix: Some(CyclicPrefixSamples::new(CYCLIC_PREFIX_SAMPLES)?),
                roll_off: Some(RollOffSamples::new(ROLL_OFF_SAMPLES)?),
                interleaver_depth: Some(InterleaverDepth::new(INTERLEAVER_DEPTH)?),
                ..Default::default()
            },
        )?;
        let mut sim = Simulation::with_ocd(ocd);
        sim.set_profile(profile)?;
        let original_id = sim.add_rxmer(observation.clone())?;

        // 4. Select the designed original; its tilt/ripple/notch are already in
        // the encoded values. Unity adds no complex channel response, and its
        // supplied correction leaves the RxMER-driven residual noise in place.
        sim.set_options(EvaluationOptions {
            rxmer_selection: RxMerSelection::Individual(original_id),
            channel_selection: Some(ChannelSelection::Unity),
            receiver_mode: RECEIVER_MODE,
            observation: ObservationDuration::new(OBSERVATION_DURATION_MS)?,
            profile_tolerance: AssociationTolerance::new(PROFILE_ASSOCIATION_TOLERANCE_SECONDS)?,
            noise_seed: NoiseSeed::new(NOISE_SEED)?,
            prbs_seed: PrbsSeed::new(PRBS_SEED)?,
            limits: RunLimits {
                runtime: RuntimeLimit::new(MAX_RUNTIME_MS)?,
                iterations: IterationLimit::new(MAX_LDPC_ITERATIONS)?,
                ..Default::default()
            },
            ..Default::default()
        })?;

        // 5. Only run() starts the shared engine. Inspect real owned reports;
        // completion does not imply successful FEC and missing values are not zero.
        let batch = sim.run()?;
        for entry in batch.iter() {
            println!("Profile {}: {:?}", profile_id.value(), entry.status());
            if let Some(fec) = entry.report().and_then(|report| report.fec()) {
                println!("Total: {}", fec.total_codewords().value());
                println!("Corrected: {}", fec.corrected_codewords().value());
                println!("Uncorrectable: {}", fec.uncorrectable_codewords().value());
                if let Some(ber) = entry
                    .report()
                    .and_then(|report| report.profile())
                    .and_then(|profile| profile.post_fec_ber())
                {
                    println!(
                        "BER: {:.*e}",
                        BER_SIGNIFICANT_DIGITS_AFTER_DECIMAL,
                        ber.value()
                    );
                } else {
                    println!("BER: unavailable");
                }
            } else {
                println!("Observation-window FEC report unavailable.");
            }
        }
    }
    println!(
        "Total experiment wall time: {:.3} seconds",
        started.elapsed().as_secs_f64()
    );
    Ok(())
}
