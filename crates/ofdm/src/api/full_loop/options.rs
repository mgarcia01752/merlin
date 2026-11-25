/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit bounded execution policies; scientific parameter types retain existing owners.
use super::*;
pub use crate::signal_processing::receiver::channel_estimation::MceSettings;
use crate::simulation::channel::config::MAX_CHANNEL_DECODER_ITERATIONS;
pub use crate::simulation::channel::{
    constellation::ConstellationSelection, protected_constellation::ProtectedCaptureSelection,
};
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
use serde::{Deserialize, Serialize};

/// Transport bound for full-loop options, separate from original input/resource limits.
pub const MAX_FULL_LOOP_OPTIONS_BYTES: usize = 65536;
/// Application full-loop observation bound matching the accepted downstream workbench.
pub const MAX_FULL_LOOP_DURATION_MS: u64 = 1000;

/// Existing default source observation interval in milliseconds, excluding acquisition/drain.
const DEFAULT_FULL_LOOP_DURATION_MS: u64 = 3;
/// Existing per-decode iteration default; validation uses the scientific owner's ceiling.
const DEFAULT_FULL_LOOP_ITERATIONS: usize = 50;
/// Existing active host-runtime default in milliseconds; separate from source observation time.
const DEFAULT_FULL_LOOP_RUNTIME_MS: u64 = 300_000;
/// Existing default number of detailed completed words retained by a full-loop run.
const DEFAULT_FULL_LOOP_RETAINED_WORDS: usize = 64;
/// Existing reproducible impairment RNG seed; independent of the source's PRBS state.
const DEFAULT_FULL_LOOP_NOISE_SEED: u64 = 41;
/// Existing default paired payload/NCP constellation detail request.
const DEFAULT_FULL_LOOP_CONSTELLATION_PAIRS: usize = 65_536;
/// Existing independent per-stream protected-role constellation detail default.
const DEFAULT_FULL_LOOP_PROTECTED_PAIRS: usize = 8192;

/// Typed editable execution/detail policy; a request freezes an independently validated copy.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FullLoopOptions {
    pub duration_ms: u64,
    pub max_iterations: usize,
    pub max_runtime_ms: u64,
    pub max_retained_words: usize,
    pub noise_seed: u64,
    pub receiver_mode: ReceiverMode,
    pub mce: MceSettings,
    pub constellation: Option<ConstellationSelection>,
    pub protected_constellation: Option<ProtectedCaptureSelection>,
}
impl Default for FullLoopOptions {
    /// Preserves supported workbench defaults independently of existing evaluation defaults.
    /// # Returns
    /// 3 ms, 50 iterations, five-minute active host budget and finite observation details.
    fn default() -> Self {
        Self {
            duration_ms: DEFAULT_FULL_LOOP_DURATION_MS,
            max_iterations: DEFAULT_FULL_LOOP_ITERATIONS,
            max_runtime_ms: DEFAULT_FULL_LOOP_RUNTIME_MS,
            max_retained_words: DEFAULT_FULL_LOOP_RETAINED_WORDS,
            noise_seed: DEFAULT_FULL_LOOP_NOISE_SEED,
            receiver_mode: ReceiverMode::SuppliedChannel,
            mce: MceSettings::default(),
            constellation: Some(ConstellationSelection {
                requested_pairs: DEFAULT_FULL_LOOP_CONSTELLATION_PAIRS,
                capture_ncp: true,
                ..Default::default()
            }),
            protected_constellation: Some(ProtectedCaptureSelection {
                pairs_per_stream: DEFAULT_FULL_LOOP_PROTECTED_PAIRS,
                ..Default::default()
            }),
        }
    }
}
impl FullLoopOptions {
    /// Validates finite application/receiver/detail limits without starting scientific work.
    /// # Returns
    /// Unit; physical profile-specific admission is deferred to request preparation.
    /// # Errors
    /// Rejects duration/iteration/runtime/detail limits and existing MCE/protected policy errors.
    pub fn validate(&self) -> ApiResult<()> {
        if !(1..=MAX_FULL_LOOP_DURATION_MS).contains(&self.duration_ms)
            || !(1..=MAX_CHANNEL_DECODER_ITERATIONS).contains(&self.max_iterations)
        {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Full-loop duration/iteration limits exceed bounds",
            ));
        }
        crate::simulation::channel::ChannelJobLimits {
            max_runtime_ms: self.max_runtime_ms,
            max_retained_words: self.max_retained_words,
        }
        .validate()?;
        self.mce.validate()?;
        if let Some(policy) = &self.protected_constellation {
            policy.validate()?;
        }
        Ok(())
    }
    /// Admits bounded options JSON through the same typed request-policy validator.
    /// # Arguments
    /// * `bytes` - Up to 64 KiB strict options JSON; unknown fields reject.
    /// # Returns
    /// Editable validated policy; admission starts no job/source stream.
    /// # Errors
    /// Preserves malformed/unknown/limit/scientific policy failures.
    pub fn from_json(bytes: &[u8]) -> ApiResult<Self> {
        if bytes.len() > MAX_FULL_LOOP_OPTIONS_BYTES {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Full-loop options exceed 64 KiB",
            ));
        }
        let result: Self = serde_json::from_slice(bytes)
            .map_err(|cause| ApiError::new(ErrorCode::InvalidInput, cause.to_string()))?;
        result.validate()?;
        Ok(result)
    }
}

impl RetainedPayload for FullLoopOptions {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            duration_ms,
            max_iterations,
            max_runtime_ms,
            max_retained_words,
            noise_seed,
            receiver_mode,
            mce,
            constellation,
            protected_constellation,
        } = self;
        merlin_api::retention_support::sum([
            duration_ms.retained_children(),
            max_iterations.retained_children(),
            max_runtime_ms.retained_children(),
            max_retained_words.retained_children(),
            noise_seed.retained_children(),
            receiver_mode.retained_children(),
            mce.retained_children(),
            constellation.retained_children(),
            protected_constellation.retained_children(),
        ])
    }
}
