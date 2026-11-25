/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Retained idle project restoration through existing public scientific owners.
use super::{
    source::retain_measured_source, MeasuredExperiment, MeasuredInputInfo, PreparedSession,
    RetentionBudget,
};
use merlin::common::Project;
use serde::Deserialize;

// The immutable public Project already validates the complete wire contract.
// This projection reads only the measured source needed by the existing owner;
// other fields stay with public admission, without another private schema check.
#[derive(Deserialize)]
struct ProjectEnvelope {
    contents: ProjectContents,
}

#[derive(Deserialize)]
struct ProjectContents {
    input: ProjectInput,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum ProjectInput {
    Configuration {},
    Measured { document: String },
}

impl PreparedSession {
    /// Restores and retains an already-admitted upstream project while idle.
    ///
    /// The public project supplies the complete scientific session, including
    /// accepted runtime overrides, receiver policy and edited-source provenance.
    /// Configuration documents are never used to rebuild that session. Measured
    /// source is parsed only through its public owner to recover the existing
    /// capture inventory and retain the exact original plan alongside it.
    ///
    /// # Arguments
    /// * `project` - Immutable admitted direction project. Its source owner remains
    ///   the caller's separately charged responsibility during preparation.
    /// * `budget` - Existing application counter, including old and candidate owners.
    ///
    /// # Returns
    /// An idle candidate using existing actual scientific metadata and measured
    /// source retention. No run, source progress, file operation or host adoption
    /// occurs. Temporary public builders retain their existing finite limits;
    /// this does not introduce an aggregate preparation-peak guarantee.
    ///
    /// # Errors
    /// Preserves public project/measurement/JSON diagnostic strings and existing
    /// admission debug diagnostics for retained accounting failures. A wrong
    /// direction rejects through the public project. Failure drops only the
    /// candidate and leaves accepted controllers, reports and project unchanged.
    pub(crate) fn from_project(
        project: &Project,
        budget: &RetentionBudget,
    ) -> Result<Self, String> {
        let simulation = project
            .upstream_simulation()
            .map_err(|error| error.to_string())?;
        let envelope: ProjectEnvelope = serde_json::from_slice(project.original_document())
            .map_err(|error| error.to_string())?;
        let (inventory, source) = match envelope.contents.input {
            ProjectInput::Configuration {} => (None, None),
            ProjectInput::Measured { document } => {
                let plan = MeasuredExperiment::from_json(document.as_bytes())
                    .map_err(|error| error.to_string())?;
                let inventory = MeasuredInputInfo {
                    ucd_fragments: plan.ucd_fragments.len(),
                    rxmer_captures: plan.rxmer.len(),
                    pre_eq_initial: plan.pre_eq_initial.is_some(),
                    pre_eq_events: plan.pre_eq_events.len(),
                };
                let source =
                    retain_measured_source(&plan, budget).map_err(|error| format!("{error:?}"))?;
                (Some(inventory), Some(source))
            }
        };
        Self::retain_simulation(simulation, budget, inventory, source)
            .map_err(|error| format!("{error:?}"))
    }
}

#[cfg(test)]
#[path = "project_restore_tests.rs"]
mod tests;
