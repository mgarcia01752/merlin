/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Independent numbered copies, half-sample weights and adjacent-symbol sums.
use docsis_ofdma::{
    configuration::{Numerology, Waveform},
    signal_processing::{cyclic_extension, transmit_window},
    transmitter::BurstAssembler,
    types::{Identity, Iq},
};

#[test]
fn numbered_prefix_suffix_and_exact_two_symbol_timeline() {
    let geometry = Waveform::new(Numerology::Khz50, 192, 64, 6, 20_000_000).unwrap();
    let x: Vec<_> = (0..2048)
        .map(|i| Iq {
            re: i as f64,
            im: -(i as f64),
        })
        .collect();
    let y = cyclic_extension(&geometry, &x).unwrap();
    assert_eq!(y.len(), 2304);
    assert_eq!(&y[..192], &x[1856..]);
    assert_eq!(&y[192..2240], &x);
    assert_eq!(&y[2240..], &x[..64]);
    let window = transmit_window(&geometry);
    assert!(window[0] > 0.0);
    for q in 0..64 {
        assert!((window[q] + window[2240 + q] - 1.0).abs() < 1e-15);
        assert!((window[q] - window[2303 - q]).abs() < 1e-15);
    }
    let id = Identity {
        run: 1,
        generation: 1,
        grant: 7,
    };
    let mut assembly = BurstAssembler::new(id, geometry, 73, 2).unwrap();
    assert!(assembly.finish().is_err());
    let first = assembly.push(&vec![Iq { re: 1.0, im: 0.0 }; 2048]).unwrap();
    let second = assembly.push(&vec![Iq { re: 3.0, im: 0.0 }; 2048]).unwrap();
    assert_eq!(
        (first.start, first.samples.len(), second.start),
        (73, 2240, 2313)
    );
    for (q, weight) in window.iter().take(64).enumerate() {
        assert!((second.samples[q].re - (1.0 + 2.0 * weight)).abs() < 1e-14);
    }
    assert!(second.samples[192..].iter().all(|z| z.re == 3.0));
    let tail = assembly.finish().unwrap();
    assert_eq!((tail.start, tail.samples.len()), (4553, 64));
    assert!(assembly.finish().unwrap().samples.is_empty());
    assert!(assembly.push(&x).is_err());
}
