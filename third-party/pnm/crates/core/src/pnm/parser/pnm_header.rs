/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! First-tier header object retaining the unprocessed binary payload.
use super::{
    model::pnm_header::{PnmHeaderModel, PnmHeaderParameters},
    pnm_file_type::{identify, FileDescriptor, HeaderLayout},
};
use crate::common::reader::Reader;
use crate::diagnostic::{ErrorCode, ParseError};
use crate::pnm::data_type::{CaptureTime, FileVersionComponent, PnmFileType};

/// File-format version, distinct from DOCSIS version and file identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PnmFormatVersion {
    pub major: FileVersionComponent,
    pub minor: FileVersionComponent,
}

/// Common metadata and remaining bytes borrowed from the original capture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PnmHeader<'input> {
    descriptor: &'static FileDescriptor,
    file_type: PnmFileType,
    format_version: PnmFormatVersion,
    capture_time: Option<CaptureTime>,
    pnm_data: &'input [u8],
    payload_offset: usize,
}
impl<'input> PnmHeader<'input> {
    /// Recognizes and decodes an evidenced common header exactly once.
    ///
    /// # Arguments
    /// * `bytes` - Complete original capture, retained for the object's lifetime.
    /// # Returns
    /// Header metadata plus the unchanged remaining payload slice.
    /// # Errors
    /// Reports unknown identity/layout/version or truncated fields with file offsets.
    #[tracing::instrument(level = "debug", skip_all, fields(input_bytes = bytes.len()), err)]
    pub fn from_bytes(bytes: &'input [u8]) -> Result<Self, ParseError> {
        let mut reader = Reader::new(bytes);
        let file_type = reader.read_bytes("file_type")?;
        let descriptor = identify(file_type).ok_or(ParseError {
            code: ErrorCode::UnsupportedType,
            offset: 0,
            field: "file_type",
        })?;
        let format_version = PnmFormatVersion {
            major: reader.read_u8("major_version")?,
            minor: reader.read_u8("minor_version")?,
        };
        if descriptor
            .header_version
            .is_some_and(|supported| format_version != supported)
        {
            return Err(ParseError {
                code: ErrorCode::UnsupportedVersion,
                offset: 4,
                field: "format_version",
            });
        }
        let layout = descriptor.layout.ok_or(ParseError {
            code: ErrorCode::UnsupportedLayout,
            offset: 4,
            field: "header_layout",
        })?;
        let capture_time = match layout {
            HeaderLayout::WithCaptureTime => Some(reader.read_u32_be("capture_time")?),
            HeaderLayout::WithoutCaptureTime => None,
        };
        tracing::debug!(
            file_type = descriptor.tag,
            payload_offset = reader.position(),
            "Parsed PNM header"
        );
        Ok(Self {
            descriptor,
            file_type,
            format_version,
            capture_time,
            pnm_data: reader.remaining(),
            payload_offset: reader.position(),
        })
    }
    /// Borrows the central identity descriptor.
    ///
    /// # Returns
    /// The single routing/layout definition used to recognize this header.
    pub fn descriptor(&self) -> &'static FileDescriptor {
        self.descriptor
    }
    /// Borrows unprocessed format-specific bytes.
    ///
    /// # Returns
    /// The original input slice immediately following the common header.
    pub fn pnm_data(&self) -> &'input [u8] {
        self.pnm_data
    }
    /// Reports the payload's position in the original file.
    ///
    /// # Returns
    /// The number of bytes consumed by the common header.
    pub fn payload_offset(&self) -> usize {
        self.payload_offset
    }
    /// Builds the header-only serializable output model.
    ///
    /// # Returns
    /// PyPNM-compatible field names; missing capture time remains None.
    pub fn to_model(&self) -> PnmHeaderModel {
        PnmHeaderModel {
            pnm_header: PnmHeaderParameters {
                file_type: self.descriptor.prefix,
                file_type_version: self.file_type[3],
                major_version: self.format_version.major,
                minor_version: self.format_version.minor,
                capture_time: self.capture_time,
            },
            data: None,
        }
    }
    /// Exports header metadata as bounded pretty JSON.
    ///
    /// # Arguments
    /// * `include_payload` - Explicitly include remaining bytes as hexadecimal data.
    /// * `max_bytes` - Maximum JSON size and payload-hex allocation budget.
    /// # Returns
    /// A complete JSON representation without modifying the binary input.
    /// # Errors
    /// Reports serialization failure or an exceeded export budget.
    pub fn to_json(&self, include_payload: bool, max_bytes: usize) -> serde_json::Result<String> {
        let mut output = self.to_model();
        if include_payload {
            let length = self
                .pnm_data
                .len()
                .checked_mul(2)
                .filter(|length| *length <= max_bytes)
                .ok_or_else(|| {
                    serde_json::Error::io(std::io::Error::other("payload hex exceeds JSON budget"))
                })?;
            let mut hex = String::with_capacity(length);
            const DIGITS: &[u8; 16] = b"0123456789abcdef";
            for &byte in self.pnm_data {
                hex.push(DIGITS[usize::from(byte >> 4)] as char);
                hex.push(DIGITS[usize::from(byte & 0x0f)] as char);
            }
            output.data = Some(hex);
        }
        crate::common::json::to_json(&output, max_bytes)
    }
}
impl HasPnmHeader for PnmHeader<'_> {
    /// Borrows this header through the shared capture interface.
    ///
    /// # Returns
    /// This header, without copying metadata or bytes.
    fn pnm_header(&self) -> &PnmHeader<'_> {
        self
    }
}
/// Implemented by captures that embed a common header.
/// Default methods provide the shared base-class API through composition.
pub trait HasPnmHeader {
    /// Borrows the header embedded in this capture.
    ///
    /// # Returns
    /// A shared reference; ownership stays with the capture.
    fn pnm_header(&self) -> &PnmHeader<'_>;
    /// Reads the encoded file-type identifier.
    ///
    /// # Returns
    /// Four identifier bytes without assuming protocol applicability.
    fn file_type(&self) -> PnmFileType {
        self.pnm_header().file_type
    }
    /// Reads optional capture time metadata.
    ///
    /// # Returns
    /// Epoch seconds when present; None remains distinct from zero.
    fn capture_time(&self) -> Option<CaptureTime> {
        self.pnm_header().capture_time
    }
    /// Reads the encoded format version.
    ///
    /// # Returns
    /// The named major/minor components, not the DOCSIS protocol version.
    fn format_version(&self) -> PnmFormatVersion {
        self.pnm_header().format_version
    }
}
