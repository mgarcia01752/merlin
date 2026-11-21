/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! TX composition: one pending physical frame; continuous source, per-grant mask.
use super::{encode_grant, BurstAssembler};
use crate::{
    bit_processing::{pilot_reference, Randomizer23},
    configuration::UsConfiguration,
    error::invalid,
    grant::{select, GrantPlan},
    resources::{CellRole, ResourcePlan},
    signal_processing::{map_symbol, PreEqBank, Transform},
    source::Prbs15,
    types::{BitRange, Identity, Iq, SampleBlock},
    PhyError, PhyResult,
};
use std::sync::Arc;

/// One emitted frame with bounded actual first-symbol stage observations.
pub struct TransmittedFrame {
    /// Absolute configured frame identity.
    pub frame: u64,
    /// Actual indexed CP/taper/overlap samples; final frame includes tail once.
    pub block: SampleBlock,
    /// First complete physical symbol before Pre-EQ, for passive inspection.
    pub grid: Vec<Iq>,
    /// Full bounded frame truth retained only for independent simulator metrics.
    pub reference_grid: Vec<Iq>,
    /// Same first symbol after applied transmitter coefficients.
    pub corrected_grid: Vec<Iq>,
    /// First useful time symbol, distinct from continuous emitted samples.
    pub useful: Vec<Iq>,
    /// Actual first useful sample origin, independent of optional leading silence.
    pub useful_origin: u64,
}

/// Finite scheduled transmitter holding one grant bit stream and one frame.
pub struct Transmitter {
    config: Arc<UsConfiguration>,
    identity: Identity,
    resources: ResourcePlan,
    plan: GrantPlan,
    source_range: BitRange,
    bits: Vec<u8>,
    next_frame: usize,
    guards: crate::configuration::GuardIntervals,
    bank: PreEqBank,
    updates: Option<crate::measurements::PreEqPlayback>,
    transform: Transform,
    assembler: Option<BurstAssembler>,
    pilots: Vec<Iq>,
}

