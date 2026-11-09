/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Stateful TX symbol overlap; only the RP tail is retained between calls.
use super::{samples::validate_samples, SampleBlock, WaveformGeometry};
use crate::{error::invalid, types::IqPair, LabError};
/// Fixed-session TX overlap-add with explicit stream start, terminal drain and reset.
pub struct OverlapAdd {
    geometry: WaveformGeometry,
    tail: Vec<IqPair>,
    next_sample: u64,
    finished: bool,
}
impl OverlapAdd {
    /// Starts a stream without fabricating a preceding symbol's tail.
    /// # Arguments
    /// * `geometry` - Validated per-run window/stride snapshot.
    /// * `first_sample` - Absolute tick for the first emitted block.
    /// # Returns
    /// Empty overlap history; startup taper remains explicit in the first prefix.
    pub fn new(geometry: &WaveformGeometry, first_sample: u64) -> Self {
        Self {
            geometry: geometry.clone(),
            tail: Vec::new(),
            next_sample: first_sample,
            finished: false,
        }
    }
    /// Overlaps one extended window with prior history and advances one symbol period.
    /// # Arguments
    /// * `windowed` - Exactly N+CP+RP finite samples, already tapered.
    /// # Returns
    /// One N+CP-sample block; retains only the final RP values for the next symbol.
    /// # Errors
    /// Rejects invalid size/numbers, overflow or use after finish without changing state.
    pub fn push(&mut self, windowed: &[IqPair]) -> Result<SampleBlock, LabError> {
        validate_samples(windowed, self.geometry.extended_samples())?;
        if self.finished {
            return Err(invalid(
                "Overlap-add stream is finished; reset before reuse.",
            ));
        }
        let stride = self.geometry.stride_samples();
        let mut samples = windowed[..stride].to_vec();
        for (sample, previous) in samples.iter_mut().zip(&self.tail) {
            sample[0] += previous[0];
            sample[1] += previous[1];
        }
        let output = SampleBlock::new(self.next_sample, samples)?;
        self.tail = windowed[stride..].to_vec();
        self.next_sample = output.end_index();
        Ok(output)
    }
    /// Emits the final RP tail once and closes the stream.
    /// # Returns
    /// A short drain block; repeated calls return an empty block at the same end tick.
    /// # Errors
    /// Rejects sample-index overflow without consuming the retained tail.
    pub fn finish(&mut self) -> Result<SampleBlock, LabError> {
        let output = SampleBlock::new(self.next_sample, self.tail.clone())?;
        self.tail.clear();
        self.next_sample = output.end_index();
        self.finished = true;
        Ok(output)
    }
    /// Explicitly starts a new session with no overlap history.
    /// # Arguments
    /// * `first_sample` - New absolute first sample tick.
    /// # Returns
    /// Unit; subsequent input must begin at the new run's first symbol.
    pub fn reset(&mut self, first_sample: u64) {
        self.tail.clear();
        self.next_sample = first_sample;
        self.finished = false;
    }
}
