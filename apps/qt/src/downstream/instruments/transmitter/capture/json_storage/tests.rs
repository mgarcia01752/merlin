/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Independent final-Value measurements; these fixtures represent presentation, not science.
use super::*;
use merlin::downstream::{FullLoopPilotValue, FullLoopTransportCell};

// Spare slots deliberately exceed the fixed two-value fixture, independently of DSP limits.
const SPARE_ELEMENT_SLOTS: usize = 17;
// Presentation scalar anchors; no scientific amplitude/normalization claim.
const FIXTURE_IQ: [f64; 2] = [1.0, -1.0];

fn spare<T>(values: impl IntoIterator<Item = T>) -> Vec<T> {
    let mut output = Vec::with_capacity(SPARE_ELEMENT_SLOTS);
    output.extend(values);
    output
}

fn measured(value: &Value) -> RetainedResources {
    match value {
        Value::String(value) => {
            assert_eq!(value.len(), value.capacity());
            RetainedResources {
                payload_bytes: value.capacity(),
                ..Default::default()
            }
        }
        Value::Array(values) => {
            assert_eq!(values.len(), values.capacity());
            let mut result = RetainedResources {
                payload_bytes: values.capacity() * size_of::<Value>(),
                ..Default::default()
            };
            for value in values {
                let child = measured(value);
                result.payload_bytes += child.payload_bytes;
                result.ordered_entries += child.ordered_entries;
            }
            result
        }
        Value::Object(values) => {
            let mut result = RetainedResources {
                payload_bytes: values.len() * size_of::<(String, Value)>(),
                ordered_entries: values.len(),
                shared_roots: 0,
            };
            for (key, value) in values {
                assert_eq!(key.len(), key.capacity());
                let child = measured(value);
                result.payload_bytes += key.capacity() + child.payload_bytes;
                result.ordered_entries += child.ordered_entries;
            }
            result
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => RetainedResources::default(),
    }
}

#[test]
fn every_current_tagged_variant_matches_exact_json_storage_without_changing_nulls_or_values() {
    let variants = [
        ProbeData::Bits(spare([0, 1])),
        ProbeData::Bytes(spare([0, u8::MAX])),
        ProbeData::Iq(spare([FIXTURE_IQ])),
        ProbeData::OptionalIq(spare([None, Some(FIXTURE_IQ)])),
        ProbeData::QamLabels(spare([0, u16::MAX])),
        ProbeData::TransportCells(spare([
            None,
            Some(FullLoopTransportCell::PilotPlaceholder),
            Some(FullLoopTransportCell::Sample(FIXTURE_IQ)),
        ])),
        // First physical bin with a zero pilot-sequence bit: scalar JSON fields.
        ProbeData::Pilots(spare([FullLoopPilotValue {
            bin: 0,
            sequence_bit: 0,
            iq: FIXTURE_IQ,
        }])),
        // Nonfinite serialization stays serde_json's existing explicit null behavior.
        ProbeData::Llrs(spare([f64::NAN, -1.0])),
    ];
    for data in variants {
        let expected = serde_json::to_value(&data).unwrap();
        let plan = children(&data).unwrap();
        let mut actual = serde_json::to_value(&data).unwrap();
        normalize(&mut actual);
        assert_eq!(actual, expected);
        assert_eq!(measured(&actual), plan);
        assert_eq!(plan.shared_roots, 0);
        match data {
            ProbeData::Bits(values) | ProbeData::Bytes(values) => {
                assert_eq!(values.capacity(), SPARE_ELEMENT_SLOTS)
            }
            _ => {}
        }
    }
}

#[test]
fn json_population_and_buffer_overflow_reject_before_any_target_is_built() {
    assert!(matches!(
        array(usize::MAX),
        Err(PlanError::Retention(RetentionError::ArithmeticOverflow))
    ));
    assert!(matches!(
        combine(
            RetainedResources {
                ordered_entries: usize::MAX,
                ..Default::default()
            },
            RetainedResources {
                ordered_entries: 1,
                ..Default::default()
            }
        ),
        Err(PlanError::Retention(RetentionError::ArithmeticOverflow))
    ));
    assert!(matches!(
        combine(
            RetainedResources {
                payload_bytes: usize::MAX,
                ..Default::default()
            },
            RetainedResources {
                payload_bytes: 1,
                ..Default::default()
            }
        ),
        Err(PlanError::Retention(RetentionError::ArithmeticOverflow))
    ));
}

#[test]
fn planned_phase_text_preserves_the_existing_debug_contract_for_all_phases() {
    use merlin::downstream::FullLoopStagePhase;
    for phase in [
        FullLoopStagePhase::Warmup,
        FullLoopStagePhase::Observation,
        FullLoopStagePhase::Drain,
    ] {
        assert_eq!(super::super::plan::phase(phase), format!("{phase:?}"));
    }
}
