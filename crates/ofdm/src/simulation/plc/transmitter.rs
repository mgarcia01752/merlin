/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin whole-frame composition; coding, permutation, masks and QAM retain their owners.
use crate::{
    error::invalid,
    transmission::{
        plc::{
            block_interleaving::interleave,
            geometry::{PLC_DATA_SYMBOLS, PLC_PREAMBLE_SYMBOLS},
            mapping,
            preamble::{self, PlcPreambleSymbol},
            EncodedPlcBlock, InterleavedPlcBlock, PlcBlockCodec, PlcCodewordNibbles,
            PlcMessageFrame, PlcRandomizer, RandomizedPlcBlock,
        },
        SubcarrierSpacing,
    },
    types::{IqPair, PLC_FRAME_SYMBOLS},
    SimulationError,
};
use serde::Serialize;
/// Completed scientific boundaries for one PLC codeword, before optional observation.
#[derive(Debug, Serialize)]
pub struct PlcCodewordSignals {
    pub encoded: EncodedPlcBlock,
    pub coded: PlcCodewordNibbles,
    pub interleaved: InterleavedPlcBlock,
    pub randomized: RandomizedPlcBlock,
    pub iq: Vec<IqPair>,
}
/// Bounded whole-frame PLC result:120 data columns followed by eight preamble columns.
#[derive(Debug, Serialize)]
pub struct PlcFrameSignals {
    pub frame_index: u64,
    pub spacing: SubcarrierSpacing,
    pub message: PlcMessageFrame,
    pub codewords: Vec<PlcCodewordSignals>,
    pub preamble: Vec<PlcPreambleSymbol>,
}
impl PlcFrameSignals {
    /// Borrows one absolute channel symbol's local PLC column for physical assembly.
    /// # Arguments
    /// * `symbol_index` - PLC-aligned channel index within this result's frame.
    /// # Returns
    ///8or16 normalized cells, or None outside this frame/inconsistent edited metadata.
    pub fn symbol_iq(&self, symbol_index: u64) -> Option<&[IqPair]> {
        let start = self.frame_index.checked_mul(PLC_FRAME_SYMBOLS)?;
        let phase = usize::try_from(symbol_index.checked_sub(start)?).ok()?;
        if phase >= PLC_FRAME_SYMBOLS as usize {
            return None;
        }
        if phase >= PLC_DATA_SYMBOLS {
            return self
                .preamble
                .get(phase - PLC_DATA_SYMBOLS)
                .map(|symbol| symbol.iq.as_slice());
        }
        let geometry = self.message.geometry();
        let word = phase / geometry.data_symbols_per_codeword;
        let first = (phase % geometry.data_symbols_per_codeword) * geometry.plc_subcarriers;
        self.codewords
            .get(word)?
            .iq
            .get(first..first + geometry.plc_subcarriers)
    }
}
/// Generates one complete known-phase PLC frame, independent of a waveform runner.
/// # Arguments
/// * `message` - Exact timestamp/MC information frame with caller-owned reference time.
/// * `frame_index` - Absolute128-symbol channel-frame index; checked for overflow.
/// # Returns
/// Owned staged bytes/bits/labels/IQ and unrandomized BPSK preamble columns.
/// Work is bounded to10or20 codewords and at most2048 complex cells.
/// # Errors
/// Rejects index overflow or propagates message/codec/permutation/mapping failures.
pub fn generate_frame(
    message: PlcMessageFrame,
    frame_index: u64,
) -> Result<PlcFrameSignals, SimulationError> {
    let start = frame_index
        .checked_mul(PLC_FRAME_SYMBOLS)
        .and_then(|index| index.checked_add(PLC_FRAME_SYMBOLS - 1))
        .map(|end| end - (PLC_FRAME_SYMBOLS - 1))
        .ok_or_else(|| invalid("PLC frame index overflows channel symbols."))?;
    let spacing = message.spacing();
    let codec = PlcBlockCodec::new()?;
    let mut randomizer = PlcRandomizer::new(spacing);
    let mut codewords = Vec::with_capacity(message.geometry().codewords_per_frame);
    for information in message.information_blocks()? {
        let encoded = codec.encode(&information)?;
        let coded = PlcCodewordNibbles::from_bits(&encoded.transmitted_bits)?;
        let interleaved = interleave(spacing, &coded);
        let mut randomized = randomizer.apply(&interleaved)?;
        randomized.first_symbol_index += start;
        let iq = mapping::map(&randomized)?;
        codewords.push(PlcCodewordSignals {
            encoded,
            coded,
            interleaved,
            randomized,
            iq,
        });
    }
    let preamble = (0..PLC_PREAMBLE_SYMBOLS)
        .map(|phase| preamble::generate(spacing, start + PLC_DATA_SYMBOLS as u64 + phase as u64))
        .collect::<Result<_, _>>()?;
    Ok(PlcFrameSignals {
        frame_index,
        spacing,
        message,
        codewords,
        preamble,
    })
}
