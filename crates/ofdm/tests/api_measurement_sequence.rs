/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent linear-power, synthetic-clock and immutable original-input admission anchors.
#![cfg(all(feature = "simulation", feature = "pnm-input"))]
use docsis_ofdm_fec::{api::*, signal_processing::noise::residual_power_mer};
use serde_json::{json, Value};

const PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
const RXMER: &[u8] = include_bytes!("../../../examples/measurements/single-channel/rxmer.json");
const CHANNEL: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/channel_estimation.json");

fn profile() -> Dpd {
    Dpd::from_bytes(
        &NamedCapture::new("profile", PROFILE.to_vec()).unwrap(),
        ProfileId::new(0).unwrap(),
    )
    .unwrap()
}
fn measured(db: f64) -> RxMer {
    let mut value: Value = serde_json::from_slice(RXMER).unwrap();
    value["pnm_header"]["capture_time"] = json!(1466967);
    value["values"] = json!(vec![db; 7480]);
    RxMer::from_bytes(&NamedCapture::new("original", serde_json::to_vec(&value).unwrap()).unwrap())
        .unwrap()
}
fn options(mode: MeasurementMode) -> MeasurementSequenceOptions {
    MeasurementSequenceOptions {
        mode,
        interval_symbols: 10,
        tolerance: AssociationTolerance::ZERO,
    }
}
fn timing() -> TransmissionConfiguration {
    TransmissionConfiguration::new(TransmissionSettings::default()).unwrap()
}
fn close(left: f64, right: f64) {
    assert!((left - right).abs() < 1e-12, "{left} != {right}");
}

#[test]
fn idle_capacity_has_literal_strict_boundaries_without_profile_or_captures() {
    // Independent one-millisecond anchors with a 512-sample CP: 25 kHz has
    // 23 complete 42.5-us symbols; 50 kHz has 44 complete 22.5-us symbols.
    let duration = ObservationDuration::from_milliseconds(1).unwrap();
    for (spacing, complete_symbols) in [
        (SubcarrierSpacing::Khz25, 23),
        (SubcarrierSpacing::Khz50, 44),
    ] {
        let configuration = TransmissionConfiguration::new(TransmissionSettings {
            subcarrier_spacing: spacing,
            ..Default::default()
        })
        .unwrap();
        let before = serde_json::to_value(configuration.settings()).unwrap();
        for mode in [MeasurementMode::Hold, MeasurementMode::Interpolate] {
            let mut policy = MeasurementSequenceOptions {
                mode,
                interval_symbols: complete_symbols - 1,
                tolerance: AssociationTolerance::ZERO,
            };
            assert_eq!(policy.capacity(&configuration, duration).unwrap(), 2);
            policy.interval_symbols = complete_symbols;
            assert_eq!(policy.capacity(&configuration, duration).unwrap(), 1);
            policy.interval_symbols = u64::MAX;
            assert_eq!(policy.capacity(&configuration, duration).unwrap(), 1);
            policy.interval_symbols = 0;
            assert_eq!(
                policy
                    .capacity(&configuration, duration)
                    .unwrap_err()
                    .code(),
                ErrorCode::InvalidInput
            );
            assert_eq!(policy.interval_symbols, 0);
            policy.interval_symbols = 1;
            assert_eq!(
                policy.capacity(&configuration, duration).unwrap() as u64,
                complete_symbols
            );
            // Independent existing policy/domain anchors: at most 256 captures,
            // with the accepted one-hour observation domain retained for this API.
            let long_duration = ObservationDuration::from_milliseconds(3_600_000).unwrap();
            assert_eq!(policy.capacity(&configuration, long_duration).unwrap(), 256);
        }
        for (mode, expected) in [
            (MeasurementMode::Single, 1),
            (MeasurementMode::Average, 256),
        ] {
            let policy = MeasurementSequenceOptions {
                mode,
                interval_symbols: 0, // Dormant static interval is intentionally ignored.
                tolerance: AssociationTolerance::ZERO,
            };
            assert_eq!(policy.capacity(&configuration, duration).unwrap(), expected);
        }
        assert_eq!(
            serde_json::to_value(configuration.settings()).unwrap(),
            before
        );
    }
}

