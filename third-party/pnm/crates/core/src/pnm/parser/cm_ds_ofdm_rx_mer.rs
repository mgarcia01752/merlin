/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! RxMER version 1.0, based on I27 Table 68 and I13 Table 101 text extracts.
use super::pnm_header::{HasPnmHeader, PnmHeader};
use crate::common::reader::Reader;
use crate::diagnostic::{ErrorCode, ParseError};
use crate::pnm::data_type::units::{HZ_PER_KHZ, QUARTER_DB_PER_DB};
use crate::pnm::data_type::{
    ChannelId, DataLengthBytes, FrequencyHz, MacAddress, RxMerDb, SubcarrierIdx,
    SubcarrierSpacingKhz,
};
use serde::{ser::SerializeStruct, Serialize, Serializer};

const RXMER_ZERO_FREQUENCY_OFFSET: usize = 17;
const RXMER_LENGTH_OFFSET: usize = 24;
const RXMER_UNAVAILABLE: u8 = 0xff;
pub use super::parse_limits::ParseLimits;

/// Decoded RxMER metadata with samples borrowed from the caller.
/// No statistics, I/O or allocation of sample arrays occurs here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmDsOfdmRxMer<'input> {
    header: PnmHeader<'input>,
    pub channel_id: ChannelId,
    pub mac_address: MacAddress,
    pub subcarrier_zero_frequency_hz: FrequencyHz,
    pub first_active_subcarrier_index: SubcarrierIdx,
    pub subcarrier_spacing_khz: SubcarrierSpacingKhz,
    raw_values: &'input [u8],
}
impl HasPnmHeader for CmDsOfdmRxMer<'_> {
    /// Borrows the composed common header.
    ///
    /// # Returns
    /// A shared header reference valid for the capture borrow.
    fn pnm_header(&self) -> &PnmHeader<'_> {
        &self.header
    }
}
impl<'input> CmDsOfdmRxMer<'input> {
    /// Decodes an RxMER 1.0 file without copying its samples.
    ///
    /// # Arguments
    /// * `bytes` - Complete capture retained for the returned capture lifetime.
    /// * `limits` - Caller-selected sample-count budget.
    ///
    /// # Returns
    /// A capture borrowing bytes; absent measurements remain distinguishable.
    ///
    /// # Errors
    /// Reports field/offset errors for truncation, unsupported type/version,
    /// exceeded limits, frequency overflow or a payload length mismatch, including trailing bytes.
    pub fn parse(bytes: &'input [u8], limits: ParseLimits) -> Result<Self, ParseError> {
        Self::from_header(PnmHeader::from_bytes(bytes)?, limits)
    }
    /// Decodes RxMER-specific fields from an already-parsed header.
    ///
    /// # Arguments
    /// * `header` - First-tier header retaining the remaining original bytes.
    /// * `limits` - Maximum accepted sample count.
    /// # Returns
    /// A capture retaining the header and borrowing raw samples without copying.
    /// # Errors
    /// Rejects non-RxMER identity, truncation, length/limit violations or overflow.
    /// Diagnostics remain relative to the original file.
    #[tracing::instrument(level = "debug", skip_all, fields(file_type = header.descriptor().tag, max_samples = limits.max_samples), err)]
    pub fn from_header(header: PnmHeader<'input>, limits: ParseLimits) -> Result<Self, ParseError> {
        if header.descriptor().decoder != super::pnm_file_type::PnmDecoderKind::RxMer {
            return Err(ParseError {
                code: ErrorCode::UnsupportedType,
                offset: 0,
                field: "file_type",
            });
        }
        let error = |code, offset, field| ParseError {
            code,
            offset,
            field,
        };
        let mut reader = Reader::with_origin(header.pnm_data(), header.payload_offset())?;
        let channel_id: ChannelId = reader.read_u8("channel_id")?;
        let mac_address: MacAddress = reader.read_bytes("mac_address")?;
        let subcarrier_zero_frequency_hz: FrequencyHz = reader.read_u32_be("zero_frequency")?;
        let first_active_subcarrier_index: SubcarrierIdx =
            reader.read_u16_be("first_subcarrier")?;
        let subcarrier_spacing_khz: SubcarrierSpacingKhz = reader.read_u8("spacing_khz")?;
        let data_length_bytes: DataLengthBytes = reader.read_u32_be("data_length")?;
        // Convert the encoded length separately for host-sized indexing and limits.
        let payload_length = usize::try_from(data_length_bytes)
            .map_err(|_| error(ErrorCode::LimitExceeded, RXMER_LENGTH_OFFSET, "data_length"))?;
        if payload_length > limits.max_samples {
            return Err(error(
                ErrorCode::LimitExceeded,
                RXMER_LENGTH_OFFSET,
                "data_length",
            ));
        }
        let raw_values = reader.remaining();
        if raw_values.len() != payload_length {
            return Err(error(
                ErrorCode::LengthMismatch,
                RXMER_LENGTH_OFFSET,
                "data_length",
            ));
        }
        let capture = Self {
            header,
            channel_id,
            mac_address,
            subcarrier_zero_frequency_hz,
            first_active_subcarrier_index,
            subcarrier_spacing_khz,
            raw_values,
        };
        // Range validation is eager; sample iteration is not needed during parsing.
        let _frequencies = capture.frequencies_hz()?;
        tracing::debug!(
            sample_count = capture.raw_values.len(),
            "Parsed RxMER payload"
        );
        Ok(capture)
    }
    /// Serializes this capture directly to bounded pretty JSON.
    ///
    /// # Arguments
    /// * `max_bytes` - Maximum output bytes including indentation.
    /// # Returns
    /// A JSON string; unavailable readings remain null.
    /// # Errors
    /// Reports serialization failures or an exceeded output limit.
    pub fn to_json(&self, max_bytes: usize) -> serde_json::Result<String> {
        crate::common::json::to_json(self, max_bytes)
    }
    /// Borrows the unchanged encoded RxMER samples.
    ///
    /// # Returns
    /// The original sample slice, including unavailable-value sentinels.
    pub fn raw_values(&self) -> &'input [u8] {
        self.raw_values
    }
    /// Lazily converts quarter-dB samples without allocating.
    ///
    /// # Returns
    /// An iterator of dB values; None denotes an unavailable sample.
    pub fn values_db(&self) -> impl Iterator<Item = Option<RxMerDb>> + '_ {
        self.raw_values.iter().copied().map(decode_rxmer_value)
    }
    /// Validates the frequency range and lazily yields sample frequencies.
    ///
    /// # Returns
    /// A no-allocation Hz iterator, empty for an empty payload. Zero spacing is
    /// preserved. Revalidation protects callers after public metadata changes.
    /// No PHY-range validation is implied.
    ///
    /// # Errors
    /// Returns ArithmeticOverflow when a sample frequency cannot fit FrequencyHz.
    pub fn frequencies_hz(&self) -> Result<impl Iterator<Item = FrequencyHz>, ParseError> {
        let overflow = || ParseError {
            code: ErrorCode::ArithmeticOverflow,
            offset: RXMER_ZERO_FREQUENCY_OFFSET,
            field: "sample_frequency_hz",
        };
        let sample_count = u32::try_from(self.raw_values.len()).map_err(|_| overflow())?;
        let spacing_hz = FrequencyHz::from(self.subcarrier_spacing_khz) * HZ_PER_KHZ;
        let start_hz = if sample_count == 0 {
            self.subcarrier_zero_frequency_hz
        } else {
            FrequencyHz::from(self.first_active_subcarrier_index)
                .checked_mul(spacing_hz)
                .and_then(|offset_hz| self.subcarrier_zero_frequency_hz.checked_add(offset_hz))
                .ok_or_else(overflow)?
        };
        if sample_count > 0 {
            (sample_count - 1)
                .checked_mul(spacing_hz)
                .and_then(|offset_hz| start_hz.checked_add(offset_hz))
                .ok_or_else(overflow)?;
        }
        // Nonnegative, monotonic offsets are bounded by the checked final sample.
        Ok((0..sample_count).map(move |index| start_hz + index * spacing_hz))
    }
}

