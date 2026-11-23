/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Immutable original measurement files decoded through the pinned PNM library.
use crate::downstream::common::constants::MAX_MEASUREMENT_COLLECTION_ORIGINAL_BYTES;
use crate::downstream::common::constants::UTF8_JSON_BYTE_ORDER_MARK;
use merlin::downstream::{
    inputs::CaptureLoader, CaptureKind, CaptureProvenance, ChannelEstimateFrame, NamedCapture,
    RxMerFrame, ScenarioKey, MAX_CAPTURE_BYTES,
};
use std::sync::Arc;

/// Authoritative public input-byte ceiling; the picker and admission share this bound.
pub(crate) const MAX_INPUT_BYTES: usize = MAX_CAPTURE_BYTES;
/// Independent legacy editor retention policy, shared with its batch/Designer adapters.
pub(crate) const MAX_TOTAL_BYTES: usize = MAX_MEASUREMENT_COLLECTION_ORIGINAL_BYTES;

#[derive(Clone)]
pub(crate) enum Measurement {
    Rxmer(Arc<RxMerFrame>),
    Response(Arc<ChannelEstimateFrame>),
}
#[derive(Clone)]
pub(crate) struct Capture {
    pub name: String,
    pub original: Arc<Vec<u8>>,
    pub format: &'static str,
    pub hash: String,
    pub decoded_hash: String,
    pub measurement: Measurement,
    pub designer: Option<Arc<super::designer::Origin>>,
}
impl Capture {
    /// Decodes a single original input, detecting contents rather than extensions.
    /// # Arguments
    /// * `name` - Local display identity.
    /// * `bytes` - Bounded original bytes retained unchanged.
    /// * `rxmer` - Expected measurement type; wrong types reject.
    /// # Returns
    /// Typed Rust observation and both original/decoded hashes.
    /// # Errors
    /// Reports input format, decoder, shared JSON contract or resource diagnostics.
    pub(crate) fn decode(name: String, bytes: Vec<u8>, rxmer: bool) -> Result<Self, String> {
        if bytes.len() > MAX_INPUT_BYTES {
            return Err("Input exceeds 16 MiB.".into());
        }
        // Preserve the existing Designer-only JSON transport normalization.
        // Ordinary capture admission below retains its complete original bytes.
        if rxmer {
            let json = bytes
                .strip_prefix(UTF8_JSON_BYTE_ORDER_MARK)
                .unwrap_or(&bytes);
            if let Some(capture) = super::designer::decode(json)? {
                return Ok(capture);
            }
        }
        let original =
            NamedCapture::new(&name, bytes).map_err(|cause| cause.message().to_owned())?;
        let (format, hash, decoded_hash, measurement) = if rxmer {
            let capture =
                CaptureLoader::rxmer(&original).map_err(|cause| cause.message().to_owned())?;
            let format = display_format(capture.provenance())?;
            let hash = capture.provenance().original_sha256().to_owned();
            let mut frame = capture.frame().clone();
            let decoded_hash = frame.source.sha256.clone();

            // Preserve the existing editor DTO's original-file metadata projection.
            // The public observation and its decoded/original provenance stay unchanged.
            frame.source.sha256 = hash.clone();
            frame.source.byte_length = original.bytes().len();
            frame.source.file_kind = if format == "Binary PNM" {
                "binary_pnm_rxmer"
            } else {
                "decoded_json_rxmer"
            }
            .into();
            (
                format,
                hash,
                decoded_hash,
                Measurement::Rxmer(Arc::new(frame)),
            )
        } else {
            let capture = CaptureLoader::channel_estimate(&original)
                .map_err(|cause| cause.message().to_owned())?;
            let format = display_format(capture.provenance())?;
            let hash = capture.provenance().original_sha256().to_owned();
            let mut frame = capture.frame().clone();
            let decoded_hash = frame.source.sha256.clone();
            frame.source.sha256 = hash.clone();
            frame.source.byte_length = original.bytes().len();
            frame.source.file_kind = if format == "Binary PNM" {
                "binary_pnm_channel_response"
            } else {
                "decoded_json_channel_response"
            }
            .into();
            (
                format,
                hash,
                decoded_hash,
                Measurement::Response(Arc::new(frame)),
            )
        };
        Ok(Self {
            name,
            original: Arc::new(original.bytes().to_vec()),
            format,
            hash,
            decoded_hash,
            measurement,
            designer: None,
        })
    }
    /// Admits this unchanged original/selected DTO through the public RxMER owner.
    /// # Returns
    /// Immutable original-grid observation, including existing Designer metadata.
    /// # Errors
    /// Rejects the wrong type or invalid frame without changing this input.
    pub(crate) fn rxmer(&self) -> Result<merlin::downstream::RxMer, String> {
        let Measurement::Rxmer(frame) = &self.measurement else {
            return Err("Expected RxMER input.".into());
        };
        let original = NamedCapture::new(&self.name, self.original.as_ref().clone())
            .map_err(|cause| cause.message().to_owned())?;
        merlin::downstream::RxMer::from_frame(&original, frame.as_ref().clone())
            .map_err(|cause| cause.message().to_owned())
    }

