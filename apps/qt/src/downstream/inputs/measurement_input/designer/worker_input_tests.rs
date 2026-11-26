/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Passive actual worker-input children, independent of containing-owner admission.
use super::*;
use crate::downstream::inputs::measurement_input::model::{self, Options};
use merlin::downstream::inputs::{CaptureScope, ValueEdit};
use std::{borrow::Cow, mem::size_of};

// Independent requested spare capacities, not production or scientific limits.
const EXTRA_TEXT_BYTES: usize = 37;
const EXTRA_EDIT_SLOTS: usize = 11;
const EXTRA_SELECTED_SLOTS: usize = 7;

#[test]
fn request_source_and_recipe_spare_capacity_exclude_fixed_root_and_preserve_values() {
    let session = super::admission_tests::fixture();
    let mut request = session.request.clone();
    let grid = &session.profile.key.grid;

    // One valid full-grid edit uses the fixture's existing baseline value;
    // this metadata query never materializes it or changes original captures.
    request.recipe.edits.push(ValueEdit {
        scope: CaptureScope::All,
        first_hz: grid.frequency(0).unwrap(),
        last_hz: grid.frequency(grid.sample_count - 1).unwrap(),
        value_db: request.baseline_db,
    });
    let before = serde_json::to_value(&request).unwrap();
    let source_pointer = request.source.as_ptr();
    let edits_pointer = request.recipe.edits.as_ptr();
    let mut expected = RetainedResources {
        payload_bytes: request.source.capacity()
            + request.recipe.edits.capacity() * size_of::<ValueEdit>(),
        ordered_entries: 0,
        shared_roots: 0,
        ..Default::default()
    };
    assert_eq!(request_children(&request).unwrap(), expected);
    assert_eq!(request_children(&request).unwrap(), expected);
    assert_eq!(request.source.as_ptr(), source_pointer);
    assert_eq!(request.recipe.edits.as_ptr(), edits_pointer);

    let old = request.source.capacity();
    request.source.reserve_exact(old + EXTRA_TEXT_BYTES);
    expected.payload_bytes += request.source.capacity() - old;
    assert_eq!(request_children(&request).unwrap(), expected);
    let old = request.recipe.edits.capacity();
    let mut edits = Vec::with_capacity(old + EXTRA_EDIT_SLOTS);
    edits.append(&mut request.recipe.edits);
    expected.payload_bytes += (edits.capacity() - old) * size_of::<ValueEdit>();
    request.recipe.edits = edits;
    assert_eq!(request_children(&request).unwrap(), expected);
    assert_eq!(request_children(&request).unwrap(), expected);
    assert_eq!(serde_json::to_value(&request).unwrap(), before);
}

#[test]
fn options_owned_interval_and_selected_storage_are_passive_while_borrowed_text_costs_zero() {
    let session = super::admission_tests::fixture();
    let mut options = Options::default();
    let Cow::Borrowed(interval) = &options.interval_symbols else {
        panic!("Existing default interval is borrowed static text");
    };
    let interval = *interval;
    assert_eq!(
        model::options_children(&options).unwrap(),
        RetainedResources::default()
    );

    let mut owned_interval = String::with_capacity(interval.len() + EXTRA_TEXT_BYTES);
    owned_interval.push_str(interval);
    options.interval_symbols = Cow::Owned(owned_interval);
    let hash = &session.hashes[0];
    let mut owned_hash = String::with_capacity(hash.len() + EXTRA_TEXT_BYTES);
    owned_hash.push_str(hash);
    let mut selected = Vec::with_capacity(EXTRA_SELECTED_SLOTS);
    selected.push(owned_hash);
    options.selected_captures = Some(selected);
    let Cow::Owned(interval) = &options.interval_symbols else {
        panic!("Actual owned interval fixture");
    };
    let selected = options.selected_captures.as_ref().unwrap();
    let mut expected = RetainedResources {
        payload_bytes: interval.capacity()
            + selected.capacity() * size_of::<String>()
            + selected[0].capacity(),
        ordered_entries: 0,
        shared_roots: 0,
        ..Default::default()
    };
    let before = serde_json::to_value(&options).unwrap();
    let interval_pointer = interval.as_ptr();
    let selected_pointer = selected.as_ptr();
    assert_eq!(model::options_children(&options).unwrap(), expected);
    assert_eq!(model::options_children(&options).unwrap(), expected);
    assert_eq!(options.interval_symbols.as_ptr(), interval_pointer);
    assert_eq!(
        options.selected_captures.as_ref().unwrap().as_ptr(),
        selected_pointer
    );

    let Cow::Owned(interval) = &mut options.interval_symbols else {
        panic!("Unchanged owned interval fixture");
    };
    let old = interval.capacity();
    interval.reserve_exact(old + EXTRA_TEXT_BYTES);
    expected.payload_bytes += interval.capacity() - old;
    assert_eq!(model::options_children(&options).unwrap(), expected);
    let selected = options.selected_captures.as_mut().unwrap();
    let old = selected.capacity();
    let mut larger = Vec::with_capacity(old + EXTRA_SELECTED_SLOTS);
    larger.append(selected);
    expected.payload_bytes += (larger.capacity() - old) * size_of::<String>();
    *selected = larger;
    assert_eq!(model::options_children(&options).unwrap(), expected);
    let hash = &mut options.selected_captures.as_mut().unwrap()[0];
    let old = hash.capacity();
    hash.reserve_exact(old + EXTRA_TEXT_BYTES);
    expected.payload_bytes += hash.capacity() - old;
    assert_eq!(model::options_children(&options).unwrap(), expected);
    assert_eq!(serde_json::to_value(&options).unwrap(), before);

    let owned = std::mem::replace(
        &mut options.interval_symbols,
        Cow::Borrowed(crate::downstream::common::constants::DEFAULT_REPLAY_INTERVAL_TEXT),
    );
    let Cow::Owned(interval) = owned else {
        panic!("Owned interval transferred without cloning");
    };
    expected.payload_bytes -= interval.capacity();
    assert_eq!(model::options_children(&options).unwrap(), expected);
    assert_eq!(serde_json::to_value(&options).unwrap(), before);
}
