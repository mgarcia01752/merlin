/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Modulation profiles: I27 Table 80 and D.2.1.7 / I13 Table 113.
use super::{
    parse_limits::ParseLimits,
    pnm_file_type::PnmDecoderKind,
    pnm_header::{HasPnmHeader, PnmHeader},
    support::{
        payload::{budget, error, finish, require_decoder},
        serialization::{serialize_header, serialize_mac, serialize_spacing},
    },
};
use crate::common::reader::Reader;
use crate::pnm::data_type::modulation::ModulationAssignment;
use crate::{diagnostic::ParseError, pnm::data_type::*};
use serde::{ser::SerializeMap, Serialize, Serializer};

/// Decoded measurement retaining its common header and original payload.
#[derive(Debug, Clone, PartialEq)]
pub struct CmDsOfdmModulationProfile<'input> {
    header: PnmHeader<'input>,
    pub channel_id: ChannelId,
    pub mac_address: MacAddress,
    pub num_profiles: ProfileCount,
    pub subcarrier_zero_frequency_hz: FrequencyHz,
    pub first_active_subcarrier_index: SubcarrierIdx,
    pub subcarrier_spacing_khz: SubcarrierSpacingKhz,
    pub profile_data_length_bytes: DataLengthBytes,
    pub profiles: Vec<ModulationProfile>,
}
impl<'input> CmDsOfdmModulationProfile<'input> {
    /// Parses a complete capture, borrowing its original bytes.
    ///
    /// # Arguments
    /// * `bytes` - Complete input retained by the caller.
    /// * `limits` - Application sample budget.
    /// # Returns
    /// Typed metadata and borrowed measurement data.
    /// # Errors
    /// Reports unsupported identity/version, truncation, lengths or resource limits.
    pub fn parse(bytes: &'input [u8], limits: ParseLimits) -> Result<Self, ParseError> {
        Self::from_header(PnmHeader::from_bytes(bytes)?, limits)
    }
    /// Decodes the second tier using the centrally identified header.
    ///
    /// # Arguments
    /// * `header` - Header retaining original payload and offsets.
    /// * `limits` - Application sample budget.
    /// # Returns
    /// A capture with unchanged raw data retained by reference.
    /// # Errors
    /// Reports wrong identity, truncated fields, inconsistent lengths or limits.
    #[tracing::instrument(level = "debug", skip_all, fields(file_type = header.descriptor().tag), err)]
    pub fn from_header(header: PnmHeader<'input>, limits: ParseLimits) -> Result<Self, ParseError> {
        require_decoder(&header, PnmDecoderKind::ModulationProfile)?;
        let mut reader = Reader::with_origin(header.pnm_data(), header.payload_offset())?;
        let channel_id = reader.read_u8("channel_id")?;
        let mac_address = reader.read_bytes("mac_address")?;
        let num_profiles = reader.read_u8("num_profiles")?;
        let subcarrier_zero_frequency_hz = reader.read_u32_be("zero_frequency")?;
        let first_active_subcarrier_index = reader.read_u16_be("first_subcarrier")?;
        let subcarrier_spacing_khz = reader.read_u8("spacing_khz")?;
        let length_offset = reader.position();
        let profile_data_length_bytes = reader.read_u32_be("profile_data_length_bytes")?;
        let length = usize::try_from(profile_data_length_bytes).map_err(|_| {
            error(
                crate::diagnostic::ErrorCode::LimitExceeded,
                length_offset,
                "profile_data_length_bytes",
            )
        })?;
        let origin = reader.position();
        let bytes = reader.read_slice(length, "profiles")?;
        finish(&reader)?;
        let mut profiles_reader = Reader::with_origin(bytes, origin)?;
        let mut remaining = limits.max_samples;
        budget(usize::from(num_profiles), &mut remaining, origin)?;
        let mut profiles = Vec::new();
        for _ in 0..num_profiles {
            let profile_id = profiles_reader.read_u8("profile_id")?;
            let length = usize::from(profiles_reader.read_u16_be("profile_length")?);
            let origin = profiles_reader.position();
            let mut scheme_reader =
                Reader::with_origin(profiles_reader.read_slice(length, "schemes")?, origin)?;
            let mut schemes = Vec::new();
            while !scheme_reader.remaining().is_empty() {
                let offset = scheme_reader.position();
                budget(1, &mut remaining, offset)?;
                let scheme_type = scheme_reader.read_u8("schema_type")?;
                let scheme = match scheme_type {
                    RANGE_SCHEME => ModulationScheme::Range {
                        modulation_order: scheme_reader.read_u8("modulation_order")?,
                        num_subcarriers: scheme_reader.read_u16_be("num_subcarriers")?,
                    },
                    SKIP_SCHEME => ModulationScheme::Skip {
                        main_modulation_order: scheme_reader.read_u8("main_modulation_order")?,
                        skip_modulation_order: scheme_reader.read_u8("skip_modulation_order")?,
                        num_subcarriers: scheme_reader.read_u16_be("num_subcarriers")?,
                    },
                    _ => {
                        return Err(error(
                            crate::diagnostic::ErrorCode::UnsupportedLayout,
                            offset,
                            "schema_type",
                        ))
                    }
                };
                schemes.push(scheme);
            }
            profiles.push(ModulationProfile {
                profile_id,
                schemes,
            });
        }
        finish(&profiles_reader)?;
        Ok(Self {
            header,
            channel_id,
            mac_address,
            num_profiles,
            subcarrier_zero_frequency_hz,
            first_active_subcarrier_index,
            subcarrier_spacing_khz,
            profile_data_length_bytes,
            profiles,
        })
    }
    /// Serializes the capture directly into bounded pretty JSON.
    ///
    /// # Arguments
    /// * `max_bytes` - Maximum encoded output size.
    /// # Returns
    /// Human-readable JSON without a duplicate capture model.
    /// # Errors
    /// Propagates serialization and output-budget failures.
    pub fn to_json(&self, max_bytes: usize) -> serde_json::Result<String> {
        crate::common::json::to_json(self, max_bytes)
    }
}
impl HasPnmHeader for CmDsOfdmModulationProfile<'_> {
    /// Borrows the shared first-tier header.
    ///
    /// # Returns
    /// Header metadata and the original payload view.
    fn pnm_header(&self) -> &PnmHeader<'_> {
        &self.header
    }
}
const RANGE_SCHEME: u8 = 0;
const SKIP_SCHEME: u8 = 1;
/// One profile retaining compact assignments rather than expanded subcarrier arrays.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ModulationProfile {
    pub profile_id: ProfileId,
    pub schemes: Vec<ModulationScheme>,
}
/// Encoded assignments; JSON adds semantic fields while retaining numeric codes.
#[derive(Debug, Clone, PartialEq)]
pub enum ModulationScheme {
    Range {
        modulation_order: ModulationOrder,
        num_subcarriers: SubcarrierIdx,
    },
    Skip {
        main_modulation_order: ModulationOrder,
        skip_modulation_order: ModulationOrder,
        num_subcarriers: SubcarrierIdx,
    },
}

