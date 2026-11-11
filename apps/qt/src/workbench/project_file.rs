/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Self-contained bounded project format; all reads/validation stay in Rust.
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, io::Write, path::Path};
pub(crate) const MAX_PROJECT_BYTES: usize = 144 * 1024 * 1024;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Project {
    pub schema_version: u32,
    pub application: String,
    pub transmission: docsis_ofdm_fec::transmission::TransmissionSettings,
    pub sections: BTreeMap<String, Value>,
    pub receiver: Option<crate::simulation::closed_loop::Options>,
    pub comparison_enabled: bool,
    pub dark: bool,
}
/// Encodes original file bytes and their digest without storing a path dependency.
/// # Arguments
/// * `name` - Display basename.
/// * `bytes` - Exact accepted original input bytes.
/// # Returns
/// Base64 document with original byte count and SHA-256.
pub(crate) fn encode(name: &str, bytes: &[u8]) -> Value {
    json!({"name":name,"byte_length":bytes.len(),"sha256":format!("{:x}",Sha256::digest(bytes)),"base64":STANDARD.encode(bytes)})
}
/// Validates an embedded original before returning any decoded payload.
/// # Arguments
/// * `value` - One stored original object.
/// # Returns
/// Bounded display name and exact bytes, verified against count/hash.
/// # Errors
/// Rejects unknown fields, invalid base64, names, counts or digest mismatches.
pub(crate) fn decode(value: &Value) -> Result<(String, Vec<u8>), String> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Stored {
        name: String,
        byte_length: usize,
        sha256: String,
        base64: String,
    }
    let stored: Stored = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
    if stored.name.len() > 1024
        || stored.byte_length > 16 * 1024 * 1024
        || stored.base64.len() > 24 * 1024 * 1024
    {
        return Err("Embedded input exceeds limits.".into());
    }
    let bytes = STANDARD.decode(stored.base64).map_err(|e| e.to_string())?;
    if bytes.len() != stored.byte_length || format!("{:x}", Sha256::digest(&bytes)) != stored.sha256
    {
        return Err("Embedded input length or SHA-256 mismatch.".into());
    }
    Ok((stored.name, bytes))
}
impl Project {
    /// Validates every section into a fresh Rust snapshot before atomic adoption.
    /// # Returns
    /// Independent accepted source/profile/measurement inputs.
    /// # Errors
    /// Rejects version drift, unknown sections, malformed originals or invalid settings.
    pub(crate) fn validate(&self) -> Result<crate::workbench::session::Inputs, String> {
        if ![1, 2].contains(&self.schema_version)
            || self.application != "MERLIN"
            || self.sections.len() > 5
            || self.sections.keys().any(|k| {
                ![
                    "source",
                    "profile",
                    "measurements",
                    "channel_model",
                    "channel_power",
                ]
                .contains(&k.as_str())
            })
        {
            return Err("Unsupported workbench format/sections.".into());
        }
        self.transmission.validate().map_err(|e| e.to_string())?;
        let source = crate::inputs::source_input::restore_source(
            self.sections.get("source").unwrap_or(&Value::Null),
        )?;
        let profile = crate::inputs::profile_input::restore_profile(
            self.sections.get("profile").unwrap_or(&Value::Null),
            &self.transmission,
        )?;
        let measurements = crate::inputs::measurement_input::restore_measurements(
            self.sections.get("measurements").unwrap_or(&Value::Null),
            &self.transmission,
            profile.as_ref(),
        )?;
        let inputs = crate::workbench::session::Inputs {
            power: crate::inputs::channel_model::restore_power(
                self.sections.get("channel_power").unwrap_or(&Value::Null),
            )?,
            source,
            profiles: crate::inputs::profile_input::restore_profiles(
                self.sections.get("profile").unwrap_or(&Value::Null),
            )?,
            profile,
            playback: measurements.playback,
            response: measurements.response,
            additional_loss: crate::inputs::channel_model::restore(
                self.sections.get("channel_model").unwrap_or(&Value::Null),
            )?,
        };
        if let Some(options) = &self.receiver {
            if self.schema_version == 1 && options.selected_profile_ids.is_some() {
                return Err("Explicit profile selection requires workbench schema 2.".into());
            }
            options.validate(&inputs)?;
        }
        Ok(inputs)
    }
    /// Atomically writes one reviewed workbench destination using a private sibling file.
    /// # Arguments
    /// * `path` - Explicit user-selected project destination.
    /// # Returns
    /// Unit after sync and atomic rename; old contents survive serialization/write failure.
    /// # Errors
    /// Rejects invalid configuration before writing; also reports serialization,
    /// file-size, filesystem or nonregular destination errors.
    pub(crate) fn write(&self, path: &Path) -> Result<(), String> {
        self.validate()?;
        if crate::platform::input_file::is_original(path) {
            return Err("Project Save cannot overwrite an imported original input.".into());
        }
        if path.exists()
            && !std::fs::symlink_metadata(path)
                .map_err(|e| e.to_string())?
                .file_type()
                .is_file()
        {
            return Err("Choose a regular project destination.".into());
        }
        let bytes = serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?;
        if bytes.len() > MAX_PROJECT_BYTES {
            return Err("Workbench exceeds 144 MiB.".into());
        }
        let temporary = path.with_file_name(format!(
            ".merlin-save-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| e.to_string())?
                .as_nanos()
        ));
        let write = || -> Result<(), String> {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)
                .map_err(|e| e.to_string())?;
            file.write_all(&bytes).map_err(|e| e.to_string())?;
            file.sync_all().map_err(|e| e.to_string())?;
            std::fs::rename(&temporary, path).map_err(|e| e.to_string())?;
            Ok(())
        };
        let result = write();
        if result.is_err() {
            let _ = std::fs::remove_file(&temporary);
        }
        result
    }
}

