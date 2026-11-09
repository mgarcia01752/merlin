/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Local-path adaptation only; the shared Rust importer owns scientific validation.
use crate::{
    arguments::Arguments,
    json_io::{read_bytes, read_json, write_json},
};
use docsis_ofdm_fec::{
    scenario::pnm_json::{import, import_with_contract, DecoderContract, JsonCapture},
    types::ProtocolScope,
};
use serde::Deserialize;
use std::path::PathBuf;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    schema_version: u16,
    protocol: ProtocolScope,
    decoder_contract: Option<DecoderContract>,
    rxmer: PathBuf,
    channel_estimation: PathBuf,
    modulation_profile: PathBuf,
    tolerance_seconds: u32,
}
/// Imports current decoded captures into the standalone scenario schema.
/// # Arguments
/// * `arguments` - Request/result paths; capture paths resolve relative to the request.
/// # Returns
/// Unit after non-overwriting scenario export.
/// # Errors
/// Rejects unrelated options, schemas, I/O, bounds or scientific incompatibilities.
pub(super) fn run(arguments: Arguments) -> Result<(), String> {
    if arguments.scenario.is_some()
        || arguments.candidate.is_some()
        || arguments.settings.is_some()
        || arguments.profile.is_some()
        || arguments.frame_supplied
        || arguments.probes_output.is_some()
    {
        return Err("import-pnm accepts --input and --output only.".into());
    }
    let request: Request = read_json(&arguments.input)?;
    if !matches!(
        (request.schema_version, &request.decoder_contract),
        (1, None) | (2, Some(_))
    ) {
        return Err("Unsupported import request version.".into());
    }
    if let Some(contract) = &request.decoder_contract {
        contract.validate().map_err(|error| error.to_string())?;
    }
    let directory = arguments
        .input
        .parent()
        .ok_or("Missing request directory.")?;
    let paths = [
        request.rxmer,
        request.channel_estimation,
        request.modulation_profile,
    ]
    .map(|path| directory.join(path));
    let bytes = paths
        .iter()
        .map(|path| read_bytes(path))
        .collect::<Result<Vec<_>, _>>()?;
    let names = paths
        .iter()
        .map(|path| {
            path.file_name()
                .ok_or("Missing capture filename.")
                .map(|name| name.to_string_lossy().into_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let import_capture =
        |rxmer, channel, profiles, protocol, tolerance| match &request.decoder_contract {
            Some(contract) => {
                import_with_contract(contract, rxmer, channel, profiles, protocol, tolerance)
            }
            None => import(rxmer, channel, profiles, protocol, tolerance),
        };
    let scenario = import_capture(
        JsonCapture {
            name: &names[0],
            bytes: &bytes[0],
        },
        JsonCapture {
            name: &names[1],
            bytes: &bytes[1],
        },
        JsonCapture {
            name: &names[2],
            bytes: &bytes[2],
        },
        request.protocol,
        request.tolerance_seconds,
    )
    .map_err(|error| error.to_string())?;
    write_json(&scenario, &arguments.output)?;
    log::info!(
        "pnm-import-complete rxmer-frames={} profiles={} channel-frames={}",
        scenario.frames.len(),
        scenario.profiles.len(),
        scenario.channels.len()
    );
    Ok(())
}
