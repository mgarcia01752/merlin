/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Constellation display: I27 Table 65 / I13 Table 98; complex s2.13.
use super::{
    parse_limits::ParseLimits,
    pnm_file_type::PnmDecoderKind,
    pnm_header::{HasPnmHeader, PnmHeader},
    support::{
        payload::{finish, payload, require_decoder},
        serialization::{serialize_header, serialize_mac, serialize_spacing},
        values::{IqSamples, IQ_PAIR_BYTES},
    },
};
use crate::common::reader::Reader;
use crate::pnm::lib::fixed_point_decoder::QFormat;
use crate::{diagnostic::ParseError, pnm::data_type::*};
use serde::Serialize;

/// Decoded measurement retaining its common header and original payload.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CmDsConstDispMeas<'input> {
    #[serde(rename = "pnm_header", serialize_with = "serialize_header")]
    header: PnmHeader<'input>,
    pub channel_id: ChannelId,
    #[serde(serialize_with = "serialize_mac")]
    pub mac_address: MacAddress,
    #[serde(rename = "subcarrier_zero_frequency")]
    pub subcarrier_zero_frequency_hz: FrequencyHz,
    pub actual_modulation_order: ConstellationModulationOrder,
    pub num_sample_symbols: SymbolCount,
    #[serde(rename = "subcarrier_spacing", serialize_with = "serialize_spacing")]
    pub subcarrier_spacing_khz: SubcarrierSpacingKhz,
    pub data_length: DataLengthBytes,
    iq: IqSamples<'input>,
    pub constellation_analysis: crate::pnm::analysis::constellation::ConstellationAnalysis,
}
impl<'input> CmDsConstDispMeas<'input> {
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
        require_decoder(&header, PnmDecoderKind::ConstellationDisplay)?;
        let mut reader = Reader::with_origin(header.pnm_data(), header.payload_offset())?;
        let channel_id = reader.read_u8("channel_id")?;
        let mac_address = reader.read_bytes("mac_address")?;
        let subcarrier_zero_frequency_hz = reader.read_u32_be("zero_frequency")?;
        let actual_modulation_order = reader.read_u16_be("actual_modulation_order")?;
        let num_sample_symbols = reader.read_u16_be("num_sample_symbols")?;
        let subcarrier_spacing_khz = reader.read_u8("spacing_khz")?;
        let bytes = payload(&mut reader, IQ_PAIR_BYTES, limits)?;
        finish(&reader)?;
        let data_length = bytes.len() as DataLengthBytes;
        let iq = IqSamples {
            bytes,
            format: QFormat::S2_13,
            excluded_pair: false,
        };
        let mut capture = Self {
            header,
            channel_id,
            mac_address,
            subcarrier_zero_frequency_hz,
            actual_modulation_order,
            num_sample_symbols,
            subcarrier_spacing_khz,
            data_length,
            iq,
            constellation_analysis:
                crate::pnm::analysis::constellation::ConstellationAnalysis::from_modulation_type(
                    actual_modulation_order,
                ),
        };
        capture.constellation_analysis.mer =
            crate::pnm::analysis::constellation::mer::from_capture(&capture);
        Ok(capture)
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
impl HasPnmHeader for CmDsConstDispMeas<'_> {
    /// Borrows the shared first-tier header.
    ///
    /// # Returns
    /// Header metadata and the original payload view.
    fn pnm_header(&self) -> &PnmHeader<'_> {
        &self.header
    }
}
impl<'input> CmDsConstDispMeas<'input> {
    /// Borrows the unchanged encoded complex samples.
    ///
    /// # Returns
    /// Original I/Q bytes, with no second copy of the file.
    pub fn raw_iq(&self) -> &'input [u8] {
        self.iq.bytes
    }
    /// Lazily decodes fixed-point I/Q pairs.
    ///
    /// # Returns
    /// [I, Q] s2.13 pairs, already normalized by the CM. All encoded pairs
    /// remain valid here, including the channel-estimate exclusion bit pattern.
    pub fn iq(&self) -> impl Iterator<Item = Option<IqPair>> + '_ {
        self.iq.iq()
    }
}
