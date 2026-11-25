/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent temporal-power arithmetic and exact acquisition compatibility.
use pnm::{
    common::signal_processing::spectrum::mean_level_dbmv,
    pnm::{
        analysis::spectrum::{Series, SpectrumAggregate},
        parser::{cm_spectrum_analysis::CmSpectrumAnalysis, parse_limits::ParseLimits},
    },
};
fn series(values: Vec<Option<f64>>, epoch: u32) -> Series {
    let capture =
        CmSpectrumAnalysis::parse(pnm_fixtures::SPECTRUM, ParseLimits::default()).unwrap();
    let mut series = Series::from_capture(&capture, format!("misleading-name-{epoch}")).unwrap();
    series.key.sample_count = values.len();
    series.values_dbmv = values;
    series.capture_time = epoch;
    series
}
#[test]
fn linear_power_mean_has_independent_known_values_and_extreme_stability() {
    assert!((mean_level_dbmv(&[0.0, 10.0]).unwrap() - 7.403626894942439).abs() < 1e-12);
    assert!((mean_level_dbmv(&[-40.0, -30.0]).unwrap() + 32.59637310505756).abs() < 1e-12);
    assert_eq!(mean_level_dbmv(&[327.67, 327.67]), Some(327.67));
    assert_eq!(mean_level_dbmv(&[-327.68, -327.68]), Some(-327.68));
    assert_eq!(mean_level_dbmv(&[]), None);
    assert_eq!(mean_level_dbmv(&[f64::NAN]), None);
}
#[test]
fn sorted_sources_unrounded_average_extrema_missing_and_json_contract() {
    let early = series(vec![Some(0.0), None, None], 100);
    let late = series(vec![Some(10.0), Some(-23.75), None], 200);
    let result = SpectrumAggregate::calculate(&[&late, &early]).unwrap();
    assert_eq!(result.sources[0].capture_time, 100);
    assert_eq!(result.points[0].minimum_dbmv, Some(0.0));
    assert_eq!(result.points[0].maximum_dbmv, Some(10.0));
    assert!((result.points[0].average_dbmv.unwrap() - 7.403626894942439).abs() < 1e-12);
    assert_eq!(result.points[1].average_dbmv, Some(-23.75));
    assert_eq!(result.points[1].valid_count, 1);
    assert_eq!(result.points[2].average_dbmv, None);
    assert_eq!(result.points[2].valid_count, 0);
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["average_method"], "linear_power");
    assert_eq!(json["value_units"], "dBmV");
    assert!(json["points"][2]["average_dbmv"].is_null());
}
#[test]
fn all_acquisition_fields_are_part_of_compatibility_even_when_labels_match() {
    let original = series(vec![Some(-20.0)], 100);
    for field in 0..11 {
        let mut changed = original.clone();
        match field {
            0 => changed.key.mac_address[5] += 1,
            1 => changed.key.channel_id += 1,
            2 => changed.key.major_version += 1,
            3 => changed.key.minor_version += 1,
            4 => changed.key.first_segment_center_hz += 1,
            5 => changed.key.last_segment_center_hz += 1,
            6 => changed.key.segment_span_hz += 1,
            7 => changed.key.bins_per_segment += 1,
            8 => changed.key.encoded_enbw += 1,
            9 => changed.key.window_function += 1,
            _ => changed.key.sample_count += 1,
        }
        assert!(SpectrumAggregate::calculate(&[&original, &changed]).is_err());
    }
    assert!(SpectrumAggregate::calculate(&[]).is_err());
    let single = SpectrumAggregate::calculate(&[&original]).unwrap();
    assert_eq!(single.points[0].minimum_dbmv, single.points[0].maximum_dbmv);
}
#[test]
fn grid_rejects_zero_overfull_and_overflowing_metadata() {
    let series = series(vec![Some(-20.0)], 100);
    let mut key = series.key;
    assert!(key.frequencies_hz().is_ok());
    key.bins_per_segment = 0;
    assert!(key.frequencies_hz().is_err());
    key.bins_per_segment = 1;
    key.segment_span_hz = 0;
    assert!(key.frequencies_hz().is_err());
    key.segment_span_hz = 100;
    key.first_segment_center_hz = u32::MAX - 50;
    key.last_segment_center_hz = key.first_segment_center_hz;
    key.sample_count = 2;
    assert!(key.frequencies_hz().is_err());
}
