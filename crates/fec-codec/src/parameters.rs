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
