/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Byte-constrained transport admission and received NCP-boundary length discovery.
use crate::{error::invalid, SimulationError};
use fec_codec::{
    parameters::{BCH_INPUT_BITS, LDPC_CODEWORD_BITS, PAYLOAD_PARITY_BITS},
    shortening::Shortening,
};
const BITS_PER_BYTE: usize = 8;
/// Maximum one-word received span, including at most13 last-cell padding bits.
pub const MAX_RECEIVED_CODEWORD_BITS: usize =
    LDPC_CODEWORD_BITS + crate::types::ofdm_modulation::MAX_OFDM_BITS_PER_SYMBOL as usize - 1;
/// Validates a shortened word for unambiguous pointer-based reception.
/// # Arguments
/// * `geometry` - Nonempty codec information length, including any caller-owned header.
/// # Returns
/// Unit when wire length is an odd number of bytes and shorter than the mother word.
/// No MAC header or stuffing bytes are invented by this operation.
/// # Errors
/// Rejects full, non-byte or even-byte lengths (PHY I20 §8.3.2).
pub fn validate_shortened_transport(geometry: Shortening) -> Result<(), SimulationError> {
    if geometry.payload_bits() >= BCH_INPUT_BITS
        || geometry.transmitted_bits() % (2 * BITS_PER_BYTE) != BITS_PER_BYTE
    {
        return Err(invalid(
            "Shortened transport requires an odd wire-byte count below16200 bits.",
        ));
    }
    Ok(())
}
/// Determines wire length from a terminated received span, excluding trailing cell bits.
/// # Arguments
/// * `span_bits` - Sum of actual per-cell loadings between accepted received boundaries.
/// # Returns
/// Checked original payload geometry; full words saturate at16200 received bits.
/// Short words round down to the nearest odd byte count per PHY I20 §8.3.2.
/// # Errors
/// Rejects empty/too-short spans or more than one full word plus13 final-cell bits.
pub fn discover_payload_length(span_bits: usize) -> Result<Shortening, SimulationError> {
    if span_bits > MAX_RECEIVED_CODEWORD_BITS {
        return Err(invalid(
            "Received interval exceeds one codeword and final-cell padding.",
        ));
    }
    let wire = if span_bits >= LDPC_CODEWORD_BITS {
        LDPC_CODEWORD_BITS
    } else {
        let bytes = span_bits / BITS_PER_BYTE;
        let odd = if bytes % 2 == 0 {
            bytes.saturating_sub(1)
        } else {
            bytes
        };
        odd * BITS_PER_BYTE
    };
    let payload = wire
        .checked_sub(PAYLOAD_PARITY_BITS)
        .ok_or_else(|| invalid("Received span is shorter than FEC parity."))?;
    Shortening::new(payload).map_err(invalid)
}
