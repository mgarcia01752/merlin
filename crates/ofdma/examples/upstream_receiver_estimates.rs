/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Separate Rust-process references using the existing two generic configuration documents.
use docsis_ofdma::api::*;

const RETAINED_FRAME_BUDGET: usize = 3;
const RETENTION_SCALAR_BUDGET: usize = 262_144;
const GENERIC_50KHZ: &[u8] = include_bytes!("../../../examples/cli/upstream/us-50khz.json");
const GENERIC_25KHZ: &[u8] = include_bytes!("../../../examples/cli/upstream/us-25khz.json");

/// Exports actual retained receiver estimates from both generic closed loops.
/// # Returns
/// Unit after printing owned inventory and complete optional physical-frequency vectors.
/// # Errors
/// Propagates configuration, execution, query and serialization failures without fabricated data.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut references = Vec::new();
    for bytes in [GENERIC_50KHZ, GENERIC_25KHZ] {
        let mut simulation = Simulation::new(Default::default())?;
        simulation.configure_json(bytes)?;
        simulation.set_receiver_observation_policy(ReceiverObservationPolicy::new(
            RETAINED_FRAME_BUDGET,
            RETENTION_SCALAR_BUDGET,
        )?)?;
        let report = simulation.run()?;
        let summary = report.receiver_observations();
        let estimates = summary
            .frames
            .iter()
            .map(|frame| {
                report.receiver_estimate(ReceiverEstimateRequest {
                    retained_index: frame.retained_index,
                    ..Default::default()
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        references.push(serde_json::json!({"summary": summary, "estimates": estimates}));
    }
    println!("{}", serde_json::to_string(&references)?);
    Ok(())
}
