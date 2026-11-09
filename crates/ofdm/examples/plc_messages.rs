/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Valid TS+idle-MC frame and bounded FEC inputs, before PLC encoding.
use docsis_ofdm_fec::transmission::{
    plc::{ExtendedTimestamp, PlcMessageFrame},
    SubcarrierSpacing,
};
/// Writes actual PLC information bytes and selected pre-encoder data as JSON.
/// # Returns
/// Unit after printing one8K frame's dimensions, message model and ordered inputs.
/// Timestamp is a synthetic explicit reference, not the host clock.
/// # Errors
/// Propagates frame validation, serialization or output failures.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = ExtendedTimestamp::new(0x0000_0102_0304_0506, 19, 10)?;
    let frame = PlcMessageFrame::idle(SubcarrierSpacing::Khz25, timestamp)?;
    let information = frame.information_blocks()?;
    let first = &information[0];
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "experiment": "plc_timestamp_and_idle_channel",
            "geometry": frame.geometry(),
            "message_frame": frame,
            "information_blocks": information,
            "first_block_bits": first.bits().as_slice(),
            "timestamp_block_bytes": &frame.to_bytes()[..12],
        }))?
    );
    Ok(())
}
