/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Actual performance owner admission; independent fixtures are not aggregate acceptance.
use super::*;
use crate::downstream::instruments::receiver::performance;
use crate::downstream::workbench::session::Context;
use crate::shell::{ApplicationSessions, RetainedResources};
use merlin::downstream::{FullLoopControl, FullLoopRequest};
use serde_json::json;

// Existing supported small observation fixture; no new scientific execution limit.
const OBSERVATION_MS: u64 = 1;

// Independent accounting-only ceiling leaves arithmetic headroom for typed limit tests.
// It is neither a backend byte estimate nor production policy; the host supplies that policy.
const FIXTURE_LIMIT: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX / 2,
    ordered_entries: usize::MAX / 2,
    shared_roots: usize::MAX / 2,
    channel_count: usize::MAX / 2,
    slot_count: usize::MAX / 2,
    presentation_utf16_bytes: 0,
    presentation_entries: 0,
    retained_views: 0,
    open_views: 0,
};

fn request() -> FullLoopRequest {
    let mut document: Value = serde_json::from_slice(include_bytes!(
        "../../../../../../../examples/workbench/downstream/single/workbench.merlin.json"
    ))
    .unwrap();
    document["receiver"]["duration_ms"] = json!(OBSERVATION_MS);
    merlin::downstream::inputs::Workbench::prepare_full_loop(
        &serde_json::to_vec(&document).unwrap(),
    )
    .unwrap()
    .request(0)
    .unwrap()
}
fn result(cancelled: bool) -> ResultData {
    let request = request();
    let control = FullLoopControl::new();
    if cancelled {
        control.cancel();
    }
    ResultData::new(
        request
            .run_bounded(&control, request.result_retained_bound().unwrap())
            .unwrap(),
    )
}
fn profile(result: &ResultData) -> SourceProfileFrame {
    result
        .job
        .settings
        .playback
        .as_ref()
        .unwrap()
        .profile
        .clone()
}
fn context() -> Context {
    Context::for_application(ApplicationSessions::new(FIXTURE_LIMIT).unwrap()).unwrap()
}
fn candidate(result: &ResultData) -> PreparedEntry {
    PreparedEntry::from_result(result.backend().profile_performance().unwrap(), result)
}
fn target(result: &ResultData) -> RetainedResources {
    let budget = RetentionBudget::with_limits(FIXTURE_LIMIT);
    let entry = candidate(result).admit(&budget).unwrap();
    let required = budget.usage();
    drop(entry);
    assert_eq!(budget.usage(), RetainedResources::default());
    required
}
fn plus(a: RetainedResources, b: RetainedResources) -> RetainedResources {
    RetainedResources {
        payload_bytes: a.payload_bytes.checked_add(b.payload_bytes).unwrap(),
        ordered_entries: a.ordered_entries.checked_add(b.ordered_entries).unwrap(),
        shared_roots: a.shared_roots.checked_add(b.shared_roots).unwrap(),
        ..Default::default()
    }
}
fn normalized(value: &Value) {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
        Value::String(value) => assert_eq!(value.capacity(), value.len()),
        Value::Array(values) => {
            assert_eq!(values.capacity(), values.len());
            for value in values {
                normalized(value);
            }
        }
        Value::Object(values) => {
            for (key, value) in values {
                assert_eq!(key.capacity(), key.len());
                normalized(value);
            }
        }
    }
}

