/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Independent small address literal plus complete bounded bijection coverage.
use docsis_ofdma::resources::{addresses, block_sizes};

#[test]
fn exact_partition_and_first_diagonal_source_cases() {
    for (count, expected) in [
        (0, vec![]),
        (24, vec![24]),
        (25, vec![12, 13]),
        (48, vec![24, 24]),
        (49, vec![16, 16, 17]),
    ] {
        assert_eq!(block_sizes(count).unwrap(), expected);
    }
    let visits = addresses(6, 8).unwrap();
    assert_eq!(
        &visits[..8],
        &[
            (0, 0),
            (4, 4),
            (2, 2),
            (1, 6),
            (5, 1),
            (3, 5),
            (0, 3),
            (4, 7)
        ]
    );
    assert_eq!(visits[8], (4, 0)); // Accepted unreversed diagonal counter 1.
}

#[test]
fn every_supported_geometry_has_exact_independent_cartesian_coverage() {
    for k in 6..=36 {
        for l in 1..=384 {
            let visits = addresses(k, l).unwrap();
            assert_eq!(visits.len(), k * l);
            let mut seen = vec![false; k * l];
            for (t, f) in visits {
                assert!(t < k && f < l);
                assert!(!seen[t * l + f], "duplicate at K={k},L={l}");
                seen[t * l + f] = true;
            }
            assert!(seen.into_iter().all(|present| present));
        }
    }
}
