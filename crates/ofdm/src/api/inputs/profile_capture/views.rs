/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Authoritative profile summaries and bounded raw extrema; no pixel-space calculations.
use super::*;
use crate::signal_processing::{ofdm::useful_symbol_rate, shannon::minimum_snr_db};

pub(super) fn summary(
    profile: &CapturedProfile,
    origin: ProfileOrigin,
) -> ApiResult<ProfileSummary> {
    let mut role_counts = ProfileCarrierRole::ALL
        .iter()
        .map(|kind| ProfileRoleCount {
            kind: *kind,
            count: 0,
        })
        .collect::<Vec<_>>();
    let mut bits = 0u64;
    let mut complete = true;
    for carrier in &profile.assignments {
        if let Some(count) = role_counts
            .iter_mut()
            .find(|count| count.kind == carrier.carrier_type)
        {
            count.count += 1;
        }
        match carrier.carrier_type {
            ProfileCarrierRole::Data => {
                if let Some(loading) = carrier.bit_loading {
                    bits = bits.checked_add(u64::from(loading)).ok_or_else(|| {
                        ApiError::new(ErrorCode::ResourceLimit, "Profile data bit count overflow")
                    })?;
                } else {
                    complete = false;
                }
            }
            ProfileCarrierRole::Unknown | ProfileCarrierRole::Reserved => complete = false,
            _ => {}
        }
    }
    let rate = useful_symbol_rate(bits, profile.key.grid.spacing_hz).ok_or_else(|| {
        ApiError::new(
            ErrorCode::ResourceLimit,
            "Profile uncoded capacity exceeds supported units",
        )
    })?;
    Ok(ProfileSummary {
        profile_id: profile.profile_id,
        origin,
        capture_time: CaptureTime(profile.capture_time),
        carrier_count: profile.assignments.len(),
        role_counts,
        data_bits_per_useful_symbol: bits,
        uncoded_data_capacity_bps: rate,
        capacity_complete: complete,
    })
}

pub(super) fn inspect(
    capture: &ProfileCapture,
    request: &ProfileViewRequest,
) -> ApiResult<ProfileInspection> {
    inspect_profiles(
        &capture.profiles,
        &capture.summaries,
        &capture.source.key,
        vec![capture.provenance.clone()],
        request,
    )
}

pub(super) fn inspect_profiles(
    profiles: &[CapturedProfile],
    summaries: &[ProfileSummary],
    key: &ScenarioKey,
    sources: Vec<CaptureProvenance>,
    request: &ProfileViewRequest,
) -> ApiResult<ProfileInspection> {
    request.validate()?;
    key.grid.validate()?;
    let mut ids = request
        .profiles
        .clone()
        .unwrap_or_else(|| summaries.iter().map(|profile| profile.profile_id).collect());
    ids.sort_unstable();
    ids.dedup();
    if ids.len() > request.max_buckets {
        return Err(ApiError::new(
            ErrorCode::ResourceLimit,
            "Profile bucket budget is smaller than the selected profile count",
        ));
    }
    let selected = ids
        .iter()
        .map(|id| {
            profiles
                .iter()
                .find(|profile| profile.profile_id == *id)
                .ok_or_else(|| {
                    ApiError::new(
                        ErrorCode::InvalidIdentifier,
                        "Source profile identity is unavailable",
                    )
                })
        })
        .collect::<ApiResult<Vec<_>>>()?;
    let total = key.grid.sample_count;
    let (start, count) = if request.viewport.count == 0 {
        (0, total)
    } else {
        (request.viewport.start, request.viewport.count)
    };
    let end = start
        .checked_add(count)
        .filter(|end| *end <= total)
        .ok_or_else(|| {
            ApiError::new(
                ErrorCode::InvalidInput,
                "Profile viewport exceeds the original grid",
            )
        })?;
    if count == 0 {
        return Err(ApiError::new(
            ErrorCode::InvalidInput,
            "Empty profile viewport",
        ));
    }
    let mut role_counts = ProfileCarrierRole::ALL
        .iter()
        .map(|kind| ProfileRoleCount {
            kind: *kind,
            count: 0,
        })
        .collect::<Vec<_>>();
    let mut series = Vec::with_capacity(selected.len());
    let per_profile = request.max_buckets / selected.len().max(1);
    for profile in selected {
        for carrier in &profile.assignments {
            if let Some(count) = role_counts
                .iter_mut()
                .find(|count| count.kind == carrier.carrier_type)
            {
                count.count += 1;
            }
        }
        if profile.key != *key || profile.assignments.len() != total {
            return Err(ApiError::new(
                ErrorCode::IncompatibleGrid,
                "Profile view requires exact original grids",
            ));
        }
        let stride = count.div_ceil(per_profile);
        let mut buckets = Vec::new();
        for offset in (start..end).step_by(stride) {
            buckets.push(bucket(
                profile,
                offset,
                (end - offset).min(stride),
                request,
            )?);
        }
        series.push(ProfileSeriesView {
            profile_id: profile.profile_id,
            buckets,
        });
    }
    Ok(ProfileInspection {
        key: key.clone(),
        sources,
        metric: request.metric,
        start,
        count,
        first_frequency_hz: key.grid.frequency(start)?,
        last_frequency_hz: key.grid.frequency(end - 1)?,
        profiles: summaries.to_vec(),
        selected_role_counts: role_counts,
        series,
    })
}

