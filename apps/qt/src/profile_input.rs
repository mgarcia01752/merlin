/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! DPD input lifecycle and filesystem adapter; all scientific validation is Rust.
mod document;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use docsis_ofdm_fec::transmission::TransmissionSettings;
use document::{Capture, MAX_INPUT_BYTES};
use serde_json::{json, Value};

#[cxx_qt::bridge]
pub mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type ProfileInput = super::ProfileInputRust;

        /// Starts an edit session from the accepted input.
        /// # Returns
        /// JSON preview; opening does not modify accepted state.
        #[qinvokable]
        fn begin(self: Pin<&mut ProfileInput>) -> QString;
        /// Stages a bounded local file, detected by contents.
        /// # Arguments
        /// * `url` - Local file URL from the native picker.
        /// # Returns
        /// Preview JSON or an error; failure preserves the previous draft.
        #[qinvokable]
        fn stage(self: Pin<&mut ProfileInput>, url: &QString) -> QString;
        /// Opens the OS-native picker and stages a chosen capture.
        /// # Returns
        /// Preview JSON; Cancel preserves the current draft.
        #[qinvokable]
        fn choose_file(self: Pin<&mut ProfileInput>) -> QString;
        /// Stages the sanitized embedded binary demo without filesystem writes.
        /// # Returns
        /// Validated demo preview JSON or an error.
        #[qinvokable]
        fn demo(self: Pin<&mut ProfileInput>) -> QString;
        /// Queries the staged profile's bounded tree rows.
        /// # Arguments
        /// * `profile_id` - Available capture profile ID.
        /// # Returns
        /// Preview JSON or an error, without changing accepted input.
        #[qinvokable]
        fn preview(self: &ProfileInput, profile_id: i32) -> QString;
        /// Stages removal; Cancel can still preserve the accepted input.
        /// # Returns
        /// Empty-input preview JSON.
        #[qinvokable]
        fn clear_draft(self: Pin<&mut ProfileInput>) -> QString;
        /// Validates and accepts the staged capture/profile, or staged removal.
        /// # Arguments
        /// * `profile_id` - Staged capture selection; ignored for removal.
        /// * `settings` - Accepted OCD transmission JSON.
        /// # Returns
        /// Empty on success or a diagnostic preserving accepted input.
        #[qinvokable]
        fn apply(self: Pin<&mut ProfileInput>, profile_id: i32, settings: &QString) -> QString;
        /// Reloads accepted original captures after atomic project Open.
        #[qinvokable]
        fn reload(self: Pin<&mut ProfileInput>);
        /// Drops staged bytes and restores the accepted snapshot.
        #[qinvokable]
        fn cancel(self: Pin<&mut ProfileInput>);
        /// Reports accepted profile/OCD readiness using the core validator.
        /// # Arguments
        /// * `settings` - Accepted OCD transmission JSON.
        /// # Returns
        /// Compact JSON status/reason and selected-profile summary.
        #[qinvokable]
        fn state(self: &ProfileInput, settings: &QString) -> QString;
    }
}

/// Original bytes and decoded records remain Rust-owned behind this adapter.
#[derive(Default)]
pub struct ProfileInputRust {
    draft: Option<Capture>,
    accepted: Option<(Capture, u8)>,
}

fn empty() -> Value {
    json!({"source_name":"", "format":"", "profile_ids":[], "rows":[], "selected_profile_id":-1})
}
fn response(result: Result<Value, String>) -> QString {
    QString::from(match result {
        Ok(value) => value.to_string(),
        Err(error) => json!({"error":error}).to_string(),
    })
}
fn settings(document: &str) -> Result<TransmissionSettings, String> {
    if document.len() > 4096 {
        return Err("OCD request exceeds 4096 bytes.".into());
    }
    let settings: TransmissionSettings =
        serde_json::from_str(document).map_err(|error| error.to_string())?;
    settings.validate().map_err(|error| error.to_string())?;
    Ok(settings)
}

impl ProfileInputRust {
    fn from_saved(value: &Value) -> Result<Self, String> {
        if value.is_null() {
            return Ok(Self::default());
        }
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Saved {
            file: Value,
            profile_id: u8,
        }
        let saved: Saved = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
        let (name, bytes) = crate::project_file::decode(&saved.file)?;
        let capture = Capture::decode(name, bytes)?;
        capture.profile(saved.profile_id)?;
        Ok(Self {
            draft: Some(capture.clone()),
            accepted: Some((capture, saved.profile_id)),
        })
    }

