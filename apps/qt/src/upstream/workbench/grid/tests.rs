/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Independent coordinate/transport anchors and actual retained viewport ownership.
use super::*;
use crate::{
    shell::{ApplicationSessions, RetainedResources},
    upstream::PreparedSession,
};
use merlin::upstream::{ConfigurationDocument, ReceiverObservationPolicy};
use serde_json::json;

const DOCUMENT_50: &[u8] = include_bytes!("../../../../../../examples/cli/upstream/us-50khz.json");
const DOCUMENT_25: &[u8] = include_bytes!("../../../../../../examples/cli/upstream/us-25khz.json");

fn controller(bytes: &[u8]) -> (ApplicationSessions, UpstreamController) {
    let application = ApplicationSessions::desktop().unwrap();
    let document = ConfigurationDocument::from_json(bytes).unwrap();
    let prepared = PreparedSession::from_document(
        document,
        ReceiverObservationPolicy::default(),
        application.budget(),
    )
    .unwrap();
    let key = application.configure_upstream(true).unwrap();
    let controller = UpstreamController::new(key, prepared).unwrap();
    (application, controller)
}

#[test]
fn fifty_khz_literal_coordinates_metadata_and_absent_visits_remain_distinct() {
    let (application, controller) = controller(DOCUMENT_50);
    let display = observe(
        &controller,
        ResourceGridRequest {
            frame_index: 1,
            first_symbol: 2,
            symbol_count: 1,
            first_carrier: 127,
            carrier_count: 3,
            ..Default::default()
        },
        application.budget(),
    )
    .unwrap();
    let snapshot = &display.value().snapshot;
    assert_eq!(snapshot["format"], "merlin.upstream.resource_grid");
    assert_eq!(snapshot["schema_version"], 1);
    assert_eq!(snapshot["transmission_planned"], true);
    assert!(snapshot["configuration_generation"].is_string());
    let grid = &snapshot["grid"];
    assert_eq!(grid["reference_plane"], "configured_cm_transmit_resources");
    assert_eq!(grid["grant_id"], "1");
    assert_eq!(grid["grant_epoch_sample"], "0");
    assert_eq!(grid["grant_first_frame"], "0");
    assert_eq!(grid["absolute_frame"], "1");
    assert_eq!(grid["frame_prefix_sample"], "13440");
    assert_eq!(grid["grant_frames"], 2);
    assert_eq!(grid["frame_index"], 1);
    assert_eq!(grid["frame_symbols"], 6);
    assert_eq!(grid["fft_samples"], 2048);
    assert_eq!(grid["sample_rate_hz"], 102_400_000);
    assert_eq!(grid["spacing_hz"], 50_000);
    assert_eq!(grid["prefix_samples"], 192);
    assert_eq!(grid["rolloff_samples"], 64);
    assert_eq!(grid["symbol_stride_samples"], 2240);
    assert_eq!(grid["first_symbol"], 2);
    assert_eq!(grid["symbol_count"], 1);
    assert_eq!(grid["first_carrier"], 127);
    assert_eq!(grid["carrier_count"], 3);
    assert_eq!(
        grid["counts"],
        json!({"data":1,"complementary":0,
        "known_pilot":1,"zero_loaded":0,"excluded":1,"unallocated":0})
    );
    let cells = grid["cells"].as_array().unwrap();
    assert_eq!(cells.len(), 3);
    assert_eq!(cells[0]["role"], "excluded");
    assert_eq!(cells[0]["physical_carrier"], 127);
    assert_eq!(cells[0]["minislot_index"], Value::Null);
    assert_eq!(cells[0]["visit_index"], Value::Null);
    assert_eq!(cells[0]["amplitude_gain"], Value::Null);
    assert_eq!(cells[0]["frame_bit_offset"], Value::Null);
    assert_eq!(cells[1]["role"], "known_pilot");
    assert_eq!(cells[1]["physical_carrier"], 128);
    assert_eq!(cells[1]["frame_symbol"], 2);
    assert_eq!(cells[1]["grant_symbol"], 8);
    assert_eq!(cells[1]["prefix_start_sample"], "17920");
    assert_eq!(cells[1]["useful_start_sample"], "18112");
    assert_eq!(cells[1]["cell_loading"], 0);
    assert_eq!(cells[1]["amplitude_gain"], 1.0);
    assert_eq!(cells[2]["role"], "data");
    assert_eq!(cells[2]["physical_carrier"], 129);
}

