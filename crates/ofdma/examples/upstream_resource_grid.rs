/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Separate-process reference for configured upstream resources, without running a waveform.
use docsis_ofdma::api::*;

const PROFILE_ID: u16 = 224;
const DATA_IUC: u8 = 9;
const PROFILE_RANDOMIZER_SEED: u32 = 1;
const SOURCE_PRBS15_SEED: u16 = 1;
const PREFIX_SAMPLES: usize = 96;
const ROLLOFF_SAMPLES: usize = 32;
const FRAME_SYMBOLS: usize = 6;
const BANDWIDTH_HZ: u32 = 20_000_000;
const DATA_LOADING: u8 = 4;
const PROFILE_MINISLOT_COUNT: usize = 4;
const ZERO_LOADED_MINISLOT_INDEX: usize = 1;
const VIEWPORT_BORDER_CARRIERS: usize = 1;
const GRANT_ID: u64 = 17;
const GRANT_EPOCH_SAMPLE: u64 = 4096;
const FIRST_FRAME_ID: u64 = 11;
const GRANT_FRAME_COUNT: usize = 2;
const SELECTED_FRAME_INDEX: usize = 1;
const ESTIMATOR_PHASE_LIMIT: f64 = 0.02;
const ESTIMATOR_POWER_FLOOR: f64 = 1e-12;
const LDPC_ITERATION_BUDGET: usize = 50;

/// Exports both admitted numerologies' original configured resource rectangles.
/// # Returns
/// Unit after printing owned snapshots; no source or waveform is executed.
/// # Errors
/// Propagates typed configuration/window/resource or JSON transport failures.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut snapshots = Vec::new();
    for (mode, first_carrier, pattern) in [(Numerology::Khz50, 100, 1), (Numerology::Khz25, 200, 8)]
    {
        let width = mode.carriers_per_minislot();
        let waveform = Waveform::new(
            mode,
            PREFIX_SAMPLES,
            ROLLOFF_SAMPLES,
            FRAME_SYMBOLS,
            BANDWIDTH_HZ,
        )?;
        let profile = Profile::new(
            PROFILE_ID,
            DATA_IUC,
            PROFILE_RANDOMIZER_SEED,
            (0..PROFILE_MINISLOT_COUNT)
                .map(|index| Minislot {
                    first_carrier: first_carrier + index * width,
                    loading: if index == ZERO_LOADED_MINISLOT_INDEX {
                        0
                    } else {
                        DATA_LOADING
                    },
                    pilot_pattern: if index == ZERO_LOADED_MINISLOT_INDEX {
                        0
                    } else {
                        pattern
                    },
                })
                .collect(),
        )?;
        let grant = SyntheticGrant {
            id: GRANT_ID,
            profile_id: PROFILE_ID,
            epoch_sample: GRANT_EPOCH_SAMPLE,
            first_frame: FIRST_FRAME_ID,
            frames: GRANT_FRAME_COUNT,
            minislots: vec![0, 1, 2],
        };
        let configuration = UsConfiguration::new(
            waveform,
            vec![profile],
            grant,
            vec![first_carrier - VIEWPORT_BORDER_CARRIERS],
        )?;
        let receiver = ReceiverConfig::new(
            Estimator::FrameFlatLs,
            ESTIMATOR_PHASE_LIMIT,
            ESTIMATOR_POWER_FLOOR,
            LDPC_ITERATION_BUDGET,
        )?;
        let mut simulation = Simulation::new(ExperimentOptions::default())?;
        simulation.configure(ExperimentConfig::new(
            configuration,
            SOURCE_PRBS15_SEED,
            receiver,
        )?)?;
        snapshots.push(simulation.resource_grid(ResourceGridRequest {
            frame_index: SELECTED_FRAME_INDEX,
            first_carrier: first_carrier - VIEWPORT_BORDER_CARRIERS,
            carrier_count: width * PROFILE_MINISLOT_COUNT + 2 * VIEWPORT_BORDER_CARRIERS,
            symbol_count: 0,
            ..Default::default()
        })?);
    }
    println!("{}", serde_json::to_string(&snapshots)?);
    Ok(())
}
