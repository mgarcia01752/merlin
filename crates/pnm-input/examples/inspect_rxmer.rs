/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: public owners and fixture paths.
 */
//! Independently executed Rust consumer of authored upstream CCAP RxMER files.
use pnm_input::upstream::RxmerCapture;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut observations = Vec::new();
    for fixture in [
        include_str!("../../../examples/measurements/upstream/rxmer/ccap-50khz.hex"),
        include_str!("../../../examples/measurements/upstream/rxmer/ccap-25khz.hex"),
    ] {
        let digits: String = fixture.chars().filter(|c| !c.is_whitespace()).collect();
        let bytes = digits
            .as_bytes()
            .chunks_exact(2)
            .map(|pair| {
                u8::from_str_radix(std::str::from_utf8(pair)?, 16)
                    .map_err(Box::<dyn std::error::Error>::from)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let capture = RxmerCapture::from_pnm(&bytes)?;
        observations.push(serde_json::from_str::<serde_json::Value>(
            &capture.to_json()?,
        )?);
    }
    println!("{}", serde_json::to_string(&observations)?);
    Ok(())
}
