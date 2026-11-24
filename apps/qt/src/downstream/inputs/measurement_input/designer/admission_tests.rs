/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Actual Session admission and worker-copy preparation; runtime acceptance is separate.
use super::*;
use crate::shell::RetentionReservation;
use merlin::downstream::inputs::CaptureLoader;
use std::mem::size_of;

// Independent requested spare slots distinguish source storage from actual
// clone storage without assuming that Clone preserves or shrinks capacities.
const EXTRA_HASH_SLOTS: usize = 13;

// Independent fixture policy, not a production application or scientific limit.
const FIXTURE_LIMIT: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};

#[derive(Clone, Copy)]
enum Dimension {
    Payload,
    Entries,
    Roots,
}

/// Prepares a genuine single-capture imported Session without scientific edits.
///
/// # Returns
/// Existing public profile/catalog and original RxMER fixture validated by Session::begin.
/// The result is temporary fixture preparation until the caller admits it.
///
/// # Panics
/// Panics if a checked existing fixture no longer loads or associates.
pub(super) fn fixture() -> Session {
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
    let capture = Capture::decode(
        "rxmer.json".into(),
        include_bytes!("../../../../../../../examples/measurements/single-channel/rxmer.json")
            .to_vec(),
        true,
    )
    .unwrap();
    Session::begin(&[capture], profile, catalog).unwrap()
}

fn resources(session: &Session) -> RetainedResources {
    let mut resources = session.retained_resources().unwrap();
    let actual_layout =
        size_of::<Retained<Session>>().max(size_of::<(Session, Option<RetentionReservation>)>());
    resources.payload_bytes = resources
        .payload_bytes
        .checked_sub(size_of::<Session>())
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
    }
}

fn originals(session: &Session) -> Vec<Vec<u8>> {
    session
        .origins
        .iter()
        .map(|origin| origin.design.original_bytes().to_vec())
        .collect()
}

#[test]
fn actual_clone_overlap_and_old_profile_alias_keep_each_session_charge_until_final_drop() {
    fn assert_send<T: Send>() {}
    assert_send::<Retained<Session>>();
    let mut session = fixture();
    session.hashes.reserve_exact(EXTRA_HASH_SLOTS);
    let original_bytes = originals(&session);
    let original_request = serde_json::to_value(&session.request).unwrap();
    let budget = RetentionBudget::with_limits(FIXTURE_LIMIT);
    let source = session.admit(&budget).unwrap();
    let source_charge = resources(source.value());
    assert_eq!(source.fixture_reserved_resources(), source_charge);
    assert_eq!(budget.usage(), source_charge);
    let profile_lifetime = Arc::downgrade(&source.value().profile);

    let current = source.value().clone_admitted(&budget).unwrap();
    let current_charge = resources(current.value());
    assert_eq!(current.fixture_reserved_resources(), current_charge);
    assert_eq!(budget.usage(), plus(source_charge, current_charge));
    assert!(Arc::ptr_eq(
        &source.value().profile,
        &current.value().profile
    ));
    assert_eq!(current.value().hashes, source.value().hashes);
    assert_eq!(originals(current.value()), original_bytes);
    assert_eq!(
        serde_json::to_value(&current.value().request).unwrap(),
        original_request
    );

    // Moving the former current Session to an old owner transfers its exact
    // charge; its shared profile alias survives disposal of the new copy.
    let old = source;
    drop(current);
    assert_eq!(budget.usage(), source_charge);
    assert!(profile_lifetime.upgrade().is_some());
    assert_eq!(originals(old.value()), original_bytes);
    assert_eq!(
        serde_json::to_value(&old.value().request).unwrap(),
        original_request
    );
    drop(old);
    assert_eq!(budget.usage(), RetainedResources::default());
    assert!(profile_lifetime.upgrade().is_none());
}

#[test]
fn every_positive_clone_dimension_rejects_without_losing_source_bytes_or_ownership() {
    let budget = RetentionBudget::with_limits(FIXTURE_LIMIT);
    let source = fixture().admit(&budget).unwrap();
    let accepted = budget.usage();
    assert!(
        accepted.payload_bytes > 0 && accepted.ordered_entries > 0 && accepted.shared_roots > 0
    );
    let metadata = source.value().retained_resources().unwrap();
    let original_bytes = originals(source.value());
    let original_request = serde_json::to_value(&source.value().request).unwrap();
    let hashes = source.value().hashes.clone();
    for dimension in [Dimension::Payload, Dimension::Entries, Dimension::Roots] {
        // Exhaust the actual remaining host quantity, with no estimated
        // scientific or wrapper allowance. Each candidate has positive demand.
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
        let reservation = budget.reserve_resources(pressure).unwrap();
        let full = budget.usage();
        let profile_aliases = Arc::strong_count(&source.value().profile);
        assert!(source.value().clone_admitted(&budget).is_err());
        assert_eq!(budget.usage(), full);
        assert_eq!(Arc::strong_count(&source.value().profile), profile_aliases);
        assert_eq!(source.value().retained_resources().unwrap(), metadata);
        assert_eq!(source.value().hashes, hashes);
        assert_eq!(originals(source.value()), original_bytes);
        assert_eq!(
            serde_json::to_value(&source.value().request).unwrap(),
            original_request
        );
        drop(reservation);
        assert_eq!(budget.usage(), accepted);
    }
    drop(source);
    assert_eq!(budget.usage(), RetainedResources::default());
}
