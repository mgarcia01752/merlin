/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Concrete application-owned atomic controls, distinct from private std queues.
use crate::shell::{Retained, RetainedResources, RetentionBudget, RetentionError};
use std::{
    mem::size_of,
    sync::{
        atomic::{AtomicBool, AtomicU8, AtomicUsize},
        Arc,
    },
};

// Each control has exactly one separately allocated, application-owned pointee.
const CONTROL_SHARED_ROOTS: usize = 1;

/// Admits the actual cancellation flag before allocating its shared owner.
/// # Arguments
/// * `budget` - Canonical host covering every GUI and worker alias.
/// # Returns
/// A false flag; its exact retained pointee and root survive until final drop.
/// # Errors
/// Checked aggregate rejection leaves existing controls and accepted state intact.
pub(crate) fn cancellation(
    budget: &RetentionBudget,
) -> Result<Arc<Retained<AtomicBool>>, RetentionError> {
    admit(AtomicBool::new(false), budget)
}

/// Admits the actual completed-capture counter shared by Designer and export.
/// # Arguments
/// * `budget` - Canonical host covering all simultaneous control aliases.
/// # Returns
/// A zero counter with exact pointee/root coverage; no scientific limit changes.
/// # Errors
/// Checked aggregate rejection before Arc allocation or dispatch.
pub(crate) fn completed(
    budget: &RetentionBudget,
) -> Result<Arc<Retained<AtomicUsize>>, RetentionError> {
    admit(AtomicUsize::new(0), budget)
}

/// Admits the loader's existing finite phase counter before publication.
/// # Arguments
/// * `budget` - Same host as the source, prepared project and loader queue.
/// # Returns
/// A zero stage counter, guarded through the final loader or worker alias.
/// # Errors
/// Checked aggregate rejection without starting work or changing a project.
pub(crate) fn load_stage(
    budget: &RetentionBudget,
) -> Result<Arc<Retained<AtomicU8>>, RetentionError> {
    admit(AtomicU8::new(0), budget)
}

fn admit<Control>(
    control: Control,
    budget: &RetentionBudget,
) -> Result<Arc<Retained<Control>>, RetentionError> {
    let reservation = budget.reserve_resources(RetainedResources {
        payload_bytes: size_of::<Retained<Control>>(),
        shared_roots: CONTROL_SHARED_ROOTS,
        ..Default::default()
    })?;
    Ok(Arc::new(reservation.retain(control)))
}

#[cfg(test)]
mod tests;
