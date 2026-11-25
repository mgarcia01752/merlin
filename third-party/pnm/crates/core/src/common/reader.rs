/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::diagnostic::{ErrorCode, ParseError};

/// Cursor over borrowed bytes with checked, transactional reads.
/// Failed reads leave the cursor unchanged.
pub struct Reader<'input> {
    bytes: &'input [u8],
    offset: usize,
    origin: usize,
}
impl<'input> Reader<'input> {
    /// Starts a reader at byte offset zero.
    ///
    /// # Arguments
    /// * `bytes` - Input retained for the reader lifetime.
    ///
    /// # Returns
    /// A cursor borrowing the input without copying it.
    pub fn new(bytes: &'input [u8]) -> Self {
        Self {
            bytes,
            offset: 0,
            origin: 0,
        }
    }
    /// Starts a payload reader with original-file diagnostic offsets.
    ///
    /// # Arguments
    /// * `bytes` - Borrowed payload bytes.
    /// * `origin` - Payload offset within the original file.
    /// # Returns
    /// A reader positioned at the beginning of the payload.
    /// # Errors
    /// Returns ArithmeticOverflow if the represented file range exceeds usize.
    pub fn with_origin(bytes: &'input [u8], origin: usize) -> Result<Self, ParseError> {
        origin.checked_add(bytes.len()).ok_or(ParseError {
            code: ErrorCode::ArithmeticOverflow,
            offset: origin,
            field: "payload_offset",
        })?;
        Ok(Self {
            bytes,
            offset: 0,
            origin,
        })
    }

    /// Borrows all unread bytes without advancing the cursor.
    ///
    /// # Returns
    /// A slice into the original input, valid for its full lifetime.
    pub fn remaining(&self) -> &'input [u8] {
        &self.bytes[self.offset..]
    }

    /// Reports the next unread byte's offset in the original input.
    ///
    /// # Returns
    /// The current byte offset without advancing the reader.
    pub fn position(&self) -> usize {
        self.origin + self.offset
    }

    /// Reads one unsigned byte and advances only on success.
    ///
    /// # Arguments
    /// * `field` - Static field label used in failure diagnostics.
    ///
    /// # Returns
    /// The decoded unsigned value.
    ///
    /// # Errors
    /// Returns Truncated at the original cursor without advancing on failure.
    pub fn read_u8(&mut self, field: &'static str) -> Result<u8, ParseError> {
        Ok(self.read_bytes::<1>(field)?[0])
    }

    /// Reads a big-endian unsigned 16-bit field and advances only on success.
    ///
    /// # Arguments
    /// * `field` - Static field label used in failure diagnostics.
    ///
    /// # Returns
    /// The decoded unsigned value.
    ///
    /// # Errors
    /// Returns Truncated at the original cursor without advancing on failure.
    pub fn read_u16_be(&mut self, field: &'static str) -> Result<u16, ParseError> {
        Ok(u16::from_be_bytes(self.read_bytes(field)?))
    }

    /// Reads a big-endian unsigned 32-bit field and advances only on success.
    ///
    /// # Arguments
    /// * `field` - Static field label used in failure diagnostics.
    ///
    /// # Returns
    /// The decoded unsigned value.
    ///
    /// # Errors
    /// Returns Truncated at the original cursor without advancing on failure.
    pub fn read_u32_be(&mut self, field: &'static str) -> Result<u32, ParseError> {
        Ok(u32::from_be_bytes(self.read_bytes(field)?))
    }

    /// Borrows a variable-length field, advancing only on success.
    ///
    /// # Arguments
    /// * `length` - Required byte count.
    /// * `field` - Diagnostic field name.
    /// # Returns
    /// A slice into the original capture.
    /// # Errors
    /// Reports truncation at the original cursor without advancing.
    pub fn read_slice(
        &mut self,
        length: usize,
        field: &'static str,
    ) -> Result<&'input [u8], ParseError> {
        let error = ParseError {
            code: ErrorCode::Truncated,
            offset: self.position(),
            field,
        };
        let end = self
            .offset
            .checked_add(length)
            .ok_or_else(|| error.clone())?;
        let result = self.bytes.get(self.offset..end).ok_or(error)?;
        self.offset = end;
        Ok(result)
    }

    /// Copies a fixed-size field and advances only on success.
    ///
    /// # Arguments
    /// * `BYTE_COUNT` - Number of bytes to read.
    /// * `field` - Static field label used in failure diagnostics.
    ///
    /// # Returns
    /// Exactly BYTE_COUNT bytes in source order; zero-length reads are permitted.
    ///
    /// # Errors
    /// Returns Truncated at the original cursor if bounds or addition fail.
    pub fn read_bytes<const BYTE_COUNT: usize>(
        &mut self,
        field: &'static str,
    ) -> Result<[u8; BYTE_COUNT], ParseError> {
        let error = ParseError {
            code: ErrorCode::Truncated,
            offset: self.position(),
            field,
        };
        let end_exclusive = self
            .offset
            .checked_add(BYTE_COUNT)
            .ok_or_else(|| error.clone())?;
        let slice = self.bytes.get(self.offset..end_exclusive).ok_or(error)?;
        let mut result = [0; BYTE_COUNT];
        result.copy_from_slice(slice);
        self.offset = end_exclusive;
        Ok(result)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn typed_reads_decode_big_endian_fields() {
        let mut reader = Reader::new(&[0xab, 0x12, 0x34, 0x89, 0xab, 0xcd, 0xef]);
        assert_eq!(reader.read_u8("channel").unwrap(), 0xab);
        assert_eq!(reader.read_u16_be("index").unwrap(), 0x1234);
        assert_eq!(reader.read_u32_be("frequency").unwrap(), 0x89abcdef);
        let failure = reader.read_u8("past_end").unwrap_err();
        assert_eq!(failure.code, ErrorCode::Truncated);
        assert_eq!(failure.offset, 7);
        assert_eq!(failure.field, "past_end");
    }

    #[test]
    fn failed_typed_reads_preserve_cursor_and_diagnostics() {
        let mut reader = Reader::new(&[0xaa, 0x12, 0x34, 0x56]);
        assert_eq!(reader.read_u8("prefix").unwrap(), 0xaa);
        let failure = reader.read_u32_be("frequency").unwrap_err();
        assert_eq!(failure.code, ErrorCode::Truncated);
        assert_eq!(failure.offset, 1);
        assert_eq!(failure.field, "frequency");
        assert_eq!(reader.read_u16_be("index").unwrap(), 0x1234);
        let failure = reader.read_u16_be("short_index").unwrap_err();
        assert_eq!(failure.code, ErrorCode::Truncated);
        assert_eq!(failure.offset, 3);
        assert_eq!(failure.field, "short_index");
        assert_eq!(reader.read_u8("last").unwrap(), 0x56);
    }

    #[test]
    fn truncated_read_does_not_advance() {
        let mut reader = Reader::new(&[1, 2, 3]);
        assert_eq!(reader.read_bytes::<2>("first").unwrap(), [1, 2]);
        assert_eq!(reader.read_bytes::<2>("second").unwrap_err().offset, 2);
        assert_eq!(reader.read_bytes::<1>("last").unwrap(), [3]);
    }
}
