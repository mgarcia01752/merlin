/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Job-owned PLC code setup, separate from payload and NCP codec instances.
use super::{
    matrix::mother_matrix,
    parameters::{PLC_INFORMATION_BITS, PLC_INFORMATION_BYTES, PLC_MOTHER_BITS},
};
use crate::sparse_encoder::SparseEncoder;
use ldpc_toolbox::{
    decoder::{
        factory::{DecoderFactory, DecoderImplementation},
        LdpcDecoder,
    },
    sparse::SparseMatrix,
};
/// Parity outcome and decisions; success alone does not establish message CRC/truth.
#[derive(Debug)]
pub struct PlcBlockResult {
    pub information: [u8; PLC_INFORMATION_BITS],
    pub mother_codeword: [u8; PLC_MOTHER_BITS],
    pub iterations: usize,
    pub success: bool,
}
impl PlcBlockResult {
    /// Packs information decisions into the original byte order, including on failure.
    /// # Returns
    ///36 bytes with bit a0 at byte0's MSB. Check success and higher-layer CRC/truth
    /// before accepting these bytes; failed decisions remain diagnostic data.
    pub fn information_bytes(&self) -> [u8; PLC_INFORMATION_BYTES] {
        std::array::from_fn(|byte| {
            (0..8).fold(0, |value, offset| {
                (value << 1) | self.information[byte * 8 + offset]
            })
        })
    }
}
/// Reusable PLC encoder/soft decoder built from the pinned (480,288) matrix.
/// No shortened-zero priors, payload BCH or NCP control matrix are used.
pub struct PlcFecCodec {
    pub(super) encoder: SparseEncoder,
    pub(super) decoder: Box<dyn LdpcDecoder>,
    pub(super) matrix: SparseMatrix,
}
impl PlcFecCodec {
    /// Builds job-owned encoder and horizontal-layered sum-product decoder setup.
    /// # Returns
    /// Independent safe Rust codec; does not alter global or caller state.
    /// # Errors
    /// Reports an unsupported parity dependency instead of a caller-input panic.
    pub fn new() -> Result<Self, String> {
        let matrix = mother_matrix();
        let encoder = SparseEncoder::from_h(&matrix)?;
        let decoder = DecoderImplementation::HLPhif64.build_decoder(matrix.clone());
        log::debug!(
            "plc-codec-ready information-bits={PLC_INFORMATION_BITS} mother-bits={PLC_MOTHER_BITS}"
        );
        Ok(Self {
            encoder,
            decoder,
            matrix,
        })
    }
}
