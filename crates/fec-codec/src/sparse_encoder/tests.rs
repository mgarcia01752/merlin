/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Isolated synthetic encoder setup/encode checks, independent of simulation.
use super::SparseEncoder;
use ldpc_toolbox::{encoder::Encoder, gf2::GF2, sparse::SparseMatrix};
use ndarray::Array1;
use num_traits::{One, Zero};
use std::{hint::black_box, time::Instant};

const SYNTHETIC_BLOCK: usize = 16;
const BENCHMARK_SIZES: [usize; 5] = [80, 192, 384, 768, 1536];
const SETUP_REPEATS: usize = 5;
const ENCODE_REPEATS: usize = 100;

fn synthetic_matrix(parity_bits: usize) -> SparseMatrix {
    let information_bits = parity_bits * 3 / 2;
    let mut matrix = SparseMatrix::new(parity_bits, information_bits + parity_bits);
    for row in 0..parity_bits {
        for column in [
            row % information_bits,
            (row * 3 + 1) % information_bits,
            (row * 5 + 7) % information_bits,
        ] {
            matrix.insert(row, column);
        }
        let group = row / SYNTHETIC_BLOCK;
        let offset = row % SYNTHETIC_BLOCK;
        let diagonal = group * SYNTHETIC_BLOCK + (offset + group * 7 + 3) % SYNTHETIC_BLOCK;
        matrix.insert(row, information_bits + diagonal);
        if group > 0 {
            matrix.insert(
                row,
                information_bits
                    + (group - 1) * SYNTHETIC_BLOCK
                    + (offset + group * 5) % SYNTHETIC_BLOCK,
            );
        }
        if group > 1 {
            matrix.insert(
                row,
                information_bits
                    + (group - 2) * SYNTHETIC_BLOCK
                    + (offset + group * 3 + 1) % SYNTHETIC_BLOCK,
            );
        }
    }
    matrix
}
fn message(information_bits: usize) -> Array1<GF2> {
    Array1::from_iter((0..information_bits).map(|index| {
        if (index * 13 + index / 7) % 2 == 0 {
            GF2::zero()
        } else {
            GF2::one()
        }
    }))
}
fn assert_syndrome(matrix: &SparseMatrix, word: &Array1<GF2>) {
    for row in 0..matrix.num_rows() {
        assert!(matrix
            .iter_row(row)
            .map(|column| word[*column])
            .sum::<GF2>()
            .is_zero());
    }
}

#[test]
fn upstream_synthetic_baseline_has_systematic_words_and_zero_syndromes() {
    for parity_bits in [80, 192] {
        let matrix = synthetic_matrix(parity_bits);
        let data = message(matrix.num_cols() - matrix.num_rows());
        let encoder = Encoder::from_h(&matrix).unwrap();
        let word = encoder.encode(&data);
        assert_eq!(
            &word.as_slice().unwrap()[..data.len()],
            data.as_slice().unwrap()
        );
        assert_syndrome(&matrix, &word);
    }
}

#[test]
fn sparse_substitution_matches_all_hand_solved_words_with_permuted_rows() {
    // H information=[[1,1],[0,1]], parity=[[1,0],[1,1]].
    // Thus p0=a0 XOR a1 and p1=p0 XOR a1=a0, even when row order is reversed.
    let mut matrix = SparseMatrix::new(2, 4);
    for (row, columns) in [(1, vec![0, 1, 2]), (0, vec![1, 2, 3])] {
        for column in columns {
            matrix.insert(row, column);
        }
    }
    let original = matrix.clone();
    let encoder = SparseEncoder::from_h(&matrix).unwrap();
    for (information, expected) in [
        ([0, 0], [0, 0, 0, 0]),
        ([0, 1], [0, 1, 1, 0]),
        ([1, 0], [1, 0, 1, 1]),
        ([1, 1], [1, 1, 0, 1]),
    ] {
        let data =
            Array1::from_iter(information.map(
                |value| {
                    if value == 0 {
                        GF2::zero()
                    } else {
                        GF2::one()
                    }
                },
            ));
        let word = encoder.encode(&data);
        let actual: Vec<_> = word
            .iter()
            .map(|value| u8::from(!value.is_zero()))
            .collect();
        assert_eq!(actual, expected);
        assert_syndrome(&matrix, &word);
    }
    assert_eq!(matrix, original);
}

