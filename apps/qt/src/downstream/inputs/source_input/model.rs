/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Rust-owned source draft, immutable input and accepted core configuration.
use crate::downstream::common::constants::{
    INITIAL_CONTINUOUS_SOURCE_SEED, INITIAL_FILE_FILL_SEED, MAX_SOURCE_OPTIONS_DOCUMENT_BYTES,
};
use merlin::downstream::{FileFillPolicy, NamedCapture, PrbsSeed, SourceFileInfo, SourceInput};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Options {
    mode: String,
    seed: u16,
    append: bool,
    target_bits: u64,
    fill_seed: u16,
}

impl Default for Options {
    /// Restores explicit editor choices with independent continuous and file-fill seeds.
    /// # Returns
    /// Legacy PRBS mode and disabled append with a dormant zero target; no source advances.
    fn default() -> Self {
        Self {
            mode: "prbs15".into(),
            seed: INITIAL_CONTINUOUS_SOURCE_SEED,
            append: false,
            target_bits: 0,
            fill_seed: INITIAL_FILE_FILL_SEED,
        }
    }
}

#[derive(Clone)]
struct SourceFile {
    payload: SourceInput,
}

struct Accepted {
    options: Options,
    file: Option<SourceFile>,
    config: SourceInput,
}

#[derive(Default)]
pub(crate) struct SourceModel {
    file: Option<SourceFile>,
    accepted: Option<Accepted>,
}

impl SourceModel {
    /// Serializes accepted source choices and original bytes for project persistence.
    /// # Returns
    /// A bounded Rust-owned document, or null before acceptance.
    pub(crate) fn export(&self) -> Value {
        self.accepted.as_ref().map(|a|json!({"options":a.options,"file":a.file.as_ref().map(|f|crate::downstream::workbench::project_file::encode(f.payload.name().expect("Finite source name"),f.payload.original_bytes().expect("Finite source bytes")))})).unwrap_or(Value::Null)
    }
    /// Reconstructs an editor from validated original project bytes without publishing it.
    /// # Arguments
    /// * `value` - Versioned project source section.
    /// # Returns
    /// A fresh accepted editor model, independent of current UI state.
    /// # Errors
    /// Rejects unknown choices, invalid embedded bytes or core source limits.
    pub(crate) fn from_saved(value: &Value) -> Result<Self, String> {
        if value.is_null() {
            return Ok(Self::default());
        }
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Saved {
            options: Options,
            file: Option<Value>,
        }
        let saved: Saved = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
        let mut model = Self::default();
        if let Some(file) = saved.file {
            let (name, bytes) = crate::downstream::workbench::project_file::decode(&file)?;
            model.stage(name, bytes)?;
        }
        model.accept(&serde_json::to_string(&saved.options).map_err(|e| e.to_string())?)?;
        Ok(model)
    }
    /// Borrows accepted source identity through an immutable typed snapshot.
    /// # Returns
    /// Shared file storage/configuration and explicit seed, or None before Apply.
    pub(crate) fn snapshot(&self) -> Option<crate::downstream::workbench::session::AcceptedSource> {
        self.accepted.as_ref().map(|accepted| {
            crate::downstream::workbench::session::AcceptedSource {
                config: accepted.config.clone(),
                seed: accepted.options.seed,
            }
        })
    }
    /// Restores accepted bytes/options for a new edit session.
    /// # Returns
    /// Initial options and bounded metadata; no source bits are generated.
    pub(crate) fn begin(&mut self) -> Value {
        self.file = self
            .accepted
            .as_ref()
            .and_then(|accepted| accepted.file.clone());
        let options = self
            .accepted
            .as_ref()
            .map(|accepted| accepted.options.clone())
            .unwrap_or_default();
        json!({"options":options, "file":self.file_summary()})
    }

    /// Stages exact original bytes without committing accepted source.
    /// # Arguments
    /// * `name` - Display basename only, never a scientific input path.
    /// * `bytes` - Original bounded bytes, unchanged by inspection.
    /// # Returns
    /// Success after input validation and original-byte hashing.
    /// # Errors
    /// Rejects core input bounds before replacing the previous draft.
    pub(crate) fn stage(&mut self, name: String, bytes: Vec<u8>) -> Result<(), String> {
        let payload = if name.is_empty() {
            SourceInput::unnamed_file(&bytes, None)
        } else {
            NamedCapture::new(name, bytes).and_then(|input| SourceInput::file(input, None))
        }
        .map_err(|error| error.message().to_owned())?;
        self.file = Some(SourceFile { payload });
        Ok(())
    }

