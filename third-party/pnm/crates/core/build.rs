/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Validate preserved PyPNM resources and embed normalized QAM tables at build time.
use std::{collections::BTreeSet, env, fmt::Write, fs, path::PathBuf};

#[path = "build/qam_boundaries.rs"]
mod qam_boundaries;

const SUPPORTED_ORDERS: [u16; 12] = [2, 4, 16, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384];

/// Generates static reference points; runtime decoding needs no text files.
/// # Panics
/// Fails the build for missing, malformed, duplicate or incorrectly scaled tables.
fn main() {
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
        for &(in_phase, quadrature) in &points {
            writeln!(
                output,
                "[{:?}, {:?}],",
                f64::from(in_phase) * scale_factor,
                f64::from(quadrature) * scale_factor
            )
            .unwrap();
        }
        output.push_str("],\n");
        qam_boundaries::write_boundaries(&points, scale_factor, &mut output);
        output.push_str("},\n");
    }
    output.push_str("];\n");
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("qam_references.rs"),
        output,
    )
    .unwrap();
}
