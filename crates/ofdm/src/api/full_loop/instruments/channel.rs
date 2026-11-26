/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Calibrated actual channel observations and explicitly derived scalar amplifier views.
use super::*;
use crate::{
    signal_processing::{channel::spectrum as electrical, waveform::bin_power::bin_power},
    transmission::layout::CarrierLayout,
};

/// Fixed legacy channel boundaries; receiver input always uses actual modeled output.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FullLoopChannelTap {
    BeforeAmplifier,
    AfterAmplifier,
    ReceiverInput,
}
/// Scalar amplifier boundary; neither choice runs the physical channel or receiver.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmplifierBoundary {
    Before,
    After,
}

/// Full-band electrical PSD with original source context and explicit scalar derivation.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopChannelSpectrum {
    pub stage: FullLoopStage,
    pub context: FullLoopStageContext,
    pub source_reference_plane: &'static str,
    /// Applied ratio; the source/result reference planes distinguish actual and derived views.
    pub derived_amplitude: f64,
    pub first_sample: u64,
    pub available_samples: usize,
    pub used_samples: usize,
    pub snapshots_used: usize,
    pub power: ChannelSpectrum,
}

/// Original TX bin observation plus an explicitly identified amplifier-only projection.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopAmplifierBins {
    pub boundary: AmplifierBoundary,
    pub reference_plane: &'static str,
    pub amplitude: f64,
    /// Context and stage identify the retained original TX source of the derived values.
    pub observation: FullLoopPhysicalBins,
}

impl FullLoopResult {
    /// Projects actual retained time-stage PSD into accepted electrical or relative units.
    /// # Arguments
    /// * `query` - Existing contiguous time-stage, work and original-bin viewport selection.
    /// # Returns
    /// Owned PSD and full occupied-band power; gain is already present only where observed.
    /// Original normalized queries, retained data and decoder outcomes remain unchanged.
    /// # Errors
    /// Preserves missing samples, grid/work/window and finite calibration failures.
    pub fn calibrated_spectrum(
        &self,
        query: FullLoopSpectrumQuery,
    ) -> ApiResult<FullLoopChannelSpectrum> {
        self.channel_spectrum_view(&query, 1.0, None)
    }

    /// Inspects a fixed channel tap using actual samples or the exact amplifier-only scalar.
    /// # Arguments
    /// * `tap` - Before/after amplifier or actual receiver input, independent of CPA/H/noise.
    /// * `query` - TX overlap stage for amplifier taps, ChannelOutput for receiver input;
    ///   other fields retain the existing finite spectrum selection contract.
    /// # Returns
    /// Owned calibrated/relative PSD with direct source metadata and explicit derived ratio.
    /// No channel or transmitter is executed and no missing observation is substituted.
    /// # Errors
    /// Rejects mismatched stages and existing sample/grid/work/calibration/overflow errors.
    pub fn channel_spectrum(
        &self,
        tap: FullLoopChannelTap,
        query: FullLoopSpectrumQuery,
    ) -> ApiResult<FullLoopChannelSpectrum> {
        let expected_stage = match tap {
            FullLoopChannelTap::ReceiverInput => FullLoopStage::ChannelOutput,
            _ => FullLoopStage::TxOverlap,
        };
        if query.stage != expected_stage {
            return Err(ApiError::new(
                ErrorCode::InvalidInput,
                "Channel tap and source stage differ",
            ));
        }
        let (amplitude, reference) = match tap {
            FullLoopChannelTap::AfterAmplifier => (
                electrical::amplifier_amplitude(&self.job.settings.impairments.power)?,
                "after_amplifier_before_cpa_and_physical_channel",
            ),
            FullLoopChannelTap::BeforeAmplifier => (1.0, "before_amplifier"),
            FullLoopChannelTap::ReceiverInput => (1.0, "channel_output_receiver_input"),
        };
        self.channel_spectrum_view(&query, amplitude, Some(reference))
    }