#[test]
fn average_names_every_original_with_equal_linear_power_weight() {
    let originals = vec![measured(40.0), measured(20.0), measured(30.0)];
    let hashes = originals
        .iter()
        .map(|v| v.provenance().original_sha256().to_owned())
        .collect::<Vec<_>>();
    let seq = MeasurementSequence::new(
        &profile(),
        &timing(),
        originals,
        options(MeasurementMode::Average),
    )
    .unwrap();
    let snapshot = seq.at(99);
    close(snapshot.normalized_power[0].unwrap(), 0.0037);
    assert_eq!(snapshot.contributors.len(), 3);
    for (i, source) in snapshot.contributors.iter().enumerate() {
        assert_eq!(source.index, i);
        close(source.weight.value(), 1.0 / 3.0);
        assert_eq!(source.provenance.original_sha256(), hashes[i]);
        assert_eq!(source.provenance.original_epoch().value(), 1466967);
    }
    assert_eq!(seq.info().last_transition_symbol, 0);
    assert_eq!(
        seq.capacity(ObservationDuration::new(1).unwrap()).unwrap(),
        256
    );
    let view = seq
        .preview(99, MeasurementViewport { start: 0, count: 1 })
        .unwrap();
    assert_eq!(view.view.axis_label, "Equivalent RxMER (dB)");
    assert!(view.view.points[0].readout.contains("24.318"));
    assert_eq!(view.contributors.len(), 3);
    assert_eq!(seq.capture(2).unwrap().values()[0].unwrap().value(), 30.0);
}

#[test]
fn temporal_queries_use_physical_symbol_clock_power_interpolation_and_last_hold() {
    let originals = vec![measured(40.0), measured(20.0)];
    let seq = MeasurementSequence::new(
        &profile(),
        &timing(),
        originals.clone(),
        options(MeasurementMode::Interpolate),
    )
    .unwrap();
    let info = seq.info();
    assert_eq!(info.interval_symbols, 10);
    assert_eq!(info.last_transition_symbol, 10);
    close(info.interval_milliseconds, 0.425);
    close(info.last_transition_milliseconds, 0.425);
    let first = seq.at(0);
    close(first.normalized_power[0].unwrap(), 0.0001);
    assert_eq!(first.contributors.len(), 1);
    let middle = seq.at(5);
    close(middle.normalized_power[0].unwrap(), 0.00505);
    assert_eq!(middle.contributors.len(), 2);
    close(middle.contributors[1].weight.value(), 0.5);
    let last = seq.at(u64::MAX);
    close(last.normalized_power[0].unwrap(), 0.01);
    assert_eq!(last.contributors[0].index, 1);
    assert_eq!(
        seq.capacity(ObservationDuration::new(1).unwrap()).unwrap(),
        3
    );
    assert_eq!(
        serde_json::to_value(seq.at(0)).unwrap(),
        serde_json::to_value(first).unwrap()
    );
    let held = MeasurementSequence::new(
        &profile(),
        &timing(),
        originals,
        options(MeasurementMode::Hold),
    )
    .unwrap();
    close(held.at(9).normalized_power[0].unwrap(), 0.0001);
    close(held.at(10).normalized_power[0].unwrap(), 0.01);
    assert_eq!(held.at(9).contributors.len(), 1);
}

