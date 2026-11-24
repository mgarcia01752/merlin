/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Directional/common control allocation audits and supported real-run passivity.
use super::*;
use std::{
    mem::size_of,
    panic::{catch_unwind, AssertUnwindSafe},
};

const DOCUMENT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../examples/cli/upstream/us-50khz.json"
));

// Independent valid association identities keep both comparison runs identical.
const FIXTURE_RUN_ID: u64 = 1;
const FIXTURE_GENERATION: u64 = 1;

// Stored pause-accounting anchor in milliseconds; metadata must not reset it.
const ACCUMULATED_PAUSE_MS: u64 = 7;

fn expected(shared_stop: bool) -> RetainedFootprint {
    RetainedFootprint {
        payload_bytes: size_of::<RunControl>()
            + size_of::<State>()
            + if shared_stop {
                size_of::<AtomicBool>()
            } else {
                0
            },
        ordered_entries: 0,
        shared_roots: if shared_stop { 2 } else { 1 },
    }
}

fn assert_passive(control: &RunControl, shared_stop: bool) {
    let before = control.progress();
    let flags = (
        control.inner.cancel.load(Ordering::Acquire),
        control.inner.claimed.load(Ordering::Acquire),
    );
    let pause = {
        let pause = control
            .inner
            .pause
            .lock()
            .unwrap_or_else(|cause| cause.into_inner());
        (pause.requested, pause.since, pause.accumulated)
    };
    let threshold = control
        .inner
        .cancel_after_completed_grants
        .load(Ordering::Acquire);
    assert_eq!(control.retained_footprint().unwrap(), expected(shared_stop));
    assert_eq!(control.progress(), before);
    assert_eq!(
        (
            control.inner.cancel.load(Ordering::Acquire),
            control.inner.claimed.load(Ordering::Acquire)
        ),
        flags
    );
    let after = control
        .inner
        .pause
        .lock()
        .unwrap_or_else(|cause| cause.into_inner());
    assert_eq!((after.requested, after.since, after.accumulated), pause);
    assert_eq!(
        control
            .inner
            .cancel_after_completed_grants
            .load(Ordering::Acquire),
        threshold
    );
}

#[test]
fn optional_common_token_adds_only_its_atomic_and_one_root_and_aliases_survive() {
    let plain = RunControl::new();
    assert_passive(&plain, false);
    let shared = merlin_api::RunControl::new();
    let control = RunControl::with_cancellation(shared.clone());
    assert_eq!(
        control.retained_footprint().unwrap().payload_bytes
            - plain.retained_footprint().unwrap().payload_bytes,
        size_of::<AtomicBool>()
    );
    let weak = Arc::downgrade(&control.inner);
    let alias = control.clone();
    assert!(Arc::ptr_eq(&control.inner, &alias.inner));
    assert_passive(&control, true);
    drop(control);
    assert!(weak.upgrade().is_some());
    shared.cancel();
    assert_passive(&alias, true);
    assert!(alias.is_cancelled());
    drop(alias);
    assert!(weak.upgrade().is_none());
    assert!(shared.is_cancelled());
}

#[test]
fn metadata_does_not_claim_reset_pause_clocks_or_advance_progress() {
    let control = RunControl::new();
    assert_passive(&control, false);
    control.pause();
    {
        let mut pause = control.inner.pause.lock().unwrap();
        pause.since = Some(Instant::now());
        pause.accumulated = Duration::from_millis(ACCUMULATED_PAUSE_MS);
    }
    let mut progress = control.progress();
    progress.status = RunStatus::Paused;
    control.publish(progress);
    assert_passive(&control, false);
    control.resume();
    let lease = control.claim().unwrap();
    assert_passive(&control, false);
    assert!(matches!(
        control.claim(),
        Err(PhyError::Association {
            field: "control.busy"
        })
    ));
    drop(lease);
    control.cancel();
    let mut terminal = control.progress();
    terminal.status = RunStatus::Cancelled;
    control.publish(terminal);
    assert_passive(&control, false);
    let lease = control.claim().unwrap();
    drop(lease);
}

#[test]
fn separate_poisoned_progress_and_pause_locks_recover_without_clearing_them() {
    let control = RunControl::new();
    assert!(catch_unwind(AssertUnwindSafe(|| {
        let _guard = control.inner.progress.lock().unwrap();
        panic!("Progress mutex poison fixture");
    }))
    .is_err());
    assert!(catch_unwind(AssertUnwindSafe(|| {
        let _guard = control.inner.pause.lock().unwrap();
        panic!("Pause mutex poison fixture");
    }))
    .is_err());
    assert_passive(&control, false);
    assert!(control.inner.progress.is_poisoned());
    assert!(control.inner.pause.is_poisoned());
    assert_eq!(control.progress().status, RunStatus::Idle);
}

fn without_elapsed(mut value: serde_json::Value) -> serde_json::Value {
    fn visit(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Object(object) => {
                let _ = object.remove("elapsed_ms");
                for child in object.values_mut() {
                    visit(child);
                }
            }
            serde_json::Value::Array(values) => {
                for child in values {
                    visit(child);
                }
            }
            _ => {}
        }
    }
    visit(&mut value);
    value
}

#[test]
fn metadata_preserves_supported_completed_and_shared_precancelled_run_science() {
    let document = crate::api::ConfigurationDocument::from_json(DOCUMENT).unwrap();
    let (configuration, options) = document.prepare().unwrap();
    let baseline = crate::simulation::run(
        &configuration,
        &options,
        FIXTURE_RUN_ID,
        FIXTURE_GENERATION,
        &RunControl::new(),
    )
    .unwrap();
    let control = RunControl::new();
    assert_passive(&control, false);
    let measured = crate::simulation::run(
        &configuration,
        &options,
        FIXTURE_RUN_ID,
        FIXTURE_GENERATION,
        &control,
    )
    .unwrap();
    assert_eq!(measured.status, RunStatus::Completed);
    assert!(measured.reference_recovered);
    assert_eq!(
        without_elapsed(serde_json::to_value(&measured).unwrap()),
        without_elapsed(serde_json::to_value(&baseline).unwrap())
    );
    assert_passive(&control, false);
    let common = merlin_api::RunControl::new();
    let cancelled = RunControl::with_cancellation(common.clone());
    common.cancel();
    assert_passive(&cancelled, true);
    let partial = crate::simulation::run(
        &configuration,
        &options,
        FIXTURE_RUN_ID,
        FIXTURE_GENERATION,
        &cancelled,
    )
    .unwrap();
    assert_eq!(partial.status, RunStatus::Cancelled);
    assert_eq!(partial.generated_source_bits, 0);
    assert_eq!(partial.compared_source_bits, 0);
    assert_passive(&cancelled, true);
}
