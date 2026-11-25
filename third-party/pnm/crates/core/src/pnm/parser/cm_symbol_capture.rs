/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Symbol capture is intentionally unsupported pending a verified implementation.
use super::pnm_header::{HasPnmHeader, PnmHeader};
/// Header-only symbol capture view. No semantic decoder is registered.
#[derive(Debug, Clone, PartialEq)]
pub struct CmSymbolCapture<'input> {
    header: PnmHeader<'input>,
}
impl<'input> CmSymbolCapture<'input> {
    /// Recognizes symbol capture without interpreting its measurement payload.
    ///
    /// # Arguments
    /// * `bytes` - Complete capture retained by the caller.
    /// # Returns
    /// A header-only view; the payload remains available through pnm_header().
    /// # Errors
    /// Reports unsupported identity/version or a truncated common header.
    pub fn parse(bytes: &'input [u8]) -> Result<Self, crate::diagnostic::ParseError> {
        let header = PnmHeader::from_bytes(bytes)?;
        if header.descriptor().kind != super::pnm_file_type::PnmFileKind::SymbolCapture {
            return Err(super::support::payload::error(
                crate::diagnostic::ErrorCode::UnsupportedType,
                0,
                "file_type",
            ));
        }
        Ok(Self { header })
    }
}
impl HasPnmHeader for CmSymbolCapture<'_> {
    /// Borrows the common header without decoding symbol data.
    ///
    /// # Returns
    /// The header and unchanged original payload.
    fn pnm_header(&self) -> &PnmHeader<'_> {
        &self.header
    }
}
