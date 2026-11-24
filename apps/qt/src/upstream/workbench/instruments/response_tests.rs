/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Backend numerical anchors and explicit zero/missing response transport.
use super::*;
use merlin::upstream::{
    ConfigurationDocument, ConfiguredResponseRequest, Iq, ReceiverEstimateRequest,
    ReceiverObservationPolicy, Simulation,
};
use std::f64::consts::FRAC_PI_2;

const SOURCE_50: &[u8] = include_bytes!("../../../../../../examples/cli/upstream/us-50khz.json");
const SOURCE_25: &[u8] = include_bytes!("../../../../../../examples/cli/upstream/us-25khz.json");
// Independent test retention ceiling; enough Cartesian slots for one 4096-bin estimate.
const ESTIMATE_TEST_SCALARS: usize = 8192;

fn simulation(bytes: &[u8], two_tap: bool) -> Simulation {
    let document = ConfigurationDocument::from_json(bytes).unwrap();
    let (mut configuration, options) = document.prepare().unwrap();
    if two_tap {
        configuration.channel.taps = vec![Iq { re: 1.0, im: 0.0 }, Iq { re: 0.5, im: 0.0 }];
        configuration.channel.delay_samples = 2;
        configuration.channel.phase_radians = FRAC_PI_2;
    }
    let mut simulation = Simulation::new(options).unwrap();
    simulation.configure(configuration).unwrap();
    simulation
}

fn request(source: Source, metric: Metric, first: usize, count: usize) -> ResponseRequest {
    ResponseRequest {
        source,
        metric,
        grant_symbol: 0,
        retained_index: 0,
        first,
        count,
    }
}

fn close(value: &Value, expected: f64) {
    assert!((value.as_f64().unwrap() - expected).abs() < 1e-11);
}

fn received() -> (ReceiverEstimateSnapshot, ReceiverObservationSummary) {
    let mut simulation = simulation(SOURCE_50, false);
    simulation
        .set_receiver_observation_policy(
            ReceiverObservationPolicy::new(1, ESTIMATE_TEST_SCALARS).unwrap(),
        )
        .unwrap();
    let report = simulation.run().unwrap();
    (
        report
            .receiver_estimate(ReceiverEstimateRequest::default())
            .unwrap(),
        report.receiver_observations(),
    )
}

#[test]
fn response_transport_is_exact_and_unknown_or_overflowing_fields_reject() {
    let value = json!({"source":"received_estimate","metric":"phase","grant_symbol":0,
        "retained_index":0,"first":128,"count":4});
    let decoded: ResponseRequest = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), value);
    let mut unknown = value.clone();
    unknown["assumed_h"] = true.into();
    assert!(serde_json::from_value::<ResponseRequest>(unknown).is_err());
    let mut wrong = value;
    wrong["first"] = "18446744073709551616".into();
    assert!(serde_json::from_value::<ResponseRequest>(wrong).is_err());
}

#[test]
fn real_two_tap_response_keeps_literal_db_phase_and_lossless_optional_identities() {
    for bytes in [SOURCE_50, SOURCE_25] {
        let simulation = simulation(bytes, true);
        let mut source = simulation
            .configured_response(ConfiguredResponseRequest::default())
            .unwrap();
        // Independent transport-only identities on an owned snapshot copy.
        source.configuration_generation = Some(9_007_199_254_740_993);
        source.grant_id = 9_007_199_254_740_995;
        source.run_id = None;
        source.pre_eq_generation = None;
        let original = serde_json::to_value(&source).unwrap();
        let amplitude = project_configured(
            &source,
            &request(Source::PhysicalChannel, Metric::Amplitude, 0, 0),
            None,
        )
        .unwrap();
        assert_eq!(
            amplitude["metadata"]["configuration_generation"],
            "9007199254740993"
        );
        assert_eq!(amplitude["metadata"]["grant_id"], "9007199254740995");
        assert_eq!(amplitude["metadata"]["run_id"], Value::Null);
        assert_eq!(amplitude["metadata"]["pre_eq_generation"], Value::Null);
        assert_eq!(
            amplitude["metadata"]["reference_plane"],
            "configured_deterministic_physical_h"
        );
        let dc = source.fft_samples / 2;
        close(
            &amplitude["viewport"]["points"][dc]["value"],
            3.521_825_181_113_624_7,
        );
        assert_eq!(amplitude["viewport"]["points"][dc]["index"], dc);
        assert_eq!(
            amplitude["viewport"]["points"][0]["detail"]["excluded"],
            true
        );
        let phase = project_configured(
            &source,
            &request(Source::PhysicalChannel, Metric::Phase, 0, 0),
            None,
        )
        .unwrap();
        close(&phase["viewport"]["points"][dc]["value"], FRAC_PI_2);
        assert_eq!(serde_json::to_value(&source).unwrap(), original);
        assert!(project_configured(
            &source,
            &request(Source::Composite, Metric::Amplitude, 0, 0),
            None
        )
        .is_err());
        assert!(project_configured(
            &source,
            &request(Source::PhysicalChannel, Metric::Amplitude, 1, 0),
            None
        )
        .is_err());
    }
}

