/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Optional Apache-2.0 file/binary adapter. No Qt, Python or subprocess dependency.
use docsis_ofdm_fec::{
    api::*,
    input_format::{detect, InputFormat},
    scenario::pnm_json::DecoderContract,
};
use pnm::pnm::parser::{
    cm_ds_ofdm_chan_estimate_coef::CmDsOfdmChanEstimateCoef,
    cm_ds_ofdm_modulation_profile::CmDsOfdmModulationProfile, cm_ds_ofdm_rx_mer::CmDsOfdmRxMer,
    parse_limits::ParseLimits,
};
use std::{
    io::{Read, Write},
    path::Path,
};
/// Stateless native decoder dispatch; classification is based only on contents.
pub struct CaptureLoader;
fn decode(input: &NamedCapture, kind: u8) -> ApiResult<(NamedCapture, Option<DecoderContract>)> {
    if detect(input.bytes()) != InputFormat::BinaryPnm {
        return Ok((input.clone(), None));
    }
    let limits = ParseLimits::default();
    let json = match kind {
        10 => CmDsOfdmModulationProfile::parse(input.bytes(), limits)
            .map_err(|e| ApiError::new(ErrorCode::UnsupportedFormat, e.to_string()))?
            .to_json(MAX_CAPTURE_BYTES),
        4 => CmDsOfdmRxMer::parse(input.bytes(), limits)
            .map_err(|e| ApiError::new(ErrorCode::UnsupportedFormat, e.to_string()))?
            .to_json(MAX_CAPTURE_BYTES),
        2 => CmDsOfdmChanEstimateCoef::parse(input.bytes(), limits)
            .map_err(|e| ApiError::new(ErrorCode::UnsupportedFormat, e.to_string()))?
            .to_json(MAX_CAPTURE_BYTES),
        _ => {
            return Err(ApiError::new(
                ErrorCode::UnsupportedCapability,
                "Unsupported capture kind",
            ))
        }
    }
    .map_err(|e| ApiError::new(ErrorCode::ResourceLimit, e.to_string()))?;
    Ok((
        NamedCapture::new(input.name(), json.into_bytes())?,
        Some(DecoderContract {
            revision: 1,
            producer: "pnm-decode".into(),
            producer_version: "0.3.6".into(),
        }),
    ))
}
impl CaptureLoader {
    /// Loads a single selected profile from supported binary PNM or decoded JSON.
    /// # Arguments
    /// * `input` - Bounded original named bytes, extensions ignored.
    /// * `id` - Explicit captured profile ID.
    /// # Returns
    /// Immutable high-level API input; no simulation runs.
    /// # Errors
    /// Rejects wrong kinds, invalid lengths/versions and bounded decoder failures.
    pub fn profile(input: &NamedCapture, id: ProfileId) -> ApiResult<Dpd> {
        let (decoded, contract) = decode(input, 10)?;
        Dpd::from_decoded(input, &decoded, id, contract)
    }
    /// Loads one RxMER without requiring a channel or profile.
    /// # Arguments
    /// * `input` - Bounded original binary or JSON bytes.
    /// # Returns
    /// Immutable exact-grid observation with original and intermediate provenance.
    /// # Errors
    /// Rejects malformed/wrong-kind/unsupported capture data.
    pub fn rxmer(input: &NamedCapture) -> ApiResult<RxMer> {
        let (decoded, contract) = decode(input, 4)?;
        RxMer::from_decoded(input, &decoded, contract)
    }
    /// Loads one channel response independently of RxMER.
    /// # Arguments
    /// * `input` - Bounded original binary or JSON bytes.
    /// # Returns
    /// Immutable original complex H and provenance.
    /// # Errors
    /// Rejects malformed/wrong-kind/unsupported capture data.
    pub fn channel_estimate(input: &NamedCapture) -> ApiResult<ChannelEstimate> {
        let (decoded, contract) = decode(input, 2)?;
        ChannelEstimate::from_decoded(input, &decoded, contract)
    }
}
/// Reads at most the input budget plus one detection byte, including special files.
/// # Arguments
/// * `path` - Caller-selected input; opened read-only.
/// # Returns
/// Bounded exact original bytes with display identity.
/// # Errors
/// Returns structured I/O and input-budget errors; never writes the source.
pub fn read_capture(path: impl AsRef<Path>) -> ApiResult<NamedCapture> {
    let path = path.as_ref();
    let file = std::fs::File::open(path).map_err(io_error)?;
    let mut bytes = Vec::new();
    file.take((MAX_CAPTURE_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(io_error)?;
    NamedCapture::new(path.to_string_lossy(), bytes)
}
fn io_error(error: std::io::Error) -> ApiError {
    ApiError::new(ErrorCode::Io, error.to_string())
}
/// Optional single-kind measurement file conveniences.
pub trait FromFile: Sized {
    /// Loads a supported measurement through the bounded native byte adapter.
    /// # Arguments
    /// * `path` - Read-only input path, never a format hint.
    /// # Returns
    /// Immutable validated input; no simulation.
    /// # Errors
    /// Preserves structured file, decoder and validation failures.
    fn from_file(path: impl AsRef<Path>) -> ApiResult<Self>;
}
/// Profile loading always requires explicit selection of the captured identity.
pub trait ProfileFromFile: Sized {
    /// Loads a selected profile through the bounded native byte adapter.
    /// # Arguments
    /// * `path` - Read-only input path, never a format hint.
    /// * `id` - Captured profile identity, including multi-profile captures.
    /// # Returns
    /// Immutable selected profile; no simulation.
    /// # Errors
    /// Preserves structured I/O, decoder, validation and missing-ID failures.
    fn from_file(path: impl AsRef<Path>, id: ProfileId) -> ApiResult<Self>;
}
impl ProfileFromFile for Dpd {
    /// Loads an explicitly selected profile without changing any source bytes.
    /// # Arguments
    /// * `path` - Binary or JSON input file.
    /// * `id` - Captured profile identity.
    /// # Returns
    /// Owned immutable profile; simulation remains unstarted.
    /// # Errors
    /// Returns the same bounded input/decoder errors as CaptureLoader.
    fn from_file(path: impl AsRef<Path>, id: ProfileId) -> ApiResult<Self> {
        CaptureLoader::profile(&read_capture(path)?, id)
    }
}
impl FromFile for RxMer {
    /// Loads one independent original RxMER capture.
    /// # Arguments
    /// * `path` - Bounded binary or JSON input file.
    /// # Returns
    /// Owned immutable observation retaining original bytes.
    /// # Errors
    /// Returns the same input/decoder errors as CaptureLoader.
    fn from_file(path: impl AsRef<Path>) -> ApiResult<Self> {
        CaptureLoader::rxmer(&read_capture(path)?)
    }
}
impl FromFile for ChannelEstimate {
    /// Loads one independent original channel-response capture.
    /// # Arguments
    /// * `path` - Bounded binary or JSON input file.
    /// # Returns
    /// Owned immutable response retaining original bytes.
    /// # Errors
    /// Returns the same input/decoder errors as CaptureLoader.
    fn from_file(path: impl AsRef<Path>) -> ApiResult<Self> {
        CaptureLoader::channel_estimate(&read_capture(path)?)
    }
}
/// Writes an already serialized report to a new path, never replacing an input.
/// # Arguments
/// * `path` - New output destination.
/// * `json` - Owned report serialization produced by the Rust high-level API.
/// # Returns
/// Unit after writing; an existing destination is untouched.
/// # Errors
/// Returns structured I/O errors, including already-existing destinations.
pub fn save_json(path: impl AsRef<Path>, json: &str) -> ApiResult<()> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(io_error)?;
    file.write_all(json.as_bytes()).map_err(io_error)
}
