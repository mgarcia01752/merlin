/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Passive numerical queries delegate to existing directional scientific owners.
mod response;
mod spectrum;
use super::*;
pub use crate::signal_processing::receiver::diagnostics::{
    LlrStatistics as FullLoopLlrStatistics, ResponseDiagnostics as FullLoopResponseDiagnostics,
};
pub use response::*;
use serde::{Deserialize, Serialize};
pub use spectrum::*;

/// Actual retained coding-order LDPC soft-value statistics, separate from all-word totals.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopLlrObservation {
    pub context: FullLoopStageContext,
    pub element_start: usize,
    pub total_elements: usize,
    pub sign_convention: &'static str,
    pub statistics: FullLoopLlrStatistics,
}

impl FullLoopResult {
    /// Summarizes an actual retained payload LDPC-input event through the likelihood owner.
    /// # Arguments
    /// * `snapshot` - Zero-based actual retained codeword event; no later history is generated.
    /// # Returns
    /// Owned 41-bin histogram and statistics with original coding-order metadata and totals.
    /// A truncated event is explicitly described by its retained count and original total.
    /// # Errors
    /// Rejects unavailable snapshots, absent retained stages or invalid actual soft values.
    pub fn llr_statistics(&self, snapshot: usize) -> ApiResult<FullLoopLlrObservation> {
        let event = selected(self, FullLoopStage::PayloadLdpcInput, snapshot)?;
        let FullLoopStageData::Llrs(values) = &event.data else {
            return Err(super::super::error::invalid(
                "Retained LDPC input is not LLR data",
            ));
        };
        Ok(FullLoopLlrObservation {
            context: event.context.clone(),
            element_start: event.element_start,
            total_elements: event.total_elements,
            sign_convention: "log_p_zero_over_p_one",
            statistics: crate::signal_processing::receiver::diagnostics::llr_statistics(values)?,
        })
    }
}

fn selected(
    result: &FullLoopResult,
    stage: FullLoopStage,
    snapshot: usize,
) -> ApiResult<&FullLoopStageSnapshot> {
    result.stages.events(stage)?.get(snapshot).ok_or_else(|| {
        ApiError::new(
            ErrorCode::InvalidIdentifier,
            "Retained stage event is unavailable",
        )
    })
}
