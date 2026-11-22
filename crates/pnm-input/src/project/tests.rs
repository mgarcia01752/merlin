/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Aggregate original-storage boundary without allocating oversized payloads.
use super::*;
use serde_json::json;

#[test]
fn cached_original_storage_admits_the_inclusive_boundary_and_rejects_sum_overflow() {
    let mut downstream = Project::from_bytes(include_bytes!(
        "../../../../examples/workbench/single/workbench.merlin.json"
    ))
    .unwrap();
    let upstream = Project::from_bytes(
        &serde_json::to_vec(&json!({
            // Independent schema-3 and disabled-observation transport anchors.
            "schema_version":3,"application":"MERLIN","direction":"upstream",
            "contents":{"input":{"kind":"configuration","document":include_str!(
                "../../../../examples/cli/upstream/us-50khz.json")},
            "options":docsis_ofdma::api::ExperimentOptions::default(),
            "receiver_observations":{"maximum_frames":0,"maximum_scalar_slots":0}}
        }))
        .unwrap(),
    )
    .unwrap();
    // Instrument only the cached admitted byte counter: the 96 MiB policy edge
    // and checked-add overflow must not require huge capture allocations in tests.
    let original_limit_bytes = 96 * 1024 * 1024;
    downstream.original_input_bytes = original_limit_bytes;
    assert_eq!(
        Project::collection_info(&[&downstream])
            .unwrap()
            .original_input_bytes,
        original_limit_bytes
    );
    assert_eq!(
        Project::collection_info(&[&downstream, &upstream])
            .err()
            .unwrap()
            .code(),
        ErrorCode::ResourceLimit
    );
    downstream.original_input_bytes = usize::MAX;
    assert_eq!(
        Project::collection_info(&[&downstream, &upstream])
            .err()
            .unwrap()
            .code(),
        ErrorCode::ResourceLimit
    );
}
