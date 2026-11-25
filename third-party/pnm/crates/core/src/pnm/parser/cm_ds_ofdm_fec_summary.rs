/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! FEC summary: I27 Table 71 and D.2.1.6 / I13 Table 104; no capture timestamp.
use super::{
    parse_limits::ParseLimits,
    pnm_file_type::PnmDecoderKind,
    pnm_header::{HasPnmHeader, PnmHeader},
    support::{
        payload::{budget, finish, require_decoder},
        serialization::{serialize_header, serialize_mac},
        values::COUNTER_BYTES,
    },
};
use crate::common::reader::Reader;
use crate::{diagnostic::ParseError, pnm::data_type::*};
use serde::Serialize;

/// Decoded measurement retaining its common header and original payload.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CmDsOfdmFecSummary<'input> {
    #[serde(rename = "pnm_header", serialize_with = "serialize_header")]
    header: PnmHeader<'input>,
    pub channel_id: ChannelId,
    #[serde(serialize_with = "serialize_mac")]
    pub mac_address: MacAddress,
    pub summary_type: FecSummaryType,
    pub num_profiles: ProfileCount,
    pub fec_summary_data: Vec<FecProfile<'input>>,
}
impl<'input> CmDsOfdmFecSummary<'input> {
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
        require_decoder(&header, PnmDecoderKind::FecSummary)?;
        let mut reader = Reader::with_origin(header.pnm_data(), header.payload_offset())?;
        let channel_id = reader.read_u8("channel_id")?;
        let mac_address = reader.read_bytes("mac_address")?;
        let summary_type = reader.read_u8("summary_type")?;
        let num_profiles = reader.read_u8("num_profiles")?;
        let mut remaining = limits.max_samples;
        budget(usize::from(num_profiles), &mut remaining, reader.position())?;
        let mut fec_summary_data = Vec::new();
        for _ in 0..num_profiles {
            let profile_id = reader.read_u8("profile_id")?;
            let number_of_sets = reader.read_u16_be("number_of_sets")?;
            budget(
                usize::from(number_of_sets),
                &mut remaining,
                reader.position(),
            )?;
            let bytes = reader.read_slice(
                usize::from(number_of_sets) * FEC_RECORD_BYTES,
                "codeword_entries",
            )?;
            fec_summary_data.push(FecProfile {
                profile_id,
                number_of_sets,
                codeword_entries: FecRecords(bytes),
            });
        }
        finish(&reader)?;
        Ok(Self {
            header,
            channel_id,
            mac_address,
            summary_type,
            num_profiles,
            fec_summary_data,
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
impl HasPnmHeader for CmDsOfdmFecSummary<'_> {
    /// Borrows the shared first-tier header.
    ///
    /// # Returns
    /// Header metadata and the original payload view.
    fn pnm_header(&self) -> &PnmHeader<'_> {
        &self.header
    }
}
const FEC_RECORD_BYTES: usize = 16;
/// One profile with borrowed timestamped codeword records.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FecProfile<'input> {
    pub profile_id: ProfileId,
    pub number_of_sets: RecordCount,
    codeword_entries: FecRecords<'input>,
}
/// One decoded codeword interval. Capture time remains absent in the file header.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CodewordEntry {
    pub timestamp: CaptureTime,
    pub total_codewords: CounterValue,
    pub corrected: CounterValue,
    pub uncorrectable: CounterValue,
}
impl FecProfile<'_> {
    /// Lazily reads validated intervals from the original payload.
    ///
    /// # Returns
    /// Timestamp and three unsigned counters per interval.
    pub fn entries(&self) -> impl Iterator<Item = CodewordEntry> + '_ {
        self.codeword_entries.entries()
    }
}
#[derive(Debug, Clone, PartialEq)]
struct FecRecords<'input>(&'input [u8]);
impl FecRecords<'_> {
    fn entries(&self) -> impl Iterator<Item = CodewordEntry> + '_ {
        self.0.chunks_exact(FEC_RECORD_BYTES).map(|record| {
            let counter = |offset| {
                CounterValue::from_be_bytes([
                    record[offset],
                    record[offset + 1],
                    record[offset + 2],
                    record[offset + 3],
                ])
            };
            CodewordEntry {
                timestamp: counter(0),
                total_codewords: counter(COUNTER_BYTES),
                corrected: counter(2 * COUNTER_BYTES),
                uncorrectable: counter(3 * COUNTER_BYTES),
            }
        })
    }
}
impl Serialize for FecRecords<'_> {
    /// Streams intervals directly from the original bytes.
    ///
    /// # Arguments
    /// * `serializer` - Output destination.
    /// # Returns
    /// An array of timestamped codeword records.
    /// # Errors
    /// Propagates destination failures.
    fn serialize<Output: serde::Serializer>(
        &self,
        serializer: Output,
    ) -> Result<Output::Ok, Output::Error> {
        serializer.collect_seq(self.entries())
    }
}
