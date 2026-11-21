/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Validated typed handoff from native capture admission to the independent PHY.
use super::{ApiResult, ConfigurationDocument, ExperimentConfig, ExperimentOptions, PhyError};
use serde::Deserialize;

/// Owned capture-adapter input, with original documents separate from execution.
///
/// Construction validates domain readiness and the provenance envelope. Capture
/// association and the accuracy of provenance claims remain the caller's duty;
/// the PHY contains no binary capture parser or filesystem dependency.
#[derive(Debug, Clone)]
pub struct MeasuredConfiguration {
    pub(crate) configuration: ExperimentConfig,
    pub(crate) options: ExperimentOptions,
    pub(crate) document: ConfigurationDocument,
    pub(crate) provenance_json: String,
}

impl MeasuredConfiguration {
    /// Validates an immutable measured-input handoff without starting processing.
    ///
    /// # Arguments
    /// * `configuration` - Resolved scientific input after complete association.
    /// * `options` - Finite execution and retained-output budgets.
    /// * `document` - Original generic source context, before captured projection.
    /// * `provenance_json` - Schema1 upstream provenance, at most 16 MiB.
    ///
    /// # Returns
    /// An owned prepared input; raw source documents remain unchanged.
    ///
    /// # Errors
    /// Rejects invalid configuration, unsupported provenance format/version,
    /// malformed JSON or exceeded byte/work budgets before any source consumption.
    pub fn new(
        configuration: ExperimentConfig,
        options: ExperimentOptions,
        document: ConfigurationDocument,
        provenance_json: String,
    ) -> ApiResult<Self> {
        if provenance_json.len() > 16 * 1024 * 1024 {
            return Err(PhyError::ResourceLimit {
                field: "measured.provenance_bytes",
            }
            .into());
        }
        #[derive(Deserialize)]
        struct Envelope {
            format: String,
            schema_version: u32,
        }
        let envelope: Envelope = serde_json::from_str(&provenance_json)?;
        if envelope.format != "merlin.upstream.measured-provenance" || envelope.schema_version != 1
        {
            return Err(PhyError::Unsupported {
                field: "measured.provenance_format_version",
            }
            .into());
        }
        document.validate()?;
        crate::simulation::preview(&configuration, &options)?;
        Ok(Self {
            configuration,
            options,
            document,
            provenance_json,
        })
    }
}
