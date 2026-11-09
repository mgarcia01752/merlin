/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin typed bridge to the independent PLC primitive; no duplicate LDPC mathematics.
use super::{
    geometry::{PLC_CODEWORD_BYTES, PLC_INFORMATION_BITS},
    PlcInformationBlock,
};
use crate::SimulationError;
use fec_codec::plc::{puncturing::puncture, PlcFecCodec};
use serde::Serialize;
// Message geometry must remain consistent with the separate codec definition.
const _: () = assert!(PLC_INFORMATION_BITS == fec_codec::plc::PLC_INFORMATION_BITS);
const _: () = assert!(PLC_CODEWORD_BYTES * 8 == fec_codec::plc::PLC_CODEWORD_BITS);
/// Separate mother and punctured boundaries, before nibble grouping or interleaving.
#[derive(Debug, Serialize)]
pub struct EncodedPlcBlock {
    pub mother_bits: Vec<u8>,
    pub transmitted_bits: Vec<u8>,
}
/// Diagnostic decode result; failed decisions remain visible, never silently accepted.
#[derive(Debug, Serialize)]
pub struct DecodedPlcBlock {
    pub information: PlcInformationBlock,
    pub mother_decisions: Vec<u8>,
    pub iterations: usize,
    pub success: bool,
}
/// Job-owned adapter for one-block operations; frame orchestration stays outside DSP.
pub struct PlcBlockCodec {
    codec: PlcFecCodec,
}
impl PlcBlockCodec {
    /// Builds reusable independent PLC codec setup.
    /// # Returns
    /// Job-owned codec with no payload/NCP shortening or shared decoder state.
    /// # Errors
    /// Returns PlcCodecSetup on matrix/encoder setup failure.
    pub fn new() -> Result<Self, SimulationError> {
        Ok(Self {
            codec: PlcFecCodec::new()
                .map_err(|error| SimulationError::new("PlcCodecSetup", error))?,
        })
    }
    /// Encodes one typed information block without changing it.
    /// # Arguments
    /// * `information` - Validated36-byte message input, expanded MSB first.
    /// # Returns
    ///480 mother bits and384 transmitted bits, each independently inspectable.
    /// # Errors
    /// Returns PlcEncodeFailure if the underlying stage rejects its inputs.
    pub fn encode(
        &self,
        information: &PlcInformationBlock,
    ) -> Result<EncodedPlcBlock, SimulationError> {
        let mother = self
            .codec
            .encode_mother(&information.bits())
            .map_err(|error| SimulationError::new("PlcEncodeFailure", error))?;
        let transmitted =
            puncture(&mother).map_err(|error| SimulationError::new("PlcEncodeFailure", error))?;
        Ok(EncodedPlcBlock {
            mother_bits: mother.to_vec(),
            transmitted_bits: transmitted.to_vec(),
        })
    }
    /// Decodes one transmitted block's likelihoods without transmitter truth input.
    /// # Arguments
    /// * `llrs` -384 finite natural-log P(0)/P(1) values; positive favors zero.
    /// * `max_iterations` - Application work limit,1..100.
    /// # Returns
    /// Information decisions, complete mother decisions, convergence and actual
    /// iterations. Callers check success and message CRC/truth separately.
    /// # Errors
    /// Returns PlcDecodeFailure on invalid input/work limits or unexpected output.
    pub fn decode(
        &mut self,
        llrs: &[f64],
        max_iterations: usize,
    ) -> Result<DecodedPlcBlock, SimulationError> {
        let decoded = self
            .codec
            .decode(llrs, max_iterations)
            .map_err(|error| SimulationError::new("PlcDecodeFailure", error))?;
        Ok(DecodedPlcBlock {
            information: PlcInformationBlock::from_bytes(&decoded.information_bytes())?,
            mother_decisions: decoded.mother_codeword.to_vec(),
            iterations: decoded.iterations,
            success: decoded.success,
        })
    }
}