    /// Queries bounded original-file metadata without exporting payload bytes.
    /// # Returns
    /// Selection, basename, size and original-byte SHA-256, or explicit absence.
    pub(crate) fn file_summary(&self) -> Value {
        match &self.file {
            Some(file) => {
                let info = file
                    .payload
                    .file_info()
                    .expect("SourceFile retains finite input");
                json!({"selected":true,"name":file.payload.name(),"bytes":info.original_bytes,"bits":info.target_bits,"sha256":file.payload.original_sha256()})
            }
            None => json!({"selected":false,"name":"","bytes":null,"bits":null,"sha256":""}),
        }
    }

    fn options(document: &str) -> Result<Options, String> {
        if document.len() > MAX_SOURCE_OPTIONS_DOCUMENT_BYTES {
            return Err("Source options exceed 2048 bytes.".into());
        }
        serde_json::from_str(document).map_err(|error| format!("Invalid source options: {error}"))
    }

    fn validate(&self, options: &Options) -> Result<(SourceInput, Option<SourceFileInfo>), String> {
        match options.mode.as_str() {
            "prbs15" => {
                let seed =
                    PrbsSeed::new(options.seed).map_err(|error| error.message().to_owned())?;
                Ok((SourceInput::prbs15(seed), None))
            }
            "file" => {
                let file = self
                    .file
                    .as_ref()
                    .ok_or("Select a payload file before Apply.")?;
                let fill = if options.append {
                    Some(
                        FileFillPolicy::new(
                            options.target_bits,
                            PrbsSeed::new(options.fill_seed)
                                .map_err(|error| error.message().to_owned())?,
                        )
                        .map_err(|error| error.message().to_owned())?,
                    )
                } else {
                    None
                };
                let source = file
                    .payload
                    .with_fill(fill)
                    .map_err(|error| error.message().to_owned())?;
                let info = source.file_info();
                Ok((source, info))
            }
            _ => Err("Choose Continuous PRBS15 or File.".into()),
        }
    }

    /// Validates draft options through the public API and reads its meaningful-bit counts.
    /// # Arguments
    /// * `document` - Source options JSON with no file contents.
    /// # Returns
    /// Original/target/fill counts or a diagnostic; no acceptance is changed.
    pub(crate) fn describe(&self, document: &str) -> Value {
        let result = Self::options(document).and_then(|options| self.validate(&options));
        match result {
            Ok((_, Some(info))) => {
                json!({"error":"","original_bits":info.original_bits,"target_bits":info.target_bits,"fill_bits":info.fill_bits})
            }
            Ok((_, None)) => {
                // Preserve the existing continuous-mode display convention, not finite EOF.
                json!({"error":"","original_bits":0,"target_bits":0,"fill_bits":0})
            }
            Err(error) => {
                json!({"error":error,"original_bits":null,"target_bits":null,"fill_bits":null})
            }
        }
    }

    /// Freezes validated source configuration for the next run.
    /// # Arguments
    /// * `document` - Options for PRBS15 or the staged exact-byte file.
    /// # Returns
    /// Success after atomically replacing the accepted snapshot.
    /// # Errors
    /// Invalid JSON, seeds, missing files or fill targets preserve acceptance.
    pub(crate) fn accept(&mut self, document: &str) -> Result<(), String> {
        let options = Self::options(document)?;
        let (config, _) = self.validate(&options)?;
        let file = if options.mode == "file" {
            self.file.clone()
        } else {
            None
        };
        self.accepted = Some(Accepted {
            options,
            file,
            config,
        });
        Ok(())
    }

