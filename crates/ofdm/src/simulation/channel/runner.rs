/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Symbol-at-a-time reference orchestration; component modules own every algorithm.
use super::{
    config::{
        MAX_ACQUISITION_SYMBOLS, MAX_CHANNEL_DECODER_ITERATIONS, MAX_COMPLETION_SYMBOLS,
        QUEUED_PAYLOAD_TARGET,
    },
    link::WaveformLink,
    ChannelPhase, ChannelRunSettings, ChannelStep, ChannelSummary, ChannelWordResult,
};
use crate::{
    error::invalid,
    simulation::{
        payload::{observation, PayloadChannelSymbol, PayloadReceiver, PayloadTransmitter},
        source::{PayloadSource, PayloadVerifier, PrbsComparison},
        test_points::{CellProbeGrid, ProbeObserver, RunPhase, TestPointProbe},
    },
    types::PLC_FRAME_SYMBOLS,
    SimulationError,
};
use fec_codec::parameters::BCH_INPUT_BITS;

/// Frozen single-channel session, advanced explicitly by the caller without reseeding.
/// Owns independent TX/RX, PLC, source and truth state; never stores a whole waveform.
/// Calls are sequential through exclusive borrowing; stopping calls pauses host work.
pub struct ChannelRun {
    constellation: Option<super::constellation::ConstellationCollector>,
    settings: ChannelRunSettings,
    link: WaveformLink,
    transmitter: Option<PayloadTransmitter>,
    receiver: Option<PayloadReceiver>,
    grid: Option<CellProbeGrid>,
    source: PayloadSource,
    verifier: PayloadVerifier,
    next_symbol: u64,
    next_impairment_epoch: usize,
    summary: ChannelSummary,
}
impl ChannelRun {
    /// Creates a fresh validated session; actual PLC acquisition happens during stepping.
    ///
    /// # Arguments
    /// * `settings` - Borrowed scientific settings, cloned and frozen for this session.
    /// # Returns
    /// Unstarted acquisition state with a finite duration plan and empty comparisons.
    /// # Errors
    /// Rejects invalid geometry, work limits, duration, source or codec setup.
    pub fn new(settings: &ChannelRunSettings) -> Result<Self, SimulationError> {
        let timing = settings.validate()?;
        let mut link = WaveformLink::new(
            &settings.transmission,
            &settings.layout,
            settings.ncp_modulation,
            settings.receive_block_samples,
            &settings.impairments,
            settings.llr_variance,
            settings.data_bit_loading.as_deref(),
        )?;
        if settings.receiver_mode
            == crate::signal_processing::receiver::channel_estimation::ReceiverMode::PilotEstimated
        {
            link.use_pilot_estimation(&settings.layout)?;
        }
        link.protected_constellation = settings
            .protected_constellation
            .as_ref()
            .map(|selection| {
                super::protected_constellation::ProtectedConstellation::new(
                    selection.clone(),
                    settings.receiver_mode,
                )
            })
            .transpose()?;
        log::info!(
            "channel-created observation-samples={} source-symbols={}",
            timing.observation_samples,
            timing.complete_symbols
        );
        let (source, verifier) = settings.source.stages(settings.prbs_seed)?;
        let file_source = verifier.file_summary(&source);
        Ok(Self {
            constellation: settings.constellation.as_ref().map(|selection| {
                super::constellation::ConstellationCollector::new(selection, settings.receiver_mode)
            }),
            settings: settings.clone(),
            link,
            transmitter: None,
            receiver: None,
            grid: None,
            source,
            verifier,
            next_symbol: 0,
            next_impairment_epoch: 0,
            summary: ChannelSummary {
                file_source,
                measured_mer_db: None,
                compared_data_carriers: 0,
                window: None,
                phase: ChannelPhase::Acquisition,
                timing,
                prbs_seed: settings.prbs_seed,
                payload_origin_symbol: None,
                acquisition_symbols: 0,
                observation_symbols: 0,
                completion_symbols: 0,
                interleaver_drain_symbols: 0,
                waveform_tail_samples: 0,
                total_transmitted_samples: 0,
                rx_warmup_symbols: 0,
                generated_payloads: 0,
                recovered_payloads: 0,
                pending_payload_bits: 0,
                comparison: PrbsComparison {
                    first_bit: 0,
                    bits_tested: 0,
                    bit_errors: 0,
                    first_error_bit: None,
                },
                payload_ber: None,
                payload_codeword_errors: 0,
                payload_ldpc_failures: 0,
                payload_bch_failures: 0,
                payload_mean_ldpc_iterations: None,
                payload_ldpc_iteration_sum: 0,
                payload_ldpc_iterations: vec![0; MAX_CHANNEL_DECODER_ITERATIONS + 1],
                bch_corrected_bits: 0,
                accepted_ncp_fields: 0,
                ncp_ldpc_iterations: 0,
                plc_frames: 0,
                plc_bit_errors: 0,
                plc_ldpc_iterations: 0,
                maximum_bin_error: 0.0,
            },
        })
    }

