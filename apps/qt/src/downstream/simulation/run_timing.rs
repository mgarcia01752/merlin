/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Monotonic wall timing and bounded worker progress, separate from simulation time.
use crate::shared::presentation::constants::{
    MINUTES_PER_HOUR, SECONDS_PER_HOUR, SECONDS_PER_MINUTE,
};
use crate::shell::{
    Retained, RetainedResources, RetentionBudget, RetentionError, RetentionReservation,
};
use merlin::downstream::{ChannelPhase, FullLoopControl};
use serde_json::{json, Value};
use std::{
    mem::size_of,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

const MAX_PROFILE_TIMINGS: usize =
    crate::downstream::instruments::receiver::performance::MAX_PROFILES;

struct State {
    started: Instant,
    profile_started: Instant,
    finished: Option<Instant>,
    last_progress: Instant,
    phase: &'static str,
    recovered: u64,
    mce: Option<bool>,
    profiles: Vec<(u8, u64)>,
}

/// Single-use timing admission without starting its monotonic clock.
pub(crate) struct PreparedTiming {
    reservation: RetentionReservation,
}

pub(crate) struct RunTiming {
    state: Mutex<State>,

    // State and its profile buffer drop before releasing this shared owner charge.
    _guard: Retained<()>,
}

fn phase_label(phase: ChannelPhase) -> &'static str {
    match phase {
        ChannelPhase::Acquisition => "Acquiring",
        ChannelPhase::Completion => "Completing",
        ChannelPhase::Failed => "Failed",
        ChannelPhase::Observation => "Observing",
        ChannelPhase::InterleaverDrain => "Draining",
        ChannelPhase::Finished => "Finishing",
    }
}

fn millis(start: Instant, end: Instant) -> u64 {
    end.saturating_duration_since(start)
        .as_millis()
        .min(u128::from(u64::MAX)) as u64
}

fn formatted(milliseconds: u64) -> String {
    let seconds = Duration::from_millis(milliseconds).as_secs();
    // Clock fields use at least two display digits; hours remain unbounded.
    format!(
        "{:02}:{:02}:{:02}",
        seconds / SECONDS_PER_HOUR,
        seconds / SECONDS_PER_MINUTE % MINUTES_PER_HOUR,
        seconds % SECONDS_PER_MINUTE
    )
}

impl RunTiming {
    /// Starts the existing monotonic clock by consuming its exact prior admission.
    ///
    /// # Arguments
    /// * `prepared` - Single-use token reserved before host dispatch/status adoption.
    ///
    /// # Returns
    /// One guarded Arc shared by GUI and worker, with exact preallocated profile
    /// capacity. The clock starts here, never while the preparation token is held.
    pub(crate) fn new(prepared: PreparedTiming) -> Arc<Self> {
        Self::at(Instant::now(), prepared)
    }

    /// Reserves complete timing storage without starting clocks or changing a model.
    ///
    /// # Arguments
    /// * `budget` - Canonical host counter covering old, candidate and executing owners.
    ///
    /// # Returns
    /// Single-use admission for the actual final pointee and existing maximum profile
    /// capacity, with one shared root and no ordered entries. No Arc is allocated yet.
    ///
    /// # Errors
    /// Checked layout/capacity arithmetic or any aggregate dimension before dispatch.
    pub(crate) fn prepare(budget: &RetentionBudget) -> Result<PreparedTiming, RetentionError> {
        Ok(PreparedTiming {
            reservation: budget.reserve_resources(resources(MAX_PROFILE_TIMINGS)?)?,
        })
    }

    fn at(now: Instant, prepared: PreparedTiming) -> Arc<Self> {
        let mut profiles: Vec<(u8, u64)> = Vec::from(
            vec![(u8::default(), u64::default()); MAX_PROFILE_TIMINGS].into_boxed_slice(),
        );
        profiles.clear();
        Arc::new(Self {
            state: Mutex::new(State {
                started: now,
                profile_started: now,
                finished: None,
                last_progress: now,
                phase: "Preparing",
                recovered: 0,
                mce: None,
                profiles,
            }),
            _guard: prepared.reservation.retain(()),
        })
    }

    /// Resets current-profile timing without resetting total sweep time.
    pub(crate) fn begin_profile(&self) {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        let now = Instant::now();
        state.profile_started = now;
        state.last_progress = now;
        state.phase = "Preparing";
        state.recovered = 0;
        state.mce = None;
    }

    /// Records actual post-step progress without advancing the engine.
    /// # Arguments
    /// * `phase` - Public downstream processing phase after a completed step.
    /// * `recovered` - Current profile's completed payload-word count.
    #[cfg(test)]
    pub(crate) fn progress(&self, phase: ChannelPhase, recovered: u64) {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        state.last_progress = Instant::now();
        state.phase = phase_label(phase);
        state.recovered = recovered;
    }
    /// Projects actual public notifications without treating UI polling as progress.
    /// # Arguments
    /// * `control` - Current profile's public one-shot execution control.
    /// # Returns
    /// Completed source symbols, or zero before the first backend notification.
    pub(crate) fn observe(&self, control: &FullLoopControl) -> u64 {
        let snapshot = control.snapshot();
        let Some(progress) = snapshot.progress else {
            return 0;
        };
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        state.phase = phase_label(progress.phase);
        state.recovered = progress.recovered_payloads;
        state.mce = progress
            .receiver_reliability
            .as_ref()
            .map(|status| status.tracking);
        if let Some(notified) = control
            .last_progress_age_ms()
            .and_then(|age| Instant::now().checked_sub(Duration::from_millis(age)))
        {
            state.last_progress = notified;
        }
        progress.completed_source_symbols
    }