    /// Reports explicit source acceptance rather than simulation completion.
    /// # Returns
    /// Block status and accessible readiness reason, without source bytes.
    pub(crate) fn state(&self) -> Value {
        match &self.accepted {
            None => {
                json!({"status":"attention","reason":"Apply a validated PRBS15 or file source before starting."})
            }
            Some(accepted) => {
                let reason = if let Some(info) = accepted.config.file_info() {
                    format!("Exact-byte file source accepted: {} bytes, {} meaningful bits for the next run.", info.original_bytes, info.target_bits)
                } else {
                    format!(
                        "Continuous PRBS15 accepted with seed {} for the next run.",
                        accepted.options.seed
                    )
                };
                json!({"status":"complete","reason":reason})
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    #[test]
    fn acceptance_rejection_cancel_and_modes_preserve_source_snapshot() {
        let mut model = SourceModel::default();
        let initial = model.begin();
        assert_eq!(model.state()["status"], "attention");
        let mut options = initial["options"].clone();
        options["seed"] = json!(0);
        assert!(model.accept(&options.to_string()).is_err());
        options["seed"] = json!(32767);
        model.accept(&options.to_string()).unwrap();
        let accepted = model.state();
        options["mode"] = json!("file");
        assert!(model.accept(&options.to_string()).is_err());
        assert_eq!(model.state(), accepted);
        model
            .stage("exact.txt".into(), vec![0xef, 0xbb, 0xbf, 0, 0xff, 13, 10])
            .unwrap();
        assert_eq!(model.file_summary()["bits"], 56);
        model.accept(&options.to_string()).unwrap();
        model.stage("replacement.bin".into(), vec![1, 2]).unwrap();
        let restored = model.begin();
        assert_eq!(restored["file"]["name"], "exact.txt");
        assert_eq!(
            model
                .file
                .as_ref()
                .unwrap()
                .payload
                .original_bytes()
                .unwrap(),
            &[0xef, 0xbb, 0xbf, 0, 0xff, 13, 10]
        );
        assert_eq!(
            model.file_summary()["sha256"],
            format!("{:x}", Sha256::digest([0xef, 0xbb, 0xbf, 0, 0xff, 13, 10]))
        );
    }

    #[test]
    fn finite_fill_bounds_empty_equal_and_partial_byte_targets() {
        let mut model = SourceModel::default();
        model.stage("one.bin".into(), vec![0x80]).unwrap();
        let mut options =
            json!({"mode":"file","seed":32767,"append":true,"target_bits":11,"fill_seed":1});
        assert_eq!(model.describe(&options.to_string())["fill_bits"], 3);
        model.accept(&options.to_string()).unwrap();
        for target in [0, 7, SourceInput::file_limits().max_total_bits + 1] {
            options["target_bits"] = json!(target);
            assert!(model.accept(&options.to_string()).is_err());
        }
        options["target_bits"] = json!(8);
        model.accept(&options.to_string()).unwrap();
        assert_eq!(model.describe(&options.to_string())["fill_bits"], 0);
        options["fill_seed"] = json!(0);
        assert!(model.accept(&options.to_string()).is_err());
        options["append"] = json!(false);
        model.accept(&options.to_string()).unwrap();
        model.stage("empty.bin".into(), vec![]).unwrap();
        model.accept(&options.to_string()).unwrap();
        options["append"] = json!(true);
        options["fill_seed"] = json!(32767);
        options["target_bits"] = json!(3);
        model.accept(&options.to_string()).unwrap();
        assert_eq!(model.describe(&options.to_string())["fill_bits"], 3);
        assert!(model.accept(&" ".repeat(2049)).is_err());
    }

    #[test]
    fn public_source_snapshot_shares_bytes_and_preserves_exact_saved_section() {
        let mut model = SourceModel::default();

        // Same independent three-octet/64-bit/seed-seven saved-source compatibility fixture.
        let bytes = [0x81, 0x00, 0x5a];
        model.stage(String::new(), bytes.to_vec()).unwrap();
        let options = json!({"mode":"file","seed":0,"append":true,"target_bits":64,"fill_seed":7});
        assert_eq!(model.describe(&options.to_string())["original_bits"], 24);
        assert_eq!(model.describe(&options.to_string())["fill_bits"], 40);
        model.accept(&options.to_string()).unwrap();
        let snapshot = model.snapshot().unwrap();
        assert_eq!(
            snapshot.config.original_bytes().unwrap().as_ptr(),
            model
                .file
                .as_ref()
                .unwrap()
                .payload
                .original_bytes()
                .unwrap()
                .as_ptr()
        );

        // Literal base64 gQBa encodes the exact original octets; appended bits are excluded.
        let expected = json!({"options":options,"file":{"name":"","byte_length":3,
            "base64":"gQBa","sha256":format!("{:x}",Sha256::digest(bytes))}});
        assert_eq!(model.export(), expected);
        let restored = SourceModel::from_saved(&expected).unwrap();
        assert_eq!(restored.export(), expected);
        assert_eq!(
            restored.snapshot().unwrap().config.original_bytes(),
            Some(bytes.as_slice())
        );
        assert_eq!(restored.snapshot().unwrap().config.target_bits(), Some(64));
        assert_eq!(restored.snapshot().unwrap().seed, 0);
    }
}
