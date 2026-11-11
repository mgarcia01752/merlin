/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Manual old/new demapper kernel timing; not a correctness or CI timing threshold.
use docsis_ofdm_fec::signal_processing::modulation::{qam_llrs, reference_for_order};
use std::{hint::black_box, time::Instant};

#[inline(never)]
fn full_scan(order: u16, received: [f64; 2], variance: f64) -> Vec<f64> {
    let table = reference_for_order(usize::from(order)).unwrap();
    let bits = usize::from(table.bit_loading);
    assert!(
        received.iter().all(|value| value.is_finite()) && variance.is_finite() && variance > 0.0
    );
    let mut output = vec![0.0; bits];
    let mut minima = [[f64::INFINITY; 2]; 14];
    for (label, point) in table.iq.iter().enumerate() {
        let distance = (received[0] - point[0]).powi(2) + (received[1] - point[1]).powi(2);
        assert!(distance.is_finite());
        for (bit, minimum) in minima[..bits].iter_mut().enumerate() {
            let value = (label >> (bits - 1 - bit)) & 1;
            minimum[value] = minimum[value].min(distance);
        }
    }
    for (value, minimum) in output.iter_mut().zip(&minima) {
        *value = ((minimum[1] - minimum[0]) / variance).clamp(-100.0, 100.0);
    }
    output
}
/// Compares equivalent finite-input batches and prints cold/warm kernel timing.
///
/// An optional cell count is bounded to1..65536. Every output checksum must match;
/// timing ratios are measurements only and are never used as a test threshold.
fn main() {
    let cells: usize = std::env::args()
        .nth(1)
        .map(|value| value.parse().expect("Positive cell count"))
        .unwrap_or(512);
    assert!((1..=65536).contains(&cells));
    println!("order,cells,cold_ms,full_scan_ms,indexed_ms,speedup,checksum");
    for order in [2, 4, 16, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384] {
        let points: Vec<_> = (0..cells)
            .map(|index| {
                [
                    ((index * 37 % 257) as f64 / 128.0 - 1.0) * 1.2,
                    ((index * 53 % 251) as f64 / 125.0 - 1.0) * 1.2,
                ]
            })
            .collect();
        let cold = Instant::now();
        black_box(qam_llrs(order, black_box(points[0]), 0.37).unwrap());
        let cold_ms = cold.elapsed().as_secs_f64() * 1000.0;
        let mut old_times = Vec::new();
        let mut new_times = Vec::new();
        let mut checksum = 0.0;
        for _ in 0..3 {
            let start = Instant::now();
            let mut expected = 0.0;
            for point in &points {
                expected += black_box(full_scan(order, black_box(*point), 0.37))
                    .iter()
                    .sum::<f64>();
            }
            old_times.push(start.elapsed().as_secs_f64() * 1000.0);
            let start = Instant::now();
            let mut actual = 0.0;
            for point in &points {
                actual += black_box(qam_llrs(order, black_box(*point), 0.37).unwrap())
                    .iter()
                    .sum::<f64>();
            }
            new_times.push(start.elapsed().as_secs_f64() * 1000.0);
            assert_eq!(actual.to_bits(), expected.to_bits());
            checksum = actual;
        }
        old_times.sort_unstable_by(f64::total_cmp);
        new_times.sort_unstable_by(f64::total_cmp);
        let old_ms = old_times[1];
        let new_ms = new_times[1];
        println!(
            "{order},{cells},{cold_ms:.6},{old_ms:.6},{new_ms:.6},{:.2},{checksum:.12}",
            old_ms / new_ms
        );
    }
}
