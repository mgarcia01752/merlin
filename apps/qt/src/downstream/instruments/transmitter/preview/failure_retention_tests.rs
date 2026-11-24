/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual failed preview publication keeps old science and its queued error owner.
use super::*;
use crate::shell::{ApplicationSessions, RetainedResources};
use merlin::downstream::TransmissionSettings;
use std::time::{Duration, Instant};

// Independent host ceilings and scheduling watchdog; no scientific size estimates.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};
const WATCHDOG: Duration = Duration::from_secs(30);

fn wait_for(mut condition: impl FnMut() -> bool) {
    let deadline = Instant::now() + WATCHDOG;
    while !condition() {
        assert!(Instant::now() < deadline, "Preview error disposal watchdog");
        std::thread::yield_now();
    }
}

#[test]
fn real_target_failure_preserves_old_result_and_outlives_gui_until_payload_drop() {
    let application = ApplicationSessions::new(LIMITS).unwrap();
    let budget = application.budget().clone();
    let host = budget.usage();
    let mut model = Analyzer::for_context(Context::for_application(application.clone()).unwrap());
    let settings = serde_json::to_string(&TransmissionSettings::default()).unwrap();
    tests::configure(&mut model, &settings, tests::inputs());
    model.generate();
    let completed_control = Arc::downgrade(&model.job.as_ref().unwrap().control);
    tests::wait(&mut model);
    wait_for(|| completed_control.upgrade().is_none());
    let original_result = model.result.as_ref().unwrap().value() as *const _;
    let source = Arc::clone(model.prepared.as_ref().unwrap());
    let token = model.result_token;
    let revision = model.revision;
    let control = retention::control_resources(
        TransmitterPreviewControl::new()
            .retained_footprint()
            .unwrap(),
    )
    .unwrap();

    // Synthetic root pressure leaves the exact control demand available. The real
    // Job dispatches and only its backend product target rejects; error bytes fit.
    let pressure = budget
        .reserve_resources(RetainedResources {
            shared_roots: LIMITS.shared_roots - budget.usage().shared_roots - control.shared_roots,
            ..Default::default()
        })
        .unwrap();
    model.generate();
    let job = model
        .job
        .take()
        .expect("Actual control must admit and dispatch");
    let execution = Arc::downgrade(&job.control);
    let Err(failure) = job.receiver.recv_timeout(WATCHDOG).unwrap() else {
        panic!("Actual root pressure must reject the generated target");
    };
    let WorkerFailure::Detailed(retained) = &failure else {
        panic!("Error bytes must fit despite the independent root pressure");
    };
    let charge = retained.fixture_reserved_resources();
    assert!(failure.message().contains("retained-data limit"));
    assert_eq!(
        model.result.as_ref().unwrap().value() as *const _,
        original_result
    );
    assert!(Arc::ptr_eq(model.prepared.as_ref().unwrap(), &source));
    assert_eq!(model.result_token, token);
    assert_eq!(model.revision, revision);
    drop(job);
    wait_for(|| execution.upgrade().is_none());
    drop(pressure);
    drop(source);
    drop(model);
    assert_eq!(
        budget.usage(),
        RetainedResources {
            payload_bytes: host.payload_bytes + charge.payload_bytes,
            ..host
        }
    );
    drop(failure);
    assert_eq!(budget.usage(), host);
}
