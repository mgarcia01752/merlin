/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: public owners and fixture paths.
 */
//! Standalone high-level Rust PRBS→CM-US-TX→unity→CMTS-US-RX experiment.
use docsis_ofdma::api::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let waveform = Waveform::new(Numerology::Khz50, 192, 64, 6, 20_000_000)?;
    let profile = Profile::new(
        224,
        5,
        0x7f_ffff,
        (0..8)
            .map(|index| Minislot {
                first_carrier: 128 + 8 * index,
                loading: 4,
                pilot_pattern: 1,
            })
            .collect(),
    )?;
    let grant = SyntheticGrant {
        id: 1,
        profile_id: 224,
        epoch_sample: 0,
        first_frame: 0,
        frames: 2,
        minislots: (0..8).collect(),
    };
    let us = UsConfiguration::new(waveform, vec![profile], grant, vec![0, 1])?;
    let receiver = ReceiverConfig::new(Estimator::FrameFlatLs, 0.02, 1e-12, 50)?;
    let mut simulation = Simulation::new(ExperimentOptions::default())?;
    simulation.configure(ExperimentConfig::new(us, 1, receiver)?)?;
    let report = simulation.run()?;
    if !report.reference_recovered {
        return Err("reference experiment did not fully recover source".into());
    }
    println!("{}", report.to_json()?);
    Ok(())
}
