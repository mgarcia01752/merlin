/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! RX composition only; owns known/bootstrap configuration and receives samples alone.
use crate::{
    signal_processing::{
        receiver::{
            channel_estimation::{ChannelEstimate, PilotChannelEstimator},
            equalization::KnownReferenceEqualizer,
            extraction::{self, ExtractedSignals},
            timing::SymbolFramer,
        },
        waveform::{cyclic_prefix, fft::ForwardTransform, SampleBlock, WaveformGeometry},
    },
    transmission::layout::{CarrierLayout, CarrierLayoutSettings},
    types::IqPair,
    SimulationError,
};
/// Completed RX boundaries; no transmitter truth or source object is retained.
pub struct ReceivedSymbol {
    pub symbol_index: u64,
    pub channel_estimate: Option<ChannelEstimate>,
    /// Applied inverse-gain power for raw-noise calibration.
    pub inverse_gain_power: Vec<f64>,
    pub period: SampleBlock,
    pub useful: Vec<IqPair>,
    pub bins: Vec<IqPair>,
    pub equalized: Vec<IqPair>,
    pub signals: ExtractedSignals,
}
/// Frozen known-reference RX with independent timing, transform and extraction owners.
pub struct WaveformReceiver {
    geometry: WaveformGeometry,
    layout: CarrierLayout,
    framer: SymbolFramer,
    transform: ForwardTransform,
    equalizer: KnownReferenceEqualizer,
    estimator: Option<PilotChannelEstimator>,
    reference_rms: f64,
}
/// Explicit reference/bootstrap assumptions, not discovered synchronization.
pub struct ReceiveReference {
    pub first_input_sample: u64,
    pub first_symbol_sample: u64,
    pub first_symbol_index: u64,
    pub reference_rms: f64,
    pub channel_response: Vec<IqPair>,
}
impl WaveformReceiver {
    /// Creates RX-owned state from supplied reference/bootstrap settings.
    /// # Arguments
    /// * `geometry` - RX settings; after PLC acquisition these may come from decoded OCD.
    /// * `layout` - Explicit copied bootstrap/received carrier assignments, not a TX instance.
    /// * `reference` - Known timing/channel response and amplitude reference, not payload truth.
    /// # Returns
    /// Independent RX modules. No blind acquisition or estimated-channel claim is made.
    /// # Errors
    /// Rejects physical layout, response size/numbers or inconsistent timing.
    pub fn new(
        geometry: &WaveformGeometry,
        layout: &CarrierLayoutSettings,
        reference: ReceiveReference,
    ) -> Result<Self, SimulationError> {
        if reference.channel_response.len() != geometry.fft_samples() {
            return Err(crate::error::invalid(
                "RX response size does not match FFT geometry.",
            ));
        }
        Ok(Self {
            estimator: None,
            reference_rms: reference.reference_rms,
            geometry: geometry.clone(),
            layout: CarrierLayout::new(geometry.settings(), layout)?,
            framer: SymbolFramer::new(
                geometry,
                reference.first_input_sample,
                reference.first_symbol_sample,
                reference.first_symbol_index,
            )?,
            transform: ForwardTransform::new(geometry.settings().subcarrier_spacing),
            equalizer: KnownReferenceEqualizer::new(
                &reference.channel_response,
                reference.reference_rms,
            )?,
        })
    }
    /// Creates a receiver whose correction is estimated solely from received pilots.
    /// # Arguments
    /// * `geometry` - Known bootstrap numerology; no channel coefficients.
    /// * `layout` - Independently supplied receiver carrier/PLC map.
    /// * `first_input_sample` - Incoming stream sample origin.
    /// * `first_symbol_sample` - Known initial symbol boundary.
    /// * `first_symbol_index` - Known PLC phase expressed as absolute symbol index.
    /// * `reference_rms` - Positive nonpilot RMS reference.
    /// # Returns
    /// Empty pilot-estimation history and independent receiver stages.
    /// # Errors
    /// Rejects invalid geometry, timing or RMS.
    pub fn from_pilots(
        geometry: &WaveformGeometry,
        layout: &CarrierLayoutSettings,
        first_input_sample: u64,
        first_symbol_sample: u64,
        first_symbol_index: u64,
        reference_rms: f64,
    ) -> Result<Self, SimulationError> {
        let mut receiver = Self::new(
            geometry,
            layout,
            ReceiveReference {
                first_input_sample,
                first_symbol_sample,
                first_symbol_index,
                reference_rms,
                channel_response: vec![[1.0, 0.0]; geometry.fft_samples()],
            },
        )?;
        receiver.estimator = Some(PilotChannelEstimator::new(
            CarrierLayout::new(geometry.settings(), layout)?,
            reference_rms,
        )?);
        Ok(receiver)
    }
    /// Acquires sample timing from a bounded received prefix window, then receives it.
    /// # Arguments
    /// * `geometry` - Known numerology, without a transmitted symbol boundary.
    /// * `layout` - Independent bootstrap carrier assignments.
    /// * `input` - Initial contiguous received samples covering the search window.
    /// * `first_symbol_index` - Known PLC phase; this operation does not acquire PLC phase.
    /// * `max_offset_samples` - Inclusive CP search extent, at most one period.
    /// * `minimum_correlation` - Required normalized squared CP correlation in (0,1].
    /// # Returns
    /// Live unit-RMS pilot receiver, measured timing and completed initial symbols.
    /// Continue with the next contiguous sample block; initial input is already consumed.
    /// # Errors
    /// Rejects ambiguous/weak timing, invalid geometry, or initial receive failure.
    pub fn acquire_from_pilots(
        geometry: &WaveformGeometry,
        layout: &CarrierLayoutSettings,
        input: &SampleBlock,
        first_symbol_index: u64,
        max_offset_samples: usize,
        minimum_correlation: f64,
    ) -> Result<
        (
            Self,
            crate::signal_processing::receiver::channel_estimation::timing::TimingEstimate,
            Vec<ReceivedSymbol>,
        ),
        SimulationError,
    > {
        let timing=crate::signal_processing::receiver::channel_estimation::timing::estimate_symbol_boundary(geometry,input,max_offset_samples,minimum_correlation)?;
        let mut receiver = Self::from_pilots(
            geometry,
            layout,
            input.start_index(),
            timing.first_symbol_sample,
            first_symbol_index,
            1.0,
        )?;
        let symbols = receiver.receive(input)?;
        Ok((receiver, timing, symbols))
    }
    /// Updates the explicitly supplied reference mode without changing sample framing.
    /// # Arguments
    /// * `response` - Same-grid finite nonzero gains for a reference experiment.
    /// # Returns
    /// Unit after atomically replacing reference coefficients.
    /// # Errors
    /// Rejects invalid gains or use in pilot-estimated mode, where H must not enter RX.
    pub fn set_reference_response(&mut self, response: &[IqPair]) -> Result<(), SimulationError> {
        if self.estimator.is_some() {
            return Err(crate::error::invalid(
                "Supplied H cannot update a pilot-estimated receiver.",
            ));
        }
        self.equalizer = KnownReferenceEqualizer::new(response, self.reference_rms)?;
        Ok(())
    }
    /// Receives a contiguous bounded sample block and recovers complete symbols only.
    /// # Arguments
    /// * `input` - Received time-domain I/Q, with absolute sample identity.
    /// # Returns
    /// Completed period/useful/bin/equalized/extracted results; partial symbols stay buffered.
    /// # Errors
    /// Rejects stream discontinuity or downstream numerical failure. Framing is advanced
    /// once complete periods are accepted; after numerical failure start a new RX session.
    pub fn receive(&mut self, input: &SampleBlock) -> Result<Vec<ReceivedSymbol>, SimulationError> {
        let mut output = Vec::new();
        for frame in self.framer.push(input)? {
            let useful = cyclic_prefix::remove(&self.geometry, frame.period.samples())?.to_vec();
            let bins = self.transform.apply(&useful)?;
            let channel_estimate = self
                .estimator
                .as_mut()
                .map(|estimator| estimator.estimate(frame.symbol_index, &bins))
                .transpose()?;
            let equalized = if let Some(estimate) = &channel_estimate {
                KnownReferenceEqualizer::new(&estimate.response, self.reference_rms)?
                    .apply(&bins)?
            } else {
                self.equalizer.apply(&bins)?
            };
            let inverse_gain_power = if let Some(estimate) = &channel_estimate {
                estimate
                    .response
                    .iter()
                    .map(|gain| {
                        1.0 / ((gain[0] * gain[0] + gain[1] * gain[1])
                            * self.reference_rms
                            * self.reference_rms)
                    })
                    .collect()
            } else {
                self.equalizer.inverse_gain_power()
            };
            let signals = extraction::extract(&self.layout, frame.symbol_index, &equalized)?;
            output.push(ReceivedSymbol {
                symbol_index: frame.symbol_index,
                channel_estimate,
                inverse_gain_power,
                period: frame.period,
                useful,
                bins,
                equalized,
                signals,
            });
        }
        Ok(output)
    }
    /// Reports a truncated final symbol or the explicit TX window drain.
    /// # Returns
    /// Pending sample count, never counted as a recovered symbol or payload.
    pub fn pending_samples(&self) -> usize {
        self.framer.pending_samples()
    }
}
