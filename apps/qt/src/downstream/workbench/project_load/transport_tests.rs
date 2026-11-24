/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Loader rejection before dispatch, with the genuine canonical host unchanged.
use super::*;
use crate::shell::ApplicationSessions;

// Independent counter-fixture ceilings; scientific and desktop policies are unchanged.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
};

#[test]
fn exhausted_channel_or_slots_reject_before_loader_work_and_keep_accepted_host() {
    let context = Context::for_application(ApplicationSessions::new(LIMITS).unwrap()).unwrap();
    let budget = context.application.budget();
    let before = context
        .application
        .snapshot(crate::shell::Direction::Downstream);
    let baseline = budget.usage();
    for pressure in [
        RetainedResources {
            channel_count: LIMITS.channel_count - baseline.channel_count,
            ..Default::default()
        },
        RetainedResources {
            slot_count: LIMITS.slot_count - baseline.slot_count,
            ..Default::default()
        },
    ] {
        let guard = budget.reserve_resources(pressure).unwrap();
        let full = budget.usage();
        let mut loader = Loader::default();
        let status = loader.start_with(budget, |_, _| panic!("Rejected loader must not run"));
        assert_eq!(status["error"], WORKER_TRANSPORT_ADMISSION_MESSAGE);
        assert!(!loader.is_loading());
        assert_eq!(budget.usage(), full);
        assert_eq!(
            context
                .application
                .snapshot(crate::shell::Direction::Downstream),
            before
        );
        assert!(context.prepared.borrow().receiver.is_none());
        drop(guard);
        assert_eq!(budget.usage(), baseline);
    }
}
