/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 §7.5.13.8/Figure68: independent PLC state, clocked after each nibble.
use super::{
    geometry::{PLC_BITS_PER_CELL, PLC_CODEWORD_BITS, PLC_DATA_SYMBOLS, PLC_PREAMBLE_SYMBOLS},
    InterleavedPlcBlock, PlcFrameGeometry, PlcNibble,
};
use crate::{
    error::invalid,
    signal_processing::modulation::randomization::{Lfsr, RandomizerState},
    transmission::SubcarrierSpacing,
    types::PLC_FRAME_SYMBOLS,
    LabError,
};
use serde::Serialize;

/// Owned transmitter output, before PLC16-QAM mapping; one time-major codeword.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RandomizedPlcBlock {
    /// Fixed spacing defining the6x16 or12x8 local grid.
    pub spacing: SubcarrierSpacing,
    /// Channel symbol index, with0 the first symbol after the initial preamble.
    pub first_symbol_index: u64,
    /// Registers immediately before the first label, after any frame reset.
    pub state_before: RandomizerState,
    /// Registers after the final label; includes its required clock.
    pub state_after: RandomizerState,
    values: Vec<PlcNibble>,
}
impl RandomizedPlcBlock {
    /// Borrows the randomized labels without copying or advancing a stream.
    /// # Returns
    ///96 labels in time-major, ascending local carrier order; each0..15.
    pub fn values(&self) -> &[PlcNibble] {
        &self.values
    }
}

/// Owned soft receiver output, before inverse PLC block interleaving.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DescrambledPlcBlock {
    /// Fixed spacing defining the time-major local PLC grid.
    pub spacing: SubcarrierSpacing,
    /// Known-reference channel symbol index of the first received label.
    pub first_symbol_index: u64,
    /// Receiver registers before the first label, after any frame reset.
    pub state_before: RandomizerState,
    /// Receiver registers after the final label's required clock.
    pub state_after: RandomizerState,
    llrs: Vec<f64>,
}
impl DescrambledPlcBlock {
    /// Borrows time-major soft bits after the XOR mask has been undone.
    /// # Returns
    ///384 finite MSB-first LLRs, positive for bit0; original magnitudes preserved.
    pub fn llrs(&self) -> &[f64] {
        &self.llrs
    }
}

/// Frame-aligned PLC stream. TX and RX each own an independent instance.
///
/// Starts at the first post-preamble symbol. Each successful operation consumes
/// exactly one96-cell block. State crosses codeword/symbol boundaries; the eight
/// preamble slots never enter this API. The next frame reseeds immediately before
/// its first label. This is known-reference sequencing, not acquisition or seek.
#[derive(Clone)]
pub struct PlcRandomizer {
    spacing: SubcarrierSpacing,
    next_symbol_index: u64,
    lfsr: Lfsr,
}
impl PlcRandomizer {
    /// Constructs an independent stream at phase0 with the specified PLC seeds.
    /// # Arguments
    /// * `spacing` - Fixed channel spacing; cannot change within a stream.
    /// # Returns
    /// PLC-only state D0=0x4A7,D1=0xB4C; no data/NCP instance is accessed.
    pub fn new(spacing: SubcarrierSpacing) -> Self {
        Self {
            spacing,
            next_symbol_index: 0,
            lfsr: Lfsr::plc(),
        }
    }

    /// Copies live registers without advancing, resetting or exposing mutable state.
    /// # Returns
    /// Last processed cell's post-clock registers, or initial seeds before use.
    /// At a frame boundary these remain unchanged until the next block begins.
    pub fn state(&self) -> RandomizerState {
        self.lfsr.state()
    }

    /// Reports the channel index of the next data block, including preamble gaps.
    /// # Returns
    ///0,6,...,114,128,... at25kHz or0,12,...,108,128,... at50kHz.
    pub fn next_symbol_index(&self) -> u64 {
        self.next_symbol_index
    }

