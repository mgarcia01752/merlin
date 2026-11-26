/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Finite labelled reference projections and Euclidean half-plane regions over original tables.
use crate::{
    error::invalid, signal_processing::modulation::reference_for_order, types::IqPair,
    SimulationError,
};
use serde::Serialize;

/// Maximum copied original constellation labels in one passive reference page.
const MAX_REFERENCE_VIEW_POINTS: usize = 4096;
/// Finite decision viewport half extent in nearest-neighbor spacings, not a slicing threshold.
const DECISION_VIEWPORT_NEIGHBOR_SPACINGS: f64 = 2.5;
/// Independent safety ceiling for vertices produced by half-plane clipping.
const MAX_DECISION_REGION_VERTICES: usize = 64;
/// Maximum nearest labelled references shown beside the selected decision region.
const MAX_NEARBY_REFERENCE_POINTS: usize = 49;

/// One original label and its known-amplitude reference I/Q; no received sample is fitted.
#[derive(Serialize)]
#[non_exhaustive]
pub struct ReferencePoint {
    pub label: u16,
    pub iq: IqPair,
}
/// Bounded page of original label-ordered coordinates.
#[derive(Serialize)]
#[non_exhaustive]
pub struct ReferenceView {
    pub order: u16,
    pub bits_per_symbol: u8,
    pub normalization_scale_factor: f64,
    pub reference_amplitude: f64,
    pub start: usize,
    pub total_points: usize,
    pub points: Vec<ReferencePoint>,
}
/// A nearby actual reference label in coordinates relative to the selected center.
#[derive(Serialize)]
#[non_exhaustive]
pub struct RelativeReference {
    pub label: u16,
    pub relative_iq: IqPair,
}
/// Closed geometric nearest-reference region intersected with an explicit finite viewport.
#[derive(Serialize)]
#[non_exhaustive]
pub struct DecisionRegion {
    pub order: u16,
    pub selected_label: u16,
    pub reference_amplitude: f64,
    pub center_iq: IqPair,
    pub viewport_half_extent: f64,
    pub boundary_tie_rule: &'static str,
    pub nearby_references: Vec<RelativeReference>,
    pub vertices_relative: Vec<IqPair>,
}

fn validate_amplitude(amplitude: f64) -> Result<(), SimulationError> {
    if !amplitude.is_finite() || amplitude <= 0.0 {
        return Err(invalid("Reference amplitude must be finite and positive"));
    }
    Ok(())
}
fn scaled(point: IqPair, amplitude: f64) -> Result<IqPair, SimulationError> {
    let value = point.map(|coordinate| coordinate * amplitude);
    if value.iter().any(|coordinate| !coordinate.is_finite()) {
        return Err(invalid("Reference coordinates exceed finite range"));
    }
    if point
        .iter()
        .zip(value)
        .any(|(original, scaled)| *original != 0.0 && scaled == 0.0)
    {
        return Err(invalid("Reference coordinate underflows its nonzero scale"));
    }
    Ok(value)
}

/// Copies a finite page from the original reference with an explicitly known amplitude.
/// # Arguments
/// * `order`, `amplitude` - Existing table and positive finite reference multiplier.
/// * `start`, `count` - Original label window; zero count copies up to 4096 remaining points.
/// # Returns
/// Owned labelled I/Q and original normalization metadata; no source/session state changes.
/// # Errors
/// Rejects unsupported references, invalid windows/work/amplitude and coordinate overflow.
pub(crate) fn reference_view(
    order: u16,
    amplitude: f64,
    start: usize,
    count: usize,
) -> Result<ReferenceView, SimulationError> {
    validate_amplitude(amplitude)?;
    let reference = reference_for_order(usize::from(order))
        .ok_or_else(|| invalid("Unsupported constellation reference"))?;
    if start >= reference.iq.len() || count > MAX_REFERENCE_VIEW_POINTS {
        return Err(invalid(
            "Reference viewport is outside original labels or exceeds 4096 points",
        ));
    }
    let count = (reference.iq.len() - start).min(if count == 0 {
        MAX_REFERENCE_VIEW_POINTS
    } else {
        count
    });
    let points = reference.iq[start..start + count]
        .iter()
        .enumerate()
        .map(|(offset, point)| {
            Ok(ReferencePoint {
                label: (start + offset) as u16,
                iq: scaled(*point, amplitude)?,
            })
        })
        .collect::<Result<Vec<_>, SimulationError>>()?;
    Ok(ReferenceView {
        order,
        bits_per_symbol: reference.bit_loading,
        normalization_scale_factor: reference.scale_factor,
        reference_amplitude: amplitude,
        start,
        total_points: reference.iq.len(),
        points,
    })
}

