/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use pnm::common::signal_processing::{
    decision_boundary::DecisionBoundary, qam::reference_for_order,
};

#[test]
fn square_qam_uses_half_spacing_grid_and_unbounded_outer_regions() {
    for (order, count) in [
        (2, 1),
        (4, 2),
        (16, 6),
        (64, 14),
        (256, 30),
        (1024, 62),
        (4096, 126),
        (16384, 254),
    ] {
        let reference = reference_for_order(order).unwrap();
        assert_eq!(reference.decision_boundaries.len(), count, "{order}-QAM");
        for boundary in reference.decision_boundaries {
            assert!(boundary.minimum.is_none() && boundary.maximum.is_none());
            assert!(boundary.direction[0] == 0.0 || boundary.direction[1] == 0.0);
        }
    }
    // For 16-QAM the raw levels are -3,-1,1,3 and Es=10.
    // Independent 50-digit Decimal reference: 2/sqrt(10), tolerance1e-14.
    let reference = reference_for_order(16).unwrap();
    let mut thresholds: Vec<_> = reference
        .decision_boundaries
        .iter()
        .filter(|line| line.direction[0] == 0.0)
        .map(|line| line.origin[0])
        .collect();
    thresholds.sort_by(f64::total_cmp);
    for (actual, expected) in thresholds
        .iter()
        .zip([-0.6324555320336759, 0.0, 0.6324555320336759])
    {
        assert!((actual - expected).abs() < 1e-14);
    }
    let bpsk = reference_for_order(2).unwrap();
    assert_eq!(
        bpsk.decision_boundaries[0].clipped([-2.0, -3.0], [2.0, 3.0]),
        Some([[0.0, -3.0], [0.0, 3.0]])
    );
}
#[test]
fn cross_qam_has_four_diagonal_rays_instead_of_false_corner_squares() {
    for (order, count) in [(128, 26), (512, 50), (2048, 98), (8192, 194)] {
        let reference = reference_for_order(order).unwrap();
        assert_eq!(reference.decision_boundaries.len(), count, "{order}-QAM");
        let diagonal: Vec<_> = reference
            .decision_boundaries
            .iter()
            .filter(|line| line.direction[0] != 0.0 && line.direction[1] != 0.0)
            .collect();
        assert_eq!(diagonal.len(), 4);
        for line in diagonal {
            assert!(line.minimum.is_none() ^ line.maximum.is_none());
            let endpoints = line.clipped([-100.0, -100.0], [100.0, 100.0]).unwrap();
            assert!(endpoints
                .iter()
                .any(|point| point.iter().any(|value| value.abs() == 100.0)));
            for point in endpoints {
                assert!((point[0].abs() - point[1].abs()).abs() < 1e-12);
            }
        }
    }
    let reference = reference_for_order(128).unwrap();
    let corner = reference
        .decision_boundaries
        .iter()
        .find(|line| line.direction == [1.0, 1.0] && line.minimum.is_some())
        .unwrap();
    assert!((corner.minimum.unwrap() - 0.8834522085987724).abs() < 1e-14); // 8/sqrt(82).
}
#[test]
fn every_compiled_edge_is_equidistant_from_nearest_symbols() {
    // Independent brute-force distance check against every reference symbol;
    // no Delaunay/Voronoi implementation is reused by this oracle.
    for order in [4, 16, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384] {
        let reference = reference_for_order(order).unwrap();
        for boundary in reference.decision_boundaries {
            let [start, end] = boundary.clipped([-2.0, -2.0], [2.0, 2.0]).unwrap();
            for fraction in [0.173, 0.371, 0.683] {
                let point = [
                    start[0] + fraction * (end[0] - start[0]),
                    start[1] + fraction * (end[1] - start[1]),
                ];
                let mut first = f64::INFINITY;
                let mut second = f64::INFINITY;
                for symbol in reference.iq {
                    let distance = (point[0] - symbol[0]).powi(2) + (point[1] - symbol[1]).powi(2);
                    if distance < first {
                        second = first;
                        first = distance;
                    } else {
                        second = second.min(distance);
                    }
                }
                assert!(
                    (second - first).abs() < 1e-10,
                    "{order}-QAM has a non-nearest boundary at {point:?}"
                );
            }
        }
    }
}
#[test]
fn clipping_rejects_invalid_bounds_and_keeps_real_ray_endpoints() {
    let ray = DecisionBoundary {
        origin: [0.0, 0.0],
        direction: [1.0, 1.0],
        minimum: Some(1.0),
        maximum: None,
    };
    assert_eq!(
        ray.clipped([-2.0, -2.0], [2.0, 2.0]),
        Some([[1.0, 1.0], [2.0, 2.0]])
    );
    assert!(ray.clipped([-1.0, -1.0], [0.5, 0.5]).is_none());
    assert!(ray.clipped([2.0, -1.0], [3.0, 1.0]).is_none());
    assert!(ray.clipped([f64::NAN, 0.0], [1.0, 1.0]).is_none());
    assert!(ray.clipped([0.0, 0.0], [0.0, 1.0]).is_none());
}
