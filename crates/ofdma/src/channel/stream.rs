/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Ordered causal FIR, sample AWGN and emitted-clock delay/phase with exact drain.
use crate::{
    configuration::SAMPLE_RATE_HZ,
    error::{invalid, limit},
    types::{Identity, Iq, SampleBlock},
    PhyError, PhyResult,
};
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, f64::consts::TAU};

/// Frozen synthetic channel settings, distinct from measured RxMER or TX C.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChannelConfig {
    /// Causal impulse taps h[0..L], 1..256; exact [1+j0] selects bypass.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 256>")]
    pub taps: Vec<Iq>,
    /// Complex circular sample-noise power E|n|²; zero consumes no RNG draws.
    pub noise_variance: f64,
    /// Explicit independent synthetic noise stream seed.
    pub noise_seed: [u8; 32],
    /// Nonnegative integer delay in sample ticks, at most 4096.
    pub delay_samples: usize,
    /// First emitted-tick phase in radians, independent of receiver timing.
    pub phase_radians: f64,
    /// Positive emitted-clock rotation frequency in hertz, within Nyquist.
    pub frequency_offset_hz: f64,
}

impl Default for ChannelConfig {
    /// Selects bit-preserving unity response/noise/timing settings.
    ///
    /// Returns configuration only; no RNG is advanced or signal generated.
    fn default() -> Self {
        Self {
            taps: vec![Iq { re: 1.0, im: 0.0 }],
            noise_variance: 0.0,
            noise_seed: [1; 32],
            delay_samples: 0,
            phase_radians: 0.0,
            frequency_offset_hz: 0.0,
        }
    }
}

impl ChannelConfig {
    /// Checks the complete synthetic stage composition before accepting samples.
    ///
    /// Returns unit; rejects nonfinite settings, tap/delay/work limits and invalid
    /// variance/CFO without changing the candidate or any accepted pipeline.
    pub fn validate(&self) -> PhyResult<()> {
        if self.taps.is_empty()
            || self.taps.len() > 256
            || self.taps.iter().any(|z| !z.is_finite())
            || !self.noise_variance.is_finite()
            || self.noise_variance < 0.0
            || self.noise_variance > 0.0 && (self.noise_variance * 0.5).sqrt() == 0.0
            || self.delay_samples > 4096
            || !self.phase_radians.is_finite()
            || !self.frequency_offset_hz.is_finite()
            || self.frequency_offset_hz.abs() > f64::from(SAMPLE_RATE_HZ) / 2.0
        {
            return Err(invalid("channel.configuration"));
        }
        Ok(())
    }
    /// Returns exact extra terminal response/delay sample count.
    ///
    /// This count does not alter configured receiver window origins.
    pub fn extra_samples(&self) -> PhyResult<usize> {
        self.validate()?;
        Ok(self.taps.len() - 1 + self.delay_samples)
    }
}

/// Transactional bounded stream; all numerical algorithms and RNG are Rust.
#[derive(Clone)]
pub struct Channel {
    config: ChannelConfig,
    identity: Identity,
    input_next: u64,
    output_next: u64,
    history: VecDeque<Iq>,
    delay: VecDeque<Iq>,
    rng: ChaCha8Rng,
    phase: f64,
    response_unity: bool,
    phase_bypass: bool,
    finished: bool,
    input_count: usize,
}

