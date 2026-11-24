/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual-report projection parity with independent transport/reference anchors.
use super::*;
use crate::shell::RetainedResources;
use merlin::upstream::{
    ConfigurationDocument, QamConstellationRequest, QamPairRequest, ReportJson, Simulation,
    MAX_QAM_CONSTELLATION_REFERENCES, MAX_QAM_PAIR_POINTS, MAX_QAM_QUERY_POINTS,
};
use serde_json::json;

const SOURCE_50: &[u8] = include_bytes!("../../../../../../examples/cli/upstream/us-50khz.json");
const SOURCE_25: &[u8] = include_bytes!("../../../../../../examples/cli/upstream/us-25khz.json");
// Independent finite fixture policies, not estimates of production footprints.
const RETAINED_TEST_POINTS: usize = 32_768;
const TEST_RESOURCES: RetainedResources = RetainedResources {
    payload_bytes: 16 * 1024 * 1024,
    ordered_entries: 262_144,
    shared_roots: 0,
    channel_count: 0,
    slot_count: 0,
    presentation_utf16_bytes: 0,
    presentation_entries: 0,
    retained_views: 0,
    open_views: 0,
};

fn actual_report(bytes: &[u8]) -> BatchReport {
    report_with_observations(bytes, true)
}

fn report_with_observations(bytes: &[u8], retain: bool) -> BatchReport {
    let document = ConfigurationDocument::from_json(bytes).unwrap();
    let (config, mut options) = document.prepare().unwrap();
    options.retained_points = RETAINED_TEST_POINTS;
    options.retained_taps = vec![
        Tap::TxGrid,
        Tap::TxPreEq,
        Tap::RxFft,
        Tap::RxEqualized,
        Tap::TxOutput,
    ];
    if !retain {
        options.retained_points = 0;
        options.retained_taps.clear();
    }
    let mut simulation = Simulation::new(options).unwrap();
    simulation.configure(config).unwrap();
    simulation.run().unwrap()
}

fn budget() -> RetentionBudget {
    RetentionBudget::with_limits(TEST_RESOURCES)
}

fn close(actual: &Value, expected: f64) {
    assert!((actual.as_f64().unwrap() - expected).abs() < 1e-12);
}

#[test]
fn flat_request_transport_round_trips_exact_modes_and_rejects_unknown_fields() {
    for value in [
        json!({"mode":"raw","tap":"RxFft","loading":null,"maximum_points":7}),
        json!({"mode":"constellation","tap":"TxGrid","loading":0,"maximum_points":7,"maximum_references":2}),
        json!({"mode":"pairing","left":"TxGrid","right":"RxEqualized","loading":null,"maximum_pairs":7}),
    ] {
        let request: QamRequest = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(request).unwrap(), value);
        let mut unknown = value;
        unknown["physical_channel_correction"] = true.into();
        assert!(serde_json::from_value::<QamRequest>(unknown).is_err());
    }
    assert!(serde_json::from_value::<QamRequest>(
        json!({"mode":"raw","tap":"RxFft","loading":null,"maximum_points":-1})
    )
    .is_err());
}

#[test]
fn canonical_bpsk_reference_and_original_pilots_have_independent_literal_screen_anchors() {
    let report = actual_report(SOURCE_50);
    let budget = budget();
    let value = qam(
        &report,
        QamRequest::Constellation(QamConstellationRequest {
            tap: Tap::TxGrid,
            loading: 0,
            maximum_points: 4,
            maximum_references: 2,
        }),
        &budget,
    )
    .unwrap();
    let display = &value.value().snapshot;
    assert_eq!(display["status"], "ready");
    assert_eq!(display["metadata"]["reference_bits"], 1);
    assert_eq!(display["metadata"]["payload_loading"], 0);
    assert_eq!(display["metadata"]["normalization"], "tx_role_gain_removed");
    let references = display["scatter"]["references"].as_array().unwrap();
    assert_eq!(references.len(), 2);
    assert_eq!(
        references[0]["detail"],
        json!({"label":0,"value":{"re":1.0,"im":0.0}})
    );
    assert_eq!(
        references[1]["detail"],
        json!({"label":1,"value":{"re":-1.0,"im":0.0}})
    );
    close(&references[0]["x"], 0.9545454545454546);
    close(&references[1]["x"], 0.04545454545454547);
    close(&references[0]["y"], 0.5);
    assert_eq!(
        display["limits"],
        json!({"maximum_points":MAX_QAM_QUERY_POINTS,
        "maximum_references":MAX_QAM_CONSTELLATION_REFERENCES,"maximum_pairs":MAX_QAM_PAIR_POINTS})
    );
    for point in display["scatter"]["points"].as_array().unwrap() {
        assert_eq!(point["detail"]["loading"], 0);
        assert!(point["detail"]["identity"]["run"].is_string());
        assert!(point["detail"]["normalization_divisor"].as_f64().unwrap() > 0.0);
    }
}