    /// Copies actual TX physical bins at the selected scalar amplifier boundary.
    /// # Arguments
    /// * `boundary` - Before or after the accepted amplifier alone, before CPA/H/noise.
    /// * `snapshot` - Actual retained TX physical-bin event.
    /// * `start`, `count` - Existing at-most-one-FFT viewport; zero count selects the remainder.
    /// # Returns
    /// Owned original roles/context/missing values and exact direct or scalar-derived I/Q/power.
    /// Received QAM remains a separate actual receiver-cell operation.
    /// # Errors
    /// Preserves original observation/viewport failures and rejects invalid/overflowing scalars.
    pub fn amplifier_bins(
        &self,
        boundary: AmplifierBoundary,
        snapshot: usize,
        start: usize,
        count: usize,
    ) -> ApiResult<FullLoopAmplifierBins> {
        let mut observation =
            self.physical_bins(FullLoopStage::PhysicalBins, snapshot, start, count)?;
        let (amplitude, reference_plane) = match boundary {
            AmplifierBoundary::Before => (1.0, "before_amplifier"),
            AmplifierBoundary::After => (
                electrical::amplifier_amplitude(&self.job.settings.impairments.power)?,
                "after_amplifier_before_cpa_and_physical_channel",
            ),
        };
        if boundary == AmplifierBoundary::After {
            for bin in &mut observation.bins {
                bin.iq = bin
                    .iq
                    .map(|iq| electrical::scale_iq(iq, amplitude))
                    .transpose()?;
                (bin.linear_power, bin.power_db) = bin_power(bin.iq)?;
            }
        }
        Ok(FullLoopAmplifierBins {
            boundary,
            reference_plane,
            amplitude,
            observation,
        })
    }

    fn channel_spectrum_view(
        &self,
        query: &FullLoopSpectrumQuery,
        amplitude: f64,
        reference_plane: Option<&'static str>,
    ) -> ApiResult<FullLoopChannelSpectrum> {
        let prepared = self.prepare_spectrum(query, amplitude)?;
        let power = self.calibrated_power(
            &prepared.power,
            &prepared.context,
            reference_plane.unwrap_or(prepared.reference_plane),
            query.start,
            query.count,
        )?;
        Ok(FullLoopChannelSpectrum {
            stage: query.stage,
            context: prepared.context,
            source_reference_plane: prepared.reference_plane,
            derived_amplitude: amplitude,
            first_sample: prepared.first_sample,
            available_samples: prepared.available_samples,
            used_samples: prepared.used_samples,
            snapshots_used: prepared.snapshots_used,
            power,
        })
    }

    /// Reuses accepted grid/layout/electrical admission for either existing sampling basis.
    /// # Arguments
    /// * `estimate`, `context` - Original full-resolution PSD and its retained physical grid.
    /// * `reference_plane` - Existing source/derived boundary, independent of sampling basis.
    /// * `start`, `count` - Original physical-bin viewport; zero count selects the remaining span.
    /// # Returns
    /// Bounded calibrated/relative spectrum and full occupied-band power from the existing owner.
    /// # Errors
    /// Preserves layout, original-grid, calibration, viewport and numerical failures.
    pub(super) fn calibrated_power(
        &self,
        estimate: &crate::signal_processing::waveform::spectrum::PowerSpectrum,
        context: &FullLoopStageContext,
        reference_plane: &'static str,
        start: usize,
        count: usize,
    ) -> ApiResult<ChannelSpectrum> {
        let settings = &self.job.settings;
        let layout = CarrierLayout::new(&settings.transmission, &settings.layout)?;
        let grid = context.physical_grid.ok_or_else(|| {
            ApiError::new(
                ErrorCode::MissingCoverage,
                "Channel spectrum has no original grid",
            )
        })?;
        if grid.spacing != settings.transmission.subcarrier_spacing
            || grid.zero_frequency_hz != settings.layout.zero_frequency_hz
        {
            return Err(ApiError::new(
                ErrorCode::IncompatibleGrid,
                "Channel spectrum original grid differs",
            ));
        }
        Ok(electrical::view(
            estimate,
            &layout,
            &settings.impairments.power,
            reference_plane,
            start,
            count,
        )?)
    }
}
