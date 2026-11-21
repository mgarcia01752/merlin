/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 *
 * Adapted for MERLIN from MORGANA revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: module ownership/imports; codec reuses MERLIN sparse planning.
 */
//! Source-derived matrix/word acceptance, separate from shared-data round trips.
#[path = "source_fixtures.rs"]
mod source_fixtures;

use super::{fixture, llrs};
use crate::upstream_ldpc::DocsisLdpcCode;

struct SourceFixture {
    code: DocsisLdpcCode,
    codeword_bits: usize,
    information_bits: usize,
    lift: usize,
    row_fingerprints: [u64; 5],
    expanded_fingerprint: u64,
    edge_count: usize,
    impulses: &'static [ImpulseFixture],
}

struct ImpulseFixture {
    information_one: usize,

    // Coordinates relative to the first parity bit, in increasing order.
    parity_ones: &'static [usize],
}

fn source(code: DocsisLdpcCode) -> &'static SourceFixture {
    source_fixtures::FAMILIES
        .iter()
        .find(|source| source.code == code)
        .unwrap()
}

// FNV-1a with an explicitly specified byte encoding, not Rust's platform hasher.
fn fingerprint(bytes: impl IntoIterator<Item = u8>) -> u64 {
    bytes.into_iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3)
    })
}

fn every_source_cell_and_lifted_edge(code: DocsisLdpcCode) {
    let source = source(code);
    let parameters = code.parameters();
    assert_eq!(parameters.codeword_bits, source.codeword_bits);
    assert_eq!(parameters.information_bits, source.information_bits);
    assert_eq!(parameters.lifting_factor, source.lift);
    let shifts = code.shifts();
    assert_eq!(shifts.len(), 5);
    for (index, row) in shifts.iter().enumerate() {
        assert_eq!(row.len(), source.codeword_bits / source.lift);
        assert_eq!(
            fingerprint(row.iter().flat_map(|shift| shift.to_le_bytes())),
            source.row_fingerprints[index],
            "{code:?} source mismatch in zero-based base row {index}"
        );
    }
    let matrix = code.matrix().unwrap();
    assert_eq!(
        matrix.num_rows(),
        source.codeword_bits - source.information_bits
    );
    assert_eq!(matrix.num_cols(), source.codeword_bits);

    // Read every lifted edge. Sort explicitly; storage iteration order is not
    // part of SparseMatrix's public contract and must not affect this check.
    let mut edges = Vec::with_capacity(source.edge_count);
    for row in 0..matrix.num_rows() {
        edges.extend(
            matrix
                .iter_row(row)
                .map(|column| (row as u32, *column as u32)),
        );
    }
    edges.sort_unstable();
    assert_eq!(edges.len(), source.edge_count);
    assert_eq!(
        fingerprint(edges.iter().flat_map(|(row, column)| {
            row.to_le_bytes().into_iter().chain(column.to_le_bytes())
        })),
        source.expanded_fingerprint,
        "{code:?} expanded matrix differs from the source-derived reference"
    );
}

fn frozen_impulse_parity_and_recovery(code: DocsisLdpcCode) {
    let source = source(code);
    let mother_codec = fixture(code, source.information_bits, 50);
    for impulse in source.impulses {
        let mut mother = vec![0; source.codeword_bits];
        mother[impulse.information_one] = 1;
        for &parity_one in impulse.parity_ones {
            assert!(parity_one < source.codeword_bits - source.information_bits);
            mother[source.information_bits + parity_one] = 1;
        }
        assert!(
            mother_codec
                .encode(&mother[..source.information_bits])
                .unwrap()
                == mother,
            "{code:?} frozen parity for information bit {}",
            impulse.information_one
        );

        // Decode the frozen expected word, not the production encoder's output.
        // I=1 and full K are included, plus block and affected-column boundaries.
        for retained in [impulse.information_one + 1, source.information_bits] {
            let mut codec = fixture(code, retained, 50);
            let transmitted: Vec<_> = mother[..retained]
                .iter()
                .chain(&mother[source.information_bits..])
                .copied()
                .collect();
            assert_eq!(codec.encode(&mother[..retained]).unwrap(), transmitted);
            assert!(codec.valid_syndrome(&transmitted));
            let soft = llrs(&transmitted, 20.0);
            let decoded = codec.decode(&soft).unwrap();
            assert!(decoded.converged && decoded.parity_valid && decoded.shortened_zeros_valid);
            assert_eq!(decoded.iterations, 0);
            assert_eq!(decoded.mother_codeword, mother);
            assert_eq!(decoded.information_bits, mother[..retained]);
            assert_eq!(decoded.hard_decision_changes, 0);
        }
    }
}

