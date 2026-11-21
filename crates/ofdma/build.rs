/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 * Adapted from MERLIN crates/ofdm/build/qam_tables.rs at
 * 096d5a476194d4f2e016863d723b225257bb6037.
 * Modifications: upstream orders only, integer-coordinate output and local paths.
 */
//! Compile selected preserved resources; malformed source data fails the build.
use std::{collections::BTreeSet, env, fmt::Write, fs, path::PathBuf};

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("../ofdm/resources/qam-table");
    println!("cargo:rerun-if-changed=../ofdm/resources/qam-table");
    let mut output = String::from("const TABLES: &[(u8, &[[i16; 2]])] = &[\n");
    for (bits, energy) in [
        (1, 1),
        (2, 2),
        (4, 10),
        (6, 42),
        (7, 82),
        (8, 170),
        (9, 330),
        (10, 682),
        (11, 1322),
        (12, 2730),
    ] {
        let order = 1_usize << bits;
        let raw = fs::read_to_string(root.join(format!("{order}QAM_table.txt"))).unwrap();
        let points: Vec<[i16; 2]> = raw
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let values: Vec<_> = line
                    .split_whitespace()
                    .map(|value| value.parse::<i16>().unwrap())
                    .collect();
                assert_eq!(values.len(), 2);
                [values[0], values[1]]
            })
            .collect();
        assert_eq!(points.len(), order);
        assert_eq!(points.iter().collect::<BTreeSet<_>>().len(), order);
        assert_eq!(
            points
                .iter()
                .map(|[i, q]| i64::from(*i).pow(2) + i64::from(*q).pow(2))
                .sum::<i64>(),
            energy * order as i64
        );
        writeln!(output, "({bits}, &[").unwrap();
        let permutation: Vec<_> = match bits {
            1 => vec![1, 0],
            2 => vec![1, 3, 0, 2],
            _ => (0..order).collect(),
        };
        for index in permutation {
            writeln!(output, "{:?},", points[index]).unwrap();
        }
        output.push_str("]),\n");
    }
    output.push_str("];\n");
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("qam_tables.rs"),
        output,
    )
    .unwrap();
}
