/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin selection of continuous PRBS or finite file stages; algorithms keep one owner.
use super::file::FilePayload;
#[cfg(feature = "simulation")]
use super::{
    file::{FileSource, FileSourceSummary, FileVerifier, RecoveredFile},
    Prbs15, PrbsComparison, PrbsSeed, PrbsVerifier,
};
#[cfg(feature = "simulation")]
use crate::SimulationError;
/// Immutable shared source configuration; raw file bytes never enter JSON output.
#[derive(Clone, Default, serde::Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PayloadSourceConfig {
    #[default]
    Prbs15,
    File {
        payload: FilePayload,
    },
}
#[cfg(feature = "simulation")]
pub(crate) enum PayloadSource {
    Prbs(Prbs15),
    File(FileSource),
}
#[cfg(feature = "simulation")]
pub(crate) enum PayloadVerifier {
    Prbs(PrbsVerifier),
    File(FileVerifier),
}
#[cfg(feature = "simulation")]
impl PayloadSourceConfig {
    /// Builds independently owned generation and truth state for a new run.
    /// # Arguments
    /// * `seed` - Continuous PRBS seed; file fill has its own explicit seed.
    /// # Returns
    /// Private source/verifier stages sharing only immutable original file bytes.
    /// # Errors
    /// Rejects invalid PRBS setup before any stream advances.
    pub(crate) fn stages(
        &self,
        seed: PrbsSeed,
    ) -> Result<(PayloadSource, PayloadVerifier), SimulationError> {
        Ok(match self {
            Self::Prbs15 => (
                PayloadSource::Prbs(Prbs15::new(seed)?),
                PayloadVerifier::Prbs(PrbsVerifier::new(seed)?),
            ),
            Self::File { payload } => (
                PayloadSource::File(FileSource::new(payload.clone())?),
                PayloadVerifier::File(FileVerifier::new(payload.clone())?),
            ),
        })
    }
}
#[cfg(feature = "simulation")]
impl PayloadSource {
    /// Emits the next BCH payload or finite EOF.
    /// # Returns
    /// Complete information bits; continuous PRBS never returns EOF.
    /// # Errors
    /// Propagates bounded source failures.
    pub(crate) fn next_payload(&mut self) -> Result<Option<Vec<u8>>, SimulationError> {
        match self {
            Self::Prbs(source) => source.next_payload().map(Some),
            Self::File(source) => source.next_payload(),
        }
    }
    /// Reports meaningful emitted source bits.
    /// # Returns
    /// Cursor independent of coding padding.
    pub(crate) fn position(&self) -> u64 {
        match self {
            Self::Prbs(source) => source.position(),
            Self::File(source) => source.position(),
        }
    }
}
#[cfg(feature = "simulation")]
impl PayloadVerifier {
    /// Compares actual decoded payload with independent selected-source truth.
    /// # Arguments
    /// * `bits` - Decoded BCH information payload.
    /// # Returns
    /// Meaningful comparison only, excluding final file padding.
    /// # Errors
    /// Propagates malformed decision/sequence or work-bound failures.
    pub(crate) fn compare(&mut self, bits: &[u8]) -> Result<PrbsComparison, SimulationError> {
        match self {
            Self::Prbs(verifier) => verifier.compare(bits),
            Self::File(verifier) => verifier.compare_payload(bits),
        }
    }
    /// Borrows cumulative meaningful comparison totals.
    /// # Returns
    /// Source-ordered file/fill or continuous PRBS totals.
    pub(crate) fn totals(&self) -> &PrbsComparison {
        match self {
            Self::Prbs(verifier) => verifier.totals(),
            Self::File(verifier) => verifier.totals(),
        }
    }
    /// Reports finite-file progress separately from continuous PRBS metadata.
    /// # Arguments
    /// * `source` - Matching generation stage from the same immutable configuration.
    /// # Returns
    /// File counters or None for continuous PRBS.
    pub(crate) fn file_summary(&self, source: &PayloadSource) -> Option<FileSourceSummary> {
        match (self, source) {
            (Self::File(verifier), PayloadSource::File(source)) => {
                Some(verifier.summary(source.position(), source.padding_bits()))
            }
            _ => None,
        }
    }
    /// Copies bounded original-file recovery for API/CLI output.
    /// # Returns
    /// File output or None for continuous PRBS.
    pub(crate) fn recovered_file(&self) -> Option<RecoveredFile> {
        match self {
            Self::File(verifier) => Some(verifier.recovered_file()),
            Self::Prbs(_) => None,
        }
    }
}
