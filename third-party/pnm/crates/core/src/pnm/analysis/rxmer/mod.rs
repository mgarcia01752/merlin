/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Original-sample RxMER range analysis shared by CLI and native desktop.
use crate::{
    common::{
        json,
        signal_processing::{
            bit_loading::{is_data_bit_loading, BitLoadingModel},
            ofdm::useful_symbol_rate,
        },
    },
    pnm::{
        data_type::{units::HZ_PER_KHZ, BitRateBps, BitsPerSymbol, ChannelId, FrequencyHz},
        parser::cm_ds_ofdm_rx_mer::CmDsOfdmRxMer,
    },
};
use serde::Serialize;
use std::fmt;

/// Requested uniform-modulation analysis, defaulting to the full capture.
#[derive(Debug, Clone, Copy)]
pub struct RxMerAnalysisRequest {
    pub model: BitLoadingModel,
    pub extra_margin_db: f64,
    pub maximum_bit_loading: BitsPerSymbol,
    pub start_frequency_hz: Option<FrequencyHz>,
    pub stop_frequency_hz: Option<FrequencyHz>,
}
impl Default for RxMerAnalysisRequest {
    /// Uses the approved heuristic, zero extra margin and requested 8192-QAM cap.
    /// # Returns
    /// Full-capture request; the cap is an analysis choice, not device capability.
    fn default() -> Self {
        Self {
            model: BitLoadingModel::default(),
            extra_margin_db: 0.0,
            maximum_bit_loading: 13,
            start_frequency_hz: None,
            stop_frequency_hz: None,
        }
    }
}
/// Analysis failure independent of binary-decoder offsets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RxMerAnalysisError(pub &'static str);
impl fmt::Display for RxMerAnalysisError {
    /// Describes invalid analysis input without leaking capture contents.
    /// # Arguments
    /// * `formatter` - Destination for the static diagnostic.
    /// # Returns
    /// Formatting result.
    /// # Errors
    /// Propagates formatter write failures.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}
impl std::error::Error for RxMerAnalysisError {}
/// Exact original carrier position, with unavailable/unsupported values explicit.
#[derive(Debug, Clone, Serialize)]
pub struct RxMerBitLoadingPoint {
    pub sample_index: usize,
    pub frequency_hz: FrequencyHz,
    pub rxmer_db: Option<f64>,
    pub bit_loading: Option<BitsPerSymbol>,
    pub ideal_bit_loading: Option<BitsPerSymbol>,
}
/// One hypothetical UNIFORM order over all measured positions in the selection.
#[derive(Debug, Clone, Serialize)]
pub struct UniformScenario {
    pub bit_loading: BitsPerSymbol,
    pub modulation: &'static str,
    pub model_threshold_db: Option<f64>,
    pub status: &'static str,
    pub estimated_gross_rate_bps: Option<BitRateBps>,
}
/// Immutable analysis identity and target provenance.
#[derive(Debug, Clone, Serialize)]
pub struct ModelInfo {
    pub id: BitLoadingModel,
    pub revision: u8,
    pub extra_margin_db: f64,
    pub maximum_bit_loading: BitsPerSymbol,
    pub source: &'static str,
}
/// Actual selected carrier centers and source indices, not display-bin bounds.
#[derive(Debug, Clone, Serialize)]
pub struct Selection {
    pub start_frequency_hz: Option<FrequencyHz>,
    pub stop_frequency_hz: Option<FrequencyHz>,
    pub first_sample_index: usize,
    pub sample_count: usize,
    pub subcarrier_spacing_hz: FrequencyHz,
}
/// Coverage distinguishes missing readings from unknown physical carrier roles.
#[derive(Debug, Clone, Serialize)]
pub struct Coverage {
    pub measured_count: usize,
    pub unavailable_count: usize,
    pub carrier_roles_verified: bool,
    pub status: &'static str,
}
/// Weakest-carrier uniform result; no averaging can hide a narrow low-MER notch.
#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    pub minimum_rxmer_db: Option<f64>,
    pub limiting_frequency_hz: Option<FrequencyHz>,
    pub uniform_bit_loading: Option<BitsPerSymbol>,
    pub uniform_modulation: Option<&'static str>,
    pub model_threshold_db: Option<f64>,
    pub remaining_margin_db: Option<f64>,
    pub status: &'static str,
}
/// Gross useful-symbol estimate, explicitly conditional on data-carrier roles.
#[derive(Debug, Clone, Serialize)]
pub struct GrossRate {
    pub estimated_gross_rate_bps: Option<BitRateBps>,
    pub basis: &'static str,
    pub carrier_basis: &'static str,
    pub excluded_overheads: [&'static str; 4],
}
/// Full-resolution portable result; UI adapters must bound the plot separately.
#[derive(Debug, Clone, Serialize)]
pub struct RxMerBitLoadingAnalysis {
    pub analysis_type: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_average: Option<aggregate::AverageInfo>,
    pub channel_id: ChannelId,
    pub model: ModelInfo,
    pub selection: Selection,
    pub coverage: Coverage,
    pub summary: Summary,
    pub rate: GrossRate,
    pub uniform_scenarios: Vec<UniformScenario>,
    pub subcarriers: Vec<RxMerBitLoadingPoint>,
}
impl RxMerBitLoadingAnalysis {
    /// Analyzes original samples before any display sampling or aggregation.
    /// # Arguments
    /// * `capture` - Validated RxMER capture; source bytes remain borrowed/unchanged.
    /// * `request` - Model, cap, extra margin and optional inclusive Hz window.
    /// # Returns
    /// All selected original readings, scenarios and weakest measured-carrier result.
    /// Empty full captures are valid; missing readings yield incomplete coverage.
    /// # Errors
    /// Rejects unsupported caps, invalid margins, zero/overflowing grids, unmatched
    /// range endpoints, out-of-capture windows or windows with no carrier centers.
    pub fn calculate(
        capture: &CmDsOfdmRxMer<'_>,
        request: RxMerAnalysisRequest,
    ) -> Result<Self, RxMerAnalysisError> {
        let spacing = u32::from(capture.subcarrier_spacing_khz) * HZ_PER_KHZ;
        let readings: Vec<_> = capture
            .frequencies_hz()
            .map_err(|_| RxMerAnalysisError("RxMER frequency grid overflows."))?
            .zip(capture.values_db())
            .collect();
        Self::calculate_series(capture.channel_id, spacing, &readings, request)
    }
    /// Analyzes decoded or aggregated RxMER without requantizing into wire bytes.
    /// # Arguments
    /// * `channel_id` - Decoded source channel.
    /// * `spacing` - Positive carrier spacing in Hz.
    /// * `readings` - Ordered, uniformly spaced carrier centers and optional dB values.
    /// * `request` - Model controls and inclusive frequency bounds.
    /// # Returns
    /// The same full-resolution model result used for individual captures.
    /// # Errors
    /// Rejects invalid controls, nonfinite readings, inconsistent grids or bounds.
    pub fn calculate_series(
        channel_id: ChannelId,
        spacing: FrequencyHz,
        readings: &[(FrequencyHz, Option<f64>)],
        request: RxMerAnalysisRequest,
    ) -> Result<Self, RxMerAnalysisError> {
        if !request.extra_margin_db.is_finite()
            || request.extra_margin_db < 0.0
            || !is_data_bit_loading(request.maximum_bit_loading)
        {
            return Err(RxMerAnalysisError(
                "Use a finite nonnegative margin and a supported data-modulation cap.",
            ));
        }
        if spacing == 0
            || readings
                .iter()
                .any(|(_, value)| value.is_some_and(|value| !value.is_finite()))
            || readings
                .windows(2)
                .any(|pair| pair[1].0.checked_sub(pair[0].0) != Some(spacing))
        {
            return Err(RxMerAnalysisError(
                "RxMER analysis requires a finite uniform frequency grid.",
            ));
        }
        let frequencies: Vec<_> = readings.iter().map(|point| point.0).collect();
        let bounds = match (request.start_frequency_hz, request.stop_frequency_hz) {
            (None, None) => None,
            (Some(start), Some(stop))
                if start <= stop
                    && frequencies.first().is_some_and(|first| start >= *first)
                    && frequencies.last().is_some_and(|last| stop <= *last) =>
            {
                Some((start, stop))
            }
            _ => {
                return Err(RxMerAnalysisError(
                    "Provide ordered start/stop frequencies inside this capture.",
                ))
            }
        };
        let subcarriers: Vec<_> = frequencies
            .into_iter()
            .zip(readings.iter().map(|point| point.1))
            .enumerate()
            .filter(|(_, (frequency, _))| {
                bounds.is_none_or(|(start, stop)| (start..=stop).contains(frequency))
            })
            .map(
                |(sample_index, (frequency_hz, rxmer_db))| RxMerBitLoadingPoint {
                    sample_index,
                    frequency_hz,
                    rxmer_db,
                    bit_loading: rxmer_db.and_then(|mer| {
                        request.model.supported_bits(
                            mer,
                            request.extra_margin_db,
                            request.maximum_bit_loading,
                        )
                    }),
                    ideal_bit_loading: rxmer_db.and_then(|mer| {
                        BitLoadingModel::IdealShannon.supported_bits(
                            mer,
                            request.extra_margin_db,
                            request.maximum_bit_loading,
                        )
                    }),
                },
            )
            .collect();
        if bounds.is_some() && subcarriers.is_empty() {
            return Err(RxMerAnalysisError(
                "The selected window contains no carrier centers.",
            ));
        }
        let measured_count = subcarriers
            .iter()
            .filter(|point| point.rxmer_db.is_some())
            .count();
        let unavailable_count = subcarriers.len() - measured_count;
        let limiting = subcarriers
            .iter()
            .filter_map(|point| point.rxmer_db.map(|mer| (mer, point.frequency_hz)))
            .min_by(|first, last| first.0.total_cmp(&last.0));
        let minimum_mer = limiting.map(|entry| entry.0);
        let uniform_bits = minimum_mer.and_then(|mer| {
            request
                .model
                .supported_bits(mer, request.extra_margin_db, request.maximum_bit_loading)
        });
        let target = uniform_bits.and_then(|bits| request.model.target_db(bits));
        let rate_for = |bits: BitsPerSymbol| -> Result<Option<BitRateBps>, RxMerAnalysisError> {
            if measured_count == 0 {
                return Ok(None);
            }
            let total_bits = (measured_count as u64)
                .checked_mul(u64::from(bits))
                .ok_or(RxMerAnalysisError("Selected carrier-bit count overflows."))?;
            useful_symbol_rate(total_bits, spacing)
                .map(Some)
                .ok_or(RxMerAnalysisError("Selected gross rate overflows."))
        };
        let uniform_scenarios = (2..=request.maximum_bit_loading)
            .filter(|bits| is_data_bit_loading(*bits))
            .map(|bits| {
                let threshold = request.model.target_db(bits);
                Ok(UniformScenario {
                    bit_loading: bits,
                    modulation: modulation_name(bits),
                    model_threshold_db: threshold,
                    status: match (minimum_mer, threshold) {
                        (None, _) => "no_measurements",
                        (_, None) => "not_covered_by_model",
                        (Some(mer), Some(threshold))
                            if mer >= threshold + request.extra_margin_db =>
                        {
                            "meets_target"
                        }
                        _ => "below_target",
                    },
                    estimated_gross_rate_bps: rate_for(bits)?,
                })
            })
            .collect::<Result<Vec<_>, RxMerAnalysisError>>()?;
        Ok(Self {
            analysis_type: "rxmer_bit_loading",
            temporal_average: None,
            channel_id,
            model: ModelInfo {
                id: request.model,
                revision: 1,
                extra_margin_db: request.extra_margin_db,
                maximum_bit_loading: request.maximum_bit_loading,
                source: match request.model {
                    BitLoadingModel::WebuiMerHeuristic => {
                        "PyPNM-CMTS-WebUI 25f40bda rxMerSelectionInsights.ts"
                    }
                    BitLoadingModel::OssiCandidateDefaults => {
                        "CM-OSSIv3.1 I27 D.2.11 Table 72 default targets"
                    }
                    BitLoadingModel::IdealShannon => {
                        "Ideal AWGN Shannon bound; MER used as effective SNR"
                    }
                },
            },
            selection: Selection {
                start_frequency_hz: subcarriers.first().map(|point| point.frequency_hz),
                stop_frequency_hz: subcarriers.last().map(|point| point.frequency_hz),
                first_sample_index: subcarriers.first().map_or(0, |point| point.sample_index),
                sample_count: subcarriers.len(),
                subcarrier_spacing_hz: spacing,
            },
            coverage: Coverage {
                measured_count,
                unavailable_count,
                carrier_roles_verified: false,
                status: if measured_count == 0 {
                    "no_measurements"
                } else if unavailable_count > 0 {
                    "measured_carriers_only"
                } else {
                    "all_selected_measured"
                },
            },
            summary: Summary {
                minimum_rxmer_db: minimum_mer,
                limiting_frequency_hz: limiting.map(|entry| entry.1),
                uniform_bit_loading: uniform_bits,
                uniform_modulation: uniform_bits.map(modulation_name),
                model_threshold_db: target,
                remaining_margin_db: minimum_mer
                    .zip(target)
                    .map(|(mer, target)| mer - target - request.extra_margin_db),
                status: if measured_count == 0 {
                    "no_measurements"
                } else if uniform_bits.is_none() {
                    "below_model_minimum"
                } else {
                    "meets_measured_target"
                },
            },
            rate: GrossRate {
                estimated_gross_rate_bps: uniform_bits.map(rate_for).transpose()?.flatten(),
                basis: "ideal_gross_useful_symbol",
                carrier_basis: "assume_all_measured_carriers_are_data",
                excluded_overheads: [
                    "ldpc_and_other_fec",
                    "cyclic_prefix",
                    "pilots_and_plc",
                    "mac_and_other_phy_overhead",
                ],
            },
            uniform_scenarios,
            subcarriers,
        })
    }
    /// Serializes the same bounded full-resolution result used by CLI and Qt exports.
    /// # Arguments
    /// * `max_bytes` - Maximum UTF-8 JSON bytes, enforced by the common serializer.
    /// # Returns
    /// Pretty JSON preserving missing readings and model provenance.
    /// # Errors
    /// Reports a size-limit or serialization failure.
    pub fn to_json(&self, max_bytes: usize) -> serde_json::Result<String> {
        json::to_json(self, max_bytes)
    }
}
fn modulation_name(bits: BitsPerSymbol) -> &'static str {
    crate::pnm::data_type::modulation::ModulationAssignment::from_code(bits).modulation_order
}

pub mod aggregate;
