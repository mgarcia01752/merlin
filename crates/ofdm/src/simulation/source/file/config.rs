/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::{
    error::invalid,
    simulation::source::{Prbs15, PrbsSeed},
    SimulationError,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
/// Application input-memory ceiling, not a PHY minimum/alignment requirement.
pub const MAX_FILE_BYTES: usize = 16 * 1024 * 1024;
/// Finite original-plus-fill work ceiling in meaningful source bits.
pub const MAX_FILE_BITS: u64 = MAX_FILE_BYTES as u64 * 8;
/// Explicit finite fill policy, independent of channel noise and duration.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrbsFill {
    pub target_bits: u64,
    pub seed: PrbsSeed,
}
/// Path-independent request selection; the adapter supplies original file bytes.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SourceSelection {
    #[default]
    Prbs15,
    File {
        fill: Option<PrbsFill>,
    },
}
/// Validated immutable original input shared across source, truth and settings.
/// Raw bytes are deliberately absent from JSON metadata.
#[derive(Clone, Serialize)]
pub struct FilePayload {
    #[serde(skip)]
    bytes: Arc<[u8]>,
    pub(crate) original_bytes: usize,
    pub(crate) target_bits: u64,
    pub(crate) fill: Option<PrbsFill>,
}
impl FilePayload {
    /// Validates original bytes and an optional explicit append-to-fill target.
    /// # Arguments
    /// * `bytes` - Exact original bytes, copied into shared immutable ownership.
    /// * `fill` - Optional finite total bit count and independent nonzero PRBS seed.
    /// # Returns
    /// Validated input; empty files are allowed, with or without explicit fill.
    /// # Errors
    /// Rejects excessive bytes/bits, targets below the file length or invalid seeds.
    pub fn new(bytes: &[u8], fill: Option<PrbsFill>) -> Result<Self, SimulationError> {
        if bytes.len() > MAX_FILE_BYTES {
            return Err(invalid("Payload file exceeds16MiB."));
        }
        let original_bits = bytes.len() as u64 * 8;
        let target_bits = fill.map_or(original_bits, |policy| policy.target_bits);
        if target_bits < original_bits || target_bits > MAX_FILE_BITS {
            return Err(invalid(
                "File fill target is below the original size or exceeds the source budget.",
            ));
        }
        if let Some(policy) = fill {
            Prbs15::new(policy.seed)?;
        }
        Ok(Self {
            bytes: Arc::from(bytes),
            original_bytes: bytes.len(),
            target_bits,
            fill,
        })
    }
    /// Borrows unchanged original bytes without advancing a source or verifier.
    /// # Returns
    /// Exactly the caller-provided input, including zeros, BOM and line endings.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Reports the finite original-plus-fill length before coding padding.
    /// # Returns
    /// Meaningful bits, independent of observation duration or FEC word boundaries.
    pub fn target_bits(&self) -> u64 {
        self.target_bits
    }
}
