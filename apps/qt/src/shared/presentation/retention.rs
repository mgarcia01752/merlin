/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Existing application reservations for persistent logical presentation copies.
use crate::shell::{Retained, RetainedResources, RetentionBudget, RetentionReservation};
use std::mem::size_of;

/// Maximum exact integer accepted from a JavaScript Number (ECMAScript safe range).
/// This conversion boundary is independent of every application population limit.
const JAVASCRIPT_MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;

/// One concrete view's admitted state; QObject and JavaScript engine headers are excluded.
pub(crate) struct View {
    budget: RetentionBudget,
    current: Option<RetentionReservation>,
    candidate: Option<RetentionReservation>,
    open: Option<RetentionReservation>,
    // Actual state data and reservations are disposed before this fixed witness.
    _fixed: Retained<()>,
}

impl View {
    /// Admits the actual Rust state and one retained-view population before allocation.
    ///
    /// # Arguments
    /// * `budget` - Canonical application accounting shared by both directions.
    /// # Returns
    /// One empty view state; data and open status have independent reservations.
    /// # Errors
    /// Rejects fixed storage or retained-view admission without changing the budget.
    pub(crate) fn new(budget: &RetentionBudget) -> Result<Box<Self>, String> {
        let fixed = budget
            .reserve_resources(RetainedResources {
                payload_bytes: size_of::<Self>(),
                retained_views: 1,
                ..Default::default()
            })
            .map_err(|_| "Presentation view limits would be exceeded.".to_owned())?;
        Ok(Box::new(Self {
            budget: budget.clone(),
            current: None,
            candidate: None,
            open: None,
            _fixed: fixed.retain(()),
        }))
    }

    /// Pre-admits a measured candidate while the current presentation stays covered.
    ///
    /// # Arguments
    /// * `utf16_bytes` - Exact logical UTF-16 bytes, including object-key strings.
    /// * `entries` - Exact logical array slots plus object properties.
    /// # Returns
    /// True with a pending reservation, or false with both existing owners unchanged.
    /// Reentrant preparation cannot replace a pending transaction.
    pub(crate) fn prepare(&mut self, utf16_bytes: f64, entries: f64) -> bool {
        if self.candidate.is_some() {
            return false;
        }
        let Some(resources) = quantities(utf16_bytes, entries) else {
            return false;
        };
        match self.budget.reserve_resources(resources) {
            Ok(candidate) => {
                self.candidate = Some(candidate);
                true
            }
            Err(_) => false,
        }
    }

    /// Finishes replacement after QML has assigned the admitted candidate.
    /// # Returns
    /// True after retiring the old reservation, or false if no candidate exists.
    pub(crate) fn commit(&mut self) -> bool {
        let Some(candidate) = self.candidate.take() else {
            return false;
        };
        self.current = Some(candidate);
        true
    }

    /// Disposes only the unpublished candidate after an aborted QML assignment.
    /// The accepted current data and open-view population stay charged.
    pub(crate) fn cancel(&mut self) {
        self.candidate = None;
    }

    /// Admits an open slot before opening a native view; repeated claims share it.
    /// # Returns
    /// True for an admitted/already-open view, false without mutation on rejection.
    pub(crate) fn begin_open(&mut self) -> bool {
        if self.open.is_some() {
            return true;
        }
        match self.budget.reserve_resources(RetainedResources {
            open_views: 1,
            ..Default::default()
        }) {
            Ok(open) => {
                self.open = Some(open);
                true
            }
            Err(_) => false,
        }
    }

    /// Ends visibility coverage without releasing hidden presentation data.
    /// The owner must call clear only after its persistent aliases are cleared.
    pub(crate) fn end_open(&mut self) {
        self.open = None;
    }

    /// Releases data only after QML has cleared the corresponding properties.
    /// Open status and fixed view storage remain independently covered.
    pub(crate) fn clear(&mut self) {
        self.candidate = None;
        self.current = None;
    }

    /// Reports whether an empty closed state can be disposed by its adapter.
    /// # Returns
    /// True only when no accepted, pending or open owner remains.
    pub(crate) fn is_empty(&self) -> bool {
        self.current.is_none() && self.candidate.is_none() && self.open.is_none()
    }
}

fn quantities(utf16_bytes: f64, entries: f64) -> Option<RetainedResources> {
    fn count(value: f64) -> Option<usize> {
        if !value.is_finite()
            || !(0.0..=JAVASCRIPT_MAX_SAFE_INTEGER).contains(&value)
            || value.fract() != 0.0
        {
            return None;
        }
        usize::try_from(value as u64).ok()
    }
    Some(RetainedResources {
        presentation_utf16_bytes: count(utf16_bytes)?,
        presentation_entries: count(entries)?,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests;
