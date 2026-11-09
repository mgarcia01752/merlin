/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 sections7.5.13 and7.5.15: PLC and physical pilot locations.
use crate::{
    error::invalid,
    transmission::{SubcarrierSpacing, TransmissionSettings},
    types::{FrequencyHz, SubcarrierIdx},
    SimulationError,
};
use serde::{Deserialize, Serialize};

/// Physical scattered-pilot pattern period in OFDM symbols.
pub const PILOT_PERIOD_SYMBOLS: u64 = crate::types::PLC_FRAME_SYMBOLS;
const PILOT_FREQUENCY_PERIOD: usize = 128;
const PLC_NEIGHBORHOOD_HZ: u32 = 6_000_000;
const PLC_GRID_HZ: u32 = 1_000_000;
const MIN_ADDITIONAL_PILOTS: usize = 8;
const MAX_ADDITIONAL_PILOTS: usize = 120;

/// Physical-bin classification; NCP remains a transported cell, not a fixed bin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PhysicalCarrierRole {
    /// Candidate data location; transported NCP cells may subsequently occupy it.
    Data,
    Excluded,
    Plc,
    ContinuousPilot,
    ScatteredPilot,
}

/// Explicit caller-supplied layout in PHY increasing-frequency bin order.
///
/// Geometry and collisions are validated. Additional-pilot generation and full
/// spectral/PLC-message compliance are not certified by this representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CarrierLayoutSettings {
    /// Absolute frequency of PHY bin0, not an FFT-library array convention.
    pub zero_frequency_hz: FrequencyHz,
    /// First bin inside the active envelope, inclusive.
    pub first_active_bin: SubcarrierIdx,
    /// End of the active envelope, exclusive; internal exclusions remain explicit.
    pub active_end_exclusive: SubcarrierIdx,
    /// First PLC bin; width follows spacing and is never caller-selected.
    pub plc_first_bin: SubcarrierIdx,
    /// Internal excluded bins; duplicates and out-of-envelope entries are rejected.
    pub excluded_bins: Vec<SubcarrierIdx>,
    /// Additional continuous pilots, excluding the eight predefined PLC pilots.
    pub additional_continuous_pilots: Vec<SubcarrierIdx>,
}

/// Immutable validated physical map, reusable across arbitrary symbol indices.
pub struct CarrierLayout {
    spacing: SubcarrierSpacing,
    zero_frequency_hz: FrequencyHz,
    static_roles: Vec<PhysicalCarrierRole>,
    scattered_anchor: usize,
}
impl CarrierLayout {
    /// Builds static bin roles without changing either input.
    /// # Arguments
    /// * `transmission` - Validated fixed channel numerology.
    /// * `layout` - Explicit geometry and additional-pilot positions.
    /// # Returns
    /// Owned FFT-sized map with physical exclusions, PLC and continuous pilots.
    /// # Errors
    /// Rejects invalid geometry, overflow, duplicate assignments and collisions.
    pub fn new(
        transmission: &TransmissionSettings,
        layout: &CarrierLayoutSettings,
    ) -> Result<Self, SimulationError> {
        Self::build(transmission, layout, true)
    }

    pub(super) fn planning_base(
        transmission: &TransmissionSettings,
        layout: &CarrierLayoutSettings,
    ) -> Result<Self, SimulationError> {
        Self::build(transmission, layout, false)
    }

