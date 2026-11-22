/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Streaming RX-01..09, one useful window/frame and one retained word of LLRs.
use crate::{
    bit_processing::Randomizer23,
    configuration::{ReceiverConfig, UsConfiguration},
    error::{invalid, limit},
    grant::{select, GrantPlan},
    instruments::soft::store::{SoftCellMetrics, SoftFrameContext},
    resources::{Cell, CellRole, ResourcePlan},
    signal_processing::{equalize, estimate, max_log, Transform},
    types::{Identity, Iq, SampleBlock},
    PhyError, PhyResult,
};
use fec_codec::upstream_ldpc::{DecodeResult, LdpcCodec, LdpcSettings};
use serde::Serialize;
use std::sync::Arc;

/// Actual decoded word and logical payload positions, without source truth.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DecodedWord {
    /// Zero-based codeword order within the configured grant.
    pub ordinal: usize,
    /// Meaningful source-region offset known from the shared byte/fill plan.
    pub payload_offset: usize,
    /// Decisions only for the whole-byte payload region; includes failed words.
    pub payload: Vec<u8>,
    /// Fill-one consistency, distinct from decoder parity/source comparison.
    pub information_fill_valid: bool,
    /// Actual codec decisions, convergence, parity, known zeros and iterations.
    pub decoded: DecodeResult,
}

/// Owned completed science at an input/cancellation boundary.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReceiveBatch {
    /// Actual completed codewords, including failed decoder decisions.
    pub words: Vec<DecodedWord>,
    /// True when cooperative stop ended this receive stream mid-call.
    pub interrupted: bool,
}

/// Job-owned receiver with transactional input calls and complete missing ranges.
#[derive(Clone)]
pub struct Receiver {
    config: Arc<UsConfiguration>,
    identity: Identity,
    policy: ReceiverConfig,
    resources: Arc<ResourcePlan>,
    plan: GrantPlan,
    transform: Arc<Transform>,
    mask: Randomizer23,
    next_tick: u64,
    observation_end: u64,
    next_symbol: usize,
    current: Vec<Iq>,
    available: usize,
    frame: Vec<Option<Vec<Iq>>>,
    next_word: usize,
    word_llrs: Vec<f64>,
    coded_offset: usize,
    finished: bool,
    missing_symbols: usize,
    frames: usize,
    last_grid: Option<Vec<Iq>>,
    last_corrected: Vec<Iq>,
    last_cells: Vec<(Cell, Option<Iq>)>,
    receiver_estimates: Option<crate::instruments::receiver::ReceiverEstimateStore>,
}

impl Receiver {
    /// Independently derives resources/layout from `config`, timing and `policy`.
    ///
    /// `identity` binds the receive stream. Returns idle state at configured B.
    /// Rejects invalid IDs, mismatched grant and resource budgets before samples.
    /// No transmitter object, seed, original bits or channel model is accepted.
    pub fn new(
        config: Arc<UsConfiguration>,
        identity: Identity,
        policy: ReceiverConfig,
    ) -> PhyResult<Self> {
        identity.validate()?;
        if identity.grant != config.grant().id {
            return Err(PhyError::Association {
                field: "receiver.grant",
            });
        }
        let resources = ResourcePlan::new(&config)?;
        let plan = select(resources.grant_bits)?;
        let mode = config.waveform().numerology();
        let n = mode.fft_len();
        let k = config.waveform().frame_symbols();
        let seed = config.selected_profile().seed();
        let next_tick = config.grant().epoch_sample;
        let observation_end = next_tick
            + (config.grant().frames * k * config.waveform().stride() + config.waveform().rolloff())
                as u64;
        Ok(Self {
            config,
            identity,
            policy,
            resources: Arc::new(resources),
            plan,
            transform: Arc::new(Transform::new(mode)),
            mask: Randomizer23::new(seed)?,
            next_tick,
            observation_end,
            next_symbol: 0,
            current: vec![Iq::default(); n],
            available: 0,
            frame: Vec::with_capacity(k),
            next_word: 0,
            word_llrs: Vec::new(),
            coded_offset: 0,
            finished: false,
            missing_symbols: 0,
            frames: 0,
            last_grid: None,
            last_corrected: Vec::new(),
            last_cells: Vec::new(),
            receiver_estimates: None,
        })
    }
    /// Accepts an explicit finite receive observation interval independently of B.
    ///
    /// `start`/`end` are absolute received ticks and can truncate or surround the
    /// scheduled burst. Returns idle state; rejects reversed/excessive extents.
    /// Configured useful windows are unchanged, including windows outside input.
    pub fn for_observation(
        config: Arc<UsConfiguration>,
        identity: Identity,
        policy: ReceiverConfig,
        start: u64,
        end: u64,
    ) -> PhyResult<Self> {
        if end
            .checked_sub(start)
            .is_none_or(|length| length > crate::configuration::MAX_JOB_SAMPLES as u64)
        {
            return Err(limit("receiver.observation_extent"));
        }
        let mut receiver = Self::new(config, identity, policy)?;
        receiver.next_tick = start;
        receiver.observation_end = end;
        Ok(receiver)
    }
    /// Admits actual contiguous indexed `block` and returns newly completed words.
    ///
    /// Rejects nonfinite/oversized (>65536) data, gaps, duplicates, stale IDs and
    /// failed numerical transactions without changing accepted receiver state.
    /// CP/guards are discarded by configured index; windows are never slid.
    pub fn push(&mut self, block: &SampleBlock) -> PhyResult<Vec<DecodedWord>> {
        Ok(self.push_with_control(block, &mut || false)?.words)
    }

