/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual immutable downstream request/plan owners and cross-budget alias coverage.
use crate::shell::{
    Retained, RetainedResources, RetentionBudget, RetentionError, RetentionReservation,
};
use merlin::{
    common::RetainedFootprint,
    downstream::{ApiError, FullLoopRequest, FullLoopSweepPlan},
};
use std::{mem::size_of, sync::Arc};

// Each owner has an immutable scientific allocation and a retained wrapper allocation.
// Their private refcount headers are excluded by the approved payload-byte contract.
const OWNER_SHARED_ROOTS: usize = 2;

pub(super) type SharedRequest = Arc<Retained<Arc<FullLoopRequest>>>;
pub(super) type SharedPlan = Arc<Retained<Arc<FullLoopSweepPlan>>>;

#[derive(Debug)]
pub(super) enum OwnerError {
    Metadata(ApiError),
    Retention(RetentionError),
}

/// Admits an actual prepared request before allocating its final shared wrappers.
///
/// # Arguments
/// * `request` - Complete public candidate; temporary builders retain backend limits.
/// * `budget` - Same application budget as the accepting downstream host.
///
/// # Returns
/// Immutable aliases sharing one reservation until the final outer owner drops.
///
/// # Errors
/// Preserves public metadata overflow and checked aggregate dimension rejection.
pub(super) fn admit_request(
    request: FullLoopRequest,
    budget: &RetentionBudget,
) -> Result<SharedRequest, OwnerError> {
    let reservation = reserve::<FullLoopRequest>(request.retained_footprint(), budget)?;
    Ok(Arc::new(reservation.retain(Arc::new(request))))
}

/// Finishes finite runtime preparation before retaining the actual execution input.
///
/// # Arguments
/// * `request` - Owned selected-profile preparation candidate, consumed here.
/// * `runtime_ms` - Remaining active runtime; the public API only permits narrowing.
/// * `budget` - Canonical host policy, including accepted source and old result owners.
///
/// # Returns
/// The exact narrowed request with its current public footprint and concrete wrappers
/// charged throughout execution. The intermediate candidate drops before admission;
/// finite preparation builders are separate from final retained ownership.
///
/// # Errors
/// Preserves public runtime validation and atomic aggregate admission errors before
/// any execution control, source or scientific stage advances.
pub(super) fn admit_run_request(
    request: FullLoopRequest,
    runtime_ms: u64,
    budget: &RetentionBudget,
) -> Result<SharedRequest, OwnerError> {
    let narrowed = request
        .with_runtime_limit(runtime_ms)
        .map_err(OwnerError::Metadata)?;
    drop(request);
    admit_request(narrowed, budget)
}

/// Admits an actual prepared sweep before allocating its final shared wrappers.
///
/// # Arguments
/// * `plan` - Complete immutable public plan, including its own base/catalog owners.
/// * `budget` - Same application budget as the accepting downstream host.
///
/// # Returns
/// Immutable aliases sharing one reservation; repeated backend Arc roots count conservatively.
///
/// # Errors
/// Preserves public metadata overflow and checked aggregate dimension rejection.
pub(super) fn admit_plan(
    plan: FullLoopSweepPlan,
    budget: &RetentionBudget,
) -> Result<SharedPlan, OwnerError> {
    let reservation = reserve::<FullLoopSweepPlan>(plan.retained_footprint(), budget)?;
    Ok(Arc::new(reservation.retain(Arc::new(plan))))
}

/// Reserves target-host coverage before aliasing the same immutable scientific request.
///
/// # Arguments
/// * `request` - Source host's charged owner; its original aliases remain charged.
/// * `budget` - Target host budget, independent of the original reservation.
///
/// # Returns
/// A target-charged outer owner of the exact same inner Arc; no scientific copy.
///
/// # Errors
/// Rejection allocates no target wrapper/alias and preserves the source owner.
#[cfg(test)]
pub(super) fn rebind_request(
    request: &SharedRequest,
    budget: &RetentionBudget,
) -> Result<SharedRequest, OwnerError> {
    let reservation = reserve::<FullLoopRequest>(request.value().retained_footprint(), budget)?;
    Ok(Arc::new(reservation.retain(Arc::clone(request.value()))))
}

/// Reserves target-host coverage before aliasing the same immutable scientific plan.
///
/// # Arguments
/// * `plan` - Source host's charged owner; its original aliases remain charged.
/// * `budget` - Target host budget, independent of the original reservation.
///
/// # Returns
/// A target-charged outer owner of the exact same inner Arc; no scientific copy.
///
/// # Errors
/// Rejection allocates no target wrapper/alias and preserves the source owner.
#[cfg(test)]
pub(super) fn rebind_plan(
    plan: &SharedPlan,
    budget: &RetentionBudget,
) -> Result<SharedPlan, OwnerError> {
    let reservation = reserve::<FullLoopSweepPlan>(plan.value().retained_footprint(), budget)?;
    Ok(Arc::new(reservation.retain(Arc::clone(plan.value()))))
}

fn reserve<Scientific>(
    scientific: Result<RetainedFootprint, ApiError>,
    budget: &RetentionBudget,
) -> Result<RetentionReservation, OwnerError> {
    let scientific = scientific.map_err(OwnerError::Metadata)?;
    let resources = resources::<Scientific>(scientific).map_err(OwnerError::Retention)?;
    budget
        .reserve_resources(resources)
        .map_err(OwnerError::Retention)
}

fn resources<Scientific>(
    scientific: RetainedFootprint,
) -> Result<RetainedResources, RetentionError> {
    // The scientific fixed value is in its own inner Arc allocation. Keep it once;
    // the separate Retained<Arc<T>> wrapper adds only its own actual fixed layout.
    Ok(RetainedResources {
        payload_bytes: scientific
            .payload_bytes
            .checked_add(size_of::<Retained<Arc<Scientific>>>())
            .ok_or(RetentionError::ArithmeticOverflow)?,
        ordered_entries: scientific.ordered_entries,
        shared_roots: scientific
            .shared_roots
            .checked_add(OWNER_SHARED_ROOTS)
            .ok_or(RetentionError::ArithmeticOverflow)?,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests;
