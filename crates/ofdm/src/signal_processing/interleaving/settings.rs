/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Validated reduced-domain geometry from PHY I20 section7.5.6, PDF129–130.
use crate::{error::invalid, transmission::SubcarrierSpacing, LabError};
/// Choice of transmitter delays or complementary receiver delays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterleaverDirection {
    Transmit,
    Receive,
}
/// Immutable validated settings; no physical PLC/CP/excluded bins in this domain.
#[derive(Debug, Clone, Copy)]
pub struct InterleaverSettings {
    spacing: SubcarrierSpacing,
    cells: usize,
    depth: usize,
}
impl InterleaverSettings {
    /// Validates fixed reduced-domain size and convolutional depth.
    /// # Arguments
    /// * `spacing` -25or50kHz, selecting PHY mode-specific bounds.
    /// * `cells` - NI data/NCP plus scattered-pilot positions; excludes PLC/CP/exclusions.
    /// * `depth` - M,1..16 at25kHz or1..32 at50kHz.
    /// # Returns
    /// Owned immutable settings; small synthetic NI is allowed for algorithm tests.
    /// # Errors
    /// Rejects empty/oversized domains or unsupported depths.
    pub fn new(spacing: SubcarrierSpacing, cells: usize, depth: usize) -> Result<Self, LabError> {
        if cells == 0
            || cells > spacing.max_interleaver_cells()
            || depth == 0
            || depth > usize::from(spacing.max_time_interleaver_depth())
        {
            return Err(invalid("Invalid interleaver NI or spacing-specific depth."));
        }
        Ok(Self {
            spacing,
            cells,
            depth,
        })
    }
    /// Returns the validated fixed domain length.
    /// # Returns
    /// NI positions including pilot placeholders and NCP cells.
    pub fn cells(self) -> usize {
        self.cells
    }
    /// Returns convolutional depth in OFDM-symbol periods.
    /// # Returns
    /// M; individual TX branches delay0..M-1 symbols.
    pub fn depth(self) -> usize {
        self.depth
    }
    /// Returns numerology used to validate the settings.
    /// # Returns
    ///25or50kHz spacing, without changing the configuration.
    pub fn spacing(self) -> SubcarrierSpacing {
        self.spacing
    }
    /// Returns latency of a matched transmitter/receiver delay pair.
    /// # Returns
    /// M-1 complete symbol periods; no waveform or decoder latency is included.
    pub fn pair_delay_symbols(self) -> usize {
        self.depth - 1
    }
}
