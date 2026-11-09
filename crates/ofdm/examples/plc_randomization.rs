/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded direct PLC stage snapshots at the first, last and next-frame codeword.
use docsis_ofdm_fec::transmission::{
    plc::{InterleavedPlcBlock, PlcFrameGeometry, PlcRandomizer},
    SubcarrierSpacing,
};
/// Prints three selected96-label blocks per spacing with register snapshots.
/// # Returns
/// Unit after writing JSON to stdout; no files or captures are modified.
/// # Errors
/// Propagates label validation, sequence arithmetic or JSON serialization errors.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let geometry = PlcFrameGeometry::new(spacing);
        let input = InterleavedPlcBlock::from_values(spacing, &[0; 96])?;
        let mut stream = PlcRandomizer::new(spacing);
        for word in 0..=geometry.codewords_per_frame {
            let result = stream.apply(&input)?;
            if word == 0 || word >= geometry.codewords_per_frame - 1 {
                results.push(result);
            }
        }
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "experiment": "plc_randomization_known_phase",
            "input": "96 zero labels per block; not a valid encoded PLC frame",
            "blocks": results,
        }))?
    );
    Ok(())
}
