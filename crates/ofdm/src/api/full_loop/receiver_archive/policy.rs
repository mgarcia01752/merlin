/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent history copy caps; no source, live capture or scientific-domain defaults change.
use super::*;
use crate::common::constants::*;
use serde::{Deserialize, Serialize};

/// Reducible application-history caps; zero omits that role/event without fabricating data.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FullLoopReceiverArchiveInput {
    pub payload_cells: usize,
    pub ncp_cells: usize,
    pub protected_cells_per_stream: usize,
    pub decoder_llrs: usize,
    /// Owner-accounted fixed/payload/context bytes; excludes allocator/control-block overhead.
    pub max_retained_bytes: usize,
}
impl Default for FullLoopReceiverArchiveInput {
    /// Selects the existing finite Qt history counts and independent eight-MiB byte ceiling.
    /// # Returns
    /// Editable retention policy only; creating it captures nothing and executes no science.
    fn default() -> Self {
        Self {
            payload_cells: MAX_RECEIVER_ARCHIVE_PAYLOAD_CELLS,
            ncp_cells: MAX_RECEIVER_ARCHIVE_NCP_CELLS,
            protected_cells_per_stream: MAX_RECEIVER_ARCHIVE_PROTECTED_CELLS,
            decoder_llrs: MAX_RECEIVER_ARCHIVE_DECODER_LLRS,
            max_retained_bytes: MAX_RECEIVER_ARCHIVE_BYTES,
        }
    }
}

/// Frozen validated history policy; count reduction is independent of live scientific retention.
#[derive(Clone, Serialize)]
#[serde(transparent)]
pub struct FullLoopReceiverArchivePolicy(FullLoopReceiverArchiveInput);
impl FullLoopReceiverArchivePolicy {
    /// Validates independent caps and a finite byte budget before any observation copies.
    /// # Arguments
    /// * `input` - Reducible per-role/prefix limits; bytes must be positive and within the ceiling.
    /// # Returns
    /// Immutable owned policy; zero count limits deliberately omit the corresponding data.
    /// # Errors
    /// Reports ResourceLimit for an excessive count or zero/excessive byte budget.
    pub fn new(input: FullLoopReceiverArchiveInput) -> ApiResult<Self> {
        if input.payload_cells > MAX_RECEIVER_ARCHIVE_PAYLOAD_CELLS
            || input.ncp_cells > MAX_RECEIVER_ARCHIVE_NCP_CELLS
            || input.protected_cells_per_stream > MAX_RECEIVER_ARCHIVE_PROTECTED_CELLS
            || input.decoder_llrs > MAX_RECEIVER_ARCHIVE_DECODER_LLRS
            || input.max_retained_bytes == 0
            || input.max_retained_bytes > MAX_RECEIVER_ARCHIVE_BYTES
        {
            return Err(limit(
                "Receiver archive policy exceeds finite history budgets",
            ));
        }
        Ok(Self(input))
    }

    /// Admits strict bounded policy JSON through the same typed validator.
    /// # Arguments
    /// * `bytes` - At most4096 UTF-8 JSON bytes; absent fields use documented history defaults.
    /// # Returns
    /// Frozen validated policy without touching any run or observation.
    /// # Errors
    /// Rejects transport size, malformed/unknown fields and count/byte-domain violations.
    pub fn from_json(bytes: &[u8]) -> ApiResult<Self> {
        if bytes.len() > MAX_RECEIVER_ARCHIVE_POLICY_BYTES {
            return Err(limit("Receiver archive policy exceeds transport budget"));
        }
        let input = serde_json::from_slice(bytes)
            .map_err(|cause| super::super::super::error::invalid(cause.to_string()))?;
        Self::new(input)
    }

    /// Borrows immutable accepted count and byte limits without allocating observations.
    /// # Returns
    /// Policy input valid for this policy's lifetime; clones do not change any run.
    pub fn settings(&self) -> &FullLoopReceiverArchiveInput {
        &self.0
    }
}
