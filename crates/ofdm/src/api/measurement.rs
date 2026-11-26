/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Public original-capture previews delegate projections and bucketing to existing owners.
use super::{ApiResult, CaptureProvenance, ChannelEstimate, ProfileCapture, RxMer};
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

/// Configured synthetic response on an original source grid; never a measured capture.
#[derive(Serialize)]
#[non_exhaustive]
pub struct ConfiguredResponsePreview {
    /// Original byte-sized source identity, independent of simulation-profile admission.
    pub profile_id: u8,
    /// Exact source grid, without resampling or inferred numerology compatibility.
    pub grid: FrequencyGrid,
    /// Existing bounded projection with missing/zero-value semantics preserved.
    pub view: MeasurementView,
    /// Explicit gain/phase/real/imaginary selection.
    pub projection: &'static str,
    /// Declared synthetic reference; no capture epoch/hash or received H-hat is invented.
    pub reference_plane: &'static str,
}

impl ProfileCapture {
    /// Previews explicit unity H on an original catalog grid without creating a capture.
    /// # Arguments
    /// * `profile_id` - Original byte-sized source identity, never a run-ID coercion.
    /// * `viewport` - Original offsets; zero count selects the complete source grid.
    /// * `metric` - Existing gain, wrapped phase, real or imaginary response projection.
    /// # Returns
    /// Bounded synthetic-unity view with original grid/identity and explicit reference.
    /// Unity applies to every source position, including excluded profile positions;
    /// it does not claim a measured response, receiver estimate or compatible PHY run.
    /// No source, simulation, original bytes or stored report changes.
    /// # Errors
    /// InvalidIdentifier for an absent source; existing grid/viewport/range errors otherwise.
    pub fn unity_response_preview(
        &self,
        profile_id: u8,
        viewport: MeasurementViewport,
        metric: ChannelResponseMetric,
    ) -> ApiResult<ConfiguredResponsePreview> {
        let grid = self.profile_ref(profile_id)?.key.grid.clone();
        let (metric, projection, units) = projection_parts(metric);

        // The multiplicative complex identity is H = 1 + j0, with no missing locations.
        let values = vec![response_value(Some([1.0, 0.0]), metric); grid.sample_count];
        let view = measurement_view(&grid, &values, viewport.start, viewport.count, units)?;
        Ok(ConfiguredResponsePreview {
            profile_id,
            grid,
            view,
            projection,
            reference_plane: "configured_synthetic_unity_response",
        })
    }
}

fn projection_parts(metric: ChannelResponseMetric) -> (ResponseMetric, &'static str, &'static str) {
    match metric {
        ChannelResponseMetric::GainDb => (ResponseMetric::GainDb, "gain_db", "Gain (dB)"),
        ChannelResponseMetric::PhaseDegrees => (
            ResponseMetric::PhaseDegrees,
            "phase_degrees",
            "Phase (degrees)",
        ),
        ChannelResponseMetric::Real => (ResponseMetric::Real, "real", "Real"),
        ChannelResponseMetric::Imaginary => (ResponseMetric::Imaginary, "imaginary", "Imaginary"),
    }
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
        let (metric, projection, units) = projection_parts(metric);
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
