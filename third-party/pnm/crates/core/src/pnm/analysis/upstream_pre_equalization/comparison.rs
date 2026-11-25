/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! OSSI PNN6/7 compatibility precheck and explicitly hypothetical correction results.
use super::PreEqualizationAnalysis;
pub use crate::common::signal_processing::pre_equalization::CorrectionMode;
use crate::{
    common::signal_processing::{
        channel_response::{group_delay_seconds, magnitude_db, GroupDelaySeconds, MagnitudeDb},
        pre_equalization::{apply, reference_db},
    },
    pnm::{
        data_type::{units::HZ_PER_KHZ, ChannelId, FrequencyHz, IqPair, MacAddress, SampleCount},
        parser::{
            cm_us_ofdma_pre_eq::CmUsOfdmaPreEq, model::pnm_header::PnmHeaderParameters,
            pnm_file_type::PnmFileKind, pnm_header::HasPnmHeader,
        },
    },
};
use serde::Serialize;

/// Field-specific failure preserving expected/actual information for UI and CLI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ComparisonError {
    pub field: &'static str,
    pub message: String,
}
impl std::fmt::Display for ComparisonError {
    /// Formats the affected field and its precheck failure.
    /// # Arguments
    /// * `formatter` - Diagnostic destination.
    /// # Returns
    /// Formatting result, propagating destination errors.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.field, self.message)
    }
}
impl std::error::Error for ComparisonError {}
fn failure(field: &'static str, message: impl Into<String>) -> ComparisonError {
    ComparisonError {
        field,
        message: message.into(),
    }
}
/// Compatible capture identities and exact shared grid; file-time order is policy only.
#[derive(Debug, Clone, Serialize)]
pub struct PairMetadata {
    pub original_header: PnmHeaderParameters,
    pub correction_header: PnmHeaderParameters,
    pub channel_id: ChannelId,
    #[serde(serialize_with = "crate::pnm::data_type::serialize_mac")]
    pub mac_address: MacAddress,
    #[serde(serialize_with = "crate::pnm::data_type::serialize_mac")]
    pub cmts_mac_address: MacAddress,
    pub first_frequency_hz: FrequencyHz,
    pub last_frequency_hz: FrequencyHz,
    pub subcarrier_spacing_hz: FrequencyHz,
    pub sample_count: SampleCount,
    pub file_time_difference_s: u32,
    pub precheck_passed: bool,
    pub update_mode_encoded: bool,
    pub application_status_known: bool,
}
/// One full-resolution prediction/delta row; undefined dB and delay stay null.
#[derive(Debug, Clone, Serialize)]
pub struct ComparisonPoint {
    pub sample_index: usize,
    pub frequency_hz: FrequencyHz,
    pub original_iq: Option<IqPair>,
    pub correction_iq: Option<IqPair>,
    pub predicted_iq: Option<IqPair>,
    pub delta_iq: Option<IqPair>,
    pub original_db: Option<MagnitudeDb>,
    pub predicted_db: Option<MagnitudeDb>,
    pub delta_db: Option<MagnitudeDb>,
    pub original_aligned_db: Option<MagnitudeDb>,
    pub predicted_aligned_db: Option<MagnitudeDb>,
    pub aligned_delta_db: Option<MagnitudeDb>,
    pub original_delay_s: Option<GroupDelaySeconds>,
    pub predicted_delay_s: Option<GroupDelaySeconds>,
    pub delta_delay_s: Option<GroupDelaySeconds>,
}
/// Full-data summaries, independent of UI decimation and display alignment.
#[derive(Debug, Clone, Serialize)]
pub struct ComparisonSummary {
    pub original_magnitude_span_db: Option<MagnitudeDb>,
    pub predicted_magnitude_span_db: Option<MagnitudeDb>,
    pub original_reference_db: Option<MagnitudeDb>,
    pub predicted_reference_db: Option<MagnitudeDb>,
    pub raw_reference_shift_db: Option<MagnitudeDb>,
    pub aligned_delta_min_db: Option<MagnitudeDb>,
    pub aligned_delta_max_db: Option<MagnitudeDb>,
}
/// Predicted result, with explicit mode, limitations and retained raw coefficient delta.
#[derive(Debug, Clone, Serialize)]
pub struct PreEqComparison {
    pub analysis_type: &'static str,
    pub pair: PairMetadata,
    pub mode: CorrectionMode,
    pub predicted: bool,
    pub assumptions: [&'static str; 4],
    pub alignment_basis: &'static str,
    pub summary: ComparisonSummary,
    pub points: Vec<ComparisonPoint>,
}
/// Checks file roles, headers, grid, payload counts and strictly newer encoded time.
/// # Arguments
/// * `original` - Current PNN6 baseline.
/// * `correction` - Last-update PNN7 file to attach.
/// # Returns
/// Matching grid/device metadata and the positive file-time gap.
/// # Errors
/// Names the first mismatch; no mode, application history or physical TX power is inferred.
pub fn precheck(
    original: &CmUsOfdmaPreEq<'_>,
    correction: &CmUsOfdmaPreEq<'_>,
) -> Result<PairMetadata, ComparisonError> {
    if original.pnm_header().descriptor().kind != PnmFileKind::UpstreamPreEqualization {
        return Err(failure(
            "original_type",
            "Expected a current PNN6 Pre-EQ baseline.",
        ));
    }
    if correction.pnm_header().descriptor().kind != PnmFileKind::UpstreamPreEqualizationLastUpdate {
        return Err(failure(
            "correction_type",
            "Expected a last-update PNN7 correction.",
        ));
    }
    let original_header = original.pnm_header().to_model().pnm_header;
    let correction_header = correction.pnm_header().to_model().pnm_header;
    macro_rules! matching {
        ($field:ident) => {
            if original.$field != correction.$field {
                return Err(failure(
                    stringify!($field),
                    format!(
                        "Expected {:?}; received {:?}.",
                        original.$field, correction.$field
                    ),
                ));
            }
        };
    }
    if original_header.major_version != correction_header.major_version
        || original_header.minor_version != correction_header.minor_version
    {
        return Err(failure(
            "format_version",
            "File-format versions must match.",
        ));
    }
    matching!(channel_id);
    for (field, expected, received) in [
        ("mac_address", original.mac_address, correction.mac_address),
        (
            "cmts_mac_address",
            original.cmts_mac_address,
            correction.cmts_mac_address,
        ),
    ] {
        if expected != received {
            return Err(failure(
                field,
                format!(
                    "Expected {}; received {}.",
                    crate::pnm::data_type::mac_text(&expected),
                    crate::pnm::data_type::mac_text(&received)
                ),
            ));
        }
    }
    matching!(subcarrier_zero_frequency_hz);
    matching!(first_active_subcarrier_index);
    matching!(subcarrier_spacing_khz);
    matching!(data_length);
    let sample_count = original.iq().count();
    if sample_count == 0 || sample_count != correction.iq().count() {
        return Err(failure(
            "sample_count",
            "Require nonempty, equally sized coefficient arrays.",
        ));
    }
    let original_time = original_header
        .capture_time
        .ok_or_else(|| failure("original_time", "Baseline timestamp is missing."))?;
    let correction_time = correction_header
        .capture_time
        .ok_or_else(|| failure("correction_time", "Correction timestamp is missing."))?;
    let elapsed = correction_time.checked_sub(original_time).filter(|elapsed|*elapsed>0).ok_or_else(||failure("capture_time",format!("Correction must be newer than epoch {original_time}; received {correction_time}.")))?;
    let grid = PreEqualizationAnalysis::from_capture(original)
        .map_err(|error| failure("frequency_grid", error.to_string()))?;
    Ok(PairMetadata {
        original_header,
        correction_header,
        channel_id: original.channel_id,
        mac_address: original.mac_address,
        cmts_mac_address: original.cmts_mac_address,
        first_frequency_hz: grid.frequencies_hz[0],
        last_frequency_hz: *grid.frequencies_hz.last().unwrap(),
        subcarrier_spacing_hz: u32::from(original.subcarrier_spacing_khz) * HZ_PER_KHZ,
        sample_count,
        file_time_difference_s: elapsed,
        precheck_passed: true,
        update_mode_encoded: false,
        application_status_known: false,
    })
}
fn difference(first: Option<f64>, second: Option<f64>) -> Option<f64> {
    first.zip(second).map(|(first, second)| first - second)
}
fn minimum(values: impl Iterator<Item = Option<f64>>) -> Option<f64> {
    values.flatten().reduce(f64::min)
}
fn maximum(values: impl Iterator<Item = Option<f64>>) -> Option<f64> {
    values.flatten().reduce(f64::max)
}
fn span(values: impl Iterator<Item = Option<f64>> + Clone) -> Option<f64> {
    difference(maximum(values.clone()), minimum(values))
}
impl PreEqComparison {
    /// Predicts apply-then-delta from the entire compatible original/correction pair.
    /// # Arguments
    /// * `original`, `correction` - Separately decoded Q formats on the same exact grid.
    /// * `mode` - Required user interpretation, never inferred from the file.
    /// # Returns
    /// Full-resolution raw/aligned magnitude, complex delta and delay change in seconds.
    /// # Errors
    /// Propagates compatibility/time/empty/grid failures before producing any result.
    pub fn calculate(
        original: &CmUsOfdmaPreEq<'_>,
        correction: &CmUsOfdmaPreEq<'_>,
        mode: CorrectionMode,
    ) -> Result<Self, ComparisonError> {
        let pair = precheck(original, correction)?;
        let original_iq: Vec<_> = original.iq().collect();
        let correction_iq: Vec<_> = correction.iq().collect();
        let predicted_iq: Vec<_> = original_iq
            .iter()
            .copied()
            .zip(correction_iq.iter().copied())
            .map(|(original, correction)| apply(original, correction, mode))
            .collect();
        let original_reference = reference_db(&original_iq);
        let predicted_reference = reference_db(&predicted_iq);
        let original_delay = group_delay_seconds(&original_iq, pair.subcarrier_spacing_hz);
        let predicted_delay = group_delay_seconds(&predicted_iq, pair.subcarrier_spacing_hz);
        let points: Vec<_> = (0..pair.sample_count)
            .map(|index| {
                let original_db = magnitude_db(original_iq[index]);
                let predicted_db = magnitude_db(predicted_iq[index]);
                let original_aligned_db = difference(original_db, original_reference);
                let predicted_aligned_db = difference(predicted_db, predicted_reference);
                ComparisonPoint {
                    sample_index: index,
                    frequency_hz: pair.first_frequency_hz
                        + index as u32 * pair.subcarrier_spacing_hz,
                    original_iq: original_iq[index],
                    correction_iq: correction_iq[index],
                    predicted_iq: predicted_iq[index],
                    delta_iq: predicted_iq[index].zip(original_iq[index]).map(
                        |(
                            [predicted_in_phase, predicted_quadrature],
                            [original_in_phase, original_quadrature],
                        )| {
                            [
                                predicted_in_phase - original_in_phase,
                                predicted_quadrature - original_quadrature,
                            ]
                        },
                    ),
                    original_db,
                    predicted_db,
                    delta_db: difference(predicted_db, original_db),
                    original_aligned_db,
                    predicted_aligned_db,
                    aligned_delta_db: difference(predicted_aligned_db, original_aligned_db),
                    original_delay_s: original_delay[index],
                    predicted_delay_s: predicted_delay[index],
                    delta_delay_s: difference(predicted_delay[index], original_delay[index]),
                }
            })
            .collect();
        let summary = ComparisonSummary {
            original_magnitude_span_db: span(points.iter().map(|point| point.original_db)),
            predicted_magnitude_span_db: span(points.iter().map(|point| point.predicted_db)),
            original_reference_db: original_reference,
            predicted_reference_db: predicted_reference,
            raw_reference_shift_db: difference(predicted_reference, original_reference),
            aligned_delta_min_db: minimum(points.iter().map(|point| point.aligned_delta_db)),
            aligned_delta_max_db: maximum(points.iter().map(|point| point.aligned_delta_db)),
        };
        Ok(Self { analysis_type:"upstream_pre_eq_comparison",pair,mode,predicted:true,assumptions:["User-selected Adjust/Set; mode not encoded in PNM", "File-time order does not prove adjacent updates", "No modem clipping, rounding, rejection or normalization simulated", "Coefficient response, not measured TX power or plant response"],alignment_basis:"Each magnitude curve referenced to its own mean coefficient power over all matched carriers; presentation only",summary,points })
    }
    /// Exports full-resolution comparison using the common JSON byte budget.
    /// # Arguments
    /// * `max_bytes` - Maximum serialized size.
    /// # Returns
    /// Pretty JSON with source provenance, raw prediction and labeled alignment.
    /// # Errors
    /// Reports serialization failure or an exceeded output budget.
    pub fn to_json(&self, max_bytes: usize) -> serde_json::Result<String> {
        crate::common::json::to_json(self, max_bytes)
    }
}
