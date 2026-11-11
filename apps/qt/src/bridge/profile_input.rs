/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! DPD input lifecycle and filesystem adapter; all scientific validation is Rust.
pub use crate::inputs::profile_input::ProfileModel as ProfileInputRust;
use crate::inputs::profile_input::{document, settings};
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use document::MAX_INPUT_BYTES;
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

fn empty() -> Value {
    json!({"source_name":"", "format":"", "profile_ids":[], "rows":[], "selected_profile_id":-1})
}
fn response(result: Result<Value, String>) -> QString {
    QString::from(match result {
        Ok(value) => value.to_string(),
        Err(error) => json!({"error":error}).to_string(),
    })
}
impl bridge::ProfileInput {
    /// Adopts already validated project captures without a file-picker round trip.
    pub fn reload(mut self: core::pin::Pin<&mut Self>) {
        if let Ok(model) =
            ProfileInputRust::from_saved(&crate::workbench::session::saved("profile"))
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
        let result = crate::platform::input_file::read_local(&url.to_string(), MAX_INPUT_BYTES)
            .and_then(|(name, bytes)| self.as_mut().rust_mut().stage_bytes(name, bytes));
        response(result)
    }
    /// Opens the OS picker without a Qt fallback; cancellation keeps the draft.
    /// # Returns
    /// Draft preview or a diagnostic preserving the accepted input.
    pub fn choose_file(mut self: core::pin::Pin<&mut Self>) -> QString {
        match crate::platform::native_file_picker::choose(
            "Open Downstream OFDM Modulation-Profile Capture",
        ) {
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
                    "../../../../examples/measurements/single-channel/modulation_profile.bin"
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
            crate::workbench::session::set_profile(selected);
            let section=self.rust().accepted.as_ref().map(|(c,id)|json!({"file":crate::workbench::project_file::encode(&c.name,&c.original),"profile_id":id})).unwrap_or(Value::Null);
            crate::workbench::session::put_saved("profile", section);
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
