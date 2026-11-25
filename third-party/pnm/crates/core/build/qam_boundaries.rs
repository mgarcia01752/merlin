/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Compile nearest-symbol Voronoi boundaries from integer QAM resources.
use delaunator::{triangulate, Point};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
};

const GEOMETRY_TOLERANCE: f64 = 1e-10;
type LineKey = (i64, i64, i64);
type Interval = (f64, f64);

fn gcd(mut left: i64, mut right: i64) -> i64 {
    while right != 0 {
        (left, right) = (right, left % right);
    }
    left.abs()
}
// Equidistance: (second-first) dot position = (|second|²-|first|²)/2.
// Integer coordinates give exact canonical keys for merging collinear edges.
fn line_key(first: (i16, i16), second: (i16, i16)) -> LineKey {
    let mut normal_i = i64::from(second.0) - i64::from(first.0);
    let mut normal_q = i64::from(second.1) - i64::from(first.1);
    let mut constant = (i64::from(second.0).pow(2) + i64::from(second.1).pow(2)
        - i64::from(first.0).pow(2)
        - i64::from(first.1).pow(2))
        / 2;
    assert_eq!(
        (i64::from(second.0).pow(2) + i64::from(second.1).pow(2)
            - i64::from(first.0).pow(2)
            - i64::from(first.1).pow(2))
            % 2,
        0,
        "Reference coordinates require half-integer bisector support"
    );
    let divisor = gcd(gcd(normal_i, normal_q), constant);
    normal_i /= divisor;
    normal_q /= divisor;
    constant /= divisor;
    if normal_i < 0 || (normal_i == 0 && normal_q < 0) {
        normal_i = -normal_i;
        normal_q = -normal_q;
        constant = -constant;
    }
    (normal_i, normal_q, constant)
}
fn coordinates(key: LineKey) -> ([f64; 2], [f64; 2]) {
    let (normal_i, normal_q, constant) = key;
    let norm = (normal_i * normal_i + normal_q * normal_q) as f64;
    (
        [
            normal_i as f64 * constant as f64 / norm,
            normal_q as f64 * constant as f64 / norm,
        ],
        [-(normal_q as f64), normal_i as f64],
    )
}
fn interval(
    points: &[(i16, i16)],
    site: usize,
    neighbors: &BTreeSet<usize>,
    key: LineKey,
) -> Option<Interval> {
    let (origin, direction) = coordinates(key);
    let first = points[site];
    let mut lower = f64::NEG_INFINITY;
    let mut upper = f64::INFINITY;
    // Delaunay neighbors supply the Voronoi half-planes for this site.
    for &neighbor in neighbors {
        let other = points[neighbor];
        let delta_i = f64::from(other.0) - f64::from(first.0);
        let delta_q = f64::from(other.1) - f64::from(first.1);
        let coefficient = delta_i * direction[0] + delta_q * direction[1];
        let limit = (f64::from(other.0).powi(2) + f64::from(other.1).powi(2)
            - f64::from(first.0).powi(2)
            - f64::from(first.1).powi(2))
            / 2.0
            - delta_i * origin[0]
            - delta_q * origin[1];
        if coefficient > 0.0 {
            upper = upper.min(limit / coefficient);
        } else if coefficient < 0.0 {
            lower = lower.max(limit / coefficient);
        } else if limit < -GEOMETRY_TOLERANCE {
            return None;
        }
    }
    if upper - lower > GEOMETRY_TOLERANCE {
        Some((lower, upper))
    } else {
        None
    }
}
fn bound_literal(value: f64, scale: f64) -> String {
    if value.is_finite() {
        format!("Some({:?})", value * scale)
    } else {
        "None".into()
    }
}

/// Emits merged exact bisectors, including unbounded outer rays and lines.
/// # Arguments
/// * `points` - Validated integer resource coordinates with even coordinate differences.
/// * `scale` - Unit-power normalization multiplier applied to coordinates and parameters.
/// * `output` - Generated Rust source destination owned by the Cargo build script.
/// # Panics
/// Fails the build if a non-BPSK table cannot be triangulated or output formatting fails.
pub fn write_boundaries(points: &[(i16, i16)], scale: f64, output: &mut String) {
    let mut neighbors = vec![BTreeSet::new(); points.len()];
    if points.len() == 2 {
        neighbors[0].insert(1);
        neighbors[1].insert(0);
    } else {
        let sites: Vec<_> = points
            .iter()
            .map(|&(in_phase, quadrature)| Point {
                x: f64::from(in_phase),
                y: f64::from(quadrature),
            })
            .collect();
        let triangulation = triangulate(&sites);
        assert!(!triangulation.triangles.is_empty());
        for triangle in triangulation.triangles.chunks_exact(3) {
            for (first, second) in [
                (triangle[0], triangle[1]),
                (triangle[1], triangle[2]),
                (triangle[2], triangle[0]),
            ] {
                neighbors[first].insert(second);
                neighbors[second].insert(first);
            }
        }
    }
    let mut lines: BTreeMap<LineKey, Vec<Interval>> = BTreeMap::new();
    for (site, adjacent) in neighbors.iter().enumerate() {
        for &neighbor in adjacent.range(site + 1..) {
            let key = line_key(points[site], points[neighbor]);
            if let Some(segment) = interval(points, site, adjacent, key) {
                lines.entry(key).or_default().push(segment);
            }
        }
    }
    output.push_str("decision_boundaries: &[\n");
    for (key, mut intervals) in lines {
        intervals.sort_by(|left, right| left.0.total_cmp(&right.0));
        let mut merged: Vec<Interval> = Vec::new();
        for (lower, upper) in intervals {
            if let Some(previous) = merged.last_mut() {
                if lower <= previous.1 + GEOMETRY_TOLERANCE {
                    previous.1 = previous.1.max(upper);
                    continue;
                }
            }
            merged.push((lower, upper));
        }
        let (origin, direction) = coordinates(key);
        for (lower, upper) in merged {
            writeln!(output,"DecisionBoundary {{ origin: [{:?}, {:?}], direction: {direction:?}, minimum: {}, maximum: {} }},",origin[0]*scale,origin[1]*scale,bound_literal(lower,scale),bound_literal(upper,scale)).unwrap();
        }
    }
    output.push_str("],\n");
}