#[test]
fn final_capacity_and_nested_map_population_replace_the_wrapper_once_with_checked_overflow() {
    // Independent spare-capacity anchors exercise requested storage without a growth estimate.
    const SPARE_TEXT_BYTES: usize = 37;
    const SPARE_POINT_SLOTS: usize = 11;
    const SPARE_JSON_SLOTS: usize = 7;
    let mut error = String::with_capacity(SPARE_TEXT_BYTES);
    error.push_str("full diagnostic");
    let mut entry = PreparedEntry::failed(1, 2, error); // Independent captured ID/channel anchors.
    let mut status = String::with_capacity(SPARE_TEXT_BYTES);
    status.push_str("failed");
    entry.status = status;
    entry.points = Vec::with_capacity(SPARE_POINT_SLOTS);
    entry.points.push(Point {
        hz: 100_000_000, // Original center at 100 MHz; unrelated to admission policy.
        mer: None,
        ideal: None,
        samples: 0,
        plc: false,
    });
    let mut key = String::with_capacity(SPARE_TEXT_BYTES);
    key.push_str("items");
    let mut value = String::with_capacity(SPARE_TEXT_BYTES);
    value.push_str("nested text");
    let mut values = Vec::with_capacity(SPARE_JSON_SLOTS);
    values.push(Value::String(value));
    entry.fec_details = Some(Value::Object(
        [(key, Value::Array(values))].into_iter().collect(),
    ));
    let before = entry.resources().unwrap();
    let expected = RetainedResources {
        payload_bytes: size_of::<Entry>()
            + "failed".len()
            + size_of::<Point>()
            + "full diagnostic".len()
            + size_of::<(String, Value)>()
            + "items".len()
            + size_of::<Value>()
            + "nested text".len(),
        ordered_entries: 1,
        shared_roots: 0,
        ..Default::default()
    };
    assert!(before.payload_bytes > expected.payload_bytes);
    let budget = RetentionBudget::with_limits(expected);
    let entry = entry.admit(&budget).unwrap();
    assert_eq!(budget.usage(), expected);
    assert_eq!(entry.points.capacity(), entry.points.len());
    assert_eq!(entry.status.capacity(), entry.status.len());
    assert_eq!(
        entry.error.as_ref().unwrap().capacity(),
        entry.error.as_ref().unwrap().len()
    );
    normalized(entry.fec_details.as_ref().unwrap());
    drop(entry);
    assert_eq!(budget.usage(), RetainedResources::default());
    assert_eq!(add(usize::MAX, 1), Err(RetentionError::ArithmeticOverflow));
    assert_eq!(
        product(usize::MAX, size_of::<Value>()),
        Err(RetentionError::ArithmeticOverflow)
    );
    let mut full = RetainedResources {
        ordered_entries: usize::MAX,
        ..Default::default()
    };
    assert_eq!(
        append(
            &mut full,
            RetainedResources {
                ordered_entries: 1,
                ..Default::default()
            }
        ),
        Err(RetentionError::ArithmeticOverflow)
    );
}

#[test]
fn real_complete_and_precancelled_entries_preserve_science_nulls_coordinates_and_fec_json() {
    for cancelled in [false, true] {
        let result = result(cancelled);
        let profile = profile(&result);
        let source_before = result.backend().retained_footprint().unwrap();
        let expected = result.backend().profile_performance().unwrap();
        let expected_details = super::super::super::fec::snapshot(&result);
        let context = context();
        let budget = context.application.budget().clone();
        performance::begin(&context, std::slice::from_ref(&profile)).unwrap();
        performance::retain(&context, &profile, &result).unwrap();
        context.performance.with(|entries| {
            let entries = entries.borrow();
            let entry = &entries[0];
            assert_eq!(entry.id, expected.profile_id.value());
            assert_eq!(entry.channel, expected.key.channel_id);
            assert_eq!(
                entry.status,
                serde_json::to_value(expected.status)
                    .unwrap()
                    .as_str()
                    .unwrap()
            );
            assert_eq!(
                serde_json::to_value(&entry.fec).unwrap(),
                serde_json::to_value(&expected.fec).unwrap()
            );
            assert_eq!(entry.ber, expected.ber.map(|value| value.value()));
            assert_eq!(entry.pending, expected.unrecovered_generated_payloads);
            assert_eq!(entry.error, expected.error);
            assert_eq!(entry.fec_details.as_ref().unwrap(), &expected_details);
            assert_eq!(entry.points.len(), expected.points.len());
            for (actual, expected) in entry.points.iter().zip(&expected.points) {
                assert_eq!(actual.hz, expected.frequency_hz.value());
                assert_eq!(
                    actual.mer,
                    expected.measurement.mer().map(|value| value.value())
                );
                assert_eq!(
                    actual.ideal,
                    expected.ideal_shannon_mer_db.map(|value| value.value())
                );
                assert_eq!(actual.samples, expected.measurement.samples().value());
                assert_eq!(actual.plc, expected.role == ProfileCarrierRole::Plc);
            }
            normalized(entry.fec_details.as_ref().unwrap());
        });
        assert_eq!(
            performance::view(&context, i32::from(profile.profile_id), 0, 0)["total"],
            expected.points.len()
        );
        assert_eq!(
            result.backend().retained_footprint().unwrap(),
            source_before
        );
        drop(context);
        assert_eq!(
            budget.usage(),
            crate::shell::ApplicationSessions::fixture_counter_resources()
        );
    }
}

