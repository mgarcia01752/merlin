/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Native downstream editor transport beside public runnable project admission.
use super::project_file::{Project, MAX_PROJECT_BYTES};
use crate::shared::project_file::PROJECT_SCHEMA_VERSION;
use merlin::{
    common::{Project as PublicProject, ProjectDirection},
    downstream::{FullLoopObservationInput, FullLoopSweepPlan},
};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

/// Established partial-editor wire versions, independent of current Save routing.
pub(crate) const LEGACY_EDITOR_SCHEMAS: [u32; 2] = [1, 2];
const PROJECT_LIMIT_MESSAGE: &str = "Workbench exceeds 144 MiB.";

/// Editor state and, for schema 3, the complete independently admitted public plan.
pub(crate) struct Decoded {
    /// Original typed legacy editor document; native partial preparation remains authoritative.
    pub(crate) project: Project,
    /// Actual accepted plan, including its original observation policy/settings.
    /// None only for raw schema 1/2, whose runnable eligibility is not inferred here.
    pub(crate) public_plan: Option<FullLoopSweepPlan>,
}

#[derive(Deserialize)]
struct Header {
    schema_version: u32,
}

// Only project-admitted schema-3 bytes reach this projection. All strict envelope
// and observation validation remains with the public Project owner.
#[derive(Deserialize)]
struct Envelope {
    contents: Contents,
}

#[derive(Deserialize)]
struct Contents {
    workbench: String,
}

#[derive(Serialize)]
struct Output<'project, 'observations> {
    schema_version: u32,
    application: &'static str,
    direction: ProjectDirection,
    contents: OutputContents<'project, 'observations>,
}

#[derive(Serialize)]
struct OutputContents<'project, 'observations> {
    workbench: &'project str,
    observations: Option<&'observations FullLoopObservationInput>,
}

struct ProjectOutput {
    bytes: Vec<u8>,
    maximum_bytes: usize,
}

/// Decodes legacy partial editors or a publicly admitted downstream schema-3 project.
///
/// # Arguments
/// * `bytes` - Complete project bytes within the existing desktop project ceiling.
///
/// # Returns
/// Owned typed editor state. Raw schema 1/2 retain the existing partial-editor
/// path, without requiring runnable public admission; the caller must still use
/// the existing native preparation/validation before adoption. Schema 3 carries
/// the actual public plan, whose request observation policy/settings remain
/// available without copying or dropping them. No simulation or file I/O starts.
/// Caller-owned retention admission covers returned owners and overlapping input.
///
/// # Errors
/// Rejects byte limits, malformed legacy structure and all public envelope,
/// source, observation or runnable-admission failures. A public upstream project
/// is rejected as the wrong direction. Failure never alters caller state.
pub(crate) fn decode(bytes: &[u8]) -> Result<Decoded, String> {
    decode_with_limit(bytes, MAX_PROJECT_BYTES)
}

/// Serializes one explicitly eligible runnable editor as a public schema-3 project.
///
/// # Arguments
/// * `Sections` - Existing owned or borrowed serializable saved-section container.
/// * `project` - Exact accepted legacy editor values; inner schema/values are unchanged.
/// * `observations` - Actual accepted optional public observation input, borrowed
///   without applying a default, stripping stages or changing phase/budget choices.
///
/// # Returns
/// Complete publicly admitted project bytes. The caller selects the separate
/// existing schema-2 writer for incomplete editors before invoking this function;
/// this operation never falls back after a scientific or observation error.
/// Serialization/admission execute off-GUI under the caller's existing finite
/// preparation and retention policy. No file or simulation is started.
///
/// # Errors
/// Rejects bounded inner/outer serialization and all public runnable/policy
/// admission failures, preserving the editor and borrowed observation input.
pub(crate) fn encode_runnable<Sections: Serialize>(
    project: &Project<Sections>,
    observations: Option<&FullLoopObservationInput>,
) -> Result<Vec<u8>, String> {
    encode_runnable_with_limit(project, observations, MAX_PROJECT_BYTES)
}

fn decode_with_limit(bytes: &[u8], maximum_bytes: usize) -> Result<Decoded, String> {
    if bytes.len() > maximum_bytes {
        return Err(PROJECT_LIMIT_MESSAGE.into());
    }
    let header: Header = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    if LEGACY_EDITOR_SCHEMAS.contains(&header.schema_version) {
        return Ok(Decoded {
            project: serde_json::from_slice(bytes).map_err(|error| error.to_string())?,
            public_plan: None,
        });
    }
    let admitted = PublicProject::from_bytes(bytes).map_err(|error| error.to_string())?;
    if admitted.direction() != ProjectDirection::Downstream {
        return Err("Choose a downstream project.".into());
    }
    let envelope: Envelope =
        serde_json::from_slice(admitted.original_document()).map_err(|error| error.to_string())?;
    let project =
        serde_json::from_str(&envelope.contents.workbench).map_err(|error| error.to_string())?;
    let plan = admitted
        .downstream_plan()
        .map_err(|error| error.to_string())?;
    Ok(Decoded {
        project,
        public_plan: Some(plan),
    })
}

fn encode_runnable_with_limit<Sections: Serialize>(
    project: &Project<Sections>,
    observations: Option<&FullLoopObservationInput>,
    maximum_bytes: usize,
) -> Result<Vec<u8>, String> {
    let mut editor = ProjectOutput {
        bytes: Vec::new(),
        maximum_bytes,
    };
    serde_json::to_writer_pretty(&mut editor, project).map_err(|error| error.to_string())?;
    let workbench = std::str::from_utf8(&editor.bytes).map_err(|error| error.to_string())?;
    let wire = Output {
        schema_version: PROJECT_SCHEMA_VERSION,
        application: "MERLIN",
        direction: ProjectDirection::Downstream,
        contents: OutputContents {
            workbench,
            observations,
        },
    };
    let mut output = ProjectOutput {
        bytes: Vec::new(),
        maximum_bytes,
    };
    serde_json::to_writer(&mut output, &wire).map_err(|error| error.to_string())?;
    PublicProject::from_bytes(&output.bytes).map_err(|error| error.to_string())?;
    Ok(output.bytes)
}

impl Write for ProjectOutput {
    /// Appends one complete serializer fragment after checked byte admission.
    ///
    /// # Arguments
    /// * `bytes` - One output fragment; existing buffered bytes remain unchanged on error.
    ///
    /// # Returns
    /// Complete fragment length; this in-memory writer never reports partial writes.
    ///
    /// # Errors
    /// Rejects byte-count overflow, exceeded inclusive ceiling or allocation failure
    /// before appending any bytes from this fragment.
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

    /// Completes an in-memory flush without I/O or source progress.
    ///
    /// # Returns
    /// Unit because all serialized bytes already reside in the output vector.
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
#[path = "project_envelope_tests.rs"]
mod tests;
