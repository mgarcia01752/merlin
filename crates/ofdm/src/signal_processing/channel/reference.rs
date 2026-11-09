/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Unity, zero-delay transfer from TX complex-envelope output to RX input.
use crate::signal_processing::waveform::SampleBlock;
/// Reference channel with no noise, filtering, frequency offset or propagation delay.
pub struct NoiselessChannel;
impl NoiselessChannel {
    /// Transfers a bounded sample block between explicit TX-output and RX-input planes.
    /// # Arguments
    /// * `transmitted` - Validated time samples at 204.8 Msamples/s.
    /// # Returns
    /// Owned identical samples and absolute indices; block splits do not change transfer.
    pub fn transfer(transmitted: &SampleBlock) -> SampleBlock {
        transmitted.clone()
    }
}
