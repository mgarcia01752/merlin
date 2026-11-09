/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Pure Rust codec building blocks and the concatenated BCH/LDPC path.
#![forbid(unsafe_code)]
pub mod bch;
pub mod concatenated;
pub mod crc;
pub mod ldpc;
pub mod ncp;
pub mod parameters;
pub mod plc;
pub mod shortening;

// Retain established root imports while implementation lives with each codec.
pub use bch::{decode_bch, encode_bch, BchResult};
pub use ldpc::{LdpcCodec, LdpcResult};
pub use parameters::{BCH_INPUT_BITS, LDPC_CODEWORD_BITS, LDPC_INPUT_BITS};

fn binary_bits(bits: &[u8], expected: usize) -> Result<(), String> {
    if bits.len() != expected || bits.iter().any(|bit| *bit > 1) {
        log::debug!(
            "binary-input-rejected expected-bits={expected} actual-bits={}",
            bits.len()
        );
        return Err(format!("Expected exactly {expected} binary bits."));
    }
    Ok(())
}
