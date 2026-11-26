/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual Rust-process soft references from both existing generic configuration documents.
use docsis_ofdma::api::*;

const RETAINED_CELL_BUDGET: usize = 1024;
const RETAINED_WORD_BUDGET: usize = 16;
const WORD_INPUT_BUDGET: usize = 262_144;
const GENERIC_50KHZ: &[u8] = include_bytes!("../../../examples/cli/upstream/us-50khz.json");
const GENERIC_25KHZ: &[u8] = include_bytes!("../../../examples/cli/upstream/us-25khz.json");

/// Exports actual computed-cell and completed LDPC-input observations at both spacings.
/// # Returns
/// Unit after printing original summary/page metadata and every retained input-word vector.
/// # Errors
/// Propagates configuration, execution, bounded-query and serialization failures without replay.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut references = Vec::new();
    for bytes in [GENERIC_50KHZ, GENERIC_25KHZ] {
        let mut simulation = Simulation::new(Default::default())?;
        simulation.configure_json(bytes)?;
        simulation.set_receiver_observation_policy(
            ReceiverObservationPolicy::default().with_soft_metrics(SoftObservationPolicy::new(
                RETAINED_CELL_BUDGET,
                RETAINED_WORD_BUDGET,
                WORD_INPUT_BUDGET,
            )?),
        )?;
        let report = simulation.run()?;
        let summary = report.soft_observations();
        let words = summary
            .words
            .iter()
            .map(|word| {
                report.soft_word(SoftWordRequest {
                    retained_index: word.retained_index,
                    ..Default::default()
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        references.push(serde_json::json!({"summary":summary, "cells":report.soft_cells(Default::default())?, "words":words}));
    }
    println!("{}", serde_json::to_string(&references)?);
    Ok(())
}
