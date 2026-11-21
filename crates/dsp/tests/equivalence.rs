/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent analytical vectors plus frozen pre-extraction numerical equivalence.
use merlin_dsp::{
    prbs15,
    transform::{TransformPlan, TransformSize},
    ComplexSample, DspError,
};
use std::{collections::HashSet, f64::consts::TAU};
#[path = "support/legacy_transform.rs"]
mod legacy;

const SIZES: [TransformSize; 3] = [
    TransformSize::Samples2048,
    TransformSize::Samples4096,
    TransformSize::Samples8192,
];

fn dense(length: usize) -> Vec<ComplexSample> {
    (0..length)
        .map(|index| {
            [
                (index * 31 % 1021) as f64 / 1024.0 - 0.5,
                (index * 73 % 1021) as f64 / 1024.0 - 0.5,
            ]
        })
        .collect()
}
fn assert_bits(left: &[ComplexSample], right: &[ComplexSample]) {
    assert_eq!(left.len(), right.len());
    for (left, right) in left.iter().flatten().zip(right.iter().flatten()) {
        assert_eq!(left.to_bits(), right.to_bits());
    }
}
fn close(left: f64, right: f64) {
    assert!(
        (left - right).abs() < 3e-9,
        "{left} differs from independent {right}"
    );
}

#[test]
fn all_geometries_match_frozen_former_kernel_bit_for_bit() {
    for size in SIZES {
        let length = size.samples();
        let current = TransformPlan::new(size);
        let former = legacy::TransformPlan::new(length);
        for mut input in [dense(length), vec![[0.0, -0.0]; length]] {
            input[0][1] = -0.0;
            assert_bits(
                &current.inverse(&input).unwrap(),
                &former.apply(&input, true).unwrap(),
            );
            assert_bits(
                &current.forward(&input).unwrap(),
                &former.apply(&input, false).unwrap(),
            );
        }
    }
}

#[test]
fn independent_centered_tones_and_impulses_cover_both_signs_and_scales() {
    for size in SIZES {
        let length = size.samples();
        let transform = TransformPlan::new(size);
        let scale = 1.0 / (length as f64).sqrt();
        for bin in [0, length / 2, length / 2 + 1, length - 1] {
            let mut bins = vec![[0.0, 0.0]; length];
            bins[bin] = [1.0, 0.0];
            let time = transform.inverse(&bins).unwrap();
            for tick in [0, 1, 17, length / 2, length - 1] {
                let angle = TAU * tick as f64 * (bin as f64 - length as f64 / 2.0) / length as f64;
                close(time[tick][0], angle.cos() * scale);
                close(time[tick][1], angle.sin() * scale);
            }
        }
        let mut time = vec![[0.0, 0.0]; length];
        time[1] = [1.0, 0.0];
        let bins = transform.forward(&time).unwrap();
        for bin in [0, 1, length / 2, length - 1] {
            let angle = -TAU * (bin as f64 - length as f64 / 2.0) / length as f64;
            close(bins[bin][0], angle.cos() * scale);
            close(bins[bin][1], angle.sin() * scale);
        }
    }
}

#[test]
fn dense_grid_matches_independent_direct_sums_and_parseval() {
    for size in SIZES {
        let length = size.samples();
        let bins = dense(length);
        let output = TransformPlan::new(size).inverse(&bins).unwrap();
        for tick in [0, 1, 17, length / 2, length - 1] {
            let mut expected = [0.0, 0.0];
            for (bin, [real, imaginary]) in bins.iter().enumerate() {
                let angle = TAU * tick as f64 * (bin as f64 - length as f64 / 2.0) / length as f64;
                expected[0] += real * angle.cos() - imaginary * angle.sin();
                expected[1] += real * angle.sin() + imaginary * angle.cos();
            }
            close(output[tick][0], expected[0] / (length as f64).sqrt());
            close(output[tick][1], expected[1] / (length as f64).sqrt());
        }
        let energy = |values: &[ComplexSample]| values.iter().flatten().map(|v| v * v).sum::<f64>();
        close(energy(&bins), energy(&output));
    }
}

#[test]
fn invalid_shape_nonfinite_overflow_and_rejection_leave_plan_reusable() {
    for size in SIZES {
        let transform = TransformPlan::new(size);
        let valid = dense(size.samples());
        let expected = transform.forward(&valid).unwrap();
        assert_eq!(transform.forward(&[]), Err(DspError::InvalidSamples));
        for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut input = valid.clone();
            input[1][1] = invalid;
            assert_eq!(transform.inverse(&input), Err(DspError::InvalidSamples));
        }
        assert_eq!(
            transform.inverse(&vec![[f64::MAX, f64::MAX]; size.samples()]),
            Err(DspError::NonfiniteOutput)
        );
        assert_bits(&transform.forward(&valid).unwrap(), &expected);
        std::thread::scope(|scope| {
            for _ in 0..4 {
                scope.spawn(|| assert_bits(&transform.forward(&valid).unwrap(), &expected));
            }
        });
    }
}

#[test]
fn prbs_matches_independent_literal_history_for_multiple_seeds_and_period_edges() {
    for seed in [1, 0x1234, 0x7fff] {
        let mut history: Vec<u8> = (0..15).map(|bit| ((seed >> bit) & 1) as u8).collect();
        for index in 0..32782 {
            history.push(history[index + 14] ^ history[index]);
        }
        let mut state = seed;
        for expected in history {
            let (next, output) = prbs15::step(state);
            assert_eq!(output, expected);
            state = next;
        }
    }
    assert_eq!(prbs15::step(0), (0, 0));
    assert_eq!(prbs15::step(0x8001), prbs15::step(1));
}

#[test]
fn prbs_full_period_and_arbitrary_chunking_match_former_recurrence() {
    let mut state = 1;
    let mut former = 1;
    let mut windows = HashSet::new();
    let mut ones = 0;
    for index in 0..32767 {
        assert!(windows.insert(state));
        let (next, output) = prbs15::step(state);
        let expected = (former & 1) as u8;
        let feedback = (former ^ (former >> 14)) & 1;
        former = (former >> 1) | (feedback << 14);
        assert_eq!((next, output), (former, expected), "bit {index}");
        state = next;
        ones += usize::from(output);
    }
    assert_eq!(state, 1);
    assert_eq!(ones, 16384);
    let mut whole = 0x1234;
    let expected: Vec<_> = (0..65549)
        .map(|_| {
            let (next, output) = prbs15::step(whole);
            whole = next;
            output
        })
        .collect();
    let mut chunked = 0x1234;
    let mut actual = Vec::new();
    for count in [0, 1, 14, 32767, 3, 32764] {
        for _ in 0..count {
            let (next, output) = prbs15::step(chunked);
            chunked = next;
            actual.push(output);
        }
    }
    assert_eq!(actual, expected);
    assert_eq!(chunked, whole);
}
