/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use super::FilePayload;
use crate::{simulation::source::Prbs15, SimulationError};
use fec_codec::parameters::BCH_INPUT_BITS;
/// Finite source cursor; file bits are MSB-first, appended PRBS follows its own recurrence.
pub struct FileSource {
    input: FilePayload,
    fill: Option<Prbs15>,
    position: u64,
    padding_bits: u64,
}
impl FileSource {
    /// Creates a fresh source without reading paths or consuming any data.
    /// # Arguments
    /// * `input` - Previously validated immutable file/fill configuration.
    /// # Returns
    /// Source aligned to file bit zero, with independent optional fill state.
    /// # Errors
    /// Propagates invalid fill seed setup; construction never changes the input.
    pub fn new(input: FilePayload) -> Result<Self, SimulationError> {
        let fill = input
            .fill
            .map(|policy| Prbs15::new(policy.seed))
            .transpose()?;
        Ok(Self {
            input,
            fill,
            position: 0,
            padding_bits: 0,
        })
    }
    /// Reports meaningful bits emitted so far, excluding final BCH padding.
    /// # Returns
    /// Absolute file-plus-fill cursor, bounded by the configured target.
    pub fn position(&self) -> u64 {
        self.position
    }
    /// Reports zero padding used in the final information payload.
    /// # Returns
    /// Zero until the final partial payload; padding is not source data.
    pub fn padding_bits(&self) -> u64 {
        self.padding_bits
    }
    /// Emits one complete BCH information payload, padding only its final tail.
    /// # Returns
    /// None at EOF, without changing state; otherwise exactly BCH_INPUT_BITS bits.
    /// # Errors
    /// Propagates bounded fill generation failures without committing source state.
    pub fn next_payload(&mut self) -> Result<Option<Vec<u8>>, SimulationError> {
        let count = (self.input.target_bits - self.position).min(BCH_INPUT_BITS as u64) as usize;
        if count == 0 {
            return Ok(None);
        }
        let original_end = self.input.original_bytes as u64 * 8;
        let file_count = original_end.saturating_sub(self.position).min(count as u64) as usize;
        let mut bits = Vec::with_capacity(BCH_INPUT_BITS);
        for offset in 0..file_count {
            let index = self.position as usize + offset;
            bits.push((self.input.bytes()[index / 8] >> (7 - index % 8)) & 1);
        }
        if let Some(fill) = &mut self.fill {
            bits.extend(fill.generate(count - file_count)?);
        }
        self.position += count as u64;
        self.padding_bits += (BCH_INPUT_BITS - count) as u64;
        bits.resize(BCH_INPUT_BITS, 0);
        Ok(Some(bits))
    }
}
