/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! One Rust engine; source truth exists only in this independent observer branch.
use super::{
    request::preview, BatchReport, ExperimentConfig, ExperimentOptions, GrantReport,
    ProgressSnapshot, RunControl, RunFailure, RunStatus,
};
use crate::{
    channel::{Channel, RxmerChannel},
    error::limit,
    instruments::{ObservationStore, Tap},
    receiver::{DecodedWord, Receiver},
    resources::ResourcePlan,
    source::{Prbs15, PrbsVerifier},
    transmitter::Transmitter,
    types::{BitRange, Identity, SampleBlock},
    PhyResult,
};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

/// Runs a previously admissible frozen request through actual TX/channel/RX.
///
/// `run_id`/`generation` bind results; `control` is a shared cooperative handle.
/// Returns owned completed/partial science, including on cancellation/runtime
/// failure. Invalid aggregate admission or a busy control rejects before work.
pub fn run(
    config: &ExperimentConfig,
    options: &ExperimentOptions,
    run_id: u64,
    generation: u64,
    control: &RunControl,
) -> PhyResult<BatchReport> {
    let planned = preview(config, options)?;
    Identity {
        run: run_id,
        generation,
        grant: config.us_configuration.grant().id,
    }
    .validate()?;
    let _lease = control.claim()?;
    let started = Instant::now();
    let timeout = Duration::from_millis(options.timeout_ms);
    let mut source = Prbs15::new(config.source_seed)?;
    let mut verifier = PrbsVerifier::new(config.source_seed)?;
    let mut report = BatchReport {
        format: "merlin.upstream.experiment",
        schema_version: 1,
        source_configuration_json: None,
        source_configuration_original: None,
        source_configuration_sha256: None,
        measurement_provenance_json: None,
        noise_generator:
            "chacha8-standardnormal-v1; rand=0.9.5; rand_chacha=0.9.0; rand_distr=0.5.1",
        configuration: config.clone(),
        options: options.clone(),
        planned,
        status: RunStatus::Running,
        grants: Vec::new(),
        not_started_grants: Vec::new(),
        generated_source_bits: 0,
        compared_source_bits: 0,
        unverified_source_bits: 0,
        bit_errors: 0,
        ber: None,
        reference_recovered: false,
        failure: None,
        elapsed_ms: 0,
        observations: ObservationStore::new(options.retained_points, &options.retained_taps)?,
    };
    publish(control, &report, run_id, started);
    let mut next_epoch =
        config.us_configuration.grant().epoch_sample - config.guards.leading as u64;
    for ordinal in 0..options.grants {
        if let Some(status) = stop_status(control, started, timeout) {
            report.status = status;
            break;
        }
        let mut grant = config.us_configuration.grant().clone();
        grant.id += ordinal as u64;
        grant.first_frame += (ordinal * grant.frames) as u64;
        grant.epoch_sample = next_epoch + config.guards.leading as u64;
        let us = Arc::new(config.us_configuration.with_grant(grant)?);
        let identity = Identity {
            run: run_id,
            generation,
            grant: us.grant().id,
        };
        let mut tx = Transmitter::new(us.clone(), identity, &mut source, config.pre_eq.clone())?;
        tx.set_guards(config.guards)?;
        if let Some(updates) = &config.pre_eq_updates {
            tx.set_pre_eq_updates(updates.clone())?;
        }
        let source_range = tx.source_range();
        let mut outcome = GrantReport {
            identity,
            generated_source: source_range,
            emitted_samples: 0,
            received_samples: 0,
            compared_bits: 0,
            unverified_source_bits: source_range.length(),
            planned_transmit_samples: if tx.plan().transmit {
                us.grant().frames * us.waveform().frame_symbols() * us.waveform().stride()
                    + us.waveform().rolloff()
                    + config.guards.leading
                    + config.guards.trailing
            } else {
                0
            },
            pending_transmit_samples: 0,
            unprocessed_codewords: tx.plan().codewords.len(),
            bit_errors: 0,
            words: Vec::new(),
            completed: false,
            missing_symbols: 0,
            matched_data_mer: Default::default(),
            matched_data_mer_value: crate::instruments::MerValue::Unavailable,
        };
        let mut channel = match &config.rxmer {
            Some(playback) if tx.plan().transmit => {
                RunChannel::Residual(Box::new(RxmerChannel::with_guards(
                    us.clone(),
                    playback.clone(),
                    identity,
                    config.channel.noise_seed,
                    config.guards,
                )?))
            }
            _ => RunChannel::Samples(Box::new(Channel::new(
                config.channel.clone(),
                identity,
                next_epoch,
            )?)),
        };
        let channel_support = if tx.plan().transmit {
            us.grant().frames * us.waveform().frame_symbols() * us.waveform().stride()
                + us.waveform().rolloff()
                + config.channel.extra_samples()?
                + config.guards.leading
                + config.guards.trailing
        } else {
            0
        };
        let mut receiver = Receiver::for_observation(
            us.clone(),
            identity,
            config.receiver.clone(),
            next_epoch,
            next_epoch + channel_support as u64,
        )?;
        let resources = ResourcePlan::new(&us)?;
        let mut observed_frames = 0;
        report.generated_source_bits += source_range.length();
        let mut terminal = None;
        let processing = (|| -> PhyResult<()> {
            loop {
                if let Some(status) = stop_status(control, started, timeout) {
                    terminal = Some(status);
                    break;
                }
                let Some(frame) = tx.next_frame()? else {
                    break;
                };
                report.observations.record_grid(
                    Tap::TxGrid,
                    identity,
                    frame.frame,
                    &frame.grid,
                    &resources,
                );
                report.observations.record_grid(
                    Tap::TxPreEq,
                    identity,
                    frame.frame,
                    &frame.corrected_grid,
                    &resources,
                );
                report.observations.record_samples(
                    Tap::TxUseful,
                    &SampleBlock {
                        identity,
                        start: frame.useful_origin,
                        samples: frame.useful.clone(),
                    },
                );
                let quantum = options
                    .chunk_samples
                    .min(1_048_576 / config.channel.taps.len());
                for (part_index, samples) in frame.block.samples.chunks(quantum).enumerate() {
                    if let Some(status) = stop_status(control, started, timeout) {
                        terminal = Some(status);
                        break;
                    }
                    let block = SampleBlock {
                        identity,
                        start: frame.block.start + (part_index * quantum) as u64,
                        samples: samples.to_vec(),
                    };
                    let received = channel.push(&block)?;
                    outcome.received_samples += received.samples.len();
                    report.observations.record_samples(Tap::TxOutput, &block);
                    report
                        .observations
                        .record_samples(Tap::ChannelInput, &block);
                    report
                        .observations
                        .record_samples(Tap::ChannelOutput, &received);
                    report.observations.record_samples(Tap::RxInput, &received);
                    outcome.emitted_samples += samples.len();
                    let batch = receiver.push_with_control(&received, &mut || {
                        stop_status(control, started, timeout).is_some()
                    })?;
                    compare_words(batch.words, source_range, &mut verifier, &mut outcome)?;
                    if receiver.frames() > observed_frames {
                        let frame_id = us.grant().first_frame + receiver.frames() as u64 - 1;
                        if let Some(grid) = receiver.last_grid() {
                            report.observations.record_grid(
                                Tap::RxFft,
                                identity,
                                frame_id,
                                grid,
                                &resources,
                            );
                        }
                        for (cell, value) in receiver.last_payload_cells() {
                            if let Some(received) = value {
                                let ideal = frame.reference_grid[cell.symbol
                                    * us.waveform().numerology().fft_len()
                                    + cell.carrier]
                                    * cell.gain.recip();
                                outcome.matched_data_mer.observe(ideal, *received)?;
                            }
                            if let Some(value) = value {
                                report.observations.record_cell(
                                    Tap::RxEqualized,
                                    identity,
                                    frame_id,
                                    cell,
                                    *value,
                                );
                            }
                        }
                        observed_frames = receiver.frames();
                    }
                    if batch.interrupted {
                        terminal = stop_status(control, started, timeout);
                    }
                    publish_partial(control, &report, &outcome, run_id, started);
                    if terminal.is_some() {
                        break;
                    }
                }
                if terminal.is_some() {
                    break;
                }
            }
            if terminal.is_none() {
                let tail = channel.finish()?;
                outcome.received_samples += tail.samples.len();
                if !tail.samples.is_empty() {
                    report
                        .observations
                        .record_samples(Tap::ChannelOutput, &tail);
                    report.observations.record_samples(Tap::RxInput, &tail);
                    let batch = receiver.push_with_control(&tail, &mut || {
                        stop_status(control, started, timeout).is_some()
                    })?;
                    compare_words(batch.words, source_range, &mut verifier, &mut outcome)?;
                    if batch.interrupted {
                        terminal = stop_status(control, started, timeout);
                    }
                }
            }
            if terminal.is_none() {
                compare_words(
                    receiver.finish()?,
                    source_range,
                    &mut verifier,
                    &mut outcome,
                )?;
                outcome.completed = true;
            }
            Ok(())
        })();
        outcome.unverified_source_bits = source_range.length() - outcome.compared_bits;
        outcome.pending_transmit_samples =
            outcome.planned_transmit_samples - outcome.emitted_samples;
        outcome.unprocessed_codewords = tx.plan().codewords.len() - outcome.words.len();
        outcome.missing_symbols = receiver.missing_symbols();
        outcome.matched_data_mer_value = outcome.matched_data_mer.value();
        report.compared_source_bits += outcome.compared_bits;
        report.bit_errors += outcome.bit_errors;
        next_epoch = next_epoch
            .checked_add(outcome.received_samples as u64)
            .ok_or_else(|| limit("run.epoch"))?;
        report.grants.push(outcome);
        if let Err(error) = processing {
            report.status = RunStatus::Failed;
            let (category, field) = match &error {
                crate::PhyError::InvalidInput { field } => ("InvalidInput", Some(*field)),
                crate::PhyError::Unsupported { field } => ("UnsupportedCapability", Some(*field)),
                crate::PhyError::ResourceLimit { field } => ("ResourceLimit", Some(*field)),
                crate::PhyError::Association { field } => ("Association", Some(*field)),
                crate::PhyError::Codec { .. } => ("Codec", None),
            };
            report.failure = Some(RunFailure {
                diagnostic: error.to_string(),
                category,
                field,
            });
            break;
        }
        if let Some(status) = terminal {
            report.status = status;
            break;
        }
        publish(control, &report, run_id, started);
    }
    if report.status == RunStatus::Running {
        report.status = RunStatus::Completed;
    }
    for ordinal in report.grants.len()..options.grants {
        report
            .not_started_grants
            .push(config.us_configuration.grant().id + ordinal as u64);
    }
    report.unverified_source_bits = report.generated_source_bits - report.compared_source_bits;
    report.ber = if report.compared_source_bits == 0 {
        None
    } else {
        Some(report.bit_errors as f64 / report.compared_source_bits as f64)
    };
    report.reference_recovered = report.status == RunStatus::Completed
        && report.generated_source_bits > 0
        && report.compared_source_bits == report.generated_source_bits
        && report.bit_errors == 0
        && report.grants.iter().all(|grant| {
            grant.completed
                && grant.missing_symbols == 0
                && grant.words.iter().all(|word| {
                    word.decoded.converged
                        && word.decoded.parity_valid
                        && word.decoded.shortened_zeros_valid
                        && word.information_fill_valid
                })
        });
    report.elapsed_ms = control
        .elapsed(started)
        .as_millis()
        .min(u128::from(u64::MAX)) as u64;
    publish(control, &report, run_id, started);
    Ok(report)
}

