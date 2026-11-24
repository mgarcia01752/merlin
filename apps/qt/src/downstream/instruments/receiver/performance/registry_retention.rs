/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Passive spare-slot metadata outside each already-admitted performance Entry.
use super::Entry;
use crate::shell::{RetainedResources, RetentionError};
use std::mem::size_of;

/// Measures only the actual performance Vec slots not occupied by charged entries.
///
/// # Arguments
/// * `entries` - Exact retained registry, preserving its requested spare capacity.
///
/// # Returns
/// Spare slot bytes only, with zero ordered entries and shared roots. Each live
/// Entry's last guard already charges its entire fixed layout and every dynamic
/// child; counting live slots or those children here would count them twice.
/// The fixed Vec handle belongs to the separate Store infrastructure owner.
///
/// # Errors
/// Checked spare-slot subtraction or layout multiplication overflow. The query
/// borrows entries and leaves all data, capacities and reservations unchanged.
pub(super) fn resources(entries: &Vec<Entry>) -> Result<RetainedResources, RetentionError> {
    for entry in entries {
        // Every current field, including the guard, belongs to the live Entry
        // charge from retention::PreparedEntry::admit. A new field must be
        // audited there and here before it can silently change this boundary.
        let Entry {
            id: _,
            channel: _,
            status: _,
            fec: _,
            ber: _,
            pending: _,
            points: _,
            error: _,
            fec_details: _,
            _retained: _,
        } = entry;
    }
    let spare_slots = entries
        .capacity()
        .checked_sub(entries.len())
        .ok_or(RetentionError::ArithmeticOverflow)?;
    Ok(RetainedResources {
        payload_bytes: spare_slots
            .checked_mul(size_of::<Entry>())
            .ok_or(RetentionError::ArithmeticOverflow)?,
        ordered_entries: 0,
        shared_roots: 0,
    })
}

#[cfg(test)]
mod tests;
