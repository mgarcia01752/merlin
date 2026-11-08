/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Isolated source→BCH→LDPC→reverse decode→compare; no QAM or channel model.
use fec_codec::{
    concatenated::{compare_payload, ConcatenatedCodec},
    parameters::BCH_INPUT_BITS,
};
/// Runs a deterministic test-pattern block through the concatenated codec.
///
/// This pattern is not PRBS. All scientific work is called through the public API.
/// # Returns
/// Unit after reporting exact tested-bit and mismatch counts.
/// # Errors
/// Propagates invalid codec setup or receiver failures to the example runtime.
fn main() -> Result<(), String> {
    let source: Vec<u8> = (0..BCH_INPUT_BITS)
        .map(|position| u8::from((position * 17 + position / 11) % 19 < 9))
        .collect();
    let mut codec = ConcatenatedCodec::new()?;
    let encoded = codec.encode(&source)?;
    let decoded = codec.decode_noiseless(&encoded.ldpc_codeword, 30)?;
    let comparison = compare_payload(&source, &decoded)?;
    println!(
        "Payload bits: {} · bit errors: {}",
        comparison.bits_tested, comparison.bit_errors
    );
    if !decoded.ldpc.success || !decoded.bch.success || comparison.bit_errors != 0 {
        return Err("Noiseless codec-loop recovery failed.".into());
    }
    Ok(())
}
