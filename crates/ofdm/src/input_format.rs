/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Content classification shared by file adapters; extensions are never evidence.

/// Coarse input representation, separate from decoder identity/version validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    BinaryPnm,
    JsonText,
    UnsupportedText,
    UnsupportedBinary,
    Empty,
}

/// Classifies bounded caller-owned bytes without decoding or modifying them.
///
/// # Arguments
/// * `bytes` - File contents already bounded by the owning filesystem adapter.
/// # Returns
/// A content hint. PNM prefixes and JSON delimiters still require full validation.
/// JSON may have an initial UTF-8 BOM; other text encodings are not converted.
pub fn detect(bytes: &[u8]) -> InputFormat {
    if bytes.starts_with(b"PNN") || bytes.starts_with(b"PNM") {
        return InputFormat::BinaryPnm;
    }
    let text = json_bytes(bytes);
    let first = text.iter().find(|byte| !byte.is_ascii_whitespace());
    match first {
        None => InputFormat::Empty,
        Some(b'{' | b'[') => InputFormat::JsonText,
        _ if std::str::from_utf8(bytes).is_ok()
            && bytes
                .iter()
                .all(|byte| !byte.is_ascii_control() || byte.is_ascii_whitespace()) =>
        {
            InputFormat::UnsupportedText
        }
        _ => InputFormat::UnsupportedBinary,
    }
}

/// Borrows JSON bytes after an optional UTF-8 BOM, preserving the caller's input.
/// # Arguments
/// * `bytes` - Original file contents, also retained separately for provenance.
/// # Returns
/// The input slice with only an initial UTF-8 BOM skipped, when present.
pub fn json_bytes(bytes: &[u8]) -> &[u8] {
    bytes.strip_prefix(&[0xef, 0xbb, 0xbf]).unwrap_or(bytes)
}
