/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! MULPI I25 Table28/29 projection; no guessed assignment, bandwidth or epoch.
use super::{ApiError, ApiResult, CaptureFailure, UcdCapture, UcdTlv};
use docsis_ofdma::configuration::ucd::{
    Band, BurstDescriptor, IrPowerControl, OfdmaChannel, ProfileGroup, TimestampSnapshot,
};
use std::collections::BTreeMap;

fn invalid(field: &'static str) -> ApiError {
    docsis_ofdma::PhyError::InvalidInput { field }.into()
}
fn fields<'capture>(
    tlvs: &'capture [UcdTlv],
    allowed: &[u8],
) -> ApiResult<BTreeMap<u8, &'capture [u8]>> {
    let mut result = BTreeMap::new();
    for tlv in tlvs {
        if !allowed.contains(&tlv.kind) {
            return Err(ApiError::Capture {
                category: CaptureFailure::Unsupported,
                field: "ucd.projection_tlv",
                offset: tlv.offset,
            });
        }
        if result.insert(tlv.kind, tlv.value.as_slice()).is_some() {
            return Err(invalid("ucd.duplicate_projection_tlv"));
        }
    }
    Ok(result)
}
fn required<'a>(fields: &BTreeMap<u8, &'a [u8]>, kind: u8, length: usize) -> ApiResult<&'a [u8]> {
    fields
        .get(&kind)
        .copied()
        .filter(|v| v.len() == length)
        .ok_or_else(|| invalid("ucd.required_field_length"))
}
fn integer(value: &[u8]) -> u32 {
    value.iter().fold(0, |n, b| (n << 8) | u32::from(*b))
}
fn optional(fields: &BTreeMap<u8, &[u8]>, kind: u8, length: usize) -> ApiResult<Option<u32>> {
    fields
        .get(&kind)
        .map(|v| {
            if v.len() != length {
                return Err(invalid("ucd.optional_field_length"));
            }
            Ok(integer(v))
        })
        .transpose()
}
fn bands(fields: &BTreeMap<u8, &[u8]>, kind: u8, mandatory: bool) -> ApiResult<Vec<Band>> {
    let Some(value) = fields.get(&kind) else {
        if mandatory {
            return Err(invalid("ucd.required_bands"));
        }
        return Ok(Vec::new());
    };
    if value.is_empty() || value.len() % 4 != 0 {
        return Err(invalid("ucd.band_encoding"));
    }
    Ok(value
        .chunks_exact(4)
        .map(|v| Band {
            first: integer(&v[..2]) as u16,
            last: integer(&v[2..]) as u16,
        })
        .collect())
}