#[test]
fn each_positive_target_dimension_rejects_without_replacing_history_and_zero_roots_are_exact() {
    let result = result(false);
    let profile = profile(&result);
    let required = target(&result);
    assert!(required.payload_bytes > 0 && required.ordered_entries > 0);
    assert_eq!(
        required.shared_roots, 0,
        "No shared scientific/presentation payload Arc"
    );
    let exact = RetentionBudget::with_limits(required);
    let admitted = candidate(&result).admit(&exact).unwrap();
    assert_eq!(exact.usage(), required);
    drop(admitted);
    for ordered in [false, true] {
        let mut insufficient = required;
        if ordered {
            insufficient.ordered_entries -= 1;
        } else {
            insufficient.payload_bytes -= 1;
        }
        let smaller = RetentionBudget::with_limits(insufficient);
        let Err(error) = candidate(&result).admit(&smaller) else {
            panic!("One-unit target deficit must reject before adoption");
        };
        assert!(matches!(
            error,
            RetentionError::LimitExceeded { .. } | RetentionError::OrderedEntriesExceeded { .. }
        ));
        assert_eq!(smaller.usage(), RetainedResources::default());
        let context = context();
        let budget = context.application.budget().clone();
        performance::begin(&context, std::slice::from_ref(&profile)).unwrap();
        performance::retain(&context, &profile, &result).unwrap();
        let before = performance::view(&context, i32::from(profile.profile_id), 0, 0);
        let details = performance::fec_view(&context, i32::from(profile.profile_id));
        let source_before = result.backend().retained_footprint().unwrap();
        let current = budget.usage();

        // Explicit synthetic accounting pressure; no scientific population is estimated.
        let pressure = budget
            .reserve_resources(if ordered {
                RetainedResources {
                    ordered_entries: FIXTURE_LIMIT.ordered_entries - current.ordered_entries,
                    ..Default::default()
                }
            } else {
                RetainedResources {
                    payload_bytes: FIXTURE_LIMIT.payload_bytes - current.payload_bytes,
                    ..Default::default()
                }
            })
            .unwrap();
        let error = performance::retain(&context, &profile, &result).unwrap_err();
        assert!(matches!(
            error,
            RetentionError::LimitExceeded { .. } | RetentionError::OrderedEntriesExceeded { .. }
        ));
        assert_eq!(
            performance::view(&context, i32::from(profile.profile_id), 0, 0),
            before
        );
        assert_eq!(
            performance::fec_view(&context, i32::from(profile.profile_id)),
            details
        );
        assert_eq!(
            result.backend().retained_footprint().unwrap(),
            source_before
        );
        drop(pressure);
    }
}

#[test]
fn begin_admits_every_initial_entry_before_clearing_either_accepted_history() {
    let result = result(false);
    let profile = profile(&result);
    let context = context();
    let budget = context.application.budget().clone();
    performance::begin(&context, std::slice::from_ref(&profile)).unwrap();
    performance::retain(&context, &profile, &result).unwrap();
    let history =
        super::super::super::qam_history::get(&context, i32::from(profile.profile_id)).unwrap();
    let before = performance::fec_view(&context, i32::from(profile.profile_id));
    let before_usage = budget.usage();
    let independent = RetentionBudget::with_limits(FIXTURE_LIMIT);
    let initial = PreparedEntry::not_started(&profile)
        .admit(&independent)
        .unwrap();
    let initial_charge = independent.usage();
    drop(initial);
    let mut second = profile.clone();
    second.profile_id ^= 1; // Distinct byte identity only; neither fixture is scientifically run.
    let profiles = [profile.clone(), second];

    // Admit exactly one candidate, so rejection of the second must roll it back.
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: FIXTURE_LIMIT.payload_bytes
                - before_usage.payload_bytes
                - initial_charge.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    let with_pressure = budget.usage();
    assert!(matches!(
        performance::begin(&context, &profiles),
        Err(RetentionError::LimitExceeded { .. })
    ));
    assert_eq!(budget.usage(), with_pressure);
    assert_eq!(
        performance::fec_view(&context, i32::from(profile.profile_id)),
        before
    );
    let same =
        super::super::super::qam_history::get(&context, i32::from(profile.profile_id)).unwrap();
    assert!(std::sync::Arc::ptr_eq(&history, &same));
    drop(pressure);
    assert_eq!(budget.usage(), before_usage);
}

