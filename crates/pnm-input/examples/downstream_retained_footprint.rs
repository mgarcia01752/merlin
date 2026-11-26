/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent Rust-process metadata for identical saved-project native inputs.
use docsis_ofdm_fec::api::{
    FullLoopControl, FullLoopObservationPolicy, FullLoopReceiverArchiveInput,
    FullLoopReceiverArchivePolicy,
};
use pnm_input::Workbench;
use serde_json::json;

/// Produces actual current-root quantities, never a future allocation estimate.
///
/// # Returns
/// Writes JSON for both tracked projects, observation modes and terminal states.
///
/// # Errors
/// Propagates existing project, simulation, observation and serialization errors.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let projects: [(&str, &[u8]); 2] = [
        (
            "single",
            include_bytes!("../../../examples/workbench/single/workbench.merlin.json"),
        ),
        (
            "multi-rxmer",
            include_bytes!("../../../examples/workbench/multi-rxmer/workbench.merlin.json"),
        ),
    ];
    let mut cases = Vec::new();
    for (name, bytes) in projects {
        for observed in [false, true] {
            for cancelled in [false, true] {
                let plan = Workbench::prepare_full_loop(bytes)?;
                let plan_metadata = plan.retained_footprint()?;
                let request = plan.request(0)?;
                let request = if observed {
                    request
                        .with_observations(FullLoopObservationPolicy::new(Default::default())?)?
                } else {
                    request
                };
                let request_metadata = request.retained_footprint()?;
                let control = FullLoopControl::new();
                if cancelled {
                    control.cancel();
                }
                // Match the native worker's immutable request clone before dispatch.
                let result = request.clone().run(&control)?;
                let archive = result.receiver_archive(FullLoopReceiverArchivePolicy::new(
                    FullLoopReceiverArchiveInput::default(),
                )?)?;
                cases.push(json!({
                    "project": name,
                    "observed": observed,
                    "cancelled": cancelled,
                    "plan": plan_metadata,
                    "request": request_metadata,
                    "result": result.retained_footprint()?,
                    "archive": archive.retained_footprint()?,
                    "archive_info": archive.info(),
                    "status": result.report().status,
                    "recovery": result.fec_recovery(),
                    "inputs": result.inputs().summary()?,
                }));
            }
        }
    }
    println!("{}", serde_json::to_string(&json!({"cases": cases}))?);
    Ok(())
}
