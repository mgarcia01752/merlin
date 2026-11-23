/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Translate accepted original editor sections into passive public API inputs.
use crate::downstream::workbench::project_file::decode;
use merlin::downstream::{
    inputs::CaptureLoader, FileFillPolicy, NamedCapture, PrbsSeed, ProfileCapture, SourceInput,
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceOptions {
    mode: String,
    seed: u16,
    append: bool,
    target_bits: u64,
    fill_seed: u16,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceSection {
    options: SourceOptions,
    file: Option<Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProfileSection {
    file: Value,
    profile_id: u8,
}

/// Passive API inputs with original bytes and original encoded profile identity.
pub(crate) struct TransmitterInputs {
    pub source: SourceInput,
    pub capture: ProfileCapture,
    pub profile_id: u8,
}

fn original(value: &Value) -> Result<NamedCapture, String> {
    let (name, bytes) = decode(value)?;
    NamedCapture::new(name, bytes).map_err(|cause| cause.message().to_owned())
}

/// Admits the accepted source and complete original profile capture through the public API.
///
/// # Arguments
/// * `source` - Accepted source section, including any dormant original file.
/// * `profile` - Accepted original profile section and byte-sized selected identity.
///
/// # Returns
/// Owned immutable API inputs, or None if either editor is unaccepted. No source,
/// waveform, RNG or receiver advances. File mode ignores the dormant continuous seed;
/// disabled fill leaves its seed and target dormant, preserving existing behavior.
///
/// # Errors
/// Rejects malformed sections, original integrity failures and public source/capture
/// admission errors. No accepted editor or current session is modified.
pub(crate) fn transmitter_inputs(
    source: &Value,
    profile: &Value,
) -> Result<Option<TransmitterInputs>, String> {
    if source.is_null() || profile.is_null() {
        return Ok(None);
    }
    let source: SourceSection =
        serde_json::from_value(source.clone()).map_err(|cause| cause.to_string())?;
    let profile: ProfileSection =
        serde_json::from_value(profile.clone()).map_err(|cause| cause.to_string())?;
    // Integrity-check dormant originals, as the existing project/source editor does.
    let file = source.file.as_ref().map(decode).transpose()?;
    let options = source.options;
    let admit_source = || match options.mode.as_str() {
        "prbs15" => Ok(SourceInput::prbs15(PrbsSeed::new(options.seed)?)),
        "file" => {
            let (name, bytes) = file.ok_or_else(|| {
                merlin::downstream::ApiError::new(
                    merlin::downstream::ErrorCode::MissingSetting,
                    "Select a payload file before Apply.",
                )
            })?;
            let fill = if options.append {
                Some(FileFillPolicy::new(
                    options.target_bits,
                    PrbsSeed::new(options.fill_seed)?,
                )?)
            } else {
                None
            };
            if name.is_empty() {
                SourceInput::unnamed_file(&bytes, fill)
            } else {
                SourceInput::file(NamedCapture::new(name, bytes)?, fill)
            }
        }
        _ => Err(merlin::downstream::ApiError::new(
            merlin::downstream::ErrorCode::UnsupportedCapability,
            "Choose Continuous PRBS15 or File.",
        )),
    };
    let source = admit_source().map_err(|cause| cause.message().to_owned())?;
    let capture = CaptureLoader::profile_capture(&original(&profile.file)?)
        .map_err(|cause| cause.message().to_owned())?;
    Ok(Some(TransmitterInputs {
        source,
        capture,
        profile_id: profile.profile_id,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::downstream::workbench::project_file::encode;
    use serde_json::json;

    #[test]
    fn originals_and_dormant_file_seed_fill_remain_distinct() {
        let profile = json!({"profile_id":0,"file":encode("original.bin",include_bytes!(
            "../../../../../examples/measurements/single-channel/modulation_profile.bin"))});
        // Zero seeds/target are intentionally dormant in file mode without append.
        let mut source = json!({"file":encode("payload.bin",b"\0exact\r\n"),"options":{
            "mode":"file","seed":0,"append":false,"target_bits":0,"fill_seed":0}});
        let admitted = transmitter_inputs(&source, &profile).unwrap().unwrap();
        assert_eq!(admitted.source.kind(), "file");
        assert_eq!(
            admitted.source.original_bytes(),
            Some(b"\0exact\r\n".as_slice())
        );
        assert!(admitted.source.prbs_seed().is_none());
        assert_eq!(admitted.source.target_bits(), Some(64)); // Eight original bytes, MSB-first bits.
        assert_eq!(
            admitted.capture.provenance().original_bytes(),
            include_bytes!(
                "../../../../../examples/measurements/single-channel/modulation_profile.bin"
            )
        );
        source["options"]["append"] = json!(true);
        source["options"]["target_bits"] = json!(80); // Independent finite fill target above 64 original bits.
        source["options"]["fill_seed"] = json!(1); // Independent valid fill seed, not the dormant continuous seed.
        assert_eq!(
            transmitter_inputs(&source, &profile)
                .unwrap()
                .unwrap()
                .source
                .target_bits(),
            Some(80)
        );
        // Preserve the legacy empty file-label case through its explicit public operation.
        source["file"]["name"] = json!("");
        let unnamed = transmitter_inputs(&source, &profile).unwrap().unwrap();
        assert_eq!(unnamed.source.name(), Some(""));
        assert_eq!(
            unnamed.source.original_bytes(),
            Some(b"\0exact\r\n".as_slice())
        );
        assert_eq!(unnamed.source.target_bits(), Some(80));
        source["file"]["sha256"] = json!("invalid");
        assert!(transmitter_inputs(&source, &profile).is_err());
        assert!(transmitter_inputs(&Value::Null, &profile)
            .unwrap()
            .is_none());
        assert!(transmitter_inputs(&source, &Value::Null).unwrap().is_none());
    }
}
