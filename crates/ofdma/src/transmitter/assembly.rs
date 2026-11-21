/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! TX-08 retained-tail assembly at stride N+C, with isolated burst edges.
use crate::{
    configuration::Waveform,
    error::{invalid, limit},
    signal_processing::{cyclic_extension, transmit_window},
    types::{Identity, Iq, SampleBlock},
    PhyError, PhyResult,
};

/// One admitted finite burst; owns overlap state and emits each tail exactly once.
#[derive(Clone)]
pub struct BurstAssembler {
    identity: Identity,
    geometry: Waveform,
    total: usize,
    next: usize,
    tick: u64,
    tail: Vec<Iq>,
    window: Vec<f64>,
    finished: bool,
}

impl BurstAssembler {
    /// Freezes `geometry`, prefix `origin`, symbol count `total` and `identity`.
    ///
    /// Returns empty-history state. Rejects zero/excessive count or tick overflow
    /// before accepting any useful symbol; no source or receiver state is used.
    pub fn new(
        identity: Identity,
        geometry: Waveform,
        origin: u64,
        total: usize,
    ) -> PhyResult<Self> {
        identity.validate()?;
        if total == 0 || total > 9216 {
            return Err(invalid("burst.symbols"));
        }
        let support = total
            .checked_mul(geometry.stride())
            .and_then(|v| v.checked_add(geometry.rolloff()))
            .ok_or_else(|| limit("burst.support"))?;
        if support > crate::configuration::MAX_JOB_SAMPLES {
            return Err(limit("burst.support"));
        }
        origin
            .checked_add(support as u64)
            .ok_or_else(|| limit("burst.end"))?;
        let window = transmit_window(&geometry);
        Ok(Self {
            identity,
            geometry,
            total,
            next: 0,
            tick: origin,
            tail: Vec::new(),
            window,
            finished: false,
        })
    }
    /// Admits the next complete useful `symbol` and emits one indexed stride.
    ///
    /// Returns N+C actual samples, preserving caller buffers. Rejects invalid
    /// input, premature/excess state or nonfinite overlap before committing state.
    pub fn push(&mut self, symbol: &[Iq]) -> PhyResult<SampleBlock> {
        if self.finished || self.next >= self.total {
            return Err(PhyError::Association {
                field: "burst.next_symbol",
            });
        }
        let mut values = cyclic_extension(&self.geometry, symbol)?;
        for (value, weight) in values.iter_mut().zip(&self.window) {
            *value = *value * *weight;
        }
        for (value, tail) in values.iter_mut().zip(&self.tail) {
            *value = *value + *tail;
        }
        if values.iter().any(|z| !z.is_finite()) {
            return Err(invalid("burst.output"));
        }
        let tail = values.split_off(self.geometry.stride());
        let output = SampleBlock {
            identity: self.identity,
            start: self.tick,
            samples: values,
        };
        self.tail = tail;
        self.next += 1;
        self.tick += self.geometry.stride() as u64;
        Ok(output)
    }
    /// Emits the final R-sample tail once after every scheduled symbol.
    ///
    /// Returns an empty block on repeated finish; rejects premature finish.
    /// No synthetic neighbor or extra full useful symbol is appended.
    pub fn finish(&mut self) -> PhyResult<SampleBlock> {
        if self.next != self.total {
            return Err(PhyError::Association {
                field: "burst.finish",
            });
        }
        let samples = std::mem::take(&mut self.tail);
        let output = SampleBlock {
            identity: self.identity,
            start: self.tick,
            samples,
        };
        self.tick += output.samples.len() as u64;
        self.finished = true;
        Ok(output)
    }
}
