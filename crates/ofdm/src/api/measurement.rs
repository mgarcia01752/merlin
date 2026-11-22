/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Public original-capture previews delegate projections and bucketing to existing owners.
use super::{ApiResult, CaptureProvenance, ChannelEstimate, RxMer};
use crate::scenario::{
    measurement_view::{measurement_view, response_value, ResponseMetric},
    FrequencyGrid,
};
use serde::Serialize;

pub use crate::scenario::measurement_view::{
    AxisTick, MeasurementPoint, MeasurementView, MAX_MEASUREMENT_POINTS,
};

/// Original source offsets; zero count requests the complete grid from its origin.
#[derive(Clone, Copy, Default)]
pub struct MeasurementViewport {
    /// Zero-based original array offset; ignored for the complete-grid request.
    pub start: usize,
    /// Number of original carriers before bounded min/max grouping; zero means all.
    pub count: usize,
}

/// Explicit projection of captured H, distinct from a receiver's equalizer response.
#[derive(Clone, Copy)]
pub enum ChannelResponseMetric {
    /// Magnitude in dB; zero magnitude remains unavailable.
    GainDb,
    /// Wrapped phase in degrees, using the existing complex projection owner.
    PhaseDegrees,
    /// Original dimensionless real component.
    Real,
    /// Original dimensionless imaginary component.
    Imaginary,
}

/// Owned bounded view with original identity; retaining it never runs a simulation.
#[derive(Serialize)]
pub struct MeasurementPreview {
    /// At most 1024 min/max groups; gaps and units retain existing viewer semantics.
    pub view: MeasurementView,
    /// Unchanged full-resolution grid, independent of the selected viewport.
    pub grid: FrequencyGrid,
    /// Original identity/time/hash, retaining original bytes in immutable Rust ownership.
    pub provenance: CaptureProvenance,
    /// Explicit original-observation projection, never simulated BER/FEC or equalizer gain.
    pub projection: &'static str,
}

impl RxMer {
    /// Previews original measured RxMER without averaging, replay or source advancement.
    ///
    /// # Arguments
    /// * `viewport` - Checked by the existing owner against the original sample array.
    ///
    /// # Returns
    /// Owned bounded dB view with exact original grid/provenance and missing samples.
    ///
    /// # Errors
    /// Preserves invalid/overflowing viewport and unrepresentable plotting-range errors.
    pub fn preview(&self, viewport: MeasurementViewport) -> ApiResult<MeasurementPreview> {
        Ok(MeasurementPreview {
            view: measurement_view(
                &self.frame.key.grid,
                &self.frame.mer_db,
                viewport.start,
                viewport.count,
                "RxMER (dB)",
            )?,
            grid: self.frame.key.grid.clone(),
            provenance: self.provenance.clone(),
            projection: "rxmer_db",
        })
    }
}

impl ChannelEstimate {
    /// Previews an explicit original complex H projection without inverse/equalizer semantics.
    ///
    /// # Arguments
    /// * `viewport` - Original sample offsets; zero count selects the complete grid.
    /// * `metric` - Magnitude dB, wrapped degrees, real or imaginary H component.
    ///
    /// # Returns
    /// Owned bounded view; missing/nonfinite projections remain explicit gaps.
    ///
    /// # Errors
    /// Preserves existing grid, viewport-overflow and representable-range admission errors.
    pub fn preview(
        &self,
        viewport: MeasurementViewport,
        metric: ChannelResponseMetric,
    ) -> ApiResult<MeasurementPreview> {
        let (metric, projection, units) = match metric {
            ChannelResponseMetric::GainDb => (ResponseMetric::GainDb, "gain_db", "Gain (dB)"),
            ChannelResponseMetric::PhaseDegrees => (
                ResponseMetric::PhaseDegrees,
                "phase_degrees",
                "Phase (degrees)",
            ),
            ChannelResponseMetric::Real => (ResponseMetric::Real, "real", "Real"),
            ChannelResponseMetric::Imaginary => {
                (ResponseMetric::Imaginary, "imaginary", "Imaginary")
            }
        };
        let values = self
            .frame
            .iq
            .iter()
            .map(|sample| response_value(*sample, metric))
            .collect::<Vec<_>>();
        Ok(MeasurementPreview {
            view: measurement_view(
                &self.frame.key.grid,
                &values,
                viewport.start,
                viewport.count,
                units,
            )?,
            grid: self.frame.key.grid.clone(),
            provenance: self.provenance.clone(),
            projection,
        })
    }
}
