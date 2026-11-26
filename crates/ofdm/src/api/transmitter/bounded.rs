/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Pre-production idle preview targets; scientific generation remains in its existing owner.
use super::*;
use crate::{
    api::full_loop::result_retention::normalize,
    signal_processing::waveform::spectrum::{
        PowerSpectrum, MAX_SPECTRUM_AVERAGES, WELCH_HOP_DIVISOR,
    },
    transmission::layout::CarrierLayoutSettings,
    types::IqPair,
};
use merlin_api::retention_support::sum;
use std::mem::size_of;

struct Prepared {
    settings: TransmissionSettings,
    layout: CarrierLayoutSettings,
    key: ScenarioKey,
    profile_id: u8,
    sources: Vec<CaptureProvenance>,
    source: SourceInput,
    samples: usize,
    fft_samples: usize,
}

fn resource_limit() -> ApiError {
    ApiError::new(
        ErrorCode::ResourceLimit,
        "Idle TX retained output bound overflow or exhaustion",
    )
}

fn sample_count(fft_samples: usize, averages: usize) -> ApiResult<usize> {
    averages
        .checked_sub(1)
        .and_then(|preceding| preceding.checked_mul(fft_samples / WELCH_HOP_DIVISOR))
        .and_then(|extra| fft_samples.checked_add(extra))
        .ok_or_else(resource_limit)
}

fn admit(required: RetainedFootprint, limit: RetainedFootprint) -> ApiResult<()> {
    if required.payload_bytes > limit.payload_bytes
        || required.ordered_entries > limit.ordered_entries
        || required.shared_roots > limit.shared_roots
    {
        return Err(resource_limit());
    }
    Ok(())
}

impl Prepared {
    fn new(request: &TransmitterPreviewRequest) -> Self {
        let (settings, mut layout) = request.request.retained_output_snapshot();
        normalize::layout(&mut layout);
        let mut sources = request.sources.clone();
        for source in &mut sources {
            source.normalize_result_storage();
        }
        normalize::vector(&mut sources);
        let mut source = request.source.clone();
        source.normalize_result_storage();
        Self {
            fft_samples: settings.subcarrier_spacing.fft_samples() as usize,
            settings,
            layout,
            key: request.key.clone(),
            profile_id: request.profile_id,
            sources,
            source,
            samples: 0,
        }
    }

    fn bound(&mut self) -> ApiResult<RetainedFootprint> {
        self.samples = sample_count(self.fft_samples, MAX_SPECTRUM_AVERAGES)?;
        let sample_bytes = self
            .samples
            .checked_mul(size_of::<IqPair>())
            .ok_or_else(resource_limit)?;
        let density_bytes = self
            .fft_samples
            .checked_mul(size_of::<f64>())
            .ok_or_else(resource_limit)?;
        let payload_bytes = size_of::<TransmitterPreview>()
            .checked_add(size_of::<engine::TransmitterPreview>())
            .and_then(|fixed| fixed.checked_add(sample_bytes))
            .and_then(|total| total.checked_add(density_bytes))
            .ok_or_else(resource_limit)?;
        sum([
            Some(RetainedFootprint {
                payload_bytes,
                ordered_entries: 0,
                shared_roots: 1, // The new preview's one private engine Arc.
            }),
            self.settings.retained_children(),
            self.layout.retained_children(),
            self.key.retained_children(),
            self.sources.retained_children(),
            self.source.retained_children(),
        ])
        .ok_or_else(resource_limit)
    }

    fn finish(self, generated: engine::TransmitterPreview) -> TransmitterPreview {
        let engine::TransmitterPreview {
            settings: _,
            layout: _,
            mut samples,
            first_sample,
            generated_source_bits,
            warmup_symbols,
            source_exhausted,
            mut spectrum,
        } = generated;
        normalize::vector(&mut samples);
        let PowerSpectrum {
            zero_frequency_hz: _,
            spacing_hz: _,
            fft_samples: _,
            averages: _,
            equivalent_noise_bandwidth_hz: _,
            density,
        } = &mut spectrum;
        normalize::vector(density);
        debug_assert!(samples.len() <= self.samples);
        debug_assert_eq!(density.len(), self.fft_samples);
        TransmitterPreview {
            inner: Arc::new(engine::TransmitterPreview {
                settings: self.settings,
                layout: self.layout,
                samples,
                first_sample,
                generated_source_bits,
                warmup_symbols,
                source_exhausted,
                spectrum,
            }),
            key: self.key,
            profile_id: self.profile_id,
            sources: self.sources,
            source: self.source,
        }
    }
}