    fn stage_bytes(&mut self, name: String, bytes: Vec<u8>) -> Result<Value, String> {
        let capture = Capture::decode(name, bytes)?;
        let id = capture
            .profiles
            .iter()
            .find(|profile| profile.profile_id == 0)
            .map_or(capture.profiles[0].profile_id, |profile| profile.profile_id);
        let preview = capture.preview(id)?;
        self.draft = Some(capture);
        Ok(preview)
    }
    fn accept(&mut self, id: i32, document: &str) -> Result<(), String> {
        if let Some(capture) = &self.draft {
            let id = u8::try_from(id).map_err(|_| "Choose a valid profile ID.")?;
            capture.compatible(id, &settings(document)?)?;
            self.accepted = Some((capture.clone(), id));
        } else {
            self.accepted = None;
        }
        Ok(())
    }
}

/// Decodes an embedded original profile without mutating current state.
/// # Arguments
/// * `value` - Original bytes and selected profile ID.
/// * `_settings` - Current numerology; cross-input errors remain readiness diagnostics.
/// # Returns
/// Typed profile, or None when unconfigured.
/// # Errors
/// Reports malformed bytes, digest or unavailable profile IDs.
pub(crate) fn restore_profile(
    value: &Value,
    _settings: &TransmissionSettings,
) -> Result<Option<docsis_ofdm_fec::profile::candidate::CapturedProfile>, String> {
    let model = ProfileInputRust::from_saved(value)?;
    model
        .accepted
        .map(|(capture, id)| capture.profile(id).cloned())
        .transpose()
}
impl bridge::ProfileInput {
    /// Adopts already validated project captures without a file-picker round trip.
    pub fn reload(mut self: core::pin::Pin<&mut Self>) {
        if let Ok(model) = ProfileInputRust::from_saved(&crate::workbench_session::saved("profile"))
        {
            self.as_mut().rust_mut().draft = model.draft;
            self.as_mut().rust_mut().accepted = model.accepted;
        }
    }