fn compare_words(
    words: Vec<DecodedWord>,
    range: BitRange,
    verifier: &mut PrbsVerifier,
    outcome: &mut GrantReport,
) -> PhyResult<()> {
    for word in words {
        if word.payload_offset != outcome.compared_bits {
            return Err(crate::PhyError::Association {
                field: "source_ledger.coverage",
            });
        }
        let comparison =
            verifier.compare(range.start() + word.payload_offset as u64, &word.payload)?;
        outcome.compared_bits += word.payload.len();
        outcome.bit_errors += comparison.bit_errors;
        outcome.words.push(word);
    }
    Ok(())
}

fn stop_status(control: &RunControl, started: Instant, timeout: Duration) -> Option<RunStatus> {
    control.checkpoint();
    if control.is_cancelled() {
        Some(RunStatus::Cancelled)
    } else if control.elapsed(started) >= timeout {
        Some(RunStatus::TimeLimit)
    } else {
        None
    }
}

fn publish(control: &RunControl, report: &BatchReport, run_id: u64, started: Instant) {
    control.publish(ProgressSnapshot {
        status: report.status,
        run: Some(run_id),
        completed_grants: report.grants.iter().filter(|grant| grant.completed).count(),
        emitted_samples: report
            .grants
            .iter()
            .map(|grant| grant.emitted_samples)
            .sum(),
        compared_bits: report.compared_source_bits,
        elapsed_ms: control
            .elapsed(started)
            .as_millis()
            .min(u128::from(u64::MAX)) as u64,
    });
}

