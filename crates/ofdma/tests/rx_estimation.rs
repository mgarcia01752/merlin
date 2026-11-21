/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Engineering LS/ZF expectations from literal pilot signs and complex values.
use docsis_ofdma::{
    configuration::*,
    resources::ResourcePlan,
    signal_processing::{equalize, estimate},
    types::Iq,
};

fn configuration() -> UsConfiguration {
    let profile = Profile::new(
        1,
        5,
        1,
        vec![Minislot {
            first_carrier: 0,
            loading: 4,
            pilot_pattern: 1,
        }],
    )
    .unwrap();
    UsConfiguration::new(
        Waveform::new(Numerology::Khz50, 192, 64, 6, 20_000_000).unwrap(),
        vec![profile],
        SyntheticGrant {
            id: 1,
            profile_id: 1,
            epoch_sample: 100,
            first_frame: 0,
            frames: 1,
            minislots: vec![0],
        },
        vec![],
    )
    .unwrap()
}

#[test]
fn literal_flat_complex_ls_zero_forcing_and_unobserved_failure() {
    let config = configuration();
    let plan = ResourcePlan::new(&config).unwrap();
    let mut symbols = vec![Some(vec![Iq::default(); 2048]); 6];
    // Physical bins 0 and 7 have pilot sign -1; Figure 39 P is at t=0,2.
    for t in [0, 2] {
        for k in [0, 7] {
            symbols[t].as_mut().unwrap()[k] = Iq { re: -2.0, im: -3.0 };
        }
    }
    let fitted = estimate(&config, &plan, &symbols, &Estimator::FrameFlatLs).unwrap();
    assert_eq!(fitted[3], Some(Iq { re: 2.0, im: 3.0 }));
    let z = equalize(Iq { re: 4.0, im: 6.0 }, fitted[3], 1e-12)
        .unwrap()
        .unwrap();
    assert!((z - Iq { re: 2.0, im: 0.0 }).power() < 1e-28);
    assert_eq!(fitted[8], None);
    assert_eq!(
        equalize(Iq::default(), Some(Iq::default()), 1e-12).unwrap(),
        None
    );
    assert!(
        estimate(&config, &plan, &vec![None; 6], &Estimator::FrameFlatLs)
            .unwrap()
            .iter()
            .all(Option::is_none)
    );
    symbols[0] = None;
    assert_eq!(
        estimate(&config, &plan, &symbols, &Estimator::FrameFlatLs).unwrap()[3],
        Some(Iq { re: 2.0, im: 3.0 })
    );
}

#[test]
fn linear_brackets_gaps_and_within_frame_step_never_use_hidden_truth() {
    let config = configuration();
    let plan = ResourcePlan::new(&config).unwrap();
    let mut symbols = vec![Some(vec![Iq::default(); 2048]); 6];
    for t in [0, 2] {
        symbols[t].as_mut().unwrap()[0] = Iq { re: -1.0, im: 0.0 };
        symbols[t].as_mut().unwrap()[7] = Iq { re: -3.0, im: -2.0 };
    }
    let fitted = estimate(
        &config,
        &plan,
        &symbols,
        &Estimator::FramePilotLinear { maximum_gap: 7 },
    )
    .unwrap();
    let expected = Iq {
        re: 1.0 + 6.0 / 7.0,
        im: 6.0 / 7.0,
    };
    assert!((fitted[3].unwrap() - expected).power() < 1e-28);
    assert_eq!(
        estimate(
            &config,
            &plan,
            &symbols,
            &Estimator::FramePilotLinear { maximum_gap: 6 }
        )
        .unwrap()[3],
        None
    );
    symbols[0].as_mut().unwrap()[7] = Iq { re: -1.0, im: 0.0 };
    symbols[2].as_mut().unwrap()[0] = Iq { re: -3.0, im: -2.0 };
    let flat = estimate(&config, &plan, &symbols, &Estimator::FrameFlatLs).unwrap();
    assert_eq!(flat[3], Some(Iq { re: 2.0, im: 1.0 }));
}
