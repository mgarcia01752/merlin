/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Native schema-3 upstream project transport over the accepted public owner.
//!
//! The caller retains accepted original input under its existing budget, supplies
//! actual current options/policy and owns off-GUI execution, reservations and I/O.
//! This module never reconstructs source documents or implements scientific logic.

use crate::shared::project_file::MAX_PROJECT_BYTES;
use merlin::{
    common::{Project, ProjectDirection},
    upstream::{ExperimentOptions, ReceiverObservationPolicy, SoftObservationPolicyInfo},
};
use serde::Serialize;
use std::io::{self, Write};

/// Direction-tagged wire contract; `Project::from_bytes` validates this value.
const PROJECT_SCHEMA_VERSION: u32 = 3;
const PROJECT_LIMIT_MESSAGE: &str = "Upstream project exceeds the byte limit.";

/// Borrowed exact accepted input, before any outer project escaping.
pub(super) enum Input<'input> {
    /// Complete original generic configuration JSON, including dormant presets.
    Configuration(&'input [u8]),
    /// Current accepted fields and separate exact original configuration provenance.
    EditedConfiguration {
        document: &'input [u8],
        original_document: &'input [u8],
    },
    /// Complete original measured-plan JSON, including captures and association.
    Measured(&'input [u8]),
}

#[derive(Serialize)]
struct Wire<'input> {
    schema_version: u32,
    application: &'static str,
    direction: ProjectDirection,
    contents: Contents<'input>,
}

#[derive(Serialize)]
struct Contents<'input> {
    input: Source<'input>,
    options: &'input ExperimentOptions,
    receiver_observations: Observations,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Source<'input> {
    Configuration {
        document: &'input str,
        #[serde(skip_serializing_if = "Option::is_none")]
        original_document: Option<&'input str>,
    },
    Measured {
        document: &'input str,
    },
}

#[derive(Serialize)]
struct Observations {
    maximum_frames: usize,
    maximum_scalar_slots: usize,

    // Schema 3 calls this field `soft`, independently of policy.info().soft_metrics.
    soft: Option<SoftObservationPolicyInfo>,
}

struct ProjectOutput {
    bytes: Vec<u8>,
    maximum_bytes: usize,
}

/// Encodes exact accepted upstream input and current settings as a schema-3 project.
///
/// Input strings and options are borrowed during synchronous serialization. The
/// inner source is never parsed/reformatted by this adapter. Final public project
/// admission validates source, settings, association and observation compatibility
/// while idle; it neither runs a simulation nor changes the caller's session.
///
/// # Arguments
/// * `input` - Complete accepted original configuration or measured-plan UTF-8 bytes.
/// * `options` - Actual current accepted execution/retention settings, including
///   overrides that differ from the original source document.
/// * `observations` - Actual accepted receiver observation policy.
///
/// # Returns
/// Owned complete project bytes within the shared desktop project ceiling. The
/// caller owns reservation of serialization/admission scratch and returned output
/// while accepted source remains retained, and must apply original-input
/// protection before writing a chosen destination.
///
/// # Errors
/// Rejects input/output byte limits, invalid UTF-8, serialization/allocation errors
/// and all errors from public `Project::from_bytes` admission. No file is opened
/// and no caller-owned input, options or policy is changed on failure.
pub(super) fn encode(
    input: Input<'_>,
    options: &ExperimentOptions,
    observations: ReceiverObservationPolicy,
) -> Result<Vec<u8>, String> {
    encode_with_limit(input, options, observations, MAX_PROJECT_BYTES)
}

fn encode_with_limit(
    input: Input<'_>,
    options: &ExperimentOptions,
    observations: ReceiverObservationPolicy,
    maximum_bytes: usize,
) -> Result<Vec<u8>, String> {
    let original = match &input {
        Input::Configuration(bytes) | Input::Measured(bytes) => *bytes,
        Input::EditedConfiguration { document, .. } => *document,
    };
    if original.len() > maximum_bytes {
        return Err(PROJECT_LIMIT_MESSAGE.into());
    }
    let document = std::str::from_utf8(original).map_err(|error| error.to_string())?;
    let source = match input {
        Input::Configuration(_) => Source::Configuration {
            document,
            original_document: None,
        },
        Input::EditedConfiguration {
            original_document, ..
        } => {
            if original_document.len() > maximum_bytes {
                return Err(PROJECT_LIMIT_MESSAGE.into());
            }
            Source::Configuration {
                document,
                original_document: Some(
                    std::str::from_utf8(original_document).map_err(|error| error.to_string())?,
                ),
            }
        }
        Input::Measured(_) => Source::Measured { document },
    };
    let policy = observations.info();
    let wire = Wire {
        schema_version: PROJECT_SCHEMA_VERSION,
        application: "MERLIN",
        direction: ProjectDirection::Upstream,
        contents: Contents {
            input: source,
            options,
            receiver_observations: Observations {
                maximum_frames: policy.maximum_frames,
                maximum_scalar_slots: policy.maximum_scalar_slots,
                soft: policy.soft_metrics,
            },
        },
    };
    let mut output = ProjectOutput {
        bytes: Vec::new(),
        maximum_bytes,
    };
    serde_json::to_writer(&mut output, &wire).map_err(|error| error.to_string())?;
    Project::from_bytes(&output.bytes).map_err(|error| error.to_string())?;
    Ok(output.bytes)
}

impl Write for ProjectOutput {
    /// Appends one serializer fragment only when its complete byte count fits.
    ///
    /// # Arguments
    /// * `bytes` - UTF-8 output fragment emitted by the JSON serializer.
    ///
    /// # Returns
    /// The complete fragment length; partial writes are never reported.
    ///
    /// # Errors
    /// Rejects checked length overflow, the inclusive output ceiling or allocation
    /// failure before appending any part of this fragment.
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self
            .bytes
            .len()
            .checked_add(bytes.len())
            .is_none_or(|length| length > self.maximum_bytes)
        {
            return Err(io::Error::other(PROJECT_LIMIT_MESSAGE));
        }
        self.bytes
            .try_reserve(bytes.len())
            .map_err(|error| io::Error::other(error.to_string()))?;
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    /// Completes an in-memory flush without filesystem or scientific side effects.
    ///
    /// # Returns
    /// Unit; serializer fragments are already held in the output vector.
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
#[path = "project_tests.rs"]
mod tests;
