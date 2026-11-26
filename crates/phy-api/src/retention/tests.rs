/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent requested-capacity and ownership anchors for metadata walkers.
use super::support::{capacity_bytes, payload_bytes, retained_footprint, sum, RetainedPayload};
use super::RetainedFootprint;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::mem::size_of;
use std::sync::Arc;

#[test]
fn nested_vectors_and_strings_include_spare_capacity_without_mutation() {
    // Deliberately different unused capacities expose length-only accounting.
    let mut text = String::with_capacity(31);
    text.push_str("abc");
    let mut values = Vec::with_capacity(7);
    values.push(text);
    let expected =
        size_of::<Vec<String>>() + values.capacity() * size_of::<String>() + values[0].capacity();
    let before = (values.capacity(), values[0].clone(), values[0].capacity());
    assert_eq!(retained_footprint(&values), Some(payload_bytes(expected)));
    assert_eq!(values.capacity(), before.0);
    assert_eq!(values[0], before.1);
    assert_eq!(values[0].capacity(), before.2);
}

#[test]
fn repeated_shared_edges_repeat_pointees_and_count_roots_without_headers() {
    // Two aliases to one retained vector must conservatively count it twice.
    let mut data = Vec::<u8>::with_capacity(19);
    data.extend_from_slice(&[1, 2, 3]);
    let shared = Arc::new(data);
    let values = vec![shared.clone(), shared.clone()];
    let expected = size_of::<Vec<Arc<Vec<u8>>>>()
        + values.capacity() * size_of::<Arc<Vec<u8>>>()
        + 2 * (size_of::<Vec<u8>>() + shared.capacity());
    let footprint = retained_footprint(&values).unwrap();
    assert_eq!(footprint.payload_bytes, expected);
    assert_eq!(footprint.shared_roots, 2);
    assert_eq!(footprint.ordered_entries, 0);
    assert_eq!(Arc::strong_count(&shared), 3);
}

#[test]
fn tuple_children_include_both_owned_fields_without_repeating_inline_padding() {
    // Unequal spare capacities belong to independent fields of one inline tuple.
    let mut text = String::with_capacity(17);
    text.push('x');
    let mut samples = Vec::<u16>::with_capacity(9);
    samples.push(1);
    let expected =
        size_of::<(String, Vec<u16>)>() + text.capacity() + samples.capacity() * size_of::<u16>();
    assert_eq!(
        retained_footprint(&(text, samples)),
        Some(payload_bytes(expected))
    );
}

#[test]
fn tree_entries_include_key_and_value_payloads_and_exclude_private_nodes() {
    let mut key = String::with_capacity(23);
    key.push_str("key");
    let data = vec![5_u8; 3].into_boxed_slice();
    let expected = size_of::<BTreeMap<String, Box<[u8]>>>()
        + size_of::<String>()
        + key.capacity()
        + size_of::<Box<[u8]>>()
        + data.len();
    let values = BTreeMap::from([(key, data)]);
    let footprint = retained_footprint(&values).unwrap();
    assert_eq!(footprint.payload_bytes, expected);
    assert_eq!(footprint.ordered_entries, 1);
    assert_eq!(footprint.shared_roots, 0);
    let set = BTreeSet::from([11_u16, 29_u16]);
    let footprint = retained_footprint(&set).unwrap();
    assert_eq!(footprint.ordered_entries, 2);
    assert_eq!(
        footprint.payload_bytes,
        size_of::<BTreeSet<u16>>() + 2 * size_of::<u16>()
    );
}

#[test]
fn ring_capacity_boxed_slices_and_inline_options_are_owned_once() {
    let mut values = VecDeque::with_capacity(11);
    values.push_back(Some(vec![1_u8, 2].into_boxed_slice()));
    values.push_front(None);
    let expected = size_of::<VecDeque<Option<Box<[u8]>>>>()
        + values.capacity() * size_of::<Option<Box<[u8]>>>()
        + 2; // Two bytes in the one owned boxed slice, not an allocator estimate.
    assert_eq!(retained_footprint(&values), Some(payload_bytes(expected)));
    let empty: Option<Vec<u8>> = None;
    assert_eq!(
        retained_footprint(&empty),
        Some(payload_bytes(size_of::<Option<Vec<u8>>>()))
    );
    let fixed = [1_u64, 2_u64];
    assert_eq!(
        retained_footprint(&fixed),
        Some(payload_bytes(size_of::<[u64; 2]>()))
    );
}

#[test]
fn every_addition_and_capacity_multiplication_checks_overflow() {
    for maximum in [
        RetainedFootprint {
            payload_bytes: usize::MAX,
            ..RetainedFootprint::default()
        },
        RetainedFootprint {
            ordered_entries: usize::MAX,
            ..RetainedFootprint::default()
        },
        RetainedFootprint {
            shared_roots: usize::MAX,
            ..RetainedFootprint::default()
        },
    ] {
        let additional = RetainedFootprint {
            payload_bytes: usize::from(maximum.payload_bytes != 0),
            ordered_entries: usize::from(maximum.ordered_entries != 0),
            shared_roots: usize::from(maximum.shared_roots != 0),
        };
        assert_eq!(sum([Some(maximum), Some(additional)]), None);
    }
    assert_eq!(sum([Some(RetainedFootprint::default()), None]), None);
    assert_eq!(capacity_bytes::<u16>(usize::MAX), None);
    assert_eq!(capacity_bytes::<()>(usize::MAX), Some(payload_bytes(0)));
}

#[test]
fn owner_overflow_propagates_without_returning_an_underestimated_root() {
    struct Overflow;

    impl RetainedPayload for Overflow {
        /// Provides a synthetic impossible child bound for checked-root testing.
        ///
        /// # Returns
        /// Maximum bytes; adding the nonzero fixed owner must fail.
        fn retained_children(&self) -> Option<RetainedFootprint> {
            Some(payload_bytes(usize::MAX))
        }
    }

    // A nonzero inline owner proves fixed value size is also added with checking.
    let value = Some(Overflow);
    assert!(size_of::<Option<Overflow>>() > 0);
    assert_eq!(retained_footprint(&value), None);
}
