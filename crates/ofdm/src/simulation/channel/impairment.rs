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
    synthesis_response: Option<Vec<crate::types::IqPair>>,
    awgn: SampleAwgn,
    rng: ChaCha8Rng,
    filter: Option<crate::signal_processing::channel::filter::StreamingChannelFilter>,
    pub filter_diagnostics: Option<crate::signal_processing::channel::filter::FilterDiagnostics>,
    pub filter_tail_samples: usize,
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
        filter_settings: &crate::signal_processing::channel::filter::ChannelFilterSettings,
    ) -> Result<Self, SimulationError> {
        let prepared = crate::signal_processing::channel::composition::prepare_channel(
            geometry.settings(),
            layout,
            settings,
            variance,
            filter_settings,
        )?;
        let plan = prepared.plan;
        let filter_diagnostics = prepared
            .filter
            .as_ref()
            .map(|plan| plan.diagnostics().clone());
        let filter = prepared
            .filter
            .as_ref()
            .map(|plan| {
                crate::signal_processing::channel::filter::StreamingChannelFilter::new(plan, 0)
            })
            .transpose()?;
        let sample_power = match settings.noise {
            NoiseModel::Awgn { sample_power } => sample_power,
            _ => 0.0,
        } + if filter.is_some() {
            plan.receiver_noise_power
        } else {
            0.0
        };
        let synthesis = if settings.power != Default::default()
            || matches!(settings.noise, NoiseModel::Rxmer { .. })
            || settings.response.is_some()
            || settings.additional_loss
                != crate::signal_processing::channel::additional_loss::AdditionalLoss::Off
        {
            Some(WaveformTransmitter::new(geometry, 0, 0))
        } else {
            None
        };
        Ok(Self {
            synthesis_response: prepared.synthesis_response,
            filter,
            filter_diagnostics,
            filter_tail_samples: 0,
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
        let source = if let Some(synthesis) = &mut self.synthesis {
            let mut altered = physical.clone();
            let plan = if self.filter.is_some() {
                Some(ImpairmentPlan {
                    response: self
                        .synthesis_response
                        .as_ref()
                        .expect("continuous filter synthesis plan")
                        .clone(),
                    receiver_noise_power: 0.0,
                    residual_power: self.plan.residual_power.clone(),
                    llr_variance: self.plan.llr_variance.clone(),
                })
            } else {
                None
            };
            altered.bins = plan.as_ref().unwrap_or(&self.plan).apply_bins(
                &physical.bins,
                &physical.roles,
                &mut self.rng,
            )?;
            synthesis.transmit(&altered)?.output
        } else {
            original.clone()
        };
        if let Some(filter) = &mut self.filter {
            self.awgn.transfer(&filter.transfer(&source)?)
        } else if self.synthesis.is_some() {
            Ok(source)
        } else {
            self.awgn.transfer(&source)
        }
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
        let source = if let Some(synthesis) = &mut self.synthesis {
            synthesis.finish()?
        } else {
            original.clone()
        };
        if let Some(filter) = &mut self.filter {
            let filtered = filter.transfer(&source)?;
            let tail = filter.finish()?;
            self.filter_tail_samples = tail.samples().len();
            let mut samples = filtered.samples().to_vec();
            samples.extend_from_slice(tail.samples());
            self.awgn
                .transfer(&SampleBlock::new(filtered.start_index(), samples)?)
        } else if self.synthesis.is_some() {
            Ok(source)
        } else {
            self.awgn.transfer(&source)
        }
    }
}
