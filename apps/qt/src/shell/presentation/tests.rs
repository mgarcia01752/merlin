/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shell metadata tests never replace scientific readiness or lifecycle evidence.
use super::*;
use crate::shell::RunCancellation;
use std::sync::Arc;

struct Control;
impl RunCancellation for Control {
    /// Supplies a nonblocking fixture cancellation.
    ///
    /// Does not start an engine, acknowledge termination or release the lease.
    fn request_cancel(&self) {}
}

#[test]
fn unconfigured_directions_keep_distinct_canonical_text_identities() {
    let application = ApplicationSessions::desktop().unwrap();
    let before = application.budget().usage();
    let snapshot = state(&application);
    assert_eq!(snapshot["selectedDirection"], "downstream");
    assert_eq!(snapshot["applicationBusy"], false);
    assert_eq!(snapshot["executionDirection"], Value::Null);
    assert_eq!(snapshot["executionStatus"], "");
    for direction in [Direction::Downstream, Direction::Upstream] {
        let expected = identity(application.snapshot(direction).key);
        assert!(!expected.is_empty());
        assert!(expected.starts_with(direction_tag(direction)));
    }
    assert_ne!(snapshot["downstreamIdentity"], snapshot["upstreamIdentity"]);
    assert_eq!(snapshot["downstreamDirty"], false);
    assert_eq!(snapshot["upstreamDirty"], false);
    assert_eq!(application.budget().usage(), before);
}

#[test]
fn selection_and_scalar_projection_preserve_running_paused_and_cancelling_lease() {
    let application = ApplicationSessions::desktop().unwrap();
    let downstream = application.configure_downstream(true).unwrap();
    let upstream = application.snapshot(Direction::Upstream);
    let lease = application.begin(downstream, Arc::new(Control)).unwrap();
    let token = lease.token();
    application.select_direction(Direction::Upstream);
    let running = state(&application);
    assert_eq!(running["selectedDirection"], "upstream");
    assert_eq!(running["executionDirection"], "downstream");
    assert_eq!(running["executionStatus"], "Downstream simulation running");
    assert_eq!(running["applicationBusy"], true);
    application.acknowledge_pause(token, true).unwrap();
    assert_eq!(
        state(&application)["executionStatus"],
        "Downstream simulation paused"
    );
    application.select_direction(Direction::Downstream);
    assert_eq!(application.active().unwrap().token, token);
    application.cancel(token).unwrap();
    application.select_direction(Direction::Upstream);
    assert_eq!(
        state(&application)["executionStatus"],
        "Downstream simulation cancelling"
    );
    assert_eq!(application.snapshot(Direction::Upstream), upstream);
    assert_eq!(application.snapshot(Direction::Downstream).key, downstream);
    application.finish(lease).unwrap();
    assert_eq!(state(&application)["applicationBusy"], false);
}

#[test]
fn configured_identity_and_dirty_metadata_change_only_for_their_real_direction() {
    let application = ApplicationSessions::desktop().unwrap();
    let old = state(&application);
    application.configure_upstream(true).unwrap();
    let current = state(&application);
    assert_eq!(current["downstreamIdentity"], old["downstreamIdentity"]);
    assert_eq!(current["downstreamDirty"], false);
    assert_ne!(current["upstreamIdentity"], old["upstreamIdentity"]);
    assert_eq!(current["upstreamDirty"], true);
    assert_eq!(current["selectedDirection"], "downstream");
}