    /// Reads the exact original response through public native admission.
    /// # Returns
    /// Immutable H with separate original and decoded provenance.
    /// # Errors
    /// Rejects the wrong type or invalid original without changing this input.
    pub(crate) fn response(&self) -> Result<merlin::downstream::ChannelEstimate, String> {
        if !matches!(&self.measurement, Measurement::Response(_)) {
            return Err("Expected a channel-response capture.".into());
        }
        let original = NamedCapture::new(&self.name, self.original.as_ref().clone())
            .map_err(|cause| cause.message().to_owned())?;
        CaptureLoader::channel_estimate(&original).map_err(|cause| cause.message().to_owned())
    }

    /// Borrows the exact capture association key without reconstructing coordinates.
    /// # Returns
    /// Original device/channel/grid identity.
    pub(crate) fn key(&self) -> &ScenarioKey {
        match &self.measurement {
            Measurement::Rxmer(frame) => &frame.key,
            Measurement::Response(frame) => &frame.key,
        }
    }
    /// Reports original capture epoch, never synthetic playback timing.
    /// # Returns
    /// Epoch seconds from the PNM header.
    pub(crate) fn epoch(&self) -> u32 {
        match &self.measurement {
            Measurement::Rxmer(frame) => frame.capture_time,
            Measurement::Response(frame) => frame.capture_time,
        }
    }
}

// Detected representation is public provenance-report metadata, only used for labels.
fn display_format(provenance: &CaptureProvenance) -> Result<&'static str, String> {
    let report = serde_json::to_value(provenance).map_err(|cause| cause.to_string())?;
    let binary = serde_json::to_value(CaptureKind::BinaryPnm).map_err(|cause| cause.to_string())?;
    let json = serde_json::to_value(CaptureKind::DecodedJson).map_err(|cause| cause.to_string())?;
    if report["original_kind"] == binary {
        Ok("Binary PNM")
    } else if report["original_kind"] == json {
        Ok("Decoded JSON")
    } else {
        Err("Unsupported measurement representation.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use sha2::{Digest, Sha256};

    #[test]
    fn public_ordinary_admission_preserves_binary_bom_hashes_and_legacy_source_metadata() {
        let binary =
            include_bytes!("../../../../../../examples/workbench/single/ds-channel-estimate.bin");
        let capture = Capture::decode("wrong.json".into(), binary.to_vec(), false).unwrap();
        let Measurement::Response(frame) = &capture.measurement else {
            panic!("Expected channel response");
        };
        assert_eq!(capture.format, "Binary PNM");
        assert_eq!(capture.original.as_slice(), binary);
        assert_eq!(capture.hash, format!("{:x}", Sha256::digest(binary)));
        assert_eq!(frame.source.sha256, capture.hash);
        assert_eq!(frame.source.byte_length, binary.len());
        assert_eq!(frame.source.file_kind, "binary_pnm_channel_response");
        assert_ne!(capture.hash, capture.decoded_hash);
        assert_eq!(frame.key.channel_id, 34);
        assert_eq!(frame.capture_time, 1391100);
        let public = CaptureLoader::channel_estimate(
            &NamedCapture::new("wrong.json", binary.to_vec()).unwrap(),
        )
        .unwrap();
        assert_eq!(frame.iq, public.frame().iq);
        assert_eq!(capture.decoded_hash, public.frame().source.sha256);
        assert_ne!(frame.source.sha256, public.frame().source.sha256);
        let mut source: Value = serde_json::from_slice(include_bytes!(
            "../../../../../../examples/measurements/single-channel/rxmer.json"
        ))
        .unwrap();

        // Independent null/level and UTF-8 BOM anchors preserve the legacy decoded hash.
        source["values"][0] = Value::Null;
        source["values"][1] = json!(42.75);
        let decoded = serde_json::to_vec(&source).unwrap();
        let mut original = b"\xef\xbb\xbf".to_vec();
        original.extend(&decoded);
        let capture = Capture::decode("wrong.bin".into(), original.clone(), true).unwrap();
        let Measurement::Rxmer(frame) = &capture.measurement else {
            panic!("Expected RxMER");
        };
        assert_eq!(capture.format, "Decoded JSON");
        assert_eq!(capture.original.as_ref(), &original);
        assert_eq!(capture.hash, format!("{:x}", Sha256::digest(&original)));
        assert_eq!(
            capture.decoded_hash,
            format!("{:x}", Sha256::digest(&decoded))
        );
        assert_eq!(frame.source.sha256, capture.hash);
        assert_eq!(frame.source.byte_length, original.len());
        assert_eq!(frame.source.file_kind, "decoded_json_rxmer");
        assert_eq!(frame.mer_db[0], None);
        assert_eq!(frame.mer_db[1], Some(42.75));
    }
}
