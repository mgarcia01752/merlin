/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared PNM data types and unit definitions.
pub mod modulation;
pub mod units;

/// Four-byte PNM file identifier in its encoded byte order.
/// This alias provides a domain name; it does not validate supported identifiers.
pub type PnmFileType = [u8; 4];

/// Six octets of a MAC address in encoded order.
/// This alias names the representation; it does not validate address semantics.
pub type MacAddress = [u8; 6];

/// Capture timestamp encoded as unsigned epoch seconds; no calendar validation.
pub type CaptureTime = u32;
/// Encoded channel identifier; range semantics belong to the measurement format.
pub type ChannelId = u8;
/// Encoded subcarrier index, matching the RxMER header's 16-bit field.
pub type SubcarrierIdx = u16;
/// Frequency in Hz; arithmetic must be checked against the u32 range.
pub type FrequencyHz = u32;
/// Encoded subcarrier spacing in kHz; no PHY spacing validation is implied.
pub type SubcarrierSpacingKhz = u8;
/// One encoded file-version component; not a DOCSIS protocol version.
pub type FileVersionComponent = u8;
/// Length in bytes as encoded in a 32-bit file field.
pub type DataLengthBytes = u32;
/// Decoder/application sample budget or in-memory sample count.
pub type SampleCount = usize;
/// Decoded RxMER in decibels; unavailable measurements are represented by Option.
pub type RxMerDb = f64;

/// Decoded [I, Q] pair: in-phase (real), then quadrature (imaginary).
pub type IqPair = [f64; 2];
/// Unsigned measurement counter encoded in four bytes.
pub type CounterValue = u32;
/// Profile identifier encoded in one byte.
pub type ProfileId = u8;
/// Number of profiles encoded in one byte.
pub type ProfileCount = u8;
/// Number of data bits carried by one modulation symbol.
pub type BitsPerSymbol = u8;
/// Encoded modulation assignment; unknown values remain representable.
pub type ModulationOrder = u8;
/// Constellation modulation selection encoded in two bytes.
pub type ConstellationModulationOrder = u16;
/// Constellation symbol count encoded in two bytes.
pub type SymbolCount = u16;
/// Encoded histogram symmetry flag.
pub type HistogramSymmetry = u8;
/// Encoded FEC summary selection.
pub type FecSummaryType = u8;
/// Encoded record count per profile.
pub type RecordCount = u16;
/// Spectrum amplitude in dB referenced to 0 dBmV.
pub type AmplitudeDb = f64;
/// Encoded spectrum bin count per segment.
pub type SegmentBinCount = u16;
/// Encoded spectrum noise bandwidth parameter.
pub type NoiseBandwidth = u16;
/// Encoded spectrum window selection.
pub type WindowFunction = u16;

/// Service-flow identifier encoded in four bytes.
pub type ServiceFlowId = u32;
/// Exact 16-byte service-flow label, including padding.
pub type ServiceFlowLabel = [u8; 16];
/// Latency histogram edge in hundredths of a millisecond.
pub type LatencyBinEdge = u16;
/// Latency in microseconds.
pub type LatencyUs = u32;
/// Latency histogram packet counter encoded in eight bytes.
pub type PacketCount = u64;

/// Carrier count used by compressed-profile analysis, independent of CPU width.
pub type CarrierCount = u64;
/// Total coded data bits carried by one useful OFDM symbol.
pub type BitCount = u64;
/// Ideal gross transmission rate in bits per second, before overhead.
pub type BitRateBps = u64;

mod mac;
pub(crate) use mac::{mac_text, serialize_mac};