#[test]
fn rejected_setup_preserves_the_full_error_and_host_failure_keeps_its_prior_entry() {
    let context = context();
    let budget = context.application.budget().clone();
    let diagnostic = "Original setup error: exact text, null context, and Unicode λ.";
    let mut supplied = String::with_capacity(diagnostic.len() + 19); // Independent spare bytes.
    supplied.push_str(diagnostic);
    performance::reject(&context, 3, 4, supplied).unwrap(); // Original ID/channel test anchors.
    context.performance.with(|entries| {
        let entries = entries.borrow();
        let entry = &entries[0];
        assert_eq!(entry.status, "failed");
        assert_eq!(entry.error.as_deref(), Some(diagnostic));
        assert_eq!(entry.error.as_ref().unwrap().capacity(), diagnostic.len());
        assert!(entry.fec.is_none() && entry.fec_details.is_none() && entry.points.is_empty());
    });
    let before = performance::fec_view(&context, 3);
    assert_eq!(before["reason"], diagnostic);
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: FIXTURE_LIMIT.payload_bytes - budget.usage().payload_bytes,
            ..Default::default()
        })
        .unwrap();
    assert!(performance::reject(
        &context,
        3,
        4,
        "New error must not replace the prior entry".into()
    )
    .is_err());
    assert_eq!(performance::fec_view(&context, 3), before);
    drop(pressure);
    drop(context);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}

#[test]
fn retained_catalogs_have_no_spare_slots_after_every_runtime_mutation() {
    let context = context();
    let budget = context.application.budget().clone();
    let baseline = budget.usage();

    // Distinct original source identities exercise growth, replacement and clear.
    for id in [3, 4, 5] {
        performance::reject(&context, id, 1, "Original setup diagnostic.".into()).unwrap();
        let entries = context.performance.borrow();
        assert_eq!(entries.capacity(), entries.len());
        assert_eq!(
            super::super::registry_retention::resources(&entries).unwrap(),
            RetainedResources::default()
        );
    }
    performance::reject(&context, 4, 1, "Replacement diagnostic.".into()).unwrap();
    assert_eq!(
        context.performance.borrow()[1].error.as_deref(),
        Some("Replacement diagnostic.")
    );
    {
        let entries = context.performance.borrow();
        assert_eq!(entries.capacity(), entries.len());
    }
    performance::clear(&context);
    assert_eq!(context.performance.borrow().capacity(), 0);
    assert_eq!(budget.usage(), baseline);
    drop(context);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}

#[test]
fn late_qam_catalog_rejection_after_performance_admission_preserves_both_histories() {
    let context = context();
    let budget = context.application.budget().clone();
    let result = result(false);
    let profile = profile(&result);
    performance::begin(&context, std::slice::from_ref(&profile)).unwrap();
    performance::retain(&context, &profile, &result).unwrap();
    let old =
        super::super::super::qam_history::get(&context, i32::from(profile.profile_id)).unwrap();
    let old_fec = performance::fec_view(&context, i32::from(profile.profile_id));
    let before = budget.usage();
    let independent = RetentionBudget::with_limits(FIXTURE_LIMIT);
    let candidate = PreparedEntry::not_started(&profile)
        .admit(&independent)
        .unwrap();
    let performance_charge = independent.usage();
    drop(candidate);
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: FIXTURE_LIMIT.payload_bytes
                - before.payload_bytes
                - performance_charge.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    let pressured = budget.usage();
    assert!(performance::begin(&context, std::slice::from_ref(&profile)).is_err());
    assert_eq!(budget.usage(), pressured);
    assert_eq!(
        performance::fec_view(&context, i32::from(profile.profile_id)),
        old_fec
    );
    let current =
        super::super::super::qam_history::get(&context, i32::from(profile.profile_id)).unwrap();
    assert!(std::sync::Arc::ptr_eq(&old, &current));
    drop(pressure);
    assert_eq!(budget.usage(), before);
}

#[test]
fn admitted_candidate_overlap_replacement_clear_and_context_drop_release_concrete_storage() {
    let result = result(false);
    let context = context();
    let budget = context.application.budget().clone();
    let metadata = budget.usage();
    let required = target(&result);
    performance::insert(&context, candidate(&result).admit(&budget).unwrap());
    assert_eq!(budget.usage(), plus(metadata, required));
    let next = candidate(&result).admit(&budget).unwrap();
    assert_eq!(budget.usage(), plus(metadata, plus(required, required)));
    performance::insert(&context, next);
    assert_eq!(budget.usage(), plus(metadata, required));
    performance::clear(&context);
    assert_eq!(budget.usage(), metadata);
    performance::insert(&context, candidate(&result).admit(&budget).unwrap());
    assert_eq!(budget.usage(), plus(metadata, required));
    drop(context);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}
