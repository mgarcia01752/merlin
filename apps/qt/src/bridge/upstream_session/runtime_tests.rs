/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual unavailable adapter construction, same-host retry and fixed-owner disposal.
use super::*;
use crate::shell::{ApplicationSessions, Direction, RetainedResources};

// Independent host ceilings; no scientific or display-size policy is inferred.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};

#[test]
fn fixed_upstream_rejection_keeps_only_host_handles_and_retries_without_host_changes() {
    let application = ApplicationSessions::new(LIMITS).unwrap();

    let budget = application.budget();
    let baseline = budget.usage();
    let before = application.snapshot(Direction::Upstream);
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: LIMITS.payload_bytes - baseline.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    let mut owner = UpstreamSessionRust::for_application(application.clone());
    assert!(owner.model.is_none());
    assert!(owner.ensure_model().is_none());
    assert_eq!(owner.current_state()["can_start"], false);
    assert_eq!(application.snapshot(Direction::Upstream), before);
    drop(pressure);
    assert!(owner.ensure_model().is_some());
    assert_eq!(application.snapshot(Direction::Upstream), before);
    assert!(budget.usage().payload_bytes > baseline.payload_bytes);
    drop(owner);
    assert_eq!(budget.usage(), baseline);
}
