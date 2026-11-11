/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Geometry admission and conservative finite-distance bounds for cached searches.
use super::{axis::AxisIndex, search::SpatialIndex};
use crate::{signal_processing::modulation::qam::QamReference, types::IqPair};

enum Geometry {
    Axes(AxisIndex),
    Spatial(SpatialIndex),
}
/// Bounded immutable search geometry derived from one embedded reference.
pub(super) struct DemapIndex {
    bounds: [[f64; 2]; 2],
    geometry: Geometry,
}
impl DemapIndex {
    /// Initializes a bounded immutable search without changing reference geometry.
    /// # Arguments
    /// * `reference` - Supported static table, keyed by its unique bit loading.
    /// # Returns
    /// Verified separable-axis lookups or exact spatial hypothesis searches.
    pub(super) fn new(reference: &QamReference) -> Self {
        let bounds = std::array::from_fn(|axis| {
            reference
                .iq
                .iter()
                .fold([f64::INFINITY, f64::NEG_INFINITY], |range, point| {
                    [range[0].min(point[axis]), range[1].max(point[axis])]
                })
        });
        let geometry = match AxisIndex::new(reference) {
            Some(axes) => Geometry::Axes(axes),
            None => Geometry::Spatial(SpatialIndex::new(reference)),
        };
        Self { bounds, geometry }
    }

    /// Checks that every point distance is finite using an enclosing rectangle.
    /// # Arguments
    /// * `received` - Already finite normalized I/Q.
    /// # Returns
    /// True only if the upper bound is finite; false requests the legacy full scan
    /// so a nonexistent corner of cross-QAM cannot introduce a new input error.
    pub(super) fn finite_distances(&self, received: IqPair) -> bool {
        let farthest: [f64; 2] = std::array::from_fn(|axis| {
            (received[axis] - self.bounds[axis][0])
                .abs()
                .max((received[axis] - self.bounds[axis][1]).abs())
        });
        (farthest[0].powi(2) + farthest[1].powi(2)).is_finite()
    }

    /// Fills exact squared-distance minima for every label-bit hypothesis.
    /// # Arguments
    /// * `received` - Validated normalized I/Q.
    /// * `output` - Exactly one MSB-first pair per reference bit.
    /// # Returns
    /// Unit; lookup/search changes neither the table nor the received sample.
    pub(super) fn distances(&self, received: IqPair, output: &mut [[f64; 2]]) {
        match &self.geometry {
            Geometry::Axes(axes) => axes.distances(received, output),
            Geometry::Spatial(spatial) => spatial.distances(received, output),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal_processing::modulation::reference_for_order;

    #[test]
    fn geometry_admission_never_substitutes_square_labels_for_cross_qam() {
        for order in [2, 4, 16, 64, 256, 1024, 4096, 16384] {
            let index = DemapIndex::new(reference_for_order(order).unwrap());
            assert!(
                matches!(index.geometry, Geometry::Axes(_)),
                "{order}-QAM must use verified axis labels"
            );
        }
        for order in [128, 512, 2048, 8192] {
            let index = DemapIndex::new(reference_for_order(order).unwrap());
            assert!(
                matches!(index.geometry, Geometry::Spatial(_)),
                "{order}-QAM must preserve its cross geometry"
            );
        }
    }
}
