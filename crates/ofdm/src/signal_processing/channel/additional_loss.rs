/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit magnitude-only additional loss; measured observations remain immutable.
use crate::{error::invalid, types::FrequencyHz, SimulationError};
use serde::{Deserialize, Serialize};

/// Cable-distance entry units; the serialized value retains the user's selection.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LengthUnit {
    Feet,
    Metres,
}
/// Pinned attenuation dataset, independent of velocity-only echo presets.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CableModel {
    Belden1694aV1,
}
/// Additional response configuration; Off preserves the original numerical path.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum AdditionalLoss {
    #[default]
    Off,
    /// Signed high-edge minus low-edge gain; low-edge gain is zero dB.
    Manual { tilt_db: f64 },
    /// Full added cable attenuation, including low-edge insertion loss.
    Cable {
        cable: CableModel,
        length: f64,
        unit: LengthUnit,
    },
}

// Belden 1694A, nominal attenuation dB/100 ft, revision 0.716 (2026-05-27).
// https://www.belden.com/products/cable/coax-triax-cable/75-ohm-coax-cable/1694a
// Reviewed 2026-11-10. Frequency MHz; linear interpolation is MERLIN's model.
const BELDEN_1694A: &[(f64, f64)] = &[
    (1.0, 0.24),
    (3.58, 0.44),
    (5.0, 0.52),
    (6.0, 0.57),
    (7.0, 0.61),
    (10.0, 0.71),
    (12.0, 0.78),
    (25.0, 1.08),
    (67.5, 1.65),
    (71.5, 1.69),
    (88.5, 1.86),
    (100.0, 1.95),
    (135.0, 2.24),
    (143.0, 2.3),
    (180.0, 2.57),
    (270.0, 3.17),
    (360.0, 3.69),
    (540.0, 4.5),
    (720.0, 5.3),
    (750.0, 5.4),
    (1000.0, 6.3),
    (1500.0, 7.8),
    (2000.0, 9.2),
    (2250.0, 9.8),
    (3000.0, 11.5),
];
impl AdditionalLoss {
    /// Validates application parameter limits without requiring an accepted channel.
    /// # Returns
    /// Unit for finite manual slope or bounded nonnegative cable length.
    /// # Errors
    /// Rejects slopes outside ±30 dB and lengths above 1000 ft / 304.8 m.
    pub fn validate(&self) -> Result<(), SimulationError> {
        let valid = match *self {
            Self::Off => true,
            Self::Manual { tilt_db } => tilt_db.is_finite() && tilt_db.abs() <= 30.0,
            Self::Cable { length, unit, .. } => {
                let limit = match unit {
                    LengthUnit::Feet => 1000.0,
                    LengthUnit::Metres => 304.8,
                };
                length.is_finite() && (0.0..=limit).contains(&length)
            }
        };
        if !valid {
            return Err(invalid(
                "Additional loss exceeds the slope or cable-length limits.",
            ));
        }
        Ok(())
    }
    /// Evaluates additional gain in dB at one physical carrier center.
    /// # Arguments
    /// * `frequency` - Carrier center in Hz, within the accepted active band.
    /// * `low` - Lowest active carrier center in Hz.
    /// * `high` - Highest active carrier center in Hz, strictly greater than low.
    /// # Returns
    /// Signed magnitude gain; cable gain includes full insertion loss.
    /// # Errors
    /// Rejects invalid settings/bands, unsupported cable frequencies or loss above60 dB.
    pub fn gain_db(
        &self,
        frequency: FrequencyHz,
        low: FrequencyHz,
        high: FrequencyHz,
    ) -> Result<f64, SimulationError> {
        self.validate()?;
        if high <= low || frequency < low || frequency > high {
            return Err(invalid("Invalid additional-loss frequency band."));
        }
        let gain = match *self {
            Self::Off => 0.0,
            Self::Manual { tilt_db } => {
                tilt_db * f64::from(frequency - low) / f64::from(high - low)
            }
            Self::Cable { length, unit, .. } => {
                let mhz = f64::from(frequency) / 1_000_000.0;
                let points = BELDEN_1694A;
                if mhz < points[0].0 || mhz > points[points.len() - 1].0 {
                    return Err(invalid(
                        "Belden 1694A requires active frequencies within 1–3000 MHz.",
                    ));
                }
                let pair = points
                    .windows(2)
                    .find(|pair| mhz <= pair[1].0)
                    .ok_or_else(|| invalid("Missing cable attenuation interval."))?;
                let attenuation = pair[0].1
                    + (pair[1].1 - pair[0].1) * (mhz - pair[0].0) / (pair[1].0 - pair[0].0);
                let feet = match unit {
                    LengthUnit::Feet => length,
                    LengthUnit::Metres => length / 0.3048,
                };
                -attenuation * feet / 100.0
            }
        };
        if !gain.is_finite() || gain.abs() > 60.0 {
            return Err(invalid(
                "Additional loss exceeds 60 dB over the active channel.",
            ));
        }
        Ok(gain)
    }
}