#[test]
fn periodic_impulse_uses_the_complete_backend_period_even_with_a_cropped_frequency_query() {
    let simulation = simulation(SOURCE_50, true);
    let mut source = simulation
        .configured_response(ConfiguredResponseRequest {
            first_carrier: 100,
            carrier_count: 3,
            include_impulse: true,
            ..Default::default()
        })
        .unwrap();
    let request = request(Source::PhysicalChannel, Metric::Impulse, 2, 2);
    let display = project_configured(&source, &request, None).unwrap();
    let points = display["viewport"]["points"].as_array().unwrap();
    assert_eq!(points.len(), 2);
    assert_eq!(points[0]["index"], 2);
    assert_eq!(points[1]["index"], 3);
    close(&points[0]["value"], 0.0);
    close(&points[1]["value"], -6.020_599_913_279_624);
    close(&points[0]["coordinate"], 0.01953125);
    assert_eq!(display["viewport"]["total"], 2048);
    assert_eq!(display["viewport"]["start"], 2);
    assert_eq!(display["metadata"]["first_carrier"], 100);
    assert_eq!(display["metadata"]["carrier_count"], 3);
    assert_eq!(display["metadata"]["impulse"]["period_samples"], 2048);
    let mut bad = request;
    bad.first = 2048;
    assert!(project_configured(&source, &bad, None).is_err());
    source.impulse.as_mut().unwrap().points.pop();
    assert!(project_configured(&source, &request, None).is_err());
    source.impulse = None;
    assert_eq!(
        project_configured(&source, &request, None).unwrap()["status"],
        "unavailable"
    );
}

#[test]
fn received_transport_preserves_every_original_hole_and_some_zero_without_line_bridging() {
    let (mut source, mut summary) = received();
    // Explicit owned DTO fixtures exercise transport, not fabricated decoder evidence.
    source.points = source.points[128..132].to_vec();
    source.first_carrier = 128;
    source.carrier_count = 4;
    source.points[0].value = Some(Iq { re: 1.0, im: 0.0 });
    source.points[0].amplitude_db = Some(0.0);
    source.points[1].value = Some(Iq::default());
    source.points[1].amplitude_db = None;
    source.points[1].phase_radians = None;
    source.points[2].value = None;
    source.points[2].amplitude_db = None;
    source.points[2].phase_radians = None;
    source.points[3].value = Some(Iq { re: 1.0, im: 0.0 });
    source.points[3].amplitude_db = Some(0.0);
    source.frame.identity.run = 9_007_199_254_740_993;
    source.frame.frame = u64::MAX;
    summary.frames[0] = source.frame.clone();
    let display = project_received(
        &source,
        &request(Source::ReceivedEstimate, Metric::Amplitude, 128, 4),
        &summary,
    )
    .unwrap();
    let points = display["viewport"]["points"].as_array().unwrap();
    assert_eq!(points.len(), 4);
    assert_eq!(points[1]["index"], 129);
    assert_eq!(points[2]["index"], 130);
    assert_eq!(points[1]["y"], Value::Null);
    assert_eq!(points[2]["minimum_y"], Value::Null);
    assert_eq!(points[1]["detail"]["value"], json!({"re":0.0,"im":0.0}));
    assert_eq!(points[2]["detail"]["value"], Value::Null);
    assert!(points[1]["readout"]
        .as_str()
        .unwrap()
        .contains("exact-zero"));
    assert!(points[2]["readout"]
        .as_str()
        .unwrap()
        .contains("no actual estimate"));
    assert_eq!(points[3]["gap"], true);
    close(&points[0]["y"], 0.5);
    close(&points[3]["y"], 0.5);
    assert_eq!(
        display["metadata"]["frame"]["identity"]["run"],
        "9007199254740993"
    );
    assert_eq!(
        display["metadata"]["frame"]["frame"],
        "18446744073709551615"
    );
    assert_eq!(
        display["choices"]["frames"].as_array().unwrap().len(),
        summary.frames.len()
    );
    assert_eq!(
        display["choices"]["frames"][0]["original"]["frame"],
        "18446744073709551615"
    );
    let mut mismatch = request(Source::ReceivedEstimate, Metric::Amplitude, 128, usize::MAX);
    assert!(project_received(&source, &mismatch, &summary).is_err());
    mismatch.retained_index = 1;
    assert!(project_received(&source, &mismatch, &summary).is_err());
}

#[test]
fn actual_received_impulse_is_unavailable_and_missing_values_never_get_a_finite_floor() {
    let (mut source, summary) = received();
    let impulse = project_received(
        &source,
        &request(Source::ReceivedEstimate, Metric::Impulse, 0, 0),
        &summary,
    )
    .unwrap();
    assert_eq!(impulse["status"], "unavailable");
    assert!(impulse["message"]
        .as_str()
        .unwrap()
        .contains("no public inverse"));
    assert_eq!(impulse["viewport"]["points"], json!([]));
    for point in &mut source.points {
        point.value = None;
        point.amplitude_db = None;
        point.phase_radians = None;
    }
    let display = project_received(
        &source,
        &request(Source::ReceivedEstimate, Metric::Phase, 0, 0),
        &summary,
    )
    .unwrap();
    assert_eq!(
        display["viewport"]["points"].as_array().unwrap().len(),
        2048
    );
    assert!(display["viewport"]["points"]
        .as_array()
        .unwrap()
        .iter()
        .all(|point| point["y"].is_null()));
    assert_eq!(display["viewport"]["y_ticks"], json!([]));
}
