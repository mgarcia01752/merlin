/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Independent recurrence in a chronological ring, not the TX shift register.
use crate::{
    configuration::MAX_CODED_BITS,
    error::{invalid, limit},
    types::BitRange,
    PhyError, PhyResult,
};
use serde::Serialize;

/// Contiguous source-comparison outcome, independent of decoder convergence.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Comparison {
    /// Absolute range actually compared, never inferred from successful parity.
    pub range: BitRange,
    /// Actual Hamming distance including failed decoder decisions.
    pub bit_errors: usize,
    /// Unavailable for an empty range rather than a misleading zero BER.
    pub ber: Option<f64>,
}

/// Independent expected sequence requiring contiguous comparison coverage.
#[derive(Debug, Clone)]
pub struct PrbsVerifier {
    history: [u8; 15],
    oldest: usize,
    position: u64,
}

impl PrbsVerifier {
    /// Initializes an independent chronological recurrence from `seed`.
    ///
    /// Returns idle state; rejects zero/out-of-range seed without source access.
    pub fn new(seed: u16) -> PhyResult<Self> {
        if seed == 0 || seed > 32767 {
            return Err(invalid("verifier.seed"));
        }
        let history = std::array::from_fn(|index| ((seed >> index) & 1) as u8);
        Ok(Self {
            history,
            oldest: 0,
            position: 0,
        })
    }
    /// Compares recovered decisions against independently regenerated bits.
    ///
    /// `start` must equal next contiguous position; `bits` must be binary and
    /// bounded. Returns errors/BER, preserving state on invalid ranges or input.
    pub fn compare(&mut self, start: u64, bits: &[u8]) -> PhyResult<Comparison> {
        if start != self.position {
            return Err(PhyError::Association {
                field: "verifier.start",
            });
        }
        if bits.len() > MAX_CODED_BITS {
            return Err(limit("verifier.chunk"));
        }
        if bits.iter().any(|bit| *bit > 1) {
            return Err(invalid("verifier.bits"));
        }
        let range = BitRange::new(start, bits.len())?;
        let mut errors = 0;
        for &actual in bits {
            let expected = self.history[self.oldest];
            let newest = (self.oldest + 14) % 15;
            self.history[self.oldest] = self.history[newest] ^ expected;
            self.oldest = (self.oldest + 1) % 15;
            errors += usize::from(actual != expected);
        }
        self.position = range.end();
        Ok(Comparison {
            range,
            bit_errors: errors,
            ber: if bits.is_empty() {
                None
            } else {
                Some(errors as f64 / bits.len() as f64)
            },
        })
    }
}
