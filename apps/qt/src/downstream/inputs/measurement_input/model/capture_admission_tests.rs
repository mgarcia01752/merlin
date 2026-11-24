/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Generated capture ownership fixtures; actual bridge/worker admission is separate.
use super::*;
use crate::{
    downstream::inputs::measurement_input::designer,
    shell::{Retained, RetainedResources, RetentionBudget, RetentionReservation},
};
use merlin::downstream::{inputs::CaptureLoader, NamedCapture, RxMerFrame};
use std::{
    mem::size_of,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
};

// Genuine single-capture generation keeps the existing imported baseline and
// default recipe. These are fixture dimensions, not new Designer limits.
const FIXTURE_CAPTURE_COUNT: usize = 1;

// Independent spare capacity anchors, unrelated to science or production policy.
const EXTRA_CAPTURE_SLOTS: usize = 13;
const EXTRA_ORIGINAL_BYTES: usize = 41;

// Every generated fixture Capture has original, typed-frame and Designer Origin Arcs.
const CAPTURE_SHARED_OWNER_EDGES: usize = 3;

// Independent host policy for source-only accounting fixtures.
const FIXTURE_LIMIT: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
};

#[derive(Clone, Copy)]
enum Dimension {
    Payload,
    Entries,
    Roots,
}

fn fixture() -> Vec<Capture> {
    let input = NamedCapture::new(
        "profile.json",
        include_bytes!(
            "../../../../../../../examples/measurements/single-channel/modulation_profile.json"
        )
        .to_vec(),
    )
    .unwrap();
    let catalog = CaptureLoader::profile_capture(&input).unwrap();
    let profile = Arc::new(catalog.frame(catalog.profile_ids()[0]).unwrap().clone());
    let source = Capture::decode(
        "original-rxmer.json".into(),
        include_bytes!("../../../../../../../examples/measurements/single-channel/rxmer.json")
            .to_vec(),
        true,
    )
    .unwrap();
    let session = designer::Session::begin(&[source], profile, catalog).unwrap();
    let completed = AtomicUsize::new(0);
    let captures = session
        .build(&session.request, &AtomicBool::new(false), &completed)
        .unwrap();
    assert_eq!(captures.len(), FIXTURE_CAPTURE_COUNT);
    assert_eq!(completed.load(Ordering::Relaxed), FIXTURE_CAPTURE_COUNT);
    assert!(captures[0].designer.is_some());
    captures
}

fn resources(captures: &Vec<Capture>) -> RetainedResources {
    let mut resources = retention::capture_batch_resources(captures).unwrap();
    let actual_layout = size_of::<Retained<Vec<Capture>>>()
        .max(size_of::<(Vec<Capture>, Option<RetentionReservation>)>());
    resources.payload_bytes = resources
        .payload_bytes
        .checked_sub(size_of::<Vec<Capture>>())
        .unwrap()
        .checked_add(actual_layout)
        .unwrap();
    resources
}

fn plus(left: RetainedResources, right: RetainedResources) -> RetainedResources {
    RetainedResources {
        payload_bytes: left.payload_bytes.checked_add(right.payload_bytes).unwrap(),
        ordered_entries: left
            .ordered_entries
            .checked_add(right.ordered_entries)
            .unwrap(),
        shared_roots: left.shared_roots.checked_add(right.shared_roots).unwrap(),
        ..Default::default()
    }
}

#[test]
fn generated_batch_counts_fixed_root_once_and_actual_slot_and_original_capacity_passively() {
    let mut captures = fixture();
    let capture = &captures[0];
    let Measurement::Rxmer(frame) = &capture.measurement else {
        panic!("Actual generated RxMER fixture");
    };
    let origin = designer::origin_resources(capture.designer.as_ref().unwrap()).unwrap();
    let mut expected = RetainedResources {
        payload_bytes: size_of::<Vec<Capture>>()
            + captures.capacity() * size_of::<Capture>()
            + capture.name.capacity()
            + capture.hash.capacity()
            + capture.decoded_hash.capacity()
            + size_of::<Vec<u8>>()
            + capture.original.capacity()
            + size_of::<RxMerFrame>()
            + frame.mer_db.capacity() * size_of::<Option<f64>>()
            + frame.source.name.capacity()
            + frame.source.sha256.capacity()
            + frame.source.file_kind.capacity()
            + origin.payload_bytes,
        ordered_entries: origin.ordered_entries,
        shared_roots: origin.shared_roots + CAPTURE_SHARED_OWNER_EDGES,
        ..Default::default()
    };
    let original = capture.original.as_ref().clone();
    let hash = capture.hash.clone();
    let frame_values = serde_json::to_value(frame.as_ref()).unwrap();
    let frame_pointer = Arc::as_ptr(frame);
    let origin_pointer = Arc::as_ptr(capture.designer.as_ref().unwrap());
    assert_eq!(
        retention::capture_batch_resources(&captures).unwrap(),
        expected
    );
    assert_eq!(
        retention::capture_batch_resources(&captures).unwrap(),
        expected
    );

    let old = captures.capacity();
    let mut slots = Vec::with_capacity(old + EXTRA_CAPTURE_SLOTS);
    slots.append(&mut captures);
    expected.payload_bytes += (slots.capacity() - old) * size_of::<Capture>();
    captures = slots;
    assert_eq!(
        retention::capture_batch_resources(&captures).unwrap(),
        expected
    );
    let bytes =
        Arc::get_mut(&mut captures[0].original).expect("Only the fixture owns this byte Arc");
    let old = bytes.capacity();
    bytes.reserve_exact(old + EXTRA_ORIGINAL_BYTES);
    expected.payload_bytes += bytes.capacity() - old;
    assert_eq!(
        retention::capture_batch_resources(&captures).unwrap(),
        expected
    );
    assert_eq!(captures[0].original.as_ref(), &original);
    assert_eq!(captures[0].hash, hash);
    let Measurement::Rxmer(frame) = &captures[0].measurement else {
        panic!("Unchanged generated RxMER fixture");
    };
    assert_eq!(serde_json::to_value(frame.as_ref()).unwrap(), frame_values);
    assert_eq!(Arc::as_ptr(frame), frame_pointer);
    assert_eq!(
        Arc::as_ptr(captures[0].designer.as_ref().unwrap()),
        origin_pointer
    );
}

