/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Immutable finite actual TX results and viewports; PSD grouping stays with its numerical owner.
use super::*;
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
use serde::Serialize;

/// Maximum actual contiguous samples copied by a single public query.
pub const MAX_TRANSMITTER_PREVIEW_SAMPLES: usize = 4096;

/// Read-only shareable actual idle TX snapshot, independent of later calls and controls.
#[derive(Clone)]
pub struct TransmitterPreview {
    pub(super) inner: Arc<engine::TransmitterPreview>,
    pub(super) key: ScenarioKey,
    pub(super) profile_id: u8,
    pub(super) sources: Vec<CaptureProvenance>,
    pub(super) source: SourceInput,
}

/// Scalar scientific metadata; source/loading remain separate from actual samples.
#[derive(Serialize)]
pub struct TransmitterPreviewInfo {
    pub reference_plane: &'static str,
    pub key: ScenarioKey,
    pub profile_id: u8,
    pub sources: Vec<CaptureProvenance>,
    pub settings: TransmissionSettings,
    pub layout: crate::transmission::layout::CarrierLayoutSettings,
    pub source_kind: &'static str,
    pub source_sha256: Option<String>,
    pub sample_count: usize,
    pub first_sample: u64,
    pub generated_source_bits: u64,
    pub warmup_symbols: usize,
    pub source_exhausted: bool,
}
/// Contiguous actual post-overlap I/Q with absolute sample origin; no downsampling.
#[derive(Serialize)]
pub struct TransmitterSampleView {
    pub first_sample: u64,
    pub samples: Vec<[f64; 2]>,
}

/// Actual centered Welch PSD extrema; frequency axes use original grid metadata.
#[derive(Serialize)]
pub struct TransmitterSpectrum {
    pub reference_plane: &'static str,
    pub density_units: &'static str,
    pub sample_rate_hz: u32,
    pub zero_frequency_hz: u32,
    pub spacing_hz: u32,
    pub fft_samples: usize,
    pub averages: usize,
    pub equivalent_noise_bandwidth_hz: f64,
    pub buckets: Vec<TransmitterSpectrumBucket>,
}
/// Exact original FFT bin span and extrema in dB re one normalized unit squared/Hz.
#[derive(Serialize)]
pub struct TransmitterSpectrumBucket {
    pub first_bin: usize,
    pub last_bin: usize,
    pub minimum_db: Option<f64>,
    pub maximum_db: Option<f64>,
}

impl TransmitterPreview {
    /// Reads owned metadata/source provenance without serializing actual samples.
    /// # Returns
    /// Frozen original geometry and actual source/sample/priming accounting.
    pub fn info(&self) -> TransmitterPreviewInfo {
        let result = &self.inner;
        TransmitterPreviewInfo {
            reference_plane: "tx_post_overlap_before_channel",
            key: self.key.clone(),
            profile_id: self.profile_id,
            sources: self.sources.clone(),
            settings: result.settings.clone(),
            layout: result.layout.clone(),
            source_kind: self.source.kind(),
            source_sha256: self.source.original_sha256().map(str::to_owned),
            sample_count: result.samples.len(),
            first_sample: result.first_sample,
            generated_source_bits: result.generated_source_bits,
            warmup_symbols: result.warmup_symbols,
            source_exhausted: result.source_exhausted,
        }
    }

    /// Copies a bounded contiguous window of actual samples, preserving absolute origin.
    /// # Arguments
    /// * `start` - Offset inside retained samples.
    /// * `count` - Positive count up to 4096; zero selects up to 4096 remaining samples.
    /// # Returns
    /// Independent owned I/Q; no source/generation state changes.
    /// # Errors
    /// Rejects out-of-range/overflow windows and per-query sample budgets.
    pub fn sample_view(&self, start: usize, count: usize) -> ApiResult<TransmitterSampleView> {
        let remaining = self
            .inner
            .samples
            .len()
            .checked_sub(start)
            .filter(|n| *n > 0)
            .ok_or_else(|| {
                super::super::error::invalid("Sample start is outside retained TX output")
            })?;
        let count = if count == 0 {
            remaining.min(MAX_TRANSMITTER_PREVIEW_SAMPLES)
        } else {
            count
        };
        if count > MAX_TRANSMITTER_PREVIEW_SAMPLES {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "TX sample query exceeds 4096 samples",
            ));
        }
        if count > remaining {
            return Err(super::super::error::invalid(
                "Sample window is outside retained TX output",
            ));
        }
        Ok(TransmitterSampleView {
            first_sample: self.inner.first_sample + start as u64,
            samples: self.inner.samples[start..start + count].to_vec(),
        })
    }

    /// Queries the existing PSD min/max grouping without changing retained samples/power.
    /// # Arguments
    /// * `start` - First original centered FFT bin.
    /// * `count` - Positive bin count; zero selects the remaining FFT span.
    /// # Returns
    /// At most 2048 actual extrema buckets and normalization/frequency metadata.
    /// # Errors
    /// Rejects empty/out-of-range/overflow windows through existing spectrum validation.
    pub fn spectrum(&self, start: usize, count: usize) -> ApiResult<TransmitterSpectrum> {
        let spectrum = &self.inner.spectrum;
        let count = if count == 0 {
            spectrum.density.len().checked_sub(start).ok_or_else(|| {
                super::super::error::invalid("Spectrum start is outside retained FFT")
            })?
        } else {
            count
        };
        let buckets = spectrum
            .buckets(start, count)?
            .into_iter()
            .map(|b| TransmitterSpectrumBucket {
                first_bin: b.first_bin,
                last_bin: b.last_bin,
                minimum_db: b.minimum_db,
                maximum_db: b.maximum_db,
            })
            .collect();
        Ok(TransmitterSpectrum {
            reference_plane: "tx_post_overlap_before_channel",
            density_units: "normalized_sample_units_squared_per_hz",
            sample_rate_hz: crate::transmission::settings::SAMPLE_RATE_HZ,
            zero_frequency_hz: spectrum.zero_frequency_hz,
            spacing_hz: spectrum.spacing_hz,
            fft_samples: spectrum.fft_samples,
            averages: spectrum.averages,
            equivalent_noise_bandwidth_hz: spectrum.equivalent_noise_bandwidth_hz,
            buckets,
        })
    }

    /// Measures this complete immutable root and all reachable retained payloads.
    ///
    /// Includes requested spare buffer capacity, private caches and each owned
    /// Arc pointee/edge, conservatively repeating aliases. Ordered entries and
    /// shared roots are separate from payload bytes; allocator, tree-node and
    /// refcount bookkeeping are excluded. No source or numerical state advances.
    ///
    /// # Returns
    /// Passive current-root metadata including its fixed value exactly once.
    /// This does not reserve future output, construction scratch or copied views.
    ///
    /// # Errors
    /// Returns ResourceLimit if retained_footprint arithmetic overflows.
    pub fn retained_footprint(&self) -> ApiResult<merlin_api::RetainedFootprint> {
        merlin_api::retention_support::retained_footprint(self)
            .ok_or_else(|| ApiError::new(ErrorCode::ResourceLimit, "retained_footprint overflow"))
    }
}

impl RetainedPayload for TransmitterPreview {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            inner,
            key,
            profile_id,
            sources,
            source,
        } = self;
        merlin_api::retention_support::sum([
            inner.retained_children(),
            key.retained_children(),
            profile_id.retained_children(),
            sources.retained_children(),
            source.retained_children(),
        ])
    }
}
