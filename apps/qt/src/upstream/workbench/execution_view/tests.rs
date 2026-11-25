/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual public worker/report projection plus explicitly constructed display cases.
use super::*;
use crate::shell::{ApplicationSessions, DispatchPhase};
use crate::upstream::workbench::tests::{
    configured, wait_for, PAUSE_FIXTURE_GRANTS as CONTROL_FIXTURE_GRANTS,
};
use std::sync::Arc;

const DOCUMENT: &[u8] = include_bytes!("../../../../../../examples/cli/upstream/us-50khz.json");

// Independent active-time expiration vector, not the application's default budget.
const TIMEOUT_FIXTURE_MS: u64 = 1;

fn row_value<'view>(view: &'view Value, label: &str) -> &'view str {
    view["verification"]["rows"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["label"] == label)
        .unwrap_or_else(|| panic!("Missing {label}: {view}"))["value"]
        .as_str()
        .unwrap()
}

#[test]
fn ber_notation_keeps_absence_zero_and_independent_literal_precision() {
    assert_eq!(ber_label(None), "Unavailable");
    assert_eq!(ber_label(Some(0.0)), "0");
    assert_eq!(ber_label(Some(0.000_531_4)), "5.314e-4");
}

#[test]
fn unconfigured_verification_has_no_fabricated_rows_or_recovery() {
    let model = Model::new(ApplicationSessions::desktop().unwrap()).unwrap();
    let state = model.state();
    assert_eq!(state["result"], Value::Null);
    assert!(state["execution_view"]["verification"]["rows"]
        .as_array()
        .unwrap()
        .is_empty());
    assert_eq!(
        state["configuration_view"]["blocks"]["verification"]["status"],
        "attention"
    );
    assert_eq!(state["execution_view"]["elapsed"], "");
}

#[test]
fn actual_completed_report_drives_exact_rows_and_previous_run_identity() {
    let application = ApplicationSessions::desktop().unwrap();
    let downstream = application.snapshot(crate::shell::Direction::Downstream);
    let mut model = configured(application.clone());
    model.start().unwrap();
    wait_for(|| model.poll()["busy"] == false);
    let (token, retained) = model.report.as_ref().unwrap();
    let retained = Arc::clone(retained);
    let report = retained.value();
    let verified = report.verification().unwrap();
    let fec = report.fec_summary().unwrap();
    let view = model.execution_presentation(None);
    assert_eq!(row_value(&view, "Run"), token.run.to_string());
    assert_eq!(row_value(&view, "Lifecycle"), "Completed");
    assert_eq!(
        row_value(&view, "PRBS15 source seed"),
        verified.source_seed.to_string()
    );
    assert!(row_value(&view, "Source").contains("Continuous PRBS15"));
    assert_eq!(
        row_value(&view, "Compared source bits"),
        verified.compared_source_bits.to_string()
    );
    assert_eq!(row_value(&view, "BER"), ber_label(verified.ber));
    assert_eq!(
        row_value(&view, "Completed / pending codewords"),
        format!("{} / {}", fec.completed_words, fec.pending_words)
    );
    assert_eq!(
        row_value(&view, "Full reference recovery"),
        if verified.reference_recovered {
            "Verified"
        } else {
            "Not verified"
        }
    );
    assert_eq!(
        model.state()["configuration_view"]["details"]["verification"],
        view["verification"]
    );
    assert!(Arc::ptr_eq(&retained, &model.report.as_ref().unwrap().1));
    assert_eq!(
        application.snapshot(crate::shell::Direction::Downstream),
        downstream
    );
    model.start().unwrap();
    let pending = model.execution_presentation(None);
    assert!(pending["verification"]["summary"]
        .as_str()
        .unwrap()
        .contains("Retained previous report"));
    assert_eq!(row_value(&pending, "Run"), row_value(&view, "Run"));
    assert_eq!(pending["verification_block"]["status"], "attention");
    model.stop().unwrap();
    wait_for(|| model.poll()["busy"] == false);
    assert!(Arc::ptr_eq(&retained, &model.report.as_ref().unwrap().1));
}

