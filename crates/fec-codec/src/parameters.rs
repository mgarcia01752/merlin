/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Dimensions of the supported downstream short8/9 coding block.
/// DOCSIS PHY I20 Table 23 / ETSI EN302769 V1.2.1 Table 3(b).
pub const BCH_INPUT_BITS: usize = 14_232;
/// BCH output and LDPC input length for the selected unshortened coding mode.
pub const LDPC_INPUT_BITS: usize = 14_400;
/// LDPC transmitted codeword length for the selected coding mode.
pub const LDPC_CODEWORD_BITS: usize = 16_200;
/// Fixed outer parity retained at every DOCSIS shortened payload length.
pub const BCH_PARITY_BITS: usize = LDPC_INPUT_BITS - BCH_INPUT_BITS;
/// Fixed inner parity retained at every DOCSIS shortened payload length.
pub const LDPC_PARITY_BITS: usize = LDPC_CODEWORD_BITS - LDPC_INPUT_BITS;
/// Total parity carried by each full or shortened downstream payload word.
pub const PAYLOAD_PARITY_BITS: usize = BCH_PARITY_BITS + LDPC_PARITY_BITS;

/// Finite reliable-bit evidence shared by isolated noiseless payload codec helpers.
pub(crate) const NOISELESS_LLR_MAGNITUDE: f64 = 20.0;
