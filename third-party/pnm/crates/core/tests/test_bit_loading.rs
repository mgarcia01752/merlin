/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent target-policy boundary cases, separate from binary parsing tests.
use pnm::common::signal_processing::bit_loading::{is_data_bit_loading, BitLoadingModel};
#[test]
fn explicit_models_are_distinct_and_do_not_invent_orders() {
    use BitLoadingModel::*;
    assert_eq!(WebuiMerHeuristic.supported_bits(39.25, 0.0, 13), Some(10));
    assert_eq!(
        OssiCandidateDefaults.supported_bits(39.25, 0.0, 13),
        Some(11)
    );
    assert_eq!(IdealShannon.supported_bits(39.25, 0.0, 13), Some(13));
    assert_eq!(IdealShannon.supported_bits(10.0, 0.0, 13), Some(2));
    assert_eq!(IdealShannon.supported_bits(0.0, 0.0, 13), None);
    assert_eq!(WebuiMerHeuristic.target_db(13), None);
    assert_eq!(WebuiMerHeuristic.supported_bits(63.5, 0.0, 13), Some(12));
    assert_eq!(
        OssiCandidateDefaults.supported_bits(63.5, 0.0, 13),
        Some(13)
    );
    assert_eq!(
        OssiCandidateDefaults.supported_bits(63.5, 0.0, 14),
        Some(14)
    );
    for bits in [0, 1, 3, 5, 15, 20, 255] {
        assert!(!is_data_bit_loading(bits));
        assert_eq!(IdealShannon.target_db(bits), None);
    }
    assert_eq!(BitLoadingModel::from_name("unknown"), None);
    assert_eq!(
        OssiCandidateDefaults.supported_bits(f64::NAN, 0.0, 13),
        None
    );
}
#[test]
fn each_policy_boundary_and_additional_margin_is_explicit() {
    use BitLoadingModel::*;
    for (model, cases) in [
        (
            WebuiMerHeuristic,
            vec![(8, 33.0), (9, 36.0), (10, 39.0), (11, 42.0), (12, 45.0)],
        ),
        (
            OssiCandidateDefaults,
            vec![
                (4, 15.0),
                (6, 21.0),
                (7, 24.0),
                (8, 27.0),
                (9, 30.5),
                (10, 34.0),
                (11, 37.0),
                (12, 41.0),
                (13, 46.0),
                (14, 52.0),
            ],
        ),
    ] {
        for (bits, target) in cases {
            assert_eq!(model.target_db(bits), Some(target));
            assert_eq!(model.supported_bits(target, 0.0, bits), Some(bits));
            assert_ne!(model.supported_bits(target - 0.25, 0.0, bits), Some(bits));
            assert_eq!(model.supported_bits(target + 0.25, 0.25, bits), Some(bits));
            assert_ne!(model.supported_bits(target, 0.25, bits), Some(bits));
        }
    }
}
