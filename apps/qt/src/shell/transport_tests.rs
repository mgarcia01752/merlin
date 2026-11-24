/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Exact application transport layouts; test synchronization is fixture-only.
use super::*;
use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    sync::Mutex,
};

const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
};

// Independent two-slot channel anchor, unrelated to desktop population policies.
const FIXTURE_SLOTS: usize = 2;

fn channel_resources() -> RetainedResources {
    RetainedResources {
        payload_bytes: size_of::<Retained<()>>(),
        shared_roots: 1,  // One actual shared guard allocation.
        channel_count: 1, // One actual constructed channel.
        slot_count: FIXTURE_SLOTS,
        ..Default::default()
    }
}

#[test]
fn channel_layout_exact_and_each_below_limit_are_atomic() {
    let expected = channel_resources();
    let budget = RetentionBudget::with_limits(expected);
    let (sender, receiver) = bounded_channel::<u8>(&budget, FIXTURE_SLOTS).unwrap();
    assert_eq!(budget.usage(), expected);
    drop(sender);
    assert_eq!(budget.usage(), expected);
    drop(receiver);
    assert_eq!(budget.usage(), RetainedResources::default());
    for maximum in [
        RetainedResources {
            payload_bytes: expected.payload_bytes - 1,
            ..expected
        },
        RetainedResources {
            shared_roots: 0,
            ..expected
        },
        RetainedResources {
            channel_count: 0,
            ..expected
        },
        RetainedResources {
            slot_count: FIXTURE_SLOTS - 1,
            ..expected
        },
    ] {
        let budget = RetentionBudget::with_limits(maximum);
        assert!(bounded_channel::<u8>(&budget, FIXTURE_SLOTS).is_err());
        assert_eq!(budget.usage(), RetainedResources::default());
    }
    let overflow = RetainedResources {
        channel_count: usize::MAX,
        ..Default::default()
    };
    assert_eq!(
        overflow.checked_add(RetainedResources {
            channel_count: 1,
            ..Default::default()
        }),
        Err(RetentionError::ArithmeticOverflow)
    );
    let overflow = RetainedResources {
        slot_count: usize::MAX,
        ..Default::default()
    };
    assert_eq!(
        overflow.checked_add(RetainedResources {
            slot_count: 1,
            ..Default::default()
        }),
        Err(RetentionError::ArithmeticOverflow)
    );
}

#[test]
fn independently_received_frames_outlive_channel_population_without_capacity_guess() {
    let budget = RetentionBudget::with_limits(LIMITS);
    let (sender, receiver) = bounded_channel::<u8>(&budget, FIXTURE_SLOTS).unwrap();
    let mut held = Vec::new();

    // Three independent values exceed channel capacity while each remains held.
    for value in [1, 2, 3] {
        sender.reserve().unwrap().send(value).unwrap();
        held.push(receiver.recv().unwrap());
    }
    let frame_bytes = size_of::<Retained<u8>>();
    assert_eq!(
        budget.used().0,
        channel_resources().payload_bytes + held.len() * frame_bytes
    );
    drop(sender);
    drop(receiver);
    assert_eq!(
        budget.usage(),
        RetainedResources {
            payload_bytes: held.len() * frame_bytes,
            ..Default::default()
        }
    );
    assert_eq!(
        held.iter().map(|frame| *frame.value()).collect::<Vec<_>>(),
        [1, 2, 3]
    );
    held.pop();
    assert_eq!(budget.used().0, held.len() * frame_bytes);
    drop(held);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn rendezvous_permit_keeps_channel_and_disconnect_returns_original_payload() {
    let budget = RetentionBudget::with_limits(LIMITS);
    let (sender, receiver) = bounded_channel::<u8>(&budget, 0).unwrap();
    let permit = sender.reserve().unwrap();
    drop(sender);
    assert_eq!(budget.usage().channel_count, 1);
    assert_eq!(budget.usage().slot_count, 0);
    drop(receiver);
    assert_eq!(budget.usage().channel_count, 1);
    assert_eq!(permit.send(7), Err(7)); // Original disconnected fixture byte.
    assert_eq!(budget.usage(), RetainedResources::default());
}

struct DropProbe {
    budget: RetentionBudget,
    observed: Arc<Mutex<Option<RetainedResources>>>,
}

impl Drop for DropProbe {
    /// Observes accounting while actual captured data is being destroyed.
    ///
    /// The task reservation must still be live for return, unwind and discard.
    fn drop(&mut self) {
        *self.observed.lock().unwrap() = Some(self.budget.usage());
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum TaskMode {
    Return,
    Unwind,
    Discard,
}

#[test]
fn actual_task_size_and_capture_drop_precede_guard_for_return_unwind_and_discard() {
    for mode in [TaskMode::Return, TaskMode::Unwind, TaskMode::Discard] {
        let budget = RetentionBudget::with_limits(LIMITS);
        let observed = Arc::new(Mutex::new(None));
        let probe = DropProbe {
            budget: budget.clone(),
            observed: Arc::clone(&observed),
        };
        let operation = move || {
            let _probe = probe;
            if mode == TaskMode::Unwind {
                panic!("Intentional task unwind fixture");
            }
        };
        let expected = RetainedResources {
            payload_bytes: size_of_val(&operation) + size_of::<AdmittedTask>(),
            ..Default::default()
        };
        let task = admit_task(&budget, operation).unwrap();
        assert_eq!(budget.usage(), expected);
        if mode == TaskMode::Discard {
            drop(task);
        } else {
            assert_eq!(
                catch_unwind(AssertUnwindSafe(|| run_task(task))).is_err(),
                mode == TaskMode::Unwind
            );
        }
        assert_eq!(*observed.lock().unwrap(), Some(expected));
        assert_eq!(budget.usage(), RetainedResources::default());
    }
}
