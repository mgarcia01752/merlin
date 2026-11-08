/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared consumer JSON and source-bound role protection.
use docsis_ofdm_fec::profile::{
    document::{verify_captured_roles, CandidateDocument, ScenarioDocument},
    evaluation::{evaluate, generate_candidate, EvaluationSettings},
};
use std::sync::atomic::AtomicBool;
#[test]
fn candidate_round_trip_and_protected_roles() {
    let scenario: ScenarioDocument =
        serde_json::from_str(include_str!("../../../examples/scenario.json")).unwrap();
    scenario.validate().unwrap();
    let settings: EvaluationSettings =
        serde_json::from_str(include_str!("../../../examples/conservative-16qam.json")).unwrap();
    let frames: Vec<_> = scenario.frames.iter().collect();
    let mut candidate = generate_candidate(
        &frames,
        Some(&scenario.profiles[0]),
        &settings,
        scenario.protocol,
        &AtomicBool::new(false),
    )
    .unwrap();
    assert_eq!(candidate.edit_range(0, 4, Some(6)).unwrap(), 1);
    assert_eq!(
        candidate.assignments[1..],
        scenario.profiles[0].assignments[1..]
    );
    let report = evaluate(&candidate, &frames, &settings, &AtomicBool::new(false)).unwrap();
    assert_eq!(report.passing_frame_count, 2);
    assert_eq!(report.worst_interval_epoch, Some(101));
    let document = CandidateDocument {
        document_kind: "docsis_ofdm_fec_candidate".into(),
        schema_version: 1,
        candidate,
        settings,
        evaluation: Some(report),
    };
    let json = serde_json::to_string(&document).unwrap();
    let mut decoded: CandidateDocument = serde_json::from_str(&json).unwrap();
    decoded.validate().unwrap();
    verify_captured_roles(&mut decoded.candidate, &scenario).unwrap();
    assert!(decoded.candidate.roles_verified);
    decoded.candidate.assignments[1] = decoded.candidate.assignments[0].clone();
    assert!(verify_captured_roles(&mut decoded.candidate, &scenario).is_err());
}
#[test]
fn bad_schemas_grids_and_nonfinite_values_are_rejected() {
    let mut scenario: ScenarioDocument =
        serde_json::from_str(include_str!("../../../examples/scenario.json")).unwrap();
    scenario.schema_version = 2;
    assert!(scenario.validate().is_err());
    scenario.schema_version = 1;
    scenario.frames[0].key.grid.spacing_hz = 0;
    assert!(scenario.validate().is_err());
    scenario.frames[0].key.grid.spacing_hz = 25000;
    scenario.frames[0].mer_db[0] = Some(f64::INFINITY);
    assert!(scenario.validate().is_err());
}
