/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Literal independent data-reference energy and honest empty/zero-error states.
use docsis_ofdma::{
    instruments::{MatchedMer, MerValue},
    types::Iq,
};
#[test]
fn explicit_empty_zero_error_and_literal_twenty_db() {
    let mut metric = MatchedMer::default();
    assert_eq!(metric.value(), MerValue::Unavailable);
    metric
        .observe(Iq { re: 1.0, im: 0.0 }, Iq { re: 1.0, im: 0.0 })
        .unwrap();
    assert_eq!(metric.value(), MerValue::ZeroError);
    let mut metric = MatchedMer::default();
    metric
        .observe(Iq { re: 1.0, im: 0.0 }, Iq { re: 1.0, im: 0.1 })
        .unwrap();
    match metric.value() {
        MerValue::FiniteDb(db) => assert!((db - 20.0).abs() < 1e-13),
        other => panic!("{other:?}"),
    }
    let before = metric.clone();
    assert!(metric
        .observe(
            Iq {
                re: f64::NAN,
                im: 0.0
            },
            Iq::default()
        )
        .is_err());
    assert_eq!(metric, before);
}
