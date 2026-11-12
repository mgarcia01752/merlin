/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Monotonic wall timing and bounded worker progress, separate from simulation time.
use docsis_ofdm_fec::simulation::channel::ChannelPhase;
use serde_json::{json, Value};
use std::{sync::Mutex, time::Instant};

const MAX_PROFILE_TIMINGS: usize = crate::instruments::receiver::performance::MAX_PROFILES;

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

pub(crate) struct RunTiming {
    state: Mutex<State>,
}

fn millis(start: Instant, end: Instant) -> u64 {
    end.saturating_duration_since(start)
        .as_millis()
        .min(u128::from(u64::MAX)) as u64
}

fn formatted(milliseconds: u64) -> String {
    let seconds = milliseconds / 1000;
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        seconds / 60 % 60,
        seconds % 60
    )
}

impl RunTiming {
    /// Starts an independent monotonic wall clock for one sweep.
    /// # Returns
    /// Thread-safe timing state; no simulation state or global clock is modified.
    pub(crate) fn new() -> Self {
        Self::at(Instant::now())
    }

    fn at(now: Instant) -> Self {
        Self {
            state: Mutex::new(State {
                started: now,
                profile_started: now,
                finished: None,
                last_progress: now,
                phase: "Preparing",
                recovered: 0,
                mce: None,
                profiles: Vec::new(),
            }),
        }
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
    /// * `phase` - Core processing phase after a completed step.
    /// * `recovered` - Current profile's completed payload-word count.
    pub(crate) fn progress(&self, phase: ChannelPhase, recovered: u64) {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        state.last_progress = Instant::now();
        state.phase = match phase {
            ChannelPhase::Acquisition => "Acquiring",
            ChannelPhase::Completion => "Completing",
            ChannelPhase::Failed => "Failed",
            ChannelPhase::Observation => "Observing",
            ChannelPhase::InterleaverDrain => "Draining",
            ChannelPhase::Finished => "Finishing",
        };
        state.recovered = recovered;
    }
    /// Records received-only MCE readiness; a full history is not an accuracy claim.
    pub(crate) fn mce(&self, tracking: Option<bool>) {
        self.state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .mce = tracking;
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
        json!({"elapsed_ms":elapsed,"elapsed_text":formatted(elapsed),
            "profile_elapsed_ms":profile,"profile_elapsed_text":formatted(profile),
            "phase":state.phase,"recovered_payloads":state.recovered,
            "mce":state.mce.map(|tracking|if tracking {"Tracking"}else{"Training"}),
            "progress_age_text":formatted(millis(state.last_progress,end)),
            "profiles":state.profiles.iter().map(|(id,elapsed)|json!({"profile_id":id,"elapsed_ms":elapsed,"elapsed_text":formatted(*elapsed)})).collect::<Vec<_>>()})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn elapsed_advances_without_progress_and_freezes_before_late_ui_poll() {
        let start = Instant::now();
        let clock = RunTiming::at(start);
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
        assert_eq!(RunTiming::at(start).snapshot_at(start)["elapsed_ms"], 0);
    }

    #[test]
    fn formatting_and_profile_progress_are_separate_from_simulated_time() {
        assert_eq!(formatted(999), "00:00:00");
        assert_eq!(formatted(3_661_000), "01:01:01");
        assert_eq!(formatted(360_000_000), "100:00:00");
        let clock = RunTiming::new();
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