impl Serialize for ModulationScheme {
    /// Serializes compact schemes with textual assignments and original codes.
    ///
    /// # Arguments
    /// * `serializer` - Destination for the JSON-compatible scheme fields.
    /// # Returns
    /// Serialized range or skip scheme with nullable constellation bit loading
    /// and an ideal Shannon reference.
    /// # Errors
    /// Propagates destination serialization errors.
    fn serialize<Destination: Serializer>(
        &self,
        serializer: Destination,
    ) -> Result<Destination::Ok, Destination::Error> {
        match *self {
            Self::Range {
                modulation_order,
                num_subcarriers,
            } => {
                let assignment = ModulationAssignment::from_code(modulation_order);
                let mut fields = serializer.serialize_map(Some(9))?;
                fields.serialize_entry("schema_type", "range")?;
                fields.serialize_entry("modulation_order", assignment.modulation_order)?;
                fields.serialize_entry("modulation_order_code", &modulation_order)?;
                fields.serialize_entry("bit_loading", &assignment.bit_loading)?;
                fields.serialize_entry("constellation", &assignment.constellation())?;
                fields.serialize_entry(
                    "shannon_minimum_mer_db",
                    &assignment.shannon_minimum_mer_db(),
                )?;
                fields.serialize_entry("preamble", &assignment.preamble())?;
                fields.serialize_entry("carrier_type", &assignment.carrier_type)?;
                fields.serialize_entry("num_subcarriers", &num_subcarriers)?;
                fields.end()
            }
            Self::Skip {
                main_modulation_order,
                skip_modulation_order,
                num_subcarriers,
            } => {
                let main = ModulationAssignment::from_code(main_modulation_order);
                let skip = ModulationAssignment::from_code(skip_modulation_order);
                let mut fields = serializer.serialize_map(Some(16))?;
                fields.serialize_entry("schema_type", "skip")?;
                fields.serialize_entry("main_modulation_order", main.modulation_order)?;
                fields.serialize_entry("main_modulation_order_code", &main_modulation_order)?;
                fields.serialize_entry("main_bit_loading", &main.bit_loading)?;
                fields.serialize_entry("main_constellation", &main.constellation())?;
                fields.serialize_entry(
                    "main_shannon_minimum_mer_db",
                    &main.shannon_minimum_mer_db(),
                )?;
                fields.serialize_entry("main_preamble", &main.preamble())?;
                fields.serialize_entry("main_carrier_type", &main.carrier_type)?;
                fields.serialize_entry("skip_modulation_order", skip.modulation_order)?;
                fields.serialize_entry("skip_modulation_order_code", &skip_modulation_order)?;
                fields.serialize_entry("skip_bit_loading", &skip.bit_loading)?;
                fields.serialize_entry("skip_constellation", &skip.constellation())?;
                fields.serialize_entry(
                    "skip_shannon_minimum_mer_db",
                    &skip.shannon_minimum_mer_db(),
                )?;
                fields.serialize_entry("skip_preamble", &skip.preamble())?;
                fields.serialize_entry("skip_carrier_type", &skip.carrier_type)?;
                fields.serialize_entry("num_subcarriers", &num_subcarriers)?;
                fields.end()
            }
        }
    }
}

