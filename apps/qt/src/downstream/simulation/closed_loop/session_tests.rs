/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual direction consumers sharing canonical application admission.
use super::*;
use crate::shell::{DispatchPhase, WorkerStatus};
use crate::upstream::{ControllerEvent, PreparedSession, UpstreamControl, UpstreamController};
use merlin::upstream::{ConfigurationDocument, ReceiverObservationPolicy, RunControl, RunStatus};
use std::time::{Duration, Instant};

// Scheduling limits are independent of simulated duration and active execution.
const ASYNC_DEADLINE: Duration = Duration::from_secs(20);
const POLL_INTERVAL: Duration = Duration::from_millis(5);
const LONG_SOURCE_DURATION_MS: u64 = 1000;
const ACTIVE_EXECUTION_LIMIT_MS: u64 = 60000;
const UPSTREAM_COMPLETION_CAPACITY: usize = 2;
const DISABLED_ESTIMATE_FRAMES: usize = 0;
const DISABLED_ESTIMATE_POINTS: usize = 0;
const UPSTREAM_DOCUMENT: &[u8] =
    include_bytes!("../../../../../../examples/cli/upstream/us-50khz.json");

fn prepared_model(context: Context) -> Model {
    prepared_model_with_duration(context, LONG_SOURCE_DURATION_MS)
}

fn prepared_model_with_duration(context: Context, duration_ms: u64) -> Model {
    let (inputs, transmission, sections) = super::tests::accepted_inputs();
    crate::downstream::workbench::session::restore(&context, inputs.clone(), sections.clone());
    let mut model = Model::default();
    model.context = context;
    model.options = Some(Options {
        duration_ms,
        max_runtime_ms: ACTIVE_EXECUTION_LIMIT_MS,
        ..Default::default()
    });
    model.verify = true;
    model.prepare_inputs(
        &serde_json::to_string(&transmission).unwrap(),
        &inputs,
        &sections,
    );
    assert!(model.request.is_some(), "{}", model.state());
    model
}