#[test]
fn raw_and_exact_pair_views_preserve_actual_source_coordinates_and_only_actual_choices() {
    let report = actual_report(SOURCE_25);
    let original = report.to_json().unwrap();
    let budget = budget();
    let raw = qam(
        &report,
        QamRequest::Raw {
            tap: Tap::RxFft,
            loading: None,
            maximum_points: 3,
        },
        &budget,
    )
    .unwrap();
    let actual = report.qam(Tap::RxFft, None, 3).unwrap();
    let snapshot = &raw.value().snapshot;
    assert_eq!(
        snapshot["selection"],
        json!({"mode":"raw","tap":"RxFft","loading":null,"maximum_points":3})
    );
    assert_eq!(snapshot["choices"]["taps"].as_array().unwrap().len(), 4);
    for (point, source) in snapshot["scatter"]["points"]
        .as_array()
        .unwrap()
        .iter()
        .zip(&actual.points)
    {
        assert_eq!(point["detail"]["carrier"], source.carrier);
        assert_eq!(point["detail"]["symbol"], source.symbol);
        assert_eq!(point["detail"]["value"], json!(source.value));
        assert_eq!(point["detail"]["frame"], source.frame.to_string());
    }
    let request = QamPairRequest {
        maximum_pairs: 2,
        ..Default::default()
    };
    let source = report.qam_pairing(request).unwrap();
    let paired = qam(&report, QamRequest::Pairing(request), &budget).unwrap();
    let paired = &paired.value().snapshot;
    assert_eq!(paired["metadata"]["matched_total"], source.matched_total);
    assert_eq!(paired["metadata"]["unpaired_left"], source.unpaired_left);
    assert_eq!(
        paired["scatter"]["points"].as_array().unwrap().len(),
        source.pairs.len() * 2
    );
    for pair in paired["scatter"]["points"]
        .as_array()
        .unwrap()
        .chunks_exact(2)
    {
        assert_eq!(pair[0]["series"], "left");
        assert_eq!(pair[1]["series"], "right");
        assert_eq!(pair[0]["detail"]["identity"], pair[1]["detail"]["identity"]);
        assert_eq!(pair[0]["detail"]["carrier"], pair[1]["detail"]["carrier"]);
        assert_eq!(pair[0]["detail"]["symbol"], pair[1]["detail"]["symbol"]);
    }
    assert_eq!(report.to_json().unwrap(), original);
}

#[test]
fn original_point_identity_stays_lossless_beyond_javascript_integer_precision() {
    let report = actual_report(SOURCE_50);
    let mut point = report.qam(Tap::RxFft, None, 1).unwrap().points.remove(0);
    // Transport-only mutation of an owned query copy, never fabricated run evidence.
    point.identity.run = 9_007_199_254_740_993;
    point.identity.generation = 9_007_199_254_740_995;
    point.identity.grant = 9_007_199_254_740_997;
    point.frame = u64::MAX;
    let value = common::point_detail(&point).unwrap();
    assert_eq!(
        value["identity"],
        json!({"run":"9007199254740993","generation":"9007199254740995","grant":"9007199254740997"})
    );
    assert_eq!(value["frame"], "18446744073709551615");
    assert_eq!(value["value"], json!(point.value));
}

