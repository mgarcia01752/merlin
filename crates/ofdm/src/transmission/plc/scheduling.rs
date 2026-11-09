/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Fixed acquisition repeats and MC segmentation, MULPI I25 §6.5.4/AnnexB.
use super::{
    descriptors::FixedPlcDescriptors, geometry::TIMESTAMP_BLOCK_BYTES, MessageChannelBlock,
    PlcFrameGeometry,
};
use crate::{
    error::invalid,
    transmission::{settings::SAMPLE_RATE_HZ, TransmissionSettings},
    types::PLC_FRAME_SYMBOLS,
    LabError,
};
/// AnnexB default200ms repetition period measured in204.8MHz sample ticks.
pub const DESCRIPTOR_REPEAT_SAMPLES: u64 = SAMPLE_RATE_HZ as u64 / 5;
/// Immutable MC result for a single consecutive PLC frame.
#[derive(Debug, Clone)]
pub struct ScheduledMessageChannel {
    pub frame_index: u64,
    pub block: MessageChannelBlock,
}
/// Bounded fixed packet cycle; state persists across MC frames and repeats.
#[derive(Clone)]
pub struct PlcMessageScheduler {
    packets: Vec<Vec<u8>>,
    packet_index: usize,
    packet_offset: usize,
    frame_index: u64,
    next_repeat: u64,
    repeat_frames: u64,
    mc_bytes: usize,
}
impl PlcMessageScheduler {
    /// Freezes a bounded acquisition set and sample-clock repetition schedule.
    /// # Arguments
    /// * `settings` - Channel symbol cadence, including cyclic prefix.
    /// * `descriptors` - Three immutable fixed acquisition packets for this channel.
    /// # Returns
    /// Scheduler beginning at frame0, with starts no later than every200ms.
    /// # Errors
    /// Rejects invalid settings or a packet cycle that cannot fit the repeat period.
    pub fn new(
        settings: &TransmissionSettings,
        descriptors: &FixedPlcDescriptors,
    ) -> Result<Self, LabError> {
        if settings != descriptors.settings() {
            return Err(invalid(
                "PLC scheduler settings differ from its frozen descriptors.",
            ));
        }
        let timing = settings.one_second_timing()?;
        let frame_samples = u64::from(timing.symbol_period_samples) * PLC_FRAME_SYMBOLS;
        let repeat_frames = DESCRIPTOR_REPEAT_SAMPLES / frame_samples;
        let mc_bytes = PlcFrameGeometry::new(settings.subcarrier_spacing).payload_bytes
            - TIMESTAMP_BLOCK_BYTES;
        let packets: Vec<_> = descriptors
            .packets()
            .iter()
            .map(|packet| packet.bytes().to_vec())
            .collect();
        let batch_bytes: usize = packets.iter().map(Vec::len).sum();
        let required_frames = batch_bytes.div_ceil(mc_bytes - 3) as u64;
        if repeat_frames == 0 || required_frames > repeat_frames {
            return Err(invalid(
                "PLC acquisition set exceeds its200ms repetition budget.",
            ));
        }
        Ok(Self {
            packet_index: packets.len(),
            packets,
            packet_offset: 0,
            frame_index: 0,
            next_repeat: 0,
            repeat_frames,
            mc_bytes,
        })
    }
    /// Reports the frozen whole-frame repeat period.
    /// # Returns
    /// Positive frame count; rounding is toward an earlier transmission deadline.
    pub fn repeat_frames(&self) -> u64 {
        self.repeat_frames
    }
    /// Produces one complete MC block and advances only after validation succeeds.
    /// # Returns
    /// Exact-capacity MC with continuation/new-start pointer and0xff idle fill.
    /// # Errors
    /// Rejects counter overflow or unexpected segment validation without advancing.
    pub fn next_block(&mut self) -> Result<ScheduledMessageChannel, LabError> {
        let mut candidate = self.clone();
        let following = candidate
            .frame_index
            .checked_add(1)
            .ok_or_else(|| invalid("PLC scheduler frame counter overflow."))?;
        if candidate.packet_index == candidate.packets.len()
            && candidate.frame_index >= candidate.next_repeat
        {
            candidate.packet_index = 0;
            candidate.packet_offset = 0;
            candidate.next_repeat = candidate
                .frame_index
                .checked_add(candidate.repeat_frames)
                .ok_or_else(|| invalid("PLC repeat index overflow."))?;
        }
        let mut segment = Vec::with_capacity(candidate.mc_bytes - 1);
        let mut pointer = None;
        let budget = candidate.mc_bytes - 3;
        while segment.len() < budget && candidate.packet_index < candidate.packets.len() {
            if candidate.packet_offset == 0 && pointer.is_none() {
                pointer = Some(segment.len() as u16);
            }
            let packet = &candidate.packets[candidate.packet_index];
            let count = (budget - segment.len()).min(packet.len() - candidate.packet_offset);
            segment.extend_from_slice(
                &packet[candidate.packet_offset..candidate.packet_offset + count],
            );
            candidate.packet_offset += count;
            if candidate.packet_offset == packet.len() {
                candidate.packet_index += 1;
                candidate.packet_offset = 0;
            }
        }
        let segment_bytes = candidate.mc_bytes - 1 - 2 * usize::from(pointer.is_some());
        segment.resize(segment_bytes, 0xff);
        let output = ScheduledMessageChannel {
            frame_index: candidate.frame_index,
            block: MessageChannelBlock::new(&segment, pointer)?,
        };
        candidate.frame_index = following;
        *self = candidate;
        Ok(output)
    }
}