impl Transmitter {
    /// Admits resources, bank, encoding and candidate source state atomically.
    ///
    /// `config` is frozen, `identity` binds this grant, `source` stays continuous
    /// across grants and `bank` is transmitter-only C. Returns idle frame state;
    /// any error leaves the supplied source unchanged. No-data consumes no bits.
    pub fn new(
        config: Arc<UsConfiguration>,
        identity: Identity,
        source: &mut Prbs15,
        bank: PreEqBank,
    ) -> PhyResult<Self> {
        identity.validate()?;
        if identity.grant != config.grant().id {
            return Err(PhyError::Association {
                field: "transmitter.grant",
            });
        }
        let resources = ResourcePlan::new(&config)?;
        let plan = select(resources.grant_bits)?;
        let mode = config.waveform().numerology();
        bank.apply(&config, &vec![Iq::default(); mode.fft_len()])?;
        let mut candidate = source.clone();
        let payload = candidate.generate(identity, plan.source_bits)?;
        let encoded = encode_grant(&plan, &payload.bits)?;
        let bits = Randomizer23::new(config.selected_profile().seed())?.apply_bits(&encoded)?;
        let assembler = if plan.transmit {
            Some(BurstAssembler::new(
                identity,
                config.waveform().clone(),
                config.grant().epoch_sample,
                config.grant().frames * config.waveform().frame_symbols(),
            )?)
        } else {
            None
        };
        *source = candidate;
        Ok(Self {
            config,
            identity,
            resources,
            plan,
            source_range: payload.range,
            bits,
            next_frame: 0,
            guards: Default::default(),
            bank,
            updates: None,
            transform: Transform::new(mode),
            assembler,
            pilots: pilot_reference(mode),
        })
    }
    /// Accepts explicit synthetic leading/trailing silence before any emission.
    ///
    /// Returns unit; rejects origin/budget violations and changes after a frame.
    /// Guards do not change configured B or any source, mask or pilot clock.
    pub fn set_guards(&mut self, guards: crate::configuration::GuardIntervals) -> PhyResult<()> {
        guards.validate(self.config.grant().epoch_sample)?;
        if self.next_frame != 0 {
            return Err(invalid("transmitter.guard_state"));
        }
        self.guards = guards;
        Ok(())
    }
    /// Accepts a fully preflighted synthetic symbol schedule before emission.
    ///
    /// Returns unit; rejects grid/initial mismatch and any change after a frame.
    /// Source admission is already complete; high-level runs preflight before it.
    pub fn set_pre_eq_updates(
        &mut self,
        updates: crate::measurements::PreEqPlayback,
    ) -> PhyResult<()> {
        updates.validate(&self.config, &self.bank)?;
        if self.next_frame != 0 {
            return Err(invalid("transmitter.update_state"));
        }
        self.updates = Some(updates);
        Ok(())
    }
    /// Borrows the pure accepted layout without generating additional source.
    ///
    /// This plan is not a transmitted waveform or a receiver result.
    pub fn plan(&self) -> &GrantPlan {
        &self.plan
    }
    /// Returns the meaningful source range actually generated for this grant.
    ///
    /// Emission and comparison coverage remain separate, especially on cancel.
    pub fn source_range(&self) -> BitRange {
        self.source_range
    }
    /// Produces the next full frame and its actual indexed waveform.
    ///
    /// Returns None after finish/no-transmission. Errors preserve prior frame
    /// cursor and assembly history; callers retain already emitted frames.
    pub fn next_frame(&mut self) -> PhyResult<Option<TransmittedFrame>> {
        if !self.plan.transmit || self.next_frame == self.config.grant().frames {
            return Ok(None);
        }
        let geometry = self.config.waveform();
        let n = geometry.numerology().fft_len();
        let k = geometry.frame_symbols();
        let mut grid = vec![Iq::default(); n * k];
        let frame_offset = self.next_frame * self.resources.frame_bits;
        for cell in &self.resources.visits {
            let index = cell.symbol * n + cell.carrier;
            grid[index] = if cell.role == CellRole::KnownPilot {
                self.pilots[cell.carrier] * cell.gain
            } else {
                let start = frame_offset + cell.bit_offset;
                let label = self.bits[start..start + cell.loading as usize]
                    .iter()
                    .fold(0_u16, |word, bit| (word << 1) | u16::from(*bit));
                map_symbol(cell.loading, label)? * cell.gain
            };
        }
        let mut assembly = self
            .assembler
            .clone()
            .ok_or_else(|| invalid("transmitter.assembly"))?;
        let mut output = Vec::with_capacity(k * geometry.stride() + geometry.rolloff());
        if self.next_frame == 0 {
            output.resize(self.guards.leading, Iq::default());
        }
        let mut first_corrected = Vec::new();
        let mut first_useful = Vec::new();
        for symbol in 0..k {
            let bank = match &self.updates {
                Some(updates) => updates.at(self.next_frame * k + symbol)?,
                None => &self.bank,
            };
            let corrected = bank.apply(&self.config, &grid[symbol * n..(symbol + 1) * n])?;
            let useful = self.transform.inverse(&corrected)?;
            output.extend(assembly.push(&useful)?.samples);
            if symbol == 0 {
                first_corrected = corrected;
                first_useful = useful;
            }
        }
        if self.next_frame + 1 == self.config.grant().frames {
            output.extend(assembly.finish()?.samples);
            output.resize(output.len() + self.guards.trailing, Iq::default());
        }
        let start =
            self.config.grant().epoch_sample + (self.next_frame * k * geometry.stride()) as u64;
        let useful_origin = start + geometry.prefix() as u64;
        let start = if self.next_frame == 0 {
            start - self.guards.leading as u64
        } else {
            start
        };
        let frame = self.config.grant().first_frame + self.next_frame as u64;
        self.next_frame += 1;
        self.assembler = Some(assembly);
        Ok(Some(TransmittedFrame {
            frame,
            block: SampleBlock {
                identity: self.identity,
                start,
                samples: output,
            },
            grid: grid[..n].to_vec(),
            reference_grid: grid,
            corrected_grid: first_corrected,
            useful: first_useful,
            useful_origin,
        }))
    }
}
