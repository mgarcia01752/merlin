/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Temporal per-carrier statistics for header-compatible RxMER captures.
use super::{RxMerAnalysisError, RxMerAnalysisRequest, RxMerBitLoadingAnalysis};
use crate::pnm::{
    data_type::{
        units::HZ_PER_KHZ, CaptureTime, ChannelId, FileVersionComponent, FrequencyHz, MacAddress,
        RxMerDb, SampleCount, SubcarrierIdx,
    },
    parser::{cm_ds_ofdm_rx_mer::CmDsOfdmRxMer, pnm_header::HasPnmHeader},
};
use serde::Serialize;

/// All metadata that must agree before temporal aggregation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct GroupKey {
    #[serde(serialize_with = "crate::pnm::data_type::serialize_mac")]
    pub mac_address: MacAddress,
    pub channel_id: ChannelId,
    pub major_version: FileVersionComponent,
    pub minor_version: FileVersionComponent,
    pub zero_frequency_hz: FrequencyHz,
    pub first_active_subcarrier_index: SubcarrierIdx,
    pub spacing_hz: FrequencyHz,
    pub sample_count: SampleCount,
}
/// One fully decoded capture; source names never determine metadata.
#[derive(Debug, Clone)]
pub struct Series {
    pub key: GroupKey,
    pub source: String,
    pub capture_time: CaptureTime,
    pub values_db: Vec<Option<RxMerDb>>,
}
impl Series {
    /// Copies decoded values and header identity for a temporal collection.
    /// # Arguments
    /// * `capture` - Validated borrowed RxMER file.
    /// * `source` - Display/provenance name, never parsed for grouping.
    /// # Returns
    /// An owned decoded series with unavailable readings preserved.
    /// # Errors
    /// Rejects empty captures, missing epochs and zero/overflowing frequency grids.
    pub fn from_capture(
        capture: &CmDsOfdmRxMer<'_>,
        source: String,
    ) -> Result<Self, RxMerAnalysisError> {
        let version = capture.format_version();
        let capture_time = capture
            .capture_time()
            .ok_or(RxMerAnalysisError("RxMER capture has no epoch."))?;
        let spacing_hz = u32::from(capture.subcarrier_spacing_khz) * HZ_PER_KHZ;
        if spacing_hz == 0 || capture.raw_values().is_empty() {
            return Err(RxMerAnalysisError(
                "RxMER aggregation requires a nonempty positive-spacing grid.",
            ));
        }
        let _frequencies = capture
            .frequencies_hz()
            .map_err(|_| RxMerAnalysisError("RxMER grid overflows."))?;
        Ok(Self {
            key: GroupKey {
                mac_address: capture.mac_address,
                channel_id: capture.channel_id,
                major_version: version.major,
                minor_version: version.minor,
                zero_frequency_hz: capture.subcarrier_zero_frequency_hz,
                first_active_subcarrier_index: capture.first_active_subcarrier_index,
                spacing_hz,
                sample_count: capture.raw_values().len(),
            },
            source,
            capture_time,
            values_db: capture.values_db().collect(),
        })
    }
}
/// Source order used for reproducible statistics and export provenance.
#[derive(Debug, Clone, Serialize)]
pub struct Source {
    pub name: String,
    pub capture_time: CaptureTime,
}
/// Statistics across valid readings at exactly one carrier center.
#[derive(Debug, Clone, Serialize)]
pub struct AggregatePoint {
    pub sample_index: usize,
    pub frequency_hz: FrequencyHz,
    pub minimum_db: Option<RxMerDb>,
    pub average_db: Option<RxMerDb>,
    pub maximum_db: Option<RxMerDb>,
    pub valid_count: usize,
}
/// Provenance attached when modulation is calculated from a temporal average.
#[derive(Debug, Clone, Serialize)]
pub struct AverageInfo {
    pub method: &'static str,
    pub group: GroupKey,
    pub capture_count: usize,
    pub first_epoch: Option<CaptureTime>,
    pub last_epoch: Option<CaptureTime>,
}
/// Full-resolution result independent of desktop presentation.
#[derive(Debug, Clone, Serialize)]
pub struct RxMerAggregate {
    pub analysis_type: &'static str,
    pub average_method: &'static str,
    pub group: GroupKey,
    pub sources: Vec<Source>,
    pub points: Vec<AggregatePoint>,
}
impl RxMerAggregate {
    /// Computes an equal-weight arithmetic dB mean, minimum and maximum per carrier.
    /// # Arguments
    /// * `captures` - Same-key series; callers deduplicate contents before calling.
    /// # Returns
    /// Epoch-ordered sources and statistics; no valid readings produces nulls.
    /// # Errors
    /// Rejects empty collections, incompatible keys, invalid lengths/grids or values.
    pub fn calculate(captures: &[&Series]) -> Result<Self, RxMerAnalysisError> {
        let first = captures
            .first()
            .ok_or(RxMerAnalysisError("No RxMER captures selected."))?;
        let key = &first.key;
        if key.spacing_hz == 0
            || key.sample_count == 0
            || captures.iter().any(|capture| {
                capture.key != *key
                    || capture.values_db.len() != key.sample_count
                    || capture
                        .values_db
                        .iter()
                        .flatten()
                        .any(|value| !value.is_finite())
            })
        {
            return Err(RxMerAnalysisError(
                "RxMER captures must have matching device, channel, format and frequency grid.",
            ));
        }
        let first_hz = u32::from(key.first_active_subcarrier_index)
            .checked_mul(key.spacing_hz)
            .and_then(|offset| key.zero_frequency_hz.checked_add(offset))
            .ok_or(RxMerAnalysisError("RxMER grid overflows."))?;
        let mut ordered = captures.to_vec();
        ordered.sort_by(|first, last| {
            (first.capture_time, &first.source).cmp(&(last.capture_time, &last.source))
        });
        let mut points = Vec::with_capacity(key.sample_count);
        for index in 0..key.sample_count {
            let frequency_hz = u32::try_from(index)
                .ok()
                .and_then(|index| index.checked_mul(key.spacing_hz))
                .and_then(|offset| first_hz.checked_add(offset))
                .ok_or(RxMerAnalysisError("RxMER grid overflows."))?;
            let mut minimum: Option<f64> = None;
            let mut maximum: Option<f64> = None;
            let mut sum = 0.0;
            let mut count = 0;
            for capture in &ordered {
                if let Some(value) = capture.values_db[index] {
                    minimum = Some(minimum.map_or(value, |old| old.min(value)));
                    maximum = Some(maximum.map_or(value, |old| old.max(value)));
                    sum += value;
                    count += 1;
                }
            }
            if !sum.is_finite() {
                return Err(RxMerAnalysisError("RxMER mean overflows."));
            }
            points.push(AggregatePoint {
                sample_index: index,
                frequency_hz,
                minimum_db: minimum,
                maximum_db: maximum,
                average_db: (count > 0).then(|| sum / count as f64),
                valid_count: count,
            });
        }
        Ok(Self {
            analysis_type: "rxmer_min_avg_max",
            average_method: "arithmetic_db_valid_readings",
            group: key.clone(),
            sources: ordered
                .iter()
                .map(|capture| Source {
                    name: capture.source.clone(),
                    capture_time: capture.capture_time,
                })
                .collect(),
            points,
        })
    }
    /// Applies the existing modulation model to unrounded average values.
    /// # Arguments
    /// * `start`, `count` - Exact aggregate source-carrier range.
    /// * `request` - Same model controls used for single-capture RxMER.
    /// # Returns
    /// Full analysis of the average line, retaining fractional dB precision.
    /// # Errors
    /// Rejects invalid ranges or model settings.
    pub fn analyze_average(
        &self,
        start: usize,
        count: usize,
        mut request: RxMerAnalysisRequest,
    ) -> Result<RxMerBitLoadingAnalysis, RxMerAnalysisError> {
        let end = start
            .checked_add(count)
            .filter(|end| *end <= self.points.len() && count > 0)
            .ok_or(RxMerAnalysisError(
                "Choose a nonempty range inside the average line.",
            ))?;
        request.start_frequency_hz = Some(self.points[start].frequency_hz);
        request.stop_frequency_hz = Some(self.points[end - 1].frequency_hz);
        let readings: Vec<_> = self
            .points
            .iter()
            .map(|point| (point.frequency_hz, point.average_db))
            .collect();
        let mut analysis = RxMerBitLoadingAnalysis::calculate_series(
            self.group.channel_id,
            self.group.spacing_hz,
            &readings,
            request,
        )?;
        analysis.temporal_average = Some(AverageInfo {
            method: self.average_method,
            group: self.group.clone(),
            capture_count: self.sources.len(),
            first_epoch: self.sources.first().map(|source| source.capture_time),
            last_epoch: self.sources.last().map(|source| source.capture_time),
        });
        Ok(analysis)
    }
}
