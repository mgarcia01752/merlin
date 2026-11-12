/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit synthetic RxMER profile edits; never infers channel phase or RF noise.
use crate::{scenario::FrequencyGrid, types::FrequencyHz, SimulationError};
use serde::{Deserialize, Serialize};

/// Application bounds, independent of a PNM wire-format representation.
pub const MAX_DESIGN_CAPTURES: usize = 256;
/// Maximum explicitly replaced frequency ranges in one recipe.
pub const MAX_VALUE_EDITS: usize = 32;
/// One-based inclusive capture selection, independent of subcarrier coordinates.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum CaptureScope {
    #[default]
    All,
    Range {
        first: usize,
        last: usize,
    },
}
impl CaptureScope {
    /// Validates a scope against the whole baseline collection.
    /// # Arguments
    /// * `count` - Number of baseline captures,1 through256.
    /// # Returns
    /// Unit if the inclusive scope is reachable.
    /// # Errors
    /// Rejects zero, reversed or out-of-collection capture indexes.
    pub fn validate(&self, count: usize) -> Result<(), SimulationError> {
        if !(1..=MAX_DESIGN_CAPTURES).contains(&count)
            || matches!(self,Self::Range{first,last} if *first==0 || first>last || *last>count)
        {
            return Err(fail(
                "Capture range must be ordered and inside 1 through the collection count.",
            ));
        }
        Ok(())
    }
    /// Tests a validated one-based capture position.
    /// # Arguments
    /// * `position` - One-based capture position.
    /// # Returns
    /// Whether the effect applies at this position; no values are altered.
    pub fn contains(&self, position: usize) -> bool {
        match *self {
            Self::All => true,
            Self::Range { first, last } => position >= first && position <= last,
        }
    }
}
/// Physical interpretation used only to suggest crest spacing; no phase is inferred.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum RippleSpacing {
    Frequency {
        spacing_hz: FrequencyHz,
    },
    EchoDistance {
        distance_metres: f64,
        velocity_factor: f64,
    },
}
impl RippleSpacing {
    /// Evaluates an explicit period or a round-trip echo-cavity helper.
    /// # Returns
    /// Crest spacing in Hz rounded to the nearest integer Hz.
    /// # Errors
    /// Rejects nonpositive/unbounded distance, velocity outside(0,1], or period
    /// outside the repository's positive u32 Hz domain.
    pub fn frequency_hz(&self) -> Result<FrequencyHz, SimulationError> {
        let hz = match *self {
            Self::Frequency { spacing_hz } => f64::from(spacing_hz),
            Self::EchoDistance {
                distance_metres,
                velocity_factor,
            } => {
                if !distance_metres.is_finite()
                    || !(0.001..=10000.0).contains(&distance_metres)
                    || !velocity_factor.is_finite()
                    || velocity_factor <= 0.0
                    || velocity_factor > 1.0
                {
                    return Err(fail("Distance must be 0.001–10000 m and velocity factor greater than 0 and at most 1."));
                }
                299_792_458.0 * velocity_factor / (2.0 * distance_metres)
            }
        };
        if !hz.is_finite() || hz.round() < 1.0 || hz.round() > f64::from(u32::MAX) {
            return Err(fail(
                "Ripple crest spacing is outside the positive Hz range.",
            ));
        }
        Ok(hz.round() as u32)
    }
}
/// Signed high-edge minus low-edge MER change, low edge anchored at zero change.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tilt {
    pub scope: CaptureScope,
    pub tilt_db: f64,
}
/// Cosine-shaped MER edit; peak-to-peak depth is independent of physical echo strength.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ripple {
    pub scope: CaptureScope,
    pub peak_to_peak_db: f64,
    pub spacing: RippleSpacing,
}
/// Lowered MER over a frequency band with optional raised-cosine edge transitions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Notch {
    pub scope: CaptureScope,
    pub center_hz: FrequencyHz,
    pub bandwidth_hz: FrequencyHz,
    pub depth_db: f64,
    pub edge_hz: FrequencyHz,
}
/// Explicit final value replacement on exact carrier centers; missing values persist.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValueEdit {
    pub scope: CaptureScope,
    pub first_hz: FrequencyHz,
    pub last_hz: FrequencyHz,
    pub value_db: f64,
}
/// Version-neutral typed edit recipe. File/WB schema versioning belongs to adapters.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RxmerRecipe {
    pub tilt: Option<Tilt>,
    pub ripple: Option<Ripple>,
    pub notch: Option<Notch>,
    pub edits: Vec<ValueEdit>,
}
fn fail(message: &str) -> SimulationError {
    SimulationError::new("RxmerDesignerInvalid", message)
}
fn bounded(value: f64, low: f64, high: f64) -> bool {
    value.is_finite() && (low..=high).contains(&value)
}
impl RxmerRecipe {
    /// Validates all rules before numerical work or draft mutation.
    /// # Arguments
    /// * `grid` - Original exact carrier centers, retained without resampling.
    /// * `count` - Full baseline collection count, not current playback selection.
    /// # Returns
    /// Unit for bounded, reachable frequency/capture scopes and finite parameters.
    /// # Errors
    /// Rejects excessive edits, unsupported values, off-grid replacements or notch
    /// bands extending outside the source grid. Input data remain unchanged.
    pub fn validate(&self, grid: &FrequencyGrid, count: usize) -> Result<(), SimulationError> {
        grid.validate()?;
        CaptureScope::All.validate(count)?;
        let low = grid.frequency(0)?;
        let high = grid.frequency(grid.sample_count - 1)?;
        if let Some(tilt) = &self.tilt {
            tilt.scope.validate(count)?;
            if !bounded(tilt.tilt_db, -60.0, 60.0) {
                return Err(fail("RxMER tilt must be finite within ±60 dB."));
            }
        }
        if let Some(ripple) = &self.ripple {
            ripple.scope.validate(count)?;
            ripple.spacing.frequency_hz()?;
            if !bounded(ripple.peak_to_peak_db, 0.0, 60.0) {
                return Err(fail("Ripple peak-to-peak depth must be 0–60 dB."));
            }
        }
        if let Some(notch) = &self.notch {
            notch.scope.validate(count)?;
            let half = f64::from(notch.bandwidth_hz) / 2.0;
            if notch.bandwidth_hz == 0
                || !bounded(notch.depth_db, 0.0, 60.0)
                || f64::from(notch.edge_hz) > half
                || f64::from(notch.center_hz) - half < f64::from(low)
                || f64::from(notch.center_hz) + half > f64::from(high)
            {
                return Err(fail("Notch needs positive bandwidth inside the source band, 0–60 dB depth and edges at most half its width."));
            }
        }
        if self.edits.len() > MAX_VALUE_EDITS {
            return Err(fail("Use at most 32 explicit value-edit ranges."));
        }
        for edit in &self.edits {
            edit.scope.validate(count)?;
            if edit.first_hz < low
                || edit.last_hz > high
                || edit.first_hz > edit.last_hz
                || (edit.first_hz - low) % grid.spacing_hz != 0
                || (edit.last_hz - low) % grid.spacing_hz != 0
                || !bounded(edit.value_db, -30.0, 120.0)
            {
                return Err(fail("Value edits need ordered exact carrier centers inside the grid and MER within −30 to 120 dB."));
            }
        }
        Ok(())
    }
    /// Derives one frame from its immutable baseline, never from an earlier result.
    /// # Arguments
    /// * `grid` - Full original source grid.
    /// * `baseline` - Same-length optional MER samples; None remains None.
    /// * `position`, `count` - One-based capture position and collection size.
    /// # Returns
    /// Owned designed dB values in original order. Applies tilt, ripple, notch,
    /// then explicit replacements in listed order. No simulation RNG is consumed.
    /// # Errors
    /// Rejects invalid recipes, positions, length, nonfinite baseline values or any
    /// designed result outside−30 to120dB. Never clips or partially changes inputs.
    pub fn apply(
        &self,
        grid: &FrequencyGrid,
        baseline: &[Option<f64>],
        position: usize,
        count: usize,
    ) -> Result<Vec<Option<f64>>, SimulationError> {
        self.validate(grid, count)?;
        if position == 0
            || position > count
            || baseline.len() != grid.sample_count
            || baseline.iter().flatten().any(|v| !v.is_finite())
        {
            return Err(fail(
                "Baseline length, values or capture position are invalid.",
            ));
        }
        let low = grid.frequency(0)?;
        let high = grid.frequency(grid.sample_count - 1)?;
        let period = self
            .ripple
            .as_ref()
            .map(|rule| rule.spacing.frequency_hz())
            .transpose()?;
        let mut result = Vec::with_capacity(baseline.len());
        for (offset, value) in baseline.iter().enumerate() {
            let Some(mut value) = *value else {
                result.push(None);
                continue;
            };
            let frequency = grid.frequency(offset)?;
            if let Some(rule) = self
                .tilt
                .as_ref()
                .filter(|rule| rule.scope.contains(position))
            {
                value += rule.tilt_db * f64::from(frequency - low) / f64::from((high - low).max(1));
            }
            if let Some(rule) = self
                .ripple
                .as_ref()
                .filter(|rule| rule.scope.contains(position))
            {
                value += rule.peak_to_peak_db / 2.0
                    * (std::f64::consts::TAU * f64::from(frequency - low)
                        / f64::from(period.unwrap_or(1)))
                    .cos();
            }
            if let Some(rule) = self
                .notch
                .as_ref()
                .filter(|rule| rule.scope.contains(position))
            {
                let distance = (f64::from(frequency) - f64::from(rule.center_hz)).abs();
                let half = f64::from(rule.bandwidth_hz) / 2.0;
                let edge = f64::from(rule.edge_hz);
                if distance <= half {
                    let weight = if edge == 0.0 || distance <= half - edge {
                        1.0
                    } else {
                        (1.0 + (std::f64::consts::PI * (distance - half + edge) / edge).cos()) / 2.0
                    };
                    value -= rule.depth_db * weight;
                }
            }
            for edit in self.edits.iter().filter(|rule| {
                rule.scope.contains(position)
                    && frequency >= rule.first_hz
                    && frequency <= rule.last_hz
            }) {
                value = edit.value_db;
            }
            if !bounded(value, -30.0, 120.0) {
                return Err(fail("Designed RxMER exceeds −30 to 120 dB; reduce the edits. Values are never clipped."));
            }
            result.push(Some(value));
        }
        Ok(result)
    }
}
