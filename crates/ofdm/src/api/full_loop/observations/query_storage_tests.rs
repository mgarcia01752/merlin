/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Storage fixtures exercise representations, not scientific stage acceptance.
use super::*;
use crate::api::full_loop::{FullLoopCellRole, FullLoopPilotValue, FullLoopTransportCell};
use crate::simulation::test_points::RunPhase;
use merlin_api::retention_support::retained_footprint;
use std::mem::{size_of, size_of_val};

// Independent requested spares distinguish data, text and source snapshot slots.
const DATA_SPARE_SLOTS: usize = 13;
const CONTEXT_SPARE_BYTES: usize = 37;
const SNAPSHOT_SPARE_SLOTS: usize = 7;

fn spare<T>(values: impl IntoIterator<Item = T>) -> Vec<T> {
    let mut output = Vec::with_capacity(DATA_SPARE_SLOTS);
    output.extend(values);
    output
}

fn data_variants() -> [FullLoopStageData; 8] {
    [
        FullLoopStageData::Bits(spare([0, 1, 0])),
        FullLoopStageData::Bytes(spare([2, 0, 3])),
        FullLoopStageData::Iq(spare([[1.0, -1.0], [0.0, 0.0], [2.0, -3.0]])),
        FullLoopStageData::OptionalIq(spare([None, Some([0.0, 0.0]), Some([2.0, -1.0])])),
        FullLoopStageData::QamLabels(spare([3, 0, 7])),
        FullLoopStageData::TransportCells(spare([
            None,
            Some(FullLoopTransportCell::PilotPlaceholder),
            Some(FullLoopTransportCell::Sample([3.0, -4.0])),
        ])),
        FullLoopStageData::Pilots(spare([
            FullLoopPilotValue {
                bin: 2,
                sequence_bit: 0,
                iq: [1.0, 0.0],
            },
            FullLoopPilotValue {
                bin: 3,
                sequence_bit: 1,
                iq: [-1.0, 0.0],
            },
            FullLoopPilotValue {
                bin: 7,
                sequence_bit: 0,
                iq: [1.0, 0.0],
            },
        ])),
        FullLoopStageData::Llrs(spare([1.5, 0.0, -2.0])),
    ]
}

fn capacity(data: &FullLoopStageData) -> usize {
    match data {
        FullLoopStageData::Bits(values) | FullLoopStageData::Bytes(values) => values.capacity(),
        FullLoopStageData::Iq(values) => values.capacity(),
        FullLoopStageData::OptionalIq(values) => values.capacity(),
        FullLoopStageData::QamLabels(values) => values.capacity(),
        FullLoopStageData::TransportCells(values) => values.capacity(),
        FullLoopStageData::Pilots(values) => values.capacity(),
        FullLoopStageData::Llrs(values) => values.capacity(),
    }
}

#[test]
fn all_eight_query_variants_fit_the_source_without_changing_values_or_source_capacities() {
    for data in data_variants() {
        let mut context =
            FullLoopStageContext::new("original run", "original config", RunPhase::Observation)
                .unwrap();
        context.run_id.reserve(CONTEXT_SPARE_BYTES);
        context.configuration_id.reserve(CONTEXT_SPARE_BYTES);
        context.transmit_cell_roles =
            Some(spare([FullLoopCellRole::Data, FullLoopCellRole::Pilot]));
        let snapshot = FullLoopStageSnapshot {
            stage: FullLoopStage::PhysicalBins,
            retained_index: 0,
            direction: SignalDirection::Transmit,
            context,
            domain: SignalDomain::PhysicalFftBin,
            units: ProbeUnits::NormalizedIq,
            element_start: 12,  // Independent original physical-element origin.
            total_elements: 99, // Producer total is larger than this retained subset.
            data,
        };
        let original = serde_json::to_value(&snapshot).unwrap();
        let mut events = Vec::with_capacity(SNAPSHOT_SPARE_SLOTS);
        events.push(snapshot);
        let mut store = RetainedStages::new(0); // Direct storage fixture; no producer admission.
        store.events.insert(FullLoopStage::PhysicalBins, events);
        let before = retained_footprint(&store).unwrap();
        let output = store
            .query(
                FullLoopStage::PhysicalBins,
                0,
                FullLoopStageViewport { start: 1, count: 1 },
            )
            .unwrap();
        assert_eq!(output.element_start, 13);
        assert_eq!(output.total_elements, 99);
        assert_eq!(
            serde_json::to_value(&output.context).unwrap(),
            original["context"]
        );
        assert_eq!(
            serde_json::to_value(&output.data).unwrap()["values"][0],
            original["data"]["values"][1]
        );
        assert_eq!(capacity(&output.data), output.data.len());
        assert_eq!(
            output.context.run_id.capacity(),
            output.context.run_id.len()
        );
        assert_eq!(
            output.context.configuration_id.capacity(),
            output.context.configuration_id.len()
        );
        let roles = output.context.transmit_cell_roles.as_ref().unwrap();
        assert_eq!(roles.capacity(), roles.len());
        let footprint = retained_footprint(&output).unwrap();
        assert!(footprint.payload_bytes <= before.payload_bytes);
        assert_eq!(footprint.ordered_entries, 0);
        assert_eq!(footprint.shared_roots, 0);
        assert_eq!(retained_footprint(&store).unwrap(), before);
        assert_eq!(
            serde_json::to_value(&store.events[&FullLoopStage::PhysicalBins][0]).unwrap(),
            original
        );
        drop(store);
        assert_eq!(
            serde_json::to_value(output.data).unwrap()["values"][0],
            original["data"]["values"][1]
        );
    }
}

#[test]
fn inventory_fixed_layout_and_empty_copy_fit_the_source_root() {
    assert!(size_of::<FullLoopStageInfo>() <= size_of::<FullLoopStageSnapshot>());
    assert!(size_of::<Vec<FullLoopStageInfo>>() <= size_of::<super::super::FullLoopResult>());
    let source = RetainedStages::new(0);
    let inventory = source.inventory();
    assert!(inventory.is_empty());
    assert_eq!(inventory.capacity(), 0);
    assert!(size_of_val(&inventory) <= retained_footprint(&source).unwrap().payload_bytes);
}
