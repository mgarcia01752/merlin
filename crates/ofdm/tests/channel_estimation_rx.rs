/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    signal_processing::receiver::channel_estimation::{PilotChannelEstimator, ReceiverMode},
    simulation::{
        channel::{ChannelPhase, ChannelRun, ChannelRunSettings},
        pilots::PilotGenerator,
        test_points::{NoopObserver, TestPointProbe},
    },
    transmission::{
        layout::{CarrierLayout, PhysicalCarrierRole},
        SubcarrierSpacing,
    },
};
fn multiply(left: [f64; 2], right: [f64; 2]) -> [f64; 2] {
    [
        left[0] * right[0] - left[1] * right[1],
        left[0] * right[1] + left[1] * right[0],
    ]
}
#[test]
fn independent_gain_delay_and_all_phases_at_both_spacings() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let (transmission, raw) = acquisition_support::acquisition_setup(spacing, false);
        let layout = CarrierLayout::new(&transmission, &raw).unwrap();
        let mut estimator = PilotChannelEstimator::new(layout, 1.0).unwrap();
        let layout = CarrierLayout::new(&transmission, &raw).unwrap();
        let generator = PilotGenerator::new(spacing);
        for symbol in 0..256 {
            let pilots = generator.generate(&layout, symbol, 1.0).unwrap();
            let mut bins = vec![[0.0, 0.0]; spacing.fft_samples() as usize];
            for pilot in pilots.continuous.iter().chain(&pilots.scattered) {
                let phase = 0.2 + f64::from(pilot.bin) * 0.13;
                bins[usize::from(pilot.bin)] =
                    multiply(pilot.iq, [1.3 * phase.cos(), 1.3 * phase.sin()]);
            }
            let estimate = estimator.estimate(symbol, &bins).unwrap();
            assert_eq!(estimate.ready, symbol >= 127);
            assert!(estimate.maximum_age_symbols < 128);
            if symbol >= 127 {
                for (bin, role) in layout.static_roles().iter().enumerate() {
                    if *role != PhysicalCarrierRole::Excluded {
                        let phase = 0.2 + bin as f64 * 0.13;
                        let expected = [1.3 * phase.cos(), 1.3 * phase.sin()];
                        for (actual, expected) in estimate.response[bin].iter().zip(expected) {
                            assert!(
                                (actual - expected).abs() < 1e-10,
                                "bin={bin} symbol={symbol}"
                            );
                        }
                    }
                }
            }
        }
    }
}
#[test]
fn bad_input_is_atomic_and_prior_prediction_does_not_fit_current_noise() {
    let (transmission, raw) =
        acquisition_support::acquisition_setup(SubcarrierSpacing::Khz50, false);
    let layout = CarrierLayout::new(&transmission, &raw).unwrap();
    let generator = PilotGenerator::new(transmission.subcarrier_spacing);
    let mut estimator =
        PilotChannelEstimator::new(CarrierLayout::new(&transmission, &raw).unwrap(), 1.0).unwrap();
    let make_bins = |symbol| {
        let pilots = generator.generate(&layout, symbol, 1.0).unwrap();
        let mut bins = vec![[0.0, 0.0]; 4096];
        for pilot in pilots.continuous.iter().chain(&pilots.scattered) {
            bins[usize::from(pilot.bin)] = pilot.iq;
        }
        bins
    };
    let first = estimator.estimate(0, &make_bins(0)).unwrap();
    assert!(first
        .pilots
        .iter()
        .all(|pilot| pilot.prior_prediction.is_none()));
    assert!(estimator.estimate(2, &make_bins(2)).is_err());
    let mut invalid = make_bins(1);
    invalid[0] = [f64::NAN, 0.0];
    assert!(estimator.estimate(1, &invalid).is_err());
    let mut bins = make_bins(1);
    let bin = first
        .pilots
        .iter()
        .find(|pilot| !pilot.scattered)
        .unwrap()
        .bin as usize;
    let mut zero = bins.clone();
    zero[bin] = [0.0, 0.0];
    assert!(estimator.estimate(1, &zero).is_err());
    let mut enormous = bins.clone();
    enormous[bin] = [1e200, 0.0];
    assert!(estimator.estimate(1, &enormous).is_err());
    bins[bin][0] *= 1.1;
    let second = estimator.estimate(1, &bins).unwrap();
    let pilot = second
        .pilots
        .iter()
        .find(|pilot| pilot.bin as usize == bin)
        .unwrap();
    assert!((pilot.response[0] - 1.1).abs() < 1e-12);
    assert!((pilot.prior_prediction.unwrap()[0] - 1.0).abs() < 1e-12);
}
#[test]
fn real_pilot_receiver_acquires_and_recovers_prbs_at_both_spacings() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let (transmission, raw) = acquisition_support::acquisition_setup(spacing, false);
        let mut settings = ChannelRunSettings::new(transmission, raw);
        settings.duration = std::time::Duration::from_micros(100);
        settings.receiver_mode = ReceiverMode::PilotEstimated;
        settings.protected_constellation = Some(Default::default());
        let mut run = ChannelRun::new(&settings).unwrap();
        while run
            .advance(&mut TestPointProbe::disabled(), &mut NoopObserver)
            .unwrap()
            .is_some()
        {}
        assert_eq!(run.summary().phase, ChannelPhase::Finished);
        assert_eq!(run.summary().comparison.bit_errors, 0);
        assert!(run.summary().comparison.bits_tested > 0);
        assert_eq!(run.summary().acquisition_symbols, 128);
        let capture = run.take_protected_constellation().unwrap();
        assert!(!capture.plc_preamble.pairs.is_empty());
        assert!(capture
            .continuous_pilots
            .pairs
            .iter()
            .any(|pair| !pair.receiver_estimate_ready));
    }
}
