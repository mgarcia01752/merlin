/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! RX composition only; owns known/bootstrap configuration and receives samples alone.
use crate::{
    signal_processing::{
        receiver::{
            equalization::KnownReferenceEqualizer,
            extraction::{self, ExtractedSignals},
            timing::SymbolFramer,
        },
        waveform::{cyclic_prefix, fft::ForwardTransform, SampleBlock, WaveformGeometry},
    },
    transmission::layout::{CarrierLayout, CarrierLayoutSettings},
    types::IqPair,
    LabError,
};
/// Completed RX boundaries; no transmitter truth or source object is retained.
pub struct ReceivedSymbol {
    pub symbol_index: u64,
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
    ) -> Result<Self, LabError> {
        if reference.channel_response.len() != geometry.fft_samples() {
            return Err(crate::error::invalid(
                "RX response size does not match FFT geometry.",
            ));
        }
        Ok(Self {
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
    /// Receives a contiguous bounded sample block and recovers complete symbols only.
    /// # Arguments
    /// * `input` - Received time-domain I/Q, with absolute sample identity.
    /// # Returns
    /// Completed period/useful/bin/equalized/extracted results; partial symbols stay buffered.
    /// # Errors
    /// Rejects stream discontinuity or downstream numerical failure. Framing is advanced
    /// once complete periods are accepted; after numerical failure start a new RX session.
    pub fn receive(&mut self, input: &SampleBlock) -> Result<Vec<ReceivedSymbol>, LabError> {
        let mut output = Vec::new();
        for frame in self.framer.push(input)? {
            let useful = cyclic_prefix::remove(&self.geometry, frame.period.samples())?.to_vec();
            let bins = self.transform.apply(&useful)?;
            let equalized = self.equalizer.apply(&bins)?;
            let signals = extraction::extract(&self.layout, frame.symbol_index, &equalized)?;
            output.push(ReceivedSymbol {
                symbol_index: frame.symbol_index,
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
