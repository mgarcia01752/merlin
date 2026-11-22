/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Complete original catalog and atomic bounded preflight; each execution reuses the public job.
use super::*;
use serde::Serialize;
use std::collections::BTreeSet;

/// Existing desktop sweep policy, independent of the checked simulation identity domain.
pub const MAX_FULL_LOOP_SWEEP_PROFILES: usize = 32;
/// Existing aggregate selected-assignment policy, independent of any one FFT dimension.
pub const MAX_FULL_LOOP_SWEEP_ASSIGNMENTS: usize = 262_144;

/// Explicit current, complete captured or selected original loading; no implicit mode change.
#[derive(Clone)]
pub enum FullLoopProfileSelection {
    Current,
    AllCaptured,
    Selected(Vec<ProfileId>),
}

/// Constant-size policy totals plus finite original/selected identity inventories.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopSweepInfo {
    pub selection: &'static str,
    /// Every encoded source ID, including entries outside the simulation identity domain.
    pub source_profile_ids: Vec<u8>,
    pub selected_profile_ids: Vec<ProfileId>,
    pub total_profile_assignments: usize,
    /// Sum of requested source-observation windows; excludes acquisition/drain and wall time.
    pub requested_observation_duration_ms: u64,
    /// Sum of per-profile active host budgets; acknowledged pause time remains excluded.
    pub maximum_sequential_runtime_ms: u64,
    pub maximum_retained_word_details: usize,
}

/// Immutable accepted serial dispatch plan; no worker or collection of scientific results is owned.
#[derive(Clone)]
pub struct FullLoopSweepPlan {
    base: FullLoopRequest,
    catalog: ProfileCapture,
    selection: FullLoopProfileSelection,
    selected: Vec<ProfileId>,
    total_assignments: usize,
}

impl FullLoopSweepPlan {
    /// Validates every selected profile before admitting any execution plan.
    /// # Arguments
    /// * `request` - Existing accepted current request, including original inputs and observations.
    /// * `catalog` - Complete original source catalog with the same ScenarioKey.
    /// * `selection` - Current preserves edited loading; captured modes select sorted originals.
    /// # Returns
    /// Shareable plan with at most 32 profiles/262144 selected assignments; no run starts.
    /// Preflight constructs one temporary request at a time and retains shared base/catalog data.
    /// # Errors
    /// Rejects key mismatch, empty/duplicate/missing/unsupported identities, aggregate limits
    /// and any existing per-profile scientific admission error, identifying the original ID.
    pub fn new(
        request: &FullLoopRequest,
        catalog: &ProfileCapture,
        selection: FullLoopProfileSelection,
    ) -> ApiResult<Self> {
        if &catalog.metadata().key != request.inputs.profile.key() {
            return Err(ApiError::new(
                ErrorCode::IncompatibleGrid,
                "Sweep source ScenarioKey differs from the current request",
            ));
        }
        let mut selected = match &selection {
            FullLoopProfileSelection::Current => vec![request.inputs.profile.profile_id()],
            FullLoopProfileSelection::AllCaptured => {
                if catalog.profile_ids().len() > MAX_FULL_LOOP_SWEEP_PROFILES {
                    return Err(ApiError::new(
                        ErrorCode::ResourceLimit,
                        "Captured sweep exceeds profile budget",
                    ));
                }
                catalog.profile_ids().iter().map(|raw| {
                    ProfileId::new(*raw).map_err(|_| ApiError::new(ErrorCode::UnsupportedCapability,
                        format!("Captured profile {raw} is outside the supported full-loop identity domain")))
                }).collect::<ApiResult<Vec<_>>>()?
            }
            FullLoopProfileSelection::Selected(ids) => {
                if ids.len() > MAX_FULL_LOOP_SWEEP_PROFILES {
                    return Err(ApiError::new(
                        ErrorCode::ResourceLimit,
                        "Selected sweep exceeds profile budget",
                    ));
                }
                ids.clone()
            }
        };
        if selected.is_empty() || selected.len() > MAX_FULL_LOOP_SWEEP_PROFILES {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Sweep requires 1..32 profiles",
            ));
        }
        let mut unique = BTreeSet::new();
        if selected.iter().any(|id| !unique.insert(id.value())) {
            return Err(ApiError::new(
                ErrorCode::InvalidInput,
                "Sweep contains duplicate profile identities",
            ));
        }
        selected.sort_by_key(|id| id.value());
        let mut plan = Self {
            base: request.clone(),
            catalog: catalog.clone(),
            selection,
            selected,
            total_assignments: 0,
        };
        for index in 0..plan.selected.len() {
            let candidate = plan.request(index)?;
            plan.total_assignments = plan
                .total_assignments
                .checked_add(candidate.inputs.profile.carriers().len())
                .filter(|total| *total <= MAX_FULL_LOOP_SWEEP_ASSIGNMENTS)
                .ok_or_else(|| {
                    ApiError::new(
                        ErrorCode::ResourceLimit,
                        "Sweep exceeds selected-assignment budget",
                    )
                })?;
        }
        Ok(plan)
    }

    /// Reads complete source IDs, selected run IDs and finite sequential resource ceilings.
    /// # Returns
    /// Owned metadata, without preparing a request or starting source/scientific work.
    pub fn info(&self) -> FullLoopSweepInfo {
        let count = self.selected.len();
        FullLoopSweepInfo {
            selection: match self.selection {
                FullLoopProfileSelection::Current => "current",
                FullLoopProfileSelection::AllCaptured => "all_captured",
                FullLoopProfileSelection::Selected(_) => "selected",
            },
            source_profile_ids: self.catalog.profile_ids().to_vec(),
            selected_profile_ids: self.selected.clone(),
            total_profile_assignments: self.total_assignments,
            requested_observation_duration_ms: self.base.options.duration_ms * count as u64,
            maximum_sequential_runtime_ms: self.base.options.max_runtime_ms * count as u64,
            maximum_retained_word_details: self.base.options.max_retained_words * count,
        }
    }

    /// Prepares one original profile through the same accepted full-loop scientific owners.
    /// # Arguments
    /// * `index` - Zero-based position in the plan's sorted selected-ID sequence.
    /// # Returns
    /// Independent immutable request preserving measurements/source/channel/options/observations.
    /// Current returns the exact accepted request, including edited hypotheses. Each run uses
    /// its own existing one-shot control; this operation starts no run or source stream.
    /// # Errors
    /// Rejects an invalid index or propagates original admission with the selected source ID.
    pub fn request(&self, index: usize) -> ApiResult<FullLoopRequest> {
        let id = self.selected.get(index).ok_or_else(|| {
            ApiError::new(
                ErrorCode::InvalidIdentifier,
                "Sweep profile position is unavailable",
            )
        })?;
        if matches!(self.selection, FullLoopProfileSelection::Current) {
            return Ok(self.base.clone());
        }
        let prepare = || {
            let mut inputs = self.base.inputs.clone();
            inputs.profile = self.catalog.simulation_profile(*id)?;
            let request = FullLoopRequest::prepare(inputs, self.base.options.clone())?;
            if let Some(policy) = &self.base.observation_policy {
                request.with_observations(policy.clone())
            } else {
                Ok(request)
            }
        };
        prepare().map_err(|mut cause: ApiError| {
            cause.message = format!("Captured profile {}: {}", id.value(), cause.message);
            cause
        })
    }
}
