/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Cooperative bounded job lifecycle around the independent channel stepper.
use super::{ChannelRun, ChannelRunSettings, ChannelSummary, ChannelWordResult};
use crate::{
    error::invalid,
    simulation::test_points::{ProbeObserver, ProbeStats, TestPointProbe},
    LabError,
};
use serde::{Deserialize, Serialize};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};
/// Maximum retained detailed comparisons; scalar scientific totals always continue.
pub const MAX_RETAINED_WORDS: usize = 10_000;
/// Application host-runtime ceiling, checked between bounded OFDM steps.
pub const MAX_JOB_RUNTIME_MS: u64 = 300_000;
/// Work/output policy independent of the simulated observation duration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ChannelJobLimits {
    pub max_runtime_ms: u64,
    pub max_retained_words: usize,
}
impl Default for ChannelJobLimits {
    /// Selects a five-minute host budget and64 retained detailed word comparisons.
    /// # Returns
    /// Finite defaults; simulation time remains a separate channel setting.
    fn default() -> Self {
        Self {
            max_runtime_ms: MAX_JOB_RUNTIME_MS,
            max_retained_words: 64,
        }
    }
}
impl ChannelJobLimits {
    /// Checks finite host-work and retained-detail policy before opening outputs.
    /// # Returns
    /// Unit for a supported runtime and memory budget; zero retained words is valid.
    /// # Errors
    /// Rejects zero/excess runtime or more than MAX_RETAINED_WORDS details.
    pub fn validate(&self) -> Result<(), LabError> {
        if !(1..=MAX_JOB_RUNTIME_MS).contains(&self.max_runtime_ms)
            || self.max_retained_words > MAX_RETAINED_WORDS
        {
            return Err(invalid("Invalid channel runtime or retained-word budget."));
        }
        Ok(())
    }
}
/// Job completion status, separate from scientific BER and stepper phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelJobStatus {
    Complete,
    Cancelled,
    RuntimeExceeded,
    Failed,
}
/// Small notification; callback latency counts toward the host-runtime budget.
#[derive(Debug, Clone, Serialize)]
pub struct ChannelProgress {
    pub phase: super::ChannelPhase,
    pub completed_source_symbols: u64,
    pub requested_source_symbols: u64,
    pub recovered_payloads: u64,
    pub elapsed_ms: u64,
}
/// Shared typed result for complete and partial runs; no calculations belong in adapters.
#[derive(Serialize)]
pub struct ChannelJobResult {
    pub schema_version: u16,
    pub status: ChannelJobStatus,
    pub elapsed_ms: u64,
    pub settings: ChannelRunSettings,
    pub limits: ChannelJobLimits,
    pub summary: ChannelSummary,
    pub retained_words: Vec<ChannelWordResult>,
    pub omitted_word_details: u64,
    pub unrecovered_generated_payloads: u64,
    pub probe_stats: ProbeStats,
    pub error: Option<LabError>,
}
/// Runs the noiseless channel with cooperative cancellation and finite host/output budgets.
/// # Arguments
/// * `settings` - Frozen scientific settings; simulated time and host budget are distinct.
/// * `limits` - Runtime and detailed-word retention bounds; zero retained words is valid.
/// * `cancel` - Caller-owned token checked before every OFDM step, including acquisition/drain.
/// * `probe` - Shared finite optional observation selection.
/// * `observer` - Read-only sink; a sink failure returns a Failed result with completed totals.
/// * `progress` - Small notification after each completed step; does not receive mutable stages.
/// # Returns
/// Complete, Cancelled, RuntimeExceeded or Failed result preserving completed comparisons.
/// No comparison is fabricated for unfinished codewords. Wall time can exceed the budget
/// by one bounded step plus callback/constructor latency; calls cannot be interrupted mid-step.
/// # Errors
/// Rejects invalid settings/limits or construction failures before a job starts.
pub fn run_channel(
    settings: &ChannelRunSettings,
    limits: &ChannelJobLimits,
    cancel: &AtomicBool,
    probe: &mut TestPointProbe,
    observer: &mut impl ProbeObserver,
    progress: impl FnMut(ChannelProgress),
) -> Result<ChannelJobResult, LabError> {
    let started = Instant::now();
    run_with_clock(settings, limits, cancel, probe, observer, progress, || {
        started.elapsed().as_millis().min(u128::from(u64::MAX)) as u64
    })
}
fn run_with_clock(
    settings: &ChannelRunSettings,
    limits: &ChannelJobLimits,
    cancel: &AtomicBool,
    probe: &mut TestPointProbe,
    observer: &mut impl ProbeObserver,
    mut progress: impl FnMut(ChannelProgress),
    mut elapsed: impl FnMut() -> u64,
) -> Result<ChannelJobResult, LabError> {
    limits.validate()?;
    let mut session = ChannelRun::new(settings)?;
    let mut retained_words = Vec::new();
    let mut omitted_word_details = 0;
    let mut error = None;
    let status = loop {
        if cancel.load(Ordering::Relaxed) {
            break ChannelJobStatus::Cancelled;
        }
        if elapsed() >= limits.max_runtime_ms {
            break ChannelJobStatus::RuntimeExceeded;
        }
        match session.advance(probe, observer) {
            Ok(Some(step)) => {
                for word in step.recovered {
                    if retained_words.len() < limits.max_retained_words {
                        retained_words.push(word);
                    } else {
                        omitted_word_details += 1;
                    }
                }
                let summary = session.summary();
                progress(ChannelProgress {
                    phase: summary.phase,
                    completed_source_symbols: summary.observation_symbols,
                    requested_source_symbols: summary.timing.complete_symbols,
                    recovered_payloads: summary.recovered_payloads,
                    elapsed_ms: elapsed(),
                });
                if summary.phase == super::ChannelPhase::Finished {
                    break ChannelJobStatus::Complete;
                }
            }
            Ok(None) => break ChannelJobStatus::Complete,
            Err(failure) => {
                error = Some(failure);
                break ChannelJobStatus::Failed;
            }
        }
    };
    if status == ChannelJobStatus::RuntimeExceeded {
        log::warn!(
            "channel-runtime-exceeded completed-source-symbols={}",
            session.summary().observation_symbols
        );
    }
    // A failed observation can follow scientific recovery in the failed step. Those
    // comparisons stay in summary even though no successful step detail was returned.
    omitted_word_details = omitted_word_details.max(
        session
            .summary()
            .recovered_payloads
            .saturating_sub(retained_words.len() as u64),
    );
    Ok(ChannelJobResult {
        schema_version: 1,
        status,
        elapsed_ms: elapsed(),
        settings: settings.clone(),
        limits: limits.clone(),
        summary: session.summary().clone(),
        retained_words,
        omitted_word_details,
        unrecovered_generated_payloads: session
            .summary()
            .generated_payloads
            .saturating_sub(session.summary().recovered_payloads),
        probe_stats: probe.stats(),
        error,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic_host_budget_returns_partial_without_fake_ber() {
        let transmission = crate::transmission::TransmissionSettings::default();
        let layout = crate::transmission::layout::CarrierLayoutSettings {
            zero_frequency_hz: 100_000_000,
            first_active_bin: 320,
            active_end_exclusive: 1280,
            plc_first_bin: 752,
            excluded_bins: vec![1200],
            additional_continuous_pilots: vec![352, 448, 544, 920, 1008, 1080, 1176, 1256],
        };
        let settings = ChannelRunSettings::new(transmission, layout);
        let result = run_with_clock(
            &settings,
            &ChannelJobLimits {
                max_runtime_ms: 1,
                max_retained_words: 0,
            },
            &AtomicBool::new(false),
            &mut TestPointProbe::disabled(),
            &mut crate::simulation::test_points::NoopObserver,
            |_| {},
            || 2,
        )
        .unwrap();
        assert_eq!(result.status, ChannelJobStatus::RuntimeExceeded);
        assert_eq!(result.summary.comparison.bits_tested, 0);
        assert_eq!(result.summary.payload_ber, None);
    }
}
