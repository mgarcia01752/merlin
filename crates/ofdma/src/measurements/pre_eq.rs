/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Synthetic snapshot/update replay with explicit immutable corresponding-probe bases.
use super::SyntheticGrid;
use crate::{
    configuration::UsConfiguration,
    error::{invalid, limit},
    signal_processing::PreEqBank,
    types::Iq,
    PhyResult,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

/// An explicit synthetic probe association; never inferred from event order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProbeReference {
    /// Nonzero probe identity.
    pub id: u64,
    /// Effective initial/event bank used for this probe.
    pub bank_id: u64,
}
/// One proposed synthetic update; source values and lineage remain retained.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PreEqOperation {
    /// Complete applied bank replacement, never multiplied with prior snapshots.
    Snapshot(PreEqBank),
    /// Replace only addressed base values, then normalize the complete bank.
    Initialize {
        /// Explicit corresponding-probe base bank identity.
        base_bank: u64,
        /// Explicit synthetic probe identity.
        probe: u64,
        /// Unique physical indices and raw initialization values.
        values: Vec<(usize, Iq)>,
    },
    /// Multiply only addressed values in the explicit probe base, then normalize.
    Adjust {
        /// Explicit corresponding-probe base bank identity.
        base_bank: u64,
        /// Explicit synthetic probe identity.
        probe: u64,
        /// Unique physical indices and multiplicative factors.
        values: Vec<(usize, Iq)>,
    },
}
/// A synthetic event activates a complete derived bank before one useful symbol.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PreEqEvent {
    /// Nonzero unique event/bank identity, distinct from initial bank ID.
    pub id: u64,
    /// Strictly increasing offset, in physical symbols of each repeated grant.
    pub symbol: usize,
    /// Exact original synthetic operation and explicit lineage.
    pub operation: PreEqOperation,
}
/// Immutable bounded complete banks and replay events; no per-symbol bank cache.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PreEqPlayback {
    grid: SyntheticGrid,
    initial_id: u64,
    initial: PreEqBank,
    events: Vec<PreEqEvent>,
    probes: Vec<ProbeReference>,
    effective: Vec<PreEqBank>,
    symbols: usize,
}
impl PreEqPlayback {
    /// Preflights all original `events`, probe bases and derived complete banks.
    ///
    /// Returns immutable synthetic replay; rejects duplicates, future/missing
    /// bases, wrong grids, bad offsets, nonfinite values and coefficient budgets.
    pub fn new(
        config: &UsConfiguration,
        initial_id: u64,
        initial: PreEqBank,
        events: Vec<PreEqEvent>,
        probes: Vec<ProbeReference>,
    ) -> PhyResult<Self> {
        if initial_id == 0
            || events.len() >= 256
            || probes.len() > 256
            || (events.len() + 1) * config.waveform().numerology().fft_len() > 1_048_576
        {
            return Err(limit("pre_eq.schedule"));
        }
        initial.validate(config)?;
        let symbols = config.grant().frames * config.waveform().frame_symbols();
        let mut banks = BTreeMap::from([(initial_id, initial.clone())]);
        let mut probe_map = BTreeMap::new();
        for probe in &probes {
            if probe.id == 0 || probe_map.insert(probe.id, probe.bank_id).is_some() {
                return Err(invalid("pre_eq.probe"));
            }
        }
        let mut effective = Vec::new();
        let mut previous = 0;
        let mut value_count = 0_usize;
        for event in &events {
            if event.id == 0
                || banks.contains_key(&event.id)
                || event.symbol <= previous
                || event.symbol >= symbols
            {
                return Err(invalid("pre_eq.event"));
            }
            let bank = match &event.operation {
                PreEqOperation::Snapshot(bank) => {
                    bank.validate(config)?;
                    bank.clone()
                }
                PreEqOperation::Initialize {
                    base_bank,
                    probe,
                    values,
                }
                | PreEqOperation::Adjust {
                    base_bank,
                    probe,
                    values,
                } => {
                    value_count = value_count
                        .checked_add(values.len())
                        .ok_or_else(|| limit("pre_eq.update_values"))?;
                    if value_count > 1_048_576
                        || values.is_empty()
                        || probe_map.get(probe) != Some(base_bank)
                    {
                        return Err(invalid("pre_eq.update_base"));
                    }
                    let mut raw = banks
                        .get(base_bank)
                        .ok_or_else(|| invalid("pre_eq.future_base"))?
                        .coefficients()
                        .to_vec();
                    let mut indices = BTreeSet::new();
                    for &(bin, value) in values {
                        if bin >= raw.len()
                            || !indices.insert(bin)
                            || config.excluded().contains(&bin)
                            || !value.is_finite()
                        {
                            return Err(invalid("pre_eq.update_values"));
                        }
                        raw[bin] = if matches!(event.operation, PreEqOperation::Adjust { .. }) {
                            raw[bin] * value
                        } else {
                            value
                        };
                    }
                    PreEqBank::normalized(config, raw, event.id)?
                }
            };
            banks.insert(event.id, bank.clone());
            effective.push(bank);
            previous = event.symbol;
        }
        if probes
            .iter()
            .any(|probe| !banks.contains_key(&probe.bank_id))
        {
            return Err(invalid("pre_eq.probe_bank"));
        }
        Ok(Self {
            grid: SyntheticGrid::new(config),
            initial_id,
            initial,
            events,
            probes,
            effective,
            symbols,
        })
    }
    /// Checks reset identity, finite duration and the configured static initial bank.
    ///
    /// Returns unit; rejects a stale schedule or silently replaced initial bank.
    pub fn validate(&self, config: &UsConfiguration, initial: &PreEqBank) -> PhyResult<()> {
        self.grid.validate(config)?;
        if self.symbols != config.grant().frames * config.waveform().frame_symbols()
            || initial != &self.initial
        {
            return Err(invalid("pre_eq.schedule_initial"));
        }
        Ok(())
    }
    /// Selects the complete immutable bank at physical `symbol` without advancing.
    ///
    /// Returns a borrowed bank; rejects outside the accepted finite duration.
    pub fn at(&self, symbol: usize) -> PhyResult<&PreEqBank> {
        if symbol >= self.symbols {
            return Err(invalid("pre_eq.symbol"));
        }
        let count = self.events.partition_point(|event| event.symbol <= symbol);
        Ok(if count == 0 {
            &self.initial
        } else {
            &self.effective[count - 1]
        })
    }
    /// Tests every effective bank for exact unity/excluded-zero compatibility.
    ///
    /// Returns false for any later nonunity event, before execution consumes input.
    pub fn all_unity(&self) -> bool {
        self.initial.is_unity() && self.effective.iter().all(PreEqBank::is_unity)
    }
}
