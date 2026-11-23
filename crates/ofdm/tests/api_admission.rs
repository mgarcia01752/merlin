/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent timing anchors and immutable byte/source admission through public APIs.
#![cfg(all(feature = "simulation", feature = "pnm-input"))]
use docsis_ofdm_fec::api::{
    Dpd, ErrorCode, FileFillPolicy, NamedCapture, ObservationDuration, PrbsSeed, SourceInput,
    SubcarrierSpacing, TransmissionConfiguration, TransmissionSettings,
};

#[test]
fn transmission_admission_uses_literal_sample_clock_accounting_without_running() {
    let original = TransmissionSettings::default();
    let accepted = TransmissionConfiguration::new(original.clone()).unwrap();
    let timing = accepted
        .timing(ObservationDuration::new(1).unwrap())
        .unwrap();
    assert_eq!(timing.sample_rate_hz, 204_800_000);
    assert_eq!(timing.fft_samples, 8192);
    assert_eq!(timing.symbol_period_samples, 8704);
    assert_eq!(timing.windowed_symbol_samples, 8832);
    assert_eq!(timing.observation_samples, 204800);
    assert_eq!(timing.complete_symbols, 23);
    assert_eq!(timing.unused_samples, 4608);
    assert_eq!(accepted.settings(), &original);
    let mut changed = original.clone();
    changed.subcarrier_spacing = SubcarrierSpacing::Khz50;
    let other = TransmissionConfiguration::new(changed).unwrap();
    let timing = other.timing(ObservationDuration::new(1).unwrap()).unwrap();
    assert_eq!(timing.fft_samples, 4096);
    assert_eq!(timing.symbol_period_samples, 4608);
    assert_eq!(timing.complete_symbols, 44);
    assert_eq!(timing.unused_samples, 2048);
    assert_eq!(accepted.settings(), &original);
}

#[test]
fn unnamed_file_preserves_exact_bytes_empty_label_bounds_and_named_validation() {
    // Independent original/fill anchors: eight bytes ->64 bits, then a finite80-bit target.
    let mut bytes = b"\0exact\r\n".to_vec();
    let source = SourceInput::unnamed_file(&bytes, None).unwrap();
    bytes[0] = 255; // Caller mutation must not replace retained original zero.
    assert_eq!(source.name(), Some(""));
    assert_eq!(source.kind(), "file");
    assert_eq!(source.original_bytes(), Some(b"\0exact\r\n".as_slice()));
    assert_eq!(source.target_bits(), Some(64));
    assert!(source.prbs_seed().is_none());
    assert_eq!(
        source.original_sha256(),
        Some("541070fc2b7aa44069811cafe1c6810b607e696dc92792089c0fd88789bb8797")
    );
    let seed = PrbsSeed::new(7).unwrap(); // Independent finite-fill recurrence choice.
    let filled =
        SourceInput::unnamed_file(b"\0exact\r\n", Some(FileFillPolicy::new(80, seed).unwrap()))
            .unwrap();
    assert_eq!(filled.target_bits(), Some(80));
    assert_eq!(filled.original_bytes(), source.original_bytes());
    assert_eq!(filled.original_sha256(), source.original_sha256());
    assert!(
        SourceInput::unnamed_file(b"\0exact\r\n", Some(FileFillPolicy::new(8, seed).unwrap()),)
            .is_err()
    );
    // Independent one-past existing16-MiB file admission boundary.
    assert!(SourceInput::unnamed_file(&vec![0; 16 * 1024 * 1024 + 1], None).is_err());
    let empty = SourceInput::unnamed_file(&[], None).unwrap();
    assert_eq!(empty.name(), Some(""));
    assert_eq!(empty.target_bits(), Some(0));
    assert_eq!(empty.original_bytes(), Some(b"".as_slice()));
    assert_eq!(
        NamedCapture::new("", vec![]).unwrap_err().code(),
        ErrorCode::ResourceLimit
    );
    assert_eq!(source.target_bits(), Some(64));
}

#[test]
fn minimum_observation_rounds_complete_periods_and_reports_over_policy_thresholds() {
    // Independent rational-clock anchors: a 512-sample CP gives 17/400 ms
    // periods at 25 kHz and 9/400 ms at 50 kHz; a 1024-sample CP gives 45 us.
    // Index 399 completes 400 periods at exactly 17/9 ms, testing exact rounding.
    for (last, spacing, prefix, expected) in [
        (0, SubcarrierSpacing::Khz25, 512, 1),
        (399, SubcarrierSpacing::Khz25, 512, 17),
        (399, SubcarrierSpacing::Khz50, 512, 9),
        (1000, SubcarrierSpacing::Khz25, 512, 43),
        (400, SubcarrierSpacing::Khz25, 512, 18),
        (2000, SubcarrierSpacing::Khz25, 512, 86),
        (1000, SubcarrierSpacing::Khz50, 512, 23),
        (1000, SubcarrierSpacing::Khz25, 1024, 46),
        (100000, SubcarrierSpacing::Khz25, 512, 4251),
    ] {
        let settings = TransmissionSettings {
            subcarrier_spacing: spacing,
            cyclic_prefix_samples: prefix,
            ..Default::default()
        };
        let configuration = TransmissionConfiguration::new(settings.clone()).unwrap();
        assert_eq!(
            configuration
                .minimum_observation_milliseconds_after(last)
                .unwrap(),
            expected
        );
        assert!(
            configuration
                .timing(ObservationDuration::new(expected).unwrap())
                .unwrap()
                .complete_symbols
                > last
        );
        if expected > 1 {
            assert!(
                configuration
                    .timing(ObservationDuration::new(expected - 1).unwrap())
                    .unwrap()
                    .complete_symbols
                    <= last
            );
        }
        assert_eq!(configuration.settings(), &settings);
    }
    let configuration = TransmissionConfiguration::new(TransmissionSettings::default()).unwrap();
    // Independent wide arithmetic: ceil(2^64 * 17/400) milliseconds. This reports
    // a threshold, not admission to the existing one-hour observation domain.
    assert_eq!(
        configuration
            .minimum_observation_milliseconds_after(u64::MAX)
            .unwrap(),
        783_986_623_132_655_944
    );
    assert!(ObservationDuration::new(783_986_623_132_655_944).is_err());
}

