/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Direct Rust-process reference for passive actual FEC/verification views.
use docsis_ofdma::api::*;

const PREFIX_SAMPLES: usize = 192;
const ROLLOFF_SAMPLES: usize = 64;
const FRAME_SYMBOLS: usize = 6;
const BANDWIDTH_HZ: u32 = 20_000_000;
const PROFILE_ID: u16 = 224;
const DATA_IUC: u8 = 5;
const PROFILE_RANDOMIZER_SEED: u32 = 0x7f_ffff;
const SOURCE_PRBS15_SEED: u16 = 1;
const FIRST_ACTIVE_CARRIER: usize = 128;
const ACTIVE_MINISLOTS: usize = 8;
const DATA_LOADING: u8 = 4;
const FIRST_50KHZ_PILOT_PATTERN: u8 = 1;
const FIRST_25KHZ_PILOT_PATTERN: u8 = 8;
const GRANT_ID: u64 = 7;
const GRANT_EPOCH_SAMPLE: u64 = 73;
const FIRST_FRAME_ID: u64 = 3;
const GRANT_FRAMES: usize = 2;
const ESTIMATOR_VARIANCE: f64 = 0.02;
const ESTIMATOR_POWER_FLOOR: f64 = 1e-12;
const LDPC_ITERATION_BUDGET: usize = 50;

/// Executes both admitted numerologies and exports actual recovery projections.
/// # Returns
/// Unit after printing scalar views and the original first completed-word page.
/// The typed high-level API owns every scientific result and copy budget.
/// # Errors
/// Propagates actual configuration, run, projection or JSON serialization failures.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut outputs = Vec::new();
    for (mode, pilot_pattern) in [
        (Numerology::Khz50, FIRST_50KHZ_PILOT_PATTERN),
        (Numerology::Khz25, FIRST_25KHZ_PILOT_PATTERN),
    ] {
        let width = mode.carriers_per_minislot();
        let profile = Profile::new(
            PROFILE_ID,
            DATA_IUC,
            PROFILE_RANDOMIZER_SEED,
            (0..ACTIVE_MINISLOTS)
                .map(|index| Minislot {
                    first_carrier: FIRST_ACTIVE_CARRIER + index * width,
                    loading: DATA_LOADING,
                    pilot_pattern,
                })
                .collect(),
        )?;
        let grant = SyntheticGrant {
            id: GRANT_ID,
            profile_id: PROFILE_ID,
            epoch_sample: GRANT_EPOCH_SAMPLE,
            first_frame: FIRST_FRAME_ID,
            frames: GRANT_FRAMES,
            minislots: (0..ACTIVE_MINISLOTS).collect(),
        };
        let waveform = Waveform::new(
            mode,
            PREFIX_SAMPLES,
            ROLLOFF_SAMPLES,
            FRAME_SYMBOLS,
            BANDWIDTH_HZ,
        )?;
        let us = UsConfiguration::new(waveform, vec![profile], grant, vec![0, 1])?;
        let receiver = ReceiverConfig::new(
            Estimator::FrameFlatLs,
            ESTIMATOR_VARIANCE,
            ESTIMATOR_POWER_FLOOR,
            LDPC_ITERATION_BUDGET,
        )?;
        let mut simulation = Simulation::new(ExperimentOptions::default())?;
        simulation.configure(ExperimentConfig::new(us, SOURCE_PRBS15_SEED, receiver)?)?;
        let report = simulation.run()?;
        outputs.push(serde_json::json!({
            "verification": report.verification()?,
            "fec_summary": report.fec_summary()?,
            "fec_words": report.fec_words(FecWordRequest::default())?,
        }));
    }
    println!("{}", serde_json::to_string(&outputs)?);
    Ok(())
}
