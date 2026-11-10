/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent manufacturer anchors and public model bounds, not just round trips.
use docsis_ofdm_fec::signal_processing::channel::additional_loss::{
    AdditionalLoss, CableModel, LengthUnit,
};

fn cable(length: f64, unit: LengthUnit) -> AdditionalLoss {
    AdditionalLoss::Cable {
        cable: CableModel::Belden1694aV1,
        length,
        unit,
    }
}
#[test]
fn full_cable_loss_uses_manufacturer_anchors_and_explicit_length_units() {
    let feet = cable(100.0, LengthUnit::Feet);
    // Belden 1694A nominal attenuation, revision 0.716: dB per100 ft.
    for (frequency, expected) in [
        (540_000_000, -4.5),
        (720_000_000, -5.3),
        (750_000_000, -5.4),
        (1_000_000_000, -6.3),
    ] {
        assert!(
            (feet.gain_db(frequency, 540_000_000, 1_000_000_000).unwrap() - expected).abs() < 1e-12
        );
        assert!(
            (cable(30.48, LengthUnit::Metres)
                .gain_db(frequency, 540_000_000, 1_000_000_000)
                .unwrap()
                - expected)
                .abs()
                < 1e-12
        );
    }
    assert!(
        (feet
            .gain_db(630_000_000, 540_000_000, 1_000_000_000)
            .unwrap()
            + 4.9)
            .abs()
            < 1e-12
    );
    assert!(
        (cable(200.0, LengthUnit::Feet)
            .gain_db(540_000_000, 540_000_000, 1_000_000_000)
            .unwrap()
            + 9.0)
            .abs()
            < 1e-12
    );
}
#[test]
fn manual_endpoints_and_zero_length_preserve_unity() {
    let tilt = AdditionalLoss::Manual { tilt_db: -6.0 };
    assert_eq!(tilt.gain_db(100, 100, 300).unwrap(), 0.0);
    assert_eq!(tilt.gain_db(200, 100, 300).unwrap(), -3.0);
    assert_eq!(tilt.gain_db(300, 100, 300).unwrap(), -6.0);
    assert_eq!(AdditionalLoss::Off.gain_db(100, 100, 300).unwrap(), 0.0);
    assert_eq!(
        cable(0.0, LengthUnit::Feet)
            .gain_db(750_000_000, 540_000_000, 1_000_000_000)
            .unwrap(),
        0.0
    );
}
#[test]
fn rejects_nonfinite_bounds_extrapolation_and_excessive_loss() {
    for length in [-1.0, 1000.1, f64::NAN, f64::INFINITY] {
        assert!(cable(length, LengthUnit::Feet).validate().is_err());
    }
    assert!(cable(304.81, LengthUnit::Metres).validate().is_err());
    for tilt_db in [-30.1, 30.1, f64::NAN] {
        assert!(AdditionalLoss::Manual { tilt_db }.validate().is_err());
    }
    let loss = cable(100.0, LengthUnit::Feet);
    assert!(loss.gain_db(999_999, 999_999, 2_000_000).is_err());
    assert!(loss
        .gain_db(3_000_000_001, 3_000_000_000, 3_000_000_001)
        .is_err());
    assert!(loss.gain_db(100, 100, 100).is_err());
    assert!(cable(1000.0, LengthUnit::Feet)
        .gain_db(1_000_000_000, 540_000_000, 1_000_000_000)
        .is_err());
}
#[test]
fn typed_serialization_preserves_models_and_rejects_unknown_cable_versions() {
    for model in [
        AdditionalLoss::Off,
        AdditionalLoss::Manual { tilt_db: 4.0 },
        cable(123.0, LengthUnit::Metres),
    ] {
        let encoded = serde_json::to_string(&model).unwrap();
        assert_eq!(
            serde_json::from_str::<AdditionalLoss>(&encoded).unwrap(),
            model
        );
    }
    assert!(serde_json::from_str::<AdditionalLoss>(
        r#"{"mode":"cable","cable":"generic_rg6","length":100,"unit":"feet"}"#
    )
    .is_err());
}
