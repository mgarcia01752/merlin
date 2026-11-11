/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Rust-owned source draft, immutable input and accepted core configuration.
use docsis_ofdm_fec::simulation::source::{
    file::{FilePayload, PrbsFill},
    PayloadSourceConfig, Prbs15,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

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
    fn default() -> Self {
        Self {
            mode: "prbs15".into(),
            seed: 32767,
            append: false,
            target_bits: 0,
            fill_seed: 32767,
        }
    }
}

#[derive(Clone)]
struct SourceFile {
    name: String,
    sha256: String,
    payload: FilePayload,
}

struct Accepted {
    options: Options,
    file: Option<SourceFile>,
    config: PayloadSourceConfig,
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
        self.accepted.as_ref().map(|a|json!({"options":a.options,"file":a.file.as_ref().map(|f|crate::workbench::project_file::encode(&f.name,f.payload.bytes()))})).unwrap_or(Value::Null)
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
            let (name, bytes) = crate::workbench::project_file::decode(&file)?;
            model.stage(name, bytes)?;
        }
        model.accept(&serde_json::to_string(&saved.options).map_err(|e| e.to_string())?)?;
        Ok(model)
    }
    /// Borrows accepted source identity through an immutable typed snapshot.
    /// # Returns
    /// Shared file storage/configuration and explicit seed, or None before Apply.
    pub(crate) fn snapshot(&self) -> Option<crate::workbench::session::AcceptedSource> {
        self.accepted
            .as_ref()
            .map(|accepted| crate::workbench::session::AcceptedSource {
                config: accepted.config.clone(),
                seed: accepted.options.seed,
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
        let payload = FilePayload::new(&bytes, None).map_err(|error| error.to_string())?;
        self.file = Some(SourceFile {
            name,
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            payload,
        });
        Ok(())
    }

    /// Queries bounded original-file metadata without exporting payload bytes.
    /// # Returns
    /// Selection, basename, size and original-byte SHA-256, or explicit absence.
    pub(crate) fn file_summary(&self) -> Value {
        match &self.file {
            Some(file) => {
                json!({"selected":true,"name":file.name,"bytes":file.payload.bytes().len(),"bits":file.payload.target_bits(),"sha256":file.sha256})
            }
            None => json!({"selected":false,"name":"","bytes":null,"bits":null,"sha256":""}),
        }
    }

    fn options(document: &str) -> Result<Options, String> {
        if document.len() > 2048 {
            return Err("Source options exceed 2048 bytes.".into());
        }
        serde_json::from_str(document).map_err(|error| format!("Invalid source options: {error}"))
    }

    fn validate(&self, options: &Options) -> Result<(PayloadSourceConfig, u64, u64), String> {
        match options.mode.as_str() {
            "prbs15" => {
                Prbs15::new(options.seed).map_err(|error| error.to_string())?;
                Ok((PayloadSourceConfig::Prbs15, 0, 0))
            }
            "file" => {
                let file = self
                    .file
                    .as_ref()
                    .ok_or("Select a payload file before Apply.")?;
                let original = file.payload.target_bits();
                let fill = if options.append {
                    Some(PrbsFill {
                        target_bits: options.target_bits,
                        seed: options.fill_seed,
                    })
                } else {
                    None
                };
                let payload = file
                    .payload
                    .with_fill(fill)
                    .map_err(|error| error.to_string())?;
                let target = payload.target_bits();
                Ok((PayloadSourceConfig::File { payload }, original, target))
            }
            _ => Err("Choose Continuous PRBS15 or File.".into()),
        }
    }

    /// Validates draft options and computes meaningful bit counts in Rust.
    /// # Arguments
    /// * `document` - Source options JSON with no file contents.
    /// # Returns
    /// Original/target/fill counts or a diagnostic; no acceptance is changed.
    pub(crate) fn describe(&self, document: &str) -> Value {
        let result = Self::options(document).and_then(|options| self.validate(&options));
        match result {
            Ok((_, original, target)) => {
                json!({"error":"","original_bits":original,"target_bits":target,"fill_bits":target-original})
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
        let (config, _, _) = self.validate(&options)?;
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
                let reason = match &accepted.config {
                    PayloadSourceConfig::Prbs15 => format!("Continuous PRBS15 accepted with seed {} for the next run.", accepted.options.seed),
                    PayloadSourceConfig::File { payload } => format!("Exact-byte file source accepted: {} bytes, {} meaningful bits for the next run.", payload.bytes().len(), payload.target_bits()),
                };
                json!({"status":"complete","reason":reason})
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use docsis_ofdm_fec::simulation::source::file::MAX_FILE_BITS;

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
            model.file.as_ref().unwrap().payload.bytes(),
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
        for target in [0, 7, MAX_FILE_BITS + 1] {
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
}
