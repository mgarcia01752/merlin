/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Current pnm-decode JSON adapter, independent of paths and decoder subprocesses.
pub mod contract;
mod measurements;
pub mod profile_metadata;
mod profiles;
pub use measurements::{import_channel, import_rxmer};
pub use profiles::{import_profile_capture, import_profiles};
mod decoded;
use crate::{
    error::invalid,
    scenario::{document::ScenarioDocument, ChannelFrame, RxMerFrame, SourceProvenance},
    types::ProtocolScope,
    SimulationError,
};
pub use contract::DecoderContract;
use decoded::{Channel, Rxmer};
use sha2::{Digest, Sha256};

/// Borrowed decoded JSON bytes and a caller-supplied display identity.
/// Hashes identify these JSON bytes, not unavailable original binary capture bytes.
pub struct JsonCapture<'input> {
    pub name: &'input str,
    pub bytes: &'input [u8],
}

/// Imports captures after admitting an explicit decoder compatibility declaration.
/// # Arguments
/// * `contract` - Export-record producer identity; never inferred from PNM headers.
/// * `rxmer` - Named decoded RxMER JSON bytes.
/// * `channel` - Named decoded channel-estimation JSON bytes.
/// * `profiles` - Named decoded modulation-profile JSON bytes.
/// * `protocol` - Explicit DOCSIS scope.
/// * `tolerance_seconds` - Maximum capture epoch distance from RxMER.
/// # Returns
/// Owned scenario with the same exact-byte hashes and grids as the legacy adapter.
/// Preserve the request alongside the scenario to retain producer declarations.
/// # Errors
/// Rejects unsupported contracts before parsing, then applies all import validation.
pub fn import_with_contract(
    contract: &DecoderContract,
    rxmer: JsonCapture<'_>,
    channel: JsonCapture<'_>,
    profiles: JsonCapture<'_>,
    protocol: ProtocolScope,
    tolerance_seconds: u32,
) -> Result<ScenarioDocument, SimulationError> {
    contract.validate()?;
    import(rxmer, channel, profiles, protocol, tolerance_seconds)
}

/// Imports legacy unversioned decoder documents without resampling.
/// Prefer import_with_contract for newly recorded producer exports.
///
/// # Arguments
/// * `rxmer` - Decoded RxMER JSON, at most16MiB.
/// * `channel` - Decoded DS channel response JSON, at most16MiB.
/// * `profiles` - Decoded modulation-profile JSON, at most16MiB.
/// * `protocol` - Explicit DOCSIS scope; never inferred from file-format versions.
/// * `tolerance_seconds` - Maximum distance from RxMER epoch for each other source.
///
/// # Returns
/// Owned scenario retaining source grids, nulls, epochs, profile IDs and JSON hashes.
/// Caller inputs remain unchanged; no files or services are accessed.
///
/// # Errors
/// Rejects wrong types/units, malformed or oversized arrays, incompatible identities,
/// grids, profile encodings and captures outside the explicit timestamp tolerance.
pub fn import(
    rxmer: JsonCapture<'_>,
    channel: JsonCapture<'_>,
    profiles: JsonCapture<'_>,
    protocol: ProtocolScope,
    tolerance_seconds: u32,
) -> Result<ScenarioDocument, SimulationError> {
    let readings: Rxmer = parse(&rxmer)?;
    let response: Channel = parse(&channel)?;
    readings.header.check(4)?;
    response.header.check(2)?;
    let key = readings.grid.key(readings.values.len())?;
    if readings.value_units != "dB"
        || readings.data_length != readings.values.len()
        || response.data_length
            != response
                .iq
                .len()
                .checked_mul(4)
                .ok_or_else(|| invalid("IQ length overflow."))?
        || response.grid.key(response.iq.len())? != key
    {
        return Err(invalid(
            "Decoded measurements have inconsistent units, identity or lengths.",
        ));
    }
    let decoded_profiles = import_profiles(profiles)?;
    if decoded_profiles.iter().any(|profile| profile.key != key) {
        return Err(invalid(
            "Profile frequency grid or coverage differs from RxMER.",
        ));
    }
    for epoch in std::iter::once(response.header.capture_time)
        .chain(decoded_profiles.iter().map(|profile| profile.capture_time))
    {
        if epoch.abs_diff(readings.header.capture_time) > tolerance_seconds {
            return Err(invalid(
                "PNM capture epochs exceed the explicit association tolerance.",
            ));
        }
    }
    let scenario = ScenarioDocument {
        schema_version: 1,
        protocol,
        frames: vec![RxMerFrame {
            key: key.clone(),
            capture_time: readings.header.capture_time,
            source: provenance(&rxmer, &readings.header, "decoded_json_rxmer"),
            mer_db: readings.values,
        }],
        channels: vec![ChannelFrame {
            key,
            capture_time: response.header.capture_time,
            source: provenance(&channel, &response.header, "decoded_json_channel_response"),
            iq: response.iq,
        }],
        profiles: decoded_profiles,
    };
    scenario.validate()?;
    Ok(scenario)
}

fn parse<Model: serde::de::DeserializeOwned>(
    source: &JsonCapture<'_>,
) -> Result<Model, SimulationError> {
    if source.bytes.len() > 16 * 1024 * 1024 {
        return Err(invalid("Decoded capture exceeds16MiB."));
    }
    serde_json::from_slice(source.bytes)
        .map_err(|error| invalid(format!("Invalid decoded capture JSON: {error}")))
}
fn provenance(source: &JsonCapture<'_>, header: &decoded::Header, kind: &str) -> SourceProvenance {
    SourceProvenance {
        name: source.name.into(),
        sha256: format!("{:x}", Sha256::digest(source.bytes)),
        byte_length: source.bytes.len(),
        capture_time: Some(header.capture_time),
        file_kind: kind.into(),
        format_major: header.major_version,
        format_minor: header.minor_version,
    }
}
