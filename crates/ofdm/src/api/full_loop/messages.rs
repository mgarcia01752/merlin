/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual retained TX message inspection; generated construction data is not an emission claim.
use super::*;
pub use crate::transmission::plc::descriptors::inspection::{
    DescriptorField as FullLoopDescriptorField, DescriptorInspection as FullLoopDescriptor,
    DescriptorMeaning as FullLoopDescriptorMeaning,
};
use crate::transmission::plc::{
    descriptors::inspection::inspect, reassembly::PlcMessageReassembler, PlcMessageFrame,
};
use serde::Serialize;

/// One CRC-validated actual TX packet and decoded fixed descriptor, never a received MAC claim.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopPlcMessage {
    pub message_type: u8,
    pub source_address: [u8; 6],
    pub packet_bytes: Vec<u8>,
    pub descriptor: FullLoopDescriptor,
    pub last_frame_context: FullLoopStageContext,
    pub last_frame_end_sample: u64,
    /// True only if actual transmitted samples reach the last carrying frame's end.
    pub completely_transmitted: bool,
}
/// First missing/invalid retained frame; earlier complete packets remain available.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopMessageIssue {
    pub retained_frame: usize,
    pub error: ApiError,
}
/// Bounded actual construction-message report with explicit reassembly and transmission coverage.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopPlcMessages {
    pub reference_plane: &'static str,
    pub retained_frames: usize,
    pub processed_frames: usize,
    pub pending_bytes: usize,
    pub packet_limit_reached: bool,
    pub frame_limit_reached: bool,
    pub messages: Vec<FullLoopPlcMessage>,
    pub issue: Option<FullLoopMessageIssue>,
}

impl FullLoopResult {
    /// Inspects actual retained TX PLC messages through the existing frame/MAC/descriptor owners.
    /// # Returns
    /// At most 32 CRC-validated complete packets from at most 64 consecutive retained frames,
    /// preserving exact bytes, typed decoded TLVs/offsets and last carrying context/coverage.
    /// These are TX construction messages; transmission is reported separately per packet.
    /// Missing/invalid later frames stop reassembly with an issue and preserve earlier packets.
    /// # Errors
    /// MissingCoverage when no actual PLC message event was retained; invalid accepted timing
    /// propagates. No hidden run, message regeneration or bridging of missing segments occurs.
    pub fn plc_messages(&self) -> ApiResult<FullLoopPlcMessages> {
        let frames = self.stages.events(FullLoopStage::PlcMessage)?;
        inspect_frames(
            frames,
            &self.job.settings.transmission,
            self.job.summary.total_transmitted_samples,
        )
    }
}

