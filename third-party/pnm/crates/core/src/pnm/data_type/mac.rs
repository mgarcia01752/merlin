/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! One JSON representation for typed MAC fields in parsers and derived analysis.
use super::MacAddress;
use serde::Serializer;

/// Formats a MAC address as lowercase colon-separated octets.
///
/// # Arguments
/// * `mac` - Exact six-octet address.
/// * `serializer` - Output destination.
/// # Returns
/// A textual MAC address.
/// # Errors
/// Propagates destination failures.
pub(crate) fn serialize_mac<Output: Serializer>(
    mac: &MacAddress,
    serializer: Output,
) -> Result<Output::Ok, Output::Error> {
    serializer.serialize_str(&mac_text(mac))
}
/// Formats typed addresses consistently in field-specific diagnostics.
/// # Arguments
/// * `mac` - Exactly six address octets.
/// # Returns
/// Lowercase colon-separated MAC text, without changing the underlying alias.
pub(crate) fn mac_text(mac: &MacAddress) -> String {
    let [first, second, third, fourth, fifth, sixth] = *mac;
    format!("{first:02x}:{second:02x}:{third:02x}:{fourth:02x}:{fifth:02x}:{sixth:02x}")
}
