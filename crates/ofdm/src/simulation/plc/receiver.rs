/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Known-phase receive composition from local PLC I/Q; never accepts TX labels/truth.
use crate::{
    error::invalid,
    transmission::{
        plc::{
            block_interleaving::deinterleave_llrs,
            geometry::{PLC_CODEWORD_NIBBLES, PLC_DATA_SYMBOLS},
            mapping, preamble, DecodedPlcBlock, PlcBlockCodec, PlcFrameGeometry, PlcMessageFrame,
            PlcRandomizer,
        },
        SubcarrierSpacing,
    },
    types::{IqPair, PLC_FRAME_SYMBOLS},
    LabError,
};
use serde::Serialize;
/// All codeword decisions plus independent message validation, including failures.
#[derive(Debug, Serialize)]
pub struct ReceivedPlcFrame {
    pub preamble_signs_match: bool,
    pub blocks: Vec<DecodedPlcBlock>,
    /// Present only when every LDPC word converged and message parsing/TS CRC passed.
    pub message: Option<PlcMessageFrame>,
    pub message_error: Option<LabError>,
}
/// Job-owned reusable decoder; known frame boundaries are explicit caller knowledge.
pub struct PlcFrameReceiver {
    spacing: SubcarrierSpacing,
    codec: PlcBlockCodec,
}
impl PlcFrameReceiver {
    /// Creates independent receiver setup without transmitter diagnostics.
    /// # Arguments
    /// * `spacing` - Known FFT numerology for local PLC columns.
    /// # Returns
    /// Receiver with reusable codec; mask state is reset at each explicit frame start.
    /// # Errors
    /// Propagates codec setup failures.
    pub fn new(spacing: SubcarrierSpacing) -> Result<Self, LabError> {
        Ok(Self {
            spacing,
            codec: PlcBlockCodec::new()?,
        })
    }
    /// Decodes one known-phase PLC frame from received equalized I/Q.
    /// # Arguments
    /// * `iq` -128 ascending local PLC columns, first post-preamble data symbol first.
    /// * `noise_power` - Positive finite complex demapper variance in normalized units.
    /// * `max_iterations` - LDPC iteration budget per word,1..100.
    /// # Returns
    /// All word decisions/iterations plus optional CRC-validated message. Preamble
    /// signs are diagnostic only; this is not blind timing or frequency acquisition.
    /// # Errors
    /// Rejects shape, nonfinite values, invalid variance/budget or decoder API errors.
    /// Decoder nonconvergence/message CRC failure remain in the returned result.
    pub fn decode(
        &mut self,
        iq: &[IqPair],
        noise_power: f64,
        max_iterations: usize,
    ) -> Result<ReceivedPlcFrame, LabError> {
        let geometry = PlcFrameGeometry::new(self.spacing);
        if iq.len() != PLC_FRAME_SYMBOLS as usize * geometry.plc_subcarriers
            || iq.iter().flatten().any(|value| !value.is_finite())
            || !noise_power.is_finite()
            || noise_power <= 0.0
            || !(1..=100).contains(&max_iterations)
        {
            return Err(invalid(
                "Invalid known-phase PLC receive grid or decoder limits.",
            ));
        }
        let mut masks = PlcRandomizer::new(self.spacing);
        let mut blocks = Vec::new();
        let mut bytes = Vec::new();
        for cells in
            iq[..PLC_DATA_SYMBOLS * geometry.plc_subcarriers].chunks_exact(PLC_CODEWORD_NIBBLES)
        {
            let llrs = mapping::demap(cells, noise_power)?;
            let plain = masks.descramble_llrs(&llrs)?;
            let decoded = self.codec.decode(
                &deinterleave_llrs(self.spacing, plain.llrs())?,
                max_iterations,
            )?;
            bytes.extend_from_slice(decoded.information.bytes());
            blocks.push(decoded);
        }
        let message_result = if blocks.iter().all(|block| block.success) {
            PlcMessageFrame::from_bytes(self.spacing, &bytes)
        } else {
            Err(LabError::new(
                "PlcFrameDecodeFailure",
                "At least one PLC LDPC word did not converge.",
            ))
        };
        let (message, message_error) = match message_result {
            Ok(message) => (Some(message), None),
            Err(error) => (None, Some(error)),
        };
        let mut preamble_signs_match = true;
        for phase in PLC_DATA_SYMBOLS..PLC_FRAME_SYMBOLS as usize {
            let expected = preamble::generate(self.spacing, phase as u64)?;
            for (sample, bit) in iq
                [phase * geometry.plc_subcarriers..(phase + 1) * geometry.plc_subcarriers]
                .iter()
                .zip(expected.bits)
            {
                preamble_signs_match &= if bit == 0 {
                    sample[0] > 0.0
                } else {
                    sample[0] < 0.0
                };
            }
        }
        Ok(ReceivedPlcFrame {
            preamble_signs_match,
            blocks,
            message,
            message_error,
        })
    }
}
