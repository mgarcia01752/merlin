/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: public owners and fixture paths.
 */
//! Run the canonical generic JSON fixture through the public Rust API.
use docsis_ofdma::api::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes: &[u8] = if std::env::args().nth(1).as_deref() == Some("synthetic") {
        include_bytes!("../../../examples/cli/upstream/us-synthetic-playback.json")
    } else {
        include_bytes!("../../../examples/cli/upstream/us-50khz.json")
    };
    let document = ConfigurationDocument::from_json(bytes)?;
    let mut session = Simulation::new(ExperimentOptions::default())?;
    session.configure_document(document)?;
    let report = session.run()?;
    if !report.reference_recovered {
        return Err("configuration experiment did not recover source".into());
    }
    println!("{}", report.to_json()?);
    Ok(())
}
