/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Whole-job admission, separate from normative per-stage geometry.
use crate::{
    configuration::{ReceiverConfig, UsConfiguration, MAX_JOB_SAMPLES},
    error::{invalid, limit},
    grant::{select, GrantPlan},
    resources::ResourcePlan,
    signal_processing::PreEqBank,
    source::Prbs15,
    types::Iq,
    PhyResult,
};
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use serde::Serialize;

/// Frozen source/configuration/receiver request; normal RX sees only its views.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ExperimentConfig {
    /// Accepted synthetic channel/profile/grant settings.
    pub us_configuration: UsConfiguration,
    /// Nonzero continuous PRBS15 payload seed; never passed to normal RX.
    pub source_seed: u16,
    /// Transmitter-only static corrective bank, not a channel estimate.
    pub pre_eq: PreEqBank,
    /// Explicit synthetic per-symbol replay, independently preflighted in full.
    pub pre_eq_updates: Option<crate::measurements::PreEqPlayback>,
    /// Explicit synthetic equalized-probe-reference residual target selection.
    pub rxmer: Option<crate::measurements::RxmerPlayback>,
    /// Explicit configured timing/estimator/reference-confidence policies.
    pub receiver: ReceiverConfig,
    /// Separate synthetic H/noise/timing configuration, never supplied to RX.
    pub channel: crate::channel::ChannelConfig,
    /// Explicit observation silence outside prefix and terminal tail support.
    pub guards: crate::configuration::GuardIntervals,
}

impl ExperimentConfig {
    /// Composes accepted `us_configuration`, `source_seed` and `receiver` policies.
    ///
    /// Returns a frozen synthetic request with normative unity Pre-EQ defaults.
    /// Rejects invalid source seed without beginning any processing.
    pub fn new(
        us_configuration: UsConfiguration,
        source_seed: u16,
        receiver: ReceiverConfig,
    ) -> PhyResult<Self> {
        Prbs15::new(source_seed)?;
        let pre_eq = PreEqBank::unity(&us_configuration);
        Ok(Self {
            us_configuration,
            source_seed,
            pre_eq,
            pre_eq_updates: None,
            rxmer: None,
            receiver,
            channel: crate::channel::ChannelConfig::default(),
            guards: Default::default(),
        })
    }
    /// Validates all mutable public candidate fields before accepting a request.
    ///
    /// Returns unit; rejects invalid source, bank reset identity or resource plan.
    /// Neither the source nor a receiver is started by admission.
    pub fn validate(&self) -> PhyResult<()> {
        self.channel.validate()?;
        self.guards
            .validate(self.us_configuration.grant().epoch_sample)?;
        if let Some(plan) = &self.pre_eq_updates {
            plan.validate(&self.us_configuration, &self.pre_eq)?;
        }
        if let Some(playback) = &self.rxmer {
            playback.validate_model(&self.us_configuration)?;
            if self.channel.taps != [Iq { re: 1.0, im: 0.0 }]
                || self.channel.noise_variance != 0.0
                || self.channel.delay_samples != 0
                || self.channel.phase_radians != 0.0
                || self.channel.frequency_offset_hz != 0.0
                || !self.pre_eq.is_unity()
                || self
                    .pre_eq_updates
                    .as_ref()
                    .is_some_and(|plan| !plan.all_unity())
            {
                return Err(crate::PhyError::Unsupported {
                    field: "rxmer.channel_composition",
                });
            }
        }
        Prbs15::new(self.source_seed)?;
        ResourcePlan::new(&self.us_configuration)?;
        self.pre_eq.apply(
            &self.us_configuration,
            &vec![Iq::default(); self.us_configuration.waveform().numerology().fft_len()],
        )?;
        Ok(())
    }
}

