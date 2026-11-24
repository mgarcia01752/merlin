/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual idle-preview transport rejection preserves its accepted scientific owners.
use super::*;
use crate::shell::ApplicationSessions;
use merlin::downstream::TransmissionSettings;
use std::time::{Duration, Instant};

const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
};
const WATCHDOG: Duration = Duration::from_secs(30);

#[test]
fn channel_and_slot_failure_precede_preview_status_clear_and_preserve_old_product() {
    let context = Context::for_application(ApplicationSessions::new(LIMITS).unwrap()).unwrap();
    let mut model = Analyzer::for_context(context).unwrap();
    tests::configure(
        &mut model,
        &serde_json::to_string(&TransmissionSettings::default()).unwrap(),
        tests::inputs(),
    );
    model.generate();
    tests::wait(&mut model);
    let deadline = Instant::now() + WATCHDOG;
    while model.context.application.budget().usage().channel_count != 0 {
        assert!(
            Instant::now() < deadline,
            "Actual preview endpoint cleanup watchdog"
        );
        std::thread::yield_now();
    }
    let prepared = Arc::clone(model.prepared.as_ref().unwrap());
    let result = model.result.as_deref().unwrap() as *const _;
    let token = model.result_token;
    let revision = model.revision;
    let notice = model.notice;
    let budget = model.context.application.budget().clone();
    for channels in [true, false] {
        let used = budget.usage();
        let pressure = budget
            .reserve_resources(if channels {
                RetainedResources {
                    channel_count: LIMITS.channel_count - used.channel_count,
                    ..Default::default()
                }
            } else {
                RetainedResources {
                    slot_count: LIMITS.slot_count - used.slot_count,
                    ..Default::default()
                }
            })
            .unwrap();
        model.generate();
        assert!(model.job.is_none());
        assert!(Arc::ptr_eq(model.prepared.as_ref().unwrap(), &prepared));
        assert_eq!(model.result.as_deref().unwrap() as *const _, result);
        assert_eq!(model.result_token, token);
        assert_eq!(model.revision, revision);
        assert_eq!(model.notice, notice);
        assert!(!model.error.message().is_empty());
        drop(pressure);
    }
}
