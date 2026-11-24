/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Concrete parsed request and generation-selection owners, without scientific work.
use super::*;
use crate::{
    downstream::inputs::{
        measurement_input::{designer, model},
        retention as count,
    },
    shell::{RetainedResources, RetentionBudget},
};
use std::mem::size_of;

pub(super) struct SelectionSnapshot {
    pub(super) hashes: Vec<String>,
    pub(super) profile_hash: String,
    pub(super) options: Options,
    pub(super) imported: bool,
}

impl SelectionSnapshot {
    /// Admits the actual GUI-side generation snapshot before worker publication.
    /// # Arguments
    /// * `self` - Independent baseline hash, selection and playback snapshot.
    /// * `budget` - Same host counter as the GUI model, Session and worker input.
    /// # Returns
    /// Unchanged snapshot with a guard covering its actual fixed and child storage.
    /// # Errors
    /// Checked metadata/host rejection drops this candidate, preserving live work.
    pub(super) fn admit(self, budget: &RetentionBudget) -> Result<Retained<Self>, String> {
        let Self {
            hashes,
            profile_hash,
            options,
            imported,
        } = &self;
        let mut resources = RetainedResources {
            payload_bytes: size_of::<Self>(),
            ..Default::default()
        };
        count::append(
            &mut resources,
            RetainedResources {
                payload_bytes: profile_hash.capacity(),
                ..Default::default()
            },
        )?;
        count::append(&mut resources, count::fixed_children(imported))?;
        count::append(&mut resources, count::vector_storage(hashes)?)?;
        for hash in hashes {
            count::append(
                &mut resources,
                RetainedResources {
                    payload_bytes: hash.capacity(),
                    ..Default::default()
                },
            )?;
        }
        count::append(&mut resources, model::options_children(options)?)?;
        count::admit_model(self, resources, budget)
    }
}

/// Admits one actual parsed recipe independently of its source Designer Session.
/// # Arguments
/// * `request` - Existing bounded parsed controls; scientific validation is unchanged.
/// * `budget` - Canonical host shared with all queued and accepted owners.
/// # Returns
/// Original request guarded through detached execution and output admission.
/// # Errors
/// Checked child/layout overflow or aggregate rejection, without changing the GUI.
pub(super) fn admit_request(
    request: Request,
    budget: &RetentionBudget,
) -> Result<Retained<Request>, String> {
    let mut resources = RetainedResources {
        payload_bytes: size_of::<Request>(),
        ..Default::default()
    };
    count::append(&mut resources, designer::request_children(&request)?)?;
    count::admit_model(request, resources, budget)
}

/// Runs existing finite generation while retaining both admitted input owners.
/// # Arguments
/// * `session`, `request` - Independently charged original and parsed recipe owners.
/// * `cancel`, `completed` - Existing cooperative controls; no lifecycle change.
/// * `budget` - Canonical target counter covering concurrent inputs and output.
/// # Returns
/// Complete actual capture batch admitted before either input guard is released.
/// # Errors
/// Original scientific/cancellation errors or final retained-target rejection.
pub(super) fn generate(
    session: Retained<Session>,
    request: Retained<Request>,
    cancel: &AtomicBool,
    completed: &AtomicUsize,
    budget: &RetentionBudget,
) -> Result<Retained<Vec<Capture>>, AdmissionError> {
    session
        .value()
        .build(request.value(), cancel, completed)
        .and_then(|captures| {
            Model::admit_capture_batch(captures, budget).map_err(AdmissionError::from)
        })
}
