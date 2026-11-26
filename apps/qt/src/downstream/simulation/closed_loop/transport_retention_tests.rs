/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual Start transport rejection precedes history, token and timing mutation.
use super::*;
use crate::downstream::instruments::receiver::performance;
use crate::shell::{ApplicationSessions, RetainedResources};
use std::time::{Duration, Instant};

const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
    presentation_utf16_bytes: 0,
    presentation_entries: 0,
    retained_views: 0,
    open_views: 0,
};
const WATCHDOG: Duration = Duration::from_secs(30);
const OBSERVATION_MS: u64 = 1; // Existing supported complete observation fixture.
const ACTIVE_RUNTIME_MS: u64 = 60_000; // Independent host execution allowance.

#[test]
fn actual_channel_and_slot_rejection_preserves_completed_report_history_and_epoch() {
    let context = Context::for_application(ApplicationSessions::new(LIMITS).unwrap()).unwrap();
    let (inputs, transmission, sections) = tests::accepted_inputs();
    crate::downstream::workbench::session::restore(&context, inputs, sections).unwrap();
    let mut model = Model::for_context(context.clone()).unwrap();
    model
        .accept_options(
            Some(Options {
                duration_ms: OBSERVATION_MS,
                max_runtime_ms: ACTIVE_RUNTIME_MS,
                ..Default::default()
            }),
            true,
            &serde_json::to_string(&transmission).unwrap(),
        )
        .unwrap();
    model.start();
    let deadline = Instant::now() + WATCHDOG;
    while model.poll()["busy"] != false || context.application.budget().usage().channel_count != 0 {
        assert!(
            Instant::now() < deadline,
            "Completed transport fixture watchdog"
        );
        std::thread::yield_now();
    }
    let report = latest(&context).unwrap();
    assert_eq!(
        report.job.status,
        merlin::downstream::ChannelJobStatus::Complete
    );
    let science = serde_json::to_value(report.backend().report()).unwrap();
    let timing = Arc::clone(model.timing.as_ref().unwrap());
    let epoch = timing.snapshot();
    let token = model.token;
    let history = performance::view(&context, -1, 0, 0); // Latest entry, full source span.
    let key = context.application.snapshot(Direction::Downstream);
    for channels in [true, false] {
        let budget = context.application.budget();
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
        model.start();
        assert!(model.worker.is_none());
        assert!(context.application.active().is_none());
        assert_eq!(context.application.snapshot(Direction::Downstream), key);
        assert_eq!(model.token, token);
        assert!(Arc::ptr_eq(model.timing.as_ref().unwrap(), &timing));
        assert_eq!(timing.snapshot(), epoch);
        assert!(Arc::ptr_eq(&latest(&context).unwrap(), &report));
        assert_eq!(
            serde_json::to_value(report.backend().report()).unwrap(),
            science
        );
        assert_eq!(performance::view(&context, -1, 0, 0), history);
        assert!(model.error.message().contains("retained-data limit"));
        drop(pressure);
    }
}
