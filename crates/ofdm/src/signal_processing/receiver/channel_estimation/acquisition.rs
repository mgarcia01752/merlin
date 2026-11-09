/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded fractional CFO and pilot-phase acquisition with known FFT/CP/layout.
use super::{
    interpolation::interpolate,
    timing::{estimate_symbol_boundary, TimingEstimate},
};
use crate::{
    error::invalid,
    signal_processing::{
        pilots::{PilotPower, PilotSequence},
        waveform::{SampleBlock, WaveformGeometry},
    },
    transmission::{
        layout::{CarrierLayout, PhysicalCarrierRole},
        settings::SAMPLE_RATE_HZ,
    },
    types::IqPair,
    SimulationError,
};
use serde::Serialize;
/// Observed timing, fractional carrier offset and recovered128-symbol pilot phase.
#[derive(Debug, Clone, Serialize)]
pub struct AcquisitionEstimate {
    pub timing: TimingEstimate,
    pub carrier_offset_hz: f64,
    pub pilot_phase: u64,
    /// Held-out scattered-pilot prediction error normalized by expected pilot power.
    pub normalized_pilot_error: f64,
}
/// Fixed fractional-frequency correction referenced to an explicit sample origin.
#[derive(Debug, Clone)]
pub struct CarrierOffsetCorrection {
    offset_hz: f64,
    origin: u64,
}
impl CarrierOffsetCorrection {
    /// Creates a derotator within the unaliased half-subcarrier interval.
    /// # Arguments
    /// * `geometry` - Known numerology; integer-carrier acquisition is not included.
    /// * `offset_hz` - Observed received offset to subtract, strictly below half spacing.
    /// * `origin` - Absolute tick defining zero correction phase.
    /// # Returns
    /// Reusable correction with no per-block phase reset.
    /// # Errors
    /// Rejects nonfinite or out-of-range fractional offsets.
    pub fn new(
        geometry: &WaveformGeometry,
        offset_hz: f64,
        origin: u64,
    ) -> Result<Self, SimulationError> {
        if !offset_hz.is_finite()
            || offset_hz.abs()
                >= f64::from(geometry.settings().subcarrier_spacing.frequency_hz()) / 2.0
        {
            return Err(invalid(
                "Fractional CFO must be strictly within half a subcarrier spacing.",
            ));
        }
        Ok(Self { offset_hz, origin })
    }
    /// Derotates a received block without modifying it or resetting phase at its edges.
    /// # Arguments
    /// * `input` - Contiguous samples with absolute ticks no earlier than the origin.
    /// # Returns
    /// Owned corrected samples retaining their absolute indices.
    /// # Errors
    /// Rejects samples before origin, beyond exact f64 integer timing, or overflow.
    pub fn apply(&self, input: &SampleBlock) -> Result<SampleBlock, SimulationError> {
        let first = input
            .start_index()
            .checked_sub(self.origin)
            .ok_or_else(|| invalid("CFO block precedes correction origin."))?;
        if first
            .checked_add(input.samples().len() as u64)
            .is_none_or(|last| last > 1u64 << 53)
        {
            return Err(invalid(
                "CFO correction exceeds exact integer sample coordinates.",
            ));
        }
        let step = -std::f64::consts::TAU * self.offset_hz / f64::from(SAMPLE_RATE_HZ);
        let corrected = input
            .samples()
            .iter()
            .enumerate()
            .map(|(index, point)| {
                let (sine, cosine) = (step * (first + index as u64) as f64).sin_cos();
                [
                    point[0] * cosine - point[1] * sine,
                    point[0] * sine + point[1] * cosine,
                ]
            })
            .collect();
        SampleBlock::new(input.start_index(), corrected)
    }
}
/// Estimates CP timing and fractional CFO from received repeated samples.
/// # Arguments
/// * `geometry` - Known FFT/prefix/roll-off; no channel or boundary truth.
/// * `input` - Received samples covering one full period and the bounded search.
/// * `max_offset` - Inclusive timing search extent, at most one period.
/// * `minimum_correlation` - Required normalized squared CP correlation in (0,1].
/// # Returns
/// Observed timing plus CFO in Hz, aliased modulo subcarrier spacing.
/// The caller must restrict experiments to the unaliased interval.
/// # Errors
/// Rejects weak/ambiguous timing, nonfinite correlation or boundary overflow.
pub fn estimate_timing_and_cfo(
    geometry: &WaveformGeometry,
    input: &SampleBlock,
    max_offset: usize,
    minimum_correlation: f64,
) -> Result<(TimingEstimate, f64), SimulationError> {
    let timing = estimate_symbol_boundary(geometry, input, max_offset, minimum_correlation)?;
    let offset = (timing.first_symbol_sample - input.start_index()) as usize;
    let mut cross = [0.0, 0.0];
    for index in offset + geometry.roll_off_samples()..offset + geometry.prefix_samples() {
        let left = input.samples()[index];
        let right = input.samples()[index + geometry.fft_samples()];
        cross[0] += left[0] * right[0] + left[1] * right[1];
        cross[1] += left[0] * right[1] - left[1] * right[0];
    }
    let offset_hz = cross[1].atan2(cross[0]) * f64::from(SAMPLE_RATE_HZ)
        / (std::f64::consts::TAU * geometry.fft_samples() as f64);
    if !offset_hz.is_finite() {
        return Err(invalid("Nonfinite received CFO estimate."));
    }
    Ok((timing, offset_hz))
}
/// Finds pilot phase using scattered pilots held out from the continuous-pilot fit.
/// # Arguments
/// * `layout` - Receiver-known physical carrier map, including continuous pilots.
/// * `bins` - CFO-corrected received FFT bins; never transmitted labels or H.
/// * `reference_rms` - Known pilot scaling reference.
/// # Returns
/// Phase0..127 and its normalized prediction error; smooth channels are assumed
/// between continuous pilots. This is not PLC preamble/message acquisition.
/// # Errors
/// Rejects malformed/nonfinite bins, insufficient pilots, weak or ambiguous matches.
pub fn estimate_pilot_phase(
    layout: &CarrierLayout,
    bins: &[IqPair],
    reference_rms: f64,
) -> Result<(u64, f64), SimulationError> {
    if bins.len() != layout.static_roles().len()
        || bins.iter().flatten().any(|value| !value.is_finite())
    {
        return Err(invalid("Invalid received pilot-phase FFT grid."));
    }
    let sequence = PilotSequence::new(layout.subcarrier_spacing());
    let amplitude = PilotPower::new(reference_rms)?.amplitude();
    let reference = |bin: usize| amplitude * (1.0 - 2.0 * f64::from(sequence.bits()[bin]));
    let nodes: Vec<_> = layout
        .static_roles()
        .iter()
        .enumerate()
        .filter(|(_, role)| **role == PhysicalCarrierRole::ContinuousPilot)
        .map(|(bin, _)| (bin, bins[bin].map(|value| value / reference(bin))))
        .collect();
    let (response, _) = interpolate(&nodes, bins.len())?;
    let mut scores = Vec::with_capacity(128);
    for phase in 0..128 {
        let roles = layout.symbol_roles(phase);
        let mut error = 0.0;
        let mut power = 0.0;
        let mut count = 0;
        for (bin, role) in roles.iter().enumerate() {
            if *role != PhysicalCarrierRole::ScatteredPilot {
                continue;
            }
            let predicted = response[bin].map(|value| value * reference(bin));
            error += (bins[bin][0] - predicted[0]).powi(2) + (bins[bin][1] - predicted[1]).powi(2);
            power += predicted[0].powi(2) + predicted[1].powi(2);
            count += 1;
        }
        if count < 2 || power <= 0.0 || !error.is_finite() || !power.is_finite() {
            return Err(invalid(
                "Insufficient usable scattered pilots for phase acquisition.",
            ));
        }
        scores.push((phase, error / power));
    }
    scores.sort_by(|left, right| left.1.total_cmp(&right.1));
    if scores[0].1 > 0.2 || scores[1].1 - scores[0].1 < 0.1 {
        return Err(invalid("Weak or ambiguous received scattered-pilot phase."));
    }
    Ok(scores[0])
}
