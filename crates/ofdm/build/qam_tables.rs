/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Validate preserved PyPNM resources and embed normalized QAM tables at build time.
use std::{collections::BTreeSet, env, fmt::Write, fs, path::PathBuf};

const SUPPORTED_ORDERS: [u16; 12] = [2, 4, 16, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384];

/// Generates static reference points; runtime decoding needs no text files.
/// # Returns
/// Unit after writing validated normalized lookup tables into Cargo OUT_DIR.
/// # Panics
/// Fails the build for missing, malformed, duplicate or incorrectly scaled tables.
pub(super) fn generate() {
    let root =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("resources/qam-table");
    println!("cargo:rerun-if-changed=resources/qam-table");
    let scale_text = fs::read_to_string(root.join("ConstellationScalingFactors.txt")).unwrap();
    let scales: Vec<(u32, f64)> = scale_text
        .lines()
        .map(|line| {
            let fields: Vec<_> = line.split_whitespace().collect();
            assert_eq!(fields.len(), 2);
            (fields[0].parse().unwrap(), fields[1].parse().unwrap())
        })
        .collect();
    let mut output = String::from("static REFERENCES: &[QamReference] = &[\n");
    for order in SUPPORTED_ORDERS {
        let bits = order.ilog2();
        let energy = scales.iter().find(|entry| entry.0 == bits).unwrap().1;
        assert!(energy.is_finite() && energy > 0.0);
        let text = fs::read_to_string(root.join(format!("{order}QAM_table.txt"))).unwrap();
        let points: Vec<(i16, i16)> = text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let fields: Vec<_> = line.split_whitespace().collect();
                assert_eq!(fields.len(), 2);
                (fields[0].parse().unwrap(), fields[1].parse().unwrap())
            })
            .collect();
        assert_eq!(points.len(), usize::from(order));
        assert_eq!(points.iter().collect::<BTreeSet<_>>().len(), points.len());
        let average_energy = points
            .iter()
            .map(|(in_phase, quadrature)| {
                f64::from(*in_phase).powi(2) + f64::from(*quadrature).powi(2)
            })
            .sum::<f64>()
            / f64::from(order);
        assert!(
            (average_energy - energy).abs() < 1e-10,
            "{order}-QAM energy disagrees with scaling resource"
        );
        let scale_factor = 1.0 / energy.sqrt();
        writeln!(output, "QamReference {{ order: {order}, bit_loading: {bits}, scale_factor: {scale_factor:?}, iq: &[").unwrap();
        // Source BPSK/QPSK rows describe geometry, not PHY bit-label order.
        // PHY I20 AnnexA Figures99–100, PDF197; preserve raw files unchanged.
        let label_to_row: Vec<usize> = match order {
            2 => vec![1, 0],
            4 => vec![1, 3, 0, 2],
            _ => (0..points.len()).collect(),
        };
        for source_row in label_to_row {
            let (in_phase, quadrature) = points[source_row];
            writeln!(
                output,
                "[{:?}, {:?}],",
                f64::from(in_phase) * scale_factor,
                f64::from(quadrature) * scale_factor
            )
            .unwrap();
        }
        output.push_str("],\n");
        output.push_str("},\n");
    }
    output.push_str("];\n");
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("qam_references.rs"),
        output,
    )
    .unwrap();
}
