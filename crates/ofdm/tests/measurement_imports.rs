/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(any(feature = "simulation", feature = "pnm-input"))]
use docsis_ofdm_fec::scenario::{
    measurement_view::{measurement_view, response_value, ResponseMetric},
    pnm_json::{import_channel, import_rxmer, JsonCapture},
};
#[test]
fn standalone_imports_and_bounded_view_preserve_nulls_grid_and_projection_units() {
    let frame = import_rxmer(JsonCapture {
        name: "rxmer",
        bytes: include_bytes!("../../../examples/measurements/single-channel/rxmer.json"),
    })
    .unwrap();
    let before = frame.mer_db.clone();
    let view = measurement_view(&frame.key.grid, &frame.mer_db, 0, 0, "RxMER dB").unwrap();
    assert!(view.points.len() <= 1024);
    assert_eq!(frame.mer_db, before);
    let huge = vec![Some(f64::MAX); frame.key.grid.sample_count];
    assert!(measurement_view(&frame.key.grid, &huge, 0, 0, "RxMER").is_err());
    assert_eq!(view.total, 7480);
    assert_eq!(view.x_ticks.first().unwrap().label, "640.000");
    assert!(measurement_view(&frame.key.grid, &frame.mer_db, 7480, 1, "RxMER").is_err());
    let mut gaps = frame.mer_db.clone();
    gaps[0] = None;
    let view = measurement_view(&frame.key.grid, &gaps, 0, 8, "RxMER").unwrap();
    assert!(view.points[0].gap);
    assert!(view.points[0].y.is_none());
    assert_eq!(
        response_value(Some([1.0, 0.0]), ResponseMetric::GainDb),
        Some(0.0)
    );
    assert_eq!(
        response_value(Some([0.0, 1.0]), ResponseMetric::PhaseDegrees),
        Some(90.0)
    );
    assert_eq!(
        response_value(Some([0.0, 0.0]), ResponseMetric::GainDb),
        None
    );
    assert_eq!(response_value(None, ResponseMetric::Real), None);
    assert!(import_channel(JsonCapture {
        name: "wrong",
        bytes: include_bytes!("../../../examples/measurements/single-channel/rxmer.json")
    })
    .is_err());
}
