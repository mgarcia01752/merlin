/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Byte display only; scientific parsing and original payload ownership stay in the backend.
use super::constants::{HEX_BYTES_PER_ROW, HEX_BYTE_DIGITS, HEX_OFFSET_MIN_DIGITS};
use serde_json::{json, Value};

/// Formats exact byte values as space-separated uppercase hexadecimal.
/// # Arguments
/// * `bytes` - Already bounded caller-owned packet or field bytes, never modified.
/// # Returns
/// Owned display text, empty for empty input; no parsing or validation occurs.
pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:0width$X}", width = HEX_BYTE_DIGITS))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Formats an original address as colon-separated lowercase hexadecimal.
/// # Arguments
/// * `bytes` - Backend-provided address bytes in their original order.
/// # Returns
/// Owned label without interpreting or changing address identity.
pub(crate) fn address(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:0width$x}", width = HEX_BYTE_DIGITS))
        .collect::<Vec<_>>()
        .join(":")
}

/// Projects exact packet bytes into the existing bounded-owner hex-view rows.
/// # Arguments
/// * `bytes` - Complete packet already bounded by the public message owner.
/// # Returns
/// Display rows preserving every byte and its original offset; no truncation,
/// reassembly, checksum calculation or interpretation occurs.
pub(crate) fn rows(bytes: &[u8]) -> Vec<Value> {
    bytes.chunks(HEX_BYTES_PER_ROW).enumerate().map(|(row, chunk)| {
        json!({"offset":format!("{:0width$X}",row * HEX_BYTES_PER_ROW, width=HEX_OFFSET_MIN_DIGITS),"hex":hex(chunk)})
    }).collect()
}
