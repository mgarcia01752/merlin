/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit bounded release measurement; not a timing-dependent ordinary test.
use merlin_dsp::{
    prbs15,
    transform::{TransformPlan, TransformSize},
};
use std::{hint::black_box, time::Instant};
#[path = "support/legacy_transform.rs"]
mod legacy;

fn elapsed_micros<T>(operation: &mut impl FnMut() -> T, repeats: usize) -> f64 {
    let started = Instant::now();
    for _ in 0..repeats {
        black_box(operation());
    }
    started.elapsed().as_secs_f64() * 1e6 / repeats as f64
}

fn paired_micros<A, B>(
    mut old: impl FnMut() -> A,
    mut new: impl FnMut() -> B,
    repeats: usize,
) -> (f64, f64) {
    for _ in 0..16 {
        black_box(old());
        black_box(new());
    }
    let mut old_samples = Vec::new();
    let mut new_samples = Vec::new();
    for sample in 0..15 {
        if sample % 2 == 0 {
            old_samples.push(elapsed_micros(&mut old, repeats));
            new_samples.push(elapsed_micros(&mut new, repeats));
        } else {
            new_samples.push(elapsed_micros(&mut new, repeats));
            old_samples.push(elapsed_micros(&mut old, repeats));
        }
    }
    old_samples.sort_by(f64::total_cmp);
    new_samples.sort_by(f64::total_cmp);
    (old_samples[7], new_samples[7])
}

#[test]
#[ignore = "explicit release benchmark; timings are host observations"]
fn benchmark_fft_and_prbs_kernels() {
    for size in [
        TransformSize::Samples2048,
        TransformSize::Samples4096,
        TransformSize::Samples8192,
    ] {
        let length = size.samples();
        let input: Vec<_> = (0..length)
            .map(|i| [(i % 31) as f64 / 32.0, (i % 13) as f64 / 16.0])
            .collect();
        let shared = TransformPlan::new(size);
        let former = legacy::TransformPlan::new(length);
        assert_eq!(
            shared.inverse(&input).unwrap(),
            former.apply(&input, true).unwrap()
        );
        let (old_setup, new_setup) = paired_micros(
            || legacy::TransformPlan::new(black_box(length)),
            || TransformPlan::new(black_box(size)),
            64,
        );
        let (old_apply, new_apply) = paired_micros(
            || former.apply(black_box(&input), true).unwrap(),
            || shared.inverse(black_box(&input)).unwrap(),
            128,
        );
        println!("FFT {length}: setup old/new {old_setup:.3}/{new_setup:.3} us; inverse old/new {old_apply:.3}/{new_apply:.3} us; ratio {:.3}",new_apply/old_apply);
    }
    let generate = |shared: bool| {
        let mut state = black_box(0x1234u16);
        (0..1_048_576)
            .map(|_| {
                if shared {
                    let (next, output) = prbs15::step(state);
                    state = next;
                    output
                } else {
                    let output = (state & 1) as u8;
                    let feedback = (state ^ (state >> 14)) & 1;
                    state = (state >> 1) | (feedback << 14);
                    output
                }
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(generate(true), generate(false));
    let (old, new) = paired_micros(
        || generate(black_box(false)),
        || generate(black_box(true)),
        4,
    );
    println!(
        "PRBS 1048576 bits: old/new {old:.3}/{new:.3} us; ratio {:.3}",
        new / old
    );
}