#[test]
fn twenty_five_khz_literal_grid_preserves_zero_loading_and_unselected_coordinates() {
    let (application, controller) = controller(DOCUMENT_25);
    let display = observe(
        &controller,
        ResourceGridRequest {
            frame_index: 1,
            first_symbol: 2,
            symbol_count: 1,
            first_carrier: 255,
            carrier_count: 130,
            ..Default::default()
        },
        application.budget(),
    )
    .unwrap();
    let grid = &display.value().snapshot["grid"];
    assert_eq!(grid["fft_samples"], 4096);
    assert_eq!(grid["spacing_hz"], 25_000);
    assert_eq!(grid["symbol_stride_samples"], 4288);
    assert_eq!(grid["frame_prefix_sample"], "25728");
    let cells = grid["cells"].as_array().unwrap();
    assert_eq!(cells.len(), 130);
    assert_eq!(cells[0]["physical_carrier"], 255);
    assert_eq!(cells[0]["role"], "excluded");
    assert_eq!(cells[1]["physical_carrier"], 256);
    assert_eq!(cells[1]["role"], "known_pilot");
    assert_eq!(cells[1]["prefix_start_sample"], "34304");
    assert_eq!(cells[1]["useful_start_sample"], "34496");
    assert_eq!(cells[65]["physical_carrier"], 320);
    assert_eq!(cells[65]["role"], "zero_loaded");
    assert_eq!(cells[65]["configured_data_loading"], 0);
    assert_eq!(cells[65]["cell_loading"], 0);
    assert_eq!(cells[65]["grant_selected"], true);
    assert_eq!(cells[65]["visit_index"], Value::Null);
    assert_eq!(cells[65]["amplitude_gain"], Value::Null);
    assert_eq!(cells[129]["physical_carrier"], 384);
    assert_eq!(cells[129]["role"], "unallocated");
    assert_eq!(cells[129]["minislot_index"], Value::Null);
    assert_eq!(cells[129]["grant_bit_offset"], Value::Null);
}

#[test]
fn all_u64_identifiers_and_sample_origins_survive_beyond_javascript_integer_precision() {
    let (_, controller) = controller(DOCUMENT_50);
    let mut source = controller
        .inspect(|simulation| {
            simulation.resource_grid(ResourceGridRequest {
                carrier_count: 1,
                ..Default::default()
            })
        })
        .unwrap()
        .unwrap();
    // Independent transport-only values, not a fabricated scientific configuration.
    source.configuration_generation = 9_007_199_254_740_993;
    source.grid.grant_id = 9_007_199_254_740_995;
    source.grid.grant_epoch_sample = 9_007_199_254_740_997;
    source.grid.grant_first_frame = 9_007_199_254_740_999;
    source.grid.absolute_frame = 9_007_199_254_741_001;
    source.grid.frame_prefix_sample = 9_007_199_254_741_003;
    source.grid.cells[0].prefix_start_sample = 9_007_199_254_741_005;
    source.grid.cells[0].useful_start_sample = u64::MAX;
    let value = presentation::snapshot(&source).unwrap();
    assert_eq!(value["configuration_generation"], "9007199254740993");
    assert_eq!(value["grid"]["grant_id"], "9007199254740995");
    assert_eq!(value["grid"]["grant_epoch_sample"], "9007199254740997");
    assert_eq!(value["grid"]["grant_first_frame"], "9007199254740999");
    assert_eq!(value["grid"]["absolute_frame"], "9007199254741001");
    assert_eq!(value["grid"]["frame_prefix_sample"], "9007199254741003");
    assert_eq!(
        value["grid"]["cells"][0]["prefix_start_sample"],
        "9007199254741005"
    );
    assert_eq!(
        value["grid"]["cells"][0]["useful_start_sample"],
        "18446744073709551615"
    );
    assert!(value["grid"]["frame_index"].is_number());
    assert!(value["grid"]["cells"][0]["physical_carrier"].is_number());
    assert_eq!(source.grid.cells[0].useful_start_sample, u64::MAX);
}

