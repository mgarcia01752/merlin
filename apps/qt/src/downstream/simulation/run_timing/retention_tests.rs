/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Independent final timing layout, capacity, epoch and alias-lifetime checks.
use super::*;

// Actual stored captured identity and deterministic elapsed-time anchor only.
const PROFILE_ID: u8 = 3;
const FIXTURE_ELAPSED: Duration = Duration::from_millis(7);

fn expected() -> RetainedResources {
    RetainedResources {
        payload_bytes: size_of::<RunTiming>() + MAX_PROFILE_TIMINGS * size_of::<(u8, u64)>(),
        ordered_entries: 0,
        shared_roots: 1,
    }
}

#[test]
fn exact_and_below_positive_dimensions_admit_or_roll_back_before_clock_construction() {
    let expected = expected();
    let budget = RetentionBudget::with_limits(expected);
    let prepared = RunTiming::prepare(&budget).unwrap();
    assert_eq!(budget.usage(), expected);
    drop(prepared);
    assert_eq!(budget.usage(), RetainedResources::default());
    for maximum in [
        RetainedResources {
            payload_bytes: expected.payload_bytes - 1,
            ..expected
        },
        RetainedResources {
            shared_roots: 0,
            ..expected
        },
    ] {
        let budget = RetentionBudget::with_limits(maximum);
        assert!(RunTiming::prepare(&budget).is_err());
        assert_eq!(budget.usage(), RetainedResources::default());
    }
    assert_eq!(
        resources(usize::MAX),
        Err(RetentionError::ArithmeticOverflow)
    );
    assert_eq!(
        resources(usize::MAX / size_of::<(u8, u64)>()),
        Err(RetentionError::ArithmeticOverflow)
    );
}

#[test]
fn every_capped_push_preserves_the_exact_initial_buffer_and_profile_values() {
    let budget = RetentionBudget::with_limits(expected());
    let clock = RunTiming::new(RunTiming::prepare(&budget).unwrap());
    let (capacity, pointer) = {
        let state = clock.state.lock().unwrap();
        assert!(state.profiles.is_empty());
        (state.profiles.capacity(), state.profiles.as_ptr())
    };
    assert_eq!(capacity, MAX_PROFILE_TIMINGS);
    for _ in 0..MAX_PROFILE_TIMINGS {
        clock.end_profile(PROFILE_ID);
    }
    let before = {
        let state = clock.state.lock().unwrap();
        assert_eq!(state.profiles.len(), MAX_PROFILE_TIMINGS);
        assert!(state.profiles.iter().all(|(id, _)| *id == PROFILE_ID));
        state.profiles.clone()
    };
    clock.end_profile(PROFILE_ID);
    let state = clock.state.lock().unwrap();
    assert_eq!(state.profiles, before);
    assert_eq!(state.profiles.capacity(), capacity);
    assert_eq!(state.profiles.as_ptr(), pointer);
    assert_eq!(budget.usage(), expected());
}

#[test]
fn prepared_token_defers_epoch_and_final_aliases_keep_the_single_reservation() {
    let budget = RetentionBudget::with_limits(expected());
    let prepared = RunTiming::prepare(&budget).unwrap();
    let epoch = Instant::now();
    let clock = RunTiming::at(epoch, prepared);
    assert_eq!(clock.snapshot_at(epoch)["elapsed_ms"], 0);
    clock.finish_at(epoch + FIXTURE_ELAPSED);
    let snapshot = clock.snapshot_at(epoch + FIXTURE_ELAPSED);
    assert_eq!(
        snapshot["elapsed_ms"],
        u64::try_from(FIXTURE_ELAPSED.as_millis()).unwrap()
    );
    let worker = Arc::clone(&clock);
    let weak = Arc::downgrade(&clock);
    drop(clock);
    assert_eq!(budget.usage(), expected());
    assert_eq!(worker.snapshot_at(epoch + FIXTURE_ELAPSED), snapshot);
    drop(worker);
    assert!(weak.upgrade().is_none());
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn old_and_candidate_timing_overlap_without_replacing_the_old_clock_on_rejection() {
    let charge = expected();
    let budget = RetentionBudget::with_limits(charge.checked_add(charge).unwrap());
    let old = RunTiming::new(RunTiming::prepare(&budget).unwrap());
    old.end_profile(PROFILE_ID);
    old.finish();
    let before = old.snapshot();
    let candidate = RunTiming::prepare(&budget).unwrap();
    assert_eq!(budget.usage(), charge.checked_add(charge).unwrap());
    assert!(RunTiming::prepare(&budget).is_err());
    assert_eq!(old.snapshot(), before);
    drop(candidate);
    assert_eq!(budget.usage(), charge);
    let alias = Arc::clone(&old);
    drop(old);
    assert_eq!(budget.usage(), charge);
    drop(alias);
    assert_eq!(budget.usage(), RetainedResources::default());
}
