/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Complete source-table label preservation, separate from external coding-chain evidence.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::signal_processing::modulation::{map_symbol, qam_llrs};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

#[test]
fn every_data_label_matches_pinned_pypnm_row_and_normative_scale() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/qam-table");
    let provenance: serde_json::Value =
        serde_json::from_slice(&std::fs::read(directory.join("provenance.json")).unwrap()).unwrap();
    // PHY3.1 I20 AnnexA.2 Table54, PDF206. These expected energies are
    // transcribed independently of the source scaling-factor file.
    for (order, energy) in [
        (16u16, 10.0_f64),
        (64, 42.0),
        (128, 82.0),
        (256, 170.0),
        (512, 330.0),
        (1024, 682.0),
        (2048, 1322.0),
        (4096, 2730.0),
        (8192, 5290.0),
        (16384, 10922.0),
    ] {
        let name = format!("{order}QAM_table.txt");
        let bytes = std::fs::read(directory.join(&name)).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&bytes)),
            provenance["files"][&name].as_str().unwrap()
        );
        let text = std::str::from_utf8(&bytes).unwrap();
        let rows: Vec<[f64; 2]> = text
            .lines()
            .filter(|line| {
                !line.trim().is_empty()
                    && !line.trim().starts_with('#')
                    && !line.trim().starts_with("//")
            })
            .map(|line| {
                let point: Vec<f64> = line
                    .split_whitespace()
                    .map(|value| value.parse().unwrap())
                    .collect();
                assert_eq!(point.len(), 2);
                [point[0], point[1]]
            })
            .collect();
        assert_eq!(rows.len(), usize::from(order));
        for (label, raw) in rows.iter().enumerate() {
            let actual = map_symbol(order, label as u16).unwrap();
            let expected = [raw[0] / energy.sqrt(), raw[1] / energy.sqrt()];
            for axis in 0..2 {
                assert!(
                    (actual[axis] - expected[axis]).abs() < 1e-14,
                    "order={order},label={label},axis={axis}"
                );
            }
        }
        // Check inverse soft-bit sign using raw expected I/Q, not map_symbol output.
        for label in [
            0,
            1,
            usize::from(order) / 3,
            usize::from(order) / 2,
            usize::from(order) - 1,
        ] {
            let expected = rows[label].map(|coordinate| coordinate / energy.sqrt());
            let llrs = qam_llrs(order, expected, 0.001).unwrap();
            for (bit, llr) in llrs.iter().enumerate() {
                assert_ne!(*llr, 0.0);
                assert_eq!(
                    usize::from(*llr < 0.0),
                    (label >> (llrs.len() - 1 - bit)) & 1
                );
            }
        }
    }
}
