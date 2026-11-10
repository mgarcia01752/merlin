/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! WAVE-RX: finite received-only numerology and integer/fractional CFO search.
use super::acquisition::{
    estimate_pilot_phase_window, estimate_timing_and_cfo, AcquisitionEstimate,
    CarrierOffsetCorrection,
};
use crate::{
    error::invalid,
    signal_processing::waveform::{fft::ForwardTransform, SampleBlock, WaveformGeometry},
    transmission::layout::{CarrierLayout, CarrierLayoutSettings},
    SimulationError,
};
use serde::Serialize;

/// Application limit, not a DOCSIS receiver-frequency requirement.
pub const MAX_INTEGER_CARRIER_OFFSET: i16 = 16;
/// Maximum independently configured bootstrap hypotheses per received search.
pub const MAX_ACQUISITION_HYPOTHESES: usize = 8;
const MINIMUM_HYPOTHESIS_ERROR_MARGIN: f64 = 0.05;
const MAX_TIMING_SEARCH_SAMPLES: usize = 1024;
const PHASE_SEARCH_SYMBOLS: usize = 2;

/// One possible receiver bootstrap configuration; no channel coefficients or TX truth.
#[derive(Clone)]
pub struct AcquisitionHypothesis {
    pub geometry: WaveformGeometry,
    pub layout: CarrierLayoutSettings,
}
/// Finite search and acceptance policy shared by direct Rust consumers.
pub struct AcquisitionSearchSettings {
    /// Inclusive timing search in sample-clock ticks, at most1024.
    pub max_offset_samples: usize,
    /// Normalized squared CP correlation acceptance threshold in (0,1].
    pub minimum_correlation: f64,
    /// Symmetric integer-carrier search radius, zero through16.
    pub max_integer_carrier_offset: i16,
}
/// Selected hypothesis and observed synchronization results before CH-EST-RX.
#[derive(Debug, Clone, Serialize)]
pub struct AcquisitionSearchEstimate {
    pub hypothesis_index: usize,
    pub integer_carrier_offset: i16,
    pub fractional_carrier_offset_hz: f64,
    /// Total CFO is stored in acquisition.carrier_offset_hz.
    pub acquisition: AcquisitionEstimate,
    pub accepted_candidates: usize,
}