    /// Restores a draft from the accepted snapshot; accepted input stays unchanged.
    /// # Returns
    /// A bounded preview of the accepted selection or an empty tree.
    pub fn begin(mut self: core::pin::Pin<&mut Self>) -> QString {
        let accepted = self.rust().accepted.clone();
        self.as_mut().rust_mut().draft = accepted.as_ref().map(|(capture, _)| capture.clone());
        response(match accepted {
            Some((capture, id)) => capture.preview(id),
            None => Ok(empty()),
        })
    }
    /// Reads and stages only a bounded local regular file.
    /// # Arguments
    /// * `url` - Native picker URL; non-local schemes are rejected.
    /// # Returns
    /// Preview JSON or a diagnostic preserving the previous draft and accepted input.
    pub fn stage(mut self: core::pin::Pin<&mut Self>, url: &QString) -> QString {
        let result = crate::input_file::read_local(&url.to_string(), MAX_INPUT_BYTES)
            .and_then(|(name, bytes)| self.as_mut().rust_mut().stage_bytes(name, bytes));
        response(result)
    }
    /// Opens the OS picker without a Qt fallback; cancellation keeps the draft.
    /// # Returns
    /// Draft preview or a diagnostic preserving the accepted input.
    pub fn choose_file(mut self: core::pin::Pin<&mut Self>) -> QString {
        match crate::native_file_picker::choose("Open Downstream OFDM Modulation-Profile Capture") {
            Ok(Some(url)) => self.as_mut().stage(&QString::from(url)),
            Ok(None) => response(Ok(json!({"cancelled":true}))),
            Err(error) => response(Err(error)),
        }
    }
    /// Stages the same sanitized binary fixture identified by the demo provenance.
    /// # Returns
    /// Validated preview JSON; no files are written or automatically accepted.
    pub fn demo(mut self: core::pin::Pin<&mut Self>) -> QString {
        response(
            self.as_mut().rust_mut().stage_bytes(
                "modulation_profile.bin".into(),
                include_bytes!(
                    "../../../examples/measurements/single-channel/modulation_profile.bin"
                )
                .to_vec(),
            ),
        )
    }
    /// Queries a bounded selected-profile tree from the staged capture.
    /// # Arguments
    /// * `profile_id` - Requested capture profile ID, 0 through 255.
    /// # Returns
    /// Preview JSON or a diagnostic; accepted input remains unchanged.
    pub fn preview(&self, profile_id: i32) -> QString {
        response(
            self.rust()
                .draft
                .as_ref()
                .ok_or_else(|| "Open a profile file first.".to_string())
                .and_then(|capture| {
                    capture.preview(u8::try_from(profile_id).map_err(|_| "Invalid profile ID.")?)
                }),
        )
    }
    /// Stages removal while retaining accepted input until Apply.
    /// # Returns
    /// An empty preview document.
    pub fn clear_draft(mut self: core::pin::Pin<&mut Self>) -> QString {
        self.as_mut().rust_mut().draft = None;
        response(Ok(empty()))
    }
    /// Accepts only a profile compatible with the accepted OCD configuration.
    /// # Arguments
    /// * `profile_id` - Staged profile ID, ignored when removing the input.
    /// * `settings` - Bounded accepted transmission configuration JSON.
    /// # Returns
    /// Empty on success or a diagnostic preserving accepted input.
    pub fn apply(
        mut self: core::pin::Pin<&mut Self>,
        profile_id: i32,
        settings: &QString,
    ) -> QString {
        let result = self
            .as_mut()
            .rust_mut()
            .accept(profile_id, &settings.to_string());
        if result.is_ok() {
            let selected = self
                .rust()
                .accepted
                .as_ref()
                .and_then(|(capture, id)| capture.profile(*id).ok())
                .cloned();
            crate::workbench_session::set_profile(selected);
            let section=self.rust().accepted.as_ref().map(|(c,id)|json!({"file":crate::project_file::encode(&c.name,&c.original),"profile_id":id})).unwrap_or(Value::Null);
            crate::workbench_session::put_saved("profile", section);
        }
        QString::from(result.err().unwrap_or_default())
    }
    /// Discards draft input and restores the accepted snapshot without I/O.
    pub fn cancel(mut self: core::pin::Pin<&mut Self>) {
        let draft = self
            .rust()
            .accepted
            .as_ref()
            .map(|(capture, _)| capture.clone());
        self.as_mut().rust_mut().draft = draft;
    }
    /// Reports readiness without inferring selection from defaults or filenames.
    /// # Arguments
    /// * `settings` - Accepted transmission configuration JSON.
    /// # Returns
    /// State JSON used by the workbench and OCD profile summary.
    pub fn state(&self, document: &QString) -> QString {
        let value = match &self.rust().accepted {
            None => {
                json!({"selected":false,"profile_status":"attention","ofdm_status":"attention","reason":"A validated modulation-profile PNM input is required.","summary":{}})
            }
            Some((capture, id)) => {
                let diagnostic = settings(&document.to_string())
                    .and_then(|settings| capture.compatible(*id, &settings));
                match diagnostic {
                    Ok(()) => {
                        json!({"selected":true,"profile_status":"complete","ofdm_status":"complete","reason":"Profile input and accepted OCD settings are valid and compatible.","summary":{"profile_id":id.to_string(),"source_file":capture.name,"subcarrier_assignments":format!("{} assignments",capture.profile(*id).map(|profile|profile.assignments.len()).unwrap_or(0))}})
                    }
                    Err(error) => {
                        json!({"selected":true,"profile_status":"error","ofdm_status":"error","reason":error,"summary":{}})
                    }
                }
            }
        };
        QString::from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const BINARY: &[u8] =
        include_bytes!("../../../examples/measurements/single-channel/modulation_profile.bin");
    const JSON: &[u8] =
        include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");

    #[test]
    fn binary_and_json_use_the_same_profile_validation() {
        let binary = Capture::decode("wrong.json".into(), BINARY.to_vec()).unwrap();
        let json = Capture::decode("wrong.bin".into(), JSON.to_vec()).unwrap();
        assert_eq!(binary.format, "Binary PNM");
        assert_eq!(json.format, "JSON text");
        for (left, right) in binary.profiles.iter().zip(json.profiles.iter()) {
            assert_eq!(left.profile_id, right.profile_id);
            assert_eq!(left.key, right.key);
            assert_eq!(left.assignments, right.assignments);
        }
        assert_eq!(binary.profiles.len(), 4);
        assert_eq!(binary.profiles[0].assignments.len(), 7480);
    }
    #[test]
    fn staged_failures_removal_and_profile_rejection_preserve_accepted_input() {
        let mut model = ProfileInputRust::default();
        model
            .stage_bytes("demo.bin".into(), BINARY.to_vec())
            .unwrap();
        let config = json!(TransmissionSettings::default()).to_string();
        model.accept(0, &config).unwrap();
        assert!(model
            .stage_bytes("bad.bin".into(), b"not JSON".to_vec())
            .is_err());
        assert_eq!(model.accepted.as_ref().unwrap().1, 0);
        assert!(model.accept(99, &config).is_err());
        let mut mismatched = json!(TransmissionSettings::default());
        mismatched["subcarrier_spacing"] = json!("50000");
        assert!(model.accept(0, &mismatched.to_string()).is_err());
        assert_eq!(model.accepted.as_ref().unwrap().1, 0);
        model.draft = None;
        assert!(model.accepted.is_some());
        model.accept(-1, &config).unwrap();
        assert!(model.accepted.is_none());
    }
}
