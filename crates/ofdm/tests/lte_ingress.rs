/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use docsis_ofdm_fec::{
    scenario::grid::FrequencyGrid, signal_processing::noise::lte_ingress::LteDownlinkIngress,
};

fn grid() -> FrequencyGrid {
    FrequencyGrid {
        zero_frequency_hz: 727_500_000,
        first_subcarrier_index: 0,
        spacing_hz: 250_000,
        sample_count: 81,
    }
}

fn model() -> LteDownlinkIngress {
    LteDownlinkIngress {
        center_frequency_hz: 737_500_000,
        interference_to_signal_db: -30.0,
    }
}

#[test]
fn independent_equal_power_and_shoulder_anchors() {
    let original = vec![Some(30.0); 81];
    let result = model().apply(&grid(), &original).unwrap();
    // At center: existing1/1000 + added1/1000 -> MER26.989700043360187dB.
    assert!((result.mer_db[40].unwrap() - 26.989_700_043_360_187).abs() < 1e-12);
    //732.75MHz: middle of the lower shoulder has half the core power.
    assert!((result.interference_to_signal_power[21] - 0.0005).abs() < 1e-15);
    assert!((result.mer_db[21].unwrap() - 28.239_087_409_443_19).abs() < 1e-12);
    assert_eq!(result.mer_db[20], Some(30.0)); // nominal lower edge732.5MHz
    assert_eq!(result.mer_db[60], Some(30.0)); // nominal upper edge742.5MHz
    assert_eq!(original, vec![Some(30.0); 81]);
}

#[test]
fn nulls_symmetry_and_physical_frequency_alignment() {
    let mut original = vec![Some(40.0); 81];
    original[40] = None;
    let result = model().apply(&grid(), &original).unwrap();
    assert_eq!(result.mer_db[40], None);
    assert_eq!(result.interference_to_signal_power[40], 0.001);
    for offset in 0..81 {
        assert!(
            (result.interference_to_signal_power[offset]
                - result.interference_to_signal_power[80 - offset])
                .abs()
                < 1e-15
        );
    }
    // Same physical grid with a different index origin must produce identical results.
    let shifted = FrequencyGrid {
        zero_frequency_hz: 702_500_000,
        first_subcarrier_index: 100,
        ..grid()
    };
    assert_eq!(
        model().apply(&shifted, &original).unwrap().mer_db,
        result.mer_db
    );
}

#[test]
fn rejects_invalid_power_readings_grids_and_coverage() {
    let original = vec![Some(40.0); 81];
    for power in [f64::NAN, f64::INFINITY, -4000.0, 4000.0] {
        let invalid = LteDownlinkIngress {
            interference_to_signal_db: power,
            ..model()
        };
        assert!(invalid.apply(&grid(), &original).is_err());
    }
    for reading in [f64::NAN, f64::INFINITY, -4000.0, 4000.0] {
        assert!(model().apply(&grid(), &vec![Some(reading); 81]).is_err());
    }
    assert!(model().apply(&grid(), &original[..80]).is_err());
    assert!(model()
        .apply(
            &FrequencyGrid {
                zero_frequency_hz: 800_000_000,
                ..grid()
            },
            &original
        )
        .is_err());
    assert!(LteDownlinkIngress {
        center_frequency_hz: u32::MAX,
        ..model()
    }
    .apply(&grid(), &original)
    .is_err());
    assert!(LteDownlinkIngress {
        center_frequency_hz: 729_000_000,
        ..model()
    }
    .apply(&grid(), &original)
    .is_err());
}
