/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! File: examples/api/rust/run_multi_rxmer_cmce_workbench.rs
//! Run saved WB settings through the native API, independently of Qt.
//! The bundled multi-RxMER WB uses CMCE (supplied CM channel estimation).
//! Other input workbenches retain their saved receiver mode.
use pnm_input::Workbench;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Loading verifies embedded captures and preflights all saved profiles. No run yet.
    let path = std::env::args()
        .nth(1)
        .ok_or("Supply a workbench.merlin.json path")?;
    let mut workbench = Workbench::from_file(path)?;
    // Only this explicit call executes the shared Rust engine. Batches own results.
    let batches = workbench.run()?;
    let values = batches
        .iter()
        .map(|batch| {
            serde_json::from_str::<serde_json::Value>(&batch.to_json()?)
                .map_err(Box::<dyn std::error::Error>::from)
        })
        .collect::<Result<Vec<_>, _>>()?;
    println!("{}", serde_json::to_string_pretty(&values)?);
    Ok(())
}
