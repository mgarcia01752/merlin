/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Frequency-domain Band12 ingress approximation; no LTE or DOCSIS waveform synthesis.
use crate::{
    error::{invalid, LabError},
    scenario::grid::FrequencyGrid,
    types::FrequencyHz,
};
use serde::{Deserialize, Serialize};

/// Nominal width of this fixed LTE-inspired test scenario, not configurable LTE support.
pub const CHANNEL_BANDWIDTH_HZ: FrequencyHz = 10_000_000;
/// Flat core representing50 LTE resource blocks at180kHz each.
pub const FLAT_CORE_BANDWIDTH_HZ: FrequencyHz = 9_000_000;
const BAND12_START_HZ: FrequencyHz = 729_000_000;
const BAND12_STOP_HZ: FrequencyHz = 746_000_000;

/// Fixed10MHz Band12 spectral approximation at an equalized residual-power plane.
/// Signal power is normalized independently to one for each DOCSIS carrier.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LteDownlinkIngress {
    /// Nominal RF center; the complete10MHz envelope must fit inside Band12.
    pub center_frequency_hz: FrequencyHz,
    /// Core interference power divided by local wanted signal power, in dB.
    /// This is neither total LTE power nor an absolute dBm/dBmV measurement.
    pub interference_to_signal_db: f64,
}

/// Directly inspectable carrier results; no original observation is mutated.
#[derive(Debug, Clone)]
pub struct IngressResult {
    /// Synthetic total MER; missing input readings remain unavailable.
    pub mer_db: Vec<Option<f64>>,
    /// Added linear interference/signal ratios at original carrier centers.
    pub interference_to_signal_power: Vec<f64>,
}

impl LteDownlinkIngress {
    /// Combines existing residual power with a center-sampled synthetic ingress envelope.
    /// No thermal floor or second independent noise source is added.
    ///
    /// # Arguments
    /// * `grid` - Validated physical carrier coordinates; no index-only remapping.
    /// * `mer_db` - Existing total residual MER in dB, with missing readings as None.
    ///
    /// # Returns
    /// Owned synthetic MER and added linear powers in unchanged carrier order.
    /// Unaffected readings are returned exactly, including nulls.
    ///
    /// # Errors
    /// Rejects invalid geometry, length, nonfinite/nonrepresentable powers, an
    /// envelope outside Band12 or outside the measured carrier-center coverage.
    pub fn apply(
        &self,
        grid: &FrequencyGrid,
        mer_db: &[Option<f64>],
    ) -> Result<IngressResult, LabError> {
        grid.validate()?;
        let envelope_start = self
            .center_frequency_hz
            .checked_sub(CHANNEL_BANDWIDTH_HZ / 2)
            .ok_or_else(|| invalid("LTE envelope lower frequency underflow."))?;
        let envelope_stop = self
            .center_frequency_hz
            .checked_add(CHANNEL_BANDWIDTH_HZ / 2)
            .ok_or_else(|| invalid("LTE envelope upper frequency overflow."))?;
        let core_power = 10.0_f64.powf(self.interference_to_signal_db / 10.0);
        if mer_db.len() != grid.sample_count
            || envelope_start < BAND12_START_HZ
            || envelope_stop > BAND12_STOP_HZ
            || envelope_start < grid.frequency(0)?
            || envelope_stop > grid.frequency(grid.sample_count - 1)?
            || !self.interference_to_signal_db.is_finite()
            || !core_power.is_finite()
            || core_power <= 0.0
        {
            return Err(invalid(
                "Invalid LTE ingress power, coverage or carrier count.",
            ));
        }
        let mut result = IngressResult {
            mer_db: Vec::with_capacity(mer_db.len()),
            interference_to_signal_power: Vec::with_capacity(mer_db.len()),
        };
        for (offset, reading) in mer_db.iter().enumerate() {
            let distance_hz = grid.frequency(offset)?.abs_diff(self.center_frequency_hz);
            let weight = envelope_weight(distance_hz);
            let interference_power = core_power * weight;
            let synthetic_mer = if let Some(mer) = reading {
                let baseline_power = 10.0_f64.powf(-mer / 10.0);
                if !mer.is_finite() || !baseline_power.is_finite() || baseline_power <= 0.0 {
                    return Err(invalid(
                        "RxMER cannot be represented as finite residual power.",
                    ));
                }
                if weight == 0.0 {
                    Some(*mer)
                } else {
                    let combined_mer = -10.0 * (baseline_power + interference_power).log10();
                    if !combined_mer.is_finite() {
                        return Err(invalid("Combined ingress residual power overflow."));
                    }
                    Some(combined_mer)
                }
            } else {
                None
            };
            result.mer_db.push(synthetic_mer);
            result.interference_to_signal_power.push(interference_power);
        }
        Ok(result)
    }
}

// A deliberate engineering envelope, not an LTE emission mask: flat core and
// cosine power shoulders, with no modeled energy outside the nominal channel.
fn envelope_weight(distance_hz: FrequencyHz) -> f64 {
    let core_half_width_hz = FLAT_CORE_BANDWIDTH_HZ / 2;
    let channel_half_width_hz = CHANNEL_BANDWIDTH_HZ / 2;
    if distance_hz <= core_half_width_hz {
        1.0
    } else if distance_hz >= channel_half_width_hz {
        0.0
    } else {
        let shoulder_fraction = f64::from(distance_hz - core_half_width_hz)
            / f64::from(channel_half_width_hz - core_half_width_hz);
        0.5 * (1.0 + (std::f64::consts::PI * shoulder_fraction).cos())
    }
}