fn inspect_frames(
    frames: &[FullLoopStageSnapshot],
    transmission: &TransmissionSettings,
    total_transmitted_samples: u64,
) -> ApiResult<FullLoopPlcMessages> {
    let timing = transmission.one_second_timing()?;
    let stride = u64::from(timing.symbol_period_samples);
    let spacing = transmission.subcarrier_spacing;
    let mut reassembler = PlcMessageReassembler::new();
    let mut report = FullLoopPlcMessages {
        reference_plane: "tx_plc_message_construction",
        retained_frames: frames.len(),
        processed_frames: 0,
        pending_bytes: 0,
        packet_limit_reached: false,
        frame_limit_reached: frames.len() > 64,
        messages: Vec::new(),
        issue: None,
    };
    let mut expected_symbol = None;
    for (index, event) in frames.iter().take(64).enumerate() {
        let parsed = (|| -> ApiResult<_> {
            let symbol = event.context.channel_symbol_index.ok_or_else(|| {
                super::super::error::invalid("PLC frame has no original channel-symbol origin")
            })?;
            if symbol % crate::types::PLC_FRAME_SYMBOLS != 0
                || expected_symbol.is_some_and(|expected| expected != symbol)
            {
                return Err(ApiError::new(
                    ErrorCode::MissingCoverage,
                    "Retained PLC frame sequence has a missing or invalid frame origin",
                ));
            }
            let end_sample = frame_end_sample(symbol, stride)
                .ok_or_else(|| super::super::error::invalid("PLC frame sample extent overflows"))?;
            let FullLoopStageData::Bytes(bytes) = &event.data else {
                return Err(super::super::error::invalid(
                    "Retained PLC message is not bytes",
                ));
            };
            if event.element_start != 0 || bytes.len() != event.total_elements {
                return Err(ApiError::new(
                    ErrorCode::MissingCoverage,
                    "Retained PLC message frame is incomplete",
                ));
            }
            let frame = PlcMessageFrame::from_bytes(spacing, bytes)?;
            let packets = reassembler.push(frame.message_channel())?;
            expected_symbol = symbol.checked_add(crate::types::PLC_FRAME_SYMBOLS);
            Ok((packets, end_sample))
        })();
        let (packets, end_sample) = match parsed {
            Ok(value) => value,
            Err(error) => {
                report.issue = Some(FullLoopMessageIssue {
                    retained_frame: index,
                    error,
                });
                break;
            }
        };
        report.processed_frames += 1;
        for packet in packets {
            if report.messages.len() == 32 {
                report.packet_limit_reached = true;
                break;
            }
            let descriptor = match inspect(&packet) {
                Ok(value) => value,
                Err(error) => {
                    report.issue = Some(FullLoopMessageIssue {
                        retained_frame: index,
                        error: error.into(),
                    });
                    break;
                }
            };
            report.messages.push(FullLoopPlcMessage {
                message_type: packet.message_type(),
                source_address: packet.source_address(),
                packet_bytes: packet.bytes().to_vec(),
                descriptor,
                last_frame_context: event.context.clone(),
                last_frame_end_sample: end_sample,
                completely_transmitted: total_transmitted_samples >= end_sample,
            });
        }
        if report.messages.len() == 32 {
            report.packet_limit_reached = true;
        }
        if report.issue.is_some() || report.packet_limit_reached {
            break;
        }
    }
    report.pending_bytes = reassembler.pending().len();
    Ok(report)
}

