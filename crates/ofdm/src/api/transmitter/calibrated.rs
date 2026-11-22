/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Electrical units for actual idle TX PSD; no channel simulation or gain is applied.
use super::*;
use crate::{signal_processing::channel::spectrum, transmission::layout::CarrierLayout};

impl TransmitterPreview {
    /// Calibrates actual retained idle TX PSD using an explicit accepted electrical reference.
    /// # Arguments
    /// * `channel` - Immutable settings supplying calibration only; gain/noise are not applied.
    /// * `start`, `count` - Original FFT-bin viewport; zero count selects the remaining span.
    /// # Returns
    /// Owned unit-labelled PSD and full occupied-band power before the amplifier.
    /// No transmitter/channel state advances, and zoom does not change integrated power.
    /// # Errors
    /// Preserves malformed original geometry, viewport, finite density and calibration failures.
    pub fn calibrated_spectrum(
        &self,
        channel: &ChannelConfiguration,
        start: usize,
        count: usize,
    ) -> ApiResult<ChannelSpectrum> {
        let layout = CarrierLayout::new(&self.inner.settings, &self.inner.layout)?;
        Ok(spectrum::view(
            &self.inner.spectrum,
            &layout,
            &channel.settings().power,
            "tx_post_overlap_before_channel",
            start,
            count,
        )?)
    }
}
