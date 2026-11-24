/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual batch queue and editor handoff, separate from finite builder scratch.
use super::*;
use crate::{
    bridge::measurement_input::MeasurementInputsRust,
    downstream::workbench::session::Context,
    shell::{ApplicationSessions, RetainedResources, RetentionBudget},
};

// Counter-only fixture ceilings, independent of desktop/scientific limits.
const TEST_LIMIT: RetainedResources = RetainedResources {
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

fn owner() -> (Session, MeasurementInputsRust, RetainedResources) {
    let (session, model) = super::tests::fixture();
    let mut context = Context::isolated();
    context.application = ApplicationSessions::new(TEST_LIMIT).unwrap();
    let mut owner = MeasurementInputsRust {
        context,
        ..Default::default()
    };
    let infrastructure = owner.context.application.budget().usage();
    owner.replace_model(model).unwrap();
    (session, owner, infrastructure)
}

fn batch(session: &Session, budget: &RetentionBudget) -> Retained<Vec<Capture>> {
    let captures = session
        .build(
            &session.request,
            &AtomicBool::new(false),
            &AtomicUsize::new(0),
        )
        .unwrap();
    Model::admit_capture_batch(captures, budget).unwrap()
}

#[test]
fn actual_queued_batch_is_charged_until_cancel_or_stale_completion_disposes_it() {
    let (session, mut owner, _) = owner();
    let budget = owner.context.application.budget().clone();
    let baseline = budget.usage();
    for cancelled in [true, false] {
        let captures = batch(&session, &budget);
        let original = Arc::downgrade(&captures.value()[0].original);
        let expected = budget.usage();
        let (sender, receiver) = mpsc::sync_channel(DESIGNER_CAPTURE_QUEUE_CAPACITY);
        let mut work = super::tests::work(&session, Options::default());
        work.receiver = receiver;
        work.cancel.store(cancelled, Ordering::Relaxed);
        if !cancelled {
            work.profile_hash.clear();
        }
        sender.send(Ok(captures)).unwrap();
        drop(sender);
        assert_eq!(budget.usage(), expected);
        assert!(original.upgrade().is_some());
        let result = work.receiver.try_recv().unwrap();
        let result = adopt_to_owner(&mut owner, work, result, Some(&session.profile));
        if cancelled {
            assert_eq!(result.unwrap()["cancelled"], true);
        } else {
            assert!(result.is_err());
        }
        assert_eq!(owner.model.draft_rx[0].hash, session.hashes[0]);
        assert!(original.upgrade().is_none());
        assert_eq!(budget.usage(), baseline);
    }
    drop(owner);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn staged_source_guard_survives_target_admission_and_rejection_in_every_dimension() {
    let (session, mut owner, infrastructure) = owner();
    let budget = owner.context.application.budget().clone();
    let baseline = budget.usage();
    let old_original = Arc::clone(&owner.model.draft_rx[0].original);
    // Measure the exact candidate on an independent fixture counter. Clone
    // capacities are measured again by the real target admission below.
    let measuring = RetentionBudget::with_limits(TEST_LIMIT);
    let mut model = owner.model.clone();
    let staged = adopt(
        super::tests::work(&session, Options::default()),
        Ok(batch(&session, &measuring)),
        &mut model,
        Some(&session.profile),
    )
    .unwrap();
    let target = model.admit(&measuring).unwrap();
    let required = target.fixture_reserved_resources();
    drop(target);
    drop(staged);
    assert_eq!(measuring.usage(), RetainedResources::default());

    for dimension in [Dimension::Payload, Dimension::Entries, Dimension::Roots] {
        let captures = batch(&session, &budget);
        let batch_charge = captures.fixture_reserved_resources();
        assert!(batch_charge.payload_bytes > 0);
        assert!(batch_charge.ordered_entries > 0);
        assert!(batch_charge.shared_roots > 0);
        let original = Arc::downgrade(&captures.value()[0].original);
        // Leave precisely enough room for the target without its still-live
        // source batch. Losing the source guard early would admit this edit.
        let pressure = match dimension {
            Dimension::Payload => RetainedResources {
                payload_bytes: usize::MAX - baseline.payload_bytes - required.payload_bytes,
                ..Default::default()
            },
            Dimension::Entries => RetainedResources {
                ordered_entries: usize::MAX - baseline.ordered_entries - required.ordered_entries,
                ..Default::default()
            },
            Dimension::Roots => RetainedResources {
                shared_roots: usize::MAX - baseline.shared_roots - required.shared_roots,
                ..Default::default()
            },
        };
        let guard = budget.reserve_resources(pressure).unwrap();
        assert!(adopt_to_owner(
            &mut owner,
            super::tests::work(&session, Options::default()),
            Ok(captures),
            Some(&session.profile),
        )
        .is_err());
        assert!(Arc::ptr_eq(
            &owner.model.draft_rx[0].original,
            &old_original
        ));
        assert!(original.upgrade().is_none());
        drop(guard);
        assert_eq!(budget.usage(), baseline);
    }
    let captures = batch(&session, &budget);
    let original = Arc::downgrade(&captures.value()[0].original);
    let expected = captures.value()[0].hash.clone();
    let result = adopt_to_owner(
        &mut owner,
        super::tests::work(&session, Options::default()),
        Ok(captures),
        Some(&session.profile),
    )
    .unwrap();
    assert_eq!(result["staged"], true);
    assert_eq!(owner.model.draft_rx[0].hash, expected);
    assert!(original.upgrade().is_some());
    assert_eq!(
        budget.usage(),
        RetainedResources {
            payload_bytes: infrastructure.payload_bytes + required.payload_bytes,
            ordered_entries: infrastructure.ordered_entries + required.ordered_entries,
            shared_roots: infrastructure.shared_roots + required.shared_roots,
        }
    );
    drop(owner);
    assert!(original.upgrade().is_none());
    assert_eq!(budget.usage(), RetainedResources::default());
}
