/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Parsed export-worker input, independent of its retained source Session.
use super::*;
use crate::{
    downstream::inputs::{measurement_input::designer, retention as count},
    shell::RetainedResources,
};
use std::mem::size_of;

pub(super) struct Input {
    pub(super) request: Request,
    pub(super) selection: Selection,
}

impl Input {
    /// Admits one actual parsed recipe and fixed selection before export spawning.
    /// # Arguments
    /// * `self` - Existing bounded parsed controls, without revalidation or cloning.
    /// * `budget` - Same application counter as Session, previous Ready and products.
    /// # Returns
    /// Unchanged guarded input retained through construction and product admission.
    /// # Errors
    /// Checked metadata/layout or host rejection preserves the old Ready snapshot.
    pub(super) fn admit(self, budget: &RetentionBudget) -> Result<Retained<Self>, String> {
        let Selection {
            first,
            last,
            clamp,
            collection,
        } = &self.selection;
        let mut resources = RetainedResources {
            payload_bytes: size_of::<Self>(),
            ..Default::default()
        };
        count::append(
            &mut resources,
            count::fixed_children(&(*first, *last, *clamp, *collection)),
        )?;
        count::append(&mut resources, designer::request_children(&self.request)?)?;
        count::admit_model(self, resources, budget)
    }
}
