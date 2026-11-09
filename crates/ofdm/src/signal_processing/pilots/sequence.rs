/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 section7.5.15.3/Figure84, PDF171; independent of data/NCP/PLC state.
use crate::{transmission::SubcarrierSpacing, types::SubcarrierIdx};
/// Binary pilot-sequence output; the generator only produces0or1.
pub type PilotBit = u8;
const REGISTER_MASK: u16 = (1 << 13) - 1;
// Register bit0 is rightmost/output in Figure84. Feedback enters the leftmost
// stage and the stages after the8th,11th and12th delays: bits12,4,1,0.
const FEEDBACK_MASK: u16 = (1 << 12) | (1 << 4) | (1 << 1) | 1;
/// Immutable per-FFT pilot bits in physical index order, including protected bins.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotSequence {
    spacing: SubcarrierSpacing,
    bits: Vec<PilotBit>,
}
impl PilotSequence {
    /// Generates one complete physical FFT's pilot signs from the all-ones seed.
    /// # Arguments
    /// * `spacing` -25kHz produces8192 bits;50kHz produces4096 bits.
    /// # Returns
    /// Owned reusable sequence. Every FFT bin clocks once; role masks are not used.
    /// Reuse this sequence each symbol; pilot positions change, this sequence does not.
    pub fn new(spacing: SubcarrierSpacing) -> Self {
        let mut register = REGISTER_MASK;
        let bits = (0..spacing.fft_samples())
            .map(|_| {
                let output = (register & 1) as PilotBit;
                register >>= 1;
                if output != 0 {
                    register ^= FEEDBACK_MASK;
                }
                output
            })
            .collect();
        Self { spacing, bits }
    }
    /// Borrows binary outputs starting at physical FFT bin0.
    /// # Returns
    /// Read-only4096or8192-element slice tied to the sequence lifetime.
    pub fn bits(&self) -> &[PilotBit] {
        &self.bits
    }
    /// Returns the numerology associated with this sequence length.
    /// # Returns
    /// The fixed25or50kHz spacing.
    pub fn spacing(&self) -> SubcarrierSpacing {
        self.spacing
    }
    /// Looks up a physical bin without reindexing through data/pilot-only positions.
    /// # Arguments
    /// * `bin` - Physical FFT index, including bins excluded from interleaving.
    /// # Returns
    /// Binary output or None for an index outside this FFT.
    pub fn bit_at(&self, bin: SubcarrierIdx) -> Option<PilotBit> {
        self.bits.get(usize::from(bin)).copied()
    }
}
