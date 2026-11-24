/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Whole-bank §7.4.17 normalization over all non-excluded physical carriers.
use crate::{
    configuration::{UsConfiguration, Waveform},
    error::invalid,
    types::Iq,
    PhyError, PhyResult,
};
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use serde::Serialize;

/// Owned static corrective transmitter bank, distinct from injected H.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PreEqBank {
    waveform: Waveform,
    excluded: Vec<usize>,
    raw: Vec<Iq>,
    coefficients: Vec<Iq>,
    generation: u64,
    unity: bool,
}

impl PreEqBank {
    /// Constructs normative zero-excluded/unity-elsewhere defaults for `config`.
    ///
    /// Returns generation 1 without inspecting grant payload or channel truth.
    pub fn unity(config: &UsConfiguration) -> Self {
        let mut coefficients =
            vec![Iq { re: 1.0, im: 0.0 }; config.waveform().numerology().fft_len()];
        for &bin in config.excluded() {
            coefficients[bin] = Iq::default();
        }
        Self {
            waveform: config.waveform().clone(),
            excluded: config.excluded().to_vec(),
            raw: coefficients.clone(),
            coefficients,
            generation: 1,
            unity: true,
        }
    }
    /// Admits and normalizes a complete synthetic raw bank transactionally.
    ///
    /// `raw` is N physical coefficients with required excluded zeros; `generation`
    /// is nonzero. Returns preserved raw values and derived coefficients.
    /// Rejects wrong grids, nonfinite/whole-zero banks or nonrepresentable norms.
    pub fn normalized(config: &UsConfiguration, raw: Vec<Iq>, generation: u64) -> PhyResult<Self> {
        let n = config.waveform().numerology().fft_len();
        if generation == 0
            || raw.len() != n
            || raw.iter().any(|z| !z.is_finite())
            || config.excluded().iter().any(|&i| raw[i] != Iq::default())
        {
            return Err(invalid("pre_eq.bank"));
        }
        let scale = raw
            .iter()
            .map(|z| z.re.abs().max(z.im.abs()))
            .fold(0.0, f64::max);
        let domain = n - config.excluded().len();
        if domain == 0 || scale == 0.0 {
            return Err(invalid("pre_eq.norm"));
        }
        let scaled_energy = raw
            .iter()
            .map(|z| {
                Iq {
                    re: z.re / scale,
                    im: z.im / scale,
                }
                .power()
            })
            .sum::<f64>()
            / domain as f64;
        let norm = scaled_energy.sqrt();
        if !norm.is_finite() || norm == 0.0 {
            return Err(invalid("pre_eq.norm"));
        }
        let coefficients: Vec<_> = raw
            .iter()
            .map(|z| Iq {
                re: (z.re / scale) / norm,
                im: (z.im / scale) / norm,
            })
            .collect();
        if coefficients.iter().any(|z| !z.is_finite()) {
            return Err(invalid("pre_eq.normalized"));
        }
        let unity = coefficients.iter().enumerate().all(|(i, z)| {
            *z == if config.excluded().contains(&i) {
                Iq::default()
            } else {
                Iq { re: 1.0, im: 0.0 }
            }
        });
        Ok(Self {
            waveform: config.waveform().clone(),
            excluded: config.excluded().to_vec(),
            raw,
            coefficients,
            generation,
            unity,
        })
    }
    /// Borrows normalized full physical coefficients without exposing mutable state.
    ///
    /// Excluded carriers remain zero; currently unused non-excluded bins persist.
    pub fn coefficients(&self) -> &[Iq] {
        &self.coefficients
    }
    /// Returns the immutable coefficient generation actually applied.
    ///
    /// It does not identify a receiver estimate or an RF channel generation.
    pub fn generation(&self) -> u64 {
        self.generation
    }
    /// Tests exact unity on every non-excluded carrier without numerical tolerance.
    ///
    /// Returns the retained admission flag; this is not an RF channel assertion.
    pub fn is_unity(&self) -> bool {
        self.unity
    }
    /// Verifies the accepted reset geometry and mask without multiplying a grid.
    ///
    /// Returns unit; stale geometry or exclusion identity rejects explicitly.
    pub fn validate(&self, config: &UsConfiguration) -> PhyResult<()> {
        if self.waveform != *config.waveform() || self.excluded != config.excluded() {
            return Err(PhyError::Association {
                field: "pre_eq.grid",
            });
        }
        Ok(())
    }
    /// Applies this bank once to a complete physical `grid` for `config`.
    ///
    /// Returns owned finite corrected bins; exact unity copies preserve bits.
    /// Rejects stale geometry/masks, nonfinite input/results and excluded energy.
    pub fn apply(&self, config: &UsConfiguration, grid: &[Iq]) -> PhyResult<Vec<Iq>> {
        if self.waveform != *config.waveform() || self.excluded != config.excluded() {
            return Err(PhyError::Association {
                field: "pre_eq.grid",
            });
        }
        if grid.len() != self.coefficients.len()
            || grid.iter().any(|z| !z.is_finite())
            || self.excluded.iter().any(|&i| grid[i] != Iq::default())
        {
            return Err(invalid("pre_eq.input"));
        }
        if self.unity {
            return Ok(grid.to_vec());
        }
        let values: Vec<_> = grid
            .iter()
            .zip(&self.coefficients)
            .map(|(x, c)| *x * *c)
            .collect();
        if values.iter().any(|z| !z.is_finite()) {
            return Err(invalid("pre_eq.output"));
        }
        Ok(values)
    }
}

impl RetainedPayload for PreEqBank {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            waveform,
            excluded,
            raw,
            coefficients,
            generation,
            unity,
        } = self;
        sum([
            waveform.retained_children(),
            excluded.retained_children(),
            raw.retained_children(),
            coefficients.retained_children(),
            generation.retained_children(),
            unity.retained_children(),
        ])
    }
}

impl PreEqBank {
    /// Normalizes only owned report-snapshot capacities without changing values.
    ///
    /// # Returns
    /// Unit; exact boxed-slice conversion preserves order and every scientific value.
    pub(crate) fn compact_report_payload(&mut self) {
        let Self {
            waveform: _,
            excluded,
            raw,
            coefficients,
            generation: _,
            unity: _,
        } = self;
        *excluded = std::mem::take(excluded).into_boxed_slice().into_vec();
        *raw = std::mem::take(raw).into_boxed_slice().into_vec();
        *coefficients = std::mem::take(coefficients).into_boxed_slice().into_vec();
    }
}

#[cfg(test)]
mod retained_footprint_tests {
    use super::*;
    use merlin_api::retention_support::retained_footprint;
    use std::mem::size_of;

    #[test]
    fn raw_and_cached_normalized_spare_capacity_are_both_counted() {
        const SPARE_COEFFICIENTS: usize = 7;
        let document = crate::configuration::ConfigurationDocument::from_json(include_bytes!(
            "../../../../examples/cli/upstream/us-50khz.json"
        ))
        .unwrap();
        let (configuration, _) = document.prepare().unwrap();
        let mut bank = configuration.pre_eq;
        let before = retained_footprint(&bank).unwrap();
        let raw = bank.raw.capacity();
        let normalized = bank.coefficients.capacity();
        bank.raw.reserve_exact(raw + SPARE_COEFFICIENTS);
        bank.coefficients
            .reserve_exact(normalized + SPARE_COEFFICIENTS);
        let delta = (bank.raw.capacity() - raw + bank.coefficients.capacity() - normalized)
            * size_of::<Iq>();
        assert_eq!(
            retained_footprint(&bank).unwrap().payload_bytes - before.payload_bytes,
            delta
        );
    }
}
