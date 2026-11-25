/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Latency: I27 Tables 83/84 / I13 Tables 116/117; synthetic validation only.
use super::{
    parse_limits::ParseLimits,
    pnm_file_type::PnmDecoderKind,
    pnm_header::{HasPnmHeader, PnmHeader},
    support::{
        payload::{budget, error, finish, require_decoder},
        serialization::{serialize_header, serialize_mac},
    },
};
use crate::common::reader::Reader;
use crate::{diagnostic::ParseError, pnm::data_type::*};
use serde::Serialize;

/// Decoded measurement retaining its common header and original payload.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CmLatencyRpt<'input> {
    #[serde(rename = "pnm_header", serialize_with = "serialize_header")]
    header: PnmHeader<'input>,
    #[serde(serialize_with = "serialize_mac")]
    pub mac_address: MacAddress,
    pub num_latency_objects: u8,
    pub latency_data: Vec<LatencySummary>,
}
impl<'input> CmLatencyRpt<'input> {
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
        require_decoder(&header, PnmDecoderKind::LatencyReport)?;
        let mut reader = Reader::with_origin(header.pnm_data(), header.payload_offset())?;
        let mac_address = reader.read_bytes("mac_address")?;
        let num_latency_objects = reader.read_u8("num_latency_objects")?;
        let mut remaining = limits.max_samples;
        budget(
            usize::from(num_latency_objects),
            &mut remaining,
            reader.position(),
        )?;
        let mut latency_data = Vec::new();
        for _ in 0..num_latency_objects {
            let sf_id = reader.read_u32_be("sf_id")?;
            let sf_label_bytes = reader.read_bytes("sf_label")?;
            let num_bin_edges = reader.read_u8("num_bin_edges")?;
            budget(
                usize::from(num_bin_edges),
                &mut remaining,
                reader.position(),
            )?;
            let mut bin_edges_hundredths_ms = Vec::new();
            for _ in 0..num_bin_edges {
                let offset = reader.position();
                let edge = reader.read_u16_be("bin_edge")?;
                if bin_edges_hundredths_ms
                    .last()
                    .is_some_and(|previous| *previous >= edge)
                {
                    return Err(error(
                        crate::diagnostic::ErrorCode::LengthMismatch,
                        offset,
                        "bin_edge_order",
                    ));
                }
                bin_edges_hundredths_ms.push(edge);
            }
            let num_snapshots = reader.read_u16_be("num_snapshots")?;
            let first_snapshot_start_timestamp =
                reader.read_u32_be("first_snapshot_start_timestamp")?;
            let bin_count = usize::from(num_bin_edges) + 1;
            budget(
                usize::from(num_snapshots) * (bin_count + 1),
                &mut remaining,
                reader.position(),
            )?;
            let mut snapshots = Vec::new();
            for _ in 0..num_snapshots {
                let snapshot_end_timestamp = reader.read_u32_be("snapshot_end_timestamp")?;
                let mut bin_counts = Vec::new();
                for _ in 0..bin_count {
                    bin_counts.push(u64::from_be_bytes(reader.read_bytes("bin_count")?));
                }
                snapshots.push(LatencySnapshot {
                    snapshot_end_timestamp,
                    bin_counts,
                    max_latency_us: reader.read_u32_be("max_latency_us")?,
                    num_histogram_updates: reader.read_u32_be("num_histogram_updates")?,
                    sanctioned_pkts: reader.read_u32_be("sanctioned_pkts")?,
                    dropped_pkts: reader.read_u32_be("dropped_pkts")?,
                    total_ect0_pkts: reader.read_u32_be("total_ect0_pkts")?,
                    reserved: reader.read_u32_be("reserved")?,
                    total_ect1_pkts: reader.read_u32_be("total_ect1_pkts")?,
                    ce_marked_ect1_pkts: reader.read_u32_be("ce_marked_ect1_pkts")?,
                });
            }
            latency_data.push(LatencySummary {
                sf_id,
                sf_label_bytes,
                bin_edges_hundredths_ms,
                first_snapshot_start_timestamp,
                snapshots,
            });
        }
        finish(&reader)?;
        Ok(Self {
            header,
            mac_address,
            num_latency_objects,
            latency_data,
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
impl HasPnmHeader for CmLatencyRpt<'_> {
    /// Borrows the shared first-tier header.
    ///
    /// # Returns
    /// Header metadata and the original payload view.
    fn pnm_header(&self) -> &PnmHeader<'_> {
        &self.header
    }
}
/// Service-flow metadata. Labels remain exact bytes rather than guessed Unicode.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LatencySummary {
    pub sf_id: ServiceFlowId,
    pub sf_label_bytes: ServiceFlowLabel,
    pub bin_edges_hundredths_ms: Vec<LatencyBinEdge>,
    pub first_snapshot_start_timestamp: CaptureTime,
    pub snapshots: Vec<LatencySnapshot>,
}
/// One interval; 64-bit histogram counts are retained without narrowing.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LatencySnapshot {
    pub snapshot_end_timestamp: CaptureTime,
    pub bin_counts: Vec<PacketCount>,
    pub max_latency_us: LatencyUs,
    pub num_histogram_updates: CounterValue,
    pub sanctioned_pkts: CounterValue,
    pub dropped_pkts: CounterValue,
    pub total_ect0_pkts: CounterValue,
    pub reserved: CounterValue,
    pub total_ect1_pkts: CounterValue,
    pub ce_marked_ect1_pkts: CounterValue,
}
