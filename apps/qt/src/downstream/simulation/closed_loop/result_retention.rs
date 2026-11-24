/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Pre-execution backend result admission; dynamic presentation copies remain separate.
use super::ResultData;
use crate::shell::{
    ApplicationError, Retained, RetainedResources, RetentionBudget, RetentionError,
    RetentionReservation,
};
use merlin::{
    common::RetainedFootprint,
    downstream::{ApiError, FullLoopControl, FullLoopRequest, FullLoopResult},
};
use std::mem::size_of;

// The queued Box later moves into one Arc<ResultData>; reserve that final root
// before science. The backend's own roots are already in its public bound.
const FINAL_RESULT_SHARED_ROOTS: usize = 1;

/// Single-use target tied to the exact immutable request borrowed during preflight.
pub(super) struct PreparedResult<'request> {
    request: &'request FullLoopRequest,
    backend_limit: RetainedFootprint,
    reservation: RetentionReservation,
}

#[derive(Debug)]
pub(super) enum ResultError {
    Backend(ApiError),
    Retention(RetentionError),
}

impl<'request> PreparedResult<'request> {
    /// Reserves the complete public result and concrete fixed wrapper before execution.
    ///
    /// # Arguments
    /// * `request` - Exact immutable, runtime-narrowed public request; never executed here.
    /// * `budget` - Canonical host policy shared with accepted, queued and old owners.
    ///
    /// # Returns
    /// Single-use preflight; temporary backend builders retain their finite policies.
    /// Fixed ResultData/JobView/Retained storage and its future Arc root are covered.
    /// Observation/capture dynamic copies and other host owners remain separate.
    ///
    /// # Errors
    /// Preserves public-bound errors and checked aggregate payload/entry/root rejection.
    /// No control, source or scientific stage advances on failure.
    pub(super) fn new(
        request: &'request FullLoopRequest,
        budget: &RetentionBudget,
    ) -> Result<Self, ResultError> {
        let backend_limit = request
            .result_retained_bound()
            .map_err(ResultError::Backend)?;
        let resources = resources(backend_limit).map_err(ResultError::Retention)?;
        let reservation = budget
            .reserve_resources(resources)
            .map_err(ResultError::Retention)?;
        Ok(Self {
            request,
            backend_limit,
            reservation,
        })
    }

    /// Executes the exact preflighted public request while retaining its target charge.
    ///
    /// # Arguments
    /// * `control` - Existing public profile control, including requested pause/cancel.
    ///
    /// # Returns
    /// The unchanged complete/partial result carrying its reservation. There is no
    /// post-run size rejection or scientific-output copy during application adoption.
    ///
    /// # Errors
    /// Preserves original public execution errors and releases only this target.
    pub(super) fn run(
        self,
        control: &FullLoopControl,
    ) -> Result<Retained<FullLoopResult>, ResultError> {
        let result = self
            .request
            .run_bounded(control, self.backend_limit)
            .map_err(ResultError::Backend)?;
        Ok(self.reservation.retain(result))
    }
}

impl ResultError {
    /// Formats existing direction/backend and host-admission errors for the UI.
    ///
    /// # Returns
    /// Original backend message or the established bounded application diagnostic.
    pub(super) fn into_message(self) -> String {
        match self {
            Self::Backend(error) => error.message().to_owned(),
            Self::Retention(error) => super::application_error(ApplicationError::Retention(error)),
        }
    }
}

fn resources(scientific: RetainedFootprint) -> Result<RetainedResources, RetentionError> {
    // FullLoopResult moves inline into JobView(Retained<FullLoopResult>). Replace
    // its fixed root exactly once with the complete actual ResultData fixed layout,
    // which includes the reservation and observation container handles, not their
    // separately unadmitted dynamic buffers. No allocator/header bytes are guessed.
    let children = scientific
        .payload_bytes
        .checked_sub(size_of::<FullLoopResult>())
        .ok_or(RetentionError::ArithmeticOverflow)?;
    Ok(RetainedResources {
        payload_bytes: children
            .checked_add(size_of::<ResultData>())
            .ok_or(RetentionError::ArithmeticOverflow)?,
        ordered_entries: scientific.ordered_entries,
        shared_roots: scientific
            .shared_roots
            .checked_add(FINAL_RESULT_SHARED_ROOTS)
            .ok_or(RetentionError::ArithmeticOverflow)?,
        ..Default::default()
    })
}

/// Provides independent test policy without adding a production unbounded fallback.
///
/// # Returns
/// Test-only ceilings; actual reservations still use public result quantities.
#[cfg(test)]
pub(super) fn fixture_budget() -> RetentionBudget {
    RetentionBudget::with_limits(RetainedResources {
        payload_bytes: usize::MAX,
        ordered_entries: usize::MAX,
        shared_roots: usize::MAX,
        channel_count: usize::MAX,
        slot_count: usize::MAX,
    })
}

/// Keeps existing already-completed scientific fixtures outside production admission.
///
/// # Arguments
/// * `result` - Actual fixture result, produced before this test-only constructor.
/// * `budget` - Same independent fixture policy as its presentation copies.
///
/// # Returns
/// Actual metadata charge for legacy query fixtures, never a pre-execution proof.
///
/// # Panics
/// A metadata/arithmetic error violates the small fixture's existing expectations.
#[cfg(test)]
pub(super) fn retain_fixture(
    result: FullLoopResult,
    budget: &RetentionBudget,
) -> Retained<FullLoopResult> {
    let footprint = result.retained_footprint().expect("Real fixture metadata");
    budget
        .reserve_resources(resources(footprint).expect("Fixture wrapper layout"))
        .expect("Independent fixture policy")
        .retain(result)
}

#[cfg(test)]
mod tests;