impl Channel {
    /// Freezes `config`, `identity` and absolute stream `origin` before processing.
    ///
    /// Returns independent empty-history state; rejects incompatible limits/IDs.
    /// Receiver settings/source truth are not accepted by the channel.
    pub fn new(config: ChannelConfig, identity: Identity, origin: u64) -> PhyResult<Self> {
        config.validate()?;
        identity.validate()?;
        let response_unity = config.taps == [Iq { re: 1.0, im: 0.0 }];
        let phase_bypass = config.phase_radians == 0.0 && config.frequency_offset_hz == 0.0;
        let history = VecDeque::from(vec![Iq::default(); config.taps.len() - 1]);
        let delay = VecDeque::from(vec![Iq::default(); config.delay_samples]);
        let rng = ChaCha8Rng::from_seed(config.noise_seed);
        let phase = config.phase_radians.rem_euclid(TAU);
        Ok(Self {
            config,
            identity,
            input_next: origin,
            output_next: origin,
            history,
            delay,
            rng,
            phase,
            response_unity,
            phase_bypass,
            finished: false,
            input_count: 0,
        })
    }
    /// Processes a contiguous borrowed `block` through the admitted stage order.
    ///
    /// Returns exactly one output sample per input, preserving inputs. Rejects
    /// stale/gapped/nonfinite or excessive MAC work before committing any state.
    /// Zero settings take a bit-preserving bypass without arithmetic/noise draws.
    pub fn push(&mut self, block: &SampleBlock) -> PhyResult<SampleBlock> {
        let end = block.validate(65536)?;
        if self.finished || block.identity != self.identity || block.start != self.input_next {
            return Err(PhyError::Association {
                field: "channel.input",
            });
        }
        if block.samples.len() * self.config.taps.len() > 1_048_576 {
            return Err(limit("channel.convolution_work"));
        }
        end.checked_add(self.config.extra_samples()? as u64)
            .ok_or_else(|| limit("channel.output_end"))?;
        let mut next = self.clone();
        let start = next.output_next;
        let mut samples = Vec::with_capacity(block.samples.len());
        for &value in &block.samples {
            let response = next.response(value)?;
            let noisy = next.noise(response)?;
            samples.push(next.delayed(noisy)?);
        }
        next.input_next = end;
        next.input_count = next
            .input_count
            .checked_add(block.samples.len())
            .ok_or_else(|| limit("channel.input_count"))?;
        next.output_next += samples.len() as u64;
        *self = next;
        Ok(SampleBlock {
            identity: self.identity,
            start,
            samples,
        })
    }
    /// Drains causal FIR then the delay FIFO exactly once after nonempty input.
    ///
    /// Returns only actual terminal samples; repeated finish is empty. Noise is
    /// generated for FIR response output, never for the delay-only final drain.
    pub fn finish(&mut self) -> PhyResult<SampleBlock> {
        let start = self.output_next;
        if self.finished || self.input_count == 0 {
            self.finished = true;
            return Ok(SampleBlock {
                identity: self.identity,
                start,
                samples: Vec::new(),
            });
        }
        let mut next = self.clone();
        let mut samples = Vec::new();
        for _ in 0..next.config.taps.len() - 1 {
            let response = next.response(Iq::default())?;
            let noisy = next.noise(response)?;
            samples.push(next.delayed(noisy)?);
        }
        while let Some(value) = next.delay.pop_front() {
            samples.push(next.rotate(value)?);
        }
        next.output_next += samples.len() as u64;
        next.finished = true;
        *self = next;
        Ok(SampleBlock {
            identity: self.identity,
            start,
            samples,
        })
    }
    fn response(&mut self, value: Iq) -> PhyResult<Iq> {
        if self.response_unity {
            return Ok(value);
        }
        let mut result = value * self.config.taps[0];
        for (old, tap) in self.history.iter().rev().zip(&self.config.taps[1..]) {
            result = result + *old * *tap;
        }
        if !result.is_finite() {
            return Err(invalid("channel.response"));
        }
        if !self.history.is_empty() {
            self.history.pop_front();
            self.history.push_back(value);
        }
        Ok(result)
    }
    fn noise(&mut self, value: Iq) -> PhyResult<Iq> {
        if self.config.noise_variance == 0.0 {
            return Ok(value);
        }
        let scale = (self.config.noise_variance * 0.5).sqrt();
        let real: f64 = StandardNormal.sample(&mut self.rng);
        let imaginary: f64 = StandardNormal.sample(&mut self.rng);
        let result = value
            + Iq {
                re: real * scale,
                im: imaginary * scale,
            };
        if !result.is_finite() {
            return Err(invalid("channel.noise"));
        }
        Ok(result)
    }
    fn delayed(&mut self, value: Iq) -> PhyResult<Iq> {
        let value = if self.delay.is_empty() {
            value
        } else {
            self.delay.push_back(value);
            self.delay
                .pop_front()
                .ok_or_else(|| invalid("channel.delay"))?
        };
        self.rotate(value)
    }
    fn rotate(&mut self, value: Iq) -> PhyResult<Iq> {
        if self.phase_bypass {
            return Ok(value);
        }
        let (im, re) = self.phase.sin_cos();
        let result = value * Iq { re, im };
        self.phase = (self.phase
            + TAU * self.config.frequency_offset_hz / f64::from(SAMPLE_RATE_HZ))
        .rem_euclid(TAU);
        if !result.is_finite() {
            return Err(invalid("channel.phase"));
        }
        Ok(result)
    }
}

impl RetainedPayload for ChannelConfig {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            taps,
            noise_variance,
            noise_seed,
            delay_samples,
            phase_radians,
            frequency_offset_hz,
        } = self;
        sum([
            taps.retained_children(),
            noise_variance.retained_children(),
            noise_seed.retained_children(),
            delay_samples.retained_children(),
            phase_radians.retained_children(),
            frequency_offset_hz.retained_children(),
        ])
    }
}
