/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use super::FilePayload;
use crate::{
    error::invalid,
    simulation::source::{PrbsComparison, PrbsVerifier},
    SimulationError,
};
use fec_codec::parameters::BCH_INPUT_BITS;
use serde::Serialize;
/// Separate meaningful segments and completion; no coding padding enters BER.
#[derive(Debug, Clone, Serialize)]
pub struct FileSourceSummary {
    pub original_bytes: usize,
    pub target_bits: u64,
    pub generated_bits: u64,
    pub coding_padding_bits: u64,
    pub file: PrbsComparison,
    /// Fill first_bit is relative to the first appended bit, not original-file bit zero.
    pub fill: PrbsComparison,
    pub file_ber: Option<f64>,
    pub fill_ber: Option<f64>,
    pub fully_generated: bool,
    pub file_fully_compared: bool,
    pub fully_compared: bool,
}
/// Original-file output only; JSON contains metadata, never the raw bytes.
#[derive(Clone, Serialize)]
pub struct RecoveredFile {
    #[serde(skip)]
    bytes: Vec<u8>,
    pub original_bytes: usize,
    pub recovered_bits: u64,
    pub complete: bool,
}
impl RecoveredFile {
    /// Borrows recovered whole bytes, including actual decoded errors.
    /// # Returns
    /// Original-file bytes only; no appended PRBS, coding padding or partial byte.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}
/// Independent truth cursor and bounded recovered original-file storage.
pub struct FileVerifier {
    input: FilePayload,
    fill: Option<PrbsVerifier>,
    file: PrbsComparison,
    totals: PrbsComparison,
    recovered: Vec<u8>,
}
fn empty_comparison() -> PrbsComparison {
    PrbsComparison {
        first_bit: 0,
        bits_tested: 0,
        bit_errors: 0,
        first_error_bit: None,
    }
}
fn ber(comparison: &PrbsComparison) -> Option<f64> {
    (comparison.bits_tested > 0)
        .then(|| comparison.bit_errors as f64 / comparison.bits_tested as f64)
}
impl FileVerifier {
    /// Creates independent file/fill truth without feeding it into receive decisions.
    /// # Arguments
    /// * `input` - Shared immutable original input and finite fill policy.
    /// # Returns
    /// Empty comparison counters and recovered-byte buffer.
    /// # Errors
    /// Propagates invalid fill seed setup.
    pub fn new(input: FilePayload) -> Result<Self, SimulationError> {
        let fill = input
            .fill
            .map(|policy| PrbsVerifier::new(policy.seed))
            .transpose()?;
        Ok(Self {
            input,
            fill,
            file: empty_comparison(),
            totals: empty_comparison(),
            recovered: Vec::new(),
        })
    }
    /// Compares one decoded BCH payload; only meaningful file/fill bits count.
    /// # Arguments
    /// * `recovered` - Exactly BCH_INPUT_BITS binary decisions, including final padding.
    /// # Returns
    /// Actual meaningful comparison with absolute file-plus-fill offsets.
    /// # Errors
    /// Rejects malformed/nonbinary input or unexpected payload after EOF before mutation.
    pub fn compare_payload(&mut self, recovered: &[u8]) -> Result<PrbsComparison, SimulationError> {
        if recovered.len() != BCH_INPUT_BITS
            || recovered.iter().any(|bit| *bit > 1)
            || self.totals.bits_tested == self.input.target_bits
        {
            return Err(invalid(
                "Invalid file comparison payload or unexpected data after EOF.",
            ));
        }
        let first = self.totals.bits_tested;
        let count = (self.input.target_bits - first).min(recovered.len() as u64) as usize;
        let file_count = (self.input.original_bytes as u64 * 8)
            .saturating_sub(first)
            .min(count as u64) as usize;
        let mut window = empty_comparison();
        window.first_bit = first;
        window.bits_tested = count as u64;
        for (offset, &bit) in recovered[..file_count].iter().enumerate() {
            let index = first as usize + offset;
            let expected = (self.input.bytes()[index / 8] >> (7 - index % 8)) & 1;
            if bit != expected {
                window.bit_errors += 1;
                window.first_error_bit.get_or_insert(index as u64);
                self.file.bit_errors += 1;
                self.file.first_error_bit.get_or_insert(index as u64);
            }
            if index % 8 == 0 {
                self.recovered.push(0);
            }
            self.recovered[index / 8] |= bit << (7 - index % 8);
        }
        self.file.bits_tested += file_count as u64;
        if let Some(verifier) = &mut self.fill {
            let fill = verifier.compare(&recovered[file_count..count])?;
            window.bit_errors += fill.bit_errors;
            if let Some(index) = fill.first_error_bit {
                window
                    .first_error_bit
                    .get_or_insert(self.input.original_bytes as u64 * 8 + index);
            }
        }
        self.totals.bits_tested += window.bits_tested;
        self.totals.bit_errors += window.bit_errors;
        if self.totals.first_error_bit.is_none() {
            self.totals.first_error_bit = window.first_error_bit;
        }
        Ok(window)
    }
    /// Borrows cumulative meaningful comparisons without advancing truth.
    /// # Returns
    /// File and appended-fill totals; coding padding is excluded.
    pub fn totals(&self) -> &PrbsComparison {
        &self.totals
    }
    /// Reports source emission and independent receive completion separately.
    /// # Arguments
    /// * `generated_bits` - Source cursor excluding coding padding.
    /// * `padding_bits` - Actually emitted final-payload coding padding.
    /// # Returns
    /// Bounded scalar counters, separate BER and explicit partial completion.
    pub fn summary(&self, generated_bits: u64, padding_bits: u64) -> FileSourceSummary {
        let fill = self
            .fill
            .as_ref()
            .map_or_else(empty_comparison, |verifier| verifier.totals().clone());
        FileSourceSummary {
            original_bytes: self.input.original_bytes,
            target_bits: self.input.target_bits,
            generated_bits,
            coding_padding_bits: padding_bits,
            file: self.file.clone(),
            file_ber: ber(&self.file),
            fill_ber: ber(&fill),
            fill,
            fully_generated: generated_bits == self.input.target_bits,
            file_fully_compared: self.file.bits_tested == self.input.original_bytes as u64 * 8,
            fully_compared: self.totals.bits_tested == self.input.target_bits,
        }
    }
    /// Copies the bounded recovered original-file prefix for API/export consumers.
    /// # Returns
    /// Whole decoded bytes only and honest completion metadata; bytes may contain errors.
    pub fn recovered_file(&self) -> RecoveredFile {
        RecoveredFile {
            bytes: self.recovered[..self.file.bits_tested as usize / 8].to_vec(),
            original_bytes: self.input.original_bytes,
            recovered_bits: self.file.bits_tested,
            complete: self.file.bits_tested == self.input.original_bytes as u64 * 8,
        }
    }
}