fn half_plane(polygon: &[IqPair], normal: IqPair, bound: f64) -> Vec<IqPair> {
    let distance = |point: IqPair| point[0] * normal[0] + point[1] * normal[1] - bound;
    if polygon.iter().all(|point| distance(*point) <= 0.0) {
        return polygon.to_vec();
    }
    let Some(mut previous) = polygon.last().copied() else {
        return Vec::new();
    };
    let mut previous_distance = distance(previous);
    let mut result = Vec::with_capacity(polygon.len() + 1);
    for &current in polygon {
        let current_distance = distance(current);
        if (previous_distance <= 0.0) != (current_distance <= 0.0) {
            let fraction = previous_distance / (previous_distance - current_distance);
            result.push([
                previous[0] + fraction * (current[0] - previous[0]),
                previous[1] + fraction * (current[1] - previous[1]),
            ]);
        }
        if current_distance <= 0.0 {
            result.push(current);
        }
        previous = current;
        previous_distance = current_distance;
    }
    result.dedup();
    if result.len() > 1 && result.first() == result.last() {
        result.pop();
    }
    result
}

/// Intersects all original nearest-reference half planes with a finite center-relative viewport.
/// # Arguments
/// * `order`, `label` - Existing constellation and original selected binary label.
/// * `amplitude` - Positive finite known reference multiplier, never fitted from received data.
/// # Returns
/// At most 49 nearest labelled references and 64 polygon vertices, in explicit reference units.
/// The viewport half extent is 2.5 times nearest-neighbor distance. Boundaries are geometric;
/// exact ties still follow the existing slicer's lowest-label rule, never a new classifier.
/// # Errors
/// Rejects unsupported references/labels, invalid amplitude, finite-range or polygon work limits.
pub(crate) fn decision_region(
    order: u16,
    label: u16,
    amplitude: f64,
) -> Result<DecisionRegion, SimulationError> {
    validate_amplitude(amplitude)?;
    let reference = reference_for_order(usize::from(order))
        .ok_or_else(|| invalid("Unsupported constellation reference"))?;
    let center = *reference
        .iq
        .get(usize::from(label))
        .ok_or_else(|| invalid("Decision label is outside the original constellation"))?;
    let mut neighbors: Vec<_> = reference
        .iq
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let offset = [point[0] - center[0], point[1] - center[1]];
            (
                index as u16,
                offset,
                offset[0] * offset[0] + offset[1] * offset[1],
            )
        })
        .collect();
    neighbors.sort_unstable_by(|left, right| left.2.total_cmp(&right.2).then(left.0.cmp(&right.0)));
    let spacing = neighbors
        .iter()
        .find(|item| item.2 > 0.0)
        .ok_or_else(|| invalid("Reference has no distinct neighbor"))?
        .2
        .sqrt();
    let radius = DECISION_VIEWPORT_NEIGHBOR_SPACINGS * spacing;
    let mut polygon = vec![
        [-radius, -radius],
        [radius, -radius],
        [radius, radius],
        [-radius, radius],
    ];
    for (_, offset, squared_distance) in neighbors.iter().filter(|item| item.0 != label) {
        polygon = half_plane(&polygon, *offset, squared_distance / 2.0);
        if polygon.len() > MAX_DECISION_REGION_VERTICES {
            return Err(invalid("Decision region exceeds 64 vertices"));
        }
    }
    let vertices_relative = polygon
        .into_iter()
        .map(|point| scaled(point, amplitude))
        .collect::<Result<Vec<_>, _>>()?;
    let nearby_references = neighbors
        .iter()
        .take(MAX_NEARBY_REFERENCE_POINTS)
        .map(|(label, offset, _)| {
            Ok(RelativeReference {
                label: *label,
                relative_iq: scaled(*offset, amplitude)?,
            })
        })
        .collect::<Result<Vec<_>, SimulationError>>()?;
    let viewport_half_extent = radius * amplitude;
    if !viewport_half_extent.is_finite() || viewport_half_extent <= 0.0 {
        return Err(invalid("Decision viewport exceeds finite range"));
    }
    Ok(DecisionRegion {
        order,
        selected_label: label,
        reference_amplitude: amplitude,
        center_iq: scaled(center, amplitude)?,
        viewport_half_extent,
        boundary_tie_rule: "lowest_label",
        nearby_references,
        vertices_relative,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bpsk_region_has_independent_half_plane_box_and_known_amplitude_geometry() {
        let region = decision_region(2, 0, 1.0).unwrap();
        assert_eq!(region.center_iq, [1.0, 0.0]);
        assert_eq!(region.viewport_half_extent, 5.0);
        assert_eq!(region.vertices_relative.len(), 4);
        assert_eq!(region.nearby_references[0].label, 0);
        assert_eq!(region.nearby_references[0].relative_iq, [0.0, 0.0]);
        assert_eq!(region.nearby_references[1].label, 1);
        assert_eq!(region.nearby_references[1].relative_iq, [-2.0, 0.0]);
        let xs: Vec<_> = region.vertices_relative.iter().map(|p| p[0]).collect();
        let ys: Vec<_> = region.vertices_relative.iter().map(|p| p[1]).collect();
        assert_eq!(xs.iter().copied().reduce(f64::min), Some(-1.0));
        assert_eq!(xs.iter().copied().reduce(f64::max), Some(5.0));
        assert_eq!(ys.iter().copied().reduce(f64::min), Some(-5.0));
        assert_eq!(ys.iter().copied().reduce(f64::max), Some(5.0));
        let scaled = decision_region(2, 0, 2.0).unwrap();
        assert_eq!(scaled.center_iq, [2.0, 0.0]);
        assert_eq!(scaled.viewport_half_extent, 10.0);
        for (point, expected) in scaled
            .vertices_relative
            .iter()
            .zip(region.vertices_relative)
        {
            assert_eq!(*point, expected.map(|v| 2.0 * v));
        }
    }
    #[test]
    fn every_accepted_reference_region_is_bounded_and_its_vertices_are_nearest_to_selected_label() {
        for order in [2, 4, 16, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384] {
            let reference = reference_for_order(order as usize).unwrap();
            for label in [0, 1, order / 3, order / 2, order - 1] {
                let region = decision_region(order, label, 1.0).unwrap();
                assert_eq!(region.center_iq, reference.iq[label as usize]);
                assert!(
                    !region.vertices_relative.is_empty() && region.vertices_relative.len() <= 64
                );
                assert!(region.nearby_references.len() <= 49);
                for point in &region.vertices_relative {
                    assert!(point
                        .iter()
                        .all(|v| v.is_finite() && v.abs() <= region.viewport_half_extent + 1e-10));
                    let actual = [
                        point[0] + region.center_iq[0],
                        point[1] + region.center_iq[1],
                    ];
                    let own_distance = point[0] * point[0] + point[1] * point[1];
                    for candidate in reference.iq {
                        let dx = actual[0] - candidate[0];
                        let dy = actual[1] - candidate[1];
                        assert!(
                            own_distance <= dx * dx + dy * dy + 1e-10,
                            "order {order} label {label}"
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn reference_pages_keep_original_labels_and_reject_unsupported_or_nonfinite_work() {
        let bpsk = reference_view(2, 2.0, 0, 0).unwrap();
        assert_eq!(bpsk.points.len(), 2);
        assert_eq!(bpsk.points[0].iq, [2.0, 0.0]);
        assert_eq!(bpsk.points[1].iq, [-2.0, 0.0]);
        let page = reference_view(16384, 1.0, 4094, 5).unwrap();
        assert_eq!(page.total_points, 16384);
        assert_eq!(
            page.points.iter().map(|p| p.label).collect::<Vec<_>>(),
            [4094, 4095, 4096, 4097, 4098]
        );
        assert_eq!(reference_view(16384, 1.0, 0, 0).unwrap().points.len(), 4096);
        assert_eq!(
            reference_view(16384, 1.0, 16383, 5).unwrap().points.len(),
            1
        );
        for amplitude in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::MAX] {
            assert!(decision_region(2, 0, amplitude).is_err());
        }
        assert!(reference_view(8, 1.0, 0, 0).is_err());
        assert!(reference_view(16, 1.0, 16, 0).is_err());
        assert!(reference_view(16384, 1.0, 0, 4097).is_err());
        assert!(reference_view(16384, f64::from_bits(1), 0, 0).is_err());
        assert!(decision_region(16, 16, 1.0).is_err());
    }
}