#[test]
fn sparse_substitution_rejects_bad_shapes_and_unsolved_dependencies() {
    for (rows, columns) in [(0, 3), (3, 2), (2, 2)] {
        assert!(SparseEncoder::from_h(&SparseMatrix::new(rows, columns)).is_err());
    }
    let mut missing = SparseMatrix::new(2, 4);
    missing.insert(0, 2);
    assert!(SparseEncoder::from_h(&missing).is_err());
    // Invertible parity [[1,1,0],[0,1,1],[1,1,1]] has no degree-one check.
    // It needs general elimination and is explicitly outside this helper's scope.
    let mut cyclic = SparseMatrix::new(3, 4);
    for (row, columns) in [(0, vec![1, 2]), (1, vec![2, 3]), (2, vec![1, 2, 3])] {
        for column in columns {
            cyclic.insert(row, column);
        }
    }
    assert!(SparseEncoder::from_h(&cyclic).is_err());
}

#[test]
fn sparse_substitution_matches_upstream_on_synthetic_shifted_parity_blocks() {
    for parity_bits in BENCHMARK_SIZES {
        let matrix = synthetic_matrix(parity_bits);
        let original = matrix.clone();
        let data = message(matrix.num_cols() - matrix.num_rows());
        let original_data = data.clone();
        let sparse = SparseEncoder::from_h(&matrix).unwrap();
        let expected = Encoder::from_h(&matrix).unwrap().encode(&data);
        assert_eq!(sparse.encode(&data), expected);
        assert_syndrome(&matrix, &expected);
        assert_eq!(matrix, original);
        assert_eq!(data, original_data);
        assert_eq!(sparse.encode(&data), expected); // Reuse cannot accumulate parity state.
    }
}

#[test]
#[ignore = "Explicit bounded encoder microbenchmark; no simulation or capture I/O"]
fn benchmark_encoder_methods_only() {
    println!("parity_bits upstream_setup_us sparse_setup_us upstream_encode_us sparse_encode_us");
    for parity_bits in BENCHMARK_SIZES {
        let matrix = synthetic_matrix(parity_bits);
        let data = message(matrix.num_cols() - matrix.num_rows());
        let start = Instant::now();
        for _ in 0..SETUP_REPEATS {
            black_box(Encoder::from_h(black_box(&matrix)).unwrap());
        }
        let setup = start.elapsed().as_secs_f64() * 1e6 / SETUP_REPEATS as f64;
        let start = Instant::now();
        for _ in 0..SETUP_REPEATS {
            black_box(SparseEncoder::from_h(black_box(&matrix)).unwrap());
        }
        let sparse_setup = start.elapsed().as_secs_f64() * 1e6 / SETUP_REPEATS as f64;
        let sparse = SparseEncoder::from_h(&matrix).unwrap();
        let encoder = Encoder::from_h(&matrix).unwrap();
        let start = Instant::now();
        for _ in 0..ENCODE_REPEATS {
            black_box(encoder.encode(black_box(&data)));
        }
        let encode = start.elapsed().as_secs_f64() * 1e6 / ENCODE_REPEATS as f64;
        let start = Instant::now();
        for _ in 0..ENCODE_REPEATS {
            black_box(sparse.encode(black_box(&data)));
        }
        let sparse_encode = start.elapsed().as_secs_f64() * 1e6 / ENCODE_REPEATS as f64;
        println!("{parity_bits} {setup:.3} {sparse_setup:.3} {encode:.3} {sparse_encode:.3}");
        assert_eq!(sparse.encode(&data), encoder.encode(&data));
        assert_syndrome(&matrix, &encoder.encode(&data));
    }
}
