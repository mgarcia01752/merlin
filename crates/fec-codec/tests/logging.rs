/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! One test process owns its logger; library setup must not claim global state.
use fec_codec::plc::PlcFecCodec;
use std::sync::Mutex;
struct Recorder(Mutex<Vec<(log::Level, String)>>);
static RECORDS: Recorder = Recorder(Mutex::new(Vec::new()));
impl log::Log for Recorder {
    /// Allows all records through the test sink's application-owned filter.
    /// # Arguments
    /// * `_metadata` - Facade level/target; payload data is never supplied here.
    /// # Returns
    /// True; log's global maximum still controls dispatch.
    fn enabled(&self, _metadata: &log::Metadata<'_>) -> bool {
        true
    }
    /// Copies scalar diagnostic messages for assertions in this isolated process.
    /// # Arguments
    /// * `record` - Application-filtered immutable record.
    /// # Returns
    /// Unit after retaining the level and formatted diagnostic.
    fn log(&self, record: &log::Record<'_>) {
        self.0
            .lock()
            .unwrap()
            .push((record.level(), record.args().to_string()));
    }
    /// Completes this memory-only sink without external I/O.
    /// # Returns
    /// Unit; no buffered file or stream exists.
    fn flush(&self) {}
}
#[test]
fn library_leaves_logger_ownership_to_consumer_and_logging_does_not_change_codec() {
    let mut codec = PlcFecCodec::new().unwrap(); // No logger is installed by setup.
    let information = [1; 288];
    let before = codec.encode(&information).unwrap();
    log::set_logger(&RECORDS).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    assert_eq!(codec.encode(&information).unwrap(), before);
    assert!(codec.encode(&[2; 288]).is_err());
    let bad_llrs: Vec<_> = (0..384)
        .map(|index| {
            if (index * 17 + index / 3) % 7 < 3 {
                20.0
            } else {
                -20.0
            }
        })
        .collect();
    let failed = codec.decode(&bad_llrs, 1).unwrap();
    assert!(!failed.success);
    let llrs: Vec<_> = before
        .iter()
        .map(|bit| if *bit == 0 { 20.0 } else { -20.0 })
        .collect();
    let enabled = codec.decode(&llrs, 100).unwrap();
    log::set_max_level(log::LevelFilter::Off);
    let disabled = codec.decode(&llrs, 100).unwrap();
    assert_eq!(enabled.information, disabled.information);
    assert_eq!(enabled.mother_codeword, disabled.mother_codeword);
    assert_eq!(enabled.iterations, disabled.iterations);
    assert_eq!(enabled.success, disabled.success);
    let records = RECORDS.0.lock().unwrap();
    assert!(records
        .iter()
        .any(|(_, message)| message.contains("binary-input-rejected")));
    assert!(records
        .iter()
        .any(|(_, message)| message.contains("plc-ldpc-not-converged")));
    assert!(records.iter().all(|(level, _)| *level == log::Level::Debug));
}
