/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Owned completed/partial science; no parity-only recovery or infinite metrics.
use super::{ExperimentConfig, ExperimentOptions, ExperimentPreview, RunStatus};
use crate::{
    receiver::DecodedWord,
    types::{BitRange, Identity},
};
use serde::Serialize;

/// Structured terminal diagnostic while preserving earlier completed science.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunFailure {
    /// Error category and stable field context as an owned diagnostic.
    pub diagnostic: String,
    /// Stable shared domain error category, independent of diagnostic wording.
    pub category: &'static str,
    /// Stable scientific boundary when supplied by the domain owner.
    pub field: Option<&'static str>,
}

/// One started grant; generated/emitted/compared coverage stays distinct.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GrantReport {
    /// Immutable run/configuration/grant association.
    pub identity: Identity,
    /// Source actually generated when transmitter admission committed.
    pub generated_source: BitRange,
    /// Emitted waveform sample count including the delivered tail prefix.
    pub emitted_samples: usize,
    /// Actual received support, including model drain beyond transmitter output.
    pub received_samples: usize,
    /// Actual source decisions compared, including failed words.
    pub compared_bits: usize,
    /// Generated source positions without completed decoded decisions.
    pub unverified_source_bits: usize,
    /// Exact planned TX support including explicit guards.
    pub planned_transmit_samples: usize,
    /// Planned waveform suffix not emitted at the stop/error boundary.
    pub pending_transmit_samples: usize,
    /// Planned FEC words without a completed decoder outcome.
    pub unprocessed_codewords: usize,
    /// Independent source Hamming distance, not inferred from FEC flags.
    pub bit_errors: usize,
    /// Completed received-only codeword outcomes, including actual failed decisions.
    pub words: Vec<DecodedWord>,
    /// True after normal grant completion, independent of recovery success.
    pub completed: bool,
    /// Useful windows known to have missing coverage.
    pub missing_symbols: usize,
    /// Independent deboosted ideal-data metric, never the input RxMER target.
    pub matched_data_mer: crate::instruments::MatchedMer,
    /// Explicit unavailable/zero-error/finite state for the matched metric.
    pub matched_data_mer_value: crate::instruments::MerValue,
}

/// Independent upstream experiment schema, distinct from standalone FEC schema 1.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BatchReport {
    /// Format identifier distinguishing this document from input/FEC JSON.
    pub format: &'static str,
    /// Experiment export schema version, independent of configuration format.
    pub schema_version: u32,
    /// Retained independent generic configuration document, absent for typed-only runs.
    pub source_configuration_json: Option<String>,
    /// Exact original generic JSON bytes, distinct from the resolved canonical export.
    pub source_configuration_original: Option<Vec<u8>>,
    /// SHA-256 over those exact original bytes, absent for typed-only runs.
    pub source_configuration_sha256: Option<String>,
    /// Additive schema1 metadata: exact measured sources/associations, absent for synthetic runs.
    pub measurement_provenance_json: Option<String>,
    /// Pinned channel-only random generator identifier and exact dependency pins.
    pub noise_generator: &'static str,
    /// Frozen scientific input snapshot including explicit synthetic provenance.
    pub configuration: ExperimentConfig,
    /// Frozen run budgets and retention policy actually used.
    pub options: ExperimentOptions,
    /// Original pure admitted schedule, not retroactively reduced on cancellation.
    pub planned: ExperimentPreview,
    /// Actual lifecycle terminal state.
    pub status: RunStatus,
    /// All started grants with owned partial/failed science.
    pub grants: Vec<GrantReport>,
    /// Explicit scheduled grant IDs that never began source admission.
    pub not_started_grants: Vec<u64>,
    /// Actual whole-byte source generated across started grants.
    pub generated_source_bits: usize,
    /// Actual source decisions independently compared.
    pub compared_source_bits: usize,
    /// Actually generated source positions without completed compared decisions.
    pub unverified_source_bits: usize,
    /// Independent Hamming distance over the compared prefix.
    pub bit_errors: usize,
    /// Unavailable if no source bits were compared; never JSON infinity.
    pub ber: Option<f64>,
    /// Requires completed nonempty full coverage and all source/FEC/fill checks.
    pub reference_recovered: bool,
    /// Terminal numerical diagnostic when processing failed after admission.
    pub failure: Option<RunFailure>,
    /// Actual monotonic execution wall time in milliseconds.
    pub elapsed_ms: u64,
    /// Passive bounded actual sample/cell history; no query reruns the experiment.
    pub observations: crate::instruments::ObservationStore,
}

impl BatchReport {
    /// Queries retained actual QAM cells at `tap`, optional `loading` and limit.
    ///
    /// Returns an owned snapshot; unavailable/disabled history rejects explicitly.
    /// The query never starts a worker, consumes RNG or changes this report.
    pub fn qam(
        &self,
        tap: crate::instruments::Tap,
        loading: Option<u8>,
        maximum: usize,
    ) -> crate::PhyResult<crate::instruments::QamSnapshot> {
        self.observations.qam(tap, loading, maximum)
    }
    /// Calculates a PSD from retained actual contiguous samples at `tap`.
    ///
    /// `mode` selects the bounded analysis FFT size. Returns owned normalized
    /// power/Hz, or explicit unavailable history; never regenerates samples.
    pub fn spectrum(
        &self,
        tap: crate::instruments::Tap,
        mode: crate::configuration::Numerology,
    ) -> crate::PhyResult<crate::instruments::SpectrumSnapshot> {
        self.observations.spectrum(tap, mode)
    }
}
