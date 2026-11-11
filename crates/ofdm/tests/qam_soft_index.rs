/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent exhaustive-distance oracle for optimized, label-preserving soft bits.
use docsis_ofdm_fec::signal_processing::modulation::{qam_llrs, reference_for_order};

fn oracle(order: u16, received: [f64; 2], variance: f64) -> Option<Vec<f64>> {
    if received.iter().any(|sample| !sample.is_finite()) || !variance.is_finite() || variance <= 0.0
    {
        return None;
    }
    let table = reference_for_order(usize::from(order))?;
    let bits = usize::from(table.bit_loading);
    let mut minima = vec![[f64::INFINITY; 2]; bits];
    for (label, point) in table.iq.iter().enumerate() {
        let distance = (received[0] - point[0]).powi(2) + (received[1] - point[1]).powi(2);
        if !distance.is_finite() {
            return None;
        }
        for (bit, minimum) in minima.iter_mut().enumerate() {
            minimum[(label >> (bits - bit - 1)) & 1] =
                minimum[(label >> (bits - bit - 1)) & 1].min(distance);
        }
    }
    Some(
        minima
            .iter()
            .map(|minimum| ((minimum[1] - minimum[0]) / variance).clamp(-100.0, 100.0))
            .collect(),
    )
}
fn assert_exact(order: u16, point: [f64; 2], variance: f64) {
    let expected = oracle(order, point, variance);
    let actual = qam_llrs(order, point, variance);
    match (actual, expected) {
        (Ok(actual), Some(expected)) => assert_eq!(
            actual
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            "{order}-QAM {point:?}, variance {variance}"
        ),
        (Err(_), None) => {}
        (actual, expected) => panic!("{order}-QAM {point:?}: {actual:?} != {expected:?}"),
    }
}
fn check_order(order: u16) {
    let table = reference_for_order(usize::from(order)).unwrap();
    for label in [0, 1, order / 4, order / 2, order - 1] {
        let point = table.iq[usize::from(label)];
        assert_exact(order, point, 0.03);
        assert_exact(order, [point[0] + 0.013, point[1] - 0.007], 0.17);
        let other = table.iq[(usize::from(label) + 1) % usize::from(order)];
        assert_exact(
            order,
            [(point[0] + other[0]) / 2.0, (point[1] + other[1]) / 2.0],
            0.5,
        );
    }
    let mut seed = 0x5eed_u64;
    for _ in 0..32 {
        let point = std::array::from_fn(|_| {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            ((seed >> 32) as f64 / u32::MAX as f64 - 0.5) * 6.0
        });
        assert_exact(order, point, 0.37);
    }
    for point in [
        [0.0, 0.0],
        [-0.0, 0.0],
        [1e100, -1e100],
        [f64::MAX.sqrt() / 4.0, 0.0],
        [f64::MAX, 0.0],
        [f64::NAN, 0.0],
        [0.0, f64::INFINITY],
    ] {
        assert_exact(order, point, 1.0);
    }
    for variance in [
        f64::from_bits(1),
        f64::MIN_POSITIVE,
        1e-300,
        1e300,
        0.0,
        -1.0,
        f64::INFINITY,
        f64::NAN,
    ] {
        assert_exact(order, [0.013, -0.027], variance);
    }
}
macro_rules! order_case {
    ($name:ident, $order:literal) => {
        #[test]
        fn $name() {
            check_order($order);
        }
    };
}
order_case!(bpsk_matches_full_scan, 2);
order_case!(qpsk_matches_full_scan, 4);
order_case!(qam16_matches_full_scan, 16);
order_case!(qam64_matches_full_scan, 64);
order_case!(cross128_matches_full_scan, 128);
order_case!(qam256_matches_full_scan, 256);
order_case!(cross512_matches_full_scan, 512);
order_case!(qam1024_matches_full_scan, 1024);
order_case!(cross2048_matches_full_scan, 2048);
order_case!(qam4096_matches_full_scan, 4096);
order_case!(cross8192_matches_full_scan, 8192);
order_case!(qam16384_matches_full_scan, 16384);

#[test]
fn concurrent_first_use_keeps_reference_bits_identical() {
    let threads: Vec<_> = (0..8)
        .map(|index| {
            std::thread::spawn(move || {
                assert_exact(4096, [index as f64 / 10.0, -0.17], 0.1);
                assert_exact(8192, [0.31, index as f64 / 10.0], 0.2);
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
}
