/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared JSON field adapters; numeric decoding lives in pnm::lib.
use super::super::pnm_header::PnmHeader;
use crate::pnm::data_type::{units, FrequencyHz, SubcarrierSpacingKhz};
use serde::{Serialize, Serializer};

/// Serializes shared header metadata without the retained payload bytes.
///
/// # Arguments
/// * `header` - First-tier header.
/// * `serializer` - Output destination.
/// # Returns
/// The common metadata fields.
/// # Errors
/// Propagates destination failures.
pub(crate) fn serialize_header<Output: Serializer>(
    header: &PnmHeader<'_>,
    serializer: Output,
) -> Result<Output::Ok, Output::Error> {
    header.to_model().pnm_header.serialize(serializer)
}
pub(crate) use crate::pnm::data_type::serialize_mac;

/// Converts encoded kHz spacing to the JSON Hz representation.
///
/// # Arguments
/// * `spacing` - Encoded kHz spacing.
/// * `serializer` - Output destination.
/// # Returns
/// An unsigned Hz value.
/// # Errors
/// Propagates destination failures.
pub(crate) fn serialize_spacing<Output: Serializer>(
    spacing: &SubcarrierSpacingKhz,
    serializer: Output,
) -> Result<Output::Ok, Output::Error> {
    serializer.serialize_u32(FrequencyHz::from(*spacing) * units::HZ_PER_KHZ)
}
