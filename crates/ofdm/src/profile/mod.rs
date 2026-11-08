/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Profile hypotheses, role protection and analytical evaluation.
pub mod candidate;
pub mod document;
pub mod evaluation;

pub use crate::error::LabError;
pub use crate::scenario::association;
#[cfg(feature = "simulation")]
pub use crate::simulation;

/// Compatibility imports; canonical models live in scenario and candidate modules.
pub mod model {
    pub use super::candidate::{
        AssignmentOrigin, CandidateProfile, CapturedProfile, CarrierAssignment,
        CANDIDATE_SCHEMA_VERSION,
    };
    pub use crate::scenario::{
        ChannelFrame, FrequencyGrid, RxMerFrame, ScenarioKey, SourceProvenance, MAX_LAB_CARRIERS,
    };
    pub use crate::types::ProtocolScope;
}
