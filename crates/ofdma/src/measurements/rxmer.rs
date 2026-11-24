/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Original synthetic dB arrays and pure power-domain playback, never noise truth to RX.
use super::SyntheticGrid;
use crate::{
    configuration::UsConfiguration,
    error::{invalid, limit},
    resources::ResourcePlan,
    PhyResult,
};
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use serde::Serialize;
use std::collections::BTreeSet;

/// A synthetic equalized unit-BPSK-reference target, explicitly not a capture.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SyntheticRxmer {
    id: u64,
    grid: SyntheticGrid,
    mer_db: Vec<Option<f64>>,
    powers: Vec<Option<f64>>,
}
impl SyntheticRxmer {
    /// Retains exact N-bin optional dB readings and explicit nonzero source `id`.
    ///
    /// Returns owned synthetic input; rejects nonfinite or nonrepresentable power.
    /// Nulls remain null; required coverage is checked when a playback is admitted.
    pub fn new(config: &UsConfiguration, id: u64, mer_db: Vec<Option<f64>>) -> PhyResult<Self> {
        if id == 0 || mer_db.len() != config.waveform().numerology().fft_len() {
            return Err(invalid("rxmer.readings"));
        }
        let powers = mer_db
            .iter()
            .map(|value| match value {
                None => Ok(None),
                Some(db) => {
                    let power = 10_f64.powf(-db / 10.0);
                    if !db.is_finite() || !power.is_finite() || power <= 0.0 {
                        return Err(invalid("rxmer.power"));
                    }
                    Ok(Some(power))
                }
            })
            .collect::<PhyResult<Vec<_>>>()?;
        Ok(Self {
            id,
            grid: SyntheticGrid::new(config),
            mer_db,
            powers,
        })
    }
}

/// Explicit input combination; offsets are physical symbols, independent of epochs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum RxmerPolicy {
    /// Exactly one source, held through the burst.
    Single,
    /// Equal-source arithmetic average of error powers.
    Average,
    /// At least two ordered sources and strictly increasing offsets starting zero.
    Hold {
        /// Explicit synthetic activation symbols.
        offsets: Vec<usize>,
    },
    /// Linear power interpolation between offsets, then hold final source.
    Interpolate {
        /// Explicit synthetic activation symbols.
        offsets: Vec<usize>,
    },
}

/// Pure query result preserving the contributors, weights and missing values.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PowerSnapshot {
    /// Exact full physical normalized error-power grid.
    pub powers: Vec<Option<f64>>,
    /// Selected synthetic source IDs and equal/temporal power weights.
    pub contributors: Vec<(u64, f64)>,
}

/// Immutable admitted source selection, shared by preview and residual synthesis.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RxmerPlayback {
    grid: SyntheticGrid,
    sources: Vec<SyntheticRxmer>,
    policy: RxmerPolicy,
    symbols: usize,
}
impl RxmerPlayback {
    /// Admits `sources` and explicit `policy` for the accepted finite grant.
    ///
    /// Returns a pure playback; rejects duplicates, incompatible grids, missing
    /// active values, counts above 256, bad offsets and nonrepresentable averages.
    pub fn new(
        config: &UsConfiguration,
        sources: Vec<SyntheticRxmer>,
        policy: RxmerPolicy,
    ) -> PhyResult<Self> {
        if sources.is_empty() || sources.len() > 256 {
            return Err(limit("rxmer.sources"));
        }
        let symbols = config.grant().frames * config.waveform().frame_symbols();
        let mut ids = BTreeSet::new();
        let resources = ResourcePlan::new(config)?;
        for source in &sources {
            source.grid.validate(config)?;
            if !ids.insert(source.id)
                || resources
                    .visits
                    .iter()
                    .any(|cell| source.powers[cell.carrier].is_none())
            {
                return Err(invalid("rxmer.source_coverage"));
            }
        }
        match &policy {
            RxmerPolicy::Single if sources.len() != 1 => return Err(invalid("rxmer.single")),
            RxmerPolicy::Hold { offsets } | RxmerPolicy::Interpolate { offsets } => {
                if sources.len() < 2
                    || offsets.len() != sources.len()
                    || offsets[0] != 0
                    || offsets.windows(2).any(|pair| pair[0] >= pair[1])
                    || offsets.last().is_some_and(|last| *last >= symbols)
                {
                    return Err(invalid("rxmer.offsets"));
                }
            }
            _ => {}
        }
        let result = Self {
            grid: SyntheticGrid::new(config),
            sources,
            policy,
            symbols,
        };
        // Average all inputs once during admission to catch overflow/underflow.
        result.at(0)?;
        Ok(result)
    }
    /// Verifies grid, grant duration and model-specific finite power admission.
    ///
    /// Returns unit; powers outside (0, 1e6] remain inspectable but cannot run.
    pub fn validate_model(&self, config: &UsConfiguration) -> PhyResult<()> {
        self.grid.validate(config)?;
        if self.symbols != config.grant().frames * config.waveform().frame_symbols() {
            return Err(invalid("rxmer.duration"));
        }
        let resources = ResourcePlan::new(config)?;
        for source in &self.sources {
            if resources
                .visits
                .iter()
                .any(|cell| source.powers[cell.carrier].is_none_or(|rho| rho > 1_000_000.0))
            {
                return Err(limit("rxmer.model_power"));
            }
        }
        Ok(())
    }
    /// Queries a bounded physical `symbol` without consuming randomness or state.
    ///
    /// Returns full-grid powers and source weights; rejects out-of-run indices.
    pub fn at(&self, symbol: usize) -> PhyResult<PowerSnapshot> {
        if symbol >= self.symbols {
            return Err(invalid("rxmer.symbol"));
        }
        let weights: Vec<(usize, f64)> = match &self.policy {
            RxmerPolicy::Single => vec![(0, 1.0)],
            RxmerPolicy::Average => (0..self.sources.len())
                .map(|i| (i, 1.0 / self.sources.len() as f64))
                .collect(),
            RxmerPolicy::Hold { offsets } | RxmerPolicy::Interpolate { offsets } => {
                let index = offsets.partition_point(|offset| *offset <= symbol) - 1;
                if matches!(self.policy, RxmerPolicy::Interpolate { .. })
                    && index + 1 < offsets.len()
                {
                    let right = (symbol - offsets[index]) as f64
                        / (offsets[index + 1] - offsets[index]) as f64;
                    vec![(index, 1.0 - right), (index + 1, right)]
                } else {
                    vec![(index, 1.0)]
                }
            }
        };
        let powers = (0..self.sources[0].powers.len())
            .map(|bin| {
                let mut sum = 0.0;
                for &(index, weight) in &weights {
                    if weight == 0.0 {
                        continue;
                    }
                    let Some(value) = self.sources[index].powers[bin] else {
                        return Ok(None);
                    };
                    sum += weight * value;
                }
                if !sum.is_finite() || sum <= 0.0 {
                    return Err(invalid("rxmer.combined_power"));
                }
                Ok(Some(sum))
            })
            .collect::<PhyResult<Vec<_>>>()?;
        Ok(PowerSnapshot {
            powers,
            contributors: weights
                .iter()
                .filter(|(_, w)| *w != 0.0)
                .map(|(i, w)| (self.sources[*i].id, *w))
                .collect(),
        })
    }
}

