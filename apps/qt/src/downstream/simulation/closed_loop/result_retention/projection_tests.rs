/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Genuine worker projection rejection retains completed science and final-alias charges.
use super::*;

#[test]
fn actual_worker_preserves_completed_science_and_static_warning_when_projection_admission_rejects()
{
    let source_policy = fixture_budget();
    let source = request_retention::admit_request(request(), &source_policy).unwrap();
    let accepted_charge = source_policy.usage();
    let backend_charge = resources(source.value().result_retained_bound().unwrap()).unwrap();
    let working_charge = working_allowance(source.value());
    let metadata = ApplicationSessions::new(fixture_budget_limits())
        .unwrap()
        .budget()
        .usage();
    let maximum = sum(
        metadata,
        sum(
            accepted_charge,
            sum(sum(working_charge, backend_charge), {
                let (adapter, profile) = worker::fixture_control_resources();
                sum(adapter, profile)
            }),
        ),
    );
    let application = ApplicationSessions::new(maximum).unwrap();
    let budget = application.budget();
    let accepted = request_retention::rebind_request(&source, budget).unwrap();
    drop(source);
    assert_eq!(source_policy.usage(), RetainedResources::default());
    let key = application.configure_downstream(true).unwrap();
    let control = worker::DownstreamControl::new(budget).unwrap();
    let lease = application.begin(key, control.clone()).unwrap();
    let worker = worker::Worker::spawn(
        accepted.clone(),
        None,
        Arc::new(super::super::super::RunTiming::new()),
        control,
        lease,
        budget.clone(),
    )
    .unwrap_or_else(|error| {
        let (cause, lease) = error.into_parts();
        application.finish(lease).unwrap();
        panic!("Real projection-fallback worker must start: {cause}");
    });
    let WorkerEvent::Message {
        token,
        value: worker::WorkerMessage::Profile(queued),
    } = worker.receiver.recv_timeout(WATCHDOG).unwrap()
    else {
        panic!("Completed science must be delivered despite projection pressure");
    };
    application.validate_callback(token).unwrap();
    assert_eq!(queued.job.status, ChannelJobStatus::Complete);
    assert!(queued.job.summary.comparison.bits_tested > 0);
    assert_eq!(queued.job.summary.comparison.bit_errors, 0);
    assert!(queued.backend().observation_stats().retained_events > 0);
    assert!(queued.observation_error.is_some());
    assert!(queued.observations.warnings.is_empty());
    assert!(queued.observations.tx.snapshots.is_empty());
    assert!(queued
        .observations
        .taps
        .iter()
        .all(|tap| tap.values.is_empty()));
    assert_eq!(
        queued.observations.fixture_resources(),
        RetainedResources::default()
    );
    assert_eq!(
        queued.observation_warnings(),
        serde_json::json!([queued.observation_error.unwrap()])
    );
    assert_eq!(
        budget.usage(),
        sum(
            sum(metadata, sum(accepted_charge, backend_charge)),
            worker::fixture_control_resources().0
        )
    );
    let report = serde_json::to_value(queued.backend().report()).unwrap();
    let current = Arc::new(*queued);
    let context = Context::for_application(application.clone()).unwrap();
    publish(&context, Some(Arc::clone(&current)));
    let mut model = Model::default();
    model.context = context.clone();
    let state = model.state();
    assert_eq!(state["reason"], current.observation_error.unwrap());
    assert_eq!(
        state["result"]["observation_warnings"],
        current.observation_warnings()
    );
    assert_eq!(state["has_result"], true);
    publish(&context, None);
    drop(model);
    drop(context);
    let old = Arc::clone(&current);
    drop(worker);
    wait_for(|| application.active().is_none());
    drop(accepted);
    assert_eq!(budget.usage(), sum(metadata, backend_charge));
    drop(current);
    assert_eq!(budget.usage(), sum(metadata, backend_charge));
    assert_eq!(
        serde_json::to_value(old.backend().report()).unwrap(),
        report
    );
    drop(old);
    assert_eq!(budget.usage(), metadata);
}