fn publish_partial(
    control: &RunControl,
    report: &BatchReport,
    outcome: &GrantReport,
    run_id: u64,
    started: Instant,
) {
    control.publish(ProgressSnapshot {
        status: RunStatus::Running,
        run: Some(run_id),
        completed_grants: report.grants.iter().filter(|grant| grant.completed).count(),
        emitted_samples: report
            .grants
            .iter()
            .map(|grant| grant.emitted_samples)
            .sum::<usize>()
            + outcome.emitted_samples,
        compared_bits: report.compared_source_bits + outcome.compared_bits,
        elapsed_ms: control
            .elapsed(started)
            .as_millis()
            .min(u128::from(u64::MAX)) as u64,
    });
}

// Dispatch retains one scientific owner per model; RX sees only returned samples.
enum RunChannel {
    Samples(Box<Channel>),
    Residual(Box<RxmerChannel>),
}
impl RunChannel {
    fn push(&mut self, block: &SampleBlock) -> PhyResult<SampleBlock> {
        match self {
            Self::Samples(model) => model.push(block),
            Self::Residual(model) => model.push(block),
        }
    }
    fn finish(&mut self) -> PhyResult<SampleBlock> {
        match self {
            Self::Samples(model) => model.finish(),
            Self::Residual(model) => model.finish(),
        }
    }
}