/// Reassembles one complete ordered version5 multipart set without changing inputs.
///
/// `fragments` starts at DA, excludes CRC and contains 1..16 complete messages.
/// Returns a canonical structural view; callers retain every original fragment
/// separately. Rejects missing/duplicate/out-of-order/mismatched parts, split TLVs
/// and expanded payloads over 65549 bytes; no MAC/PCAP inference occurs.
///
/// # Arguments
/// * `fragments` - Ordered complete envelopes; one through sixteen immutable fragments.
///
/// # Returns
/// A derived canonical structural capture; each physical fragment remains caller-owned.
///
/// # Errors
/// Rejects invalid counts, envelopes, order, identities, TLV boundaries or byte limits.
pub fn reassemble_ucd(fragments: &[Vec<u8>]) -> ApiResult<UcdCapture> {
    if fragments.is_empty() || fragments.len() > 16 {
        return Err(invalid("ucd.fragment_count"));
    }
    let mut assembled = Vec::new();
    for (index, fragment) in fragments.iter().enumerate() {
        if fragment.len() < 24 || fragment.len() > 65549 {
            return Err(invalid("ucd.fragment_size"));
        }
        let expected = ((fragments.len() - 1) << 4) | index;
        if usize::from(fragment[19]) != expected {
            return Err(invalid("ucd.fragment_sequence"));
        }
        let mut standalone = fragment.clone();
        standalone[19] = 0;
        UcdCapture::from_mac_management(&standalone)?;
        if index == 0 {
            assembled.extend_from_slice(&standalone[..24]);
        } else if assembled[..12] != fragment[..12] || assembled[20..24] != fragment[20..24] {
            return Err(invalid("ucd.fragment_identity"));
        }
        if assembled.len() + fragment.len() - 24 > 65549 {
            return Err(docsis_ofdma::PhyError::ResourceLimit {
                field: "ucd.reassembled_bytes",
            }
            .into());
        }
        assembled.extend_from_slice(&fragment[24..]);
    }
    let length = (assembled.len() - 14) as u16;
    assembled[12..14].copy_from_slice(&length.to_be_bytes());
    UcdCapture::from_mac_management(&assembled)
}
impl UcdCapture {
    /// Projects all supported channel/burst fields using explicit PHY bandwidth.
    ///
    /// Returns a source-validated channel. `bandwidth_hz` is external PHY geometry;
    /// assignment, grant and simulation timing are still separate caller inputs.
    /// Unknown/duplicate/inapplicable TLVs reject execution rather than disappear.
    ///
    /// # Arguments
    /// * `bandwidth_hz` - Explicit encompassed PHY bandwidth in hertz.
    ///
    /// # Returns
    /// A validated channel model; assignment and grant context remain separate.
    ///
    /// # Errors
    /// Rejects unsupported, duplicate, missing or inconsistent fields and invalid PHY values.
    pub fn project_channel(&self, bandwidth_hz: u32) -> ApiResult<OfdmaChannel> {
        let f = fields(
            self.channel_parameters(),
            &[3, 16, 18, 19, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33],
        )?;
        let cp = usize::from(required(&f, 26, 1)?[0]);
        let rp = usize::from(required(&f, 27, 1)?[0]);
        let cp = *[96, 128, 160, 192, 224, 256, 288, 320, 384, 512, 640]
            .get(cp.wrapping_sub(1))
            .ok_or_else(|| invalid("ucd.cp_enum"))?;
        let rp = *[0, 32, 64, 96, 128, 160, 192, 224]
            .get(rp.wrapping_sub(1))
            .ok_or_else(|| invalid("ucd.rp_enum"))?;
        let spacing = match required(&f, 28, 1)?[0] {
            1 => 25,
            2 => 50,
            _ => return Err(invalid("ucd.spacing_enum")),
        };
        let ts = required(&f, 25, 9)?;
        let high = ts[..5].iter().fold(0_u64, |n, b| (n << 8) | u64::from(*b));
        let mut bursts = Vec::new();
        for burst in self.bursts() {
            let a = fields(&burst.attributes, &[3, 4, 19, 20, 21, 22])?;
            let data_profile = a
                .get(&21)
                .map(|v| {
                    if v.is_empty() || v.len() % 2 != 0 {
                        return Err(invalid("ucd.profile_encoding"));
                    }
                    Ok(v.chunks_exact(2)
                        .map(|g| ProfileGroup {
                            modulation_index: g[0] >> 4,
                            pilot_pattern: g[0] & 15,
                            additional_minislots: g[1],
                        })
                        .collect())
                })
                .transpose()?;
            let ir_power_control = a
                .get(&22)
                .map(|v| {
                    if v.len() != 2 {
                        return Err(invalid("ucd.ir_power_encoding"));
                    }
                    Ok(IrPowerControl {
                        starting_power_quarter_dbmv: v[0],
                        step_quarter_db: v[1],
                    })
                })
                .transpose()?;
            bursts.push(BurstDescriptor {
                iuc: burst.iuc,
                preamble_length_bits: optional(&a, 3, 2)?.map(|v| v as u16),
                preamble_offset_bits: optional(&a, 4, 2)?.map(|v| v as u16),
                initial_ranging_subcarriers: optional(&a, 19, 2)?.map(|v| v as u16),
                fine_ranging_subcarriers: optional(&a, 20, 2)?.map(|v| v as u16),
                data_profile,
                ir_power_control,
            });
        }
        let h = self.header();
        let channel = OfdmaChannel {
            message_type: h.message_type,
            management_version: h.version,
            upstream_channel_id: h.upstream_channel_id,
            configuration_change_count: h.configuration_change_count,
            minislot_size_ticks: h.minislot_size_ticks,
            downstream_channel_id: h.downstream_channel_id,
            preamble_pattern: f.get(&3).map(|v| v.to_vec()),
            ranging_required: optional(&f, 16, 1)?.map(|v| v as u8),
            ranging_holdoff_priority: optional(&f, 18, 4)?,
            channel_class_id: optional(&f, 19, 4)?,
            change_indicator_bitmask: integer(required(&f, 24, 2)?) as u16,
            timestamp_snapshot: TimestampSnapshot {
                reserved: (high >> 36) as u8,
                docsis_timestamp: (high >> 4) as u32,
                divide_by_20_msb: ts[4] & 15,
                minislot_count: integer(&ts[5..]),
            },
            cyclic_prefix_samples: cp,
            rolloff_samples: rp,
            subcarrier_spacing_khz: spacing,
            subcarrier_zero_hz: integer(required(&f, 29, 4)?),
            excluded_bands: bands(&f, 30, true)?,
            unused_bands: bands(&f, 31, false)?,
            frame_symbols: usize::from(required(&f, 32, 1)?[0]),
            randomization_seed: integer(required(&f, 33, 3)?),
            encompassed_bandwidth_hz: bandwidth_hz,
            bursts,
        };
        channel.validate()?;
        Ok(channel)
    }
}
