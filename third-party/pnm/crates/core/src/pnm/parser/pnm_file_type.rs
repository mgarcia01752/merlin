/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Central identities and routing; only evidenced header layouts are enabled.
use super::pnm_header::PnmFormatVersion;
use crate::pnm::data_type::PnmFileType;

// Binary signatures contain an ASCII family followed by a numeric type code.
const PNN_PREFIX: &str = "PNN";
/// Alternate PNM family prefix, reserved for future confirmed file identities.
/// Defining this prefix does not alias PNM signatures to PNN decoder routes.
pub const PNM_PREFIX: &str = "PNM";
const LLD_PREFIX: &str = "LLD";
const FILE_PREFIX_BYTES: usize = 3;
const FORMAT_VERSION_1_0: PnmFormatVersion = PnmFormatVersion { major: 1, minor: 0 };
const SYMBOL_CAPTURE_CODE: u8 = 1;
const CHANNEL_ESTIMATE_CODE: u8 = 2;
const CONSTELLATION_DISPLAY_CODE: u8 = 3;
const RXMER_CODE: u8 = 4;
const HISTOGRAM_CODE: u8 = 5;
const UPSTREAM_PRE_EQUALIZATION_CODE: u8 = 6;
const UPSTREAM_PRE_EQUALIZATION_LAST_UPDATE_CODE: u8 = 7;
const FEC_SUMMARY_CODE: u8 = 8;
const SPECTRUM_ANALYSIS_CODE: u8 = 9;
const MODULATION_PROFILE_CODE: u8 = 10;
const LATENCY_REPORT_CODE: u8 = 1;

/// Recognized PyPNM identities, independent of decoder availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PnmFileKind {
    SymbolCapture,
    ChannelEstimate,
    ConstellationDisplay,
    RxMer,
    Histogram,
    UpstreamPreEqualization,
    UpstreamPreEqualizationLastUpdate,
    FecSummary,
    SpectrumAnalysis,
    ModulationProfile,
    LatencyReport,
}
/// Common header layout, selected only after identity/version recognition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderLayout {
    WithCaptureTime,
    WithoutCaptureTime,
}
/// Routing identity; shared routes do not erase distinct file identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PnmDecoderKind {
    RxMer,
    UpstreamPreEqualization,
    ChannelEstimate,
    ConstellationDisplay,
    Histogram,
    FecSummary,
    SpectrumAnalysis,
    ModulationProfile,
    LatencyReport,
    Unsupported,
}
/// Single identity descriptor consumed by header, dispatch and output code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileDescriptor {
    pub signature: PnmFileType,
    pub kind: PnmFileKind,
    pub tag: &'static str,
    pub prefix: &'static str,
    pub header_version: Option<PnmFormatVersion>,
    /// Enabled for version 1.0 only; None means layout not yet verified here.
    pub layout: Option<HeaderLayout>,
    pub decoder: PnmDecoderKind,
}

impl FileDescriptor {
    // Build an identity with unsupported layout/decoder defaults. The prefix
    // comes only from the three-byte family constants in this module.
    const fn new(prefix: &'static str, code: u8, kind: PnmFileKind, tag: &'static str) -> Self {
        let prefix_bytes = prefix.as_bytes();
        Self {
            signature: [prefix_bytes[0], prefix_bytes[1], prefix_bytes[2], code],
            kind,
            tag,
            prefix,
            header_version: None,
            layout: None,
            decoder: PnmDecoderKind::Unsupported,
        }
    }

    // Enable only an evidenced header layout/version pair.
    const fn with_header(mut self, version: PnmFormatVersion, layout: HeaderLayout) -> Self {
        self.header_version = Some(version);
        self.layout = Some(layout);
        self
    }

    // Select a route independently of header support or decoder availability.
    const fn with_decoder(mut self, decoder: PnmDecoderKind) -> Self {
        self.decoder = decoder;
        self
    }
}

