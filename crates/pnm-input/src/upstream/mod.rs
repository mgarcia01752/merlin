/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native bounded upstream measurement admission, separate from PHY processing.
mod association;
mod constants;
mod pre_eq;
mod rxmer;
mod rxmer_format;
mod source_json;
mod synthetic_rxmer;
mod ucd;
mod ucd_projection;
pub use association::{
    CaptureAssociation, CaptureEvent, CaptureProbe, CaptureSource, MeasuredExperiment,
};
use docsis_ofdma::api::Iq;
pub use docsis_ofdma::api::{ApiError, ApiResult, CaptureFailure};
pub use pre_eq::{PreEqCapture, PreEqCaptureKind, PreEqHeader};
pub use rxmer::{RxmerCapture, RxmerHeader};
pub use synthetic_rxmer::{
    SyntheticRxmerCapture, SyntheticRxmerRequest, SyntheticRxmerValues,
    MAX_SYNTHETIC_RXMER_REQUEST_BYTES,
};
pub use ucd::{UcdBurst, UcdCapture, UcdHeader, UcdTlv};
pub use ucd_projection::reassemble_ucd;

pub(crate) fn bounded_json(value: &impl serde::Serialize, maximum: usize) -> ApiResult<String> {
    Ok(pnm::common::json::to_json(value, maximum)?)
}
