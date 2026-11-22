/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Typed bounded profile inspection requests and source-coordinate results.
use super::*;

pub use crate::{
    scenario::pnm_json::profile_metadata::{ProfileCaptureHeader, ProfileSchemeInfo},
    types::modulation::CarrierType as ProfileCarrierRole,
};

/// Application rendering bound, independent of the number of source carriers.
pub const MAX_PROFILE_VIEW_BUCKETS: usize = 2048;
/// Complete byte-sized encoded identity inventory; independent of runtime ProfileId.
pub const MAX_PROFILE_SOURCE_IDS: usize = 256;
/// Original compact rows returned by one tree/page query.
pub const MAX_PROFILE_SCHEME_ROWS: usize = 256;

/// Explicit scientific projection, separate from receiver thresholds and measured MER.
#[derive(Clone, Copy, Serialize)]
pub enum ProfileMetric {
    #[serde(rename = "bits")]
    /// Modulated-carrier trace; zero/unmodulated assignments retain separate role spans.
    BitLoading,
    #[serde(rename = "shannon_db")]
    /// Ideal AWGN capacity bound for known constellations, independent of DOCSIS receiver limits.
    IdealShannonSnr,
}

/// Original source-role count, using the established presentation order.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ProfileRoleCount {
    pub kind: ProfileCarrierRole,
    pub count: usize,
}

/// Uncoded useful-symbol summary; no FEC/CP/MAC or measured-throughput inference.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ProfileSummary {
    /// Original byte-sized source identity, independent of legacy Simulation ProfileId.
    pub profile_id: u8,
    pub origin: ProfileOrigin,
    pub capture_time: CaptureTime,
    pub carrier_count: usize,
    pub role_counts: Vec<ProfileRoleCount>,
    pub data_bits_per_useful_symbol: u64,
    /// Known data-carrier contribution before CP, FEC and MAC overhead.
    pub uncoded_data_capacity_bps: u64,
    /// False when unknown/reserved roles or missing data loading prevent a complete estimate.
    pub capacity_complete: bool,
}

/// Small original header/catalog snapshot; compact schemes have a separate bounded query.
#[derive(Serialize)]
pub struct ProfileCaptureInfo {
    pub header: ProfileCaptureHeader,
    pub key: ScenarioKey,
    pub profile_ids: Vec<u8>,
    pub source_profile_order: Vec<u8>,
    pub original_kind: CaptureKind,
}

/// Bounded page of original compact scheme records, never reconstructed source ranges.
#[derive(Serialize)]
pub struct ProfileSchemeWindow {
    pub profile_id: u8,
    pub start: usize,
    pub total: usize,
    pub schemes: Vec<ProfileSchemeInfo>,
}

/// Explicit multi-profile request; selected identifiers refer to original source identities.
#[derive(Clone)]
pub struct ProfileViewRequest {
    pub profiles: Option<Vec<u8>>,
    pub roles: Vec<ProfileCarrierRole>,
    pub metric: ProfileMetric,
    pub viewport: MeasurementViewport,
    /// Total original-coordinate buckets across all selected profiles.
    pub max_buckets: usize,
}

impl Default for ProfileViewRequest {
    /// Selects the complete catalog/grid and existing source roles with a bounded total budget.
    /// # Returns
    /// Bit-loading request, all roles/profiles, at most 2048 buckets before role spans.
    fn default() -> Self {
        Self {
            profiles: None,
            roles: ProfileCarrierRole::ALL.to_vec(),
            metric: ProfileMetric::BitLoading,
            viewport: MeasurementViewport::default(),
            max_buckets: MAX_PROFILE_VIEW_BUCKETS,
        }
    }
}

impl ProfileViewRequest {
    /// Validates request resources before traversing any scientific profile arrays.
    /// # Returns
    /// Unit for at most 256 IDs, seven roles and a 1..2048 total bucket budget.
    /// # Errors
    /// ResourceLimit for excessive lists or an invalid output budget.
    pub fn validate(&self) -> ApiResult<()> {
        if self
            .profiles
            .as_ref()
            .is_some_and(|values| values.len() > MAX_PROFILE_SOURCE_IDS)
            || self.roles.len() > ProfileCarrierRole::ALL.len()
            || !(1..=MAX_PROFILE_VIEW_BUCKETS).contains(&self.max_buckets)
        {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Profile inspection selection/budget exceeds bounds",
            ));
        }
        Ok(())
    }
}

/// Exact original coordinate, without FFT-grid normalization or resampling.
#[derive(Clone, Copy, Serialize)]
pub struct ProfileCoordinate {
    pub offset: usize,
    pub physical_carrier: usize,
    pub frequency_hz: u32,
}

/// One selected role's actual support and loading extrema inside an original bucket.
#[derive(Serialize)]
pub struct ProfileRoleBucket {
    pub kind: ProfileCarrierRole,
    pub first: ProfileCoordinate,
    pub last: ProfileCoordinate,
    pub count: usize,
    pub minimum_bit_loading: Option<u8>,
    pub maximum_bit_loading: Option<u8>,
}

/// Raw scientific extrema and explicit gaps; pixel transforms remain presentation work.
#[derive(Serialize)]
pub struct ProfileMetricBucket {
    pub start: usize,
    pub count: usize,
    pub first_value: Option<ProfileCoordinate>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub gap: bool,
    pub roles: Vec<ProfileRoleBucket>,
}

/// One selected original profile's bounded metric/role view.
#[derive(Serialize)]
pub struct ProfileSeriesView {
    pub profile_id: u8,
    pub buckets: Vec<ProfileMetricBucket>,
}

/// Owned multi-profile observation, retaining original provenance without a hidden run.
#[derive(Serialize)]
pub struct ProfileInspection {
    pub key: ScenarioKey,
    pub sources: Vec<CaptureProvenance>,
    pub metric: ProfileMetric,
    pub start: usize,
    pub count: usize,
    pub first_frequency_hz: u32,
    pub last_frequency_hz: u32,
    pub profiles: Vec<ProfileSummary>,
    /// Whole-original-grid counts over selected profiles, independent of viewport/role filtering.
    pub selected_role_counts: Vec<ProfileRoleCount>,
    pub series: Vec<ProfileSeriesView>,
}
