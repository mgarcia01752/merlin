/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Detached actual receiver observations; no job, waveform, capture or stage-store ownership.
mod copy;
mod policy;
use super::*;
use crate::simulation::channel::{
    constellation::{ConstellationPair, ConstellationResult, NcpConstellationPair},
    protected_constellation::{ProtectedConstellation, ProtectedPair, ProtectedStream},
};
pub use policy::*;
use serde::Serialize;
use std::{mem::size_of, sync::Arc};

struct Retained {
    policy: FullLoopReceiverArchivePolicy,
    receiver_mode: ReceiverMode,
    data: Option<ConstellationResult>,
    protected: Option<ProtectedConstellation>,
    decoder: Option<FullLoopStageSnapshot>,
    retained_bytes: usize,
}

/// Immutable detached history; cloning shares only the finite copied observations.
#[derive(Clone)]
pub struct FullLoopReceiverArchive(Arc<Retained>);

/// Actual copied counts/budget and original decoder identity; absent observations stay absent.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopReceiverArchiveInfo {
    pub policy: FullLoopReceiverArchiveInput,
    pub receiver_mode: ReceiverMode,
    pub retained_bytes: usize,
    pub payload_cells: usize,
    pub ncp_cells: usize,
    pub plc_data_cells: usize,
    pub plc_preamble_cells: usize,
    pub continuous_pilot_cells: usize,
    pub scattered_pilot_cells: usize,
    pub decoder_llrs: usize,
    pub decoder_snapshot_index: Option<usize>,
    pub decoder_context: Option<FullLoopStageContext>,
    pub decoder_element_start: Option<usize>,
    pub decoder_total_elements: Option<usize>,
}
fn limit(message: &str) -> ApiError {
    ApiError::new(ErrorCode::ResourceLimit, message)
}
impl FullLoopResult {
    /// Copies finite independent receiver history without retaining this job or its inputs/stages.
    /// # Arguments
    /// * `policy` - Validated independent per-role counts, decoder prefix and byte ceiling.
    /// # Returns
    /// Detached immutable actual observations. Cell reduction selects original indices
    /// floor(k*N/min(N,cap)); decoder retention is the first actual nonempty event's prefix.
    /// Missing data stays unavailable, original coordinates/context/status stay unchanged,
    /// and clones of the returned owner share no live result or waveform/capture ownership.
    /// # Errors
    /// Rejects byte-accounting overflow or an exceeded budget before copying observations.
    /// An invalid actual protected-report selection preserves its original backend diagnostic.
    pub fn receiver_archive(
        &self,
        policy: FullLoopReceiverArchivePolicy,
    ) -> ApiResult<FullLoopReceiverArchive> {
        let decoder = if policy.settings().decoder_llrs == 0 {
            None
        } else {
            match self.stages.events(FullLoopStage::PayloadLdpcInput) {
                Ok(events) => events.iter().find(|event| {
                    matches!(&event.data, FullLoopStageData::Llrs(values) if !values.is_empty())
                }),
                Err(cause) if cause.code() == ErrorCode::MissingCoverage => None,
                Err(cause) => return Err(cause),
            }
        };
        copy::detach(
            self.job.constellation.as_ref(),
            self.job.protected_constellation.as_ref(),
            self.job.settings.receiver_mode,
            decoder,
            policy,
        )
    }
}
impl FullLoopReceiverArchive {
    /// Copies bounded archive accounting and original decoder context without advancing science.
    /// # Returns
    /// Actual retained counts, correction mode, byte scope and optional original event metadata.
    pub fn info(&self) -> FullLoopReceiverArchiveInfo {
        let retained = &self.0;
        let protected_count = |select: fn(&ProtectedConstellation) -> &ProtectedStream| {
            retained
                .protected
                .as_ref()
                .map_or(0, |report| select(report).pairs.len())
        };
        FullLoopReceiverArchiveInfo {
            policy: retained.policy.settings().clone(),
            receiver_mode: retained.receiver_mode,
            retained_bytes: retained.retained_bytes,
            payload_cells: retained
                .data
                .as_ref()
                .map_or(0, |report| report.pairs.len()),
            ncp_cells: retained
                .data
                .as_ref()
                .map_or(0, |report| report.ncp_pairs.len()),
            plc_data_cells: protected_count(|report| &report.plc_data),
            plc_preamble_cells: protected_count(|report| &report.plc_preamble),
            continuous_pilot_cells: protected_count(|report| &report.continuous_pilots),
            scattered_pilot_cells: protected_count(|report| &report.scattered_pilots),
            decoder_llrs: retained
                .decoder
                .as_ref()
                .map_or(0, |event| event.data.len()),
            decoder_snapshot_index: retained.decoder.as_ref().map(|event| event.retained_index),
            decoder_context: retained.decoder.as_ref().map(|event| event.context.clone()),
            decoder_element_start: retained.decoder.as_ref().map(|event| event.element_start),
            decoder_total_elements: retained.decoder.as_ref().map(|event| event.total_elements),
        }
    }

    /// Queries all-selected scientific MER and a bounded page from this detached history.
    /// # Arguments
    /// * `query` - Same role/profile/order/coordinate/window contract as FullLoopResult.qam.
    /// # Returns
    /// Original archived cells and the same separate reference-plane metrics; no rerun occurs.
    /// # Errors
    /// Preserves the live operation's selector, work, offset and numerical errors.
    pub fn qam(&self, query: FullLoopCellQuery) -> ApiResult<FullLoopQamView> {
        cells::queries::qam_from_reports(
            self.0.data.as_ref(),
            self.0.protected.as_ref(),
            query,
            self.0.receiver_mode,
        )
    }

    /// Queries original physical decisions through the same live scientific owner.
    /// # Arguments
    /// * `query` - Existing cell selection with at most128 decision rows.
    /// # Returns
    /// Original physical/normalized decisions and actual selected-cell likelihood statistics.
    /// # Errors
    /// Preserves the live operation's selection, work and finite-input failures.
    pub fn cell_decisions(&self, query: FullLoopCellQuery) -> ApiResult<FullLoopCellDecisions> {
        cells::queries::decisions_from_reports(
            self.0.data.as_ref(),
            self.0.protected.as_ref(),
            query,
        )
    }

    /// Queries selected actual cell likelihoods independently of archived decoder input.
    /// # Arguments
    /// * `query` - Existing bounded actual role/profile/order/coordinate/window selection.
    /// # Returns
    /// Original post-descrambling cell plane and existing Rust histogram/envelope/statistics.
    /// # Errors
    /// Preserves the live operation's selector/work and finite-observation failures.
    pub fn cell_soft_view(&self, query: FullLoopCellQuery) -> ApiResult<FullLoopCellSoftView> {
        cells::soft::soft_from_reports(self.0.data.as_ref(), self.0.protected.as_ref(), query)
    }

    /// Queries the single actual archived nonempty decoder event with its original coding context.
    /// # Returns
    /// Original stage/context/offset/total and existing diagnostics over the retained prefix.
    /// No physical-cell selection or source truth is substituted for this event.
    /// # Errors
    /// Reports MissingCoverage when omitted/unavailable; preserves original numerical failures.
    pub fn decoder_soft_view(&self) -> ApiResult<FullLoopDecoderSoftView> {
        let event = self.0.decoder.as_ref().ok_or_else(|| {
            ApiError::new(
                ErrorCode::MissingCoverage,
                "No decoder event in receiver archive",
            )
        })?;
        cells::soft::decoder_from_event(event)
    }
}

#[cfg(test)]
mod tests;
