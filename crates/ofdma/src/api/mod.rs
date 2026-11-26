/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Typed idle upstream simulation sessions over the independent Rust engine.
mod error;
mod measured;
mod reports;
mod resources;
mod session;
pub use crate::channel::ChannelConfig;
pub use crate::configuration::*;
pub use crate::instruments::{
    ConfiguredResponsePoint, ConfiguredResponseRequest, ConfiguredResponseSnapshot,
    ConfiguredResponseSource, FecSummary, FecWordPage, FecWordRequest, FecWordSnapshot,
    PeriodicImpulse, PeriodicImpulsePoint, QamSnapshot, ReceiverEstimateIdentity,
    ReceiverEstimatePoint, ReceiverEstimateRequest, ReceiverEstimateSnapshot,
    ReceiverObservationPolicy, ReceiverObservationPolicyInfo, ReceiverObservationSummary,
    SoftCellPage, SoftCellRequest, SoftCellSnapshot, SoftObservationPolicy,
    SoftObservationPolicyInfo, SoftObservationSummary, SoftWordIdentity, SoftWordRequest,
    SoftWordSnapshot, SpectrumSnapshot, Tap, VerificationSnapshot, MAX_CONFIGURED_IMPULSE_SAMPLES,
    MAX_CONFIGURED_RESPONSE_POINTS, MAX_FEC_PAGE_DECISION_BITS, MAX_FEC_PAGE_WORDS,
    MAX_RECEIVER_ESTIMATE_FRAMES, MAX_RECEIVER_ESTIMATE_POINTS, MAX_RECEIVER_OBSERVATION_SCALARS,
    MAX_RECEIVER_SOFT_CELLS, MAX_RECEIVER_SOFT_WORDS, MAX_RECEIVER_SOFT_WORD_LLRS,
    MAX_SOFT_CELL_PAGE_CELLS, MAX_SOFT_CELL_PAGE_LLRS, MAX_SOFT_WORD_QUERY_LLRS,
};
pub use crate::instruments::{
    QamConstellationRequest, QamConstellationSnapshot, QamNormalization, QamPair, QamPairRequest,
    QamPairSnapshot, QamPoint, QamReferencePoint, QamReferenceSample,
    MAX_QAM_CONSTELLATION_REFERENCES, MAX_QAM_PAIR_POINTS, MAX_QAM_QUERY_POINTS,
};
pub use crate::measurements::{
    PowerSnapshot, PreEqEvent, PreEqOperation, PreEqPlayback, ProbeReference, RxmerPlayback,
    RxmerPolicy, SyntheticRxmer,
};
pub use crate::resources::CellRole;
pub use crate::signal_processing::PreEqBank;
pub use crate::simulation::{
    BatchReport, ExperimentConfig, ExperimentOptions, ExperimentPreview, GrantReport,
    ProgressSnapshot, RunControl, RunStatus,
};
pub use crate::types::Iq;
pub use crate::PhyError;
pub use error::{ApiError, ApiResult, CaptureFailure};
pub use measured::MeasuredConfiguration;
pub use merlin_api::ErrorCode;
pub(crate) use reports::bounded_json;
pub use reports::ReportJson;
pub use resources::*;
pub use session::Simulation;
