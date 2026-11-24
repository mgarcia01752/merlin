/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent Rust-process metadata references for installed native Python.
use docsis_ofdma::api::*;

const GENERIC_50KHZ: &[u8] = include_bytes!("../../../examples/cli/upstream/us-50khz.json");
const GENERIC_25KHZ: &[u8] = include_bytes!("../../../examples/cli/upstream/us-25khz.json");
// Small observation fixtures exercise both private estimate and soft stores.
const ESTIMATE_FRAMES: usize = 2;
const ESTIMATE_SCALARS: usize = 16_384;
const SOFT_CELLS: usize = 8;
const SOFT_WORDS: usize = 2;
const SOFT_WORD_LLRS: usize = 32_768;

/// Emits passive current-root metadata for both modes and complete/cancelled runs.
///
/// # Returns
/// Unit after printing Rust-owned quantities; target layout is intentionally local.
///
/// # Errors
/// Propagates public admission, execution, footprint and serialization errors.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let idle = Simulation::new(Default::default())?.retained_footprint()?;
    let mut cases = Vec::new();
    for bytes in [GENERIC_50KHZ, GENERIC_25KHZ] {
        for observed in [false, true] {
            for cancelled in [false, true] {
                let mut simulation = Simulation::new(Default::default())?;
                simulation.configure_json(bytes)?;
                if observed {
                    simulation.set_receiver_observation_policy(
                        ReceiverObservationPolicy::new(ESTIMATE_FRAMES, ESTIMATE_SCALARS)?
                            .with_soft_metrics(SoftObservationPolicy::new(
                                SOFT_CELLS,
                                SOFT_WORDS,
                                SOFT_WORD_LLRS,
                            )?),
                    )?;
                }
                let session = simulation.retained_footprint()?;
                let control = RunControl::new();
                if cancelled {
                    control.cancel();
                }
                let report = simulation.run_with_control(&control)?;
                cases.push(serde_json::json!({
                    "observed": observed,
                    "cancelled": cancelled,
                    "session": session,
                    "report": report.retained_footprint()?,
                    "status": report.status,
                    "recovered": report.reference_recovered,
                }));
            }
        }
    }
    println!("{}", serde_json::json!({"idle": idle, "cases": cases}));
    Ok(())
}
