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
fn source_catalog_ocd_compatibility_preserves_byte_ids_without_run_admission() {
    // Independent upper-byte source ID 255; source ID 199 is absent. Each source
    // contains four carriers starting at physical carrier 356 in either spacing.
    for (spacing, spacing_hz) in [
        (SubcarrierSpacing::Khz25, 25_000),
        (SubcarrierSpacing::Khz50, 50_000),
    ] {
        let mut document: Value = serde_json::from_slice(original().bytes()).unwrap();
        document["profiles"][0]["profile_id"] = json!(255);
        document["subcarrier_spacing"] = json!(spacing_hz);
        let input = NamedCapture::new(
            "source-boundary.json",
            serde_json::to_vec(&document).unwrap(),
        )
        .unwrap();
        let capture = ProfileCapture::from_bytes(&input).unwrap();
        let mut settings = TransmissionSettings {
            subcarrier_spacing: spacing,
            ..Default::default()
        };
        let configuration = TransmissionConfiguration::new(settings.clone()).unwrap();
        let before = serde_json::to_value(configuration.settings()).unwrap();
        configuration.check_profile_capture(&capture, 0).unwrap();
        configuration.check_profile_capture(&capture, 255).unwrap();
        assert_eq!(
            configuration
                .check_profile_capture(&capture, 199)
                .unwrap_err()
                .code(),
            ErrorCode::InvalidIdentifier
        );
        assert!(ProfileId::new(255).is_err());
        settings.subcarrier_spacing = match spacing {
            SubcarrierSpacing::Khz25 => SubcarrierSpacing::Khz50,
            SubcarrierSpacing::Khz50 => SubcarrierSpacing::Khz25,
        };
        let mismatch = TransmissionConfiguration::new(settings).unwrap();
        assert_eq!(
            mismatch
                .check_profile_capture(&capture, 255)
                .unwrap_err()
                .code(),
            ErrorCode::IncompatibleGrid
        );
        assert_eq!(
            serde_json::to_value(configuration.settings()).unwrap(),
            before
        );
        assert_eq!(capture.profile_ids(), [0, 255]);
        assert_eq!(capture.provenance().original_bytes(), input.bytes());
    }

    let input = NamedCapture::new("complete.json", PROFILE.to_vec()).unwrap();
    let capture = ProfileCapture::from_bytes(&input).unwrap();
    let profile = capture
        .simulation_profile(ProfileId::new(0).unwrap())
        .unwrap();
    // Independent occupied-band rejection: the original approximately 187 MHz
    // source span cannot fit the valid minimum 24 MHz OCD channel.
    let settings = TransmissionSettings {
        occupied_bandwidth_hz: 24_000_000,
        ..Default::default()
    };
    let configuration = TransmissionConfiguration::new(settings).unwrap();
    let source_error = configuration
        .check_profile_capture(&capture, 0)
        .unwrap_err();
    let existing_error = configuration.check_profile(&profile).unwrap_err();
    assert_eq!(source_error.code(), ErrorCode::IncompatibleGrid);
    assert_eq!(source_error.code(), existing_error.code());
    assert_eq!(source_error.message(), existing_error.message());
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

#[test]
fn borrowed_source_frame_preserves_raw_identity_roles_and_capture_ownership() {
    let input = original();
    let capture = ProfileCapture::from_bytes(&input).unwrap();
    let shared = capture.clone();
    let frame: &SourceProfileFrame = capture.frame(200).unwrap();

    // Independent source order/epoch/role anchors from the four-scheme fixture.
    assert_eq!(frame.profile_id, 200);
    assert_eq!(frame.capture_time, 1466967);
    assert_eq!(frame.assignments.len(), 4);
    assert_eq!(frame.assignments[0].carrier_type, ProfileCarrierRole::Data);
    assert_eq!(frame.assignments[1].carrier_type, ProfileCarrierRole::Plc);
    assert_eq!(
        frame.assignments[2].carrier_type,
        ProfileCarrierRole::ContinuousPilot
    );
    assert_eq!(
        frame.assignments[3].carrier_type,
        ProfileCarrierRole::Unknown
    );
    assert_eq!(frame.assignments[3].bit_loading, None);
    assert!(std::ptr::eq(frame, shared.frame(200).unwrap()));
    assert_eq!(
        capture.frame(199).unwrap_err().code(),
        ErrorCode::InvalidIdentifier
    );
    assert_eq!(capture.metadata().source_profile_order, [200, 0]);
    assert!(ProfileId::new(200).is_err());
    assert_eq!(capture.provenance().original_bytes(), input.bytes());
    drop(input);
    drop(capture);
    assert_eq!(shared.frame(200).unwrap().profile_id, 200);
}

#[test]
fn synthetic_unity_preview_preserves_original_byte_identity_and_independent_response_values() {
    let input = original();
    let capture = ProfileCapture::from_bytes(&input).unwrap();
    let before = serde_json::to_value(capture.metadata()).unwrap();
    for (metric, projection, reading) in [
        (
            ChannelResponseMetric::GainDb,
            "gain_db",
            "0.000 … 0.000 Gain (dB)",
        ),
        (
            ChannelResponseMetric::PhaseDegrees,
            "phase_degrees",
            "0.000 … 0.000 Phase (degrees)",
        ),
        (ChannelResponseMetric::Real, "real", "1.000 … 1.000 Real"),
        (
            ChannelResponseMetric::Imaginary,
            "imaginary",
            "0.000 … 0.000 Imaginary",
        ),
    ] {
        let report = capture
            .unity_response_preview(200, MeasurementViewport::default(), metric)
            .unwrap();
        assert_eq!(report.profile_id, 200);
        assert_eq!(
            report.reference_plane,
            "configured_synthetic_unity_response"
        );
        assert_eq!(report.projection, projection);
        assert_eq!(report.grid.sample_count, 4);
        assert_eq!(report.grid.first_subcarrier_index, 356);
        assert_eq!(report.view.start, 0);
        assert_eq!(report.view.count, 4);
        for point in &report.view.points {
            assert_eq!(point.y, Some(0.5));
            assert_eq!(point.minimum_y, Some(0.5));
            assert!(!point.gap);
            assert!(point.readout.ends_with(reading));
        }
        assert!(report.view.points[0]
            .readout
            .starts_with("640.000–640.000 MHz"));
    }

    // Explicit unity stays known at exclusions/reserved roles of the other source profile.
    let retained = capture
        .unity_response_preview(
            0,
            MeasurementViewport::default(),
            ChannelResponseMetric::Real,
        )
        .unwrap();
    assert!(retained
        .view
        .points
        .iter()
        .all(|point| !point.gap && point.y == Some(0.5)));
    assert_eq!(serde_json::to_value(capture.metadata()).unwrap(), before);
    assert_eq!(capture.provenance().original_bytes(), input.bytes());
    drop(capture);
    assert_eq!(retained.grid.sample_count, 4);
}

#[test]
fn unity_preview_rejects_missing_source_and_invalid_viewports_without_mutation() {
    let capture = ProfileCapture::from_bytes(&original()).unwrap();
    assert_eq!(
        capture
            .unity_response_preview(
                255,
                MeasurementViewport::default(),
                ChannelResponseMetric::GainDb
            )
            .err()
            .unwrap()
            .code(),
        ErrorCode::InvalidIdentifier
    );
    for viewport in [
        MeasurementViewport { start: 4, count: 1 },
        MeasurementViewport {
            start: usize::MAX,
            count: 2,
        },
    ] {
        assert!(capture
            .unity_response_preview(200, viewport, ChannelResponseMetric::Real)
            .is_err());
    }
    let zoom = capture
        .unity_response_preview(
            200,
            MeasurementViewport { start: 1, count: 2 },
            ChannelResponseMetric::Real,
        )
        .unwrap();
    assert_eq!(
        (zoom.view.start, zoom.view.count, zoom.view.total),
        (1, 2, 4)
    );
    let full = capture
        .unity_response_preview(
            200,
            MeasurementViewport {
                start: usize::MAX,
                count: 0,
            },
            ChannelResponseMetric::Real,
        )
        .unwrap();
    assert_eq!((full.view.start, full.view.count), (0, 4));
}