    /// Admits `block` with a cooperative `stop` probe at cell/codec boundaries.
    ///
    /// Returns all completed words even if stopped during this input call.
    /// Stop commits the completed prefix and terminates the stream; invalid input
    /// still rejects transactionally. The probe receives no scientific buffers.
    pub fn push_with_control(
        &mut self,
        block: &SampleBlock,
        stop: &mut dyn FnMut() -> bool,
    ) -> PhyResult<ReceiveBatch> {
        let end = block.validate(65536)?;
        self.admit(block.identity, block.start, end)?;
        let mut candidate = self.clone();
        let mut output = Vec::new();
        let interrupted =
            candidate.consume(block.start, end, Some(&block.samples), &mut output, stop)?;
        *self = candidate;
        Ok(ReceiveBatch {
            words: output,
            interrupted,
        })
    }
    /// Admits an explicit missing half-open sample interval at `start`/`length`.
    ///
    /// Returns completed word decisions with erasures at original positions.
    /// Rejects overflow, noncontiguous or stale input without inventing zeros.
    pub fn gap(&mut self, start: u64, length: usize) -> PhyResult<Vec<DecodedWord>> {
        let end = start
            .checked_add(length as u64)
            .ok_or_else(|| limit("receiver.gap"))?;
        self.admit(self.identity, start, end)?;
        let mut candidate = self.clone();
        let mut output = Vec::new();
        candidate.consume(start, end, None, &mut output, &mut || false)?;
        *self = candidate;
        Ok(output)
    }
    /// Closes finite observation, marking unobserved scheduled windows missing.
    ///
    /// Returns newly completed words; repeat finish returns empty. Partial
    /// useful samples never become an FFT window or a claim of full reception.
    pub fn finish(&mut self) -> PhyResult<Vec<DecodedWord>> {
        if self.finished {
            return Ok(Vec::new());
        }
        let end = self.config.grant().epoch_sample
            + (self.total_symbols() * self.config.waveform().stride()) as u64;
        let mut candidate = self.clone();
        let mut output = Vec::new();
        candidate.consume(
            candidate.next_tick,
            end.max(candidate.next_tick),
            None,
            &mut output,
            &mut || false,
        )?;
        candidate.finished = true;
        *self = candidate;
        Ok(output)
    }
    /// Returns the count of useful symbols with incomplete observed coverage.
    ///
    /// Zero sample values are available data and do not increase this count.
    pub fn missing_symbols(&self) -> usize {
        self.missing_symbols
    }
    /// Returns completed frames actually processed under the accepted policy.
    ///
    /// A frame can contain missing windows; consult coverage and FEC separately.
    pub fn frames(&self) -> usize {
        self.frames
    }
    /// Borrows the last completed frame's first observed FFT grid when retained.
    ///
    /// This is received data, never a supplied transmitter grid.
    pub fn last_grid(&self) -> Option<&[Iq]> {
        self.last_grid.as_deref()
    }
    /// Borrows available equalized payload cells from the last completed frame.
    ///
    /// Cells are in inverse-visit order after role gain removal, not source bits.
    pub fn last_corrected(&self) -> &[Iq] {
        &self.last_corrected
    }
    /// Borrows matched available/unavailable payload cells of the last frame.
    ///
    /// Physical positions/loading are retained; inspection never shifts missing
    /// cells or changes subsequent masks, confidence or decoding.
    pub fn last_payload_cells(&self) -> &[(Cell, Option<Iq>)] {
        &self.last_cells
    }

    /// Transfers the optional bounded observer without changing scientific receiver state.
    /// # Arguments
    /// * `store` - Aggregate retained history from earlier grants, or None to detach it.
    /// # Returns
    /// Previously attached history, including completed frames before cancellation or failure.
    pub(crate) fn replace_estimate_store(
        &mut self,
        store: Option<crate::instruments::receiver::ReceiverEstimateStore>,
    ) -> Option<crate::instruments::receiver::ReceiverEstimateStore> {
        std::mem::replace(&mut self.receiver_estimates, store)
    }