#[test]
fn rejected_host_metadata_preserves_the_accepted_real_report_and_request() {
    // Explicit test-only ceiling plus synthetic pressure checks transactionality,
    // never a scientific payload-size estimate or an application policy default.
    const TEST_HOST_PAYLOAD_LIMIT: usize = usize::MAX;
    const COMPLETE_SOURCE_DURATION_MS: u64 = 1;
    let mut context = Context::isolated();
    context.application = crate::shell::ApplicationSessions::new(crate::shell::RetainedResources {
        payload_bytes: TEST_HOST_PAYLOAD_LIMIT,
        ordered_entries: usize::MAX,
        shared_roots: usize::MAX,
    })
    .unwrap();
    let mut model = prepared_model_with_duration(context.clone(), COMPLETE_SOURCE_DURATION_MS);
    model.start();
    wait_for(|| model.poll()["busy"] == false);
    let report = latest(&context).expect("The accepted real run must complete");
    assert_eq!(
        report.job.status,
        merlin::downstream::ChannelJobStatus::Complete
    );
    let key = model.generation;
    let request = serde_json::to_value(model.request.as_ref().unwrap().value().settings()).unwrap();
    let accepted_options = serde_json::to_value(&model.options).unwrap();
    let verified = model.verify;
    let usage = context.application.budget().usage();
    let pressure = context
        .application
        .budget()
        .reserve_resources(crate::shell::RetainedResources {
            payload_bytes: TEST_HOST_PAYLOAD_LIMIT - usage.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    // Same fallible adoption used by Qt Apply and fallback Restore; the changed
    // two-millisecond candidate and disabled comparison must never leak on failure.
    const REJECTED_SOURCE_DURATION_MS: u64 = 2;
    let mut changed = model.options.clone().unwrap();
    changed.duration_ms = REJECTED_SOURCE_DURATION_MS;
    let settings = serde_json::to_string(
        &model
            .request
            .as_ref()
            .unwrap()
            .value()
            .settings()
            .transmission,
    )
    .unwrap();
    let transmission = serde_json::from_str::<TransmissionSettings>(&settings).unwrap();
    prepare_sweep(
        &transmission,
        &crate::downstream::workbench::session::snapshot(&context),
        &changed,
        &crate::downstream::workbench::session::saved_sections(&context),
    )
    .unwrap();
    assert!(model
        .accept_options(Some(changed.clone()), false, &settings)
        .is_err());
    assert_eq!(
        serde_json::to_value(&model.options).unwrap(),
        accepted_options
    );
    assert_eq!(model.verify, verified);
    assert!(!model.invalidate());
    assert_eq!(model.generation, key);
    assert_eq!(
        serde_json::to_value(model.request.as_ref().unwrap().value().settings()).unwrap(),
        request
    );
    assert!(Arc::ptr_eq(&latest(&context).unwrap(), &report));
    assert!(context.application.active().is_none());
    assert!(model.error.contains("retained-data limit"));
    drop(pressure);
    model
        .accept_options(Some(changed), false, &settings)
        .unwrap();
    assert_eq!(
        model.options.as_ref().unwrap().duration_ms,
        REJECTED_SOURCE_DURATION_MS
    );
    assert!(!model.verify);
    assert!(model.request.is_some());
    assert_ne!(model.generation, key);
    assert!(model.invalidate());
    assert!(model.request.is_none());
    assert!(latest(&context).is_none());
}

fn wait_for(mut ready: impl FnMut() -> bool) {
    let deadline = Instant::now() + ASYNC_DEADLINE;
    while !ready() {
        assert!(
            Instant::now() < deadline,
            "Application lifecycle did not terminate"
        );
        std::thread::sleep(POLL_INTERVAL);
    }
}

#[test]
fn actual_engines_share_paused_and_cancelling_admission_without_overwriting_results() {
    let context = Context::isolated();
    let application = context.application.clone();
    let mut downstream = prepared_model(context.clone());
    let prepared = PreparedSession::from_document(
        ConfigurationDocument::from_json(UPSTREAM_DOCUMENT).unwrap(),
        ReceiverObservationPolicy::new(DISABLED_ESTIMATE_FRAMES, DISABLED_ESTIMATE_POINTS).unwrap(),
        application.budget(),
    )
    .unwrap();
    let key = application.configure_upstream(true).unwrap();
    let mut upstream = UpstreamController::new(key, prepared).unwrap();
    let target = upstream.prepare_run().unwrap();
    let backend = RunControl::new();
    backend.pause();
    let control = Arc::new(UpstreamControl::new(backend.clone()));

    downstream.start();
    downstream.pause();
    wait_for(|| {
        downstream.poll();
        application
            .active()
            .is_some_and(|run| run.phase == DispatchPhase::Paused)
    });
    assert!(matches!(
        application.begin(key, control.clone()),
        Err(ApplicationError::Session(SessionError::Dispatch(DispatchError::Busy(active))))
            if active.token.key.direction == Direction::Downstream
    ));
    assert_eq!(backend.progress().status, RunStatus::Idle);
    downstream.stop();
    assert_eq!(
        application.active().unwrap().phase,
        DispatchPhase::Cancelling
    );
    assert!(application.begin(key, control.clone()).is_err());
    wait_for(|| downstream.poll()["busy"] == false);
    assert!(application.active().is_none());
    let old_downstream =
        latest(&context).expect("Cancellation preserves its actual partial report");
    assert_eq!(
        old_downstream.job.status,
        merlin::downstream::ChannelJobStatus::Cancelled
    );

    let lease = application.begin(key, control.clone()).unwrap();
    let token = lease.token();
    if let Err(error) = upstream.start(lease, control, target, UPSTREAM_COMPLETION_CAPACITY) {
        let (reason, lease) = error.into_parts();
        application.finish(lease).unwrap();
        panic!("Real upstream admission rejected: {reason:?}");
    }
    wait_for(|| {
        upstream
            .progress()
            .is_some_and(|(_, snapshot)| snapshot.status == RunStatus::Paused)
    });
    application.acknowledge_pause(token, true).unwrap();
    assert_eq!(downstream.state()["application_busy"], true);
    assert_eq!(downstream.state()["active_direction"], "upstream");
    assert_eq!(downstream.state()["can_start"], false);
    downstream.start();
    assert!(downstream.worker.is_none());
    assert!(Arc::ptr_eq(&latest(&context).unwrap(), &old_downstream));
    assert!(downstream
        .error
        .contains("upstream simulation is already active"));
    assert_eq!(backend.progress().status, RunStatus::Paused);

    application.cancel(token).unwrap();
    assert_eq!(
        application.active().unwrap().phase,
        DispatchPhase::Cancelling
    );
    let mut actual_report = None;
    wait_for(|| match upstream.poll().unwrap() {
        ControllerEvent::Idle | ControllerEvent::Pending => false,
        ControllerEvent::Report {
            token: report_token,
            report,
        } => {
            assert_eq!(report_token, token);
            application.validate_callback(report_token).unwrap();
            assert_eq!(report.value().status, RunStatus::Cancelled);
            assert!(application.active().is_some());
            actual_report = Some(report);
            false
        }
        ControllerEvent::Terminal(terminal) => {
            assert_eq!(terminal.token(), token);
            assert!(matches!(terminal.status(), WorkerStatus::Completed));
            application.finish(terminal.into_lease()).unwrap();
            true
        }
    });
    assert!(actual_report.is_some());
    assert!(application.active().is_none());
    assert!(Arc::ptr_eq(&latest(&context).unwrap(), &old_downstream));
}

#[test]
fn canonical_generation_rejects_an_old_model_and_rebound_projects_use_the_target_host() {
    let target = Context::isolated();
    let mut old = prepared_model(target.clone());
    let old_key = old.generation.unwrap();
    let mut candidate = prepared_model(Context::isolated());
    assert_ne!(candidate.generation.unwrap().session, old_key.session);
    candidate.rebind(target.clone()).unwrap();
    assert_eq!(
        candidate.generation,
        Some(target.application.snapshot(Direction::Downstream).key)
    );
    assert_ne!(candidate.generation, Some(old_key));
    old.start();
    assert!(old.worker.is_none());
    assert!(target.application.active().is_none());
    assert!(old.error.contains("Configuration changed"));
    assert_eq!(old.state()["can_start"], false);
    candidate.start();
    candidate.pause();
    wait_for(|| {
        candidate.poll();
        target
            .application
            .active()
            .is_some_and(|run| run.phase == DispatchPhase::Paused)
    });
    let retired = target.application.active().unwrap().token;
    candidate.invalidate();
    assert!(target.application.validate_callback(retired).is_err());
    assert_eq!(
        target.application.active().unwrap().phase,
        DispatchPhase::Cancelling
    );
    wait_for(|| candidate.poll()["busy"] == false);
    assert!(target.application.active().is_none());
    assert!(latest(&target).is_none());
}

#[test]
fn destroyed_downstream_adapter_cancels_its_actual_worker_before_releasing_admission() {
    let context = Context::isolated();
    let application = context.application.clone();
    let mut model = prepared_model(context.clone());
    model.start();
    model.pause();
    wait_for(|| {
        model.poll();
        application
            .active()
            .is_some_and(|run| run.phase == DispatchPhase::Paused)
    });
    let token = application.active().unwrap().token;
    drop(model);
    // Known-terminal disposal runs on the shared transport after cancellation;
    // no GUI join or synthetic early acknowledgement is used.
    wait_for(|| application.active().is_none());
    assert!(application.validate_callback(token).is_err());
    assert!(latest(&context).is_none());
}
