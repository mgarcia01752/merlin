/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Queued receiver metadata is admitted on its exact target before project adoption.
use super::*;
use crate::{
    downstream::workbench::session,
    shell::{ApplicationSessions, Direction, RetainedResources},
};
use merlin::downstream::{ChannelFilterSettings, EchoPath};

const PROJECT: &[u8] =
    include_bytes!("../../../../../../examples/workbench/single/workbench.merlin.json");
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
};
// Source-side spare slots make the fixture independent of Clone capacity.
const ECHO_CAPACITY: usize = 5;
const DIRECT_DELAY: usize = 0;
const UNITY_GAIN: [f64; 2] = [1.0, 0.0];

fn context() -> Context {
    Context::for_application(ApplicationSessions::new(LIMITS).unwrap()).unwrap()
}

#[test]
fn prepared_options_are_target_guarded_and_foreign_zero_child_witness_rejects_before_adoption() {
    let context = context();
    let foreign = Context::for_application(ApplicationSessions::new(LIMITS).unwrap()).unwrap();
    let baseline = context.application.budget().usage();
    let mut project: Project = serde_json::from_slice(PROJECT).unwrap();
    let mut paths = Vec::with_capacity(ECHO_CAPACITY);
    paths.push(EchoPath {
        delay_samples: DIRECT_DELAY,
        gain: UNITY_GAIN,
    });
    project.receiver.as_mut().unwrap().channel_filter = ChannelFilterSettings::Echoes { paths };
    let original_options = serde_json::to_value(&project.receiver).unwrap();
    let target = context.application.snapshot(Direction::Downstream);
    let saved = session::saved_sections(&context);
    let mut prepared = PreparedProject::prepare(
        project,
        None,
        &context.application,
        target.key,
        session::snapshot(&context),
    )
    .unwrap();
    let receiver = prepared.models.receiver.as_ref().unwrap();
    receiver
        .check_prepared_target(context.application.budget())
        .unwrap();
    assert_eq!(
        serde_json::to_value(&receiver.options).unwrap(),
        original_options
    );
    assert_eq!(prepared.metadata.value()["receiver"], original_options);

    // A zero-child Options value still has a real foreign host identity. The
    // genuine project adoption checks it before accepted Inputs/readiness swap.
    let mut replacement = Model::for_context(foreign.clone()).unwrap();
    replacement
        .prepare_options_for_budget(
            Some(Default::default()),
            false,
            foreign.application.budget(),
        )
        .unwrap();
    prepared.models.receiver = Some(replacement);
    assert!(prepared.adopt(&context).is_err());
    assert_eq!(context.application.snapshot(Direction::Downstream), target);
    assert_eq!(&*session::saved_sections(&context), &*saved);
    assert!(context.prepared.borrow().receiver.is_none());
    assert_eq!(context.application.budget().usage(), baseline);
}
