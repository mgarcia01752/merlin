/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real result ownership plus separately labelled presentation selection fixtures.
use super::*;
use crate::shell::RetainedResources;
use merlin::downstream::{FullLoopControl, FullLoopRequest, FullLoopStageViewport};
use serde_json::Value;
use std::{
    mem::size_of,
    sync::{mpsc, Arc},
};

// The supported small closed-loop fixture; scheduling is bounded independently of science.
const OBSERVATION_MS: u64 = 1;
const MAX_RUNTIME_MS: u64 = 60000;
// Presentation scalar anchors; no scientific amplitude/normalization claim.
const FIXTURE_IQ: IqPair = [1.0, -1.0];

fn request() -> FullLoopRequest {
    let (inputs, transmission, sections) = super::super::tests::accepted_inputs();
    super::super::prepare(
        &transmission,
        &inputs,
        &super::super::Options {
            duration_ms: OBSERVATION_MS,
            max_runtime_ms: MAX_RUNTIME_MS,
            ..Default::default()
        },
        &sections,
    )
    .unwrap()
}

/// Executes the supported real small fixture for owner tests only.
///
/// # Returns
/// Actual bounded complete science; this source-only preparation has not run it.
///
/// # Panics
/// Fixture preparation, bound or execution failures violate the test expectation.
pub(super) fn result() -> FullLoopResult {
    let request = request();
    request
        .run_bounded(
            &FullLoopControl::new(),
            request.result_retained_bound().unwrap(),
        )
        .unwrap()
}

fn policy() -> RetentionBudget {
    super::super::result_retention::fixture_budget()
}

fn plus(a: RetainedResources, b: RetainedResources) -> RetainedResources {
    RetainedResources {
        payload_bytes: a.payload_bytes.checked_add(b.payload_bytes).unwrap(),
        ordered_entries: a.ordered_entries.checked_add(b.ordered_entries).unwrap(),
        shared_roots: a.shared_roots.checked_add(b.shared_roots).unwrap(),
        ..Default::default()
    }
}

fn json_children(value: &Value) -> RetainedResources {
    match value {
        Value::String(value) => {
            assert_eq!(value.capacity(), value.len());
            RetainedResources {
                payload_bytes: value.capacity(),
                ..Default::default()
            }
        }
        Value::Array(values) => {
            assert_eq!(values.capacity(), values.len());
            values.iter().fold(
                RetainedResources {
                    payload_bytes: values.capacity() * size_of::<Value>(),
                    ..Default::default()
                },
                |sum, value| plus(sum, json_children(value)),
            )
        }
        Value::Object(values) => values.iter().fold(
            RetainedResources {
                payload_bytes: values.len() * size_of::<(String, Value)>(),
                ordered_entries: values.len(),
                shared_roots: 0,
                ..Default::default()
            },
            |sum, (key, value)| {
                assert_eq!(key.capacity(), key.len());
                plus(
                    plus(
                        sum,
                        RetainedResources {
                            payload_bytes: key.capacity(),
                            ..Default::default()
                        },
                    ),
                    json_children(value),
                )
            },
        ),
        Value::Null | Value::Bool(_) | Value::Number(_) => RetainedResources::default(),
    }
}

fn vector_bytes<T>(value: &Vec<T>) -> usize {
    assert_eq!(value.len(), value.capacity());
    value.capacity() * size_of::<T>()
}

pub(super) fn measured(output: &Observations) -> RetainedResources {
    let Observations {
        tx,
        taps,
        responses,
        decoder_llrs,
        decoder_word: _,
        warnings,
        retained: _,
    } = output;
    let crate::transmitter_instruments::capture::Capture {
        snapshots,
        waves,
        frames,
    } = tx;
    let mut resources = RetainedResources {
        payload_bytes: snapshots.len()
            * size_of::<(Stage, crate::transmitter_instruments::capture::Snapshot)>(),
        ordered_entries: snapshots.len(),
        shared_roots: 0,
        ..Default::default()
    };
    for snapshot in snapshots.values() {
        let crate::transmitter_instruments::capture::Snapshot {
            data,
            total: _,
            offset: _,
            symbol: _,
            phase,
            source_symbol: _,
        } = snapshot;
        assert_eq!(phase.capacity(), phase.len());
        resources.payload_bytes += phase.capacity();
        resources = plus(resources, json_children(data));
    }
    resources.payload_bytes += waves.iter().map(vector_bytes).sum::<usize>();
    resources.payload_bytes += vector_bytes(frames);
    for (_, values) in frames {
        resources.payload_bytes += vector_bytes(values);
    }
    for Samples { values, next: _ } in taps {
        resources.payload_bytes += vector_bytes(values);
    }
    resources.payload_bytes += vector_bytes(responses);
    for (_, values) in responses {
        resources.payload_bytes += vector_bytes(values);
    }
    resources.payload_bytes += vector_bytes(decoder_llrs) + vector_bytes(warnings);
    for warning in warnings {
        assert_eq!(warning.len(), warning.capacity());
        resources.payload_bytes += warning.capacity();
    }
    resources
}