/// Finite caller budgets; maxima are application policy, not modem requirements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExperimentOptions {
    /// Explicit sequential grants; never a capture Cartesian product.
    pub grants: usize,
    /// Cumulative emitted sample budget, at most 16,777,216.
    pub maximum_samples: usize,
    /// External transport split, 1..65536 samples, independent of symbols.
    pub chunk_samples: usize,
    /// Monotonic runtime, 1..300000 milliseconds.
    pub timeout_ms: u64,
    /// Total retained passive I/Q observations across all taps, 0..65536.
    pub retained_points: usize,
    /// Explicit selected test points; history for other taps remains unavailable.
    pub retained_taps: Vec<crate::instruments::Tap>,
}

impl Default for ExperimentOptions {
    /// Supplies one grant, 4096-sample chunks, 30 seconds and bounded observations.
    ///
    /// Returned settings do not execute work or implicitly load configuration.
    fn default() -> Self {
        Self {
            grants: 1,
            maximum_samples: MAX_JOB_SAMPLES,
            chunk_samples: 4096,
            timeout_ms: 30_000,
            retained_points: 4096,
            retained_taps: vec![
                crate::instruments::Tap::TxOutput,
                crate::instruments::Tap::RxEqualized,
            ],
        }
    }
}

impl ExperimentOptions {
    /// Checks caller-controlled budgets before allocation or run admission.
    ///
    /// Returns unit; rejects zero/oversized counts and unsupported durations.
    pub fn validate(&self) -> PhyResult<()> {
        crate::instruments::ObservationStore::new(self.retained_points, &self.retained_taps)?;
        if !(1..=256).contains(&self.grants)
            || !(1..=MAX_JOB_SAMPLES).contains(&self.maximum_samples)
            || !(1..=65536).contains(&self.chunk_samples)
            || !(1..=300_000).contains(&self.timeout_ms)
            || self.retained_points > 65536
        {
            return Err(limit("experiment.options"));
        }
        Ok(())
    }
}

/// Pure admission/readiness snapshot, not a completed experiment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExperimentPreview {
    /// Shared complete single-grant FEC/bit-role layout.
    pub grant_plan: GrantPlan,
    /// Finite cumulative emitted waveform samples including tails.
    pub scheduled_samples: usize,
    /// Received support after synthetic causal response/delay drain.
    pub received_samples: usize,
    /// Cumulative meaningful PRBS source positions planned across grants.
    pub source_bits: usize,
    /// Cumulative codeword trials, excluding no-transmission grants.
    pub codewords: usize,
    /// All retained profile identifiers, without hardcoded catalogs.
    pub profile_ids: Vec<u16>,
}

