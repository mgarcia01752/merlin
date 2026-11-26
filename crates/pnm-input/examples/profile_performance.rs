/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent public-adapter process for original-grid receiver performance parity.
use docsis_ofdm_fec::api::FullLoopControl;
use pnm_input::project::Project;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project = Project::from_bytes(include_bytes!(
        "../../../examples/workbench/downstream/single/workbench.merlin.json"
    ))?;
    let plan = project.downstream_plan()?;
    let result = plan.request(0)?.run(&FullLoopControl::new())?;
    println!("{}", serde_json::to_string(&result.profile_performance()?)?);
    Ok(())
}
