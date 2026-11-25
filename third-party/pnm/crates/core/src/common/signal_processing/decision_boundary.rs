/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Normalized nearest-symbol Voronoi lines/rays compiled with each QAM table.
use crate::pnm::data_type::IqPair;
use serde::Serialize;

/// A line parameterized as origin + direction*t; null bounds are unbounded.
#[derive(Debug, PartialEq, Serialize)]
pub struct DecisionBoundary {
    pub origin: IqPair,
    pub direction: IqPair,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
}
impl DecisionBoundary {
    /// Clips a physical symbol boundary to a finite I/Q display window.
    /// # Arguments
    /// * `lower`, `upper` - Minimum/maximum I/Q coordinates of the viewport.
    /// # Returns
    /// Two finite endpoints, or None for invalid bounds, a missing intersection
    /// or a point-only contact. Clipping adds no artificial symbol boundaries.
    pub fn clipped(&self, lower: IqPair, upper: IqPair) -> Option<[IqPair; 2]> {
        if !lower
            .into_iter()
            .chain(upper)
            .chain(self.origin)
            .chain(self.direction)
            .all(f64::is_finite)
            || lower[0] >= upper[0]
            || lower[1] >= upper[1]
            || self.direction == [0.0, 0.0]
            || self
                .minimum
                .into_iter()
                .chain(self.maximum)
                .any(|value| !value.is_finite())
        {
            return None;
        }
        let mut minimum = self.minimum.unwrap_or(f64::NEG_INFINITY);
        let mut maximum = self.maximum.unwrap_or(f64::INFINITY);
        for axis in 0..2 {
            if self.direction[axis] == 0.0 {
                if self.origin[axis] < lower[axis] || self.origin[axis] > upper[axis] {
                    return None;
                }
                continue;
            }
            let first = (lower[axis] - self.origin[axis]) / self.direction[axis];
            let last = (upper[axis] - self.origin[axis]) / self.direction[axis];
            minimum = minimum.max(first.min(last));
            maximum = maximum.min(first.max(last));
        }
        if !minimum.is_finite() || !maximum.is_finite() || minimum >= maximum {
            return None;
        }
        let point = |parameter: f64| {
            [
                (self.origin[0] + self.direction[0] * parameter).clamp(lower[0], upper[0]),
                (self.origin[1] + self.direction[1] * parameter).clamp(lower[1], upper[1]),
            ]
        };
        Some([point(minimum), point(maximum)])
    }
}