    /// Borrows current constant-size scientific accounting without advancing any stage.
    ///
    /// # Returns
    /// Completed results so far. A non-Finished phase is not a completed duration run.
    pub fn summary(&self) -> &ChannelSummary {
        &self.summary
    }

    /// Copies bounded recovered original-file bytes without changing run state.
    /// # Returns
    /// None for continuous PRBS; file bytes and completion metadata otherwise.
    pub fn recovered_file(&self) -> Option<crate::simulation::source::file::RecoveredFile> {
        self.verifier.recovered_file()
    }

    /// Borrows finite paired receive observations without advancing any scientific state.
    /// # Returns
    /// None when disabled; otherwise committed pairs, including partial/unavailable status.
    pub fn constellation(&self) -> Option<&super::constellation::ConstellationResult> {
        self.constellation
            .as_ref()
            .map(|collector| collector.result())
    }

    /// Stops diagnostic collection and returns confirmed paired samples in receive order.
    /// # Returns
    /// Optional owned bounded result, flushing classified pending pairs on partial jobs.
    /// Scientific stepping remains usable and does not depend on this operation.
    pub fn take_constellation(&mut self) -> Option<super::constellation::ConstellationResult> {
        self.constellation
            .take()
            .map(|collector| collector.finish())
    }

    /// Borrows the latest bounded receiver-derived channel estimate.
    /// # Returns
    /// None in supplied-H mode or before reception; otherwise the latest physical grid.
    pub fn channel_estimate(
        &self,
    ) -> Option<&crate::signal_processing::receiver::channel_estimation::ChannelEstimate> {
        self.link.channel_estimate.as_ref()
    }
    /// Borrows acquisition-safe bounded pilot and PLC receive observations.
    /// # Returns
    /// Optional shared result, including partial streams and failed-frame samples.
    pub fn protected_constellation(
        &self,
    ) -> Option<&super::protected_constellation::ProtectedConstellation> {
        self.link.protected_constellation.as_ref()
    }
    /// Stops protected diagnostic retention while leaving scientific state usable.
    /// # Returns
    /// Owned bounded capture, or None when disabled/already removed.
    pub fn take_protected_constellation(
        &mut self,
    ) -> Option<super::protected_constellation::ProtectedConstellation> {
        self.link.protected_constellation.take()
    }
    /// Processes at most one complete OFDM period, or the final window-tail operation.
    ///
    /// # Arguments
    /// * `probe` - Optional finite stage/range selector; owns its shared event budget.
    /// * `observer` - Read-only sink for existing waveform and payload stage producers.
    /// # Returns
    /// One bounded completed step, including any actual recovered words. Returns None
    /// after Finished; repeated calls then do no work. Source state persists across calls.
    /// # Errors
    /// Stage/control/observer failures mark the session Failed and preserve completed
    /// summary totals; subsequent calls reject. Use run_channel for the cooperative
    /// cancellation/job facade; a failed transaction is not restartable.
    pub fn advance(
        &mut self,
        probe: &mut TestPointProbe,
        observer: &mut impl ProbeObserver,
    ) -> Result<Option<ChannelStep>, SimulationError> {
        if self.summary.phase == ChannelPhase::Failed {
            return Err(SimulationError::new(
                "ChannelSessionClosed",
                "Start a new session after channel failure.",
            ));
        }
        let outcome = self.advance_inner(probe, observer);
        self.summary.file_source = self.verifier.file_summary(&self.source);
        // Each frame's small iteration list is consumed immediately, never retained per run.
        self.summary.plc_ldpc_iterations += self
            .link
            .stats
            .plc_ldpc_iterations
            .drain(..)
            .map(|iterations| iterations as u64)
            .sum::<u64>();
        self.summary.plc_frames = self.link.stats.plc_frames;
        self.summary.plc_bit_errors = self.link.stats.plc_bit_errors;
        self.summary.maximum_bin_error = self.link.stats.maximum_bin_error;
        self.summary.compared_data_carriers = self.link.stats.compared_data_carriers;
        if self.link.stats.data_signal_power > 0.0 && self.link.stats.data_error_power > 0.0 {
            self.summary.measured_mer_db = Some(
                10.0 * (self.link.stats.data_signal_power / self.link.stats.data_error_power)
                    .log10(),
            );
        }
        if let Err(error) = &outcome {
            self.summary.phase = ChannelPhase::Failed;
            log::debug!(
                "channel-failed code={} completed-source-symbols={}",
                error.code,
                self.summary.observation_symbols
            );
        }
        outcome
    }

