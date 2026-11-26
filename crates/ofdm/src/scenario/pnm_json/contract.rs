/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit producer compatibility, independent of binary PNM format revisions.
use crate::SimulationError;
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
use serde::{Deserialize, Serialize};

/// Decoder compatibility revision implemented by this adapter.
pub const DECODER_CONTRACT_REVISION: u16 = 1;
/// Producer releases verified against the checked-in decoded capture fixtures.
pub const SUPPORTED_DECODER_VERSIONS: &[&str] = &["0.3.6"];

/// Caller-declared provenance of all three decoded JSON documents.
///
/// Ordinary decoder JSON has no producer stamp. This declaration must come from
/// the caller's export record; admission does not authenticate a producer binary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecoderContract {
    pub revision: u16,
    pub producer: String,
    pub producer_version: String,
}
impl DecoderContract {
    /// Checks the explicitly declared producer against the verified allowlist.
    /// # Returns
    /// Unit for contract revision1 and pnm-decode0.3.6.
    /// # Errors
    /// Rejects unknown revisions, producers or versions without guessing compatibility.
    pub fn validate(&self) -> Result<(), SimulationError> {
        if self.revision != DECODER_CONTRACT_REVISION
            || self.producer != "pnm-decode"
            || !SUPPORTED_DECODER_VERSIONS.contains(&self.producer_version.as_str())
        {
            return Err(SimulationError::new(
                "UnsupportedDecoderContract",
                "Unsupported decoder contract; revision1 requires pnm-decode0.3.6.",
            ));
        }
        Ok(())
    }
}

impl RetainedPayload for DecoderContract {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            revision,
            producer,
            producer_version,
        } = self;
        merlin_api::retention_support::sum([
            revision.retained_children(),
            producer.retained_children(),
            producer_version.retained_children(),
        ])
    }
}
