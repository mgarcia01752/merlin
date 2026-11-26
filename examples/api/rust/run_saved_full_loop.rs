/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent native-parity reference for the bundled saved downstream project.
use docsis_ofdm_fec::api::FullLoopControl;
use pnm_input::Workbench;
use serde_json::{json, Value};

const SAVED_PROJECT: &[u8] =
    include_bytes!("../../workbench/downstream/single/workbench.merlin.json");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut document: Value = serde_json::from_slice(SAVED_PROJECT)?;
    document["receiver"]["all_profiles"] = json!(false);
    let plan = Workbench::prepare_full_loop(&serde_json::to_vec(&document)?)?;
    let request = plan.request(0)?;
    let result = request.run(&FullLoopControl::new())?;
    println!(
        "{}",
        serde_json::to_string(&json!({
            "plan":plan.info(), "settings":request.settings(),
            "response_tolerance":request.response_tolerance().value(),
            "summary":result.report().summary, "status":result.report().status,
            "inputs":result.inputs().summary()?
        }))?
    );
    Ok(())
}