    /// Applies the PLC mask to a complete time-major interleaved codeword.
    /// # Arguments
    /// * `block` - Validated unrandomized96-cell block matching this stream's spacing.
    /// # Returns
    /// Owned labels and register snapshots; input remains unchanged. Successful
    /// calls advance one codeword, skipping preamble slots after120 data symbols.
    /// # Errors
    /// Rejects spacing mismatch or channel-index overflow without changing state.
    pub fn apply(&mut self, block: &InterleavedPlcBlock) -> Result<RandomizedPlcBlock, LabError> {
        if block.spacing() != self.spacing {
            log::debug!("plc-randomizer-rejected reason=spacing-mismatch");
            return Err(invalid("PLC block spacing must match its randomizer."));
        }
        let (mut candidate, next_symbol_index) = self.prepare()?;
        let state_before = candidate.state();
        let values = block
            .values()
            .iter()
            .map(|label| {
                let randomized = *label ^ candidate.mask(PLC_BITS_PER_CELL) as PlcNibble;
                candidate.clock();
                randomized
            })
            .collect();
        let output = RandomizedPlcBlock {
            spacing: self.spacing,
            first_symbol_index: self.next_symbol_index,
            state_before,
            state_after: candidate.state(),
            values,
        };
        self.lfsr = candidate;
        self.next_symbol_index = next_symbol_index;
        Ok(output)
    }

    /// Undoes PLC randomization on demapper soft bits using receiver-owned state.
    /// # Arguments
    /// * `llrs` -384 finite values, four MSB-first bits per time-major PLC label.
    ///   Positive means bit0. Instantiate a separate receiver stream at phase0.
    /// # Returns
    /// Owned sign-corrected LLRs, preserving magnitude and order for subsequent
    /// inverse interleaving. Advances exactly like apply; no hard decisions made.
    /// # Errors
    /// Rejects wrong length, nonfinite input or index overflow without advancing.
    pub fn descramble_llrs(&mut self, llrs: &[f64]) -> Result<DescrambledPlcBlock, LabError> {
        if llrs.len() != PLC_CODEWORD_BITS || llrs.iter().any(|value| !value.is_finite()) {
            log::debug!("plc-descrambler-rejected actual-llrs={}", llrs.len());
            return Err(invalid(
                "PLC descrambling requires384 finite bit likelihoods.",
            ));
        }
        let (mut candidate, next_symbol_index) = self.prepare()?;
        let state_before = candidate.state();
        let mut corrected = Vec::with_capacity(PLC_CODEWORD_BITS);
        for label_llrs in llrs.chunks_exact(PLC_BITS_PER_CELL) {
            let mask = candidate.mask(PLC_BITS_PER_CELL);
            for (bit, value) in label_llrs.iter().enumerate() {
                let bit_mask = 1 << (PLC_BITS_PER_CELL - 1 - bit);
                corrected.push(if mask & bit_mask == 0 {
                    *value
                } else {
                    -*value
                });
            }
            candidate.clock();
        }
        let output = DescrambledPlcBlock {
            spacing: self.spacing,
            first_symbol_index: self.next_symbol_index,
            state_before,
            state_after: candidate.state(),
            llrs: corrected,
        };
        self.lfsr = candidate;
        self.next_symbol_index = next_symbol_index;
        Ok(output)
    }

    fn prepare(&self) -> Result<(Lfsr, u64), LabError> {
        let symbols = PlcFrameGeometry::new(self.spacing).data_symbols_per_codeword as u64;
        let next = self
            .next_symbol_index
            .checked_add(symbols)
            .and_then(|next| {
                if next % PLC_FRAME_SYMBOLS == PLC_DATA_SYMBOLS as u64 {
                    next.checked_add(PLC_PREAMBLE_SYMBOLS as u64)
                } else {
                    Some(next)
                }
            });
        let next = next.ok_or_else(|| invalid("PLC randomizer channel index overflow."))?;
        let candidate = if self.next_symbol_index % PLC_FRAME_SYMBOLS == 0 {
            Lfsr::plc()
        } else {
            self.lfsr.clone()
        };
        Ok((candidate, next))
    }
}
