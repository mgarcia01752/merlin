/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Direction-tagged original-preserving project admission; no scientific implementation.
mod error;
#[cfg(test)]
mod tests;
mod upstream;
use crate::common::constants::*;
use docsis_ofdm_fec::api::{ApiError, ErrorCode, FullLoopObservationInput, FullLoopSweepPlan};
use docsis_ofdma::api::Simulation;
pub use error::ProjectError;
use serde::{Deserialize, Serialize};
use std::{io::Read, path::Path, sync::Arc};
use upstream::UpstreamContents;

/// Scientific direction of one independently saved project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectDirection {
    /// Downstream OFDM, including original schema 1/2 documents.
    Downstream,
    /// Upstream OFDMA using its separate configuration and measured-input owners.
    Upstream,
}
/// Checked original input inventory, separate from execution/result memory budgets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProjectCollectionInfo {
    /// Number of distinct direction projects, at most two.
    pub projects: usize,
    /// Sum of exact originally admitted complete document sizes.
    pub serialized_bytes: usize,
    /// Combined original input budget, including every dormant/unselected original.
    pub original_input_bytes: usize,
    /// Directions in the caller's collection order; no duplicates.
    pub directions: Vec<ProjectDirection>,
}
#[derive(Deserialize)]
struct Header {
    schema_version: u32,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Wire {
    schema_version: u32,
    application: String,
    direction: ProjectDirection,
    contents: Box<serde_json::value::RawValue>,
}
#[derive(Serialize)]
struct Output<'project> {
    schema_version: u32,
    application: &'static str,
    direction: ProjectDirection,
    contents: &'project Contents,
}
#[derive(Serialize)]
#[serde(untagged)]
enum Contents {
    Downstream(DownstreamContents),
    Upstream(UpstreamContents),
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DownstreamContents {
    workbench: String,
    observations: Option<FullLoopObservationInput>,
}

/// Immutable admitted original project; loading never starts a worker or consumes source.
pub struct Project {
    original: Arc<[u8]>,
    contents: Contents,
    original_input_bytes: usize,
}
fn invalid(message: impl Into<String>) -> ProjectError {
    ApiError::new(ErrorCode::InvalidInput, message).into()
}
fn limit(message: &str) -> ProjectError {
    ApiError::new(ErrorCode::ResourceLimit, message).into()
}
fn malformed(cause: serde_json::Error) -> ProjectError {
    invalid(cause.to_string())
}
fn io(cause: std::io::Error) -> ProjectError {
    ApiError::new(ErrorCode::Io, cause.to_string()).into()
}
impl Project {
    /// Admits complete original legacy or direction-tagged project bytes while idle.
    /// # Arguments
    /// * `bytes` - UTF-8 JSON within the inclusive saved-document byte ceiling.
    /// # Returns
    /// Owned immutable project with exact original bytes and validated scientific settings.
    /// Legacy schema 1/2 retain downstream interpretation. No simulation executes.
    /// # Errors
    /// Rejects unknown schemas/fields, corrupt originals, incompatible settings and budgets.
    /// Failure does not mutate any previously accepted project or consumer session.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProjectError> {
        if bytes.len() > MAX_WORKBENCH_BYTES {
            return Err(limit("Project exceeds the saved-document byte budget"));
        }
        let header: Header = serde_json::from_slice(bytes).map_err(malformed)?;
        let contents = match header.schema_version {
            WORKBENCH_SCHEMA_V1 | WORKBENCH_SCHEMA_V2 => Contents::Downstream(DownstreamContents {
                workbench: std::str::from_utf8(bytes)
                    .map_err(|cause| invalid(cause.to_string()))?
                    .into(),
                observations: None,
            }),
            PROJECT_SCHEMA_VERSION => {
                let wire: Wire = serde_json::from_slice(bytes).map_err(malformed)?;
                if wire.schema_version != PROJECT_SCHEMA_VERSION || wire.application != "MERLIN" {
                    return Err(invalid("Invalid project schema/application"));
                }
                match wire.direction {
                    ProjectDirection::Downstream => Contents::Downstream(
                        serde_json::from_str(wire.contents.get()).map_err(malformed)?,
                    ),
                    ProjectDirection::Upstream => Contents::Upstream(
                        serde_json::from_str(wire.contents.get()).map_err(malformed)?,
                    ),
                }
            }
            _ => {
                return Err(ApiError::new(
                    ErrorCode::UnsupportedCapability,
                    "Unsupported project schema version",
                )
                .into())
            }
        };
        let original_input_bytes = match &contents {
            Contents::Downstream(saved) => {
                crate::workbench::prepare(saved.workbench.as_bytes(), saved.observations.clone())?;
                crate::workbench::original_byte_count(saved.workbench.as_bytes())?
            }
            Contents::Upstream(saved) => saved.simulation()?.1,
        };
        if original_input_bytes > MAX_TOTAL_ORIGINAL_BYTES {
            return Err(limit(
                "Project originals exceed the combined input byte budget",
            ));
        }
        Ok(Self {
            original: Arc::from(bytes),
            contents,
            original_input_bytes,
        })
    }

