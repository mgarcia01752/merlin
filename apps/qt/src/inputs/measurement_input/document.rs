/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Immutable original measurement files decoded through the pinned PNM library.
use docsis_ofdm_fec::{
    input_format::{detect, json_bytes, InputFormat},
    scenario::{
        pnm_json::{import_channel, import_rxmer, JsonCapture},
        ChannelFrame, RxMerFrame, ScenarioKey,
    },
};
use pnm::pnm::parser::{
    cm_ds_ofdm_chan_estimate_coef::CmDsOfdmChanEstimateCoef, cm_ds_ofdm_rx_mer::CmDsOfdmRxMer,
    parse_limits::ParseLimits,
};
use sha2::{Digest, Sha256};
use std::sync::Arc;

pub(crate) const MAX_INPUT_BYTES: usize = 16 * 1024 * 1024;
pub(crate) const MAX_TOTAL_BYTES: usize = 64 * 1024 * 1024;

#[derive(Clone)]
pub(crate) enum Measurement {
    Rxmer(Arc<RxMerFrame>),
    Response(Arc<ChannelFrame>),
}
#[derive(Clone)]
pub(crate) struct Capture {
    pub name: String,
    pub original: Arc<Vec<u8>>,
    pub format: &'static str,
    pub hash: String,
    pub decoded_hash: String,
    pub measurement: Measurement,
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
        let (format,decoded)=match detect(&bytes) {
            InputFormat::BinaryPnm=>{
                let limits = ParseLimits { max_samples: 65_536 };
                let document=if rxmer {
                    CmDsOfdmRxMer::parse(&bytes,limits).map_err(|error|error.to_string())?.to_json(MAX_INPUT_BYTES)
                } else {
                    CmDsOfdmChanEstimateCoef::parse(&bytes,limits).map_err(|error|error.to_string())?.to_json(MAX_INPUT_BYTES)
                }.map_err(|error|error.to_string())?;
                ("Binary PNM",document.into_bytes())
            },
            InputFormat::JsonText=>("Decoded JSON",json_bytes(&bytes).to_vec()),
            _=>return Err("Choose binary PNM or supported decoded measurement JSON; this file has neither format.".into()),
        };
        let source = JsonCapture {
            name: &name,
            bytes: &decoded,
        };
        let hash = format!("{:x}", Sha256::digest(&bytes));
        let measurement = if rxmer {
            let mut frame = import_rxmer(source).map_err(|error| error.to_string())?;
            frame.source.sha256 = hash.clone();
            frame.source.byte_length = bytes.len();
            frame.source.file_kind = if format == "Binary PNM" {
                "binary_pnm_rxmer"
            } else {
                "decoded_json_rxmer"
            }
            .into();
            Measurement::Rxmer(Arc::new(frame))
        } else {
            let mut frame = import_channel(source).map_err(|error| error.to_string())?;
            frame.source.sha256 = hash.clone();
            frame.source.byte_length = bytes.len();
            frame.source.file_kind = if format == "Binary PNM" {
                "binary_pnm_channel_response"
            } else {
                "decoded_json_channel_response"
            }
            .into();
            Measurement::Response(Arc::new(frame))
        };
        Ok(Self {
            name,
            original: Arc::new(bytes),
            format,
            hash,
            decoded_hash: format!("{:x}", Sha256::digest(&decoded)),
            measurement,
        })
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