#[test]
fn real_pause_progress_and_cancelled_prefix_use_original_public_outcomes() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = Model::new(application.clone()).unwrap();
    let mut document: Value = serde_json::from_slice(DOCUMENT).unwrap();
    document["simulation"]["grants"] = CONTROL_FIXTURE_GRANTS.into();
    model
        .apply(&serde_json::to_vec(&document).unwrap())
        .unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    model.start().unwrap();
    wait_for(|| {
        model.poll();
        model.is_running()
    });
    model.pause().unwrap();
    wait_for(|| {
        model.poll();
        application
            .active()
            .is_some_and(|run| run.phase == DispatchPhase::Paused)
    });
    let progress = model.controller.as_ref().unwrap().progress().unwrap();
    let view = model.execution_presentation(Some(&progress));
    assert!(view["status"].as_str().unwrap().contains("Paused"));
    assert_eq!(
        view["elapsed"],
        format!("Active elapsed: {} ms", progress.1.elapsed_ms)
    );
    model.stop().unwrap();
    assert!(application.active().is_some());
    wait_for(|| model.poll()["busy"] == false);
    let report = model.report.as_ref().unwrap().1.value();
    assert_eq!(report.status, RunStatus::Cancelled);
    let view = model.execution_presentation(None);
    assert_eq!(row_value(&view, "Lifecycle"), "Cancelled");
    assert_eq!(
        row_value(&view, "Compared source bits"),
        report.compared_source_bits.to_string()
    );
    assert_eq!(
        row_value(&view, "Unverified generated bits"),
        report.unverified_source_bits.to_string()
    );
    assert_eq!(row_value(&view, "Full reference recovery"), "Not verified");
    assert_eq!(row_value(&view, "BER"), ber_label(report.ber));
    assert_eq!(view["verification_block"]["status"], "attention");
    assert!(application.active().is_none());
}

#[test]
fn actual_active_timeout_retains_its_time_limit_label_and_honest_coverage() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = Model::new(application.clone()).unwrap();
    let mut document: Value = serde_json::from_slice(DOCUMENT).unwrap();
    document["simulation"]["grants"] = CONTROL_FIXTURE_GRANTS.into();
    document["simulation"]["timeout_ms"] = TIMEOUT_FIXTURE_MS.into();
    model
        .apply(&serde_json::to_vec(&document).unwrap())
        .unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert!(model.current(), "{}", model.state());
    model.start().unwrap();
    wait_for(|| model.poll()["busy"] == false);
    let report = model.report.as_ref().unwrap().1.value();
    assert_eq!(report.status, RunStatus::TimeLimit);
    let view = model.execution_presentation(None);
    assert_eq!(row_value(&view, "Lifecycle"), "TimeLimit");
    assert_eq!(row_value(&view, "Full reference recovery"), "Not verified");
    assert_eq!(
        row_value(&view, "Not-started grants"),
        report.not_started_grants.len().to_string()
    );
    assert!(application.active().is_none());
}

#[test]
fn constructed_report_inconsistency_is_unavailable_without_fabricated_success() {
    let mut model = configured(ApplicationSessions::desktop().unwrap());
    model.start().unwrap();
    wait_for(|| model.poll()["busy"] == false);
    let mut fixture = model.report.as_ref().unwrap().1.value().clone();

    // These are representation fixtures, not scientific worker failure evidence.
    fixture.status = RunStatus::Failed;
    fixture.reference_recovered = false;
    let (failed, block) = report_presentation(&fixture, "fixture-run", false);
    assert_eq!(block["status"], "error");
    let view = json!({"verification":failed});
    assert_eq!(row_value(&view, "Lifecycle"), "Failed");
    assert_eq!(row_value(&view, "Full reference recovery"), "Not verified");
    fixture.bit_errors = fixture.compared_source_bits + 1;
    let (view, block) = report_presentation(&fixture, "fixture-run", false);
    assert!(view["rows"].as_array().unwrap().is_empty());
    assert_eq!(block["status"], "error");
}

#[test]
fn actual_current_report_before_terminal_is_never_labelled_as_an_older_run() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = configured(application.clone());
    model.start().unwrap();
    wait_for(|| {
        model.poll();
        model.report.is_some() && model.is_running()
    });
    let progress = model.controller.as_ref().unwrap().progress().unwrap();
    let (token, _) = model.report.as_ref().unwrap();
    assert_eq!(*token, progress.0);
    assert!(application.active().is_some());
    let view = model.execution_presentation(Some(&progress));
    assert_eq!(row_value(&view, "Run"), token.run.to_string());
    assert!(!view["verification"]["summary"]
        .as_str()
        .unwrap()
        .contains("previous"));
    wait_for(|| model.poll()["busy"] == false);
    assert!(application.active().is_none());
}