/// Converts one encoded RxMER sample for both typed iteration and JSON output.
///
/// # Arguments
/// * `raw_value` - Original quarter-dB byte, including the unavailable sentinel.
/// # Returns
/// A dB reading or None for unavailable data, without clamping the input.
fn decode_rxmer_value(raw_value: u8) -> Option<RxMerDb> {
    if raw_value == RXMER_UNAVAILABLE {
        None
    } else {
        Some(f64::from(raw_value) / QUARTER_DB_PER_DB)
    }
}

impl Serialize for CmDsOfdmRxMer<'_> {
    /// Serializes this capture using the existing PyPNM-aligned field names.
    ///
    /// # Arguments
    /// * `serializer` - Destination serializer; byte limits belong to to_json().
    /// # Returns
    /// Header and RxMER fields with MAC text, Hz spacing and lazy dB samples.
    /// # Errors
    /// Propagates serialization failures without modifying the capture.
    fn serialize<Output: Serializer>(
        &self,
        serializer: Output,
    ) -> Result<Output::Ok, Output::Error> {
        let mut output = serializer.serialize_struct("CmDsOfdmRxMer", 9)?;
        output.serialize_field("pnm_header", &self.header.to_model().pnm_header)?;
        output.serialize_field("channel_id", &self.channel_id)?;
        let [first, second, third, fourth, fifth, sixth] = self.mac_address;
        let mac_address =
            format!("{first:02x}:{second:02x}:{third:02x}:{fourth:02x}:{fifth:02x}:{sixth:02x}");
        output.serialize_field("mac_address", &mac_address)?;
        output.serialize_field(
            "subcarrier_zero_frequency",
            &self.subcarrier_zero_frequency_hz,
        )?;
        output.serialize_field(
            "first_active_subcarrier_index",
            &self.first_active_subcarrier_index,
        )?;
        let spacing_hz = FrequencyHz::from(self.subcarrier_spacing_khz) * HZ_PER_KHZ;
        output.serialize_field("subcarrier_spacing", &spacing_hz)?;
        output.serialize_field("data_length", &self.raw_values.len())?;
        output.serialize_field("value_units", "dB")?;
        output.serialize_field("values", &RxMerValues(self))?;
        output.end()
    }
}

/// Private adapter that streams the capture's existing decoded-value iterator.
struct RxMerValues<'capture, 'input>(&'capture CmDsOfdmRxMer<'input>);
impl Serialize for RxMerValues<'_, '_> {
    /// Serializes samples through the same conversion used by typed callers.
    ///
    /// # Arguments
    /// * `serializer` - Destination serializer.
    /// # Returns
    /// A sequence of dB values and nulls without allocating a sample vector.
    /// # Errors
    /// Propagates destination serialization errors.
    fn serialize<Output: Serializer>(
        &self,
        serializer: Output,
    ) -> Result<Output::Ok, Output::Error> {
        serializer.collect_seq(self.0.values_db())
    }
}