/// Performs aggregate frozen-request admission without source/RNG progress.
///
/// `config` and `options` are borrowed unchanged. Returns pure counts/inventory;
/// rejects overflow, sample/result/numerical-work limits and invalid settings.
pub fn preview(
    config: &ExperimentConfig,
    options: &ExperimentOptions,
) -> PhyResult<ExperimentPreview> {
    config.validate()?;
    options.validate()?;
    let us = &config.us_configuration;
    let resources = ResourcePlan::new(us)?;
    let plan = select(resources.grant_bits)?;
    let support = if plan.transmit {
        us.grant().frames * us.waveform().frame_symbols() * us.waveform().stride()
            + us.waveform().rolloff()
            + config.guards.validate(us.grant().epoch_sample)?
    } else {
        0
    };
    let samples = support
        .checked_mul(options.grants)
        .ok_or_else(|| limit("experiment.samples"))?;
    let received = samples
        .checked_add(if plan.transmit {
            config
                .channel
                .extra_samples()?
                .checked_mul(options.grants)
                .ok_or_else(|| limit("experiment.channel_tail"))?
        } else {
            0
        })
        .ok_or_else(|| limit("experiment.received_samples"))?;
    if received > options.maximum_samples {
        return Err(limit("experiment.samples"));
    }
    (us.grant().epoch_sample - config.guards.leading as u64)
        .checked_add(received as u64)
        .ok_or_else(|| limit("experiment.end"))?;
    us.grant()
        .id
        .checked_add(options.grants as u64)
        .ok_or_else(|| limit("experiment.grant_ids"))?;
    us.grant()
        .first_frame
        .checked_add((us.grant().frames * options.grants) as u64)
        .ok_or_else(|| limit("experiment.frame_ids"))?;
    let retained: usize = plan
        .codewords
        .iter()
        .map(|word| word.code.parameters().codeword_bits + word.information_bits * 2)
        .sum();
    if retained
        .checked_mul(options.grants)
        .is_none_or(|count| count > 16_777_216)
    {
        return Err(limit("experiment.result_decisions"));
    }
    let work = resources
        .visits
        .iter()
        .filter(|cell| cell.loading > 0)
        .try_fold(0_usize, |sum, cell| {
            sum.checked_add((1_usize << cell.loading) * (cell.loading as usize + 1))
        })
        .ok_or_else(|| limit("experiment.demapper_work"))?;
    if work
        .checked_mul(us.grant().frames)
        .and_then(|v| v.checked_mul(options.grants))
        .is_none_or(|count| count > 268_435_456)
    {
        return Err(limit("experiment.demapper_work"));
    }
    if options.grants == 0 {
        return Err(invalid("experiment.grants"));
    }
    Ok(ExperimentPreview {
        scheduled_samples: samples,
        received_samples: received,
        source_bits: plan.source_bits * options.grants,
        codewords: plan.codewords.len() * options.grants,
        profile_ids: us.profiles().iter().map(|profile| profile.id()).collect(),
        grant_plan: plan,
    })
}

impl RetainedPayload for ExperimentConfig {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            us_configuration,
            source_seed,
            pre_eq,
            pre_eq_updates,
            rxmer,
            receiver,
            channel,
            guards,
        } = self;
        sum([
            us_configuration.retained_children(),
            source_seed.retained_children(),
            pre_eq.retained_children(),
            pre_eq_updates.retained_children(),
            rxmer.retained_children(),
            receiver.retained_children(),
            channel.retained_children(),
            guards.retained_children(),
        ])
    }
}

impl RetainedPayload for ExperimentOptions {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            grants,
            maximum_samples,
            chunk_samples,
            timeout_ms,
            retained_points,
            retained_taps,
        } = self;
        sum([
            grants.retained_children(),
            maximum_samples.retained_children(),
            chunk_samples.retained_children(),
            timeout_ms.retained_children(),
            retained_points.retained_children(),
            retained_taps.retained_children(),
        ])
    }
}

impl RetainedPayload for ExperimentPreview {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            grant_plan,
            scheduled_samples,
            received_samples,
            source_bits,
            codewords,
            profile_ids,
        } = self;
        sum([
            grant_plan.retained_children(),
            scheduled_samples.retained_children(),
            received_samples.retained_children(),
            source_bits.retained_children(),
            codewords.retained_children(),
            profile_ids.retained_children(),
        ])
    }
}

impl ExperimentConfig {
    /// Normalizes only owned report-snapshot capacities without changing values.
    ///
    /// # Returns
    /// Unit; exact boxed-slice conversion preserves order and every scientific value.
    pub(crate) fn compact_report_payload(&mut self) {
        let Self {
            us_configuration,
            source_seed: _,
            pre_eq,
            pre_eq_updates,
            rxmer,
            receiver: _,
            channel,
            guards: _,
        } = self;
        us_configuration.compact_report_payload();
        pre_eq.compact_report_payload();
        if let Some(playback) = pre_eq_updates {
            playback.compact_report_payload();
        }
        if let Some(playback) = rxmer {
            playback.compact_report_payload();
        }
        let crate::channel::ChannelConfig {
            taps,
            noise_variance: _,
            noise_seed: _,
            delay_samples: _,
            phase_radians: _,
            frequency_offset_hz: _,
        } = channel;
        *taps = std::mem::take(taps).into_boxed_slice().into_vec();
    }
}
