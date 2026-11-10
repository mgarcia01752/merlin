/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Message inspection reuses the validated PLC frame, MAC and descriptor decoders.
use crate::closed_loop::model::latest;
use docsis_ofdm_fec::transmission::plc::{
    descriptors, mac_packets::OCD_MESSAGE_TYPE, reassembly::PlcMessageReassembler, PlcMessageFrame,
};
use serde_json::{json, Value};

fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn completed_frame(first_symbol: u64, stride: u64, transmitted_samples: u64) -> bool {
    stride > 0
        && first_symbol
            .checked_add(128)
            .and_then(|end| end.checked_mul(stride))
            .is_some_and(|end| transmitted_samples >= end)
}

fn tlv_title(ocd: bool, kind: u8) -> &'static str {
    if !ocd {
        return "Subcarrier Assignment";
    }
    match kind {
        0 => "DFT Size",
        1 => "Cyclic Prefix",
        2 => "Roll-off",
        3 => "Subcarrier Zero Frequency",
        4 => "Time Interleaving Depth",
        5 => "Subcarrier Assignment",
        6 => "Primary Capable",
        _ => "Unknown",
    }
}

/// Reassembles finite observed TX PLC frames and presents their actual MAC packets.
/// # Returns
/// At most32 packets with bounded TLV trees and exact bytes, or an honest empty state.
/// Completed-frame status uses actual transmitted samples, including partial runs.
pub(crate) fn messages() -> Value {
    let Some(result) = latest() else {
        return json!({"messages":[],"reason":"Use Start on the main workbench to capture OCD / DPD messages."});
    };
    let mut reassembler = PlcMessageReassembler::new();
    let mut records = Vec::new();
    let mut error = String::new();
    let stride = result
        .job
        .settings
        .transmission
        .one_second_timing()
        .map(|t| u64::from(t.symbol_period_samples))
        .unwrap_or(0);
    for (symbol, bytes) in &result.observations.tx.frames {
        let mut parse = || {
            let frame = PlcMessageFrame::from_bytes(
                result.job.settings.transmission.subcarrier_spacing,
                bytes,
            )?;
            reassembler.push(frame.message_channel())
        };
        let packets = match parse() {
            Ok(p) => p,
            Err(e) => {
                error = e.to_string();
                break;
            }
        };
        for packet in packets {
            if records.len() >= 32 {
                break;
            }
            let parsed = match descriptors::parse(&packet) {
                Ok(p) => p,
                Err(e) => {
                    error = e.to_string();
                    continue;
                }
            };
            let ocd = packet.message_type() == OCD_MESSAGE_TYPE;
            let name = if ocd {
                "OCD".to_string()
            } else {
                format!(
                    "DPD · {}",
                    parsed
                        .profile_id
                        .map(|p| if p == 255 {
                            "NCP".into()
                        } else {
                            format!("Profile {p}")
                        })
                        .unwrap_or_default()
                )
            };
            let complete = completed_frame(
                *symbol,
                stride,
                result.job.summary.total_transmitted_samples,
            );
            let mut offset =
                packet.bytes().len() - packet.body().len() - 4 + if ocd { 2 } else { 3 };
            let tlvs: Vec<_> = parsed.tlvs.iter().map(|tlv| {
                let title = tlv_title(ocd,tlv.kind);
                let row = json!({"name":title,"kind":tlv.kind,"length":tlv.value.len(),"offset":offset,"hex":hex(&tlv.value)});
                offset += 2+tlv.value.len(); row
            }).collect();
            let bytes = packet.bytes();
            let hex_rows: Vec<_> = bytes
                .chunks(16)
                .enumerate()
                .map(|(i, chunk)| json!({"offset":format!("{:04X}",i*16),"hex":hex(chunk)}))
                .collect();
            records.push(json!({"name":name,"type":packet.message_type(),"channel":parsed.channel_id,"profile":parsed.profile_id,"change_count":parsed.change_count,"source":packet.source_address().iter().map(|b|format!("{b:02x}")).collect::<Vec<_>>().join(":"),"bytes":bytes.len(),"symbol":symbol,"completed_frame":complete,"status":if complete {"Carried in completed TX PLC frame"} else {"Scheduled · TX PLC frame incomplete"},"integrity":"HCS / FCS valid","tlvs":tlvs,"hex_rows":hex_rows}));
        }
    }
    json!({"messages":records,"pending_bytes":reassembler.pending().len(),"frames":result.observations.tx.frames.len(),"reason":if !error.is_empty(){error.as_str()} else if records.is_empty(){"No complete OCD / DPD packet captured."} else {""},"capture_limited":result.observations.tx.frames.len()>=64 || records.len()>=32,"limit":32})
}

#[cfg(test)]
mod tests {
    use super::completed_frame;

    #[test]
    fn scheduled_frame_is_not_claimed_emitted_until_its_last_sample() {
        assert!(!completed_frame(128, 8704, 256 * 8704 - 1));
        assert!(completed_frame(128, 8704, 256 * 8704));
        assert!(!completed_frame(128, 0, u64::MAX));
        assert!(!completed_frame(u64::MAX, 8704, u64::MAX));
    }
}