#[test]
fn admitted_batch_aliases_and_split_guard_cover_old_current_overlap_until_final_disposal() {
    fn assert_send<T: Send>() {}
    assert_send::<Retained<Vec<Capture>>>();
    let budget = RetentionBudget::with_limits(FIXTURE_LIMIT);
    let old = Model::admit_capture_batch(fixture(), &budget).unwrap();
    let old_charge = resources(old.value());
    assert_eq!(old.fixture_reserved_resources(), old_charge);
    assert_eq!(budget.usage(), old_charge);
    let original = old.value()[0].original.as_ref().clone();
    let original_lifetime = Arc::downgrade(&old.value()[0].original);
    let origin_lifetime = Arc::downgrade(old.value()[0].designer.as_ref().unwrap());
    let Measurement::Rxmer(frame) = &old.value()[0].measurement else {
        panic!("Actual generated RxMER fixture");
    };
    let frame_lifetime = Arc::downgrade(frame);
    let candidate = Model::admit_capture_batch(old.value().clone(), &budget).unwrap();
    let candidate_charge = resources(candidate.value());
    assert_eq!(candidate.fixture_reserved_resources(), candidate_charge);
    assert_eq!(budget.usage(), plus(old_charge, candidate_charge));
    assert!(Arc::ptr_eq(
        &old.value()[0].original,
        &candidate.value()[0].original
    ));
    assert!(Arc::ptr_eq(
        old.value()[0].designer.as_ref().unwrap(),
        candidate.value()[0].designer.as_ref().unwrap(),
    ));
    let (captures, guard) = candidate.into_parts();
    assert_eq!(budget.usage(), plus(old_charge, candidate_charge));
    drop(old);
    assert_eq!(budget.usage(), candidate_charge);
    assert_eq!(captures[0].original.as_ref(), &original);
    assert!(original_lifetime.upgrade().is_some());
    assert!(origin_lifetime.upgrade().is_some());
    assert!(frame_lifetime.upgrade().is_some());
    drop(captures);
    assert!(original_lifetime.upgrade().is_none());
    assert!(origin_lifetime.upgrade().is_none());
    assert!(frame_lifetime.upgrade().is_none());
    assert_eq!(budget.usage(), candidate_charge);
    drop(guard);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn every_positive_candidate_dimension_rejects_without_losing_old_generated_bytes_or_aliases() {
    let budget = RetentionBudget::with_limits(FIXTURE_LIMIT);
    let old = Model::admit_capture_batch(fixture(), &budget).unwrap();
    let accepted = budget.usage();
    assert!(
        accepted.payload_bytes > 0 && accepted.ordered_entries > 0 && accepted.shared_roots > 0
    );
    let metadata = retention::capture_batch_resources(old.value()).unwrap();
    let original = old.value()[0].original.as_ref().clone();
    let hash = old.value()[0].hash.clone();
    for dimension in [Dimension::Payload, Dimension::Entries, Dimension::Roots] {
        // Exhaust the actual remaining host quantity, without inventing a
        // scientific allocation allowance or assuming Clone capacities equal.
        let pressure = match dimension {
            Dimension::Payload => RetainedResources {
                payload_bytes: usize::MAX.checked_sub(accepted.payload_bytes).unwrap(),
                ..Default::default()
            },
            Dimension::Entries => RetainedResources {
                ordered_entries: usize::MAX.checked_sub(accepted.ordered_entries).unwrap(),
                ..Default::default()
            },
            Dimension::Roots => RetainedResources {
                shared_roots: usize::MAX.checked_sub(accepted.shared_roots).unwrap(),
                ..Default::default()
            },
        };
        let guard = budget.reserve_resources(pressure).unwrap();
        let full = budget.usage();
        let original_aliases = Arc::strong_count(&old.value()[0].original);
        let origin_aliases = Arc::strong_count(old.value()[0].designer.as_ref().unwrap());
        assert!(Model::admit_capture_batch(old.value().clone(), &budget).is_err());
        assert_eq!(budget.usage(), full);
        assert_eq!(
            Arc::strong_count(&old.value()[0].original),
            original_aliases
        );
        assert_eq!(
            Arc::strong_count(old.value()[0].designer.as_ref().unwrap()),
            origin_aliases
        );
        assert_eq!(
            retention::capture_batch_resources(old.value()).unwrap(),
            metadata
        );
        assert_eq!(old.value()[0].original.as_ref(), &original);
        assert_eq!(old.value()[0].hash, hash);
        drop(guard);
        assert_eq!(budget.usage(), accepted);
    }
    drop(old);
    assert_eq!(budget.usage(), RetainedResources::default());
}
