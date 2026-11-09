/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! TX composition only: transform, prefix, window and overlap-add.
use crate::{
    error::invalid,
    signal_processing::waveform::{
        cyclic_prefix, ifft::InverseTransform, overlap_add::OverlapAdd, windowing::TransmitWindow,
        SampleBlock, WaveformGeometry,
    },
    transmission::frame::PhysicalFrequencySymbol,
    types::IqPair,
    SimulationError,
};
/// Completed separately inspectable TX stages for one bounded symbol operation.
pub struct TransmittedSymbol {
    pub symbol_index: u64,
    pub useful: Vec<IqPair>,
    pub prefixed: Vec<IqPair>,
    pub windowed: Vec<IqPair>,
    pub output: SampleBlock,
}
/// Fixed-session transmitter; owns its geometry, plans and overlap history.
pub struct WaveformTransmitter {
    geometry: WaveformGeometry,
    transform: InverseTransform,
    window: TransmitWindow,
    overlap: OverlapAdd,
    next_symbol: u64,
}
impl WaveformTransmitter {
    /// Starts TX at an explicit channel-symbol/sample identity.
    /// # Arguments
    /// * `geometry` - Validated configuration copied before the run.
    /// * `first_symbol` - First PLC-aligned output symbol index.
    /// * `first_sample` - Absolute first window sample tick.
    /// # Returns
    /// Independent TX state with no preceding tail; no data values are fabricated.
    pub fn new(geometry: &WaveformGeometry, first_symbol: u64, first_sample: u64) -> Self {
        Self {
            geometry: geometry.clone(),
            transform: InverseTransform::new(geometry.settings().subcarrier_spacing),
            window: TransmitWindow::new(geometry),
            overlap: OverlapAdd::new(geometry, first_sample),
            next_symbol: first_symbol,
        }
    }
    /// Converts one complete assembled physical symbol to a time-domain output block.
    /// # Arguments
    /// * `symbol` - Matching spacing/index and all N physical bins, including zeros/pilots/PLC.
    /// # Returns
    /// Completed stage buffers and one N+CP-sample output, retaining only overlap history.
    /// # Errors
    /// Rejects unavailable history (None), invalid shape/data or sequence/clock errors
    /// before advancing TX state; missing cells are never silently set to zero.
    pub fn transmit(
        &mut self,
        symbol: &PhysicalFrequencySymbol,
    ) -> Result<TransmittedSymbol, SimulationError> {
        if symbol.bins.len() != self.geometry.fft_samples()
            || symbol.symbol_index != self.next_symbol
            || symbol.spacing != self.geometry.settings().subcarrier_spacing
        {
            return Err(invalid(
                "TX physical symbol does not match frozen geometry/sequence.",
            ));
        }
        let next_symbol = self
            .next_symbol
            .checked_add(1)
            .ok_or_else(|| invalid("TX symbol index overflow."))?;
        let bins: Vec<_> = symbol
            .bins
            .iter()
            .copied()
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| {
                invalid("TX waveform requires complete cells; missing history is not zero.")
            })?;
        let useful = self.transform.apply(&bins)?;
        let prefixed = cyclic_prefix::insert(&self.geometry, &useful)?;
        let windowed = self.window.apply(&prefixed)?;
        let output = self.overlap.push(&windowed)?;
        self.next_symbol = next_symbol;
        Ok(TransmittedSymbol {
            symbol_index: symbol.symbol_index,
            useful,
            prefixed,
            windowed,
            output,
        })
    }
    /// Drains the final window tail and closes the TX stream.
    /// # Returns
    /// RP samples once; repeated calls produce an empty terminal block.
    /// # Errors
    /// Propagates checked sample-clock overflow without consuming the tail.
    pub fn finish(&mut self) -> Result<SampleBlock, SimulationError> {
        self.overlap.finish()
    }
}
