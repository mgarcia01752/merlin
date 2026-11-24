/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Concrete display admission; failure never discards completed scientific results.
use super::observations::{ObservationError, Observations};
use crate::shell::{RetentionBudget, RetentionError};
use merlin::downstream::{ErrorCode, FullLoopResult};

const RETENTION_MESSAGE: &str =
    "Instrument data exceeds the workbench data limit; scientific results remain available.";

/// Admits all stored display copies, or retains only a static unavailable diagnostic.
///
/// # Arguments
/// * `result` - Independently charged actual complete/partial backend output.
/// * `budget` - Canonical host policy, including all source and old-result owners.
///
/// # Returns
/// Admitted existing display values, or an empty projection with static text. No
/// fallback Vec/String is allocated or retained after admission failure. UI JSON
/// serialization remains separate bounded transport, outside this stored owner.
pub(super) fn admit(
    result: &FullLoopResult,
    budget: &RetentionBudget,
) -> (Observations, Option<&'static str>) {
    match Observations::from_result_admitted(result, budget) {
        Ok(observations) => (observations, None),
        Err(error) => {
            let message = match error {
                ObservationError::Backend(error) => match error.code() {
                    ErrorCode::ResourceLimit => RETENTION_MESSAGE,
                    _ => "Instrument observations are unavailable; scientific results remain available.",
                },
                ObservationError::Retention(error) => match error {
                    RetentionError::ArithmeticOverflow => {
                        "Instrument storage limits could not be validated; scientific results remain available."
                    }
                    _ => RETENTION_MESSAGE,
                },
                ObservationError::UnsupportedRepresentation => {
                    "Instrument observation format is unsupported; scientific results remain available."
                }
            };
            (Observations::default(), Some(message))
        }
    }
}
