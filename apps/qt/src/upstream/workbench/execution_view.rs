/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Finite presentation of original public progress, verification and FEC summaries.
use super::{error_text, Model};
use crate::shared::presentation::constants::BER_MANTISSA_DECIMAL_PLACES;
use crate::shell::RunToken;
use merlin::upstream::{BatchReport, ProgressSnapshot, RunStatus};
use serde_json::{json, Value};

impl Model {
    /// Projects actual execution scalars independently of idle configuration access.
    /// # Arguments
    /// * `progress` - Current original run token and public snapshot, absent outside execution.
    /// # Returns
    /// Transient display strings and bounded verification rows. Existing QML owners
    /// must admit persistent copies. Previous reports retain their original run label;
    /// inspection never generates source, decodes data or claims recovery from parity.
    pub(super) fn execution_presentation(
        &self,
        progress: Option<&(RunToken, ProgressSnapshot)>,
    ) -> Value {
        let (status, elapsed, activity) = if let Some((_, progress)) = progress {
            (
                format!(
                    "Upstream {:?} · {} grants completed · {} bits compared",
                    progress.status, progress.completed_grants, progress.compared_bits
                ),
                format!("Active elapsed: {} ms", progress.elapsed_ms),
                format!(
                    "{} emitted samples; {} independently compared source bits.",
                    progress.emitted_samples, progress.compared_bits
                ),
            )
        } else if self.pending.is_some() {
            (
                "Preparing upstream work".into(),
                String::new(),
                String::new(),
            )
        } else if let Some((_, report)) = &self.report {
            (
                format!("Upstream {:?}", report.value().status),
                format!("Active elapsed: {} ms", report.value().elapsed_ms),
                "Inspect PRBS-RX for actual recovery and incomplete coverage.".into(),
            )
        } else {
            (
                if self.current() {
                    "Upstream configuration ready"
                } else {
                    "No accepted upstream configuration"
                }
                .into(),
                String::new(),
                String::new(),
            )
        };
        let (verification, block) = match &self.report {
            Some((token, report)) => report_presentation(
                report.value(),
                &token.run.to_string(),
                self.pending.is_some() || progress.is_some_and(|(active, _)| active != token),
            ),
            None => (
                json!({"summary":"No actual upstream report is available. Comparison and codeword outcomes remain unavailable.","rows":[]}),
                json!({"status":"attention","reason":"Run upstream to obtain independent source verification."}),
            ),
        };
        json!({"status":status,"elapsed":elapsed,"activity":activity,
            "verification":verification,"verification_block":block})
    }
}

/// Formats a display-only BER without changing the authoritative underlying value.
/// # Arguments
/// * `ber` - Original optional public metric, absent without compared decisions.
/// # Returns
/// Explicit unavailable, exact zero or scientific notation with the shared precision.
fn ber_label(ber: Option<f64>) -> String {
    match ber {
        None => "Unavailable".into(),
        Some(0.0) => "0".into(),
        Some(value) => format!(
            "{value:.precision$e}",
            precision = BER_MANTISSA_DECIMAL_PLACES
        ),
    }
}

/// Renders one original immutable report through its validated public observations.
/// # Arguments
/// * `report` - Existing charged report; no payload or sample arrays are copied.
/// * `run` - Original opaque run identity retained as text.
/// * `previous` - This report belongs to an older operation; its origin is explicit.
/// # Returns
/// A finite detail object and configuration-block indicator. Query errors are
/// unavailable diagnostics, never a fabricated zero or successful recovery.
fn report_presentation(report: &BatchReport, run: &str, previous: bool) -> (Value, Value) {
    let observations = report
        .verification()
        .and_then(|verification| report.fec_summary().map(|fec| (verification, fec)));
    let (verification, fec) = match observations {
        Ok(values) => values,
        Err(error) => {
            return (
                json!({"summary":error_text(error.to_string()),"rows":[]}),
                json!({"status":"error","reason":"Actual report verification is unavailable."}),
            );
        }
    };
    let mut rows = vec![
        row("Run", run),
        row("Lifecycle", format!("{:?}", verification.status)),
        row(
            "Full reference recovery",
            if verification.reference_recovered {
                "Verified"
            } else {
                "Not verified"
            },
        ),
        row("Planned source bits", verification.planned_source_bits),
        row("Generated source bits", verification.generated_source_bits),
        row("Compared source bits", verification.compared_source_bits),
        row(
            "Unverified generated bits",
            verification.unverified_source_bits,
        ),
        row("Bit errors", verification.bit_errors),
        row("BER", ber_label(verification.ber)),
        row(
            "Completed / planned grants",
            format!(
                "{} / {}",
                verification.completed_grants, verification.planned_grants
            ),
        ),
        row("Not-started grants", verification.not_started_grants),
        row(
            "Completed / pending codewords",
            format!(
                "{} / {}",
                verification.completed_codewords, verification.pending_codewords
            ),
        ),
        row(
            "Converged / nonconverged codewords",
            format!("{} / {}", fec.converged_words, fec.nonconverged_words),
        ),
        row(
            "Parity / shortening / fill failures",
            format!(
                "{} / {} / {}",
                fec.parity_invalid_words, fec.shortening_invalid_words, fec.fill_invalid_words
            ),
        ),
        row("Missing symbols", verification.missing_symbols),
        row("Active elapsed (ms)", report.elapsed_ms),
    ];
    if let Some(failure) = &report.failure {
        rows.push(row("Failure category", failure.category));
        if let Some(field) = failure.field {
            rows.push(row("Failure field", field));
        }
        rows.push(row("Failure diagnostic", error_text(&failure.diagnostic)));
    }
    let summary = if previous {
        "Retained previous report while another upstream operation is active. Its original run identity and coverage are shown."
    } else if verification.reference_recovered {
        "Actual independent source, FEC, fill and full-coverage verification succeeded."
    } else {
        "Actual retained report: full reference recovery is not verified. Lifecycle completion alone does not establish recovery."
    };
    let status = if report.status == RunStatus::Failed {
        "error"
    } else if verification.reference_recovered && !previous {
        "complete"
    } else {
        "attention"
    };
    (
        json!({"summary":summary,"rows":rows}),
        json!({"status":status,"reason":summary}),
    )
}

/// Builds one finite display row without changing its original value.
/// # Arguments
/// * `label` - Compiled presentation label.
/// * `value` - Original scalar or already bounded diagnostic.
/// # Returns
/// Exact display strings; wide integer counters never pass through JavaScript numbers.
fn row(label: &str, value: impl ToString) -> Value {
    json!({"label":label,"value":value.to_string()})
}

#[cfg(test)]
mod tests;
