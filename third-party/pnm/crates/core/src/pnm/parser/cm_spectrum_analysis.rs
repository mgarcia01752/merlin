/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Spectrum: I27 Table 58 and D.2.1.4 / I13 Table 91; signed hundredths dB.
use super::{
    parse_limits::ParseLimits,
    pnm_file_type::PnmDecoderKind,
    pnm_header::{HasPnmHeader, PnmHeader},
    support::{
        payload::{error, finish, payload, require_decoder},
        serialization::{serialize_header, serialize_mac},
        values::AMPLITUDE_BYTES,
    },
};
use crate::common::reader::Reader;
use crate::{diagnostic::ParseError, pnm::data_type::*};
use serde::Serialize;

/// Decoded measurement retaining its common header and original payload.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CmSpectrumAnalysis<'input> {
    #[serde(rename = "pnm_header", serialize_with = "serialize_header")]
    header: PnmHeader<'input>,
    pub channel_id: ChannelId,
    #[serde(serialize_with = "serialize_mac")]
    pub mac_address: MacAddress,
    pub first_segment_center_frequency: FrequencyHz,
    pub last_segment_center_frequency: FrequencyHz,
    pub segment_frequency_span: FrequencyHz,
    pub num_bins_per_segment: SegmentBinCount,
    pub equivalent_noise_bandwidth: NoiseBandwidth,
    pub window_function: WindowFunction,
    pub spectrum_analysis_data_length: DataLengthBytes,
    amplitude_bin_segments_float: SpectrumSegments<'input>,
}
impl<'input> CmSpectrumAnalysis<'input> {
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
        require_decoder(&header, PnmDecoderKind::SpectrumAnalysis)?;
        let mut reader = Reader::with_origin(header.pnm_data(), header.payload_offset())?;
        let channel_id = reader.read_u8("channel_id")?;
        let mac_address = reader.read_bytes("mac_address")?;
        let first_segment_center_frequency =
            reader.read_u32_be("first_segment_center_frequency")?;
        let last_segment_center_frequency = reader.read_u32_be("last_segment_center_frequency")?;
        let segment_frequency_span = reader.read_u32_be("segment_frequency_span")?;
        let bins_offset = reader.position();
        let num_bins_per_segment = reader.read_u16_be("num_bins_per_segment")?;
        if num_bins_per_segment == 0 {
            return Err(error(
                crate::diagnostic::ErrorCode::LengthMismatch,
                bins_offset,
                "num_bins_per_segment",
            ));
        }
        let equivalent_noise_bandwidth = reader.read_u16_be("equivalent_noise_bandwidth")?;
        let window_function = reader.read_u16_be("window_function")?;
        let bytes = payload(&mut reader, AMPLITUDE_BYTES, limits)?;
        finish(&reader)?;
        Ok(Self {
            header,
            channel_id,
            mac_address,
            first_segment_center_frequency,
            last_segment_center_frequency,
            segment_frequency_span,
            num_bins_per_segment,
            equivalent_noise_bandwidth,
            window_function,
            spectrum_analysis_data_length: bytes.len() as DataLengthBytes,
            amplitude_bin_segments_float: SpectrumSegments {
                bytes,
                bins: num_bins_per_segment,
            },
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
impl HasPnmHeader for CmSpectrumAnalysis<'_> {
    /// Borrows the shared first-tier header.
    ///
    /// # Returns
    /// Header metadata and the original payload view.
    fn pnm_header(&self) -> &PnmHeader<'_> {
        &self.header
    }
}
impl<'input> CmSpectrumAnalysis<'input> {
    /// Borrows unchanged signed amplitude bytes.
    ///
    /// # Returns
    /// Original payload amplitudes in network byte order.
    pub fn raw_values(&self) -> &'input [u8] {
        self.amplitude_bin_segments_float.bytes
    }
    /// Lazily converts signed hundredths of a dB.
    ///
    /// # Returns
    /// Amplitudes referenced to 0 dBmV, including a partial final segment.
    pub fn amplitudes_db(&self) -> impl Iterator<Item = AmplitudeDb> + '_ {
        amplitudes(self.raw_values())
    }
}
fn amplitudes(bytes: &[u8]) -> impl Iterator<Item = AmplitudeDb> + '_ {
    bytes.chunks_exact(AMPLITUDE_BYTES).map(|sample| {
        f64::from(i16::from_be_bytes([sample[0], sample[1]])) / units::HUNDREDTH_DB_PER_DB
    })
}
#[derive(Debug, Clone, PartialEq)]
struct SpectrumSegments<'input> {
    bytes: &'input [u8],
    bins: SegmentBinCount,
}
struct SpectrumSegment<'input>(&'input [u8]);
impl Serialize for SpectrumSegment<'_> {
    /// Streams one amplitude segment.
    ///
    /// # Arguments
    /// * `serializer` - Output destination.
    /// # Returns
    /// Signed dB values.
    /// # Errors
    /// Propagates destination failures.
    fn serialize<Output: serde::Serializer>(
        &self,
        serializer: Output,
    ) -> Result<Output::Ok, Output::Error> {
        serializer.collect_seq(amplitudes(self.0))
    }
}
impl Serialize for SpectrumSegments<'_> {
    /// Streams segments, retaining a partial final segment.
    ///
    /// # Arguments
    /// * `serializer` - Output destination.
    /// # Returns
    /// Nested arrays of amplitudes.
    /// # Errors
    /// Propagates destination failures.
    fn serialize<Output: serde::Serializer>(
        &self,
        serializer: Output,
    ) -> Result<Output::Ok, Output::Error> {
        serializer.collect_seq(
            self.bytes
                .chunks(usize::from(self.bins) * AMPLITUDE_BYTES)
                .map(SpectrumSegment),
        )
    }
}