impl RetainedPayload for SyntheticRxmer {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            id,
            grid,
            mer_db,
            powers,
        } = self;
        sum([
            id.retained_children(),
            grid.retained_children(),
            mer_db.retained_children(),
            powers.retained_children(),
        ])
    }
}

impl RetainedPayload for RxmerPlayback {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            grid,
            sources,
            policy,
            symbols,
        } = self;
        sum([
            grid.retained_children(),
            sources.retained_children(),
            policy.retained_children(),
            symbols.retained_children(),
        ])
    }
}

impl RetainedPayload for RxmerPolicy {
    /// Visits retained temporal offsets without evaluating playback powers.
    ///
    /// # Returns
    /// Checked offset capacity, or zero for policies with no owned children.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        match self {
            Self::Single | Self::Average => Some(RetainedFootprint::default()),
            Self::Hold { offsets } | Self::Interpolate { offsets } => offsets.retained_children(),
        }
    }
}

impl RxmerPlayback {
    /// Normalizes only owned report-snapshot capacities without changing values.
    ///
    /// # Returns
    /// Unit; exact boxed-slice conversion preserves order and every scientific value.
    pub(crate) fn compact_report_payload(&mut self) {
        let Self {
            grid,
            sources,
            policy,
            symbols: _,
        } = self;
        grid.compact_report_payload();
        for source in sources.iter_mut() {
            let SyntheticRxmer {
                id: _,
                grid,
                mer_db,
                powers,
            } = source;
            grid.compact_report_payload();
            *mer_db = std::mem::take(mer_db).into_boxed_slice().into_vec();
            *powers = std::mem::take(powers).into_boxed_slice().into_vec();
        }
        match policy {
            RxmerPolicy::Single | RxmerPolicy::Average => {}
            RxmerPolicy::Hold { offsets } | RxmerPolicy::Interpolate { offsets } => {
                *offsets = std::mem::take(offsets).into_boxed_slice().into_vec();
            }
        }
        *sources = std::mem::take(sources).into_boxed_slice().into_vec();
    }
}

#[cfg(test)]
mod retained_footprint_tests {
    use merlin_api::retention_support::retained_footprint;
    use std::mem::size_of;

    #[test]
    fn cached_power_capacity_is_counted_separately_from_original_readings() {
        const SPARE_CACHED_VALUES: usize = 7;
        let document = crate::configuration::ConfigurationDocument::from_json(include_bytes!(
            "../../../../examples/cli/upstream/us-synthetic-playback.json"
        ))
        .unwrap();
        let (configuration, _) = document.prepare().unwrap();
        let mut playback = configuration.rxmer.unwrap();
        let before = retained_footprint(&playback).unwrap();
        let capacity = playback.sources[0].powers.capacity();
        playback.sources[0]
            .powers
            .reserve_exact(capacity + SPARE_CACHED_VALUES);
        let delta = (playback.sources[0].powers.capacity() - capacity) * size_of::<Option<f64>>();
        assert_eq!(
            retained_footprint(&playback).unwrap().payload_bytes - before.payload_bytes,
            delta
        );
    }
}
