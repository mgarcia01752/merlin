/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Original-byte container and the sole concrete parser dispatch path.
use super::cm_ds_const_disp_meas::CmDsConstDispMeas;
use super::cm_ds_hist::CmDsHist;
use super::cm_ds_ofdm_chan_estimate_coef::CmDsOfdmChanEstimateCoef;
use super::cm_ds_ofdm_fec_summary::CmDsOfdmFecSummary;
use super::cm_ds_ofdm_modulation_profile::CmDsOfdmModulationProfile;
use super::cm_latency_rpt::CmLatencyRpt;
use super::cm_spectrum_analysis::CmSpectrumAnalysis;
use super::cm_us_ofdma_pre_eq::CmUsOfdmaPreEq;
use super::{
    cm_ds_ofdm_rx_mer::{CmDsOfdmRxMer, ParseLimits},
    pnm_file_type::PnmDecoderKind,
    pnm_header::PnmHeader,
};
use crate::diagnostic::{ErrorCode, ParseError};
use serde::{Serialize, Serializer};

/// An imported byte view, retaining diagnostics when the header is unsupported.
#[derive(Debug)]
pub struct PnmFile<'input> {
    bytes: &'input [u8],
    header: Result<PnmHeader<'input>, ParseError>,
}
/// Implemented second-tier decoder results; add variants as decoders land.
#[derive(Debug, PartialEq)]
pub enum DecodedPnm<'input> {
    RxMer(CmDsOfdmRxMer<'input>),
    ChannelEstimate(CmDsOfdmChanEstimateCoef<'input>),
    ConstellationDisplay(CmDsConstDispMeas<'input>),
    Histogram(CmDsHist<'input>),
    UpstreamPreEqualization(CmUsOfdmaPreEq<'input>),
    FecSummary(CmDsOfdmFecSummary<'input>),
    SpectrumAnalysis(CmSpectrumAnalysis<'input>),
    ModulationProfile(CmDsOfdmModulationProfile<'input>),
    LatencyReport(CmLatencyRpt<'input>),
}
impl<'input> PnmFile<'input> {
    /// Inspects common metadata while retaining even unknown or malformed files.
    ///
    /// # Arguments
    /// * `bytes` - Original bytes owned by the caller for this object's lifetime.
    /// # Returns
    /// An inspectable file; header failures are available through header().
    pub fn from_bytes(bytes: &'input [u8]) -> Self {
        Self {
            bytes,
            header: PnmHeader::from_bytes(bytes),
        }
    }
    /// Borrows the complete unchanged file.
    ///
    /// # Returns
    /// Original bytes, including when header recognition failed.
    pub fn bytes(&self) -> &'input [u8] {
        self.bytes
    }
    /// Borrows the first-tier header or its recognition diagnostic.
    ///
    /// # Returns
    /// Parsed header and payload view on success.
    /// # Errors
    /// Reports the original header diagnostic without attempting to decode again.
    pub fn header(&self) -> Result<&PnmHeader<'input>, &ParseError> {
        self.header.as_ref()
    }
    /// Dispatches the parsed header to its central decoder route.
    ///
    /// # Arguments
    /// * `limits` - Caller resource policy for samples and nested records.
    /// # Returns
    /// Typed decoded data; the file remains available for raw inspection.
    /// # Errors
    /// Propagates header/payload errors or reports a known but unavailable decoder.
    #[tracing::instrument(level = "debug", skip_all, fields(input_bytes = self.bytes.len()), err)]
    pub fn decode(&self, limits: ParseLimits) -> Result<DecodedPnm<'input>, ParseError> {
        let header = self.header.as_ref().map_err(Clone::clone)?;
        match header.descriptor().decoder {
            PnmDecoderKind::RxMer => Ok(DecodedPnm::RxMer(CmDsOfdmRxMer::from_header(
                header.clone(),
                limits,
            )?)),
            PnmDecoderKind::ChannelEstimate => Ok(DecodedPnm::ChannelEstimate(
                CmDsOfdmChanEstimateCoef::from_header(header.clone(), limits)?,
            )),
            PnmDecoderKind::ConstellationDisplay => Ok(DecodedPnm::ConstellationDisplay(
                CmDsConstDispMeas::from_header(header.clone(), limits)?,
            )),
            PnmDecoderKind::Histogram => Ok(DecodedPnm::Histogram(CmDsHist::from_header(
                header.clone(),
                limits,
            )?)),
            PnmDecoderKind::UpstreamPreEqualization => Ok(DecodedPnm::UpstreamPreEqualization(
                CmUsOfdmaPreEq::from_header(header.clone(), limits)?,
            )),
            PnmDecoderKind::FecSummary => Ok(DecodedPnm::FecSummary(
                CmDsOfdmFecSummary::from_header(header.clone(), limits)?,
            )),
            PnmDecoderKind::SpectrumAnalysis => Ok(DecodedPnm::SpectrumAnalysis(
                CmSpectrumAnalysis::from_header(header.clone(), limits)?,
            )),
            PnmDecoderKind::ModulationProfile => Ok(DecodedPnm::ModulationProfile(
                CmDsOfdmModulationProfile::from_header(header.clone(), limits)?,
            )),
            PnmDecoderKind::LatencyReport => Ok(DecodedPnm::LatencyReport(
                CmLatencyRpt::from_header(header.clone(), limits)?,
            )),
            _ => Err(ParseError {
                code: ErrorCode::UnsupportedDecoder,
                offset: 0,
                field: "file_type",
            }),
        }
    }
}

impl DecodedPnm<'_> {
    /// Serializes the dispatched measurement directly to bounded JSON.
    ///
    /// # Arguments
    /// * `max_bytes` - Maximum output bytes.
    /// # Returns
    /// The concrete capture JSON without an extra enum wrapper.
    /// # Errors
    /// Propagates serializer and output-budget failures.
    pub fn to_json(&self, max_bytes: usize) -> serde_json::Result<String> {
        crate::common::json::to_json(self, max_bytes)
    }
}
impl Serialize for DecodedPnm<'_> {
    /// Delegates JSON serialization to the concrete capture object.
    ///
    /// # Arguments
    /// * `serializer` - Output destination.
    /// # Returns
    /// Header and format-specific measurement fields.
    /// # Errors
    /// Propagates destination errors.
    fn serialize<Output: Serializer>(
        &self,
        serializer: Output,
    ) -> Result<Output::Ok, Output::Error> {
        match self {
            Self::RxMer(capture) => capture.serialize(serializer),
            Self::ChannelEstimate(capture) => capture.serialize(serializer),
            Self::ConstellationDisplay(capture) => capture.serialize(serializer),
            Self::Histogram(capture) => capture.serialize(serializer),
            Self::UpstreamPreEqualization(capture) => capture.serialize(serializer),
            Self::FecSummary(capture) => capture.serialize(serializer),
            Self::SpectrumAnalysis(capture) => capture.serialize(serializer),
            Self::ModulationProfile(capture) => capture.serialize(serializer),
            Self::LatencyReport(capture) => capture.serialize(serializer),
        }
    }
}