fn frame_end_sample(first_symbol: u64, stride: u64) -> Option<u64> {
    if stride == 0 {
        return None;
    }
    first_symbol
        .checked_add(crate::types::PLC_FRAME_SYMBOLS)?
        .checked_mul(stride)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transmission::plc::{
        geometry::{PlcFrameGeometry, TIMESTAMP_BLOCK_BYTES},
        mac_packets::ManagementPacket,
        ExtendedTimestamp, MessageChannelBlock,
    };
    fn frame(spacing: SubcarrierSpacing, symbol: u64, packet: bool) -> FullLoopStageSnapshot {
        let capacity = PlcFrameGeometry::new(spacing).payload_bytes - TIMESTAMP_BLOCK_BYTES;
        let block = if packet {
            // Independent fixed DPD: channel 7, profile 0, count 3, default 8 bits on bins 0..31.
            let packet = ManagementPacket::new(
                [0, 0xca, 0, 0, 0, 1],
                50,
                &[7, 0, 3, 5, 5, 0x28, 0, 0, 0, 31],
            )
            .unwrap();
            let mut bytes = packet.bytes().to_vec();
            bytes.resize(capacity - 3, 0xff);
            MessageChannelBlock::new(&bytes, Some(0)).unwrap()
        } else {
            MessageChannelBlock::idle(capacity).unwrap()
        };
        let bytes = PlcMessageFrame::new(spacing, ExtendedTimestamp::new(0, 0, 0).unwrap(), block)
            .unwrap()
            .to_bytes();
        let mut context =
            FullLoopStageContext::new("packet-vector", "wire-bytes", FullLoopStagePhase::Warmup)
                .unwrap();
        context.channel_symbol_index = Some(symbol);
        context.plc_phase = Some(0);
        FullLoopStageSnapshot {
            stage: FullLoopStage::PlcMessage,
            retained_index: 0,
            direction: SignalDirection::Transmit,
            context,
            domain: SignalDomain::MessageByte,
            units: ProbeUnits::Octet,
            element_start: 0,
            total_elements: bytes.len(),
            data: FullLoopStageData::Bytes(bytes),
        }
    }

    #[test]
    fn fixed_packet_fields_offsets_and_exact_transmission_end_are_independent_anchors() {
        for (spacing, stride) in [
            (SubcarrierSpacing::Khz25, 8704),
            (SubcarrierSpacing::Khz50, 4608),
        ] {
            let transmission = TransmissionSettings {
                subcarrier_spacing: spacing,
                ..Default::default()
            };
            let frames = [frame(spacing, 128, true)];
            let end = 256 * stride;
            let scheduled = inspect_frames(&frames, &transmission, end - 1).unwrap();
            assert_eq!(scheduled.messages.len(), 1);
            assert!(!scheduled.messages[0].completely_transmitted);
            let emitted = inspect_frames(&frames, &transmission, end).unwrap();
            let packet = &emitted.messages[0];
            assert!(packet.completely_transmitted);
            assert_eq!(packet.last_frame_end_sample, end);
            assert_eq!(packet.source_address, [0, 0xca, 0, 0, 0, 1]);
            assert_eq!(packet.message_type, 50);
            assert_eq!(packet.descriptor.channel_id, 7);
            assert_eq!(packet.descriptor.profile_id, Some(0));
            assert_eq!(packet.descriptor.change_count, 3);
            assert_eq!(
                &packet.packet_bytes[26..packet.packet_bytes.len() - 4],
                &[7, 0, 3, 5, 5, 0x28, 0, 0, 0, 31]
            );
            let field = &packet.descriptor.fields[0];
            assert_eq!(field.offset, 29);
            assert_eq!(field.value, [0x28, 0, 0, 0, 31]);
            assert!(matches!(
                field.meaning,
                FullLoopDescriptorMeaning::ProfileAssignment {
                    default: true,
                    bits_per_carrier: 8,
                    modulation_order: Some(256),
                    first: 0,
                    last: 31
                }
            ));
        }
        assert_eq!(frame_end_sample(u64::MAX, 8704), None);
        assert_eq!(frame_end_sample(128, 0), None);
        assert_eq!(frame_end_sample(0, u64::MAX), None);
    }

    #[test]
    fn gaps_truncation_and_invalid_bytes_stop_without_erasing_prior_packets() {
        let transmission = TransmissionSettings::default();
        let spacing = transmission.subcarrier_spacing;
        let first = frame(spacing, 0, true);
        for second in [
            frame(spacing, 256, true),
            {
                let mut event = frame(spacing, 128, true);
                if let FullLoopStageData::Bytes(bytes) = &mut event.data {
                    bytes.truncate(16);
                }
                event
            },
            {
                let mut event = frame(spacing, 128, true);
                if let FullLoopStageData::Bytes(bytes) = &mut event.data {
                    bytes[0] ^= 1;
                }
                event
            },
        ] {
            let report = inspect_frames(&[first.clone(), second], &transmission, u64::MAX).unwrap();
            assert_eq!(report.messages.len(), 1);
            assert_eq!(report.processed_frames, 1);
            assert_eq!(report.issue.as_ref().unwrap().retained_frame, 1);
            assert!(report.messages[0].completely_transmitted);
        }
    }

    #[test]
    fn repeated_valid_frames_obey_packet_and_frame_work_budgets() {
        let transmission = TransmissionSettings::default();
        let spacing = transmission.subcarrier_spacing;
        let frames: Vec<_> = (0..64)
            .map(|index| frame(spacing, index * 128, true))
            .collect();
        let report = inspect_frames(&frames, &transmission, u64::MAX).unwrap();
        assert_eq!(report.messages.len(), 32);
        assert_eq!(report.processed_frames, 32);
        assert!(report.packet_limit_reached);
        assert!(report.issue.is_none());
        let frames: Vec<_> = (0..65)
            .map(|index| frame(spacing, index * 128, false))
            .collect();
        let report = inspect_frames(&frames, &transmission, u64::MAX).unwrap();
        assert_eq!(report.processed_frames, 64);
        assert!(report.frame_limit_reached);
        assert!(report.messages.is_empty());
        assert!(report.issue.is_none());
    }
}
