/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 section7.5.15.4: amplitude2x other-subcarrier RMS, power4x.
use crate::{error::invalid, types::IqPair, SimulationError};
use serde::Serialize;
const PILOT_AMPLITUDE_RATIO: f64 = 2.0;
/// Validated pilot scale in the same I/Q units as the supplied reference RMS.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct PilotPower {
    reference_rms: f64,
    pilot_amplitude: f64,
    reference_power: f64,
    pilot_power: f64,
    boost_db: f64,
}
impl PilotPower {
    /// Computes pilot amplitude and power from an explicit nonpilot reference.
    /// # Arguments
    /// * `reference_rms` - Positive finite RMS amplitude of other subcarriers;
    ///   1.0 is the unit-average-energy reference, not an absolute RF power level.
    /// # Returns
    /// Owned scaling metadata; this does not estimate RMS from a physical frame.
    /// # Errors
    /// Rejects invalid values or squared amplitudes that overflow/underflow to zero.
    pub fn new(reference_rms: f64) -> Result<Self, SimulationError> {
        let pilot_amplitude = PILOT_AMPLITUDE_RATIO * reference_rms;
        let reference_power = reference_rms * reference_rms;
        let pilot_power = pilot_amplitude * pilot_amplitude;
        if reference_rms <= 0.0
            || !reference_rms.is_finite()
            || !pilot_power.is_finite()
            || reference_power == 0.0
        {
            return Err(invalid(
                "Pilot reference RMS must be positive with finite nonzero powers.",
            ));
        }
        Ok(Self {
            reference_rms,
            pilot_amplitude,
            reference_power,
            pilot_power,
            boost_db: 20.0 * PILOT_AMPLITUDE_RATIO.log10(),
        })
    }
    /// Returns the caller-supplied other-subcarrier RMS amplitude.
    /// # Returns
    /// Positive finite reference in the same units as generated pilot I/Q.
    pub fn reference_rms(self) -> f64 {
        self.reference_rms
    }
    /// Returns the generated pilot amplitude, twice the reference RMS.
    /// # Returns
    /// Positive finite pilot magnitude, not a dB value.
    pub fn amplitude(self) -> f64 {
        self.pilot_amplitude
    }
    /// Scales an unboosted complex constellation point without changing it.
    /// # Arguments
    /// * `iq` - Finite normalized I/Q point; pilot producers supply unit BPSK.
    /// # Returns
    /// Owned scaled I/Q, with the same sign and phase.
    /// # Errors
    /// Rejects nonfinite input or overflowing scaled coordinates.
    pub fn scale(self, iq: IqPair) -> Result<IqPair, SimulationError> {
        let scaled = iq.map(|value| value * self.pilot_amplitude);
        if iq
            .iter()
            .chain(scaled.iter())
            .any(|value| !value.is_finite())
        {
            return Err(invalid("Invalid or overflowing pilot I/Q scaling."));
        }
        Ok(scaled)
    }
}
