/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Target-preparation transport rejection returns the original scientific owner.
use super::*;
use crate::shell::{ApplicationSessions, Direction, RetainedResources};
use merlin::upstream::Simulation;

const DOCUMENT: &[u8] = include_bytes!("../../../../../../examples/cli/upstream/us-50khz.json");
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
};

#[test]
fn channel_and_slot_rejection_return_same_idle_controller_without_scientific_preflight() {
    let application = ApplicationSessions::new(LIMITS).unwrap();
    let budget = application.budget();
    let source = PreparedSession::from_document(
        ConfigurationDocument::from_json(DOCUMENT).unwrap(),
        ReceiverObservationPolicy::new(0, 0).unwrap(),
        budget,
    )
    .unwrap(); // Existing fixture with receiver history disabled.
    let mut controller =
        UpstreamController::new(application.snapshot(Direction::Upstream).key, source).unwrap();
    let original = controller
        .inspect(|session| session as *const Simulation)
        .unwrap();
    for channels in [true, false] {
        let used = budget.usage();
        let pressure = budget
            .reserve_resources(if channels {
                RetainedResources {
                    channel_count: LIMITS.channel_count - used.channel_count,
                    ..Default::default()
                }
            } else {
                RetainedResources {
                    slot_count: LIMITS.slot_count - used.slot_count,
                    ..Default::default()
                }
            })
            .unwrap();
        let Err((returned, error)) = Pending::run(controller, budget.clone()) else {
            panic!("Actual channel must reject before preparation dispatch")
        };
        assert!(!error.is_empty());
        assert_eq!(
            returned
                .inspect(|session| session as *const Simulation)
                .unwrap(),
            original
        );
        assert!(returned.report().is_none());
        assert!(application.active().is_none());
        controller = returned;
        drop(pressure);
    }
}
