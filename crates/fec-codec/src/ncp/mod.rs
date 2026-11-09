/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! NCP control LDPC, separate from downstream payload BCH/LDPC.
mod codec;
mod matrix;
pub use codec::{NcpBlockResult, NcpFecCodec};
/// NCP information length, including a CRC block when supplied as its payload.
pub const NCP_INFORMATION_BITS: usize = 24;
/// Shortened and punctured transmitted NCP codeword length.
pub const NCP_CODEWORD_BITS: usize = 48;
pub(super) const MOTHER_INFORMATION_BITS: usize = 80;
pub(super) const MOTHER_CODEWORD_BITS: usize = 160;
pub(super) const KNOWN_ZERO_LLR: f64 = 100.0;
/// Mapping from transmitted bit indices to mother-code coordinates, in ascending
/// coordinate order. QAM label grouping is a separate transmission operation.
pub const NCP_TRANSMITTED_COORDINATES: [usize; NCP_CODEWORD_BITS] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 104, 105,
    106, 107, 108, 109, 110, 111, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140,
    141, 142, 143,
];
