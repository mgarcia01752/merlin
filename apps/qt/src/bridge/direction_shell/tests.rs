/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual adapter owner validation before direction-specific control dispatch.
use super::*;

#[test]
fn current_selected_actions_reject_hidden_foreign_and_stale_identity_without_mutation() {
    let application = ApplicationSessions::desktop().unwrap();
    let owner = DirectionShellRust {
        application: application.clone(),
        close: None,
    };
    let downstream = QString::from("downstream");
    let upstream = QString::from("upstream");
    let old = QString::from(presentation::identity(
        application.snapshot(Direction::Downstream).key,
    ));
    let other = QString::from(presentation::identity(
        application.snapshot(Direction::Upstream).key,
    ));
    let before = application.budget().usage();
    assert!(owner.accepts(&downstream, &old));
    assert!(!owner.accepts(&upstream, &other));
    assert!(!owner.accepts(&downstream, &other));
    assert!(owner.select(&upstream));
    assert!(!owner.accepts(&downstream, &old));
    assert!(owner.accepts(&upstream, &other));
    assert!(owner.select(&downstream));
    application.configure_downstream(true).unwrap();
    assert!(!owner.accepts(&downstream, &old));
    assert!(owner.accepts(
        &downstream,
        &QString::from(presentation::identity(
            application.snapshot(Direction::Downstream).key
        ))
    ));
    let foreign = ApplicationSessions::desktop().unwrap();
    assert!(!owner.accepts(
        &downstream,
        &QString::from(presentation::identity(
            foreign.snapshot(Direction::Downstream).key
        ))
    ));
    assert_eq!(application.budget().usage(), before);
    assert!(application.active().is_none());
}

#[test]
fn unknown_direction_and_empty_identity_leave_canonical_selection_and_owners_unchanged() {
    let application = ApplicationSessions::desktop().unwrap();
    let owner = DirectionShellRust {
        application: application.clone(),
        close: None,
    };
    let downstream = application.snapshot(Direction::Downstream);
    let upstream = application.snapshot(Direction::Upstream);
    for invalid in ["", "Downstream", "upstream ", "other"] {
        let value = QString::from(invalid);
        assert!(!owner.select(&value));
        assert!(!owner.accepts(&value, &QString::from("")));
    }
    assert!(!owner.accepts(&QString::from("downstream"), &QString::from("")));
    assert_eq!(application.selected_direction(), Direction::Downstream);
    assert_eq!(application.snapshot(Direction::Downstream), downstream);
    assert_eq!(application.snapshot(Direction::Upstream), upstream);
}
