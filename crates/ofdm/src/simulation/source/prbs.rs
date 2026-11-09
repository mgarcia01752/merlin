/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit PRBS15 experiment source; unrelated to the PHY cell randomizer.
use crate::{error::invalid, SimulationError};
use fec_codec::parameters::BCH_INPUT_BITS;
use serde::Serialize;

/// Fifteen initial bits; bit zero is emitted first. Zero is forbidden.
pub type PrbsSeed = u16;
/// Absolute source-bit identity, continuous across codewords and periods.
pub type SourceBitIndex = u64;
/// Experiment allocation/work bound per operation, not a DOCSIS limit.
pub const MAX_PRBS_CHUNK_BITS: usize = 1_048_576;
/// Degree of the explicitly defined sequence polynomial x^15 + x^14 + 1.
pub const PRBS15_DEGREE: u32 = 15;
/// Maximum nonzero 15-bit seed and maximal sequence period.
pub const PRBS15_PERIOD: PrbsSeed = (1 << PRBS15_DEGREE) - 1;

/// Owned continuous source with no host entropy or automatic reseeding.
#[derive(Debug, Clone)]
pub struct Prbs15 {
    state: PrbsSeed,
    position: SourceBitIndex,
}

impl Prbs15 {
    /// Initializes s[0..15] from seed bits zero through fourteen.
    ///
    /// # Arguments
    /// * `seed` - Nonzero 15-bit initial sequence, emitted least-significant bit first.
    /// # Returns
    /// Independent state at absolute bit position zero.
    /// # Errors
    /// Rejects zero and seeds with bits outside the 15-bit register.
    pub fn new(seed: PrbsSeed) -> Result<Self, SimulationError> {
        if seed == 0 || seed > PRBS15_PERIOD {
            return Err(invalid("PRBS15 requires a nonzero 15-bit seed."));
        }
        Ok(Self {
            state: seed,
            position: 0,
        })
    }

    /// Reports the number of bits already emitted, without advancing state.
    ///
    /// # Returns
    /// Absolute next-bit position, not reduced modulo the PRBS period.
    pub fn position(&self) -> SourceBitIndex {
        self.position
    }

    /// Generates a bounded chunk using s[n+15] = s[n+14] XOR s[n].
    ///
    /// # Arguments
    /// * `count` - Number of bits, zero through MAX_PRBS_CHUNK_BITS.
    /// # Returns
    /// Binary bits in transmission order; zero count leaves state unchanged.
    /// # Errors
    /// Rejects excessive work or absolute-position overflow before advancing state.
    pub fn generate(&mut self, count: usize) -> Result<Vec<u8>, SimulationError> {
        self.checked_end(count)?;
        Ok((0..count).map(|_| self.next_bit()).collect())
    }

    /// Generates the next short-code BCH information payload without reseeding.
    ///
    /// # Returns
    /// Exactly BCH_INPUT_BITS bits continuing from the previous call.
    /// # Errors
    /// Rejects absolute-position overflow without advancing state.
    pub fn next_payload(&mut self) -> Result<Vec<u8>, SimulationError> {
        self.generate(BCH_INPUT_BITS)
    }

    fn checked_end(&self, count: usize) -> Result<SourceBitIndex, SimulationError> {
        if count > MAX_PRBS_CHUNK_BITS {
            return Err(invalid("PRBS chunk exceeds the configured work bound."));
        }
        self.position
            .checked_add(count as SourceBitIndex)
            .ok_or_else(|| invalid("PRBS absolute bit position overflow."))
    }

    fn next_bit(&mut self) -> u8 {
        let output = (self.state & 1) as u8;
        let feedback = (self.state ^ (self.state >> (PRBS15_DEGREE - 1))) & 1;
        self.state = (self.state >> 1) | (feedback << (PRBS15_DEGREE - 1));
        self.position += 1;
        output
    }
}