    fn build(
        transmission: &TransmissionSettings,
        layout: &CarrierLayoutSettings,
        include_additional: bool,
    ) -> Result<Self, SimulationError> {
        transmission.validate()?;
        let spacing = transmission.subcarrier_spacing;
        let fft_size = spacing.fft_samples() as usize;
        let first = usize::from(layout.first_active_bin);
        let end = usize::from(layout.active_end_exclusive);
        if first >= end || end > fft_size {
            return Err(invalid("Active envelope is outside the FFT or empty."));
        }
        let envelope_hz = (end - first) as u32 * spacing.frequency_hz();
        if envelope_hz > transmission.occupied_bandwidth_hz {
            return Err(invalid(
                "Active envelope exceeds occupied channel bandwidth.",
            ));
        }
        // Validate every possible bin frequency once, before storing the map.
        layout
            .zero_frequency_hz
            .checked_add((fft_size as u32 - 1) * spacing.frequency_hz())
            .ok_or_else(|| invalid("FFT bin frequency overflows FrequencyHz."))?;
        let (plc_width, neighborhood_side): (usize, usize) = match spacing {
            SubcarrierSpacing::Khz25 => (16, 112),
            SubcarrierSpacing::Khz50 => (8, 56),
        };
        let plc_first = usize::from(layout.plc_first_bin);
        let plc_end = plc_first + plc_width;
        let neighborhood_first = plc_first
            .checked_sub(neighborhood_side)
            .ok_or_else(|| invalid("PLC neighborhood falls below the FFT."))?;
        let neighborhood_end = plc_end + neighborhood_side;
        if neighborhood_first < first || neighborhood_end > end {
            return Err(invalid("The entire PLC6MHz neighborhood must be active."));
        }
        let neighborhood_frequency =
            layout.zero_frequency_hz + neighborhood_first as u32 * spacing.frequency_hz();
        if neighborhood_frequency % PLC_GRID_HZ != 0 {
            return Err(invalid("PLC6MHz neighborhood must start on a1MHz grid."));
        }
        debug_assert_eq!(
            (neighborhood_end - neighborhood_first) as u32 * spacing.frequency_hz(),
            PLC_NEIGHBORHOOD_HZ
        );
        let mut roles = vec![PhysicalCarrierRole::Excluded; fft_size];
        roles[first..end].fill(PhysicalCarrierRole::Data);
        if layout.excluded_bins.len() > fft_size {
            return Err(invalid("Too many excluded-bin entries."));
        }
        for &bin in &layout.excluded_bins {
            let bin = usize::from(bin);
            if !(first..end).contains(&bin)
                || (neighborhood_first..neighborhood_end).contains(&bin)
                || roles[bin] != PhysicalCarrierRole::Data
            {
                return Err(invalid("Invalid, duplicate or PLC-neighborhood exclusion."));
            }
            roles[bin] = PhysicalCarrierRole::Excluded;
        }
        roles[plc_first..plc_end].fill(PhysicalCarrierRole::Plc);
        for bin in predefined_pilots(spacing, layout.plc_first_bin)? {
            roles[usize::from(bin)] = PhysicalCarrierRole::ContinuousPilot;
        }
        if include_additional
            && !(MIN_ADDITIONAL_PILOTS..=MAX_ADDITIONAL_PILOTS)
                .contains(&layout.additional_continuous_pilots.len())
        {
            return Err(invalid("Provide8..120 additional continuous pilots."));
        }
        let additional = if include_additional {
            layout.additional_continuous_pilots.as_slice()
        } else {
            &[]
        };
        for &bin in additional {
            let bin = usize::from(bin);
            if !(first..end).contains(&bin)
                || (neighborhood_first..neighborhood_end).contains(&bin)
                || roles[bin] != PhysicalCarrierRole::Data
            {
                return Err(invalid(
                    "Additional continuous pilot collides or is outside its domain.",
                ));
            }
            roles[bin] = PhysicalCarrierRole::ContinuousPilot;
        }
        Ok(Self {
            spacing,
            zero_frequency_hz: layout.zero_frequency_hz,
            static_roles: roles,
            scattered_anchor: plc_end,
        })
    }

    /// Returns the fixed subcarrier spacing used to construct this geometry.
    /// # Returns
    ///25or50kHz numerology for reduced-domain interleaver validation.
    pub fn subcarrier_spacing(&self) -> SubcarrierSpacing {
        self.spacing
    }

