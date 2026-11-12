/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent anchors for deliberate MER edits, not a physical RF channel model.
use docsis_ofdm_fec::{scenario::FrequencyGrid, signal_processing::rxmer_designer::*};
fn grid() -> FrequencyGrid {
    FrequencyGrid {
        zero_frequency_hz: 100_000_000,
        first_subcarrier_index: 0,
        spacing_hz: 1_000_000,
        sample_count: 9,
    }
}
#[test]
fn signed_tilt_and_exact_value_replacements_preserve_gaps() {
    let baseline = vec![
        Some(40.0),
        Some(40.0),
        None,
        Some(40.0),
        Some(40.0),
        Some(40.0),
        Some(40.0),
        Some(40.0),
        Some(40.0),
    ];
    let mut recipe = RxmerRecipe {
        tilt: Some(Tilt {
            scope: CaptureScope::All,
            tilt_db: -8.0,
        }),
        ..Default::default()
    };
    let values = recipe.apply(&grid(), &baseline, 1, 1).unwrap();
    assert_eq!(
        values,
        vec![
            Some(40.0),
            Some(39.0),
            None,
            Some(37.0),
            Some(36.0),
            Some(35.0),
            Some(34.0),
            Some(33.0),
            Some(32.0)
        ]
    );
    recipe.tilt.as_mut().unwrap().tilt_db = 8.0;
    recipe.edits.push(ValueEdit {
        scope: CaptureScope::All,
        first_hz: 102_000_000,
        last_hz: 104_000_000,
        value_db: 20.0,
    });
    let values = recipe.apply(&grid(), &baseline, 1, 1).unwrap();
    assert_eq!(&values[2..5], &[None, Some(20.0), Some(20.0)]);
    assert_eq!(values[8], Some(48.0));
    assert_eq!(baseline[0], Some(40.0));
}
#[test]
fn cosine_period_and_round_trip_distance_have_independent_anchors() {
    let recipe = RxmerRecipe {
        ripple: Some(Ripple {
            scope: CaptureScope::All,
            peak_to_peak_db: 4.0,
            spacing: RippleSpacing::Frequency {
                spacing_hz: 4_000_000,
            },
        }),
        ..Default::default()
    };
    let values = recipe.apply(&grid(), &[Some(40.0); 9], 1, 1).unwrap();
    for (index, expected) in [
        (0, 42.0),
        (1, 40.0),
        (2, 38.0),
        (4, 42.0),
        (6, 38.0),
        (8, 42.0),
    ] {
        assert!((values[index].unwrap() - expected).abs() < 1e-12);
    }
    assert_eq!(
        RippleSpacing::EchoDistance {
            distance_metres: 299_792_458.0 * 0.8 / (2.0 * 20_000_000.0),
            velocity_factor: 0.8
        }
        .frequency_hz()
        .unwrap(),
        20_000_000
    );
}
#[test]
fn notch_scope_is_inclusive_ten_through_twenty_of_fifty() {
    let recipe = RxmerRecipe {
        notch: Some(Notch {
            scope: CaptureScope::Range {
                first: 10,
                last: 20,
            },
            center_hz: 104_000_000,
            bandwidth_hz: 4_000_000,
            depth_db: 12.0,
            edge_hz: 1_000_000,
        }),
        ..Default::default()
    };
    let mut affected = 0;
    for position in 1..=50 {
        let values = recipe
            .apply(&grid(), &[Some(40.0); 9], position, 50)
            .unwrap();
        if (10..=20).contains(&position) {
            affected += 1;
            assert_eq!(values[4], Some(28.0));
            assert_eq!(values[2], Some(40.0));
            assert_eq!(values[3], Some(28.0));
        } else {
            assert_eq!(values, vec![Some(40.0); 9]);
        }
    }
    assert_eq!(affected, 11);
}
#[test]
fn invalid_bounds_and_outputs_reject_without_clipping() {
    let mut recipe = RxmerRecipe {
        tilt: Some(Tilt {
            scope: CaptureScope::Range { first: 0, last: 2 },
            tilt_db: 10.0,
        }),
        ..Default::default()
    };
    assert!(recipe.validate(&grid(), 2).is_err());
    recipe.tilt.as_mut().unwrap().scope = CaptureScope::All;
    assert!(recipe.apply(&grid(), &[Some(119.0); 9], 1, 1).is_err());
    recipe.edits.push(ValueEdit {
        scope: CaptureScope::All,
        first_hz: 100_000_001,
        last_hz: 101_000_000,
        value_db: 30.0,
    });
    assert!(recipe.validate(&grid(), 1).is_err());
    assert!(CaptureScope::All.validate(257).is_err());
    assert!(RippleSpacing::EchoDistance {
        distance_metres: 1.0,
        velocity_factor: 0.0
    }
    .frequency_hz()
    .is_err());
}
