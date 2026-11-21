/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Independent power, complete-bank, corresponding-probe and symbol boundary cases.
mod common;
use docsis_ofdma::{
    configuration::Numerology, measurements::*, signal_processing::PreEqBank, types::Iq,
};

#[test]
fn literal_power_average_hold_interpolation_nulls_and_boundaries() {
    let config = common::configuration(Numerology::Khz50, 4, 1, 2, 8);
    let n = 2048;
    let mut db10 = vec![Some(10.0); n];
    db10[0] = None;
    let mut db20 = vec![Some(20.0); n];
    db20[0] = None;
    let sources = vec![
        SyntheticRxmer::new(&config, 1, db10).unwrap(),
        SyntheticRxmer::new(&config, 2, db20).unwrap(),
    ];
    let average = RxmerPlayback::new(&config, sources.clone(), RxmerPolicy::Average).unwrap();
    let value = average.at(0).unwrap();
    assert_eq!(value.powers[0], None);
    assert!((value.powers[128].unwrap() - 0.055).abs() < 1e-15);
    assert_eq!(value.contributors, vec![(1, 0.5), (2, 0.5)]);
    let hold = RxmerPlayback::new(
        &config,
        sources.clone(),
        RxmerPolicy::Hold {
            offsets: vec![0, 10],
        },
    )
    .unwrap();
    assert_eq!(hold.at(9).unwrap().powers[128], Some(0.1));
    assert_eq!(hold.at(10).unwrap().powers[128], Some(0.01));
    assert_eq!(hold.at(11).unwrap(), hold.at(10).unwrap());
    let linear = RxmerPlayback::new(
        &config,
        sources.clone(),
        RxmerPolicy::Interpolate {
            offsets: vec![0, 10],
        },
    )
    .unwrap();
    assert_eq!(linear.at(5).unwrap(), value);
    assert!(linear.at(12).is_err());
    assert!(RxmerPlayback::new(&config, sources.clone(), RxmerPolicy::Single).is_err());
    assert!(RxmerPlayback::new(
        &config,
        vec![sources[0].clone(), sources[0].clone()],
        RxmerPolicy::Average
    )
    .is_err());
    assert!(RxmerPlayback::new(
        &config,
        sources.clone(),
        RxmerPolicy::Hold {
            offsets: vec![1, 10]
        }
    )
    .is_err());
    assert!(RxmerPlayback::new(
        &config,
        sources,
        RxmerPolicy::Hold {
            offsets: vec![0, 12]
        }
    )
    .is_err());
    let mut missing = vec![Some(20.0); n];
    missing[128] = None;
    let source = SyntheticRxmer::new(&config, 3, missing).unwrap();
    assert!(RxmerPlayback::new(&config, vec![source], RxmerPolicy::Single).is_err());
    for value in [f64::NAN, f64::INFINITY, -4000.0, 4000.0] {
        assert!(SyntheticRxmer::new(&config, 4, vec![Some(value); n]).is_err());
    }
    let extreme = SyntheticRxmer::new(&config, 5, vec![Some(-70.0); n]).unwrap();
    assert!(
        RxmerPlayback::new(&config, vec![extreme], RxmerPolicy::Single)
            .unwrap()
            .validate_model(&config)
            .is_err()
    );
}

#[test]
fn snapshots_replace_and_adjustments_use_explicit_older_probe_base() {
    let config = common::configuration(Numerology::Khz50, 4, 1, 2, 8);
    let initial = PreEqBank::unity(&config);
    let mut imaginary = vec![Iq { re: 0.0, im: 1.0 }; 2048];
    imaginary[0] = Iq::default();
    imaginary[1] = Iq::default();
    let imaginary = PreEqBank::normalized(&config, imaginary, 2).unwrap();
    let events = vec![
        PreEqEvent {
            id: 2,
            symbol: 2,
            operation: PreEqOperation::Snapshot(imaginary),
        },
        PreEqEvent {
            id: 3,
            symbol: 4,
            operation: PreEqOperation::Adjust {
                base_bank: 1,
                probe: 10,
                values: vec![(128, Iq { re: -1.0, im: 0.0 })],
            },
        },
        PreEqEvent {
            id: 4,
            symbol: 6,
            operation: PreEqOperation::Initialize {
                base_bank: 3,
                probe: 11,
                values: vec![(129, Iq { re: 0.0, im: 1.0 })],
            },
        },
    ];
    let probes = vec![
        ProbeReference { id: 10, bank_id: 1 },
        ProbeReference { id: 11, bank_id: 3 },
    ];
    let replay =
        PreEqPlayback::new(&config, 1, initial.clone(), events.clone(), probes.clone()).unwrap();
    assert!(replay.at(1).unwrap().is_unity());
    assert_eq!(
        replay.at(2).unwrap().coefficients()[128],
        Iq { re: 0.0, im: 1.0 }
    );
    // At 4 the older unity probe base is used, not the most recent all-j bank.
    assert_eq!(
        replay.at(4).unwrap().coefficients()[128],
        Iq { re: -1.0, im: 0.0 }
    );
    assert_eq!(
        replay.at(4).unwrap().coefficients()[129],
        Iq { re: 1.0, im: 0.0 }
    );
    assert_eq!(
        replay.at(6).unwrap().coefficients()[129],
        Iq { re: 0.0, im: 1.0 }
    );
    assert_eq!(replay.at(11).unwrap(), replay.at(6).unwrap());
    assert!(!replay.all_unity());
    assert!(replay.at(12).is_err());
    let mut bad = events.clone();
    bad[1].symbol = 2;
    assert!(PreEqPlayback::new(&config, 1, initial.clone(), bad, probes.clone()).is_err());
    assert!(PreEqPlayback::new(&config, 1, initial.clone(), events.clone(), vec![]).is_err());
    let mut future = events;
    future[1].operation = PreEqOperation::Adjust {
        base_bank: 4,
        probe: 11,
        values: vec![(128, Iq { re: 1.0, im: 0.0 })],
    };
    assert!(PreEqPlayback::new(&config, 1, initial, future, probes).is_err());
}
