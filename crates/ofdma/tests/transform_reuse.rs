/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Frozen upstream equivalence and measured cost of typed conversion.
#[path = "support/legacy_transform.rs"]
mod legacy;
use docsis_ofdma::{configuration::Numerology, signal_processing::Transform, types::Iq, PhyError};
use std::{hint::black_box, time::Instant};

#[test]
fn shared_transform_preserves_source_bits_and_error_fields() {
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let length = mode.fft_len();
        let old = legacy::Transform::new(mode);
        let shared = Transform::new(mode);
        let input: Vec<_> = (0..length)
            .map(|i| Iq {
                re: ((i * 29 % 97) as f64 - 48.0) / 64.0,
                im: ((i * 11 % 89) as f64 - 44.0) / 32.0,
            })
            .collect();
        for (before, after) in [
            (
                old.inverse(&input).unwrap(),
                shared.inverse(&input).unwrap(),
            ),
            (
                old.forward(&input).unwrap(),
                shared.forward(&input).unwrap(),
            ),
        ] {
            for (a, b) in before.iter().zip(after) {
                assert_eq!(
                    [a.re.to_bits(), a.im.to_bits()],
                    [b.re.to_bits(), b.im.to_bits()]
                );
            }
        }
        assert_eq!(
            shared.forward(&[]),
            Err(PhyError::InvalidInput {
                field: "transform.input"
            })
        );
        let mut overflow = vec![
            Iq {
                re: f64::MAX,
                im: f64::MAX
            };
            length
        ];
        assert_eq!(
            shared.inverse(&overflow),
            Err(PhyError::InvalidInput {
                field: "transform.output"
            })
        );
        overflow[0].re = f64::NAN;
        assert_eq!(
            shared.inverse(&overflow),
            Err(PhyError::InvalidInput {
                field: "transform.input"
            })
        );
        assert_eq!(
            shared.forward(&input).unwrap(),
            old.forward(&input).unwrap()
        );
    }
}

#[test]
#[ignore = "explicit host performance observation, not a timing assertion"]
fn benchmark_typed_transform_adapter() {
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let input = vec![Iq { re: 0.25, im: -0.5 }; mode.fft_len()];
        let old = legacy::Transform::new(mode);
        let shared = Transform::new(mode);
        for _ in 0..50 {
            black_box(old.inverse(black_box(&input)).unwrap());
            black_box(shared.inverse(black_box(&input)).unwrap());
        }
        let mut timings = [Vec::new(), Vec::new()];
        for batch in 0..15 {
            for side in if batch % 2 == 0 { [0, 1] } else { [1, 0] } {
                let start = Instant::now();
                for _ in 0..100 {
                    if side == 0 {
                        black_box(old.inverse(black_box(&input)).unwrap());
                    } else {
                        black_box(shared.inverse(black_box(&input)).unwrap());
                    }
                }
                timings[side].push(start.elapsed().as_secs_f64() * 10_000.0);
            }
        }
        timings
            .iter_mut()
            .for_each(|values| values.sort_by(f64::total_cmp));
        eprintln!(
            "N={} source_us={:.3} shared_adapter_us={:.3} ratio={:.3}",
            mode.fft_len(),
            timings[0][7],
            timings[1][7],
            timings[1][7] / timings[0][7]
        );
    }
}
