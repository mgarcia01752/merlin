/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! PHY I20 Table 7 p48, Tables 9/10 pp67–68 and Table 51 frame bounds.
use crate::{error::invalid, PhyResult};
use serde::{Deserialize, Serialize};

/// Fixed upstream sample rate, in samples per second.
pub const SAMPLE_RATE_HZ: u32 = 102_400_000;

/// Ordinary coaxial upstream numerology; RFoG requires separate admission.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Numerology {
    /// 2048 useful samples, 50 kHz spacing and 8 carriers per minislot.
    Khz50,
    /// 4096 useful samples, 25 kHz spacing and 16 carriers per minislot.
    Khz25,
}

impl Numerology {
    /// Returns the exact useful FFT size from the pinned upstream table.
    ///
    /// No timing, allocation or source state is changed.
    pub fn fft_len(self) -> usize {
        match self {
            Self::Khz50 => 2048,
            Self::Khz25 => 4096,
        }
    }
    /// Returns contiguous physical carriers per ordinary data minislot.
    ///
    /// This does not enable RFoG or REQ subslot layouts.
    pub fn carriers_per_minislot(self) -> usize {
        self.fft_len() / 256
    }
    /// Returns subcarrier spacing in integer hertz.
    ///
    /// The sample clock remains 102.4 MHz for both modes.
    pub fn spacing_hz(self) -> u32 {
        SAMPLE_RATE_HZ / self.fft_len() as u32
    }
}

/// Immutable admitted scheduled-data symbol/frame geometry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Waveform {
    numerology: Numerology,
    prefix: usize,
    rolloff: usize,
    frame_symbols: usize,
    bandwidth_hz: u32,
}

impl Waveform {
    /// Validates geometry before allocating a grid or consuming source.
    ///
    /// `prefix`/`rolloff` are exact sample counts; `frame_symbols` is K and
    /// `bandwidth_hz` is encompassed bandwidth. Returns owned accepted geometry.
    /// Rejects unsupported counts, RP>=CP and bandwidth-dependent frame bounds.
    pub fn new(
        numerology: Numerology,
        prefix: usize,
        rolloff: usize,
        frame_symbols: usize,
        bandwidth_hz: u32,
    ) -> PhyResult<Self> {
        let minimum = match numerology {
            Numerology::Khz50 => 10_000_000,
            Numerology::Khz25 => 6_400_000,
        };
        if !(minimum..=95_000_000).contains(&bandwidth_hz)
            || ![96, 128, 160, 192, 224, 256, 288, 320, 384, 512, 640].contains(&prefix)
            || ![0, 32, 64, 96, 128, 160, 192, 224].contains(&rolloff)
            || rolloff >= prefix
        {
            return Err(invalid("waveform"));
        }
        let maximum_2k = if bandwidth_hz >= 72_000_000 {
            18
        } else if bandwidth_hz >= 48_000_000 {
            24
        } else {
            36
        };
        let maximum = match numerology {
            Numerology::Khz50 => maximum_2k,
            Numerology::Khz25 => maximum_2k / 2,
        };
        if !(6..=maximum).contains(&frame_symbols) {
            return Err(invalid("waveform.frame_symbols"));
        }
        Ok(Self {
            numerology,
            prefix,
            rolloff,
            frame_symbols,
            bandwidth_hz,
        })
    }
    /// Returns the accepted upstream mode without mutation.
    ///
    /// The result determines FFT size, spacing and minislot width.
    pub fn numerology(&self) -> Numerology {
        self.numerology
    }
    /// Returns cyclic-prefix sample count C.
    ///
    /// Useful-window origins are C ticks after each prefix start.
    pub fn prefix(&self) -> usize {
        self.prefix
    }
    /// Returns roll-off overlap/tail count R.
    ///
    /// Zero selects exact rectangular cyclic extension.
    pub fn rolloff(&self) -> usize {
        self.rolloff
    }
    /// Returns useful OFDMA symbols per frame K.
    ///
    /// This is independent of how many frames a grant contains.
    pub fn frame_symbols(&self) -> usize {
        self.frame_symbols
    }
    /// Returns configured encompassed bandwidth in hertz.
    ///
    /// Exclusions and loading do not reduce this reported bandwidth.
    pub fn bandwidth_hz(&self) -> u32 {
        self.bandwidth_hz
    }
    /// Returns nominal symbol stride N+C in sample ticks.
    ///
    /// The final burst tail R is added once at terminal drain.
    pub fn stride(&self) -> usize {
        self.numerology.fft_len() + self.prefix
    }
}
