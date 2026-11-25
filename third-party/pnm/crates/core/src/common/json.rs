/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded JSON serialization for any serializable value.
use serde::Serialize;
use std::io::{self, Write};

/// Default maximum JSON output size (16 MiB), independent of parse limits.
pub const DEFAULT_MAX_JSON_BYTES: usize = 16 * 1024 * 1024;

struct BoundedOutput {
    bytes: Vec<u8>,
    limit: usize,
}
impl Write for BoundedOutput {
    /// Appends one serialized chunk within the output budget.
    ///
    /// # Arguments
    /// * `bytes` - Complete chunk to append; partial writes are not performed.
    /// # Returns
    /// The appended byte count.
    /// # Errors
    /// Returns an I/O error before appending if the chunk exceeds the budget.
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.limit.saturating_sub(self.bytes.len()) {
            return Err(io::Error::other("JSON output limit exceeded"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    /// Completes an in-memory flush without external I/O.
    ///
    /// # Returns
    /// Success; all accepted bytes are already in memory.
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
/// Serializes a model as pretty JSON with a strict output-byte limit.
///
/// # Arguments
/// * `model` - Serializable output view; no input data is modified.
/// * `max_bytes` - Maximum UTF-8 bytes to retain, including indentation.
/// # Returns
/// The complete JSON string; partial output is discarded on failure.
/// # Errors
/// Returns a Serde error for serialization failure or exceeded output budget.
pub fn to_json(model: &impl Serialize, max_bytes: usize) -> serde_json::Result<String> {
    let mut output = BoundedOutput {
        bytes: Vec::new(),
        limit: max_bytes,
    };
    serde_json::to_writer_pretty(&mut output, model)?;
    String::from_utf8(output.bytes).map_err(|error| serde_json::Error::io(io::Error::other(error)))
}
