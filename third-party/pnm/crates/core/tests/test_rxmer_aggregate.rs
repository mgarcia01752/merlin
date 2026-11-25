/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent temporal-statistic and average-analysis checks.
use pnm::pnm::{
    analysis::rxmer::{
        aggregate::{RxMerAggregate, Series},
        RxMerAnalysisRequest,
    },
    parser::cm_ds_ofdm_rx_mer::CmDsOfdmRxMer,
};
fn series(values: &[u8], epoch: u32, name: &str) -> Series {
    let mut bytes = pnm_fixtures::RXMER[..28].to_vec();
    bytes[6..10].copy_from_slice(&epoch.to_be_bytes());
    bytes[24..28].copy_from_slice(&(values.len() as u32).to_be_bytes());
    bytes.extend(values);
    Series::from_capture(
        &CmDsOfdmRxMer::parse(&bytes, Default::default()).unwrap(),
        name.into(),
    )
    .unwrap()
}
#[test]
fn temporal_statistics_use_exact_carriers_and_preserve_missing_values() {
    let late = series(&[48, 72, 255, 40], 20, "a.bin");
    let early = series(&[40, 80, 255, 255], 10, "z.bin");
    let result = RxMerAggregate::calculate(&[&late, &early]).unwrap();
    assert_eq!(result.sources[0].capture_time, 10);
    assert_eq!(result.sources[0].name, "z.bin");
    assert_eq!(result.points[0].minimum_db, Some(10.0));
    assert_eq!(result.points[0].average_db, Some(11.0));
    assert_eq!(result.points[0].maximum_db, Some(12.0));
    assert_eq!(result.points[1].average_db, Some(19.0));
    assert_eq!(result.points[2].average_db, None);
    assert_eq!(result.points[2].valid_count, 0);
    assert_eq!(result.points[3].average_db, Some(10.0));
    assert_eq!(result.points[3].valid_count, 1);
    assert_eq!(
        result.points[1].frequency_hz - result.points[0].frequency_hz,
        25_000
    );
    let json = serde_json::to_value(&result).unwrap();
    assert!(json["points"][2]["minimum_db"].is_null());
    assert_eq!(json["group"]["mac_address"], "00:ca:00:00:00:01");
}
#[test]
fn incompatible_devices_channels_formats_and_grids_are_never_combined() {
    let source = series(&[132, 133], 1, "source");
    for variant in 0..7 {
        let mut other = source.clone();
        match variant {
            0 => other.key.mac_address[0] ^= 1,
            1 => other.key.channel_id ^= 1,
            2 => other.key.spacing_hz += 1,
            3 => other.key.first_active_subcarrier_index += 1,
            4 => other.key.zero_frequency_hz += 1,
            5 => other.key.minor_version += 1,
            _ => other.key.sample_count += 1,
        }
        assert!(RxMerAggregate::calculate(&[&source, &other]).is_err());
    }
    assert!(RxMerAggregate::calculate(&[]).is_err());
    let mut invalid = source.clone();
    invalid.values_db[0] = Some(f64::NAN);
    assert!(RxMerAggregate::calculate(&[&invalid]).is_err());
}
#[test]
fn average_modulation_keeps_sub_quarter_db_precision_and_exact_zoom() {
    let first = series(&[132, 160, 255], 1, "first");
    let second = series(&[133, 162, 255], 2, "second");
    let result = RxMerAggregate::calculate(&[&first, &second]).unwrap();
    assert_eq!(result.points[0].average_db, Some(33.125));
    let analysis = result
        .analyze_average(0, 2, RxMerAnalysisRequest::default())
        .unwrap();
    assert_eq!(analysis.temporal_average.as_ref().unwrap().capture_count, 2);
    assert_eq!(analysis.subcarriers[0].rxmer_db, Some(33.125));
    assert_eq!(analysis.subcarriers[1].rxmer_db, Some(40.25));
    let zoom = result
        .analyze_average(1, 1, RxMerAnalysisRequest::default())
        .unwrap();
    assert_eq!(zoom.selection.first_sample_index, 1);
    assert_eq!(zoom.summary.minimum_rxmer_db, Some(40.25));
    assert!(result
        .analyze_average(2, 2, RxMerAnalysisRequest::default())
        .is_err());
}