#[test]
fn both_numerologies_use_actual_linear_psd_and_original_frequency_bins_without_plc_overlay() {
    for (bytes, fft, spacing) in [(SOURCE_50, 2048, 50_000), (SOURCE_25, 4096, 25_000)] {
        let report = actual_report(bytes);
        let budget = budget();
        let source = report
            .spectrum(
                Tap::TxOutput,
                report
                    .configuration
                    .us_configuration
                    .waveform()
                    .numerology(),
            )
            .unwrap();
        let value = spectrum(
            &report,
            SpectrumRequest {
                tap: Tap::TxOutput,
                first: 0,
                count: 3,
            },
            &budget,
        )
        .unwrap();
        let display = &value.value().snapshot;
        assert_eq!(display["metadata"]["fft_len"], fft);
        assert_eq!(display["metadata"]["analysis_window"], "rectangular");
        assert_eq!(
            display["metadata"]["units"],
            "normalized-complex-baseband-power-per-hz"
        );
        assert!(display["metadata"]["first_sample"].is_string());
        let viewport = &display["viewport"];
        assert_eq!(viewport["total"], fft);
        assert_eq!(viewport["bands"], json!([]));
        assert_eq!(viewport["points"][0]["frequency_hz"], -51_200_000.0);
        assert_eq!(
            viewport["points"][1]["frequency_hz"],
            -51_200_000.0 + f64::from(spacing)
        );
        for (index, point) in viewport["points"].as_array().unwrap().iter().enumerate() {
            assert_eq!(point["power_per_hz"], source.power_per_hz[index]);
            assert_eq!(point["bin"], index);
            close(&point["x"], index as f64 / 2.0);
            assert_eq!(point["gap"], false);
            assert_eq!(point["minimum_y"], point["y"]);
        }
        let single = spectrum(
            &report,
            SpectrumRequest {
                tap: Tap::TxOutput,
                first: fft - 1,
                count: 1,
            },
            &budget,
        )
        .unwrap();
        close(&single.value().snapshot["viewport"]["points"][0]["x"], 0.5);
    }
}

#[test]
fn missing_history_is_explicit_but_bad_copy_or_viewport_bounds_reject() {
    let report = report_with_observations(SOURCE_50, false);
    let budget = budget();
    let qam = qam(
        &report,
        QamRequest::Raw {
            tap: Tap::RxFft,
            loading: None,
            maximum_points: 1,
        },
        &budget,
    )
    .unwrap();
    assert_eq!(qam.value().snapshot["status"], "unavailable");
    assert_eq!(qam.value().snapshot["scatter"]["points"], json!([]));
    let spectrum = spectrum(
        &report,
        SpectrumRequest {
            tap: Tap::TxOutput,
            first: 0,
            count: 0,
        },
        &budget,
    )
    .unwrap();
    assert_eq!(spectrum.value().snapshot["status"], "unavailable");
    assert_eq!(spectrum.value().snapshot["viewport"]["points"], json!([]));
    assert!(super::qam(
        &report,
        QamRequest::Raw {
            tap: Tap::RxFft,
            loading: None,
            maximum_points: 0
        },
        &budget
    )
    .is_err());
    let available = actual_report(SOURCE_50);
    assert!(super::spectrum(
        &available,
        SpectrumRequest {
            tap: Tap::TxOutput,
            first: 2048,
            count: 1
        },
        &budget
    )
    .is_err());
}

#[test]
fn exact_current_payload_admission_overlaps_and_final_drop_release_only_the_view() {
    let report = actual_report(SOURCE_50);
    let original = report.to_json().unwrap();
    let budget = budget();
    let request = SpectrumRequest {
        tap: Tap::TxOutput,
        first: 0,
        count: 3,
    };
    let old = spectrum(&report, request, &budget).unwrap();
    let source = old.value().snapshot.clone();
    let first = old.fixture_reserved_resources();
    let new = spectrum(&report, request, &budget).unwrap();
    let second = new.fixture_reserved_resources();
    assert_eq!(budget.usage(), first.checked_add(second).unwrap());
    drop(new);
    assert_eq!(budget.usage(), first);
    for limits in [
        RetainedResources {
            payload_bytes: first.payload_bytes - 1,
            ..TEST_RESOURCES
        },
        RetainedResources {
            ordered_entries: first.ordered_entries - 1,
            ..TEST_RESOURCES
        },
    ] {
        let pressure = RetentionBudget::with_limits(limits);
        assert!(spectrum(&report, request, &pressure).is_err());
        assert_eq!(pressure.usage(), RetainedResources::default());
        assert_eq!(old.value().snapshot, source);
    }
    drop(old);
    assert_eq!(budget.usage(), RetainedResources::default());
    assert_eq!(report.to_json().unwrap(), original);
}
