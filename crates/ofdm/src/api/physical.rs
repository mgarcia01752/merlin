/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Configured physical-role inspection; source identity never becomes simulated acceptance.
use super::*;
pub use crate::transmission::layout::view::{
    PhysicalLayoutCarrier, PhysicalLayoutView, PhysicalRoleSpan,
};
pub use crate::transmission::layout::PhysicalCarrierRole;
use crate::{
    profile::candidate::CapturedProfile,
    simulation::channel::measurement::captured_layout,
    transmission::layout::{view::layout_view, CarrierLayout},
};
use serde::Serialize;

/// Configured full-FFT geometry with unchanged original source identity and capture grid.
#[derive(Serialize)]
#[non_exhaustive]
pub struct ConfiguredPhysicalLayout {
    pub reference_plane: &'static str,
    pub profile_id: u8,
    pub key: ScenarioKey,
    pub sources: Vec<CaptureProvenance>,
    pub layout: PhysicalLayoutView,
}

impl TransmissionConfiguration {
    /// Inspects authoritative physical-bin roles without creating or advancing a transmitter.
    /// # Arguments
    /// * `profile` - Accepted immutable profile on its original captured grid.
    /// * `channel_symbol` - Existing backend PLC-aligned channel-symbol convention.
    /// * `start` - Original physical FFT bin; distinct from the capture's first carrier.
    /// * `count` - Positive count, or zero for all remaining bins, at most 8192.
    /// # Returns
    /// Configured geometry and at most 1024 role spans; Data is not a transported cell role.
    /// # Errors
    /// Preserves profile/layout/grid admission; invalid windows or query budgets reject.
    pub fn physical_layout(
        &self,
        profile: &Dpd,
        channel_symbol: u64,
        start: usize,
        count: usize,
    ) -> ApiResult<ConfiguredPhysicalLayout> {
        self.inspect_layout(
            &profile.profile,
            profile.sources.clone(),
            channel_symbol,
            start,
            count,
        )
    }

    /// Inspects an original encoded catalog identity without widening legacy run-profile IDs.
    /// # Arguments
    /// * `capture` - Immutable complete source catalog, with original bytes and provenance.
    /// * `profile_id` - Original byte-sized encoded source profile identity.
    /// * `channel_symbol` - Existing backend PLC-aligned channel-symbol convention.
    /// * `start` - Original physical FFT bin, independent of capture/display offsets.
    /// * `count` - Positive count, or zero for remaining bins, at most 8192.
    /// # Returns
    /// Configured geometry only; successful inspection does not establish full-loop support.
    /// # Errors
    /// Rejects absent identities, incompatible layout/grid and invalid/oversized windows.
    pub fn physical_layout_capture(
        &self,
        capture: &ProfileCapture,
        profile_id: u8,
        channel_symbol: u64,
        start: usize,
        count: usize,
    ) -> ApiResult<ConfiguredPhysicalLayout> {
        self.inspect_layout(
            capture.profile_ref(profile_id)?,
            vec![capture.provenance().clone()],
            channel_symbol,
            start,
            count,
        )
    }

    fn inspect_layout(
        &self,
        profile: &CapturedProfile,
        sources: Vec<CaptureProvenance>,
        channel_symbol: u64,
        start: usize,
        count: usize,
    ) -> ApiResult<ConfiguredPhysicalLayout> {
        check_count(count)?;
        self.check_profile_grid(&profile.key.grid)?;
        let layout =
            CarrierLayout::new(self.settings(), &captured_layout(profile, self.settings())?)?;
        Ok(ConfiguredPhysicalLayout {
            reference_plane: "configured_physical_layout",
            profile_id: profile.profile_id,
            key: profile.key.clone(),
            sources,
            layout: layout_view(&layout, channel_symbol, start, count)?,
        })
    }
}

pub(super) fn check_count(count: usize) -> ApiResult<()> {
    if count > 8192 {
        return Err(ApiError::new(
            ErrorCode::ResourceLimit,
            "Physical query exceeds 8192 elements",
        ));
    }
    Ok(())
}