    fn total_symbols(&self) -> usize {
        if self.plan.transmit {
            self.config.grant().frames * self.config.waveform().frame_symbols()
        } else {
            0
        }
    }
    fn admit(&self, identity: Identity, start: u64, end: u64) -> PhyResult<()> {
        if self.finished || identity != self.identity || start != self.next_tick {
            return Err(PhyError::Association {
                field: "receiver.input",
            });
        }
        let maximum = self.observation_end;
        if end > maximum {
            return Err(limit("receiver.observation"));
        }
        Ok(())
    }
    fn consume(
        &mut self,
        start: u64,
        end: u64,
        samples: Option<&[Iq]>,
        output: &mut Vec<DecodedWord>,
        stop: &mut dyn FnMut() -> bool,
    ) -> PhyResult<bool> {
        let n = self.config.waveform().numerology().fft_len();
        let stride = self.config.waveform().stride();
        while self.next_symbol < self.total_symbols() {
            let useful = self.config.grant().epoch_sample
                + (self.next_symbol * stride + self.config.waveform().prefix()) as u64;
            let window_end = useful + n as u64;
            let from = start.max(useful);
            let to = end.min(window_end);
            if from < to {
                if let Some(values) = samples {
                    let local = (from - useful) as usize;
                    let count = (to - from) as usize;
                    self.current[local..local + count]
                        .copy_from_slice(&values[(from - start) as usize..(to - start) as usize]);
                    self.available += count;
                }
            }
            if end < window_end {
                break;
            }
            let transformed = if self.available == n {
                Some(self.transform.forward(&self.current)?)
            } else {
                self.missing_symbols += 1;
                None
            };
            self.frame.push(transformed);
            self.available = 0;
            self.next_symbol += 1;
            if self.frame.len() == self.config.waveform().frame_symbols() {
                let interrupted = self.decode_frame(output, stop)?;
                if interrupted {
                    self.finished = true;
                    self.next_tick = end;
                    return Ok(true);
                }
            }
        }
        self.next_tick = end;
        Ok(false)
    }
    fn decode_frame(
        &mut self,
        output: &mut Vec<DecodedWord>,
        stop: &mut dyn FnMut() -> bool,
    ) -> PhyResult<bool> {
        let estimates = estimate(
            &self.config,
            &self.resources,
            &self.frame,
            self.policy.estimator(),
        )?;
        self.last_grid = self.frame[0].clone();
        self.last_corrected.clear();
        self.last_cells.clear();
        for cell in &self.resources.visits {
            if stop() {
                return Ok(true);
            }
            if cell.role == CellRole::KnownPilot {
                continue;
            }
            let corrected = if let Some(grid) = &self.frame[cell.symbol] {
                equalize(
                    grid[cell.carrier],
                    estimates[cell.carrier],
                    self.policy.power_floor(),
                )?
                .map(|z| z * cell.gain.recip())
            } else {
                None
            };
            if let Some(value) = corrected {
                self.last_corrected.push(value);
            }
            self.last_cells.push((cell.clone(), corrected));
            let demapper_llrs = max_log(cell.loading, corrected, self.policy.variance())?;
            let soft = self.mask.apply_llrs(&demapper_llrs)?;
            let soft_context = SoftFrameContext {
                identity: self.identity,
                frame: self.config.grant().first_frame + self.frames as u64,
                reference_variance: self.policy.variance(),
            };
            if let Some(observer) = self
                .receiver_estimates
                .as_mut()
                .and_then(|store| store.soft.as_mut())
            {
                observer.record_cell(
                    soft_context,
                    cell,
                    self.coded_offset,
                    corrected,
                    SoftCellMetrics {
                        demapper: &demapper_llrs,
                        descrambled: &soft,
                    },
                );
            }
            for llr in soft {
                if let Some(word) = self.plan.codewords.get(self.next_word) {
                    self.word_llrs.push(llr);
                    if self.word_llrs.len() == word.transmitted_bits {
                        if stop() {
                            return Ok(true);
                        }
                        let settings = LdpcSettings::new(
                            word.code,
                            word.information_bits,
                            self.policy.iterations(),
                        )?;
                        let decoded = LdpcCodec::new(settings)?.decode(&self.word_llrs)?;
                        let payload_count = self
                            .plan
                            .source_bits
                            .saturating_sub(word.information_offset)
                            .min(word.information_bits);
                        let fill_valid = decoded.information_bits[payload_count..]
                            .iter()
                            .all(|bit| *bit == 1);
                        let decoded_word = DecodedWord {
                            ordinal: self.next_word,
                            payload_offset: word.information_offset,
                            payload: decoded.information_bits[..payload_count].to_vec(),
                            information_fill_valid: fill_valid,
                            decoded,
                        };
                        if let Some(observer) = self
                            .receiver_estimates
                            .as_mut()
                            .and_then(|store| store.soft.as_mut())
                        {
                            observer.record_word(
                                soft_context,
                                word,
                                &decoded_word,
                                &self.word_llrs,
                            );
                        }
                        output.push(decoded_word);
                        self.next_word += 1;
                        self.word_llrs.clear();
                    }
                }
                self.coded_offset += 1;
            }
        }
        if let Some(store) = &mut self.receiver_estimates {
            store.record(
                &self.config,
                self.identity,
                self.config.grant().first_frame + self.frames as u64,
                self.policy.estimator(),
                estimates,
            );
        }
        self.frames += 1;
        self.frame.clear();
        if self.frames == self.config.grant().frames && self.coded_offset != self.plan.capacity_bits
        {
            return Err(invalid("receiver.capacity"));
        }
        Ok(false)
    }
}