/// Central identity vocabulary; recognition alone does not enable decoding.
/// RxMER: I27 Table 68 / I13 Table 101. FEC: I27 Table 71 / I13 Table 104.
pub const FILE_TYPES: &[FileDescriptor] = &[
    FileDescriptor::new(
        PNN_PREFIX,
        SYMBOL_CAPTURE_CODE,
        PnmFileKind::SymbolCapture,
        "PNN1",
    )
    .with_header(FORMAT_VERSION_1_0, HeaderLayout::WithCaptureTime),
    FileDescriptor::new(
        PNN_PREFIX,
        CHANNEL_ESTIMATE_CODE,
        PnmFileKind::ChannelEstimate,
        "PNN2",
    )
    .with_header(FORMAT_VERSION_1_0, HeaderLayout::WithCaptureTime)
    .with_decoder(PnmDecoderKind::ChannelEstimate),
    FileDescriptor::new(
        PNN_PREFIX,
        CONSTELLATION_DISPLAY_CODE,
        PnmFileKind::ConstellationDisplay,
        "PNN3",
    )
    .with_header(FORMAT_VERSION_1_0, HeaderLayout::WithCaptureTime)
    .with_decoder(PnmDecoderKind::ConstellationDisplay),
    FileDescriptor::new(PNN_PREFIX, RXMER_CODE, PnmFileKind::RxMer, "PNN4")
        .with_header(FORMAT_VERSION_1_0, HeaderLayout::WithCaptureTime)
        .with_decoder(PnmDecoderKind::RxMer),
    FileDescriptor::new(PNN_PREFIX, HISTOGRAM_CODE, PnmFileKind::Histogram, "PNN5")
        .with_header(FORMAT_VERSION_1_0, HeaderLayout::WithCaptureTime)
        .with_decoder(PnmDecoderKind::Histogram),
    FileDescriptor::new(
        PNN_PREFIX,
        UPSTREAM_PRE_EQUALIZATION_CODE,
        PnmFileKind::UpstreamPreEqualization,
        "PNN6",
    )
    .with_header(FORMAT_VERSION_1_0, HeaderLayout::WithCaptureTime)
    .with_decoder(PnmDecoderKind::UpstreamPreEqualization),
    FileDescriptor::new(
        PNN_PREFIX,
        UPSTREAM_PRE_EQUALIZATION_LAST_UPDATE_CODE,
        PnmFileKind::UpstreamPreEqualizationLastUpdate,
        "PNN7",
    )
    .with_header(FORMAT_VERSION_1_0, HeaderLayout::WithCaptureTime)
    .with_decoder(PnmDecoderKind::UpstreamPreEqualization),
    FileDescriptor::new(
        PNN_PREFIX,
        FEC_SUMMARY_CODE,
        PnmFileKind::FecSummary,
        "PNN8",
    )
    .with_header(FORMAT_VERSION_1_0, HeaderLayout::WithoutCaptureTime)
    .with_decoder(PnmDecoderKind::FecSummary),
    FileDescriptor::new(
        PNN_PREFIX,
        SPECTRUM_ANALYSIS_CODE,
        PnmFileKind::SpectrumAnalysis,
        "PNN9",
    )
    .with_header(FORMAT_VERSION_1_0, HeaderLayout::WithCaptureTime)
    .with_decoder(PnmDecoderKind::SpectrumAnalysis),
    FileDescriptor::new(
        PNN_PREFIX,
        MODULATION_PROFILE_CODE,
        PnmFileKind::ModulationProfile,
        "PNN10",
    )
    .with_header(FORMAT_VERSION_1_0, HeaderLayout::WithCaptureTime)
    .with_decoder(PnmDecoderKind::ModulationProfile),
    FileDescriptor::new(
        LLD_PREFIX,
        LATENCY_REPORT_CODE,
        PnmFileKind::LatencyReport,
        "LLD01",
    )
    .with_header(FORMAT_VERSION_1_0, HeaderLayout::WithoutCaptureTime)
    .with_decoder(PnmDecoderKind::LatencyReport),
];

/// Resolves an exact binary signature without text normalization.
///
/// # Arguments
/// * `signature` - Original four-byte identifier, including the numeric last byte.
/// # Returns
/// The shared descriptor, or None for unrecognized identities.
pub fn identify(signature: PnmFileType) -> Option<&'static FileDescriptor> {
    FILE_TYPES
        .iter()
        .find(|descriptor| descriptor.signature == signature)
}

/// Resolves a human-readable tag using PyPNM-style case and numeric normalization.
///
/// # Arguments
/// * `tag` - Text such as PNN4, PNN10 or LLD1, with optional surrounding whitespace.
/// # Returns
/// The central descriptor; unknown prefixes, nondecimal or oversized suffixes fail.
/// This normalization never applies to binary signatures.
pub fn identify_tag(tag: &str) -> Option<&'static FileDescriptor> {
    let tag = tag.trim();
    let prefix = tag.get(..FILE_PREFIX_BYTES)?;
    let suffix = tag.get(FILE_PREFIX_BYTES..)?;
    if suffix.is_empty() || !suffix.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let discriminator = suffix.parse::<u8>().ok()?;
    FILE_TYPES.iter().find(|descriptor| {
        descriptor.prefix.eq_ignore_ascii_case(prefix)
            && descriptor.signature[FILE_PREFIX_BYTES] == discriminator
    })
}
