/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Passive labelled-reference queries; numerical preparation stays with the modulation owner.
use super::*;
use crate::signal_processing::modulation::{reference_for_order, slicing::views};
pub use views::{
    DecisionRegion as QamDecisionRegion, ReferencePoint as QamReferencePoint,
    ReferenceView as QamReferenceView, RelativeReference as QamRelativeReference,
};

fn check_order(order: u16) -> ApiResult<()> {
    if reference_for_order(usize::from(order)).is_none() {
        return Err(ApiError::new(
            ErrorCode::UnsupportedCapability,
            "Unsupported physical constellation reference",
        ));
    }
    Ok(())
}
impl Simulation {
    /// Copies original label-ordered constellation points with an explicit known amplitude.
    /// # Arguments
    /// * `order` - Existing physical reference size, including BPSK/QPSK control references.
    /// * `amplitude` - Finite positive known reference multiplier; one denotes unit mean energy.
    /// * `start`, `count` - Original label window; zero copies up to 4096 remaining points.
    /// # Returns
    /// Owned labelled coordinates/normalization metadata; no session or source is configured/run.
    /// Recognized reference coordinates do not imply a supported coding chain or modem capability.
    /// # Errors
    /// UnsupportedCapability for unknown orders, ResourceLimit for oversized output and
    /// InvalidIdentifier for absent starting labels; original amplitude/numerical errors otherwise.
    pub fn constellation_reference(
        order: u16,
        amplitude: f64,
        start: usize,
        count: usize,
    ) -> ApiResult<QamReferenceView> {
        check_order(order)?;
        if count > 4096 {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Reference output exceeds 4096 points",
            ));
        }
        if start >= usize::from(order) {
            return Err(ApiError::new(
                ErrorCode::InvalidIdentifier,
                "Reference start is outside original labels",
            ));
        }
        Ok(views::reference_view(order, amplitude, start, count)?)
    }

    /// Queries a finite nearest-reference region through the original reference-table owner.
    /// # Arguments
    /// * `order`, `label` - Existing constellation and original physical binary label.
    /// * `amplitude` - Known positive finite reference multiplier, not a fitted correction.
    /// # Returns
    /// Owned center-relative region, explicit viewport and nearby labelled references.
    /// Closed geometric boundaries retain the existing slicer's lowest-label tie convention.
    /// # Errors
    /// Rejects unsupported orders, unavailable labels, invalid amplitudes and finite/work overflow.
    pub fn decision_region(order: u16, label: u16, amplitude: f64) -> ApiResult<QamDecisionRegion> {
        check_order(order)?;
        if label >= order {
            return Err(ApiError::new(
                ErrorCode::InvalidIdentifier,
                "Decision label is outside original constellation",
            ));
        }
        Ok(views::decision_region(order, label, amplitude)?)
    }
}
