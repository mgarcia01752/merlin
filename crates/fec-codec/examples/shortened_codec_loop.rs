/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Isolated variable-length FEC loop; no QAM, files, waveform or channel dependency.
use fec_codec::shortening::{compare_payload, ShortenedCodec, Shortening};
/// Runs an explicit original bit count through true DOCSIS shortening and decoding.
/// # Returns
/// Unit after printing stage sizes, actual iterations and independently measured errors.
/// # Errors
/// Rejects invalid arguments/setup or failed recovery; optional argument is K, default6136.
fn main() -> Result<(), String> {
    let mut arguments = std::env::args().skip(1);
    let payload_bits = arguments.next().map_or(Ok(6136), |value| {
        value.parse::<usize>().map_err(|error| error.to_string())
    })?;
    if arguments.next().is_some() {
        return Err("Expected at most one original payload bit count.".into());
    }
    let shortening = Shortening::new(payload_bits)?;
    let payload: Vec<_> = (0..payload_bits)
        .map(|index| u8::from((index * 17 + index / 11) % 19 < 9))
        .collect();
    let mut codec = ShortenedCodec::new(shortening)?;
    let encoded = codec.encode(&payload)?;
    let decoded = codec.decode_noiseless(&encoded.transmitted, 30)?;
    let comparison = compare_payload(&payload, &decoded)?;
    println!(
        "Payload bits: {} · transmitted bits: {} · omitted zeros: {}",
        payload_bits,
        encoded.transmitted.len(),
        shortening.omitted_bits()
    );
    println!(
        "LDPC iterations: {} · bit errors: {}",
        decoded.ldpc.iterations, comparison.bit_errors
    );
    if !decoded.success() || comparison.bit_errors != 0 {
        return Err("Shortened codec recovery failed.".into());
    }
    Ok(())
}