fn coordinate(grid: &FrequencyGrid, offset: usize) -> ApiResult<ProfileCoordinate> {
    Ok(ProfileCoordinate {
        offset,
        physical_carrier: usize::from(grid.first_subcarrier_index) + offset,
        frequency_hz: grid.frequency(offset)?,
    })
}

fn bucket(
    profile: &CapturedProfile,
    start: usize,
    count: usize,
    request: &ProfileViewRequest,
) -> ApiResult<ProfileMetricBucket> {
    let carriers = &profile.assignments[start..start + count];
    let mut roles = Vec::new();
    for kind in ProfileCarrierRole::ALL {
        if !request.roles.contains(&kind) {
            continue;
        }
        let selected = carriers
            .iter()
            .enumerate()
            .filter(|(_, carrier)| carrier.carrier_type == kind)
            .collect::<Vec<_>>();
        let (Some(first), Some(last)) = (selected.first(), selected.last()) else {
            continue;
        };
        let loading = selected
            .iter()
            .filter_map(|(_, carrier)| carrier.bit_loading);
        roles.push(ProfileRoleBucket {
            kind,
            first: coordinate(&profile.key.grid, start + first.0)?,
            last: coordinate(&profile.key.grid, start + last.0)?,
            count: selected.len(),
            minimum_bit_loading: loading.clone().min(),
            maximum_bit_loading: loading.max(),
        });
    }
    let mut first_value = None;
    let mut minimum: Option<f64> = None;
    let mut maximum: Option<f64> = None;
    let mut gap = false;
    for (index, carrier) in carriers.iter().enumerate() {
        let value = if request.roles.contains(&carrier.carrier_type)
            && matches!(
                carrier.carrier_type,
                ProfileCarrierRole::Data
                    | ProfileCarrierRole::ContinuousPilot
                    | ProfileCarrierRole::Plc
            ) {
            carrier.bit_loading.and_then(|bits| match request.metric {
                ProfileMetric::BitLoading => Some(f64::from(bits)),
                ProfileMetric::IdealShannonSnr => minimum_snr_db(bits),
            })
        } else {
            None
        };
        if let Some(value) = value {
            if first_value.is_none() {
                first_value = Some(coordinate(&profile.key.grid, start + index)?);
            }
            minimum = Some(minimum.map_or(value, |current| current.min(value)));
            maximum = Some(maximum.map_or(value, |current| current.max(value)));
        } else {
            gap = true;
        }
    }
    Ok(ProfileMetricBucket {
        start,
        count,
        first_value,
        minimum,
        maximum,
        gap,
        roles,
    })
}
