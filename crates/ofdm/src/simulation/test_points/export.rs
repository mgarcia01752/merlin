/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded versioned JSONL adapter for already-selected typed observations.
use super::{ProbeEvent, ProbeObserver};
use crate::{error::invalid, LabError};
use std::io::Write;
/// Application bound on one selected serialized event, including metadata.
pub const MAX_EXPORT_RECORD_BYTES: usize = 1_048_576;
/// Application bound on one export stream, independent of simulation duration.
pub const MAX_EXPORT_BYTES: u64 = 64 * 1_048_576;
/// Owns a caller-provided writer; no filesystem or scientific-stage ownership.
pub struct JsonlProbeExporter<Writer: Write> {
    writer: Writer,
    max_bytes: u64,
    bytes_written: u64,
    failed: bool,
}
impl<Writer: Write> JsonlProbeExporter<Writer> {
    /// Creates a finite export adapter without writing or opening any path.
    /// # Arguments
    /// * `writer` - Caller-owned output destination, moved into the adapter.
    /// * `max_bytes` - Positive byte budget, at most MAX_EXPORT_BYTES.
    /// # Returns
    /// Empty adapter; each accepted event has a versioned JSON envelope and newline.
    /// # Errors
    /// Rejects invalid budgets before touching the writer.
    pub fn new(writer: Writer, max_bytes: u64) -> Result<Self, LabError> {
        if !(1..=MAX_EXPORT_BYTES).contains(&max_bytes) {
            return Err(invalid("Invalid probe export byte budget."));
        }
        Ok(Self {
            writer,
            max_bytes,
            bytes_written: 0,
            failed: false,
        })
    }
    /// Reports bytes successfully written as complete records.
    /// # Returns
    /// Complete-record byte count; an I/O failure may leave an incomplete final record.
    pub fn bytes_written(&self) -> u64 {
        self.bytes_written
    }
    /// Flushes the caller's writer and returns ownership.
    /// # Returns
    /// The original writer after a successful flush.
    /// # Errors
    /// Reports terminal export/flush failures; failed streams cannot be resumed.
    pub fn finish(mut self) -> Result<Writer, LabError> {
        if self.failed {
            return Err(LabError::new(
                "ProbeExportClosed",
                "Probe export failed; inspect its partial prefix.",
            ));
        }
        self.writer.flush().map_err(export_io)?;
        Ok(self.writer)
    }
}
struct RecordBuffer(Vec<u8>);
impl Write for RecordBuffer {
    /// Extends one record buffer while reserving space for its final newline.
    /// # Arguments
    /// * `bytes` - Next serializer fragment; total record bytes remain bounded.
    /// # Returns
    /// Consumed fragment length without a partial append.
    /// # Errors
    /// Rejects record-size overflow before changing the buffer.
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.0.len().saturating_add(bytes.len()) >= MAX_EXPORT_RECORD_BYTES {
            return Err(std::io::Error::other(
                "Probe record exceeds the byte limit.",
            ));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    /// Completes a serializer flush for an in-memory record.
    /// # Returns
    /// Unit; persistence belongs to the caller-owned destination writer.
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn export_io(error: std::io::Error) -> LabError {
    LabError::new("ProbeExportIo", format!("Probe output failed: {error}"))
}
impl<Writer: Write> ProbeObserver for JsonlProbeExporter<Writer> {
    /// Serializes one selected immutable event without changing scientific values.
    /// # Arguments
    /// * `event` - Validated finite view supplied by TestPointProbe.
    /// # Returns
    /// Unit after writing one complete schema-version1 JSONL record.
    /// # Errors
    /// Rejects terminal state, record/stream byte limits or I/O failure. Limit failures
    /// occur before writer mutation; I/O failure may leave a partial final record.
    fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), LabError> {
        if self.failed {
            return Err(LabError::new(
                "ProbeExportClosed",
                "Probe export is closed after failure.",
            ));
        }
        #[derive(serde::Serialize)]
        struct Envelope<'event, 'data> {
            schema_version: u8,
            event: &'event ProbeEvent<'data>,
        }
        let mut bytes = RecordBuffer(Vec::new());
        let encoded = serde_json::to_writer(
            &mut bytes,
            &Envelope {
                schema_version: 1,
                event,
            },
        );
        if encoded.is_err() || bytes.0.len() as u64 + 1 > self.max_bytes - self.bytes_written {
            self.failed = true;
            return Err(LabError::new(
                "ProbeExportLimit",
                "Probe export record or stream byte limit exceeded.",
            ));
        }
        bytes.0.push(b'\n');
        if let Err(error) = self.writer.write_all(&bytes.0) {
            self.failed = true;
            return Err(export_io(error));
        }
        self.bytes_written += bytes.0.len() as u64;
        Ok(())
    }
}
