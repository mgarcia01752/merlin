/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Exact balanced spatial searches for irregular or coordinate-coupled references.
use super::super::qam::QamReference;
use crate::types::IqPair;

/// Immutable balanced hypothesis trees retaining the original cross-QAM geometry.
pub(super) struct SpatialIndex {
    bits: Vec<[Vec<IqPair>; 2]>,
}
impl SpatialIndex {
    /// Builds bounded, balanced per-bit hypothesis trees from the original labels.
    /// # Arguments
    /// * `reference` - Validated immutable embedded constellation.
    /// # Returns
    /// O(M log M) point storage across the log2(M) binary label partitions.
    pub(super) fn new(reference: &QamReference) -> Self {
        let bits = (0..usize::from(reference.bit_loading))
            .map(|bit| {
                std::array::from_fn(|hypothesis| {
                    let mut points: Vec<_> = reference
                        .iq
                        .iter()
                        .enumerate()
                        .filter(|(label, _)| {
                            (label >> (usize::from(reference.bit_loading) - 1 - bit)) & 1
                                == hypothesis
                        })
                        .map(|(_, point)| *point)
                        .collect();
                    balance(&mut points, 0);
                    points
                })
            })
            .collect();
        Self { bits }
    }

    /// Searches both bit hypotheses exactly, pruning only impossible improvements.
    /// # Arguments
    /// * `received` - Finite normalized I/Q with validated finite distances.
    /// * `output` - One pair per MSB-first reference bit.
    /// # Returns
    /// Unit after filling minima; worst-case queries may still visit every point.
    pub(super) fn distances(&self, received: IqPair, output: &mut [[f64; 2]]) {
        for (trees, hypotheses) in self.bits.iter().zip(output) {
            for (tree, distance) in trees.iter().zip(hypotheses) {
                *distance = f64::INFINITY;
                nearest(tree, received, 0, distance);
            }
        }
    }
}
fn balance(points: &mut [IqPair], axis: usize) {
    if points.is_empty() {
        return;
    }
    let middle = points.len() / 2;
    points.select_nth_unstable_by(middle, |left, right| left[axis].total_cmp(&right[axis]));
    let (left, rest) = points.split_at_mut(middle);
    balance(left, 1 - axis);
    balance(&mut rest[1..], 1 - axis);
}
fn nearest(points: &[IqPair], received: IqPair, axis: usize, best: &mut f64) {
    if points.is_empty() {
        return;
    }
    let middle = points.len() / 2;
    let point = points[middle];
    let distance = (received[0] - point[0]).powi(2) + (received[1] - point[1]).powi(2);
    *best = (*best).min(distance);
    let delta = received[axis] - point[axis];
    let (near, far) = if delta < 0.0 {
        (&points[..middle], &points[middle + 1..])
    } else {
        (&points[middle + 1..], &points[..middle])
    };
    nearest(near, received, 1 - axis, best);
    if delta.powi(2) <= *best {
        nearest(far, received, 1 - axis, best);
    }
}