#[test]
fn actual_json_charge_covers_old_and_candidate_views_until_each_final_drop() {
    let (application, controller) = controller(DOCUMENT_50);
    let budget = application.budget();
    let baseline = budget.usage();
    let request = ResourceGridRequest {
        carrier_count: 3,
        ..Default::default()
    };
    let old = observe(&controller, request, budget).unwrap();
    let assigned = old.fixture_reserved_resources();
    assert!(assigned.payload_bytes > size_of::<Display>());
    assert!(assigned.ordered_entries > 0);
    assert_eq!(assigned.shared_roots, 0);
    assert_eq!(budget.usage(), baseline.checked_add(assigned).unwrap());
    let candidate = observe(&controller, request, budget).unwrap();
    let candidate_resources = candidate.fixture_reserved_resources();
    assert_eq!(
        budget.usage(),
        baseline
            .checked_add(assigned)
            .unwrap()
            .checked_add(candidate_resources)
            .unwrap()
    );
    drop(old);
    assert_eq!(
        budget.usage(),
        baseline.checked_add(candidate_resources).unwrap()
    );
    drop(candidate);
    assert_eq!(budget.usage(), baseline);
}

#[test]
fn actual_view_admission_rejects_byte_and_entry_pressure_without_mutating_science() {
    let (application, controller) = controller(DOCUMENT_50);
    let request = ResourceGridRequest {
        carrier_count: 3,
        ..Default::default()
    };
    let accepted = observe(&controller, request, application.budget()).unwrap();
    let old = accepted.value().snapshot.clone();
    let source = controller
        .inspect(|simulation| simulation.document().unwrap().to_json().unwrap())
        .unwrap();
    let assigned = accepted.fixture_reserved_resources();
    for limits in [
        RetainedResources {
            payload_bytes: assigned.payload_bytes - 1,
            ordered_entries: assigned.ordered_entries,
            ..Default::default()
        },
        RetainedResources {
            payload_bytes: assigned.payload_bytes,
            ordered_entries: assigned.ordered_entries - 1,
            ..Default::default()
        },
    ] {
        // Isolated policy is solely the output test seam; production uses the host counter.
        let pressure = RetentionBudget::with_limits(limits);
        assert!(observe(&controller, request, &pressure).is_err());
        assert_eq!(pressure.usage(), RetainedResources::default());
        assert_eq!(accepted.value().snapshot, old);
        assert_eq!(
            controller
                .inspect(|simulation| simulation.document().unwrap().to_json().unwrap())
                .unwrap(),
            source
        );
        assert!(application.active().is_none());
    }
}

#[test]
fn invalid_public_viewport_preserves_current_display_generation_and_idle_source() {
    let (application, controller) = controller(DOCUMENT_50);
    let old = observe(
        &controller,
        ResourceGridRequest::default(),
        application.budget(),
    )
    .unwrap();
    let generation = old.value().snapshot["configuration_generation"].clone();
    let baseline = application.budget().usage();
    assert!(observe(
        &controller,
        ResourceGridRequest {
            frame_index: 2,
            ..Default::default()
        },
        application.budget()
    )
    .is_err());
    assert_eq!(application.budget().usage(), baseline);
    let next = observe(
        &controller,
        ResourceGridRequest::default(),
        application.budget(),
    )
    .unwrap();
    assert_eq!(next.value().snapshot, old.value().snapshot);
    assert_eq!(
        next.value().snapshot["configuration_generation"],
        generation
    );
    assert!(application.active().is_none());
}
