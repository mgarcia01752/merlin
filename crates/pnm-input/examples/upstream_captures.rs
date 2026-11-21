/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: public owners and fixture paths.
 */
//! Actual measured-adapter consumer; fixtures are authored, not modem recordings.
use docsis_ofdma::api::{ExperimentOptions, ReportJson, Simulation};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut simulation = Simulation::new(ExperimentOptions::default())?;
    let bytes: &[u8] = if std::env::args().nth(1).as_deref() == Some("25") {
        include_bytes!("../../../examples/measurements/upstream/measured/capture-driven-25khz.json")
    } else {
        include_bytes!("../../../examples/measurements/upstream/measured/capture-driven.json")
    };
    simulation
        .configure_measured(pnm_input::upstream::MeasuredExperiment::from_json(bytes)?.admit()?)?;
    println!("{}", simulation.run()?.to_json()?);
    Ok(())
}
