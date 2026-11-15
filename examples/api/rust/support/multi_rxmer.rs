/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! File: examples/api/rust/support/multi_rxmer.rs
//! Shared walkthrough for the two collection receiver examples.
//! Settings intentionally match the sibling Python collection walkthroughs.
use docsis_ofdm_fec::api::*;
use pnm_input::{read_capture, CaptureLoader};
use std::{path::Path, time::Instant};

// Caller choices for the bundled fixture, not values inferred from filenames.
const FIXTURE_DIRECTORY_FROM_MANIFEST: &str = "../../examples/workbench/multi-rxmer-2";
const PROFILE_FILENAME: &str = "ds_ofdm_modulation_profile_aabbccddeeff_193_1764824373.bin";
const RXMER_FILENAME_PREFIX: &str = "ds_ofdm_rxmer_per_subcar_";
const OCCUPIED_BANDWIDTH_HZ: u32 = 192_000_000;
const OBSERVATION_DURATION_MS: u64 = 1;
const PROFILE_ASSOCIATION_TOLERANCE_SECONDS: u32 = 7_200;
const MAX_RUNTIME_MS: u64 = 300_000;
const NOISE_SEED: u64 = 41;
const PRBS_SEED: u16 = 0x7fff;
const MAX_LDPC_ITERATIONS: usize = 50;
const CYCLIC_PREFIX_SAMPLES: u16 = 512;
const ROLL_OFF_SAMPLES: u16 = 128;
const INTERLEAVER_DEPTH: u8 = 16;
const BER_SIGNIFICANT_DIGITS_AFTER_DECIMAL: usize = 3;

/// Loads the collection, explicitly runs each captured profile and prints results.
///
/// # Arguments
/// * `receiver_mode` - MCE or supplied-channel correction, independent of physical H.
/// # Returns
/// Unit after displaying owned engine results; input files remain unchanged.
/// # Errors
/// Propagates file, decoding, validation and execution errors. Partial/failed
/// runs retain their actual status and available reports rather than invented counters.
pub fn run(receiver_mode: ReceiverMode) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Resolve fixture paths independently of the caller's working directory.
    // The Cargo manifest belongs to pnm-input, not to this example directory.
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_DIRECTORY_FROM_MANIFEST);

    // 2. Discover real captured profile IDs using the same decoder as Python.
    // Do not assume profiles are consecutive or hardcode the current 0/3/4 catalog.
    // Decode once per selected profile; all loading is read-only and runs no simulation.
    let profile_input = read_capture(root.join(PROFILE_FILENAME))?;
    let profile_ids = CaptureLoader::profile_ids(&profile_input)?;
    let profiles = profile_ids
        .iter()
        .map(|id| CaptureLoader::profile(&profile_input, *id))
        .collect::<Result<Vec<_>, _>>()?;

    // 3. Sort filenames only for reproducible insertion order. Capture timestamps,
    // frequency grids, missing readings and original bytes come from the decoder.
    // Collect every matching original RxMER; never borrow another channel's estimate.
    let mut paths = std::fs::read_dir(&root)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.retain(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with(RXMER_FILENAME_PREFIX) && name.ends_with(".bin"))
    });
    paths.sort();
    if paths.is_empty() {
        return Err(format!("No RxMER captures in {}", root.display()).into());
    }
    let observations = paths
        .iter()
        .map(|path| CaptureLoader::rxmer(&read_capture(path)?))
        .collect::<Result<Vec<_>, _>>()?;
    println!(
        "Loaded {} RxMER captures; simulation has not started.",
        observations.len()
    );

    // 4. AverageAll delegates residual-power averaging and exact-grid validation
    // to the existing Rust API. Unity selects the physical response; receiver_mode
    // independently selects supplied correction or received-pilot/noise estimation.
    // Source observation excludes acquisition/completion/drain; host runtime is a
    // separate bound. The timestamp tolerance never rewrites original captures.
    let options = EvaluationOptions {
        rxmer_selection: RxMerSelection::AverageAll,
        channel_selection: Some(ChannelSelection::Unity),
        receiver_mode,
        noise_seed: NoiseSeed::new(NOISE_SEED)?,
        prbs_seed: PrbsSeed::new(PRBS_SEED)?,
        observation: ObservationDuration::new(OBSERVATION_DURATION_MS)?,
        profile_tolerance: AssociationTolerance::new(PROFILE_ASSOCIATION_TOLERANCE_SECONDS)?,
        limits: RunLimits {
            runtime: RuntimeLimit::new(MAX_RUNTIME_MS)?,
            iterations: IterationLimit::new(MAX_LDPC_ITERATIONS)?,
            ..Default::default()
        },
        ..Default::default()
    };
    println!("Profiles: {:?}; observation: {OBSERVATION_DURATION_MS} ms per profile; RxMER: AverageAll; channel: unity; receiver: {receiver_mode:?}",
        profile_ids.iter().map(|id| id.value()).collect::<Vec<_>>());
    let total_started = Instant::now();

    // 5. Each profile gets its own controller and identical initial PRBS/noise
    // seeds. Derive grid/protected roles from that profile and explicitly match
    // the saved fixture's OCD timing. Clones retain immutable original captures.
    for profile in profiles {
        let profile_id = profile.profile_id();
        let ocd = Ocd::from_profile(
            ProtocolScope::Docsis31,
            &profile,
            OcdOverrides {
                bandwidth: Some(BandwidthHz::new(OCCUPIED_BANDWIDTH_HZ)?),
                cyclic_prefix: Some(CyclicPrefixSamples::new(CYCLIC_PREFIX_SAMPLES)?),
                roll_off: Some(RollOffSamples::new(ROLL_OFF_SAMPLES)?),
                interleaver_depth: Some(InterleaverDepth::new(INTERLEAVER_DEPTH)?),
            },
        )?;
        let mut sim = Merlin::new(ocd);
        sim.set_profile(profile)?;
        for observation in &observations {
            sim.add_rxmer(observation.clone())?;
        }
        sim.set_options(options.clone())?;

        // 6. Only run() starts the shared TX/channel/RX/FEC engine. The batch
        // owns its results independently of the controller and original captures.
        println!("Profile {}: starting", profile_id.value());
        let started = Instant::now();
        let batch = sim.run()?;
        let elapsed = started.elapsed();
        for entry in batch.iter() {
            println!("Profile {}: {:?}", profile_id.value(), entry.status());
            // 7. Use observation-window decoder outcomes and actual source BER.
            // Complete does not guarantee correct decoding. Unavailable metrics
            // remain unavailable; a partial run may have no complete FEC window.
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
        println!("Profile wall time: {:.3} seconds", elapsed.as_secs_f64());
    }
    println!(
        "Total experiment wall time: {:.3} seconds",
        total_started.elapsed().as_secs_f64()
    );
    Ok(())
}
