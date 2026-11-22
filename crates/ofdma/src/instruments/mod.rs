/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Passive bounded owned QAM/SA snapshots; queries never rerun science.
mod metrics;
pub(crate) mod soft;
pub use soft::{
    SoftCellPage, SoftCellRequest, SoftCellSnapshot, SoftObservationPolicy,
    SoftObservationPolicyInfo, SoftObservationSummary, SoftWordIdentity, SoftWordRequest,
    SoftWordSnapshot, MAX_RECEIVER_SOFT_CELLS, MAX_RECEIVER_SOFT_WORDS,
    MAX_RECEIVER_SOFT_WORD_LLRS, MAX_SOFT_CELL_PAGE_CELLS, MAX_SOFT_CELL_PAGE_LLRS,
    MAX_SOFT_WORD_QUERY_LLRS,
};
pub(crate) mod receiver;
mod recovery;
pub use receiver::{
    ReceiverEstimateIdentity, ReceiverEstimatePoint, ReceiverEstimateRequest,
    ReceiverEstimateSnapshot, ReceiverObservationPolicy, ReceiverObservationPolicyInfo,
    ReceiverObservationSummary, MAX_RECEIVER_ESTIMATE_FRAMES, MAX_RECEIVER_ESTIMATE_POINTS,
    MAX_RECEIVER_OBSERVATION_SCALARS,
};
pub(crate) mod response;
pub use recovery::{
    FecSummary, FecWordPage, FecWordRequest, FecWordSnapshot, VerificationSnapshot,
    MAX_FEC_PAGE_DECISION_BITS, MAX_FEC_PAGE_WORDS,
};
pub use response::{
    ConfiguredResponsePoint, ConfiguredResponseRequest, ConfiguredResponseSnapshot,
    ConfiguredResponseSource, PeriodicImpulse, PeriodicImpulsePoint,
    MAX_CONFIGURED_IMPULSE_SAMPLES, MAX_CONFIGURED_RESPONSE_POINTS,
};
mod observer;
pub use metrics::{MatchedMer, MerValue};
mod spectrum;
pub use observer::{ObservationStore, QamPoint, QamSnapshot, Tap};
pub use spectrum::SpectrumSnapshot;
