/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Standard PNN9 acquisition compatibility and temporal linear-power statistics.
use crate::pnm::{
    data_type::{
        CaptureTime, ChannelId, FileVersionComponent, FrequencyHz, MacAddress, NoiseBandwidth,
        SampleCount, SegmentBinCount, WindowFunction,
    },
    parser::{cm_spectrum_analysis::CmSpectrumAnalysis, pnm_header::HasPnmHeader},
};
use serde::Serialize;

/// Encoded spectrum ENBW units per dimensionless noise-bandwidth factor.
pub const ENBW_UNITS_PER_FACTOR: f64 = 100.0;

/// Encoded acquisition settings that must match before spectra are combined.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct GroupKey {
    #[serde(serialize_with = "crate::pnm::data_type::serialize_mac")]
    pub mac_address: MacAddress,
    pub channel_id: ChannelId,
    pub major_version: FileVersionComponent,
    pub minor_version: FileVersionComponent,
    pub first_segment_center_hz: FrequencyHz,
    pub last_segment_center_hz: FrequencyHz,
    pub segment_span_hz: FrequencyHz,
    pub bins_per_segment: SegmentBinCount,
    pub encoded_enbw: NoiseBandwidth,
    pub window_function: WindowFunction,
    pub sample_count: SampleCount,
}
impl GroupKey {
    /// Copies standard PNN9 acquisition metadata, excluding time and measurements.
    /// # Arguments
    /// * `capture` - Decoded standard spectrum file.
    /// # Returns
    /// Exact encoded compatibility key; values are validated when deriving its grid.
    pub fn from_capture(capture: &CmSpectrumAnalysis<'_>) -> Self {
        let version = capture.format_version();
        Self {
            mac_address: capture.mac_address,
            channel_id: capture.channel_id,
            major_version: version.major,
            minor_version: version.minor,
            first_segment_center_hz: capture.first_segment_center_frequency,
            last_segment_center_hz: capture.last_segment_center_frequency,
            segment_span_hz: capture.segment_frequency_span,
            bins_per_segment: capture.num_bins_per_segment,
            encoded_enbw: capture.equivalent_noise_bandwidth,
            window_function: capture.window_function,
            sample_count: capture.raw_values().len() / 2,
        }
    }
    /// Derives bin centers using the existing single-spectrum frequency convention.
    /// # Returns
    /// One checked integer-Hz center per source bin, without accumulating rounding.
    /// # Errors
    /// Rejects empty, invalid, overfull or overflowing standard spectrum grids.
    pub fn frequencies_hz(&self) -> Result<Vec<FrequencyHz>, &'static str> {
        let span = u64::from(self.segment_span_hz);
        let bins = u64::from(self.bins_per_segment);
        let first = u64::from(self.first_segment_center_hz);
        let last = u64::from(self.last_segment_center_hz);
        if bins == 0
            || span < bins
            || last < first
            || (last - first) % span != 0
            || first * 2 < span
        {
            return Err("Invalid spectrum acquisition grid.");
        }
        let configured = ((last - first) / span + 1) * bins;
        if self.sample_count as u64 > configured {
            return Err("Spectrum contains more bins than its acquisition settings.");
        }
        let origin = (first * 2 - span) * bins;
        (0..self.sample_count)
            .map(|index| {
                u32::try_from((origin + (2 * index as u64 + 1) * span) / (2 * bins))
                    .map_err(|_| "Spectrum frequency exceeds the supported Hz range.")
            })
            .collect()
    }
    /// Returns exact fractional-Hz bin spacing from encoded acquisition fields.
    /// # Returns
    /// Frequency spacing; callers validate positive bins through frequencies_hz.
    pub fn bin_spacing_hz(&self) -> f64 {
        f64::from(self.segment_span_hz) / f64::from(self.bins_per_segment)
    }
    /// Calculates acquisition RBW using the encoded hundredths-of-spacing ENBW.
    /// # Returns
    /// Positive finite RBW, or None for invalid acquisition bandwidth metadata.
    pub fn resolution_bandwidth_hz(&self) -> Option<f64> {
        crate::common::signal_processing::spectrum::resolution_bandwidth_hz(
            self.bin_spacing_hz(),
            f64::from(self.encoded_enbw) / ENBW_UNITS_PER_FACTOR,
        )
    }
}
/// One owned spectrum; filenames are provenance only, never device/time metadata.
#[derive(Debug, Clone)]
pub struct Series {
    pub key: GroupKey,
    pub source: String,
    pub capture_time: CaptureTime,
    pub values_dbmv: Vec<Option<f64>>,
}
impl Series {
    /// Retains one decoded standard capture for collection analysis.
    /// # Arguments
    /// * `capture` - Standard PNN9 capture, already validated by its parser.
    /// * `source` - Original path/member label for provenance only.
    /// # Returns
    /// Owned unrounded readings and binary-header acquisition/time metadata.
    /// # Errors
    /// Rejects invalid frequency/RBW settings or absent header capture time.
    pub fn from_capture(
        capture: &CmSpectrumAnalysis<'_>,
        source: String,
    ) -> Result<Self, &'static str> {
        let key = GroupKey::from_capture(capture);
        key.frequencies_hz()?;
        if key.sample_count == 0 {
            return Err("Spectrum capture has no bins.");
        }
        key.resolution_bandwidth_hz()
            .ok_or("Invalid spectrum resolution bandwidth.")?;
        Ok(Self {
            key,
            source,
            capture_time: capture
                .capture_time()
                .ok_or("Spectrum capture has no epoch.")?,
            values_dbmv: capture.amplitudes_db().map(Some).collect(),
        })
    }
}
/// Binary-header chronology and source provenance included in exports/catalogs.
#[derive(Debug, Clone, Serialize)]
pub struct Source {
    pub name: String,
    pub capture_time: CaptureTime,
}
/// Full-resolution temporal statistics at one exact source bin.
#[derive(Debug, Clone, Serialize)]
pub struct AggregatePoint {
    pub sample_index: usize,
    pub frequency_hz: FrequencyHz,
    pub minimum_dbmv: Option<f64>,
    pub average_dbmv: Option<f64>,
    pub maximum_dbmv: Option<f64>,
    pub valid_count: usize,
}
/// Immutable core result shared by JSON exports and desktop presentation.
#[derive(Debug, Clone, Serialize)]
pub struct SpectrumAggregate {
    pub capture_count: SampleCount,
    pub first_epoch: CaptureTime,
    pub last_epoch: CaptureTime,
    pub analysis_type: &'static str,
    pub average_method: &'static str,
    pub value_units: &'static str,
    pub group: GroupKey,
    pub sources: Vec<Source>,
    pub points: Vec<AggregatePoint>,
}
impl SpectrumAggregate {
    /// Calculates equal-capture-weight statistics with mean-square-voltage averaging.
    /// # Arguments
    /// * `series` - At least one capture with identical encoded settings/grid.
    /// # Returns
    /// Full-resolution extrema, linear-power mean, valid counts and ordered sources.
    /// # Errors
    /// Rejects empty or incompatible inputs and invalid acquisition metadata.
    pub fn calculate(series: &[&Series]) -> Result<Self, &'static str> {
        let first = series
            .first()
            .ok_or("Select at least one spectrum capture.")?;
        if first.key.sample_count == 0 {
            return Err("Spectrum capture has no bins.");
        }
        first
            .key
            .resolution_bandwidth_hz()
            .ok_or("Invalid spectrum resolution bandwidth.")?;
        let frequencies = first.key.frequencies_hz()?;
        if series
            .iter()
            .any(|item| item.key != first.key || item.values_dbmv.len() != frequencies.len())
        {
            return Err("Spectrum capture settings must match exactly.");
        }
        let mut ordered = series.to_vec();
        ordered.sort_by(|first, last| {
            (first.capture_time, &first.source).cmp(&(last.capture_time, &last.source))
        });
        let points = frequencies
            .into_iter()
            .enumerate()
            .map(|(sample_index, frequency_hz)| {
                let values: Vec<_> = ordered
                    .iter()
                    .filter_map(|item| {
                        item.values_dbmv[sample_index].filter(|value| value.is_finite())
                    })
                    .collect();
                let minimum = values.iter().copied().min_by(f64::total_cmp);
                let maximum = values.iter().copied().max_by(f64::total_cmp);
                AggregatePoint {
                    sample_index,
                    frequency_hz,
                    minimum_dbmv: minimum,
                    average_dbmv: crate::common::signal_processing::spectrum::mean_level_dbmv(
                        &values,
                    ),
                    maximum_dbmv: maximum,
                    valid_count: values.len(),
                }
            })
            .collect();
        Ok(Self {
            capture_count: ordered.len(),
            first_epoch: ordered.first().unwrap().capture_time,
            last_epoch: ordered.last().unwrap().capture_time,
            analysis_type: "spectrum_min_avg_max",
            average_method: "linear_power",
            value_units: "dBmV",
            group: first.key.clone(),
            sources: ordered
                .iter()
                .map(|item| Source {
                    name: item.source.clone(),
                    capture_time: item.capture_time,
                })
                .collect(),
            points,
        })
    }
}
