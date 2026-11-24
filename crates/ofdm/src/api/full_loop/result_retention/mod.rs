/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Opt-in pre-production reservation; the original engine remains the scientific owner.
mod bound;
pub(in crate::api) mod normalize;
mod output;
use super::*;
use crate::simulation::{
    channel::{job::run_notified_with_cancellation, ChannelJobLimits, ChannelRunSettings},
    test_points::TestPointProbe,
};
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};

struct Frozen {
    settings: ChannelRunSettings,
    inputs: FullLoopInputs,
    policy: Option<FullLoopObservationPolicy>,
}
impl Frozen {
    fn new(request: &FullLoopRequest) -> Self {
        let FullLoopRequest {
            settings,
            options: _,
            inputs,
            observation_policy,
            response_tolerance: _,
            independent_response: _,
        } = request;
        let mut settings = (**settings).clone();
        normalize::settings(&mut settings);
        let mut inputs = inputs.clone();
        let FullLoopInputs {
            transmission: _,
            profile,
            source,
            measurements: _,
            channel,
            response,
        } = &mut inputs;
        profile.normalize_result_storage();
        source.normalize_result_storage();
        channel.normalize_result_storage();
        if let Some(crate::api::ChannelEstimate { frame, provenance }) = response {
            normalize::response(frame);
            provenance.normalize_result_storage();
        }
        let mut policy = observation_policy.clone();
        if let Some(policy) = &mut policy {
            policy.normalize_result_storage();
        }
        Self {
            settings,
            inputs,
            policy,
        }
    }

    fn children(&self) -> Option<RetainedFootprint> {
        let Self {
            settings,
            inputs,
            policy,
        } = self;
        sum([
            settings.retained_children(),
            inputs.retained_children(),
            policy.retained_children(),
        ])
    }
}

impl FullLoopRequest {
    /// Bounds the complete future owned result before producing scientific output.
    ///
    /// Includes the fixed result, repeated immutable shared input graphs, exact
    /// final snapshot storage and every finite report/stage/error payload. The
    /// source request remains independently owned and must be charged separately.
    /// Temporary snapshot builders and engine scratch use existing finite input,
    /// duration, decoder and observation limits; they are not included here.
    /// No source, RNG, control or numerical stage advances.
    ///
    /// # Returns
    /// A conservative target-layout payload/entry/shared-root reservation for
    /// complete, failed or cancelled results from run_bounded on this request.
    /// Separate calls produce the same bound regardless of Clone spare capacity.
    ///
    /// # Errors
    /// Returns ResourceLimit if result_retained_bound arithmetic overflows.
    pub fn result_retained_bound(&self) -> ApiResult<RetainedFootprint> {
        bound::calculate(self, &Frozen::new(self)).ok_or_else(overflow)
    }

    /// Admits the complete future result before starting the existing engine.
    ///
    /// # Arguments
    /// * `control` - Fresh one-shot control, including an already requested cancel.
    /// * `limit` - Pre-reserved payload bytes, ordered entries and shared roots.
    ///
    /// # Returns
    /// The unchanged scientific complete/partial result with exact final owned
    /// capacities. Completed trials, original inputs and all query data survive.
    /// The caller retains responsibility for simultaneous request/result/queued
    /// ownership; this operation does not reserve process or scratch memory.
    ///
    /// # Errors
    /// Returns ResourceLimit before control admission if any dimension is too
    /// small or overflows. Preserves Busy and original construction errors.
    /// Scientific step failures remain real partial reports, never post-run
    /// ResourceLimit errors that discard completed outcomes.
    pub fn run_bounded(
        &self,
        control: &FullLoopControl,
        limit: RetainedFootprint,
    ) -> ApiResult<FullLoopResult> {
        let frozen = Frozen::new(self);
        let reservation = bound::calculate(self, &frozen).ok_or_else(overflow)?;
        if !fits(reservation, limit) {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "result_retained_bound exceeds reserved limit",
            ));
        }
        let mut probe = frozen
            .policy
            .as_ref()
            .map_or_else(|| Ok(TestPointProbe::disabled()), |policy| policy.probe())?;
        let mut stages = observations::RetainedStages::new(
            frozen
                .policy
                .as_ref()
                .map_or(0, |policy| policy.settings().max_retained_bytes),
        );
        control.begin()?;
        let limits = ChannelJobLimits {
            max_runtime_ms: self.options.max_runtime_ms,
            max_retained_words: self.options.max_retained_words,
        };
        let job = run_notified_with_cancellation(
            &self.settings,
            &limits,
            &control.stop(),
            control.pause_gate(),
            &mut probe,
            &mut stages,
            |progress| control.progress(progress),
            |paused| control.acknowledge_pause(paused),
        );
        control.finish(job.as_ref().ok());
        let mut job = job?;
        job.settings = frozen.settings;
        output::normalize(&mut job);
        stages.normalize_result_storage();
        let result = FullLoopResult {
            job: Arc::new(job),
            inputs: frozen.inputs,
            stages: Arc::new(stages),
            observation_policy: frozen.policy,
        };
        debug_assert!(
            result
                .retained_footprint()
                .is_ok_and(|actual| fits(actual, reservation)),
            "audited final result layout exceeds its pre-production reservation"
        );
        Ok(result)
    }
}

fn overflow() -> ApiError {
    ApiError::new(ErrorCode::ResourceLimit, "result_retained_bound overflow")
}

fn fits(value: RetainedFootprint, limit: RetainedFootprint) -> bool {
    value.payload_bytes <= limit.payload_bytes
        && value.ordered_entries <= limit.ordered_entries
        && value.shared_roots <= limit.shared_roots
}

#[cfg(test)]
mod tests;
