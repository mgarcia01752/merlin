/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Length, routing and resource validation for measurement payloads.
use super::super::{
    parse_limits::ParseLimits, pnm_file_type::PnmDecoderKind, pnm_header::PnmHeader,
};
use crate::{
    common::reader::Reader,
    diagnostic::{ErrorCode, ParseError},
};

/// Constructs a diagnostic in original-file coordinates.
///
/// # Arguments
/// * `code` - Failure category.
/// * `offset` - Original file byte position.
/// * `field` - Static field identifier.
/// # Returns
/// A structured parse diagnostic.
pub(crate) fn error(code: ErrorCode, offset: usize, field: &'static str) -> ParseError {
    ParseError {
        code,
        offset,
        field,
    }
}
/// Verifies that a concrete parser received its registered route.
///
/// # Arguments
/// * `header` - Identified first-tier header.
/// * `decoder` - Required route.
/// # Returns
/// Success when the routes match.
/// # Errors
/// Reports UnsupportedType for another measurement family.
pub(crate) fn require_decoder(
    header: &PnmHeader<'_>,
    decoder: PnmDecoderKind,
) -> Result<(), ParseError> {
    if header.descriptor().decoder != decoder {
        return Err(error(ErrorCode::UnsupportedType, 0, "file_type"));
    }
    Ok(())
}
/// Requires all declared payload bytes to have been consumed.
///
/// # Arguments
/// * `reader` - Cursor following the last expected field.
/// # Returns
/// Success at the exact end of input.
/// # Errors
/// Reports LengthMismatch at the first trailing byte.
pub(crate) fn finish(reader: &Reader<'_>) -> Result<(), ParseError> {
    if !reader.remaining().is_empty() {
        return Err(error(
            ErrorCode::LengthMismatch,
            reader.position(),
            "trailing_data",
        ));
    }
    Ok(())
}
/// Charges decoded items against a shared application budget.
///
/// # Arguments
/// * `count` - Items to consume.
/// * `remaining` - Budget updated only on success.
/// * `offset` - Original file position for diagnostics.
/// # Returns
/// Success with the remaining allowance reduced.
/// # Errors
/// Reports LimitExceeded before allocating or decoding excess items.
pub(crate) fn budget(count: usize, remaining: &mut usize, offset: usize) -> Result<(), ParseError> {
    *remaining = remaining
        .checked_sub(count)
        .ok_or_else(|| error(ErrorCode::LimitExceeded, offset, "sample_count"))?;
    Ok(())
}
/// Reads a length-prefixed sample array without copying it.
///
/// # Arguments
/// * `reader` - Cursor at the u32 big-endian byte length.
/// * `width` - Nonzero encoded sample width supplied by the parser.
/// * `limits` - Application sample budget.
/// # Returns
/// A borrowed sample slice with integral sample count.
/// # Errors
/// Reports truncation, a fractional sample or an exceeded budget.
pub(crate) fn payload<'input>(
    reader: &mut Reader<'input>,
    width: usize,
    limits: ParseLimits,
) -> Result<&'input [u8], ParseError> {
    let offset = reader.position();
    let length = usize::try_from(reader.read_u32_be("data_length")?)
        .map_err(|_| error(ErrorCode::LimitExceeded, offset, "data_length"))?;
    if length % width != 0 {
        return Err(error(ErrorCode::LengthMismatch, offset, "data_length"));
    }
    let mut remaining = limits.max_samples;
    budget(length / width, &mut remaining, offset)?;
    reader.read_slice(length, "values")
}
