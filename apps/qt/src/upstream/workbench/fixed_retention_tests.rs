/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Fixed upstream GUI/temporary controller slot storage and genuine source preservation.
use super::*;
use crate::upstream::PreparedSession;
use merlin::upstream::{ConfigurationDocument, ReceiverObservationPolicy, Simulation};
use std::{
    mem::size_of,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

const DOCUMENT: &[u8] = include_bytes!("../../../../../examples/cli/upstream/us-50khz.json");
// Independent host-accounting pressure and scheduler bound; no scientific estimate.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
};
const WATCHDOG: Duration = Duration::from_secs(30);
// The concrete controller slot creates one actual Arc pointee.
const SLOT_SHARED_ROOTS: usize = 1;

#[test]
fn upstream_fixed_box_exact_and_one_byte_below_do_not_advance_host_or_claim_roots() {
    let measured_baseline = ApplicationSessions::new(LIMITS).unwrap().budget().usage();
    let fixed = size_of::<Model>();
    for shortage in [false, true] {
        let exact = measured_baseline.payload_bytes.checked_add(fixed).unwrap();
        let maximum = if shortage { exact - 1 } else { exact };
        let application = ApplicationSessions::new(RetainedResources {
            payload_bytes: maximum,
            ..LIMITS
        })
        .unwrap();
        let budget = application.budget();
        let baseline = budget.usage();
        assert_eq!(baseline, measured_baseline);
        let snapshot = application.snapshot(Direction::Upstream);
        let model = Model::new(application.clone());
        assert_eq!(application.snapshot(Direction::Upstream), snapshot);
        if shortage {
            assert!(matches!(model, Err(RetentionError::LimitExceeded { .. })));
            assert_eq!(budget.usage(), baseline);
        } else {
            let model = model.unwrap();
            assert!(budget.owns(&model._fixed_guard));
            assert_eq!(
                model._fixed_guard.fixture_reserved_resources(),
                RetainedResources {
                    payload_bytes: fixed,
                    ..Default::default()
                }
            );
            assert_eq!(budget.usage().payload_bytes, maximum);
            assert_eq!(budget.usage().ordered_entries, baseline.ordered_entries);
            assert_eq!(budget.usage().shared_roots, baseline.shared_roots);
            drop(model);
            assert_eq!(budget.usage(), baseline);
        }
    }
}

#[test]
fn controller_slot_rejection_returns_original_and_actual_preparation_aliases_keep_exact_charge() {
    let application = ApplicationSessions::new(LIMITS).unwrap();
    let budget = application.budget();
    let host = budget.usage();
    let prepared = PreparedSession::from_document(
        ConfigurationDocument::from_json(DOCUMENT).unwrap(),
        ReceiverObservationPolicy::default(),
        budget,
    )
    .unwrap();
    let key = application.snapshot(Direction::Upstream).key;
    let mut controller = UpstreamController::new(key, prepared).unwrap();
    let source_address = controller
        .inspect(|simulation| simulation as *const Simulation as usize)
        .unwrap();
    let baseline = budget.usage();
    let slot = RetainedResources {
        payload_bytes: size_of::<Retained<Mutex<Option<UpstreamController>>>>(),
        ordered_entries: 0,
        shared_roots: SLOT_SHARED_ROOTS,
        ..Default::default()
    };
    for bytes in [true, false] {
        let pressure = budget
            .reserve_resources(if bytes {
                RetainedResources {
                    payload_bytes: LIMITS.payload_bytes
                        - baseline.payload_bytes
                        - slot.payload_bytes
                        + 1,
                    ..Default::default()
                }
            } else {
                RetainedResources {
                    shared_roots: LIMITS.shared_roots - baseline.shared_roots,
                    ..Default::default()
                }
            })
            .unwrap();
        let before = budget.usage();
        let Err((original, message)) = Pending::run(controller, budget.clone()) else {
            panic!("Exact fixed slot pressure must reject before worker publication");
        };
        controller = original;
        assert!(!message.is_empty());
        assert_eq!(
            controller
                .inspect(|simulation| simulation as *const Simulation as usize)
                .unwrap(),
            source_address
        );
        assert_eq!(budget.usage(), before);
        assert_eq!(application.snapshot(Direction::Upstream).key, key);
        assert!(application.active().is_none());
        drop(pressure);
        assert_eq!(budget.usage(), baseline);
    }

    // Admit the actual slot, channel, completion and closure first. Fill the
    // remaining byte budget in the dispatch seam, then execute the real report
    // preflight. This reaches the original target boundary without a guessed
    // closure layout or an earlier transport rejection.
    let mut pressure = None;
    let pending = match Pending::run_with(controller, budget.clone(), |task| {
        pressure = Some(
            budget
                .reserve_resources(RetainedResources {
                    payload_bytes: LIMITS.payload_bytes - budget.usage().payload_bytes,
                    ..Default::default()
                })
                .unwrap(),
        );
        std::thread::spawn(move || crate::shell::run_task(task));
        Ok(())
    }) {
        Ok(pending) => pending,
        Err((_, message)) => panic!("Actual preparation must dispatch: {message}"),
    };
    assert!(pressure.is_some(), "The original target gate must execute");
    let Pending::Run { owner, .. } = &pending else {
        unreachable!()
    };
    let held = Arc::clone(owner);
    let weak = Arc::downgrade(owner);
    assert_eq!(held.fixture_reserved_resources(), slot);
    let deadline = Instant::now() + WATCHDOG;
    let completed = loop {
        if let Some(completed) = pending.poll() {
            break completed;
        }
        assert!(
            Instant::now() < deadline,
            "Genuine target preparation watchdog"
        );
        std::thread::yield_now();
    };
    let Prepared::Run {
        delivery: _delivery_guard,
        controller,
        target,
    } = completed
    else {
        unreachable!()
    };
    assert!(matches!(target, Err(WorkerFailure::AdmissionRejected(_))));
    assert_eq!(
        controller
            .inspect(|simulation| simulation as *const Simulation as usize)
            .unwrap(),
        source_address
    );
    assert!(application.active().is_none());
    drop(pending);
    drop(_delivery_guard);
    drop(pressure);
    let mut expected = baseline;
    crate::downstream::inputs::retention::append(&mut expected, slot).unwrap();
    while budget.usage() != expected {
        assert!(Instant::now() < deadline, "Actual task cleanup watchdog");
        std::thread::yield_now();
    }
    assert_eq!(budget.usage(), expected);

    // This explicit observed alias is not a worker barrier. After dropping it,
    // wait for actual closure aliases to disappear before asserting final release.
    drop(held);
    while weak.upgrade().is_some() || budget.usage() != baseline {
        assert!(
            Instant::now() < deadline,
            "Actual slot alias disposal watchdog"
        );
        std::thread::yield_now();
    }
    assert_eq!(budget.usage(), baseline);
    drop(controller);
    assert_eq!(budget.usage(), host);
}
