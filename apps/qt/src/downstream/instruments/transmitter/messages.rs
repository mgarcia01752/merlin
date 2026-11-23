/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Presentation of authoritative public TX message inspection; never a received-MAC claim.
use crate::downstream::workbench::session::Context;
use crate::downstream::{
    common::constants::{
        MAX_PLC_MESSAGE_RECORDS, MAX_TRANSMITTER_MESSAGE_FRAMES, NCP_MESSAGE_PROFILE_ID,
    },
    simulation::closed_loop::latest,
};
use crate::shared::presentation::bytes::{address, rows};
use merlin::downstream::{ErrorCode, FullLoopPlcMessages};
use serde_json::{json, Value};

fn empty(reason: &str) -> Value {
    json!({"messages":[],"pending_bytes":0,"frames":0,"reason":reason,
        "capture_limited":false,"limit":MAX_PLC_MESSAGE_RECORDS})
}

fn render(report: &FullLoopPlcMessages) -> Value {
    let records = report
        .messages
        .iter()
        .take(MAX_PLC_MESSAGE_RECORDS)
        .map(|packet| {
            let descriptor = &packet.descriptor;
            let ocd = descriptor.profile_id.is_none();
            let name = if ocd {
                "OCD".to_owned()
            } else {
                let profile = match descriptor.profile_id {
                    Some(NCP_MESSAGE_PROFILE_ID) => "NCP".to_owned(),
                    Some(id) => format!("Profile {id}"),
                    None => String::new(),
                };
                format!("DPD · {profile}")
            };
            let tlvs = descriptor
                .fields
                .iter()
                .map(|field| super::tlv_fields::row(ocd, field))
                .collect::<Vec<_>>();
            let complete = packet.completely_transmitted;
            let status = if complete {
                "Carried in completed TX PLC frame"
            } else {
                "Scheduled · TX PLC frame incomplete"
            };
            json!({"name":name,"type":packet.message_type,"channel":descriptor.channel_id,
            "profile":descriptor.profile_id,"change_count":descriptor.change_count,
            "source":address(&packet.source_address),"bytes":packet.packet_bytes.len(),
            "symbol":packet.last_frame_context.channel_symbol_index,"completed_frame":complete,
            "status":status,
            "integrity":"HCS / FCS valid","tlvs":tlvs,"hex_rows":rows(&packet.packet_bytes)})
        })
        .collect::<Vec<_>>();
    let reason = match &report.issue {
        Some(issue) => issue.error.message(),
        None if records.is_empty() => "No complete OCD / DPD packet captured.",
        None => "",
    };
    json!({"messages":records,"pending_bytes":report.pending_bytes,"frames":report.retained_frames,
        "reason":reason,"capture_limited":report.frame_limit_reached || report.packet_limit_reached
            || report.retained_frames>=MAX_TRANSMITTER_MESSAGE_FRAMES
            || report.messages.len()>=MAX_PLC_MESSAGE_RECORDS,"limit":MAX_PLC_MESSAGE_RECORDS})
}

/// Presents actual CRC-validated transmitted packets through the public result owner.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// # Returns
/// At most the existing 32 packet records, bounded TLV trees and exact original bytes.
/// The backend supplies interpretation, original coordinates, pending/issue state
/// and complete-versus-scheduled status; Qt never parses frames or wire codes.
pub(crate) fn messages(context: &Context) -> Value {
    let Some(result) = latest(context) else {
        return json!({"messages":[],"reason":"Use Start on the main workbench to capture OCD / DPD messages."});
    };
    match result.backend().plc_messages() {
        Ok(report) => render(&report),
        Err(cause) if cause.code() == ErrorCode::MissingCoverage => {
            empty("No complete OCD / DPD packet captured.")
        }
        Err(cause) => empty(cause.message()),
    }
}
