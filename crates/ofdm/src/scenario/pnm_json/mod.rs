/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Current pnm-decode JSON adapter, independent of paths and decoder subprocesses.
pub mod contract;
mod decoded;
use crate::{
    error::invalid,
    profile::candidate::{AssignmentOrigin, CapturedProfile, CarrierAssignment},
    scenario::{document::ScenarioDocument, ChannelFrame, RxMerFrame, SourceProvenance},
    types::{modulation::ModulationAssignment, ProtocolScope},
    SimulationError,
};
pub use contract::DecoderContract;
use decoded::{Channel, ProfileDocument, Rxmer};
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
    let captured: ProfileDocument = parse(&profiles)?;
    readings.header.check(4)?;
    response.header.check(2)?;
    captured.header.check(10)?;
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
        || captured.num_profiles != captured.profiles.len()
        || captured.profiles.is_empty()
        || captured.profiles.len() > 256
    {
        return Err(invalid(
            "Decoded measurements have inconsistent units, identity or lengths.",
        ));
    }
    for epoch in [response.header.capture_time, captured.header.capture_time] {
        if epoch.abs_diff(readings.header.capture_time) > tolerance_seconds {
            return Err(invalid(
                "PNM capture epochs exceed the explicit association tolerance.",
            ));
        }
    }
    let mut decoded_profiles = Vec::new();
    let mut expanded_total = key.grid.sample_count * 2;
    let mut seen = std::collections::BTreeSet::new();
    for profile in captured.profiles {
        if !seen.insert(profile.profile_id) || profile.schemes.len() > key.grid.sample_count {
            return Err(invalid("Duplicate profile ID or excessive scheme count."));
        }
        let mut assignments = Vec::new();
        for scheme in profile.schemes {
            expanded_total = expanded_total
                .checked_add(scheme.num_subcarriers)
                .ok_or_else(|| invalid("Expanded profile size overflow."))?;
            if expanded_total > 4_000_000 {
                return Err(invalid("Expanded scenario exceeds the carrier work limit."));
            }
            if scheme.schema_type != "range"
                || scheme.num_subcarriers == 0
                || assignments
                    .len()
                    .checked_add(scheme.num_subcarriers)
                    .is_none_or(|count| count > key.grid.sample_count)
            {
                return Err(invalid("Unsupported profile scheme or invalid coverage."));
            }
            let assignment = ModulationAssignment::from_code(scheme.modulation_order_code);
            if scheme.modulation_order != assignment.modulation_order
                || scheme.bit_loading != assignment.bit_loading
                || scheme.carrier_type != assignment.carrier_type
            {
                return Err(invalid(
                    "Profile text/bit-loading contradicts encoded assignment.",
                ));
            }
            assignments.extend(std::iter::repeat_n(
                CarrierAssignment {
                    carrier_type: assignment.carrier_type,
                    bit_loading: assignment.bit_loading,
                    origin: AssignmentOrigin::Captured,
                },
                scheme.num_subcarriers,
            ));
        }
        if captured.grid.key(assignments.len())? != key {
            return Err(invalid(
                "Profile frequency grid or coverage differs from RxMER.",
            ));
        }
        decoded_profiles.push(CapturedProfile {
            key: key.clone(),
            source: provenance(
                &profiles,
                &captured.header,
                "decoded_json_modulation_profile",
            ),
            capture_time: captured.header.capture_time,
            profile_id: profile.profile_id,
            assignments,
        });
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
