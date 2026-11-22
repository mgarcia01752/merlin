/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit ordered-frame diagnostics; original physical time remains visible in every segment.
use super::*;
pub use crate::signal_processing::waveform::stage_sequence::{
    StageSequenceSegment as FullLoopSpectrumSegment, StageSequenceStop as FullLoopSpectrumStop,
};
use crate::signal_processing::waveform::{
    spectrum as owner,
    stage_sequence::{collect_sequence, StageSequenceFrame},
};

/// Normalized stage-sequence PSD, distinct from a physically continuous waveform observation.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopSequenceSpectrum {
    pub stage: FullLoopStage,
    pub sampling_basis: &'static str,
    pub context: FullLoopStageContext,
    pub physical_stride_samples: usize,
    pub physically_contiguous: bool,
    pub available_samples: usize,
    pub used_samples: usize,
    pub segments: Vec<FullLoopSpectrumSegment>,
    pub stop: FullLoopSpectrumStop,
    pub power: TransmitterSpectrum,
}

impl FullLoopResult {
    /// Analyzes the explicit diagnostic sequence of consecutive retained frame-stage samples.
    /// # Arguments
    /// * `query` - TxIfft, TxPrefix, TxWindow or RxUseful; actual event/window and 1..32 averages.
    /// # Returns
    /// Owned normalized Welch PSD with original per-segment coordinates and explicit stop reason.
    /// This operation omits no retained samples within joined frames, but the diagnostic sequence
    /// can have original physical gaps or overlaps; it never claims a continuous waveform.
    /// # Errors
    /// Rejects wrong stages, missing context/events, invalid geometry/work/viewports, overflow
    /// or fewer than N retained samples. Incompatible/later incomplete history is never filled.
    pub fn spectrum_sequence(
        &self,
        query: FullLoopSpectrumQuery,
    ) -> ApiResult<FullLoopSequenceSpectrum> {
        let reference_plane = match query.stage {
            FullLoopStage::TxIfft => "tx_after_ifft_before_prefix",
            FullLoopStage::TxPrefix => "tx_after_prefix_before_window",
            FullLoopStage::TxWindow => "tx_after_window_before_overlap",
            FullLoopStage::RxUseful => "receiver_after_prefix_removal_before_fft",
            _ => {
                return Err(ApiError::new(
                    ErrorCode::UnsupportedCapability,
                    "Ordered-frame PSD requires IFFT, prefix, window or RX useful frames",
                ))
            }
        };
        if !(1..=owner::MAX_SPECTRUM_AVERAGES).contains(&query.max_averages) {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "PSD averages require 1..32",
            ));
        }
        let first = selected(self, query.stage, query.snapshot)?;
        let grid = first.context.physical_grid.ok_or_else(|| {
            ApiError::new(
                ErrorCode::MissingCoverage,
                "Sequence has no original physical grid",
            )
        })?;
        let waveform = first.context.waveform_grid.as_ref().ok_or_else(|| {
            ApiError::new(
                ErrorCode::MissingCoverage,
                "Sequence has no original waveform geometry",
            )
        })?;
        if first.context.channel_symbol_index.is_none()
            || first.context.sample_start_index.is_none()
        {
            return Err(ApiError::new(
                ErrorCode::MissingCoverage,
                "Sequence has no original symbol/sample origin",
            ));
        }
        let geometry = &waveform.geometry;
        if geometry.settings() != &self.job.settings.transmission
            || grid.spacing != geometry.settings().subcarrier_spacing
            || waveform.sample_rate_hz != crate::transmission::settings::SAMPLE_RATE_HZ
        {
            return Err(ApiError::new(
                ErrorCode::IncompatibleGrid,
                "Sequence geometry differs from its accepted job",
            ));
        }
        let frame_length = match query.stage {
            FullLoopStage::TxIfft | FullLoopStage::RxUseful => geometry.fft_samples(),
            FullLoopStage::TxPrefix => geometry.stride_samples(),
            _ => geometry.extended_samples(),
        };
        if first.total_elements != frame_length || !matches!(first.data, FullLoopStageData::Iq(_)) {
            return Err(ApiError::new(
                ErrorCode::IncompatibleGrid,
                "Sequence first event is not the original frame shape",
            ));
        }
        let frames = self.stages.events(query.stage)?[query.snapshot..]
            .iter()
            .map(|event| {
                let data = match &event.data {
                    FullLoopStageData::Iq(values) => Some(values.as_slice()),
                    _ => None,
                };
                let compatible = event.context.physical_grid == first.context.physical_grid
                    && event.context.waveform_grid == first.context.waveform_grid
                    && event.context.reference == first.context.reference
                    && event.context.phase == first.context.phase
                    && event.context.run_id == first.context.run_id
                    && event.context.configuration_id == first.context.configuration_id
                    && event.context.channel_symbol_index.is_some()
                    && event.context.sample_start_index.is_some()
                    && event.total_elements == frame_length
                    && data.is_some();
                StageSequenceFrame {
                    retained_index: event.retained_index,
                    channel_symbol: event.context.channel_symbol_index.unwrap_or(0),
                    frame_start: event.context.sample_start_index.unwrap_or(0),
                    element_start: event.element_start,
                    total_elements: event.total_elements,
                    samples: data.unwrap_or(&[]),
                    context_compatible: compatible,
                }
            });
        let length = geometry.fft_samples();
        let mut sequence = collect_sequence(
            frames,
            geometry.stride_samples(),
            length + (query.max_averages - 1) * (length / owner::WELCH_HOP_DIVISOR),
        )?;
        let power = owner::estimate(grid.spacing, grid.zero_frequency_hz, &sequence.samples)?;
        let used_samples = length + (power.averages - 1) * (length / owner::WELCH_HOP_DIVISOR);
        for segment in &mut sequence.segments {
            segment.analyzed_samples = used_samples
                .saturating_sub(segment.sequence_start)
                .min(segment.sample_count);
        }
        Ok(FullLoopSequenceSpectrum {
            stage: query.stage,
            sampling_basis: "ordered_frame_stage_samples",
            context: first.context.clone(),
            physical_stride_samples: geometry.stride_samples(),
            physically_contiguous: sequence.physically_contiguous,
            available_samples: sequence.samples.len(),
            used_samples,
            segments: sequence.segments,
            stop: sequence.stop,
            power: super::spectrum::spectrum_view(&power, &query, reference_plane)?,
        })
    }
}
