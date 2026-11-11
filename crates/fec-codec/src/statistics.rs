/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Decoder outcome accounting, independent of source verification and presentation.
use serde::Serialize;

/// One completed concatenated decode outcome; failure takes precedence over corrections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FecOutcome {
    Clean,
    Corrected,
    Uncorrectable,
}
impl FecOutcome {
    /// Classifies actual decoder flags and changes to received hard decisions.
    /// # Arguments
    /// * `ldpc_success`, `bch_success` - Actual stage acceptance flags.
    /// * `ldpc_changes`, `bch_changes` - Changed received bits, excluding synthetic positions.
    /// # Returns
    /// One mutually exclusive outcome; no source truth or iteration heuristic is used.
    pub fn from_decoder(
        ldpc_success: bool,
        bch_success: bool,
        ldpc_changes: usize,
        bch_changes: u32,
    ) -> Self {
        if !ldpc_success || !bch_success {
            Self::Uncorrectable
        } else if ldpc_changes > 0 || bch_changes > 0 {
            Self::Corrected
        } else {
            Self::Clean
        }
    }
}

/// Constant-size cumulative codeword counters with private state and read-only getters.
/// Serialize snapshots for adapters; updates accept completed decoder outcomes only.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct FecCounters {
    total_cw: u64,
    ccw: u64,
    ufec: u64,
    clean_cw: u64,
    undetected_errors: u64,
}
impl FecCounters {
    /// Constructs an empty counter set for one explicit measurement scope.
    /// # Returns
    /// Zero counts; no decoder or source state is owned.
    pub fn new() -> Self {
        Self::default()
    }
    /// Adds one completed word; both failed stages still count as one UFEC.
    /// # Arguments
    /// * `outcome` - Mutually exclusive decoder outcome.
    /// * `source_mismatch` - Independent verification error; absent truth is false.
    /// # Returns
    /// Unit; counters saturate together at u64::MAX, preserving the partition invariant.
    pub fn record(&mut self, outcome: FecOutcome, source_mismatch: bool) {
        if self.total_cw == u64::MAX {
            return;
        }
        self.total_cw += 1;
        match outcome {
            FecOutcome::Clean => self.clean_cw += 1,
            FecOutcome::Corrected => self.ccw += 1,
            FecOutcome::Uncorrectable => self.ufec += 1,
        }
        if source_mismatch && outcome != FecOutcome::Uncorrectable {
            self.undetected_errors += 1;
        }
    }
    /// Reads completed codeword attempts in this counter scope.
    /// # Returns
    /// TotalCW = clean codewords + corrected codewords + uncorrectable codewords.
    pub fn total_codewords(&self) -> u64 {
        self.total_cw
    }
    /// Reads decoder-successful codewords with actual LDPC or BCH correction.
    /// # Returns
    /// CCW, counting each word once regardless of the number of corrected bits.
    pub fn corrected_codewords(&self) -> u64 {
        self.ccw
    }
    /// Reads codewords rejected by either decoder stage.
    /// # Returns
    /// UFEC, counting the union of failed stages rather than their sum.
    pub fn uncorrectable_codewords(&self) -> u64 {
        self.ufec
    }
    /// Reads decoder-successful codewords requiring no hard-decision correction.
    /// # Returns
    /// Clean codeword count, independent of source verification.
    pub fn clean_codewords(&self) -> u64 {
        self.clean_cw
    }
    /// Reads successful decoder outcomes contradicted by independent source truth.
    /// # Returns
    /// Undetected word errors; these remain separate from decoder failure flags.
    pub fn undetected_errors(&self) -> u64 {
        self.undetected_errors
    }
    /// Copies an immutable point-in-time value for callers and future language bindings.
    /// # Returns
    /// Independent snapshot; subsequent updates cannot alter it.
    pub fn snapshot(&self) -> Self {
        *self
    }
}
