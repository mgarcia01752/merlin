/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent source-code, frequency, role, useful-rate and Shannon-view anchors.
#![cfg(all(feature = "simulation", feature = "pnm-input"))]
use docsis_ofdm_fec::{
    api::*,
    scenario::pnm_json::{import_profile_capture, import_profiles, JsonCapture},
};
use serde_json::{json, Value};

const PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");

fn scheme(code: u8, name: &str, bits: Option<u8>, role: &str) -> Value {
    json!({"schema_type":"range","num_subcarriers":1,"modulation_order_code":code,"modulation_order":name,"bit_loading":bits,"carrier_type":role})
}
fn original() -> NamedCapture {
    let mut value: Value = serde_json::from_slice(PROFILE).unwrap();
    value["num_profiles"] = json!(2);
    value["profiles"] = json!([
        {"profile_id":200,"schemes":[scheme(4,"qam_16",Some(4),"data"),scheme(20,"plc",Some(4),"plc"),scheme(1,"continuous_pilot",Some(1),"continuous_pilot"),scheme(255,"unknown",None,"unknown")]},
        {"profile_id":0,"schemes":[scheme(0,"zero_bit_loaded",Some(0),"zero_bit_loaded"),scheme(3,"reserved_3",None,"reserved"),scheme(16,"exclusion",None,"exclusion"),scheme(14,"qam_16384",Some(14),"data")]}
    ]);
    NamedCapture::new("source.json", serde_json::to_vec(&value).unwrap()).unwrap()
}
fn request(metric: ProfileMetric) -> ProfileViewRequest {
    ProfileViewRequest {
        profiles: Some(vec![200]),
        metric,
        max_buckets: 1,
        ..Default::default()
    }
}

#[test]
fn complete_source_catalog_and_compact_schemes_preserve_codes_order_and_header() {
    let input = original();
    let capture = ProfileCapture::from_bytes(&input).unwrap();
    assert_eq!(capture.profile_ids(), [0, 200]);
    assert_eq!(capture.metadata().source_profile_order, [200, 0]);
    assert_eq!(capture.metadata().header.file_type, "PNN");
    assert_eq!(capture.metadata().header.capture_time, 1466967);
    let page = capture.schemes(200, 0, 256).unwrap();
    assert_eq!(page.total, 4);
    assert_eq!(page.schemes.len(), 4);
    assert_eq!(page.schemes[1].modulation_order_code, 20);
    assert_eq!(page.schemes[1].first_physical_carrier, 357);
    assert_eq!(page.schemes[1].first_frequency_hz, 640025000);
    assert_eq!(page.schemes[1].constellation, Some("qam_16"));
    assert_eq!(page.schemes[1].preamble.unwrap().constellation, "bpsk");
    assert_eq!(
        page.schemes[1].preamble.unwrap().shannon_minimum_mer_db,
        0.0
    );
    assert_eq!(page.schemes[3].modulation_order_code, 255);
    assert_eq!(page.schemes[3].carrier_type, ProfileCarrierRole::Unknown);
    assert_eq!(page.schemes[3].bit_loading, None);
    assert_eq!(page.schemes[3].constellation, None);
    assert_eq!(page.schemes[3].ideal_shannon_snr_db, None);
    let zero = capture.schemes(0, 0, 1).unwrap();
    assert_eq!(zero.schemes[0].bit_loading, Some(0));
    assert_eq!(zero.schemes[0].ideal_shannon_snr_db, None);
    assert!(ProfileId::new(200).is_err());
    assert!(Dpd::from_bytes(&input, ProfileId::new(0).unwrap()).is_err());
    assert_eq!(capture.provenance().original_bytes(), input.bytes());
}

#[test]
fn role_capacity_and_bounded_scientific_extrema_have_independent_literals() {
    let input = original();
    let capture = ProfileCapture::from_bytes(&input).unwrap();
    let summary = capture.summary(200).unwrap();
    assert_eq!(summary.data_bits_per_useful_symbol, 4);
    assert_eq!(summary.uncoded_data_capacity_bps, 100000);
    assert!(!summary.capacity_complete);
    assert_eq!(
        capture.summary(0).unwrap().uncoded_data_capacity_bps,
        350000
    );
    let bits = capture
        .inspect(&request(ProfileMetric::BitLoading))
        .unwrap();
    assert_eq!(bits.first_frequency_hz, 640000000);
    assert_eq!(bits.last_frequency_hz, 640075000);
    let point = &bits.series[0].buckets[0];
    assert_eq!(point.first_value.unwrap().physical_carrier, 356);
    assert_eq!(point.minimum, Some(1.0));
    assert_eq!(point.maximum, Some(4.0));
    assert!(point.gap);
    assert_eq!(point.roles.len(), 4);
    let shannon = capture
        .inspect(&request(ProfileMetric::IdealShannonSnr))
        .unwrap();
    let point = &shannon.series[0].buckets[0];
    assert_eq!(point.minimum, Some(0.0));
    assert!((point.maximum.unwrap() - 11.760912590556813).abs() < 1e-12);
    assert_eq!(
        shannon
            .selected_role_counts
            .iter()
            .map(|v| v.count)
            .sum::<usize>(),
        4
    );
    drop(capture);
    drop(input);
    assert!(!shannon.sources[0].original_bytes().is_empty());
}

