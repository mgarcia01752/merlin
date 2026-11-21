/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded upstream execution through the literal unified Rust namespace.
use merlin::upstream::{ExperimentOptions, Simulation};

/// Loads one source fixture while idle, then runs the independent upstream engine.
///
/// # Returns
/// Unit after printing actual terminal state and independently compared source bits.
///
/// # Errors
/// Propagates shared admission/execution errors or incomplete reference recovery.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut simulation = Simulation::new(ExperimentOptions::default())?;
    simulation.configure_json(include_bytes!(
        "../../../examples/cli/upstream/us-50khz.json"
    ))?;
    let inventory = simulation.preview()?;
    println!("Profiles discovered by Rust: {:?}", inventory.profile_ids);
    let report = simulation.run()?;
    println!("Status: {:?}", report.status);
    println!("Compared source bits: {}", report.compared_source_bits);
    println!("Bit errors: {}", report.bit_errors);
    if !report.reference_recovered {
        return Err("reference experiment did not fully recover source".into());
    }
    Ok(())
}
