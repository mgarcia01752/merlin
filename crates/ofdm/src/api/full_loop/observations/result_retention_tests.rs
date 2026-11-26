/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Private stage-capacity fixture independent of scientific producer acceptance.
use super::*;
use crate::simulation::test_points::{ProbeContext, RunPhase, TransmitCellRole};

#[test]
fn exact_stage_storage_keeps_roles_text_nested_data_and_legacy_statistics() {
    // Separate capacities anchor map event slots, identity text, roles and data.
    const EVENT_SLOTS: usize = 5;
    const ID_BYTES: usize = 41;
    const ROLE_SLOTS: usize = 7;
    const LLR_SLOTS: usize = 13;
    let mut context =
        ProbeContext::new("original run", "original config", RunPhase::Observation).unwrap();
    context.run_id.reserve(ID_BYTES);
    context.configuration_id.reserve(ID_BYTES);
    let mut roles = Vec::with_capacity(ROLE_SLOTS);
    roles.extend([TransmitCellRole::Data, TransmitCellRole::Ncp]);
    context.transmit_cell_roles = Some(roles);
    let mut llrs = Vec::with_capacity(LLR_SLOTS);
    llrs.extend([1.5, -2.0]); // Independent finite likelihood fixtures.
    let mut snapshots = Vec::with_capacity(EVENT_SLOTS);
    snapshots.push(FullLoopStageSnapshot {
        stage: FullLoopStage::PayloadLdpcInput,
        retained_index: 0,
        direction: SignalDirection::Receive,
        context,
        domain: SignalDomain::CodewordBit,
        units: ProbeUnits::LogLikelihood,
        element_start: 0,
        total_elements: llrs.len(),
        data: FullLoopStageData::Llrs(llrs),
    });
    let mut stages = RetainedStages::new(0); // This fixture inserts ownership directly; no producer admission.
    stages
        .events
        .insert(FullLoopStage::PayloadLdpcInput, snapshots);
    let before =
        serde_json::to_value(stages.events.get(&FullLoopStage::PayloadLdpcInput).unwrap()).unwrap();
    let stats = serde_json::to_value(&stages.stats).unwrap();
    stages.normalize_result_storage();
    let snapshots = stages.events.get(&FullLoopStage::PayloadLdpcInput).unwrap();
    assert_eq!(snapshots.capacity(), snapshots.len());
    assert_eq!(
        snapshots[0].context.run_id.capacity(),
        snapshots[0].context.run_id.len()
    );
    assert_eq!(
        snapshots[0].context.configuration_id.capacity(),
        snapshots[0].context.configuration_id.len()
    );
    let roles = snapshots[0].context.transmit_cell_roles.as_ref().unwrap();
    assert_eq!(roles.capacity(), roles.len());
    let FullLoopStageData::Llrs(llrs) = &snapshots[0].data else {
        panic!("LLRs must retain their original representation");
    };
    assert_eq!(llrs.capacity(), llrs.len());
    assert_eq!(serde_json::to_value(snapshots).unwrap(), before);
    assert_eq!(serde_json::to_value(&stages.stats).unwrap(), stats);
}