/// Validates a package-check WB through the same import and request APIs as Open.
/// # Arguments
/// * `path` - Local JSON document; input is bounded to the normal project limit.
/// # Returns
/// Unit when all imported inputs and full simulation request are admissible.
/// # Errors
/// Reports nonregular/oversized files, invalid JSON or invalid configuration.
pub(crate) fn check_file(path: &Path) -> Result<(), String> {
    use std::io::Read;
    let file = std::fs::File::open(path).map_err(|error| error.to_string())?;
    if !file
        .metadata()
        .map_err(|error| error.to_string())?
        .is_file()
    {
        return Err("Choose a regular workbench file.".into());
    }
    let mut bytes = Vec::new();
    file.take(MAX_PROJECT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() > MAX_PROJECT_BYTES {
        return Err("Workbench exceeds input limits.".into());
    }
    let project: Project = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    let inputs = project.validate()?;
    let options = project
        .receiver
        .as_ref()
        .ok_or("Example has no receiver configuration.")?;
    let request = crate::simulation::closed_loop::prepare(&project.transmission, &inputs, options)
        .map_err(|error| error.to_string())?;
    request.validate().map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version_two_profile_selection_roundtrips_and_legacy_remains_explicit() {
        let original = std::fs::read(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../examples/workbench/single/workbench.merlin.json"),
        )
        .unwrap();
        let mut project: Project = serde_json::from_slice(&original).unwrap();
        project.validate().unwrap();
        project.schema_version = 2;
        project.receiver.as_mut().unwrap().selected_profile_ids = Some(vec![1, 3]);
        let restored: Project =
            serde_json::from_slice(&serde_json::to_vec(&project).unwrap()).unwrap();
        restored.validate().unwrap();
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/development/local/workbench-configuration");
        std::fs::create_dir_all(&root).unwrap();
        let destination = root.join(format!(
            "selection-roundtrip-{}.merlin.json",
            std::process::id()
        ));
        project.write(&destination).unwrap();
        let saved = std::fs::read(&destination).unwrap();
        let loaded: Project = serde_json::from_slice(&saved).unwrap();
        assert_eq!(
            loaded.receiver.as_ref().unwrap().selected_profile_ids,
            Some(vec![1, 3])
        );
        project.receiver.as_mut().unwrap().selected_profile_ids = Some(vec![15]);
        assert!(project.write(&destination).is_err());
        assert_eq!(std::fs::read(&destination).unwrap(), saved);
        std::fs::remove_file(&destination).unwrap();
        project.receiver.as_mut().unwrap().selected_profile_ids = Some(vec![1, 3]);
        let plan = crate::simulation::closed_loop::prepare_sweep(
            &restored.transmission,
            &restored.validate().unwrap(),
            restored.receiver.as_ref().unwrap(),
        )
        .unwrap();
        let _ = plan;
        project.schema_version = 1;
        assert!(project.validate().is_err());
        project.schema_version = 99;
        assert!(project.validate().is_err());
        project.schema_version = 2;
        project.receiver.as_mut().unwrap().selected_profile_ids = Some(vec![15]);
        assert!(project.validate().is_err());
        project.receiver.as_mut().unwrap().selected_profile_ids = Some(vec![]);
        project.validate().unwrap();
    }
    #[test]
    fn curated_workbenches_restore_matching_originals_and_prepare_full_requests() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/workbench");
        let manifest: Value =
            serde_json::from_slice(&std::fs::read(root.join("manifest.json")).unwrap()).unwrap();
        let bundles = manifest["bundles"].as_array().unwrap();
        assert_eq!(bundles.len(), 3);
        assert_eq!(manifest["planned"][0]["id"], "multi-rxmer-dschan");
        for bundle in bundles {
            let path = root.join(bundle["workbench"].as_str().unwrap());
            let project: Project = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
            let inputs = project.validate().unwrap();
            let profile = inputs.profile.as_ref().unwrap();
            let playback = inputs.playback.as_ref().unwrap();
            if let Some(response) = inputs.response.as_ref() {
                assert_eq!(profile.key, response.key);
            } else {
                assert_eq!(bundle["id"], "multi-rxmer-2");
                assert!(bundle["channel_estimate"].is_null());
            }
            assert!(playback.frames.iter().all(|frame| frame.key == profile.key));
            assert_eq!(
                profile.key.grid.sample_count,
                if bundle["id"] == "multi-rxmer-2" {
                    7600
                } else {
                    7480
                }
            );
            assert_eq!(profile.key.grid.spacing_hz, 25000);
            let prepared = crate::simulation::closed_loop::prepare(
                &project.transmission,
                &inputs,
                project.receiver.as_ref().unwrap(),
            )
            .unwrap();
            prepared.validate().unwrap();
            let mut with_loss: Project =
                serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
            let loss=docsis_ofdm_fec::signal_processing::channel::additional_loss::AdditionalLoss::Manual {tilt_db:-6.0};
            with_loss.sections.insert(
                "channel_model".into(),
                crate::inputs::channel_model::saved(&loss),
            );
            assert_eq!(inputs.power, Default::default());
            let power = docsis_ofdm_fec::signal_processing::channel::power::PowerConditioning {
                amplifier_gain_db: 30.0,
                attenuation_db: 6.0,
                input_data_level_dbmv_per_6mhz: Some(0.0),
                receiver_noise_dbmv_hz: Some(-120.0),
                ..Default::default()
            };
            with_loss.sections.insert(
                "channel_power".into(),
                crate::inputs::channel_model::saved_power(&power),
            );
            let roundtrip: Project =
                serde_json::from_slice(&serde_json::to_vec(&with_loss).unwrap()).unwrap();
            let restored = roundtrip.validate().unwrap();
            assert_eq!(restored.additional_loss, loss);
            assert_eq!(restored.power, power);
            let changed = crate::simulation::closed_loop::prepare(
                &roundtrip.transmission,
                &restored,
                roundtrip.receiver.as_ref().unwrap(),
            )
            .unwrap();
            assert_eq!(changed.impairments.additional_loss, loss);
            assert_eq!(changed.impairments.power, power);
            let mut invalid = serde_json::to_value(&roundtrip).unwrap();
            invalid["sections"]["channel_power"]["schema_version"] = serde_json::json!(99);
            assert!(serde_json::from_value::<Project>(invalid)
                .unwrap()
                .validate()
                .is_err());
            assert_eq!(
                roundtrip.sections["measurements"],
                project.sections["measurements"]
            );

            let measurements = &project.sections["measurements"];
            assert_eq!(measurements["options"]["mode"], bundle["mode"]);
            let captures = measurements["rxmer"].as_array().unwrap();
            let paths = bundle["rxmer"].as_array().unwrap();
            assert_eq!(captures.len(), paths.len());
            let expected_count = match bundle["id"].as_str().unwrap() {
                "single" => 1,
                "multi-rxmer" => 5,
                "multi-rxmer-2" => 132,
                other => panic!("Unknown example {other}"),
            };
            assert_eq!(captures.len(), expected_count);
            let mut originals = vec![(
                &project.sections["profile"]["file"],
                &bundle["modulation_profile"],
            )];
            if !measurements["response"].is_null() {
                originals.push((&measurements["response"], &bundle["channel_estimate"]));
            }
            originals.extend(captures.iter().zip(paths));
            for (stored, relative) in originals {
                let (name, bytes) = decode(stored).unwrap();
                let original = root.join(relative.as_str().unwrap());
                assert_eq!(original.file_name().unwrap().to_str().unwrap(), name);
                assert_eq!(std::fs::read(original).unwrap(), bytes);
            }
            let provenance: Value = serde_json::from_slice(
                &std::fs::read(root.join(bundle["provenance"].as_str().unwrap())).unwrap(),
            )
            .unwrap();
            for entry in provenance["files"].as_array().unwrap() {
                let bytes =
                    std::fs::read(path.parent().unwrap().join(entry["file"].as_str().unwrap()))
                        .unwrap();
                assert_eq!(entry["bytes"], bytes.len());
                assert_eq!(entry["sha256"], format!("{:x}", Sha256::digest(&bytes)));
                if bundle["id"] == "multi-rxmer-2" {
                    assert_eq!(&bytes[11..17], &[0, 202, 0, 0, 0, 1]);
                    let mut upstream = bytes;
                    let address = entry["upstream_mac_address"].as_str().unwrap();
                    for (index, octet) in address.split(':').enumerate() {
                        upstream[11 + index] = u8::from_str_radix(octet, 16).unwrap();
                    }
                    assert_eq!(
                        entry["upstream_sha256"],
                        format!("{:x}", Sha256::digest(upstream))
                    );
                }
            }
            assert!(path.parent().unwrap().join("LICENSE").is_file());
            assert!(path.parent().unwrap().join("NOTICE").is_file());
        }
        assert!(!root
            .join("multi-rxmer-dschan/workbench.merlin.json")
            .exists());
    }
    #[test]
    fn self_contained_project_roundtrip_preserves_originals_and_rejects_corruption() {
        let file = encode("message.txt", b"exact\0payload\n");
        let profile = encode(
            "modulation_profile.bin",
            include_bytes!(
                "../../../../examples/measurements/single-channel/modulation_profile.bin"
            ),
        );
        let rxmer = encode(
            "rxmer.bin",
            include_bytes!("../../../../examples/measurements/rxmer-fec-stress/rxmer-baseline.bin"),
        );
        let sections = BTreeMap::from([
            (
                "source".into(),
                json!({"options":{"mode":"file","seed":32767,"append":false,"target_bits":0,"fill_seed":32767},"file":file}),
            ),
            ("profile".into(), json!({"file":profile,"profile_id":0})),
            (
                "measurements".into(),
                json!({"rxmer":[rxmer],"response":null,"options":{"mode":"single","interval_symbols":"1000","tolerance_seconds":90000},"response_tolerance":90000}),
            ),
        ]);
        let project = Project {
            schema_version: 1,
            application: "MERLIN".into(),
            transmission: Default::default(),
            sections,
            receiver: None,
            comparison_enabled: true,
            dark: true,
        };
        let bytes = serde_json::to_vec(&project).unwrap();
        let restored: Project = serde_json::from_slice(&bytes).unwrap();
        let inputs = restored.validate().unwrap();
        let docsis_ofdm_fec::simulation::source::PayloadSourceConfig::File { payload } =
            inputs.source.unwrap().config
        else {
            panic!("Expected file source")
        };
        assert_eq!(payload.bytes(), b"exact\0payload\n");
        assert!(inputs.profile.is_some());
        assert_eq!(inputs.playback.unwrap().frames.len(), 1);
        let mut corrupt: Project = serde_json::from_slice(&bytes).unwrap();
        corrupt.sections.get_mut("source").unwrap()["file"]["sha256"] = json!("0".repeat(64));
        assert!(corrupt.validate().is_err());
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/development/local/qt-closed-loop");
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join(format!("roundtrip-{}.merlin.json", std::process::id()));
        project.write(&path).unwrap();
        project.write(&path).unwrap();
        let loaded: Project = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        loaded.validate().unwrap();
        std::fs::remove_file(path).unwrap();
        assert!(project.write(&root).is_err());
    }
}
