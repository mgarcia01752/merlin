/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! DPD input lifecycle and filesystem adapter; all scientific validation is Rust.
use crate::downstream::inputs::profile_input::{document, ProfileModel};
use crate::downstream::workbench::session::Context;
use crate::shell::{Retained, RetentionReservation};
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use document::MAX_INPUT_BYTES;
use serde_json::{json, Value};

/// GUI ownership of independently admitted profile draft and accepted snapshots.
#[derive(Default)]
pub struct ProfileInputRust {
    pub(super) context: Context,
    model: ProfileModel,
    // Model payload must drop before its covering reservation.
    model_guard: Option<RetentionReservation>,
}

impl ProfileInputRust {
    fn apply_profile(&mut self, profile_id: i32, settings: &str) -> Result<(), String> {
        let mut candidate = self.model.clone();
        candidate.context = self.context.clone();
        let guard = candidate.apply_transactional(profile_id, settings)?;
        self.model = candidate;
        self.model_guard = Some(guard);
        Ok(())
    }

    fn replace_model(&mut self, mut model: ProfileModel) -> Result<(), String> {
        model.context = self.context.clone();
        let admitted = model.admit(self.context.application.budget())?;
        self.adopt_model(admitted)
    }

    fn adopt_model(&mut self, admitted: Retained<ProfileModel>) -> Result<(), String> {
        if !self.context.application.budget().owns(&admitted) {
            return Err("Prepared profile belongs to another application host.".into());
        }
        let (mut model, guard) = admitted.into_parts();
        model.context = self.context.clone();
        self.model = model;
        self.model_guard = Some(guard);
        Ok(())
    }
}

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
        /// Queries the staged profile's bounded tree rows.
        /// # Arguments
        /// * `profile_id` - Available capture profile ID.
        /// # Returns
        /// Preview JSON or an error, without changing accepted input.
        #[qinvokable]
        fn preview(self: &ProfileInput, profile_id: i32) -> QString;
        /// Projects the current draft without applying it or starting a run.
        /// # Arguments
        /// * `profile_id` - Captured profile ID, not an array index.
        /// * `start`, `count` - Source offsets; zero count requests Full Span.
        /// # Returns
        /// Bounded plot JSON or a diagnostic preserving draft and accepted state.
        #[qinvokable]
        fn plot(self: &ProfileInput, profile_id: i32, start: i32, count: i32) -> QString;
        /// Inspects independent profile overlays without changing the draft.
        /// # Arguments
        /// * `request` - JSON profile IDs, carrier roles, metric and range.
        /// # Returns
        /// Bounded display metadata or an error; accepted state is untouched.
        #[qinvokable]
        fn inspect(self: &ProfileInput, request: &QString) -> QString;
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
    /// Queries the full modulation viewer using a bounded transient selection.
    /// # Arguments
    /// * `request` - JSON selection; never persisted or applied.
    /// # Returns
    /// Bounded metadata/overlays, or a diagnostic preserving all editor state.
    pub fn inspect(&self, request: &QString) -> QString {
        response(
            self.rust()
                .model
                .draft
                .as_ref()
                .ok_or_else(|| "Open a profile file first.".into())
                .and_then(|capture| {
                    crate::downstream::inputs::profile_input::plot::inspect(
                        capture,
                        &request.to_string(),
                    )
                }),
        )
    }

    /// Queries one role-aware viewport from the staged original capture.
    /// # Arguments
    /// * `profile_id` - Captured identity in 0..255.
    /// * `start`, `count` - Nonnegative offsets; zero count restores full span.
    /// # Returns
    /// Bounded display JSON; invalid selection never mutates editor state.
    pub fn plot(&self, profile_id: i32, start: i32, count: i32) -> QString {
        response((|| {
            let capture = self
                .rust()
                .model
                .draft
                .as_ref()
                .ok_or("Open a profile file first.")?;
            crate::downstream::inputs::profile_input::plot::viewport(
                capture,
                u8::try_from(profile_id).map_err(|_| "Invalid profile ID.")?,
                usize::try_from(start).map_err(|_| "Invalid start offset.")?,
                usize::try_from(count).map_err(|_| "Invalid carrier count.")?,
            )
        })())
    }
    /// Adopts already validated project captures without a file-picker round trip.
    pub fn reload(mut self: core::pin::Pin<&mut Self>) {
        let prepared =
            crate::downstream::workbench::project_load::take(&self.rust().context, |models| {
                models.profile.take().map(|model| *model)
            });
        if let Some(model) = prepared {
            let _ = self.as_mut().rust_mut().adopt_model(model);
        } else {
            let saved =
                crate::downstream::workbench::session::saved(&self.rust().context, "profile");
            if let Ok(model) = ProfileModel::from_saved(&saved) {
                let _ = self.as_mut().rust_mut().replace_model(model);
            }
        }
    }

    /// Restores a draft only after admitting its full independent editor owner.
    /// # Returns
    /// A bounded preview, or an admission error preserving previous editor state.
    pub fn begin(mut self: core::pin::Pin<&mut Self>) -> QString {
        let mut candidate = self.rust().model.clone();
        candidate.draft = candidate
            .accepted
            .as_ref()
            .map(|(capture, _)| capture.clone());
        let preview = match &candidate.accepted {
            Some((capture, id)) => capture.preview(*id),
            None => Ok(empty()),
        };
        response(preview.and_then(|preview| {
            self.as_mut().rust_mut().replace_model(candidate)?;
            Ok(preview)
        }))
    }

    /// Reads and stages only a bounded local regular file.
    /// # Arguments
    /// * `url` - Native picker URL; non-local schemes are rejected.
    /// # Returns
    /// Preview JSON or a diagnostic preserving the previous draft and accepted input.
    pub fn stage(mut self: core::pin::Pin<&mut Self>, url: &QString) -> QString {
        let result = crate::platform::input_file::read_local(&url.to_string(), MAX_INPUT_BYTES)
            .and_then(|(name, bytes)| {
                let mut candidate = self.rust().model.clone();
                let preview = candidate.stage_bytes(name, bytes)?;
                self.as_mut().rust_mut().replace_model(candidate)?;
                Ok(preview)
            });
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
    /// Queries a bounded selected-profile tree from the staged capture.
    /// # Arguments
    /// * `profile_id` - Requested capture profile ID, 0 through 255.
    /// # Returns
    /// Preview JSON or a diagnostic; accepted input remains unchanged.
    pub fn preview(&self, profile_id: i32) -> QString {
        response(
            self.rust()
                .model
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
        let mut candidate = self.rust().model.clone();
        candidate.draft = None;
        response(
            self.as_mut()
                .rust_mut()
                .replace_model(candidate)
                .map(|()| empty()),
        )
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
            .apply_profile(profile_id, &settings.to_string());
        QString::from(result.err().unwrap_or_default())
    }
    /// Discards a draft only after admitting the restored accepted editor.
    pub fn cancel(mut self: core::pin::Pin<&mut Self>) {
        let mut candidate = self.rust().model.clone();
        candidate.draft = candidate
            .accepted
            .as_ref()
            .map(|(capture, _)| capture.clone());
        let _ = self.as_mut().rust_mut().replace_model(candidate);
    }

    /// Reports readiness without inferring selection from defaults or filenames.
    /// # Arguments
    /// * `settings` - Accepted transmission configuration JSON.
    /// # Returns
    /// State JSON used by the workbench and OCD profile summary.
    pub fn state(&self, document: &QString) -> QString {
        let value = self.rust().model.state(&document.to_string());
        QString::from(value.to_string())
    }
}

#[cfg(test)]
#[path = "profile_input/retention_tests.rs"]
mod retention_tests;
