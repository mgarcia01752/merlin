/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: public owners and fixture paths.
 */
//! Synthetic multi-RxMER and explicit unity-bank replay through the Rust engine.
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
    let mut config = ExperimentConfig::new(us.clone(), 1, receiver)?;
    let sources = vec![
        SyntheticRxmer::new(&us, 1, vec![Some(35.0); 2048])?,
        SyntheticRxmer::new(&us, 2, vec![Some(45.0); 2048])?,
    ];
    config.rxmer = Some(RxmerPlayback::new(
        &us,
        sources,
        RxmerPolicy::Interpolate {
            offsets: vec![0, 6],
        },
    )?);
    config.pre_eq_updates = Some(PreEqPlayback::new(
        &us,
        1,
        config.pre_eq.clone(),
        vec![PreEqEvent {
            id: 2,
            symbol: 6,
            operation: PreEqOperation::Snapshot(config.pre_eq.clone()),
        }],
        vec![],
    )?);
    simulation.configure(config)?;
    let report = simulation.run()?;
    if !report.reference_recovered {
        return Err("reference experiment did not fully recover source".into());
    }
    println!("{}", report.to_json()?);
    Ok(())
}
