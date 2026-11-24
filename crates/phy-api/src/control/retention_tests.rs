/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Private common-stop allocation, conservative aliases and passive cancellation.
use super::*;
use crate::retention_support;
use std::mem::{size_of, size_of_val};

#[test]
fn common_child_counts_only_its_atomic_and_clones_keep_the_original_allocation() {
    let control = RunControl::new();
    let expected_children = RetainedFootprint {
        payload_bytes: size_of::<AtomicBool>(),
        ordered_entries: 0,
        shared_roots: 1,
    };
    let expected = RetainedFootprint {
        payload_bytes: size_of::<RunControl>() + expected_children.payload_bytes,
        ..expected_children
    };
    let weak = Arc::downgrade(&control.cancel);
    let alias = control.clone();
    assert!(Arc::ptr_eq(&control.cancel, &alias.cancel));
    assert_eq!(control.retained_children(), Some(expected_children));
    assert_eq!(
        retention_support::retained_footprint(&control),
        Some(expected)
    );
    assert!(!control.is_cancelled());
    drop(control);
    assert!(weak.upgrade().is_some());
    alias.cancel();
    assert_eq!(
        retention_support::retained_footprint(&alias),
        Some(expected)
    );
    assert!(alias.is_cancelled());
    drop(alias);
    assert!(weak.upgrade().is_none());
    assert_eq!(expected.shared_roots, 1);
}

#[test]
fn independently_retained_aliases_repeat_the_same_atomic_without_mutation() {
    let original = RunControl::new();
    let aliases = [original.clone(), original.clone()];
    let before = original.is_cancelled();
    let footprint = retention_support::retained_footprint(&aliases).unwrap();
    assert_eq!(
        footprint.payload_bytes,
        size_of_val(&aliases) + aliases.len() * size_of::<AtomicBool>()
    );
    assert_eq!(footprint.shared_roots, aliases.len());
    assert_eq!(footprint.ordered_entries, 0);
    assert_eq!(original.is_cancelled(), before);
    assert_eq!(
        retention_support::retained_footprint(&aliases),
        Some(footprint)
    );
}
