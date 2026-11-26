/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Separate Rust-process reference for deterministic configured C/H/periodic diagnostics.
use docsis_ofdma::api::*;
use std::f64::consts::FRAC_PI_2;

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
const CORRECTIVE_BANK_GENERATION: u64 = 11;
const RAW_CORRECTIVE_IMAGINARY_GAIN: f64 = 2.0;
const PHYSICAL_DELAY_SAMPLES: usize = 2;
const SECOND_PHYSICAL_TAP_AMPLITUDE: f64 = 0.5;

/// Exports all three configured planes and full periodic inverses at both supported spacings.
/// # Returns
/// Unit after printing six owned snapshots; no source/channel stream or waveform executes.
/// # Errors
/// Propagates actual configuration, response/query and bounded-geometry serialization failures.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut snapshots = Vec::new();
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
        let mut raw_c = vec![
            Iq {
                re: 0.0,
                im: RAW_CORRECTIVE_IMAGINARY_GAIN
            };
            mode.fft_len()
        ];
        for &carrier in us.excluded() {
            raw_c[carrier] = Iq::default();
        }
        let c = PreEqBank::normalized(&us, raw_c, CORRECTIVE_BANK_GENERATION)?;
        let receiver = ReceiverConfig::new(
            Estimator::FrameFlatLs,
            ESTIMATOR_VARIANCE,
            ESTIMATOR_POWER_FLOOR,
            LDPC_ITERATION_BUDGET,
        )?;
        let mut configuration = ExperimentConfig::new(us, SOURCE_PRBS15_SEED, receiver)?;
        configuration.pre_eq = c;
        configuration.channel.taps = vec![
            Iq { re: 1.0, im: 0.0 },
            Iq {
                re: SECOND_PHYSICAL_TAP_AMPLITUDE,
                im: 0.0,
            },
        ];
        configuration.channel.delay_samples = PHYSICAL_DELAY_SAMPLES;
        configuration.channel.phase_radians = FRAC_PI_2;
        let mut simulation = Simulation::new(ExperimentOptions::default())?;
        simulation.configure(configuration)?;
        for source in [
            ConfiguredResponseSource::CmPreEq,
            ConfiguredResponseSource::PhysicalChannel,
            ConfiguredResponseSource::Composite,
        ] {
            snapshots.push(simulation.configured_response(ConfiguredResponseRequest {
                source,
                include_impulse: true,
                ..Default::default()
            })?);
        }
    }
    println!("{}", serde_json::to_string(&snapshots)?);
    Ok(())
}