#[test]
fn transmission_json_admission_rejects_corruption_unknown_fields_and_limits() {
    let accepted = TransmissionConfiguration::new(TransmissionSettings::default()).unwrap();
    let text = serde_json::to_vec(&accepted).unwrap();
    assert_eq!(
        TransmissionConfiguration::from_json(&text)
            .unwrap()
            .settings(),
        accepted.settings()
    );
    for bytes in [b"{bad".as_slice(), b"{}".as_slice()] {
        assert_eq!(
            TransmissionConfiguration::from_json(bytes)
                .err()
                .unwrap()
                .code(),
            ErrorCode::InvalidInput
        );
    }
    let mut extra: serde_json::Value = serde_json::from_slice(&text).unwrap();
    extra["unrecognized"] = serde_json::json!(true);
    assert!(TransmissionConfiguration::from_json(&serde_json::to_vec(&extra).unwrap()).is_err());
    assert_eq!(
        TransmissionConfiguration::from_json(&vec![b' '; 4097])
            .err()
            .unwrap()
            .code(),
        ErrorCode::ResourceLimit
    );
    let bad = TransmissionSettings {
        roll_off_samples: 512,
        ..TransmissionSettings::default()
    };
    assert!(TransmissionConfiguration::new(bad).is_err());
    assert_eq!(serde_json::to_vec(&accepted).unwrap(), text);
}

#[test]
fn profile_editor_compatibility_is_separate_from_full_execution_readiness() {
    let input = NamedCapture::new(
        "original.json",
        include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json")
            .to_vec(),
    )
    .unwrap();
    let profile =
        Dpd::from_bytes(&input, docsis_ofdm_fec::api::ProfileId::new(0).unwrap()).unwrap();
    let accepted = TransmissionConfiguration::new(TransmissionSettings::default()).unwrap();
    accepted.check_profile(&profile).unwrap();
    let mismatched = TransmissionConfiguration::new(TransmissionSettings {
        subcarrier_spacing: SubcarrierSpacing::Khz50,
        ..TransmissionSettings::default()
    })
    .unwrap();
    assert_eq!(
        mismatched.check_profile(&profile).unwrap_err().code(),
        ErrorCode::IncompatibleGrid
    );
    let narrow = TransmissionConfiguration::new(TransmissionSettings {
        occupied_bandwidth_hz: 24_000_000,
        ..TransmissionSettings::default()
    })
    .unwrap();
    assert_eq!(
        narrow.check_profile(&profile).unwrap_err().code(),
        ErrorCode::IncompatibleGrid
    );
    accepted.check_profile(&profile).unwrap();
    assert_eq!(profile.sources()[0].original_bytes(), input.bytes());
}

#[test]
fn source_admission_preserves_arbitrary_original_bytes_and_independent_fill() {
    let bytes = vec![0xef, 0xbb, 0xbf, 0, b'\r', b'\n', 0xff, 1];
    let input = NamedCapture::new("unaltered.dat", bytes.clone()).unwrap();
    let fill = FileFillPolicy::new(80, PrbsSeed::new(7).unwrap()).unwrap();
    let accepted = SourceInput::file(input, Some(fill)).unwrap();
    assert_eq!(accepted.kind(), "file");
    assert_eq!(accepted.name(), Some("unaltered.dat"));
    assert_eq!(accepted.original_bytes(), Some(bytes.as_slice()));
    assert_eq!(accepted.target_bits(), Some(80));
    assert_eq!(accepted.prbs_seed(), None);
    assert_eq!(accepted.original_sha256().unwrap().len(), 64);
    let retained = accepted.clone();
    drop(accepted);
    assert_eq!(retained.original_bytes(), Some(bytes.as_slice()));
    assert_eq!(retained.target_bits(), Some(80));
    let too_short = FileFillPolicy::new(63, PrbsSeed::new(7).unwrap()).unwrap();
    assert!(SourceInput::file(NamedCapture::new("same", bytes).unwrap(), Some(too_short)).is_err());
    assert_eq!(retained.target_bits(), Some(80));
}

#[test]
fn source_admission_distinguishes_empty_file_eof_and_continuous_prbs15() {
    let empty = SourceInput::file(NamedCapture::new("empty", Vec::new()).unwrap(), None).unwrap();
    assert_eq!(empty.original_bytes(), Some([].as_slice()));
    assert_eq!(empty.target_bits(), Some(0));
    assert_eq!(
        empty.original_sha256(),
        Some("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
    );
    let continuous = SourceInput::prbs15(PrbsSeed::new(32767).unwrap());
    assert_eq!(continuous.kind(), "prbs15");
    assert_eq!(continuous.prbs_seed().unwrap().value(), 32767);
    assert!(continuous.original_bytes().is_none());
    assert!(continuous.target_bits().is_none());
    assert!(continuous.name().is_none());
    assert!(PrbsSeed::new(0).is_err());
    assert!(PrbsSeed::new(32768).is_err());
    assert!(FileFillPolicy::new(134_217_729, PrbsSeed::new(1).unwrap()).is_err());
}
