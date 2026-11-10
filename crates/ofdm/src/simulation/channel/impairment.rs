/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Composition of channel response, Gaussian bin residuals and sample-domain AWGN.
use crate::{
    signal_processing::{
        channel::{
            awgn::SampleAwgn,
            impairments::{ChannelImpairments, ImpairmentPlan, NoiseModel},
        },
        waveform::{SampleBlock, WaveformGeometry},
    },
    simulation::waveform::WaveformTransmitter,
    transmission::{frame::PhysicalFrequencySymbol, layout::CarrierLayout},
    SimulationError,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub(super) struct ImpairedChannel {
    pub plan: ImpairmentPlan,
    synthesis: Option<WaveformTransmitter>,
    awgn: SampleAwgn,
    rng: ChaCha8Rng,
}
impl ImpairedChannel {
    /// Freezes independent channel synthesis and noise state.
    /// # Arguments
    /// * `geometry` - Validated waveform numerology.
    /// * `layout` - Explicit protected physical roles.
    /// * `settings` - Exclusive noise source and optional supplied H.
    /// * `variance` - Noiseless LLR reference scale.
    /// # Returns
    /// Fresh bounded channel owners at sample/symbol origin0.
    /// # Errors
    /// Rejects invalid measured coverage, response or noise settings.
    pub(super) fn new(
        geometry: &WaveformGeometry,
        layout: &CarrierLayout,
        settings: &ChannelImpairments,
        variance: f64,
    ) -> Result<Self, SimulationError> {
        let plan = settings.plan(geometry.settings(), layout, variance)?;
        let sample_power = match settings.noise {
            NoiseModel::Awgn { sample_power } => sample_power,
            _ => 0.0,
        };
        let synthesis = if matches!(settings.noise, NoiseModel::Rxmer { .. })
            || settings.response.is_some()
            || settings.additional_loss
                != crate::signal_processing::channel::additional_loss::AdditionalLoss::Off
        {
            Some(WaveformTransmitter::new(geometry, 0, 0))
        } else {
            None
        };
        Ok(Self {
            plan,
            synthesis,
            awgn: SampleAwgn::new(sample_power, settings.seed, 0)?,
            rng: ChaCha8Rng::seed_from_u64(settings.seed),
        })
    }
    /// Replaces the validated frequency-domain plan while preserving noise and overlap history.
    /// # Arguments
    /// * `plan` - Same-grid next RxMER/channel-response epoch.
    /// # Returns
    /// Unit after replacing coefficients; no generator is reseeded.
    pub(super) fn transition(&mut self, plan: ImpairmentPlan) {
        self.plan = plan;
    }
    /// Produces one physical channel-output sample block.
    /// # Arguments
    /// * `physical` - Complete physical symbol, used only for channel synthesis.
    /// * `original` - Original TX sample block for sample-AWGN or exact passthrough.
    /// # Returns
    /// Owned channel output; receiver receives samples only, never this TX object.
    /// # Errors
    /// Rejects incomplete/nonnumeric bins or stream/transform failures; start a new
    /// channel session after synthesis failure because state may have advanced.
    pub(super) fn transfer(
        &mut self,
        physical: &PhysicalFrequencySymbol,
        original: &SampleBlock,
    ) -> Result<SampleBlock, SimulationError> {
        if let Some(synthesis) = &mut self.synthesis {
            let mut altered = physical.clone();
            altered.bins = self
                .plan
                .apply_bins(&physical.bins, &physical.roles, &mut self.rng)?;
            return Ok(synthesis.transmit(&altered)?.output);
        }
        self.awgn.transfer(original)
    }
    /// Drains channel waveform history without inventing another symbol.
    /// # Arguments
    /// * `original` - Original TX overlap tail with its absolute indices.
    /// # Returns
    /// Matching independently synthesized or sample-noised tail.
    /// # Errors
    /// Propagates sample/clock/synthesis failures; no retry transaction is implied.
    pub(super) fn finish(
        &mut self,
        original: &SampleBlock,
    ) -> Result<SampleBlock, SimulationError> {
        if let Some(synthesis) = &mut self.synthesis {
            synthesis.finish()
        } else {
            self.awgn.transfer(original)
        }
    }
}
