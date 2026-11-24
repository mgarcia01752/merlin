/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real export transport rejection preserves Ready before picker or dispatch.
use super::*;
use crate::{
    shared::presentation::constants::WORKER_TRANSPORT_ADMISSION_MESSAGE, shell::RetainedResources,
};

#[test]
fn channel_and_slot_exhaustion_preserve_the_same_ready_without_picker_or_dispatch() {
    let (mut owner, _) = admission_tests::prepared_owner();
    let budget = owner.context.application.budget().clone();
    let original = Arc::clone(admission_tests::ready(&owner));
    let baseline = budget.usage();
    for pressure in [
        RetainedResources {
            channel_count: usize::MAX - baseline.channel_count,
            ..Default::default()
        },
        RetainedResources {
            slot_count: usize::MAX - baseline.slot_count,
            ..Default::default()
        },
    ] {
        let guard = budget.reserve_resources(pressure).unwrap();
        let full = budget.usage();
        let failure = lifecycle::save_with_picker(
            &mut owner,
            ExportFormat::Json,
            |_| panic!("Rejected transport must not dispatch"),
            |_, _| panic!("Rejected transport must not choose a destination"),
        )
        .unwrap_err();
        assert_eq!(failure, WORKER_TRANSPORT_ADMISSION_MESSAGE);
        assert!(Arc::ptr_eq(admission_tests::ready(&owner), &original));
        assert_eq!(budget.usage(), full);
        drop(guard);
        assert_eq!(budget.usage(), baseline);
    }
    drop(owner);
    drop(original);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}
