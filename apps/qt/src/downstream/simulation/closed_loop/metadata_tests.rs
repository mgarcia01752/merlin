/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Inline Options children and guarded persistent receiver status, without DSP estimates.
use super::*;
use crate::shell::RetainedResources;
use merlin::downstream::{ChannelFilterSettings, EchoPath};
use std::mem::size_of;

// Independent spare buffers expose actual requested capacities, not growth policies.
const SELECTED_CAPACITY: usize = 7;
const ECHO_CAPACITY: usize = 5;
const SELECTED_ID: u8 = 1;
// Unity at zero delay is an identity-channel fixture, not a scientific limit.
const DIRECT_DELAY: usize = 0;
const UNITY_GAIN: [f64; 2] = [1.0, 0.0];
// Fixed zero-error thresholds belong only to the passive FIR variant audit.
const ZERO_FIR_ERROR: f64 = 0.0;
const MESSAGE: &str = "Original receiver admission detail.";
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};

fn context() -> Context {
    Context::for_application(crate::shell::ApplicationSessions::new(LIMITS).unwrap()).unwrap()
}

fn options() -> Options {
    let mut ids = Vec::with_capacity(SELECTED_CAPACITY);
    ids.push(SELECTED_ID);
    let mut paths = Vec::with_capacity(ECHO_CAPACITY);
    paths.push(EchoPath {
        delay_samples: DIRECT_DELAY,
        gain: UNITY_GAIN,
    });
    Options {
        selected_profile_ids: Some(ids),
        channel_filter: ChannelFilterSettings::Echoes { paths },
        ..Default::default()
    }
}

#[test]
fn options_count_each_actual_buffer_without_fixed_model_double_count_or_mutation() {
    let options = options();
    let before = serde_json::to_value(&options).unwrap();
    let ChannelFilterSettings::Echoes { paths } = &options.channel_filter else {
        unreachable!()
    };
    let expected = options.selected_profile_ids.as_ref().unwrap().capacity() * size_of::<u8>()
        + paths.capacity() * size_of::<EchoPath>();
    assert_eq!(
        metadata::options_children(&options).unwrap(),
        RetainedResources {
            payload_bytes: expected,
            ..Default::default()
        }
    );
    assert_eq!(serde_json::to_value(&options).unwrap(), before);
    let fixed = Options::default();
    assert_eq!(
        metadata::options_children(&fixed).unwrap(),
        RetainedResources::default()
    );
    let fir = Options {
        channel_filter: ChannelFilterSettings::CapturedFir {
            max_delay_samples: DIRECT_DELAY,
            causal_shift_samples: DIRECT_DELAY,
            maximum_rms_error: ZERO_FIR_ERROR,
            maximum_bin_error: ZERO_FIR_ERROR,
        },
        ..Default::default()
    };
    assert_eq!(
        metadata::options_children(&fir).unwrap(),
        RetainedResources::default()
    );
}

#[test]
fn zero_child_options_keep_host_identity_and_rejected_candidate_preserves_old_owner() {
    let foreign = context();
    let context = context();
    let budget = context.application.budget();
    let empty = budget.usage();
    let mut model = Model::for_context(context.clone()).unwrap();
    let baseline = budget.usage();
    model
        .prepare_options_for_budget(Some(Options::default()), true, budget)
        .unwrap();
    assert!(model.options_guard.is_some());
    assert_eq!(budget.usage(), baseline);
    assert!(model
        .check_metadata_target(foreign.application.budget())
        .is_err());
    model.check_metadata_target(budget).unwrap();
    let original = serde_json::to_value(&model.options).unwrap();
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: LIMITS.payload_bytes - budget.usage().payload_bytes,
            ..Default::default()
        })
        .unwrap();
    assert!(model
        .prepare_options_for_budget(Some(options()), false, budget)
        .is_err());
    assert_eq!(serde_json::to_value(&model.options).unwrap(), original);
    assert!(model.verify);
    model.check_metadata_target(budget).unwrap();
    drop(pressure);
    let candidate = options();
    let actual = metadata::options_children(&candidate).unwrap();
    model
        .prepare_options_for_budget(Some(candidate), false, budget)
        .unwrap();
    let mut expected = baseline;
    crate::downstream::inputs::retention::append(&mut expected, actual).unwrap();
    assert_eq!(budget.usage(), expected);
    drop(model);
    assert_eq!(budget.usage(), empty);
}

#[test]
fn persistent_guidance_survives_message_only_update_and_strict_foreign_rebind_rejection() {
    let source = context();
    let target = context();
    let mut model = Model::for_context(source.clone()).unwrap();
    model.error = WorkerFailure::try_admit(
        AdmissionError {
            message: MESSAGE.to_owned(),
            diagnostic: Some(Box::new(
                crate::downstream::help::catalog_diagnostic("QamRunUnavailable").unwrap(),
            )),
        },
        source.application.budget(),
    )
    .unwrap();
    let diagnostic = serde_json::to_value(model.error.diagnostic()).unwrap();
    model.set_static_message(MESSAGE);
    assert_eq!(
        serde_json::to_value(model.error.diagnostic()).unwrap(),
        diagnostic
    );
    let key = source
        .application
        .snapshot(crate::shell::Direction::Downstream)
        .key;
    let target_key = target
        .application
        .snapshot(crate::shell::Direction::Downstream)
        .key;
    let usage = source.application.budget().usage();
    let pressure = target
        .application
        .budget()
        .reserve_resources(RetainedResources {
            payload_bytes: LIMITS.payload_bytes - target.application.budget().usage().payload_bytes,
            ..Default::default()
        })
        .unwrap();
    assert!(model.rebind(target.clone()).is_err());
    assert_eq!(
        model
            .context
            .application
            .snapshot(crate::shell::Direction::Downstream)
            .key,
        key
    );
    assert_eq!(
        target
            .application
            .snapshot(crate::shell::Direction::Downstream)
            .key,
        target_key
    );
    assert_eq!(model.error.message(), MESSAGE);
    assert_eq!(
        serde_json::to_value(model.error.diagnostic()).unwrap(),
        diagnostic
    );
    assert_eq!(source.application.budget().usage(), usage);
    drop(pressure);
    model.rebind(target.clone()).unwrap();
    assert!(model.error.owns(target.application.budget()));
    assert!(!model.error.owns(source.application.budget()));
}
