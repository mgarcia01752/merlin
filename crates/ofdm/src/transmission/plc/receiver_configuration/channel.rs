/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Reconstructs received OCD geometry using the single physical-layout validator.
use super::super::descriptors::DescriptorTlv;
use crate::{
    error::invalid,
    transmission::{
        layout::{CarrierLayout, CarrierLayoutSettings, PhysicalCarrierRole},
        settings::{CYCLIC_PREFIX_SAMPLES, ROLL_OFF_SAMPLES},
        SubcarrierSpacing, TransmissionSettings,
    },
    types::FrequencyHz,
    LabError,
};
use serde::Serialize;
const ASSIGNMENT_TLV: u8 = 5;
const CONTINUOUS_PILOT: u8 = 1;
const EXCLUDED: u8 = 16;
const PLC: u8 = 20;
/// Receiver-owned physical settings reconstructed solely from received OCD TLVs.
#[derive(Debug, Clone, Serialize)]
pub struct ReceivedChannel {
    transmission: TransmissionSettings,
    layout: CarrierLayoutSettings,
    primary_capable: bool,
}
impl ReceivedChannel {
    /// Borrows settings decoded from OCD, with an inferred envelope bandwidth.
    /// # Returns
    /// Fixed spacing, CP, rolloff and depth. Occupied bandwidth is the smallest
    /// allowed 6 MHz-multiple envelope covering active bins, not an OCD wire field.
    pub fn transmission(&self) -> &TransmissionSettings {
        &self.transmission
    }
    /// Borrows received frequency origin and protected physical assignments.
    /// # Returns
    /// Immutable geometry; no transmitter-owned layout is retained.
    pub fn layout_settings(&self) -> &CarrierLayoutSettings {
        &self.layout
    }
    /// Reconstructs a validated physical map for a consumer without sharing TX state.
    /// # Returns
    /// Owned immutable layout with phase-dependent pilot roles available.
    /// # Errors
    /// Propagates physical validation failures; fields cannot be edited through this model.
    pub fn physical_layout(&self) -> Result<CarrierLayout, LabError> {
        CarrierLayout::new(&self.transmission, &self.layout)
    }
    /// Reports whether the received OCD advertises primary capability.
    /// # Returns
    /// The decoded flag; it does not assert modem registration or service readiness.
    pub fn primary_capable(&self) -> bool {
        self.primary_capable
    }
    /// Reports the inferred minimum occupied envelope used for validation.
    /// # Returns
    /// Hz rounded up to a permitted 6 MHz step, with a 24 MHz minimum.
    pub fn minimum_occupied_bandwidth_hz(&self) -> FrequencyHz {
        self.transmission.occupied_bandwidth_hz
    }
}
// Reuse the physical-layout validator instead of maintaining a second set of
// PLC-neighborhood/pilot/collision rules for received descriptor TLVs.
/// Decodes already structurally validated OCD TLVs into receiver-owned geometry.
/// # Arguments
/// * `tlvs` - Collector/parser-owned TLVs with checked field lengths and range order.
/// # Returns
/// Validated channel settings and assignments without any transmitter object.
/// # Errors
/// Rejects semantic parameters, protected-role conflicts or invalid physical geometry.
pub(crate) fn decode(tlvs: &[DescriptorTlv]) -> Result<ReceivedChannel, LabError> {
    let value = |kind| {
        tlvs.iter()
            .find(|tlv| tlv.kind == kind)
            .map(|tlv| tlv.value.as_slice())
            .ok_or_else(|| invalid("Missing mandatory OCD setting."))
    };
    let spacing = match value(0)?[0] {
        0 => SubcarrierSpacing::Khz50,
        1 => SubcarrierSpacing::Khz25,
        _ => return Err(invalid("Unsupported OCD FFT size.")),
    };
    let prefix = *CYCLIC_PREFIX_SAMPLES
        .get(usize::from(value(1)?[0]))
        .ok_or_else(|| invalid("Reserved OCD cyclic prefix."))?;
    let roll = *ROLL_OFF_SAMPLES
        .get(usize::from(value(2)?[0]))
        .ok_or_else(|| invalid("Reserved OCD rolloff."))?;
    let origin = value(3)?;
    let zero = u32::from_be_bytes([origin[0], origin[1], origin[2], origin[3]]);
    if zero == 0 || zero % spacing.frequency_hz() != 0 || value(6)?[0] > 1 {
        return Err(invalid(
            "Invalid OCD frequency origin or primary-capability value.",
        ));
    }
    let mut functions = vec![0; spacing.fft_samples() as usize];
    for tlv in tlvs.iter().filter(|tlv| tlv.kind == ASSIGNMENT_TLV) {
        let first = usize::from(u16::from_be_bytes([tlv.value[1], tlv.value[2]]));
        let last = usize::from(u16::from_be_bytes([tlv.value[3], tlv.value[4]]));
        if ![CONTINUOUS_PILOT, EXCLUDED, PLC].contains(&tlv.value[0])
            || last >= functions.len()
            || functions[first..=last].iter().any(|role| *role != 0)
        {
            return Err(invalid("Invalid or overlapping fixed OCD assignments."));
        }
        functions[first..=last].fill(tlv.value[0]);
    }
    let edge = spacing.mandatory_excluded_edge_bins();
    if functions[..edge]
        .iter()
        .chain(functions[functions.len() - edge..].iter())
        .any(|role| *role != EXCLUDED)
    {
        return Err(invalid("Received OCD omits mandatory excluded FFT edges."));
    }
    let first = functions
        .iter()
        .position(|role| *role != EXCLUDED)
        .ok_or_else(|| invalid("OCD excludes the entire channel."))?;
    let end = functions
        .iter()
        .rposition(|role| *role != EXCLUDED)
        .ok_or_else(|| invalid("Missing active OCD envelope."))?
        + 1;
    let plc_first = functions
        .iter()
        .position(|role| *role == PLC)
        .ok_or_else(|| invalid("OCD is missing PLC carriers."))?;
    let width = super::super::PlcFrameGeometry::new(spacing).plc_subcarriers;
    let left = plc_first
        .checked_sub(7 * width)
        .ok_or_else(|| invalid("OCD PLC neighborhood underflow."))?;
    let right = plc_first + 8 * width;
    use crate::transmission::settings::{CHANNEL_BANDWIDTH_STEP_HZ, MIN_CHANNEL_BANDWIDTH_HZ};
    let bandwidth = ((end - first) as u32 * spacing.frequency_hz())
        .div_ceil(CHANNEL_BANDWIDTH_STEP_HZ)
        * CHANNEL_BANDWIDTH_STEP_HZ;
    let settings = TransmissionSettings {
        occupied_bandwidth_hz: bandwidth.max(MIN_CHANNEL_BANDWIDTH_HZ),
        subcarrier_spacing: spacing,
        cyclic_prefix_samples: prefix,
        roll_off_samples: roll,
        time_interleaver_depth: value(4)?[0],
    };
    let layout = crate::transmission::layout::CarrierLayoutSettings {
        zero_frequency_hz: zero,
        first_active_bin: first as u16,
        active_end_exclusive: end as u16,
        plc_first_bin: plc_first as u16,
        excluded_bins: (first..end)
            .filter(|bin| functions[*bin] == EXCLUDED)
            .map(|bin| bin as u16)
            .collect(),
        additional_continuous_pilots: functions
            .iter()
            .enumerate()
            .filter(|(bin, role)| **role == CONTINUOUS_PILOT && !((left..right).contains(bin)))
            .map(|(bin, _)| bin as u16)
            .collect(),
    };
    let validated = CarrierLayout::new(&settings, &layout)?;
    for (expected, actual) in functions.iter().zip(validated.static_roles()) {
        let code = match actual {
            PhysicalCarrierRole::Excluded => EXCLUDED,
            PhysicalCarrierRole::ContinuousPilot => CONTINUOUS_PILOT,
            PhysicalCarrierRole::Plc => PLC,
            _ => 0,
        };
        if *expected != code {
            return Err(invalid(
                "Received OCD assignments disagree with the physical PLC layout.",
            ));
        }
    }
    Ok(ReceivedChannel {
        transmission: settings,
        layout,
        primary_capable: value(6)?[0] == 1,
    })
}
