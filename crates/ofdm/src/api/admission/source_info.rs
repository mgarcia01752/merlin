/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Passive finite-source limits and metadata; no generator or payload serialization.
use super::{ApiError, ApiResult, ErrorCode, FileFillPolicy, PrbsSeed, SourceInput};
use crate::simulation::source::{
    file::{PrbsFill, MAX_FILE_BITS, MAX_FILE_BYTES},
    PayloadSourceConfig,
};
use serde::Serialize;

/// Authoritative existing file admission/work limits, independent of capture/decoder bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct SourceFileLimits {
    /// Inclusive original-file input ceiling in octets; empty files are allowed.
    pub max_original_bytes: usize,
    /// Inclusive finite original-plus-fill ceiling in meaningful source bits.
    pub max_total_bits: u64,
}

/// Constant-size actual finite-file metadata, excluding bytes, generated fill and FEC padding.
#[derive(Clone, Copy, Serialize)]
#[non_exhaustive]
pub struct SourceFileInfo {
    /// Exact retained input length, including BOM, zeros and original line endings.
    pub original_bytes: usize,
    /// Meaningful original bits, using the octet width independently of fill policy.
    pub original_bits: u64,
    /// Validated original-plus-fill bit target, including a possible partial final byte.
    pub target_bits: u64,
    /// Appended meaningful bits; zero does not imply continuous PRBS or missing input.
    pub fill_bits: u64,
    /// Actual independently seeded optional append policy; Some may append zero bits.
    pub fill: Option<FileFillPolicy>,
}

impl SourceInput {
    /// Returns an independently validated finite fill selection sharing the original bytes.
    /// # Arguments
    /// * `fill` - Optional total meaningful-bit target and independently checked fill seed.
    /// # Returns
    /// New immutable file source sharing its original byte buffer and preserving name/hash.
    /// Bounded metadata is cloned; no source bits, padding or generator state are produced.
    /// None removes append policy and restores the exact original meaningful-bit length.
    /// # Errors
    /// Rejects continuous PRBS inputs and existing invalid finite targets/seeds without
    /// modifying this source, its original byte buffer or its prior fill selection.
    pub fn with_fill(&self, fill: Option<FileFillPolicy>) -> ApiResult<Self> {
        let PayloadSourceConfig::File { payload } = &self.configuration else {
            return Err(ApiError::new(
                ErrorCode::InvalidInput,
                "Fill replacement requires a file source",
            ));
        };
        let payload = payload.with_fill(fill.map(|policy| PrbsFill {
            target_bits: policy.target_bits,
            seed: policy.seed.value(),
        }))?;
        let mut source = self.clone();
        source.configuration = PayloadSourceConfig::File { payload };
        Ok(source)
    }

    /// Reads the existing scientific owner's finite file-memory and bit-work ceilings.
    /// # Returns
    /// Owned byte/bit limits used by file and unnamed_file admission. No source, memory
    /// allocation, generator or simulation is started; these are application resource limits.
    pub fn file_limits() -> SourceFileLimits {
        SourceFileLimits {
            max_original_bytes: MAX_FILE_BYTES,
            max_total_bits: MAX_FILE_BITS,
        }
    }

    /// Reads bounded original/fill metadata without generating or serializing payload bytes.
    /// # Returns
    /// None for continuous PRBS15; an owned snapshot for named/unnamed finite files,
    /// including empty files. Original bytes, hash, target and independent fill seed
    /// remain unchanged. Counts exclude coding padding and observation-window scope.
    pub fn file_info(&self) -> Option<SourceFileInfo> {
        let PayloadSourceConfig::File { payload } = &self.configuration else {
            return None;
        };

        // Each retained source byte is one octet; this is not a modulation bit loading.
        let original_bits = payload.original_bytes as u64 * u64::from(u8::BITS);
        Some(SourceFileInfo {
            original_bytes: payload.original_bytes,
            original_bits,
            target_bits: payload.target_bits(),
            fill_bits: payload.target_bits() - original_bits,
            fill: payload.fill.map(|fill| FileFillPolicy {
                target_bits: fill.target_bits,
                seed: PrbsSeed(fill.seed),
            }),
        })
    }
}
