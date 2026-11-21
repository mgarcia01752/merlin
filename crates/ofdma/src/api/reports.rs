/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! Bounded JSON export adapter; no destination path or overwrite policy in core.
use super::{ApiResult, BatchReport};
use std::io::{self, Write};

/// Extension contract for bounded experiment report serialization.
pub trait ReportJson {
    /// Serializes actual owned science into compact versioned JSON.
    ///
    /// Returns an owned string capped at 64 MiB. Errors preserve the report and
    /// never create or overwrite a file; unavailable metrics remain JSON null.
    fn to_json(&self) -> ApiResult<String>;
}

struct LimitedBuffer {
    bytes: Vec<u8>,
    maximum: usize,
}
impl Write for LimitedBuffer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self
            .bytes
            .len()
            .checked_add(bytes.len())
            .is_none_or(|size| size > self.maximum)
        {
            return Err(io::Error::other("JSON exceeds configured byte budget"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl ReportJson for BatchReport {
    /// Writes this immutable report through a bounded Rust serializer.
    ///
    /// Returns compact schema-versioned JSON; serialization errors retain cause.
    fn to_json(&self) -> ApiResult<String> {
        bounded_json(self, 64 * 1024 * 1024)
    }
}

pub(crate) fn bounded_json<T: serde::Serialize>(value: &T, maximum: usize) -> ApiResult<String> {
    let mut output = LimitedBuffer {
        bytes: Vec::new(),
        maximum,
    };
    serde_json::to_writer(&mut output, value)?;
    // serde_json emits valid UTF-8; retain a typed error if that invariant fails.
    String::from_utf8(output.bytes).map_err(|_| {
        crate::PhyError::InvalidInput {
            field: "report.utf8",
        }
        .into()
    })
}