#[test]
fn requests_reject_bad_identity_windows_pages_and_resources_without_mutation() {
    let capture = ProfileCapture::from_bytes(&original()).unwrap();
    assert!(capture.summary(1).is_err());
    assert!(capture.schemes(1, 0, 1).is_err());
    assert!(capture.schemes(0, 0, 257).is_err());
    assert!(capture.schemes(0, 5, 1).is_err());
    for viewport in [
        MeasurementViewport {
            start: usize::MAX,
            count: 1,
        },
        MeasurementViewport { start: 4, count: 1 },
    ] {
        let req = ProfileViewRequest {
            viewport,
            ..request(ProfileMetric::BitLoading)
        };
        assert!(capture.inspect(&req).is_err());
    }
    for budget in [0, 2049] {
        let req = ProfileViewRequest {
            max_buckets: budget,
            ..Default::default()
        };
        assert!(capture.inspect(&req).is_err());
    }
    let too_small = ProfileViewRequest {
        max_buckets: 1,
        ..Default::default()
    };
    assert!(capture.inspect(&too_small).is_err());
    let empty = ProfileViewRequest {
        profiles: Some(vec![]),
        ..Default::default()
    };
    assert!(capture.inspect(&empty).unwrap().series.is_empty());
    let no_roles = ProfileViewRequest {
        roles: vec![],
        ..request(ProfileMetric::BitLoading)
    };
    let view = capture.inspect(&no_roles).unwrap();
    assert_eq!(view.series[0].buckets[0].maximum, None);
    let mut full = request(ProfileMetric::BitLoading);
    full.viewport.start = usize::MAX;
    full.viewport.count = 0;
    assert_eq!(capture.inspect(&full).unwrap().start, 0);
    assert_eq!(capture.summary(200).unwrap().data_bits_per_useful_symbol, 4);
}

#[test]
fn legacy_import_and_opt_in_metadata_have_equal_scientific_profiles_and_errors() {
    let input = original();
    let previous = import_profiles(JsonCapture {
        name: input.name(),
        bytes: input.bytes(),
    })
    .unwrap();
    let (retained, metadata) = import_profile_capture(JsonCapture {
        name: input.name(),
        bytes: input.bytes(),
    })
    .unwrap();
    assert_eq!(
        serde_json::to_value(previous).unwrap(),
        serde_json::to_value(retained).unwrap()
    );
    assert_eq!(metadata.schemes[&200][3].modulation_order_code, 255);
    let mut bad: Value = serde_json::from_slice(input.bytes()).unwrap();
    bad["profiles"][0]["schemes"][0]["bit_loading"] = json!(6);
    let bytes = serde_json::to_vec(&bad).unwrap();
    let before = import_profiles(JsonCapture {
        name: "bad",
        bytes: &bytes,
    })
    .unwrap_err();
    let after = import_profile_capture(JsonCapture {
        name: "bad",
        bytes: &bytes,
    })
    .err()
    .unwrap();
    assert_eq!(before.code, after.code);
    assert_eq!(before.message, after.message);
}

#[test]
fn accepted_dpd_summary_and_view_keep_existing_identity_and_bounded_originals() {
    let input = NamedCapture::new("ordinary", PROFILE.to_vec()).unwrap();
    let dpd = Dpd::from_bytes(&input, ProfileId::new(0).unwrap()).unwrap();
    let capture = ProfileCapture::from_bytes(&input).unwrap();
    assert_eq!(
        serde_json::to_value(dpd.summary().unwrap()).unwrap(),
        serde_json::to_value(capture.summary(0).unwrap()).unwrap()
    );
    let req = ProfileViewRequest {
        profiles: Some(vec![0]),
        max_buckets: 2,
        ..Default::default()
    };
    let view = dpd.view(&req).unwrap();
    assert!(view.series[0].buckets.len() <= 2);
    assert_eq!(view.sources[0].original_bytes(), PROFILE);
    assert!(dpd
        .view(&ProfileViewRequest {
            profiles: Some(vec![1]),
            ..Default::default()
        })
        .is_err());
}
