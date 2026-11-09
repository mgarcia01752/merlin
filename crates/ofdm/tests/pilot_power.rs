/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit amplitude/power distinction from PHY I20 section7.5.15.4, PDF171.
use docsis_ofdm_fec::signal_processing::pilots::PilotPower;
#[test]
fn two_times_rms_is_four_times_power_and_six_db_boost() {
    for rms in [0.5, 1.0, 2.0] {
        let power = PilotPower::new(rms).unwrap();
        assert_eq!(power.reference_rms(), rms);
        assert_eq!(power.amplitude(), 2.0 * rms);
        assert_eq!(power.scale([1.0, 0.0]).unwrap(), [2.0 * rms, 0.0]);
        assert_eq!(power.scale([-1.0, 0.0]).unwrap(), [-2.0 * rms, 0.0]);
        let json = serde_json::to_value(power).unwrap();
        assert_eq!(
            json["pilot_power"].as_f64().unwrap() / json["reference_power"].as_f64().unwrap(),
            4.0
        );
        assert!((json["boost_db"].as_f64().unwrap() - 6.020_599_913_279_624).abs() < 1e-12);
    }
}
#[test]
fn invalid_references_underflow_and_overflow_are_rejected() {
    for rms in [
        0.0,
        -1.0,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MAX,
        f64::MIN_POSITIVE,
    ] {
        assert!(PilotPower::new(rms).is_err());
    }
    let power = PilotPower::new(1.0).unwrap();
    assert!(power.scale([f64::NAN, 0.0]).is_err());
    assert!(power.scale([0.0, f64::INFINITY]).is_err());
    assert!(power.scale([f64::MAX, 0.0]).is_err());
}
