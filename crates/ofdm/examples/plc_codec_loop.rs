/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Complete TS+idle-MC message recovery through the separate PLC LDPC code.
use docsis_ofdm_fec::transmission::{
    plc::{ExtendedTimestamp, PlcBlockCodec, PlcMessageFrame},
    SubcarrierSpacing,
};
/// Runs two bounded noiseless PLC frame experiments and writes JSON.
/// # Returns
/// Unit with exact compared bits, information errors, actual block iterations and
/// selected intermediate arrays. No modulation, interleaving or waveform is used.
/// # Errors
/// Propagates setup, codec, message CRC, convergence or JSON failures.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut codec = PlcBlockCodec::new()?;
    let mut results = Vec::new();
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let original = PlcMessageFrame::idle(spacing, ExtendedTimestamp::new(123, 19, 0)?)?;
        let source = original.to_bytes();
        let mut recovered = Vec::new();
        let mut iterations = Vec::new();
        let mut first_encoded = None;
        for information in original.information_blocks()? {
            let encoded = codec.encode(&information)?;
            let llrs: Vec<_> = encoded
                .transmitted_bits
                .iter()
                .map(|bit| if *bit == 0 { 20.0 } else { -20.0 })
                .collect();
            let decoded = codec.decode(&llrs, 100)?;
            if !decoded.success {
                return Err("Noiseless PLC block did not converge.".into());
            }
            recovered.extend_from_slice(decoded.information.bytes());
            iterations.push(decoded.iterations);
            if first_encoded.is_none() {
                first_encoded = Some(encoded);
            }
        }
        let parsed = PlcMessageFrame::from_bytes(spacing, &recovered)?;
        let bit_errors: u32 = source
            .iter()
            .zip(&recovered)
            .map(|(original, received)| (original ^ received).count_ones())
            .sum();
        results.push(serde_json::json!({
            "spacing_hz": spacing.frequency_hz(),
            "geometry": original.geometry(),
            "compared_information_bits": source.len() * 8,
            "information_bit_errors": bit_errors,
            "block_iterations": iterations,
            "message_frame_matches": parsed == original,
            "first_encoded_block": first_encoded,
        }));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &serde_json::json!({"experiment": "noiseless_plc_ldpc", "frames":results})
        )?
    );
    Ok(())
}