    fn advance_inner(
        &mut self,
        probe: &mut TestPointProbe,
        observer: &mut impl ProbeObserver,
    ) -> Result<Option<ChannelStep>, SimulationError> {
        loop {
            match self.summary.phase {
                ChannelPhase::Acquisition => return self.acquire(probe, observer).map(Some),
                ChannelPhase::Observation => {
                    let grid = self
                        .grid
                        .ok_or_else(|| invalid("Missing channel cell grid."))?;
                    let context = self.link.context(RunPhase::Observation);
                    let tx = self
                        .transmitter
                        .as_mut()
                        .ok_or_else(|| invalid("Missing channel TX."))?;
                    while tx.queued_codewords() < QUEUED_PAYLOAD_TARGET {
                        let Some(payload) = self.source.next_payload()? else {
                            break;
                        };
                        let mut source_context = context.clone();
                        source_context.reference =
                            crate::simulation::test_points::SignalReference::NotApplicable;
                        source_context.codeword_index = Some(self.summary.generated_payloads);
                        crate::simulation::source::observation::source_payload(
                            &payload,
                            &source_context,
                            probe,
                            observer,
                        )?;
                        let encoded = tx.enqueue(&payload)?;
                        self.summary.generated_payloads += 1;
                        observation::observe_encoded(&encoded, grid, &context, probe, observer)?;
                    }
                    let transmitted = tx.transmit()?;
                    observation::observe_transmit(&transmitted, grid, &context, probe, observer)?;
                    let step = self.exchange(
                        &transmitted.channel,
                        ChannelPhase::Observation,
                        probe,
                        observer,
                    )?;
                    self.summary.observation_symbols += 1;
                    if self.summary.observation_symbols == self.summary.timing.complete_symbols {
                        self.summary.phase = ChannelPhase::Completion;
                    }
                    return Ok(Some(step));
                }
                ChannelPhase::Completion => {
                    let tx = self
                        .transmitter
                        .as_mut()
                        .ok_or_else(|| invalid("Missing channel TX."))?;
                    if tx.queued_codewords() == 0 {
                        self.summary.phase = ChannelPhase::InterleaverDrain;
                        continue;
                    }
                    if self.summary.completion_symbols >= MAX_COMPLETION_SYMBOLS {
                        return Err(invalid(
                            "Queued payload completion exceeded the work bound.",
                        ));
                    }
                    let transmitted = tx.transmit()?;
                    let grid = self
                        .grid
                        .ok_or_else(|| invalid("Missing channel cell grid."))?;
                    observation::observe_transmit(
                        &transmitted,
                        grid,
                        &self.link.context(RunPhase::Drain),
                        probe,
                        observer,
                    )?;
                    let step = self.exchange(
                        &transmitted.channel,
                        ChannelPhase::Completion,
                        probe,
                        observer,
                    )?;
                    self.summary.completion_symbols += 1;
                    return Ok(Some(step));
                }
                ChannelPhase::InterleaverDrain => {
                    let tx = self
                        .transmitter
                        .as_mut()
                        .ok_or_else(|| invalid("Missing channel TX."))?;
                    if let Some(transmitted) = tx.drain()? {
                        let step = self.exchange(
                            &transmitted,
                            ChannelPhase::InterleaverDrain,
                            probe,
                            observer,
                        )?;
                        self.summary.interleaver_drain_symbols += 1;
                        return Ok(Some(step));
                    }
                    self.summary.waveform_tail_samples = self.link.finish(probe, observer)?;
                    self.summary.total_transmitted_samples +=
                        self.summary.waveform_tail_samples as u64;
                    if self.summary.pending_payload_bits != 0
                        || self.summary.generated_payloads != self.summary.recovered_payloads
                        || self.source.position() != self.summary.comparison.bits_tested
                    {
                        return Err(invalid(
                            "Channel ended without complete generated-payload recovery.",
                        ));
                    }
                    if let Some(window) = &mut self.summary.window {
                        window.finalize(self.summary.timing);
                    }
                    self.summary.phase = ChannelPhase::Finished;
                    log::info!(
                        "channel-finished source-symbols={} compared-bits={} payload-bit-errors={}",
                        self.summary.observation_symbols,
                        self.summary.comparison.bits_tested,
                        self.summary.comparison.bit_errors
                    );
                    return Ok(Some(ChannelStep {
                        phase: ChannelPhase::Finished,
                        channel_symbol_index: None,
                        source_symbol_index: None,
                        recovered: Vec::new(),
                    }));
                }
                ChannelPhase::Finished => return Ok(None),
                ChannelPhase::Failed => return Err(invalid("Channel session is closed.")),
            }
        }
    }