fn planned(result: &FullLoopResult) -> (RetainedResources, RetainedResources) {
    let budget = policy();
    let queries = queries::Queries::new(result, &budget).unwrap();
    let workspace = budget.usage();
    let mut plan = plan::Plan::default();
    queries
        .visit(|event| match event {
            Ok(event) => plan.observe(event),
            Err(error) => plan.warning(error.message()),
        })
        .unwrap();
    (workspace, plan.resources().unwrap())
}

#[test]
fn real_projection_matches_planned_final_storage_and_keeps_original_science_and_coordinates() {
    let result = result();
    let before = result.retained_footprint().unwrap();
    let report = serde_json::to_value(result.report()).unwrap();
    let (_, target) = planned(&result);
    let budget = policy();
    let output = Observations::from_result_admitted(&result, &budget).unwrap();
    assert_eq!(measured(&output), target);
    assert_eq!(
        budget.usage(),
        target,
        "Query slots release before returned target escapes"
    );
    assert!(!output.tx.snapshots.is_empty());
    assert!(!output.tx.waves.last().unwrap().is_empty());
    assert!(!output.taps[0].values.is_empty());
    let mut response_count = 0;
    let mut decoder_seen = false;
    for info in result.stage_inventory() {
        if !matches!(
            info.stage,
            Stage::RxChannelEstimate | Stage::PayloadLdpcInput
        ) {
            continue;
        }
        for index in 0..info.snapshots {
            let event = result
                .stage(info.stage, index, FullLoopStageViewport::default())
                .unwrap();
            if event.context.phase != FullLoopStagePhase::Observation {
                continue;
            }
            match &event.data {
                Data::Iq(_) if info.stage == Stage::RxChannelEstimate => response_count += 1,
                Data::Llrs(values) if !decoder_seen && !values.is_empty() => {
                    assert_eq!(&output.decoder_llrs[..values.len()], values);
                    assert_eq!(output.decoder_word, event.context.codeword_index);
                    decoder_seen = true;
                }
                _ => {}
            }
        }
    }
    assert_eq!(output.responses.len(), response_count);
    assert_eq!(output.decoder_llrs.is_empty(), !decoder_seen);
    for (stage, snapshot) in &output.tx.snapshots {
        let first = result
            .stage(*stage, 0, FullLoopStageViewport::default())
            .unwrap();
        assert_eq!(snapshot.total, first.total_elements);
        assert_eq!(snapshot.offset, first.element_start);
        assert_eq!(snapshot.symbol, first.context.channel_symbol_index);
        assert_eq!(snapshot.source_symbol, first.context.source_symbol_index);
        assert_eq!(snapshot.phase, format!("{:?}", first.context.phase));
        let direct = serde_json::to_value(&first.data).unwrap();
        let first_values = direct["values"].as_array().unwrap();
        assert_eq!(
            &snapshot.data["values"].as_array().unwrap()[..first_values.len()],
            first_values
        );
    }
    assert_eq!(result.retained_footprint().unwrap(), before);
    assert_eq!(serde_json::to_value(result.report()).unwrap(), report);
    drop(output);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn exact_overlap_and_dimension_rejection_preserve_source_ownership() {
    for rejection in [None, Some("payload"), Some("entries")] {
        let result = result();
        let before = result.retained_footprint().unwrap();
        let (workspace, target) = planned(&result);
        assert!(target.payload_bytes > 0 && target.ordered_entries > 0);
        assert_eq!(
            target.shared_roots, 0,
            "Owned DTOs introduce no scientific Arc"
        );
        let source_charge = RetainedResources {
            payload_bytes: before.payload_bytes + size_of::<Retained<FullLoopResult>>()
                - size_of::<FullLoopResult>(),
            ordered_entries: before.ordered_entries,
            shared_roots: before.shared_roots,
            ..Default::default()
        };
        let exact = plus(source_charge, plus(workspace, target));
        let maximum = match rejection {
            Some("payload") => RetainedResources {
                payload_bytes: exact.payload_bytes - 1,
                ..exact
            },
            Some("entries") => RetainedResources {
                ordered_entries: exact.ordered_entries - 1,
                ..exact
            },
            _ => exact,
        };
        let budget = RetentionBudget::with_limits(maximum);
        let source = budget
            .reserve_resources(source_charge)
            .unwrap()
            .retain(result);
        match Observations::from_result_admitted(source.value(), &budget) {
            Ok(output) => {
                assert!(rejection.is_none());
                assert_eq!(budget.usage(), plus(source_charge, target));
                drop(output);
            }
            Err(ObservationError::Retention(_)) => assert!(rejection.is_some()),
            Err(error) => panic!("Unexpected actual projection error: {error:?}"),
        }
        assert_eq!(budget.usage(), source_charge);
        assert_eq!(source.value().retained_footprint().unwrap(), before);
        assert_eq!(source.value().report().summary.comparison.bit_errors, 0);
        drop(source);
        assert_eq!(budget.usage(), RetainedResources::default());
    }
}

#[test]
fn queued_current_and_old_aliases_keep_dynamic_projection_charge_until_final_drop() {
    let result = result();
    let budget = policy();
    // This fixture separately admits its final shared wrapper, as ResultData does
    // in production; the projection guard owns dynamic children only.
    let wrapper = RetainedResources {
        payload_bytes: size_of::<Retained<Observations>>(),
        ordered_entries: 0,
        shared_roots: 1,
        ..Default::default()
    };
    let reserved = budget.reserve_resources(wrapper).unwrap();
    let output = Observations::from_result_admitted(&result, &budget).unwrap();
    let target = measured(&output);
    let current = Arc::new(reserved.retain(output));
    let old = Arc::clone(&current);
    let (sender, receiver) = mpsc::sync_channel(1);
    sender.send(Arc::clone(&current)).unwrap();
    drop(current);
    drop(old);
    assert_eq!(budget.usage(), plus(wrapper, target));
    let queued = receiver.recv().unwrap();
    assert!(!queued.value().tx.snapshots.is_empty());
    drop(queued);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn presentation_selection_fixture_preserves_first_contiguous_phase_frame_and_decoder_rules() {
    let result = result();
    let mut event = result
        .stage(Stage::ChannelOutput, 0, FullLoopStageViewport::default())
        .unwrap();
    // Presentation-only synthetic event sequence: original-coordinate, discontinuity,
    // repeated decoder and frame selection. It does not validate any engine behavior.
    let mut events = Vec::new();
    event.context.phase = FullLoopStagePhase::Observation;
    event.context.sample_start_index = Some(0);
    event.data = Data::Iq(vec![FIXTURE_IQ]);
    events.push(event.clone());
    event.context.sample_start_index = Some(2); // One missing original sample: do not concatenate.
    events.push(event.clone());
    event.stage = Stage::RxUseful;
    event.context.sample_start_index = Some(0);
    events.push(event.clone());
    events.push(event.clone()); // First event only.
    event.stage = Stage::RxChannelEstimate;
    events.push(event.clone());
    event.stage = Stage::PayloadLdpcInput;
    event.data = Data::Llrs(vec![-1.0]);
    event.context.codeword_index = Some(1); // Fixture's first codeword identity.
    events.push(event.clone());
    event.context.codeword_index = Some(2); // Later codeword must not replace the first.
    events.push(event.clone());
    event.stage = Stage::PlcMessage;
    event.data = Data::Bytes(vec![0]);
    event.element_start = 0;
    event.total_elements = 1; // One-byte complete presentation frame.
    event.context.channel_symbol_index = Some(1);
    events.push(event.clone());
    event.element_start = 1; // Incomplete slice must not become a complete PLC frame.
    events.push(event);
    let mut plan = plan::Plan::default();
    for event in &events {
        plan.observe(event).unwrap();
    }
    plan.warning("Original query warning").unwrap();
    let budget = policy();
    let reservation = budget.reserve_resources(plan.resources().unwrap()).unwrap();
    let mut output = Observations::default();
    for event in &events {
        output.observe(event);
    }
    output.warnings.push("Original query warning".into());
    output.normalize();
    output.retained = Some(reservation.retain(()));
    assert_eq!(output.taps[0].values, [FIXTURE_IQ]);
    assert_eq!(output.taps[2].values, [FIXTURE_IQ]);
    assert_eq!(output.responses.len(), 1);
    assert_eq!(output.decoder_word, Some(1));
    assert_eq!(output.decoder_llrs, [-1.0]);
    assert_eq!(output.tx.frames, [(1, vec![0])]);
    assert_eq!(output.warnings, ["Original query warning"]);
    assert_eq!(measured(&output), plan.resources().unwrap());
}

#[test]
fn precancelled_public_result_keeps_its_actual_empty_projection_without_fabricated_values() {
    let request = request();
    let control = FullLoopControl::new();
    control.cancel();
    let result = request
        .run_bounded(&control, request.result_retained_bound().unwrap())
        .unwrap();
    let before = serde_json::to_value(result.report()).unwrap();
    assert!(result.stage_inventory().is_empty());
    let budget = policy();
    let output = Observations::from_result_admitted(&result, &budget).unwrap();
    assert_eq!(measured(&output), RetainedResources::default());
    assert_eq!(budget.usage(), RetainedResources::default());
    assert!(output.warnings.is_empty());
    assert!(output.decoder_word.is_none());
    assert_eq!(serde_json::to_value(result.report()).unwrap(), before);
}