    /// Retains a completed profile's wall duration, bounded by sweep admission.
    /// # Arguments
    /// * `id` - Original captured profile identity, never a wire-profile alias.
    pub(crate) fn end_profile(&self, id: u8) {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        if state.profiles.len() < MAX_PROFILE_TIMINGS {
            let elapsed = millis(state.profile_started, Instant::now());
            state.profiles.push((id, elapsed));
        }
    }

    /// Freezes elapsed time on actual worker completion; repeated calls are inert.
    pub(crate) fn finish(&self) {
        self.finish_at(Instant::now());
    }

    fn finish_at(&self, now: Instant) {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        state.finished.get_or_insert(now);
    }

    /// Reads bounded elapsed strings and progress without changing clock state.
    /// # Returns
    /// Wall times include pauses and scheduling waits; completed totals stay frozen.
    pub(crate) fn snapshot(&self) -> Value {
        self.snapshot_at(Instant::now())
    }

    fn snapshot_at(&self, now: Instant) -> Value {
        let state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        let end = state.finished.unwrap_or(now);
        let elapsed = millis(state.started, end);
        let profile = state
            .profiles
            .last()
            .filter(|_| state.finished.is_some())
            .map_or_else(
                || millis(state.profile_started, end),
                |(_, elapsed)| *elapsed,
            );
        let mce = state.mce.map(|tracking| {
            if tracking {
                // This is receiver readiness text, independent of the wall clock.
                "Tracking"
            } else {
                "Training"
            }
        });
        json!({"elapsed_ms":elapsed,"elapsed_text":formatted(elapsed),
            "profile_elapsed_ms":profile,"profile_elapsed_text":formatted(profile),
            "phase":state.phase,"recovered_payloads":state.recovered,
            "mce":mce,
            "progress_age_text":formatted(millis(state.last_progress,end)),
            "profiles":state.profiles.iter().map(|(id,elapsed)|json!({"profile_id":id,"elapsed_ms":elapsed,"elapsed_text":formatted(*elapsed)})).collect::<Vec<_>>()})
    }
}

fn resources(profile_capacity: usize) -> Result<RetainedResources, RetentionError> {
    let profile_bytes = profile_capacity
        .checked_mul(size_of::<(u8, u64)>())
        .ok_or(RetentionError::ArithmeticOverflow)?;
    Ok(RetainedResources {
        payload_bytes: size_of::<RunTiming>()
            .checked_add(profile_bytes)
            .ok_or(RetentionError::ArithmeticOverflow)?,
        ordered_entries: 0,
        shared_roots: 1,
    })
}

/// Measures the complete timing target for exact existing-fixture accounting only.
///
/// # Returns
/// Actual final pointee and capacity charge; no default production policy or clock.
#[cfg(test)]
pub(crate) fn fixture_resources() -> RetainedResources {
    resources(MAX_PROFILE_TIMINGS).unwrap()
}

#[cfg(test)]
mod retention_tests;

#[cfg(test)]
mod tests {
    use super::*;

    fn prepared() -> PreparedTiming {
        let budget = RetentionBudget::with_limits(fixture_resources());
        RunTiming::prepare(&budget).unwrap()
    }

    #[test]
    fn all_public_processing_phases_preserve_existing_status_labels() {
        let clock = RunTiming::new(prepared());
        for (phase, label) in [
            (ChannelPhase::Acquisition, "Acquiring"),
            (ChannelPhase::Completion, "Completing"),
            (ChannelPhase::Failed, "Failed"),
            (ChannelPhase::Observation, "Observing"),
            (ChannelPhase::InterleaverDrain, "Draining"),
            (ChannelPhase::Finished, "Finishing"),
        ] {
            clock.progress(phase, 0);
            assert_eq!(clock.snapshot()["phase"], label);
        }
    }

    #[test]
    fn elapsed_advances_without_progress_and_freezes_before_late_ui_poll() {
        let start = Instant::now();
        let clock = RunTiming::at(start, prepared());
        assert_eq!(
            clock.snapshot_at(start + Duration::from_secs(65))["elapsed_text"],
            "00:01:05"
        );
        clock.finish_at(start + Duration::from_secs(66));
        clock.finish_at(start + Duration::from_secs(100));
        let final_state = clock.snapshot_at(start + Duration::from_secs(999));
        assert_eq!(final_state["elapsed_ms"], 66000);
        assert_eq!(final_state["elapsed_text"], "00:01:06");
        assert_eq!(final_state["progress_age_text"], "00:01:06");
        assert_eq!(
            RunTiming::at(start, prepared()).snapshot_at(start)["elapsed_ms"],
            0
        );
    }

    #[test]
    fn formatting_and_profile_progress_are_separate_from_simulated_time() {
        assert_eq!(formatted(999), "00:00:00");
        assert_eq!(formatted(3_661_000), "01:01:01");
        assert_eq!(formatted(360_000_000), "100:00:00");
        let clock = RunTiming::new(prepared());
        clock.begin_profile();
        clock.progress(ChannelPhase::InterleaverDrain, 12);
        clock.end_profile(3);
        clock.finish();
        let state = clock.snapshot();
        assert_eq!(state["phase"], "Draining");
        assert_eq!(state["recovered_payloads"], 12);
        assert_eq!(state["profiles"][0]["profile_id"], 3);
        assert_eq!(
            state["profile_elapsed_ms"],
            state["profiles"][0]["elapsed_ms"]
        );
    }
}