    fn acquire(
        &mut self,
        probe: &mut TestPointProbe,
        observer: &mut impl ProbeObserver,
    ) -> Result<ChannelStep, SimulationError> {
        if self.next_symbol >= MAX_ACQUISITION_SYMBOLS {
            return Err(SimulationError::new(
                "ReceiverNotReady",
                "PLC acquisition exceeded the frame budget.",
            ));
        }
        let index = self.next_symbol;
        let reduced = self.link.acquisition_cells(index);
        self.link
            .transfer(index, &reduced, RunPhase::Warmup, probe, observer)?;
        self.advance_clock();
        self.summary.acquisition_symbols += 1;
        if self.next_symbol % PLC_FRAME_SYMBOLS == 0 && self.link.state().require_ready().is_ok() {
            let received = self.link.state().require_ready()?;
            self.grid = Some(CellProbeGrid {
                settings: received.interleaver_settings()?,
                origin_symbol: self.next_symbol,
                ncp_modulation: received.ncp_modulation(),
            });
            self.receiver = Some(PayloadReceiver::new(self.link.state(), self.next_symbol)?);
            if let Some(collector) = &mut self.constellation {
                collector.start(
                    received,
                    self.next_symbol,
                    self.summary.timing.complete_symbols,
                )?;
            }
            self.transmitter = Some(if let Some(loading) = &self.settings.data_bit_loading {
                PayloadTransmitter::with_profile(
                    &self.settings.transmission,
                    &self.settings.layout,
                    self.settings.ncp_modulation,
                    self.next_symbol,
                    loading,
                )?
            } else {
                PayloadTransmitter::new(
                    &self.settings.transmission,
                    &self.settings.layout,
                    self.settings.ncp_modulation,
                    self.next_symbol,
                )?
            });
            self.summary.window = Some(super::accounting::WindowTotals::new(
                super::accounting::ObservationWindow::new(self.next_symbol, self.summary.timing)?,
            ));
            self.summary.payload_origin_symbol = Some(self.next_symbol);
            self.summary.phase = ChannelPhase::Observation;
        }
        Ok(ChannelStep {
            phase: ChannelPhase::Acquisition,
            channel_symbol_index: Some(index),
            source_symbol_index: None,
            recovered: Vec::new(),
        })
    }