#[test]
fn admission_rejects_duplicate_mode_epoch_coverage_counts_and_overflow_atomically() {
    let profile = profile();
    let timing = timing();
    let first = measured(40.0);
    assert!(
        MeasurementSequence::new(&profile, &timing, vec![], options(MeasurementMode::Single))
            .is_err()
    );
    assert!(MeasurementSequence::new(
        &profile,
        &timing,
        vec![first.clone(); 257],
        options(MeasurementMode::Average)
    )
    .is_err());
    assert!(MeasurementSequence::new(
        &profile,
        &timing,
        vec![first.clone(), first.clone()],
        options(MeasurementMode::Average)
    )
    .is_err());
    assert!(MeasurementSequence::new(
        &profile,
        &timing,
        vec![first.clone(), measured(20.0)],
        options(MeasurementMode::Single)
    )
    .is_err());
    assert!(MeasurementSequence::new(
        &profile,
        &timing,
        vec![first.clone()],
        options(MeasurementMode::Hold)
    )
    .is_err());
    for interval in [0, u64::MAX] {
        let mut opts = options(MeasurementMode::Interpolate);
        opts.interval_symbols = interval;
        let captures = vec![first.clone(), measured(20.0), measured(30.0)];
        assert!(MeasurementSequence::new(&profile, &timing, captures, opts).is_err());
    }
    let mut frame = first.frame().clone();
    frame.capture_time += 1;
    frame.source.capture_time = Some(frame.capture_time);
    let shifted = RxMer::from_frame(
        &NamedCapture::new("shift", b"original shifted".to_vec()).unwrap(),
        frame,
    )
    .unwrap();
    assert!(MeasurementSequence::new(
        &profile,
        &timing,
        vec![shifted],
        options(MeasurementMode::Single)
    )
    .is_err());
    let mut frame = first.frame().clone();
    frame.mer_db.fill(None);
    let gaps = RxMer::from_frame(
        &NamedCapture::new("gaps", b"original gaps".to_vec()).unwrap(),
        frame,
    )
    .unwrap();
    assert!(MeasurementSequence::new(
        &profile,
        &timing,
        vec![gaps],
        options(MeasurementMode::Single)
    )
    .is_err());
    let seq = MeasurementSequence::new(
        &profile,
        &timing,
        vec![first.clone()],
        options(MeasurementMode::Single),
    )
    .unwrap();
    assert!(seq.capture(1).is_err());
    assert!(seq
        .preview(
            0,
            MeasurementViewport {
                start: usize::MAX,
                count: 2
            }
        )
        .is_err());
    assert_eq!(
        seq.capture(0).unwrap().provenance().original_bytes(),
        first.provenance().original_bytes()
    );
    drop(first);
    let retained = seq.at(0);
    drop(seq);
    assert!(!retained.contributors[0]
        .provenance
        .original_bytes()
        .is_empty());
}

#[test]
fn explicit_unity_and_captured_h_keep_original_epoch_tolerance_and_inputs() {
    let profile = profile();
    let timing = timing();
    timing
        .check_response(&profile, None, AssociationTolerance::ZERO)
        .unwrap();
    let input = NamedCapture::new("original H", CHANNEL.to_vec()).unwrap();
    let response = ChannelEstimate::from_bytes(&input).unwrap();
    assert_eq!(
        timing
            .check_response(&profile, Some(&response), AssociationTolerance::ZERO)
            .unwrap_err()
            .code(),
        ErrorCode::AssociationFailed
    );
    timing
        .check_response(
            &profile,
            Some(&response),
            AssociationTolerance::new(90000).unwrap(),
        )
        .unwrap();
    assert_eq!(response.provenance().original_bytes(), CHANNEL);
    timing
        .check_response(&profile, None, AssociationTolerance::ZERO)
        .unwrap();
}

#[test]
fn residual_power_conversion_has_independent_db_and_unavailable_anchors() {
    close(residual_power_mer(0.0001).unwrap(), 40.0);
    close(residual_power_mer(0.01).unwrap(), 20.0);
    close(residual_power_mer(1.0).unwrap(), 0.0);
    close(residual_power_mer(100.0).unwrap(), -20.0);
    for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert_eq!(residual_power_mer(value), None);
    }
}
