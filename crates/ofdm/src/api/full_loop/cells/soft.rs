/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Existing actual cell and decoder observations remain separate soft-value reference planes.
use super::*;
use crate::signal_processing::receiver::diagnostics::soft_view;
use serde::Serialize;
pub use soft_view::{
    SoftEnvelope as FullLoopSoftEnvelope, SoftHistogram as FullLoopSoftHistogram,
    SoftHistogramBin as FullLoopSoftHistogramBin, SoftValueView as FullLoopSoftValueView,
};

/// Explicit selected physical-cell soft sequence, separate from coding-order decoder inputs.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopCellSoftView {
    pub selection: FullLoopCellSelection,
    pub source_plane: &'static str,
    pub view: FullLoopSoftValueView,
}
/// Actual retained payload LDPC input with original coding-order context and element extent.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopDecoderSoftView {
    pub stage: FullLoopStage,
    pub context: FullLoopStageContext,
    pub element_start: usize,
    pub total_elements: usize,
    pub view: FullLoopSoftValueView,
}

impl FullLoopResult {
    /// Projects actual post-descrambling selected-cell soft values through the diagnostic owner.
    /// # Arguments
    /// * `query` - Actual role/profile/order/coordinate/window selection, at most 2048 cells.
    /// # Returns
    /// Owned histogram/envelope/statistics over the explicit displayed cell window. Envelope
    /// indices describe that selected soft sequence, not a reconstructed global codeword.
    /// Protected streams keep empty soft data; no likelihoods or probabilities are synthesized.
    /// # Errors
    /// Rejects invalid selections/work/offsets or invalid actual soft observations.
    pub fn cell_soft_view(&self, query: FullLoopCellQuery) -> ApiResult<FullLoopCellSoftView> {
        let selected = Selected::new(
            self.job.constellation.as_ref(),
            self.job.protected_constellation.as_ref(),
            query,
            2048,
        )?;
        let values: Vec<_> = selected
            .window()
            .iter()
            .flat_map(|cell| cell.llrs.iter().copied())
            .collect();
        Ok(FullLoopCellSoftView {
            source_plane: "post_descrambling_cell_llrs",
            view: soft_view::prepare(&values, 0)?,
            selection: selected.info,
        })
    }

    /// Projects an actual retained payload LDPC-input event through the same diagnostic owner.
    /// # Arguments
    /// * `snapshot` - Zero-based actual retained coding-order event.
    /// # Returns
    /// Owned soft statistics/histograms/envelope with original context/offset/total. A truncated
    /// event remains explicitly shorter than its original total; missing values are never filled.
    /// # Errors
    /// Preserves missing/invalid snapshot and finite soft-value/index-budget errors.
    pub fn decoder_soft_view(&self, snapshot: usize) -> ApiResult<FullLoopDecoderSoftView> {
        let event = self
            .stages
            .events(FullLoopStage::PayloadLdpcInput)?
            .get(snapshot)
            .ok_or_else(|| {
                ApiError::new(
                    ErrorCode::InvalidIdentifier,
                    "Retained LDPC event is unavailable",
                )
            })?;
        let FullLoopStageData::Llrs(values) = &event.data else {
            return Err(super::super::super::error::invalid(
                "Retained LDPC input is not LLR data",
            ));
        };
        Ok(FullLoopDecoderSoftView {
            stage: FullLoopStage::PayloadLdpcInput,
            context: event.context.clone(),
            element_start: event.element_start,
            total_elements: event.total_elements,
            view: soft_view::prepare(values, event.element_start)?,
        })
    }
}
