/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Restricted symbol-conditioned Gaussian residual; actual time samples only.
use crate::{
    configuration::UsConfiguration,
    error::{invalid, limit},
    measurements::RxmerPlayback,
    resources::ResourcePlan,
    signal_processing::Transform,
    transmitter::BurstAssembler,
    types::{Identity, Iq, SampleBlock},
    PhyError, PhyResult,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::StandardNormal;
use std::sync::Arc;

/// Bounded RXMER-derived waveform with independent RNG and overlap history.
#[derive(Clone)]
pub struct RxmerChannel {
    config: Arc<UsConfiguration>,
    playback: Arc<RxmerPlayback>,
    identity: Identity,
    next_tick: u64,
    end: u64,
    waveform_start: u64,
    waveform_end: u64,
    symbol: usize,
    active: Arc<[bool]>,
    rng: ChaCha8Rng,
    assembly: BurstAssembler,
    transform: Arc<Transform>,
    ready: Vec<Iq>,
    cursor: usize,
    finished: bool,
}
impl RxmerChannel {
    /// Admits `playback`, grid/identity and explicit 32-byte independent noise seed.
    ///
    /// Returns empty synthesis state; config-level admission must additionally
    /// enforce identity H, every-bank unity C and disabled sample noise/timing.
    pub fn new(
        config: Arc<UsConfiguration>,
        playback: RxmerPlayback,
        identity: Identity,
        seed: [u8; 32],
    ) -> PhyResult<Self> {
        Self::with_guards(config, playback, identity, seed, Default::default())
    }
    /// Admits the same residual plus explicit zero-noise observation guards.
    ///
    /// Returns bounded state; guards change only support, never symbol/RNG clocks.
    pub fn with_guards(
        config: Arc<UsConfiguration>,
        playback: RxmerPlayback,
        identity: Identity,
        seed: [u8; 32],
        guards: crate::configuration::GuardIntervals,
    ) -> PhyResult<Self> {
        guards.validate(config.grant().epoch_sample)?;
        playback.validate_model(&config)?;
        if identity.grant != config.grant().id {
            return Err(invalid("rxmer.grant"));
        }
        let n = config.waveform().numerology().fft_len();
        let k = config.waveform().frame_symbols();
        let symbols = k * config.grant().frames;
        let mut active = vec![false; n * k];
        for cell in ResourcePlan::new(&config)?.visits {
            active[cell.symbol * n + cell.carrier] = true;
        }
        let origin = config.grant().epoch_sample;
        let support = symbols * config.waveform().stride() + config.waveform().rolloff();
        let end = origin
            .checked_add(support as u64)
            .ok_or_else(|| limit("rxmer.end"))?;
        let assembly = BurstAssembler::new(identity, config.waveform().clone(), origin, symbols)?;
        let transform = Arc::new(Transform::new(config.waveform().numerology()));
        Ok(Self {
            config,
            playback: Arc::new(playback),
            identity,
            next_tick: origin - guards.leading as u64,
            end: end
                .checked_add(guards.trailing as u64)
                .ok_or_else(|| limit("rxmer.guard_end"))?,
            waveform_start: origin,
            waveform_end: end,
            symbol: 0,
            active: active.into(),
            rng: ChaCha8Rng::from_seed(seed),
            assembly,
            transform,
            ready: Vec::new(),
            cursor: 0,
            finished: false,
        })
    }
    /// Adds one borrowed contiguous TX block to independently synthesized residual.
    ///
    /// Returns owned actual samples. Invalid identity, finite/budget or coverage
    /// failures preserve RNG, overlap, ready samples and all input cursors.
    pub fn push(&mut self, block: &SampleBlock) -> PhyResult<SampleBlock> {
        let end = block.validate(65536)?;
        if self.finished
            || block.identity != self.identity
            || block.start != self.next_tick
            || end > self.end
        {
            return Err(PhyError::Association {
                field: "rxmer.input",
            });
        }
        let mut candidate = self.clone();
        let mut samples = Vec::with_capacity(block.samples.len());
        for &sample in &block.samples {
            let tick = block.start + samples.len() as u64;
            if tick < candidate.waveform_start || tick >= candidate.waveform_end {
                samples.push(sample);
                continue;
            }
            if candidate.cursor == candidate.ready.len() {
                candidate.generate()?;
            }
            let value = sample + candidate.ready[candidate.cursor];
            if !value.is_finite() {
                return Err(invalid("rxmer.output"));
            }
            samples.push(value);
            candidate.cursor += 1;
        }
        candidate.next_tick = end;
        *self = candidate;
        Ok(SampleBlock {
            identity: self.identity,
            start: block.start,
            samples,
        })
    }
    /// Validates complete TX-matched support; no extra response/delay drain exists.
    ///
    /// Returns an empty indexed block; repeated finish is harmless. Incomplete
    /// input is an error, never a successful implicit fill or noise-only tail.
    pub fn finish(&mut self) -> PhyResult<SampleBlock> {
        if self.next_tick != self.end {
            return Err(invalid("rxmer.finish"));
        }
        self.finished = true;
        Ok(SampleBlock {
            identity: self.identity,
            start: self.next_tick,
            samples: Vec::new(),
        })
    }
    fn generate(&mut self) -> PhyResult<()> {
        let n = self.config.waveform().numerology().fft_len();
        let k = self.config.waveform().frame_symbols();
        if self.symbol == self.config.grant().frames * k {
            self.ready = self.assembly.finish()?.samples;
        } else {
            let power = self.playback.at(self.symbol)?;
            let mut grid = vec![Iq::default(); n];
            for (bin, value) in grid.iter_mut().enumerate() {
                let gi: f64 = self.rng.sample(StandardNormal);
                let gq: f64 = self.rng.sample(StandardNormal);
                if self.active[(self.symbol % k) * n + bin] {
                    let rho = power.powers[bin].ok_or_else(|| invalid("rxmer.coverage"))?;
                    let scale = (rho * 0.5).sqrt();
                    *value = Iq {
                        re: gi * scale,
                        im: gq * scale,
                    };
                }
            }
            self.ready = self.assembly.push(&self.transform.inverse(&grid)?)?.samples;
            self.symbol += 1;
        }
        self.cursor = 0;
        if self.ready.is_empty() {
            return Err(invalid("rxmer.exhausted"));
        }
        Ok(())
    }
}
