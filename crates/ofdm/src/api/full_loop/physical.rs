/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual retained FFT-bin inspection; role geometry and power retain scientific owners.
use super::*;
use crate::{
    signal_processing::waveform::bin_power::bin_power,
    transmission::layout::{view::layout_view, CarrierLayout},
};
use serde::Serialize;

/// Original complex bin and instantaneous squared magnitude; missing and zero stay distinct.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopPhysicalBin {
    pub physical_bin: u16,
    pub iq: Option<[f64; 2]>,
    pub linear_power: Option<f64>,
    /// Relative power, not spectral density; known zero has no finite dB value.
    pub power_db: Option<f64>,
}
/// Bounded actual bin window with original observation context and phase-specific layout.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopPhysicalBins {
    pub stage: FullLoopStage,
    pub retained_index: usize,
    pub context: FullLoopStageContext,
    pub element_start: usize,
    pub total_elements: usize,
    pub retained_elements: usize,
    pub power_units: &'static str,
    pub layout: PhysicalLayoutView,
    pub bins: Vec<FullLoopPhysicalBin>,
}

impl FullLoopResult {
    /// Inspects actual retained TX physical bins or raw RX FFT bins without another FFT.
    /// # Arguments
    /// * `stage` - PhysicalBins or RxFft; time-domain stages are rejected.
    /// * `snapshot` - Actual retained event index for this stage.
    /// * `start` - Offset into retained event elements, preserving the original absolute origin.
    /// * `count` - Positive count, or zero for remaining retained values, at most 8192.
    /// # Returns
    /// Owned actual I/Q, instantaneous bin power, original context and physical-role geometry.
    /// Layout Data does not claim payload/NCP/filler identity; absent history remains absent.
    /// # Errors
    /// Rejects unsupported stages, unavailable events, invalid windows, inconsistent grids,
    /// missing original symbol context and nonfinite/overflowing actual power.
    pub fn physical_bins(
        &self,
        stage: FullLoopStage,
        snapshot: usize,
        start: usize,
        count: usize,
    ) -> ApiResult<FullLoopPhysicalBins> {
        if !matches!(stage, FullLoopStage::PhysicalBins | FullLoopStage::RxFft) {
            return Err(ApiError::new(
                ErrorCode::UnsupportedCapability,
                "Physical bin query requires actual TX bins or raw RX FFT",
            ));
        }
        super::super::physical::check_count(count)?;
        let event = self.stages.events(stage)?.get(snapshot).ok_or_else(|| {
            ApiError::new(
                ErrorCode::InvalidIdentifier,
                "Retained physical event is unavailable",
            )
        })?;
        let remaining = event
            .data
            .len()
            .checked_sub(start)
            .filter(|n| *n > 0)
            .ok_or_else(|| {
                ApiError::new(
                    ErrorCode::InvalidInput,
                    "Physical query start is outside retained values",
                )
            })?;
        let count = if count == 0 { remaining } else { count };
        super::super::physical::check_count(count)?;
        if count > remaining {
            return Err(ApiError::new(
                ErrorCode::InvalidInput,
                "Physical query window is outside retained values",
            ));
        }
        let first = event.element_start.checked_add(start).ok_or_else(|| {
            ApiError::new(ErrorCode::InvalidInput, "Physical query origin overflows")
        })?;
        let symbol = event.context.channel_symbol_index.ok_or_else(|| {
            ApiError::new(
                ErrorCode::MissingCoverage,
                "Physical observation has no original channel symbol",
            )
        })?;
        let settings = &self.job.settings;
        let layout = CarrierLayout::new(&settings.transmission, &settings.layout)?;
        let grid = event.context.physical_grid.as_ref().ok_or_else(|| {
            ApiError::new(
                ErrorCode::MissingCoverage,
                "Physical observation has no original grid",
            )
        })?;
        if grid.spacing != settings.transmission.subcarrier_spacing
            || grid.zero_frequency_hz != settings.layout.zero_frequency_hz
            || event.total_elements != layout.static_roles().len()
        {
            return Err(ApiError::new(
                ErrorCode::IncompatibleGrid,
                "Physical observation differs from its accepted job layout",
            ));
        }
        let layout = layout_view(&layout, symbol, first, count)?;
        let bins = (start..start + count)
            .map(|index| {
                let iq = match &event.data {
                    FullLoopStageData::OptionalIq(values)
                        if stage == FullLoopStage::PhysicalBins =>
                    {
                        values[index]
                    }
                    FullLoopStageData::Iq(values) if stage == FullLoopStage::RxFft => {
                        Some(values[index])
                    }
                    _ => {
                        return Err(ApiError::new(
                            ErrorCode::InvalidInput,
                            "Physical observation has incompatible data kind",
                        ))
                    }
                };
                let (linear_power, power_db) = bin_power(iq)?;
                Ok(FullLoopPhysicalBin {
                    physical_bin: (event.element_start + index) as u16,
                    iq,
                    linear_power,
                    power_db,
                })
            })
            .collect::<ApiResult<Vec<_>>>()?;
        Ok(FullLoopPhysicalBins {
            stage,
            retained_index: snapshot,
            context: event.context.clone(),
            element_start: first,
            total_elements: event.total_elements,
            retained_elements: event.data.len(),
            power_units: "normalized_bin_power",
            layout,
            bins,
        })
    }
}