impl Serialize for CmDsOfdmModulationProfile<'_> {
    /// Exports raw profile schemes with shared Rust capacity analysis.
    /// # Arguments
    /// * `serializer` - Destination used by both CLI and desktop exports.
    /// # Returns
    /// Borrowed capture fields plus one derived analysis object per profile.
    /// # Errors
    /// Propagates serialization/output-limit errors without copying capture bytes.
    fn serialize<Output: Serializer>(
        &self,
        serializer: Output,
    ) -> Result<Output::Ok, Output::Error> {
        #[derive(Serialize)]
        struct CaptureJson<'capture, 'input> {
            #[serde(serialize_with = "serialize_header")]
            pnm_header: &'capture PnmHeader<'input>,
            channel_id: ChannelId,
            #[serde(serialize_with = "serialize_mac")]
            mac_address: &'capture MacAddress,
            num_profiles: ProfileCount,
            subcarrier_zero_frequency: FrequencyHz,
            first_active_subcarrier_index: SubcarrierIdx,
            #[serde(serialize_with = "serialize_spacing")]
            subcarrier_spacing: SubcarrierSpacingKhz,
            profile_data_length_bytes: DataLengthBytes,
            profiles: ProfilesJson<'capture>,
        }
        CaptureJson {
            pnm_header: &self.header,
            channel_id: self.channel_id,
            mac_address: &self.mac_address,
            num_profiles: self.num_profiles,
            subcarrier_zero_frequency: self.subcarrier_zero_frequency_hz,
            first_active_subcarrier_index: self.first_active_subcarrier_index,
            subcarrier_spacing: self.subcarrier_spacing_khz,
            profile_data_length_bytes: self.profile_data_length_bytes,
            profiles: ProfilesJson {
                profiles: &self.profiles,
                spacing: self.subcarrier_spacing_khz,
            },
        }
        .serialize(serializer)
    }
}
struct ProfilesJson<'capture> {
    profiles: &'capture [ModulationProfile],
    spacing: SubcarrierSpacingKhz,
}
impl Serialize for ProfilesJson<'_> {
    /// Streams profiles and derived summaries without cloning scheme storage.
    /// # Arguments
    /// * `serializer` - Sequence destination.
    /// # Returns
    /// Profile identities, borrowed schemes and shared derived analysis.
    /// # Errors
    /// Propagates serialization failures.
    fn serialize<Output: Serializer>(
        &self,
        serializer: Output,
    ) -> Result<Output::Ok, Output::Error> {
        use crate::pnm::analysis::modulation_profile::ModulationProfileAnalysis;
        use serde::ser::SerializeSeq;
        #[derive(Serialize)]
        struct ProfileJson<'profile> {
            profile_id: ProfileId,
            bandwidth_bps: Option<BitRateBps>,
            schemes: &'profile [ModulationScheme],
            analysis: ModulationProfileAnalysis,
        }
        let mut sequence = serializer.serialize_seq(Some(self.profiles.len()))?;
        for profile in self.profiles {
            let analysis = ModulationProfileAnalysis::calculate(profile, self.spacing);
            sequence.serialize_element(&ProfileJson {
                profile_id: profile.profile_id,
                bandwidth_bps: analysis.theoretical_throughput_bps,
                schemes: &profile.schemes,
                analysis,
            })?;
        }
        sequence.end()
    }
}