/// One comparison window, preserving absolute identity and actual mismatch count.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrbsComparison {
    pub first_bit: SourceBitIndex,
    pub bits_tested: SourceBitIndex,
    pub bit_errors: SourceBitIndex,
    pub first_error_bit: Option<SourceBitIndex>,
}

/// Independent expected stream; never changes a receive decoder's decisions.
pub struct PrbsVerifier {
    expected: Prbs15,
    totals: PrbsComparison,
}

impl PrbsVerifier {
    /// Starts an independent truth stream aligned to source bit zero.
    ///
    /// # Arguments
    /// * `seed` - Same explicit seed as the source, not receiver configuration.
    /// # Returns
    /// Empty comparison history with independently owned PRBS state.
    /// # Errors
    /// Rejects invalid PRBS15 seeds.
    pub fn new(seed: PrbsSeed) -> Result<Self, SimulationError> {
        Ok(Self {
            expected: Prbs15::new(seed)?,
            totals: PrbsComparison {
                first_bit: 0,
                bits_tested: 0,
                bit_errors: 0,
                first_error_bit: None,
            },
        })
    }

    /// Compares the next ordered recovered bits with independent PRBS truth.
    ///
    /// # Arguments
    /// * `recovered` - Binary decisions at the expected next source position.
    /// # Returns
    /// This window's errors and absolute first mismatch; advances through errors
    /// without resynchronizing. Totals retain all previous comparisons.
    /// # Errors
    /// Rejects nonbinary decisions, excessive work or position overflow before
    /// changing state. Missing, duplicate or reordered data must be handled by
    /// the caller; this verifier performs no blind PRBS phase acquisition.
    pub fn compare(&mut self, recovered: &[u8]) -> Result<PrbsComparison, SimulationError> {
        self.expected.checked_end(recovered.len())?;
        if recovered.iter().any(|bit| *bit > 1) {
            return Err(invalid(
                "PRBS comparison requires binary recovered decisions.",
            ));
        }
        let mut window = PrbsComparison {
            first_bit: self.expected.position(),
            bits_tested: recovered.len() as SourceBitIndex,
            bit_errors: 0,
            first_error_bit: None,
        };
        for (offset, bit) in recovered.iter().enumerate() {
            if *bit != self.expected.next_bit() {
                window.bit_errors += 1;
                window
                    .first_error_bit
                    .get_or_insert(window.first_bit + offset as SourceBitIndex);
            }
        }
        self.totals.bits_tested = self.expected.position();
        self.totals.bit_errors += window.bit_errors;
        if self.totals.first_error_bit.is_none() {
            self.totals.first_error_bit = window.first_error_bit;
        }
        Ok(window)
    }

    /// Borrows cumulative comparison results without consuming expected bits.
    ///
    /// # Returns
    /// Totals from source bit zero through the last accepted comparison.
    pub fn totals(&self) -> &PrbsComparison {
        &self.totals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_position_overflow_rejects_without_advancing_source_or_totals() {
        let mut source = Prbs15::new(1).unwrap();
        source.position = SourceBitIndex::MAX - 1;
        let register = source.state;
        assert!(source.generate(2).is_err());
        assert_eq!(source.state, register);
        assert_eq!(source.position(), SourceBitIndex::MAX - 1);
        assert_eq!(source.generate(1).unwrap(), [1]);
        assert_eq!(source.position(), SourceBitIndex::MAX);
        assert!(source.generate(1).is_err());
        assert!(source.generate(0).unwrap().is_empty());

        let mut verifier = PrbsVerifier::new(1).unwrap();
        verifier.expected.position = SourceBitIndex::MAX - 1;
        verifier.totals.bits_tested = SourceBitIndex::MAX - 1;
        let before = verifier.totals().clone();
        assert!(verifier.compare(&[1, 0]).is_err());
        assert_eq!(verifier.totals(), &before);
        assert_eq!(verifier.expected.state, 1);
        assert_eq!(verifier.compare(&[0]).unwrap().bit_errors, 1);
        assert_eq!(verifier.totals().bits_tested, SourceBitIndex::MAX);
    }
}