impl TransmitterPreviewRequest {
    /// Prepares a checked retained target bound without generating a waveform.
    ///
    /// # Returns
    /// Target-layout fixed and exact-capacity payload bytes, ordered entries and
    /// shared edges for the paired bounded operation. Uses the existing FFT/
    /// Welch-window policy and normalizes finite cloned input snapshots; source,
    /// RNG, controls and scientific state do not advance. This may allocate finite
    /// preparation snapshots, governed by existing input limits. The quantities
    /// exclude temporary construction/numerical scratch and consumer view copies.
    /// Callers reserve accepted/request/old/queued owners separately.
    ///
    /// # Errors
    /// ResourceLimit on checked arithmetic overflow; no underestimated success.
    pub fn preview_retained_bound(&self) -> ApiResult<RetainedFootprint> {
        Prepared::new(self).bound()
    }

    /// Generates a capacity-controlled idle target after every retained limit passes.
    ///
    /// # Arguments
    /// * `control` - Existing fresh one-shot preview progress/cancellation token.
    /// * `limit` - Caller-reserved maximum target bytes, ordered entries and shared edges.
    ///
    /// # Returns
    /// Owned original waveform/PSD/provenance/source snapshot. Final capacities
    /// are enforced before publication; source and old products stay independently
    /// charged. The original scientific generator and legacy control dispatch run
    /// unchanged. Temporary builders/scratch have separate finite execution limits.
    /// Passing quantities does not itself reserve application resources.
    ///
    /// # Errors
    /// ResourceLimit before control/source progress for insufficient dimensions or
    /// arithmetic overflow. Preserves legacy Busy, cancellation and engine errors;
    /// cancellation produces no replacement preview and leaves old owners intact.
    pub fn generate_bounded(
        &self,
        control: &TransmitterPreviewControl,
        limit: RetainedFootprint,
    ) -> ApiResult<TransmitterPreview> {
        let mut prepared = Prepared::new(self);
        let required = prepared.bound()?;
        admit(required, limit)?;
        let generated = self.generate_owned(control)?;
        let preview = prepared.finish(generated);
        debug_assert!(preview.retained_footprint().is_ok_and(|actual| {
            actual.payload_bytes <= required.payload_bytes
                && actual.ordered_entries <= required.ordered_entries
                && actual.shared_roots <= required.shared_roots
        }));
        Ok(preview)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_three_dimensions_and_each_insufficient_unit_are_checked() {
        // Independent artificial quantities exercise each limit, not PHY populations.
        let required = RetainedFootprint {
            payload_bytes: 17,
            ordered_entries: 5,
            shared_roots: 3,
        };
        admit(required, required).unwrap();
        for limit in [
            RetainedFootprint {
                payload_bytes: required.payload_bytes - 1,
                ..required
            },
            RetainedFootprint {
                ordered_entries: required.ordered_entries - 1,
                ..required
            },
            RetainedFootprint {
                shared_roots: required.shared_roots - 1,
                ..required
            },
        ] {
            assert_eq!(
                admit(required, limit).unwrap_err().code(),
                ErrorCode::ResourceLimit
            );
        }
    }

    #[test]
    fn welch_count_uses_checked_arithmetic_and_existing_hop_policy() {
        // Independent eight-sample FFT: four-sample hops across three windows need16.
        assert_eq!(sample_count(8, 3).unwrap(), 16);
        assert_eq!(
            sample_count(usize::MAX, usize::MAX).unwrap_err().code(),
            ErrorCode::ResourceLimit
        );
        assert_eq!(
            sample_count(8, 0).unwrap_err().code(),
            ErrorCode::ResourceLimit
        );
    }
}
