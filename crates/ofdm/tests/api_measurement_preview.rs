/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent original-grid/unit/gap anchors through the high-level measurement API.
#![cfg(all(feature = "simulation", feature = "pnm-input"))]
use docsis_ofdm_fec::api::{
    ChannelEstimate, ChannelResponseMetric, MeasurementViewport, NamedCapture, RxMer,
};
use serde_json::json;

const RXMER: &[u8] = include_bytes!("../../../examples/measurements/single-channel/rxmer.json");
const CHANNEL: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/channel_estimation.json");

#[test]
fn original_rxmer_preview_retains_frequency_units_time_hash_and_input_ownership() {
    let original = NamedCapture::new("unaltered", RXMER.to_vec()).unwrap();
    let capture = RxMer::from_bytes(&original).unwrap();
    let first = capture
        .preview(MeasurementViewport { start: 0, count: 1 })
        .unwrap();
    assert_eq!(first.projection, "rxmer_db");
    assert_eq!(first.view.total, 7480);
    assert_eq!(first.view.axis_label, "RxMER (dB)");
    assert_eq!(first.view.points[0].x, 0.5);
    assert_eq!(first.view.points[0].y, Some(0.5));
    assert!(first.view.points[0].readout.contains("640.000–640.000 MHz"));
    assert!(first.view.points[0].readout.contains("42.750"));
    assert_eq!(first.provenance.original_epoch().value(), 1380970);
    assert_eq!(first.grid, *capture.grid());
    let full = capture.preview(MeasurementViewport::default()).unwrap();
    assert!(full.view.points.len() <= 1024);
    assert_eq!(full.view.count, 7480);
    assert_eq!(
        full.provenance.original_sha256(),
        capture.provenance().original_sha256()
    );
    drop(capture);
    drop(original);
    assert_eq!(first.provenance.original_bytes(), RXMER);
}

#[test]
fn preview_preserves_gaps_and_rejects_bad_windows_without_changing_originals() {
    let mut value: serde_json::Value = serde_json::from_slice(RXMER).unwrap();
    value["values"][0] = serde_json::Value::Null;
    let original = NamedCapture::new("gap", serde_json::to_vec(&value).unwrap()).unwrap();
    let capture = RxMer::from_bytes(&original).unwrap();
    let view = capture
        .preview(MeasurementViewport { start: 0, count: 2 })
        .unwrap();
    assert!(view.view.points[0].gap);
    assert_eq!(view.view.points[0].y, None);
    assert!(!view.view.points[1].gap);
    for viewport in [
        MeasurementViewport {
            start: usize::MAX,
            count: 2,
        },
        MeasurementViewport {
            start: 7480,
            count: 1,
        },
        MeasurementViewport {
            start: 0,
            count: 7481,
        },
    ] {
        assert!(capture.preview(viewport).is_err());
    }
    let full = capture
        .preview(MeasurementViewport {
            start: usize::MAX,
            count: 0,
        })
        .unwrap();
    assert_eq!(full.view.start, 0);
    assert_eq!(full.view.count, 7480);
    assert!(full.view.points[0].gap);
    assert_eq!(capture.provenance().original_bytes(), original.bytes());
    assert_eq!(capture.values()[0], None);
}

#[test]
fn original_h_projections_have_literal_phase_sign_db_and_zero_gap_semantics() {
    let mut value: serde_json::Value = serde_json::from_slice(CHANNEL).unwrap();
    value["iq"] = json!([[1.0, 0.0], [0.0, 1.0], [-1.0, 0.0], [0.0, 0.0], null]);
    value["data_length"] = json!(20);
    let original = NamedCapture::new("literal H", serde_json::to_vec(&value).unwrap()).unwrap();
    let capture = ChannelEstimate::from_bytes(&original).unwrap();
    let viewport = MeasurementViewport::default();
    let real = capture
        .preview(viewport, ChannelResponseMetric::Real)
        .unwrap();
    assert_eq!(real.projection, "real");
    assert_eq!(real.view.points[0].y, Some(0.25));
    assert_eq!(real.view.points[1].y, Some(0.5));
    assert_eq!(real.view.points[2].y, Some(0.75));
    assert!(real.view.points[4].gap);
    let imaginary = capture
        .preview(viewport, ChannelResponseMetric::Imaginary)
        .unwrap();
    assert!(imaginary.view.points[1].readout.contains("1.000"));
    let phase = capture
        .preview(viewport, ChannelResponseMetric::PhaseDegrees)
        .unwrap();
    assert!(phase.view.points[1].readout.contains("90.000"));
    assert!(phase.view.points[2].readout.contains("180.000"));
    assert_eq!(phase.view.axis_label, "Phase (degrees)");
    let gain = capture
        .preview(viewport, ChannelResponseMetric::GainDb)
        .unwrap();
    assert!(gain.view.points[0].readout.contains("0.000"));
    assert!(gain.view.points[3].gap);
    assert!(gain.view.points[4].gap);
    assert_eq!(gain.view.axis_label, "Gain (dB)");
    assert_eq!(gain.provenance.original_bytes(), original.bytes());
    assert!(capture
        .preview(
            MeasurementViewport { start: 5, count: 1 },
            ChannelResponseMetric::Real
        )
        .is_err());
    assert_eq!(capture.provenance().original_bytes(), original.bytes());
}
