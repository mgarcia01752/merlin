/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared validated profile parsing, with opt-in compact inspection metadata.
use super::{
    decoded::ProfileDocument,
    parse,
    profile_metadata::{ProfileCaptureHeader, ProfileCaptureMetadata, ProfileSchemeInfo},
    provenance, JsonCapture,
};
use crate::{
    error::invalid,
    profile::candidate::{AssignmentOrigin, CapturedProfile, CarrierAssignment},
    types::modulation::ModulationAssignment,
    SimulationError,
};

/// Imports a standalone decoded modulation-profile capture through the shared contract.
///
/// # Arguments
/// * `source` - Named decoder JSON bytes, at most 16 MiB; originals stay unchanged.
/// # Returns
/// Bounded profiles retaining exact device/channel/grid identity and JSON provenance.
/// # Errors
/// Rejects unsupported format/schemes, duplicate IDs, invalid grids or contradictory
/// assignment codes/names/roles. Other captures are not needed for inspection.
pub fn import_profiles(source: JsonCapture<'_>) -> Result<Vec<CapturedProfile>, SimulationError> {
    decode_profiles(source, false).map(|(profiles, _)| profiles)
}

/// Imports the same validated profiles while retaining their compact source metadata.
/// # Arguments
/// * `source` - Bounded decoded profile JSON and original intermediate identity.
/// # Returns
/// Existing scientific profiles and exact source header/schemes, without simulation.
/// # Errors
/// Preserves the existing importer validation order, supported formats and work limits.
pub fn import_profile_capture(
    source: JsonCapture<'_>,
) -> Result<(Vec<CapturedProfile>, ProfileCaptureMetadata), SimulationError> {
    let (profiles, metadata) = decode_profiles(source, true)?;
    Ok((
        profiles,
        metadata.ok_or_else(|| invalid("Profile inspection metadata was not retained"))?,
    ))
}

fn decode_profiles(
    source: JsonCapture<'_>,
    retain_metadata: bool,
) -> Result<(Vec<CapturedProfile>, Option<ProfileCaptureMetadata>), SimulationError> {
    let captured: ProfileDocument = parse(&source)?;
    captured.header.check(10)?;
    if captured.num_profiles != captured.profiles.len()
        || captured.profiles.is_empty()
        || captured.profiles.len() > 256
    {
        return Err(invalid("Invalid modulation-profile count."));
    }
    let count = captured.profiles[0]
        .schemes
        .iter()
        .try_fold(0usize, |total, scheme| {
            total
                .checked_add(scheme.num_subcarriers)
                .ok_or_else(|| invalid("Profile size overflow."))
        })?;
    let key = captured.grid.key(count)?;
    let mut metadata = retain_metadata.then(|| ProfileCaptureMetadata {
        header: ProfileCaptureHeader {
            file_type: captured.header.file_type.clone(),
            file_type_version: captured.header.file_type_version,
            major_version: captured.header.major_version,
            minor_version: captured.header.minor_version,
            capture_time: captured.header.capture_time,
        },
        key: key.clone(),
        schemes: std::collections::BTreeMap::new(),
    });
    let mut decoded_profiles = Vec::new();
    let mut expanded_total = key.grid.sample_count * 2;
    let mut seen = std::collections::BTreeSet::new();
    for profile in captured.profiles {
        if !seen.insert(profile.profile_id) || profile.schemes.len() > key.grid.sample_count {
            return Err(invalid("Duplicate profile ID or excessive scheme count."));
        }
        let mut assignments = Vec::new();
        let mut source_schemes = Vec::new();
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
            if retain_metadata {
                source_schemes.push(ProfileSchemeInfo::new(
                    source_schemes.len(),
                    assignments.len(),
                    scheme.num_subcarriers,
                    scheme.modulation_order_code,
                    &key.grid,
                )?);
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
        if let Some(metadata) = &mut metadata {
            metadata.schemes.insert(profile.profile_id, source_schemes);
        }
        decoded_profiles.push(CapturedProfile {
            key: key.clone(),
            source: provenance(&source, &captured.header, "decoded_json_modulation_profile"),
            capture_time: captured.header.capture_time,
            profile_id: profile.profile_id,
            assignments,
        });
    }
    Ok((decoded_profiles, metadata))
}
