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
    channel_count: usize::MAX,
    slot_count: usize::MAX,
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
    let mut model =
        Analyzer::for_context(Context::for_application(application.clone()).unwrap()).unwrap();
    let settings = serde_json::to_string(&TransmissionSettings::default()).unwrap();
    tests::configure(&mut model, &settings, tests::inputs());
    let mut completed_execution = None;
    model.generate_with(|task| {
        completed_execution = Some(std::thread::spawn(move || crate::shell::run_task(task)));
        Ok(())
    });
    let completed_control = Arc::downgrade(&model.job.as_ref().unwrap().control);
    tests::wait(&mut model);
    completed_execution.take().unwrap().join().unwrap();
    wait_for(|| completed_control.upgrade().is_none());
    let original_result = model.result.as_ref().unwrap().value() as *const _;
    let source = Arc::clone(model.prepared.as_ref().unwrap());
    let token = model.result_token;
    let revision = model.revision;
    let mut pressure = None;
    let mut execution_task = None;
    model.generate_with(|task| {
        // Preserve the original product-target boundary: transport and control
        // already exist before synthetic root pressure is applied.
        pressure = Some(
            budget
                .reserve_resources(RetainedResources {
                    shared_roots: LIMITS.shared_roots - budget.usage().shared_roots,
                    ..Default::default()
                })
                .unwrap(),
        );
        execution_task = Some(std::thread::spawn(move || crate::shell::run_task(task)));
        Ok(())
    });
    let job = model
        .job
        .take()
        .expect("Actual control must admit and dispatch");
    let execution = Arc::downgrade(&job.control);
    let delivery = job.receiver.recv_timeout(WATCHDOG).unwrap();
    let frame = delivery.fixture_reserved_resources();
    let delivery_guard;
    let (result, guard) = delivery.into_parts();
    delivery_guard = guard;
    let Err(failure) = result else {
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
    execution_task.take().unwrap().join().unwrap();
    wait_for(|| execution.upgrade().is_none());
    drop(pressure);
    drop(source);
    drop(model);
    assert_eq!(
        budget.usage(),
        host.checked_add(charge)
            .unwrap()
            .checked_add(frame)
            .unwrap()
    );
    drop(failure);
    assert_eq!(budget.usage(), host.checked_add(frame).unwrap());
    drop(delivery_guard);
    assert_eq!(budget.usage(), host);
}
