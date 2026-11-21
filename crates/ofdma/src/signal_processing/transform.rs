/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: typed finite/error adapter to the shared merlin-dsp transform.
 * Adapted from MERLIN crates/ofdm/src/signal_processing/waveform/transform.rs
 * at 096d5a476194d4f2e016863d723b225257bb6037.
 * Modifications: upstream 2K/4K geometry, Iq values and local typed validation.
 */
//! Typed upstream adapter to the common centered unitary transform.
use crate::{configuration::Numerology, error::invalid, types::Iq, PhyError, PhyResult};
use merlin_dsp::transform::{TransformPlan, TransformSize};
use merlin_dsp::DspError;

/// Immutable shared numerical plan with bounded, owned Iq conversion buffers.
pub struct Transform {
    plan: TransformPlan,
    length: usize,
}
impl Transform {
    /// Prepares one shared plan for accepted upstream geometry.
    ///
    /// # Arguments
    /// * `mode` - 2K/50 kHz or 4K/25 kHz; the sample clock stays PHY-owned.
    /// # Returns
    /// Immutable reusable plan; each call owns its conversion/output storage.
    pub fn new(mode: Numerology) -> Self {
        let size = match mode {
            Numerology::Khz50 => TransformSize::Samples2048,
            Numerology::Khz25 => TransformSize::Samples4096,
        };
        Self {
            plan: TransformPlan::new(size),
            length: size.samples(),
        }
    }
    /// Synthesizes physical ascending-frequency bins with a positive exponent.
    ///
    /// # Arguments
    /// * `grid` - Exactly N finite typed complex bins; no prefix or taper.
    /// # Returns
    /// Owned useful samples with 1/sqrt(N) scaling.
    /// # Errors
    /// Rejects wrong length/nonfinite input before allocating or nonfinite output.
    pub fn inverse(&self, grid: &[Iq]) -> PhyResult<Vec<Iq>> {
        let values = self.convert(grid)?;
        self.plan
            .inverse(&values)
            .map(Self::typed)
            .map_err(Self::error)
    }
    /// Analyzes independently selected useful receive samples.
    ///
    /// # Arguments
    /// * `samples` - Exactly N finite typed complex samples, without TX truth.
    /// # Returns
    /// Owned centered physical bins with negative exponent and 1/sqrt(N) scaling.
    /// # Errors
    /// Rejects wrong length/nonfinite input before allocating or nonfinite output.
    pub fn forward(&self, samples: &[Iq]) -> PhyResult<Vec<Iq>> {
        let values = self.convert(samples)?;
        self.plan
            .forward(&values)
            .map(Self::typed)
            .map_err(Self::error)
    }
    fn convert(&self, values: &[Iq]) -> PhyResult<Vec<[f64; 2]>> {
        if values.len() != self.length || values.iter().any(|value| !value.is_finite()) {
            return Err(invalid("transform.input"));
        }
        Ok(values.iter().map(|value| [value.re, value.im]).collect())
    }
    fn typed(values: Vec<[f64; 2]>) -> Vec<Iq> {
        values.into_iter().map(|[re, im]| Iq { re, im }).collect()
    }
    fn error(error: DspError) -> PhyError {
        match error {
            DspError::InvalidSamples => invalid("transform.input"),
            DspError::NonfiniteOutput => invalid("transform.output"),
        }
    }
}
