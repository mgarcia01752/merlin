/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Histogram: I27 Table 74 / I13 Table 107; lengths are bytes.
use super::{
    parse_limits::ParseLimits,
    pnm_file_type::PnmDecoderKind,
    pnm_header::{HasPnmHeader, PnmHeader},
    support::{
        payload::{finish, payload, require_decoder},
        serialization::{serialize_header, serialize_mac},
        values::{CounterValues, COUNTER_BYTES},
    },
};
use crate::common::reader::Reader;
use crate::{diagnostic::ParseError, pnm::data_type::*};
use serde::Serialize;

/// Decoded measurement retaining its common header and original payload.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CmDsHist<'input> {
    #[serde(rename = "pnm_header", serialize_with = "serialize_header")]
    header: PnmHeader<'input>,
    #[serde(serialize_with = "serialize_mac")]
    pub mac_address: MacAddress,
    pub symmetry: HistogramSymmetry,
    pub dwell_count_values_length: DataLengthBytes,
    pub hit_count_values_length: DataLengthBytes,
    dwell_count_values: CounterValues<'input>,
    hit_count_values: CounterValues<'input>,
}
impl<'input> CmDsHist<'input> {
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
        require_decoder(&header, PnmDecoderKind::Histogram)?;
        let mut reader = Reader::with_origin(header.pnm_data(), header.payload_offset())?;
        let mac_address = reader.read_bytes("mac_address")?;
        let symmetry = reader.read_u8("symmetry")?;
        let dwell_bytes = payload(&mut reader, COUNTER_BYTES, limits)?;
        let remaining = limits.max_samples - dwell_bytes.len() / COUNTER_BYTES;
        let hit_bytes = payload(
            &mut reader,
            COUNTER_BYTES,
            ParseLimits {
                max_samples: remaining,
            },
        )?;
        finish(&reader)?;
        Ok(Self {
            header,
            mac_address,
            symmetry,
            dwell_count_values_length: dwell_bytes.len() as DataLengthBytes,
            hit_count_values_length: hit_bytes.len() as DataLengthBytes,
            dwell_count_values: CounterValues(dwell_bytes),
            hit_count_values: CounterValues(hit_bytes),
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
impl HasPnmHeader for CmDsHist<'_> {
    /// Borrows the shared first-tier header.
    ///
    /// # Returns
    /// Header metadata and the original payload view.
    fn pnm_header(&self) -> &PnmHeader<'_> {
        &self.header
    }
}
impl CmDsHist<'_> {
    /// Iterates dwell counts without copying the underlying bytes.
    ///
    /// # Returns
    /// Unsigned dwell counts in capture order.
    pub fn dwell_counts(&self) -> impl Iterator<Item = CounterValue> + '_ {
        self.dwell_count_values.values()
    }
    /// Iterates hit counts without copying the underlying bytes.
    ///
    /// # Returns
    /// Unsigned hit counts in capture order.
    pub fn hit_counts(&self) -> impl Iterator<Item = CounterValue> + '_ {
        self.hit_count_values.values()
    }
}
