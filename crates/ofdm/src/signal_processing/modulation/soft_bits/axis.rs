/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Exact bit-constrained axis lookups, admitted only for verified Cartesian labels.
use super::super::qam::QamReference;
use crate::types::IqPair;

struct Neighbors {
    below: Vec<Option<usize>>,
    above: Vec<Option<usize>>,
}
impl Neighbors {
    fn new(values: &[u8], hypothesis: u8) -> Self {
        let mut below = Vec::with_capacity(values.len() + 1);
        let mut nearest = None;
        for boundary in 0..=values.len() {
            below.push(nearest);
            if values.get(boundary) == Some(&hypothesis) {
                nearest = Some(boundary);
            }
        }
        let mut above = vec![None; values.len() + 1];
        nearest = None;
        for boundary in (0..=values.len()).rev() {
            if values.get(boundary) == Some(&hypothesis) {
                nearest = Some(boundary);
            }
            above[boundary] = nearest;
        }
        Self { below, above }
    }
}
struct BitAxis {
    axis: usize,
    hypotheses: [Neighbors; 2],
}
/// Verified Cartesian coordinates and bounded per-bit nearest-level lookups.
pub(super) struct AxisIndex {
    levels: [Vec<f64>; 2],
    bits: Vec<BitAxis>,
}
impl AxisIndex {
    /// Verifies a complete product grid and actual coordinate-separable labels.
    /// # Arguments
    /// * `reference` - Immutable embedded table, with row index equal to label.
    /// # Returns
    /// Bounded axis lookups, or None when a two-dimensional search is required.
    pub(super) fn new(reference: &QamReference) -> Option<Self> {
        let levels: [Vec<f64>; 2] = std::array::from_fn(|axis| {
            let mut values: Vec<_> = reference.iq.iter().map(|point| point[axis]).collect();
            values.sort_unstable_by(f64::total_cmp);
            values.dedup();
            values
        });
        // Build-time validation already proves that all reference coordinates are unique.
        if levels[0].len().checked_mul(levels[1].len())? != reference.iq.len() {
            return None;
        }
        let ranks: Vec<[usize; 2]> = reference
            .iq
            .iter()
            .map(|point| {
                std::array::from_fn(|axis| {
                    levels[axis]
                        .binary_search_by(|value| value.total_cmp(&point[axis]))
                        .unwrap()
                })
            })
            .collect();
        let bits = (0..usize::from(reference.bit_loading))
            .map(|bit| {
                for axis in 0..2 {
                    let mut values = vec![None; levels[axis].len()];
                    let mut separable = true;
                    for (label, rank) in ranks.iter().enumerate() {
                        let value =
                            ((label >> (usize::from(reference.bit_loading) - 1 - bit)) & 1) as u8;
                        let previous = &mut values[rank[axis]];
                        if previous.is_some_and(|previous| previous != value) {
                            separable = false;
                            break;
                        }
                        *previous = Some(value);
                    }
                    if separable {
                        let values: Vec<u8> = values.into_iter().collect::<Option<_>>()?;
                        return Some(BitAxis {
                            axis,
                            hypotheses: [Neighbors::new(&values, 0), Neighbors::new(&values, 1)],
                        });
                    }
                }
                None
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Self { levels, bits })
    }

    /// Computes exact two-dimensional minima with two shared axis searches.
    /// # Arguments
    /// * `received` - Finite normalized I/Q with validated finite distances.
    /// * `output` - One pair per MSB-first bit, exactly matching this reference.
    /// # Returns
    /// Unit after filling minima using the original I-squared plus Q-squared order.
    pub(super) fn distances(&self, received: IqPair, output: &mut [[f64; 2]]) {
        let boundaries: [usize; 2] = std::array::from_fn(|axis| {
            self.levels[axis].partition_point(|level| *level < received[axis])
        });
        let nearest: [f64; 2] = std::array::from_fn(|axis| {
            squared_minimum(
                &self.levels[axis],
                received[axis],
                [
                    boundaries[axis].checked_sub(1),
                    (boundaries[axis] < self.levels[axis].len()).then_some(boundaries[axis]),
                ],
            )
        });
        for (bit, hypotheses) in self.bits.iter().zip(output) {
            for (neighbors, distance) in bit.hypotheses.iter().zip(hypotheses) {
                let boundary = boundaries[bit.axis];
                let constrained = squared_minimum(
                    &self.levels[bit.axis],
                    received[bit.axis],
                    [neighbors.below[boundary], neighbors.above[boundary]],
                );
                *distance = if bit.axis == 0 {
                    constrained + nearest[1]
                } else {
                    nearest[0] + constrained
                };
            }
        }
    }
}
fn squared_minimum(levels: &[f64], received: f64, candidates: [Option<usize>; 2]) -> f64 {
    candidates
        .into_iter()
        .flatten()
        .map(|index| (received - levels[index]).powi(2))
        .fold(f64::INFINITY, f64::min)
}
