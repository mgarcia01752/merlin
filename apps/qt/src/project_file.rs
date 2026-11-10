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
    pub receiver: Option<crate::closed_loop::model::Options>,
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
    pub(crate) fn validate(&self) -> Result<crate::workbench_session::Inputs, String> {
        if self.schema_version != 1
            || self.application != "MERLIN"
            || self.sections.len() > 3
            || self
                .sections
                .keys()
                .any(|k| !["source", "profile", "measurements"].contains(&k.as_str()))
        {
            return Err("Unsupported workbench format/sections.".into());
        }
        self.transmission.validate().map_err(|e| e.to_string())?;
        let source = crate::source_input::restore_source(
            self.sections.get("source").unwrap_or(&Value::Null),
        )?;
        let profile = crate::profile_input::restore_profile(
            self.sections.get("profile").unwrap_or(&Value::Null),
            &self.transmission,
        )?;
        let measurements = crate::measurement_input::restore_measurements(
            self.sections.get("measurements").unwrap_or(&Value::Null),
            &self.transmission,
            profile.as_ref(),
        )?;
        let inputs = crate::workbench_session::Inputs {
            source,
            profile,
            playback: measurements.playback,
            response: measurements.response,
        };
        if let Some(options) = &self.receiver {
            if !(1..=1000).contains(&options.duration_ms)
                || !(1..=100).contains(&options.max_iterations)
                || !(1..=300000).contains(&options.max_runtime_ms)
            {
                return Err("Invalid saved receiver/run limits".into());
            }
        }
        Ok(inputs)
    }
    /// Atomically writes one reviewed workbench destination using a private sibling file.
    /// # Arguments
    /// * `path` - Explicit user-selected project destination.
    /// # Returns
    /// Unit after sync and atomic rename; old contents survive serialization/write failure.
    /// # Errors
    /// Reports serialization, file-size, filesystem or nonregular destination errors.
    pub(crate) fn write(&self, path: &Path) -> Result<(), String> {
        if crate::input_file::is_original(path) {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn self_contained_project_roundtrip_preserves_originals_and_rejects_corruption() {
        let file = encode("message.txt", b"exact\0payload\n");
        let profile = encode(
            "modulation_profile.bin",
            include_bytes!("../../../examples/measurements/single-channel/modulation_profile.bin"),
        );
        let rxmer = encode(
            "rxmer.bin",
            include_bytes!("../../../examples/measurements/rxmer-fec-stress/rxmer-baseline.bin"),
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
