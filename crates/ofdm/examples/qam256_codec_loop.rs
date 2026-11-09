/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Print per-codeword256-QAM recovery and LDPC iteration results as JSON.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use docsis_ofdm_fec::simulation::{coding_loop::NoiselessQam256Codec, source::SeededSource};
    let mut source = SeededSource::new(1);
    let mut codec = NoiselessQam256Codec::new()?;
    let mut results = Vec::new();
    for _ in 0..3 {
        let result = codec.run_block(&source.next_payload(), 30)?;
        if !result.ldpc_success || !result.bch_success || result.payload_bit_errors != 0 {
            return Err("Noiseless256-QAM recovery failed.".into());
        }
        results.push(result);
    }
    println!("{}", serde_json::to_string_pretty(&results)?);
    Ok(())
}