fn known_zero_inconsistency_is_separate(code: DocsisLdpcCode) {
    let source = source(code);

    // The last source-derived impulse has a one in the very coordinate that
    // shortening promises is zero. Consistent received parity can defeat its
    // finite +50 prior. Keep that actual outcome, even with zero syndrome.
    let impulse = source.impulses.last().unwrap();
    assert_eq!(impulse.information_one, source.information_bits - 1);
    let retained = source.information_bits - 1;
    let mut transmitted = vec![0; retained + source.codeword_bits - source.information_bits];
    for &parity_one in impulse.parity_ones {
        transmitted[retained + parity_one] = 1;
    }
    let mut codec = fixture(code, retained, 50);
    let decoded = codec.decode(&llrs(&transmitted, 50.0)).unwrap();
    assert!(decoded.converged);
    assert!(decoded.parity_valid);
    assert!(!decoded.shortened_zeros_valid);
    assert_eq!(decoded.information_bits, vec![0; retained]);
    assert_eq!(decoded.mother_codeword[retained], 1);
    assert!((1..=50).contains(&decoded.iterations));

    // Checking only transmitted decisions restores a zero and must reject.
    assert!(!codec.valid_syndrome(&transmitted));
}

fn erasures_and_saturation_preserve_decisions(code: DocsisLdpcCode) {
    let source = source(code);
    let mut codec = fixture(code, source.information_bits, 100);
    let mut word = vec![0; source.codeword_bits];
    word[0] = 1;
    for &parity_one in source.impulses[0].parity_ones {
        word[source.information_bits + parity_one] = 1;
    }
    let saturated = codec.decode(&llrs(&word, 50.0)).unwrap();
    assert_eq!(codec.decode(&llrs(&word, f64::MAX)).unwrap(), saturated);
    let mut erased = llrs(&word, 8.0);
    erased[0] = 0.0;
    let recovered = codec.decode(&erased).unwrap();
    assert!(recovered.converged && recovered.parity_valid && recovered.shortened_zeros_valid);
    assert_eq!(recovered.mother_codeword, word);
    assert!(recovered.iterations <= 100);
    assert_eq!(recovered.hard_decision_changes, 0); // Erasure has no sign decision.
    erased[0] = -0.0;
    assert_eq!(codec.decode(&erased).unwrap(), recovered);
}

macro_rules! source_cases {
    ($module:ident, $code:ident) => {
        mod $module {
            use super::*;
            const CODE: DocsisLdpcCode = DocsisLdpcCode::$code;

            #[test]
            fn complete_source_matrix_and_expansion() {
                every_source_cell_and_lifted_edge(CODE);
            }

            #[test]
            fn exact_source_derived_impulses() {
                frozen_impulse_parity_and_recovery(CODE);
            }

            #[test]
            fn converged_invalid_shortening_is_preserved() {
                known_zero_inconsistency_is_separate(CODE);
            }

            #[test]
            fn finite_saturation_and_signed_erasures() {
                erasures_and_saturation_preserve_decisions(CODE);
            }
        }
    };
}

source_cases!(long, UpstreamLong);
source_cases!(medium, UpstreamMedium);
source_cases!(short, UpstreamShort);
