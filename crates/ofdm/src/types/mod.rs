/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Domain representations for decoded measurements; no PNM parser dependency.
/// Six octets of a MAC address in encoded order.
/// This alias names the representation; it does not validate address semantics.
pub type MacAddress = [u8; 6];
/// Capture timestamp encoded as unsigned epoch seconds; no calendar validation.
pub type CaptureTime = u32;
/// Encoded channel identifier; range semantics belong to the measurement format.
pub type ChannelId = u8;
/// Frequency in Hz; arithmetic must be checked against the u32 range.
pub type FrequencyHz = u32;
/// Encoded subcarrier index, matching the RxMER header's 16-bit field.
pub type SubcarrierIdx = u16;
/// Decoded [I, Q] pair: in-phase (real), then quadrature (imaginary).
pub type IqPair = [f64; 2];
/// Profile identifier encoded in one byte.
pub type ProfileId = u8;
/// Number of data bits carried by one modulation symbol.
pub type BitsPerSymbol = u8;
/// Encoded modulation assignment; unknown values remain representable.
pub type ModulationOrder = u8;
/// Ideal gross transmission rate in bits per second, before overhead.
pub type BitRateBps = u64;
/// Total coded data bits carried by one useful OFDM symbol.
pub type BitCount = u64;
/// Carrier count used by compressed-profile analysis, independent of CPU width.
pub type CarrierCount = u64;
pub mod modulation;
pub mod units {
    /// Quarter-dB encoded steps per decibel.
    pub const QUARTER_DB_PER_DB: f64 = 4.0;
}

pub mod ofdm_modulation;

/// Supported interpretation is chosen explicitly; file versions cannot infer it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolScope {
    Docsis31,
    Docsis40Fdd,
}
