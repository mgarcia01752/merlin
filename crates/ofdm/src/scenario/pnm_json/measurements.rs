/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Standalone decoded measurement imports using the existing shared contracts.
use super::{
    decoded::{Channel, Rxmer},
    parse, provenance, JsonCapture,
};
use crate::{
    error::invalid,
    scenario::{ChannelFrame, RxMerFrame},
    SimulationError,
};

/// Imports one standalone decoded RxMER JSON without another capture or resampling.
/// # Arguments
/// * `source` - Bounded decoder JSON and display identity; bytes stay unchanged.
/// # Returns
/// Validated original samples, identity and JSON provenance; nulls remain null.
/// # Errors
/// Rejects unsupported type/version, malformed units, length/grid or finite values.
pub fn import_rxmer(source: JsonCapture<'_>) -> Result<RxMerFrame, SimulationError> {
    let decoded: Rxmer = parse(&source)?;
    decoded.header.check(4)?;
    if decoded.value_units != "dB" || decoded.data_length != decoded.values.len() {
        return Err(invalid("RxMER units or data length are inconsistent."));
    }
    let frame = RxMerFrame {
        key: decoded.grid.key(decoded.values.len())?,
        source: provenance(&source, &decoded.header, "decoded_json_rxmer"),
        capture_time: decoded.header.capture_time,
        mer_db: decoded.values,
    };
    frame.validate()?;
    Ok(frame)
}

/// Imports one standalone decoded channel-response H JSON, not reciprocal gain.
/// # Arguments
/// * `source` - Bounded original decoder JSON and name.
/// # Returns
/// Exact response/grid/provenance with unavailable values preserved.
/// # Errors
/// Rejects unsupported type/version, malformed length/grid or nonfinite IQ.
/// Zero/missing active gains are checked later against the selected layout.
pub fn import_channel(source: JsonCapture<'_>) -> Result<ChannelFrame, SimulationError> {
    let decoded: Channel = parse(&source)?;
    decoded.header.check(2)?;
    if decoded.iq.len().checked_mul(4) != Some(decoded.data_length)
        || decoded
            .iq
            .iter()
            .flatten()
            .flatten()
            .any(|value| !value.is_finite())
    {
        return Err(invalid(
            "Channel response length or IQ values are inconsistent.",
        ));
    }
    let frame = ChannelFrame {
        key: decoded.grid.key(decoded.iq.len())?,
        source: provenance(&source, &decoded.header, "decoded_json_channel_response"),
        capture_time: decoded.header.capture_time,
        iq: decoded.iq,
    };
    frame.source.validate()?;
    Ok(frame)
}