    /// Returns the protected6MHz PLC neighborhood in absolute Hz.
    /// # Returns
    /// Half-open physical interval including the PLC and its mandatory nearby pilots.
    /// # Errors
    /// Rejects missing PLC state or unrepresentable frequency arithmetic.
    pub fn plc_neighborhood_hz(&self) -> Result<std::ops::Range<FrequencyHz>, SimulationError> {
        let first = self
            .static_roles
            .iter()
            .position(|role| *role == PhysicalCarrierRole::Plc)
            .ok_or_else(|| invalid("Missing PLC bins."))?;
        let count = self
            .static_roles
            .iter()
            .filter(|role| **role == PhysicalCarrierRole::Plc)
            .count() as u32;
        let guard = (PLC_NEIGHBORHOOD_HZ - count * self.spacing.frequency_hz()) / 2;
        let start = self
            .frequency_hz(first as u16)
            .and_then(|frequency| frequency.checked_sub(guard))
            .ok_or_else(|| invalid("PLC neighborhood frequency underflow."))?;
        let stop = start
            .checked_add(PLC_NEIGHBORHOOD_HZ)
            .ok_or_else(|| invalid("PLC neighborhood frequency overflow."))?;
        Ok(start..stop)
    }

    /// Borrows static assignments before the phase-dependent scattered pattern.
    /// # Returns
    /// Immutable full FFT grid, including all excluded, PLC and continuous-pilot bins.
    pub fn static_roles(&self) -> &[PhysicalCarrierRole] {
        &self.static_roles
    }

    /// Returns one role per physical FFT bin for a symbol after the PLC preamble.
    /// # Arguments
    /// * `symbols_since_preamble_end` - Zero denotes the first symbol following
    ///   the eight-symbol PLC preamble; wraps modulo128, including preamble phases.
    /// # Returns
    /// Owned map; exclusions, PLC and continuous pilots override scattered pilots.
    pub fn symbol_roles(&self, symbols_since_preamble_end: u64) -> Vec<PhysicalCarrierRole> {
        let phase = (symbols_since_preamble_end % PILOT_PERIOD_SYMBOLS) as usize;
        let shift = match self.spacing {
            SubcarrierSpacing::Khz50 => phase,
            SubcarrierSpacing::Khz25 => 2 * (phase % 64) + phase / 64,
        };
        let residue = (self.scattered_anchor + shift) % PILOT_FREQUENCY_PERIOD;
        let mut roles = self.static_roles.clone();
        for bin in (residue..roles.len()).step_by(PILOT_FREQUENCY_PERIOD) {
            if roles[bin] == PhysicalCarrierRole::Data {
                roles[bin] = PhysicalCarrierRole::ScatteredPilot;
            }
        }
        roles
    }

    /// Returns the absolute frequency of a physical PHY bin, without reordering.
    /// # Arguments
    /// * `bin` - Increasing-frequency PHY bin index.
    /// # Returns
    /// Frequency in Hz, or None when the index is outside the configured FFT.
    pub fn frequency_hz(&self, bin: SubcarrierIdx) -> Option<FrequencyHz> {
        (usize::from(bin) < self.static_roles.len())
            .then(|| self.zero_frequency_hz + u32::from(bin) * self.spacing.frequency_hz())
    }
}

/// Returns the eight fixed pilots surrounding a PLC block.
/// # Arguments
/// * `spacing` - Frozen25/50kHz numerology.
/// * `plc_first` - First PLC bin in increasing physical order.
/// # Returns
/// Owned physical-bin identities from PHY I20 section7.5.13.
/// # Errors
/// Rejects a PLC position whose pilot neighborhood falls outside the FFT.
pub fn predefined_pilots(
    spacing: SubcarrierSpacing,
    plc_first: SubcarrierIdx,
) -> Result<Vec<SubcarrierIdx>, SimulationError> {
    let (width, distances): (u16, [u16; 4]) = match spacing {
        SubcarrierSpacing::Khz25 => (16, [30, 48, 70, 94]),
        SubcarrierSpacing::Khz50 => (8, [15, 24, 35, 47]),
    };
    let mut bins = Vec::new();
    for distance in distances {
        let low = plc_first
            .checked_sub(distance)
            .ok_or_else(|| invalid("PLC pilots fall below the FFT."))?;
        let high = plc_first
            .checked_add(width - 1)
            .and_then(|value| value.checked_add(distance))
            .ok_or_else(|| invalid("PLC pilot overflow."))?;
        if u32::from(high) >= spacing.fft_samples() {
            return Err(invalid("PLC pilots exceed the FFT."));
        }
        bins.extend([low, high]);
    }
    Ok(bins)
}