/// Selects numerology, symbol timing, total CFO and pilot phase from received samples.
/// # Arguments
/// * `hypotheses` - One to eight distinct receiver bootstrap maps; CP/RP and layout
///   are supplied possibilities, not discovered RF metadata or transmitter state.
/// * `input` - Finite received samples covering two periods plus the timing
///   search extent for every candidate; shorter candidates are not silently preferred.
/// * `settings` - Bounded timing/integer search and normalized CP acceptance policy.
/// # Returns
/// Selected hypothesis identity and measured synchronization, without mutating inputs.
/// Continue through WaveformReceiver::acquire_search to consume initial samples.
/// # Errors
/// Rejects invalid/duplicate hypotheses, insufficient samples, exceeded budgets,
/// no supported pilot match, or insufficient score separation between candidates.
pub fn search_acquisition(
    hypotheses: &[AcquisitionHypothesis],
    input: &SampleBlock,
    settings: &AcquisitionSearchSettings,
) -> Result<AcquisitionSearchEstimate, SimulationError> {
    if hypotheses.is_empty()
        || hypotheses.len() > MAX_ACQUISITION_HYPOTHESES
        || !(0..=MAX_INTEGER_CARRIER_OFFSET).contains(&settings.max_integer_carrier_offset)
        || settings.max_offset_samples > MAX_TIMING_SEARCH_SAMPLES
        || !settings.minimum_correlation.is_finite()
        || !(0.0..=1.0).contains(&settings.minimum_correlation)
        || settings.minimum_correlation == 0.0
    {
        return Err(invalid(
            "Invalid bounded WAVE-RX acquisition search policy.",
        ));
    }
    for (index, hypothesis) in hypotheses.iter().enumerate() {
        CarrierLayout::new(hypothesis.geometry.settings(), &hypothesis.layout)?;
        if input.samples().len()
            < PHASE_SEARCH_SYMBOLS * hypothesis.geometry.stride_samples()
                + settings.max_offset_samples
        {
            return Err(invalid(
                "Acquisition input must cover every hypothesis timing window.",
            ));
        }
        if hypotheses[..index].iter().any(|previous| {
            previous.geometry == hypothesis.geometry && previous.layout == hypothesis.layout
        }) {
            return Err(invalid("Duplicate receiver acquisition hypothesis."));
        }
    }
    let mut accepted = Vec::new();
    for (hypothesis_index, hypothesis) in hypotheses.iter().enumerate() {
        let Ok((timing, fractional)) = estimate_timing_and_cfo(
            &hypothesis.geometry,
            input,
            settings.max_offset_samples,
            settings.minimum_correlation,
        ) else {
            continue;
        };
        let corrected =
            CarrierOffsetCorrection::new(&hypothesis.geometry, fractional, input.start_index())?
                .apply(input)?;
        let first = (timing.first_symbol_sample - input.start_index()) as usize
            + hypothesis.geometry.prefix_samples();
        let transform = ForwardTransform::new(hypothesis.geometry.settings().subcarrier_spacing);
        let mut grids = Vec::with_capacity(PHASE_SEARCH_SYMBOLS);
        for symbol in 0..PHASE_SEARCH_SYMBOLS {
            let start = first + symbol * hypothesis.geometry.stride_samples();
            grids.push(
                transform.apply(
                    &corrected.samples()[start..start + hypothesis.geometry.fft_samples()],
                )?,
            );
        }
        let layout = CarrierLayout::new(hypothesis.geometry.settings(), &hypothesis.layout)?;
        for integer in -settings.max_integer_carrier_offset..=settings.max_integer_carrier_offset {
            let aligned: Vec<Vec<_>> = grids
                .iter()
                .map(|bins| {
                    (0..bins.len())
                        .map(|bin| {
                            bins[(bin as isize + integer as isize).rem_euclid(bins.len() as isize)
                                as usize]
                        })
                        .collect()
                })
                .collect();
            let borrowed: Vec<_> = aligned.iter().map(Vec::as_slice).collect();
            if let Ok((phase, error)) = estimate_pilot_phase_window(&layout, &borrowed, 1.0) {
                accepted.push(AcquisitionSearchEstimate {
                    hypothesis_index,
                    integer_carrier_offset: integer,
                    fractional_carrier_offset_hz: fractional,
                    acquisition: AcquisitionEstimate {
                        timing: timing.clone(),
                        carrier_offset_hz: fractional
                            + f64::from(integer)
                                * f64::from(
                                    hypothesis
                                        .geometry
                                        .settings()
                                        .subcarrier_spacing
                                        .frequency_hz(),
                                ),
                        pilot_phase: phase,
                        normalized_pilot_error: error,
                    },
                    accepted_candidates: 0,
                });
            }
        }
    }
    accepted.sort_by(|left, right| {
        left.acquisition
            .normalized_pilot_error
            .total_cmp(&right.acquisition.normalized_pilot_error)
    });
    if accepted.is_empty() {
        return Err(invalid(
            "No supported received numerology/integer-CFO pilot match.",
        ));
    }
    if accepted.len() > 1
        && accepted[1].acquisition.normalized_pilot_error
            - accepted[0].acquisition.normalized_pilot_error
            < MINIMUM_HYPOTHESIS_ERROR_MARGIN
    {
        return Err(invalid(
            "Ambiguous received numerology/integer-CFO hypotheses.",
        ));
    }
    let count = accepted.len();
    let mut selected = accepted.remove(0);
    selected.accepted_candidates = count;
    Ok(selected)
}