    fn exchange(
        &mut self,
        channel: &PayloadChannelSymbol,
        phase: ChannelPhase,
        probe: &mut TestPointProbe,
        observer: &mut impl ProbeObserver,
    ) -> Result<ChannelStep, SimulationError> {
        let interval = if phase == ChannelPhase::Observation {
            RunPhase::Observation
        } else {
            RunPhase::Drain
        };
        if phase == ChannelPhase::Observation {
            if let Some(epoch) = self
                .settings
                .impairment_timeline
                .get(self.next_impairment_epoch)
            {
                if epoch.first_observation_symbol == self.summary.observation_symbols {
                    self.link.transition(
                        &epoch.impairments,
                        self.settings.llr_variance,
                        self.settings.receiver_mode,
                    )?;
                    self.next_impairment_epoch += 1;
                }
            }
        }
        let received = self.link.transfer(
            channel.channel_symbol_index,
            &channel.reduced,
            interval,
            probe,
            observer,
        )?;
        self.advance_clock();
        let rx = self
            .receiver
            .as_mut()
            .ok_or_else(|| invalid("Missing channel RX."))?;
        let decoded = rx.receive_with_variances(
            self.link.state(),
            received.symbol_index,
            &received.signals.reduced,
            self.link.variances(),
            self.settings.max_iterations,
        )?;
        let mut words = Vec::new();
        if let Some(source) = &decoded.decoded {
            if let Some(control) = &source.ncp {
                if let Some(window) = &mut self.summary.window {
                    if (window.coordinates.first_source_symbol
                        ..window.coordinates.end_source_symbol)
                        .contains(&source.source_symbol_index)
                    {
                        window.announced_starts_within += control
                            .messages
                            .iter()
                            .filter(|message| {
                                !message.zero_loaded
                                    && message.pointer
                                        != crate::transmission::ncp::message::NULL_POINTER
                            })
                            .count()
                            as u64;
                        window.incomplete_starts_within = window
                            .announced_starts_within
                            .saturating_sub(window.completed_starts_within);
                    }
                }
                self.summary.accepted_ncp_fields += 1;
                self.summary.ncp_ldpc_iterations += control
                    .block_iterations
                    .iter()
                    .map(|iterations| *iterations as u64)
                    .sum::<u64>();
            }
            for word in &source.recovered {
                let comparison = self
                    .verifier
                    .compare(&word.decoded.bch.codeword[..BCH_INPUT_BITS])?;
                self.summary.recovered_payloads += 1;
                self.summary.payload_ldpc_failures += u64::from(!word.decoded.ldpc.success);
                self.summary.payload_bch_failures += u64::from(!word.decoded.bch.success);
                self.summary.payload_ldpc_iteration_sum += word.decoded.ldpc.iterations as u64;
                self.summary.payload_mean_ldpc_iterations = Some(
                    self.summary.payload_ldpc_iteration_sum as f64
                        / self.summary.recovered_payloads as f64,
                );
                self.summary.payload_ldpc_iterations[word.decoded.ldpc.iterations] += 1;
                self.summary.bch_corrected_bits += u64::from(word.decoded.bch.corrected_bits);
                if comparison.bit_errors > 0
                    || !word.decoded.ldpc.success
                    || !word.decoded.bch.success
                {
                    self.summary.payload_codeword_errors += 1;
                }
                let window = self
                    .summary
                    .window
                    .as_mut()
                    .ok_or_else(|| invalid("Missing observation window."))?;
                let placement = window
                    .coordinates
                    .classify(word.first_source_symbol, word.last_source_symbol)?;
                let during_drain = phase != ChannelPhase::Observation;
                window.record(
                    placement,
                    comparison.bits_tested,
                    comparison.bit_errors,
                    during_drain,
                );
                words.push(ChannelWordResult {
                    placement,
                    first_source_sample: word.first_source_symbol
                        * u64::from(self.summary.timing.symbol_period_samples),
                    end_source_sample: (word.last_source_symbol + 1)
                        * u64::from(self.summary.timing.symbol_period_samples),
                    recovered_during_drain: during_drain,
                    receive_sequence: word.receive_sequence,
                    first_source_symbol: word.first_source_symbol,
                    last_source_symbol: word.last_source_symbol,
                    comparison,
                    ldpc_success: word.decoded.ldpc.success,
                    ldpc_iterations: word.decoded.ldpc.iterations,
                    bch_success: word.decoded.bch.success,
                    bch_corrected_bits: word.decoded.bch.corrected_bits,
                });
            }
            self.summary.comparison = self.verifier.totals().clone();
            if self.summary.comparison.bits_tested > 0 {
                self.summary.payload_ber = Some(
                    self.summary.comparison.bit_errors as f64
                        / self.summary.comparison.bits_tested as f64,
                );
            }
            self.summary.pending_payload_bits = source.pending_payload_bits;
            if let Some(error) = &source.error {
                return Err(error.clone());
            }
        } else {
            self.summary.rx_warmup_symbols += 1;
        }
        if let Some(collector) = &mut self.constellation {
            collector.observe(&received, &decoded)?;
        }
        let grid = self
            .grid
            .ok_or_else(|| invalid("Missing channel cell grid."))?;
        observation::observe_receive(
            &decoded,
            grid,
            &self.link.context(interval),
            probe,
            observer,
        )?;
        Ok(ChannelStep {
            phase,
            channel_symbol_index: Some(channel.channel_symbol_index),
            source_symbol_index: decoded.source_symbol_index,
            recovered: words,
        })
    }

    fn advance_clock(&mut self) {
        // The validated one-hour plan plus bounded acquisition/completion/drain fits u64.
        self.next_symbol += 1;
        self.summary.total_transmitted_samples +=
            u64::from(self.summary.timing.symbol_period_samples);
    }
}
