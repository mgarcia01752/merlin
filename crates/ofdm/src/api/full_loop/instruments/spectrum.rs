/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Original time-domain continuity and the existing normalized Welch PSD owner.
use super::*;
use crate::signal_processing::waveform::spectrum as owner;

/// Time-boundary selection and original centered FFT-bin viewport.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FullLoopSpectrumQuery {
    pub stage: FullLoopStage,
    pub snapshot: usize,
    pub max_averages: usize,
    pub start: usize,
    /// Zero selects the remaining FFT span, with at most 2048 min/max buckets.
    pub count: usize,
}
impl Default for FullLoopSpectrumQuery {
    /// Selects actual TX post-overlap samples and at most 32 overlapping analysis windows.
    /// # Returns
    /// A passive query; no source, transmitter, channel or receiver is executed.
    fn default() -> Self {
        Self {
            stage: FullLoopStage::TxOverlap,
            snapshot: 0,
            max_averages: owner::MAX_SPECTRUM_AVERAGES,
            start: 0,
            count: 0,
        }
    }
}
/// Actual original sample accounting and normalized PSD over one contiguous retained span.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopSpectrum {
    pub stage: FullLoopStage,
    pub context: FullLoopStageContext,
    pub first_sample: u64,
    pub available_samples: usize,
    pub used_samples: usize,
    pub snapshots_used: usize,
    pub power: TransmitterSpectrum,
}

impl FullLoopResult {
    /// Estimates the existing Welch PSD from actual retained contiguous time samples.
    /// # Arguments
    /// * `query` - Actual time boundary, first event, 1..32 averages and FFT-bin viewport.
    /// # Returns
    /// Owned bounded extrema, normalization/grid/reference metadata and true sample accounting.
    /// Adjacent overlap-output/channel/framed events join only across exact contiguous origins
    /// with matching geometry, phase and reference; other boundaries use one actual event.
    /// # Errors
    /// Rejects frequency-domain stages, invalid work/viewports, unavailable or fewer than N
    /// retained samples. Gaps stop accumulation; no zero padding, interpolation or hidden run.
    pub fn spectrum(&self, query: FullLoopSpectrumQuery) -> ApiResult<FullLoopSpectrum> {
        let reference_plane = match query.stage {
            FullLoopStage::TxIfft => "tx_after_ifft_before_prefix",
            FullLoopStage::TxPrefix => "tx_after_prefix_before_window",
            FullLoopStage::TxWindow => "tx_after_window_before_overlap",
            FullLoopStage::TxOverlap => "tx_post_overlap_before_channel",
            FullLoopStage::ChannelInput => "channel_input",
            FullLoopStage::ChannelOutput => "channel_output_receiver_input",
            FullLoopStage::RxFramed => "receiver_framed_before_prefix_removal",
            FullLoopStage::RxUseful => "receiver_after_prefix_removal_before_fft",
            _ => {
                return Err(ApiError::new(
                    ErrorCode::UnsupportedCapability,
                    "PSD requires an actual time-domain stage",
                ));
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
            super::super::super::error::invalid("Retained PSD has no original physical grid")
        })?;
        let first_sample = first
            .context
            .sample_start_index
            .and_then(|start| start.checked_add(first.element_start as u64))
            .ok_or_else(|| {
                super::super::super::error::invalid("Retained PSD has no sample origin")
            })?;
        let length = grid.spacing.fft_samples() as usize;
        let limit = length + (query.max_averages - 1) * (length / 2);
        let join = matches!(
            query.stage,
            FullLoopStage::TxOverlap
                | FullLoopStage::ChannelInput
                | FullLoopStage::ChannelOutput
                | FullLoopStage::RxFramed
        );
        let mut samples = Vec::with_capacity(limit);
        let mut snapshots_used = 0;
        for event in &self.stages.events(query.stage)?[query.snapshot..] {
            let origin = event
                .context
                .sample_start_index
                .and_then(|start| start.checked_add(event.element_start as u64));
            if origin != first_sample.checked_add(samples.len() as u64)
                || event.context.physical_grid != first.context.physical_grid
                || event.context.waveform_grid != first.context.waveform_grid
                || event.context.reference != first.context.reference
                || event.context.phase != first.context.phase
                || event.context.run_id != first.context.run_id
                || event.context.configuration_id != first.context.configuration_id
            {
                break;
            }
            let FullLoopStageData::Iq(values) = &event.data else {
                return Err(super::super::super::error::invalid(
                    "Retained time stage is not I/Q",
                ));
            };
            samples.extend_from_slice(&values[..values.len().min(limit - samples.len())]);
            snapshots_used += 1;
            if !join || samples.len() == limit {
                break;
            }
        }
        let spectrum = owner::estimate(grid.spacing, grid.zero_frequency_hz, &samples)?;
        let count = if query.count == 0 {
            length.checked_sub(query.start).ok_or_else(|| {
                super::super::super::error::invalid("PSD viewport starts outside FFT")
            })?
        } else {
            query.count
        };
        let buckets = spectrum
            .buckets(query.start, count)?
            .into_iter()
            .map(|bucket| TransmitterSpectrumBucket {
                first_bin: bucket.first_bin,
                last_bin: bucket.last_bin,
                minimum_db: bucket.minimum_db,
                maximum_db: bucket.maximum_db,
            })
            .collect();
        Ok(FullLoopSpectrum {
            stage: query.stage,
            context: first.context.clone(),
            first_sample,
            available_samples: samples.len(),
            used_samples: length + (spectrum.averages - 1) * (length / 2),
            snapshots_used,
            power: TransmitterSpectrum {
                reference_plane,
                density_units: "normalized_sample_units_squared_per_hz",
                sample_rate_hz: crate::transmission::settings::SAMPLE_RATE_HZ,
                zero_frequency_hz: spectrum.zero_frequency_hz,
                spacing_hz: spectrum.spacing_hz,
                fft_samples: spectrum.fft_samples,
                averages: spectrum.averages,
                equivalent_noise_bandwidth_hz: spectrum.equivalent_noise_bandwidth_hz,
                buckets,
            },
        })
    }
}
