/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Whole-domain normalization and actual complex multiplication independent cases.
mod common;
use docsis_ofdma::{configuration::Numerology, signal_processing::PreEqBank, types::Iq};

#[test]
fn defaults_full_domain_phase_and_invalid_bank_atomicity() {
    let config = common::configuration(Numerology::Khz50, 4, 1, 1, 8);
    let mut raw = vec![Iq { re: 0.0, im: 2.0 }; 2048];
    raw[0] = Iq::default();
    raw[1] = Iq::default();
    let bank = PreEqBank::normalized(&config, raw.clone(), 2).unwrap();
    assert!(bank.coefficients()[2..]
        .iter()
        .all(|z| *z == Iq { re: 0.0, im: 1.0 }));
    let mut grid = vec![Iq::default(); 2048];
    grid[128] = Iq { re: 2.0, im: 3.0 };
    assert_eq!(
        bank.apply(&config, &grid).unwrap()[128],
        Iq { re: -3.0, im: 2.0 }
    );
    let unity = PreEqBank::unity(&config);
    grid[129] = Iq { re: -0.0, im: -0.0 };
    let output = unity.apply(&config, &grid).unwrap();
    assert_eq!(output[129].re.to_bits(), grid[129].re.to_bits());
    assert_eq!(output[129].im.to_bits(), grid[129].im.to_bits());
    assert!(PreEqBank::normalized(&config, vec![Iq::default(); 2048], 3).is_err());
    raw[0].re = 1.0;
    assert!(PreEqBank::normalized(&config, raw, 3).is_err());
    assert_eq!(bank.generation(), 2);
    assert_eq!(
        bank.apply(&config, &grid).unwrap()[128],
        Iq { re: -3.0, im: 2.0 }
    );
}