    /// Reads and admits one regular project file without changing its contents.
    /// # Arguments
    /// * `path` - Caller-selected existing regular file; extensions never select direction.
    /// # Returns
    /// Owned idle project retaining the exact original bytes.
    /// # Errors
    /// Returns structured I/O, size, schema, integrity and scientific admission errors.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ProjectError> {
        let metadata = std::fs::metadata(path.as_ref()).map_err(io)?;
        if !metadata.is_file() {
            return Err(
                ApiError::new(ErrorCode::Io, "Project input must be a regular file").into(),
            );
        }
        let file = std::fs::File::open(path).map_err(io)?;
        let metadata = file.metadata().map_err(io)?;
        if !metadata.is_file() {
            return Err(
                ApiError::new(ErrorCode::Io, "Project input must be a regular file").into(),
            );
        }
        if metadata.len() > MAX_WORKBENCH_BYTES as u64 {
            return Err(limit("Project file exceeds the saved-document byte budget"));
        }
        let mut bytes = Vec::new();
        file.take((MAX_WORKBENCH_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(io)?;
        Self::from_bytes(&bytes)
    }

    /// Inspects the scientific direction without starting admission or execution again.
    /// # Returns
    /// Direction of this independently owned project.
    pub fn direction(&self) -> ProjectDirection {
        match &self.contents {
            Contents::Downstream(_) => ProjectDirection::Downstream,
            Contents::Upstream(_) => ProjectDirection::Upstream,
        }
    }

    /// Borrows the exact originally admitted complete document bytes.
    /// # Returns
    /// Immutable original bytes, valid for this project borrow; no canonicalization occurs.
    pub fn original_document(&self) -> &[u8] {
        &self.original
    }

    /// Serializes schema 3 while preserving each embedded original document string.
    /// # Returns
    /// Bounded owned UTF-8 JSON; no destination is opened or overwritten.
    /// # Errors
    /// Rejects export overhead exceeding the unchanged per-document byte budget.
    /// A maximal legacy input remains readable even if its schema 3 wrapper cannot fit.
    pub fn to_json(&self) -> Result<String, ProjectError> {
        Ok(crate::upstream::bounded_json(
            &Output {
                schema_version: PROJECT_SCHEMA_VERSION,
                application: "MERLIN",
                direction: self.direction(),
                contents: &self.contents,
            },
            MAX_WORKBENCH_BYTES,
        )?)
    }

    /// Prepares an independent owned downstream sweep through its existing public owners.
    /// # Returns
    /// Idle request plan preserving every saved simulation and observation setting.
    /// # Errors
    /// Rejects an upstream project with InvalidInput; preserves all accepted project state.
    pub fn downstream_plan(&self) -> Result<FullLoopSweepPlan, ProjectError> {
        match &self.contents {
            Contents::Downstream(saved) => Ok(crate::workbench::prepare(
                saved.workbench.as_bytes(),
                saved.observations.clone(),
            )?),
            Contents::Upstream(_) => Err(invalid("Project direction is upstream")),
        }
    }

    /// Creates a separate idle upstream session using existing validated source owners.
    /// # Returns
    /// Owned configured session, independent of this project and later returned sessions.
    /// # Errors
    /// Rejects a downstream project with InvalidInput; no existing state is changed.
    pub fn upstream_simulation(&self) -> Result<Simulation, ProjectError> {
        match &self.contents {
            Contents::Upstream(saved) => Ok(saved.simulation()?.0),
            Contents::Downstream(_) => Err(invalid("Project direction is downstream")),
        }
    }

    /// Admits aggregate original-input storage for independent direction projects.
    /// # Arguments
    /// * `projects` - At most two immutable references with distinct directions; empty is valid.
    /// # Returns
    /// Owned checked counts in caller order. No session runs or caller storage changes.
    /// # Errors
    /// Rejects repeated directions, count/byte ceilings and arithmetic overflow.
    /// Runtime, retained results and simultaneous execution need separate admission.
    pub fn collection_info(projects: &[&Self]) -> Result<ProjectCollectionInfo, ProjectError> {
        if projects.len() > MAX_PROJECT_COLLECTION_COUNT {
            return Err(limit(
                "Project collection exceeds the direction count budget",
            ));
        }
        let mut info = ProjectCollectionInfo {
            projects: projects.len(),
            serialized_bytes: 0,
            original_input_bytes: 0,
            directions: Vec::new(),
        };
        for project in projects {
            if info.directions.contains(&project.direction()) {
                return Err(invalid("Project collection repeats a direction"));
            }
            info.directions.push(project.direction());
            info.serialized_bytes = info
                .serialized_bytes
                .checked_add(project.original.len())
                .ok_or_else(|| limit("Project serialized byte count overflow"))?;
            info.original_input_bytes = info
                .original_input_bytes
                .checked_add(project.original_input_bytes)
                .ok_or_else(|| limit("Project original input byte count overflow"))?;
        }
        if info.serialized_bytes > MAX_PROJECT_COLLECTION_BYTES
            || info.original_input_bytes > MAX_TOTAL_ORIGINAL_BYTES
        {
            return Err(limit(
                "Project collection exceeds the combined byte budgets",
            ));
        }
        Ok(info)
    }
}
