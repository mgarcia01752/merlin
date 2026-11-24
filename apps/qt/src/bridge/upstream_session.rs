/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Thin QObject adapter; the direction model owns asynchronous public API calls.
use crate::upstream::workbench::{Model, MAX_CONFIGURATION_BYTES};
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;

#[cxx_qt::bridge]
pub mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type UpstreamSession = super::UpstreamSessionRust;

        /// Starts off-GUI validation of an existing public configuration document.
        /// # Arguments
        /// * `document` - Original bounded upstream configuration JSON.
        /// # Returns
        /// Empty when staged; immediate error otherwise. Poll reports final acceptance.
        #[qinvokable]
        fn apply(self: Pin<&mut UpstreamSession>, document: &QString) -> QString;
        /// Requests off-GUI target preflight followed by shared run admission.
        /// # Returns
        /// Empty when requested; error preserves prior configuration and report.
        #[qinvokable]
        fn start(self: Pin<&mut UpstreamSession>) -> QString;
        /// Toggles this upstream run's public pause/resume control.
        /// # Returns
        /// Empty after request; error cannot control a different session.
        #[qinvokable]
        fn pause(self: &UpstreamSession) -> QString;
        /// Cancels preparation or requests exact-token cooperative upstream stop.
        /// # Returns
        /// Empty after request; execution admission remains until actual termination.
        #[qinvokable]
        fn stop(self: Pin<&mut UpstreamSession>) -> QString;
        /// Adopts bounded current-generation completions without waiting for workers.
        /// # Returns
        /// Scalar readiness, progress, recovery results and lossless application IDs.
        #[qinvokable]
        fn poll(self: Pin<&mut UpstreamSession>) -> QString;
        /// Reads accepted state without polling or copying scientific arrays.
        /// # Returns
        /// Same bounded scalar presentation shape as Poll.
        #[qinvokable]
        fn state(self: &UpstreamSession) -> QString;
    }
}

/// Fixed-direction Qt owner; tab selection never changes its scientific session.
pub struct UpstreamSessionRust {
    application: crate::shell::ApplicationSessions,
    model: Option<Box<Model>>,
}

impl Default for UpstreamSessionRust {
    /// Binds upstream controls to the canonical host without a panic fallback.
    /// # Returns
    /// Independent admitted state, or minimal references while fixed storage rejects.
    fn default() -> Self {
        Self::for_application(crate::downstream::workbench::session::Context::default().application)
    }
}

impl UpstreamSessionRust {
    /// Preflights the actual fixed GUI model against this exact application.
    /// # Arguments
    /// * `application` - Canonical shared lifecycle and retention host.
    /// # Returns
    /// Only host references on rejection; no full uncharged model is constructed.
    fn for_application(application: crate::shell::ApplicationSessions) -> Self {
        let model = Model::new(application.clone()).ok();
        Self { application, model }
    }

    /// Retries a rejected constructor without changing accepted readiness or data.
    /// # Returns
    /// Existing/admitted model or None while the same host remains at its limit.
    fn ensure_model(&mut self) -> Option<&mut Model> {
        if self.model.is_none() {
            self.model = Model::new(self.application.clone()).ok();
        }
        self.model.as_deref_mut()
    }

    /// Reads bounded control state, including canonical global admission.
    /// # Returns
    /// Normal model state or explicit unavailability without fabricated results.
    fn current_state(&self) -> serde_json::Value {
        self.model.as_deref().map_or_else(
            || {
                let active = self.application.active();
                serde_json::json!({"configured":false,"preparing":false,"busy":false,
                    "application_busy":active.is_some(),
                    "active_direction":active.map(|run|match run.token.key.direction {
                        crate::shell::Direction::Downstream=>"downstream",
                        crate::shell::Direction::Upstream=>"upstream",
                    }),
                    "can_start":false,"session":null,"generation":null,"progress":null,
                    "has_result":false,"result":null,
                    "error":crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE})
            },
            Model::state,
        )
    }
}

impl bridge::UpstreamSession {
    /// Stages existing public JSON for background scientific validation.
    /// # Arguments
    /// * `document` - Input limited before UTF-8 copy and again by exact byte count.
    /// # Returns
    /// Empty after staging; errors preserve accepted state.
    pub fn apply(mut self: core::pin::Pin<&mut Self>, document: &QString) -> QString {
        if document.len() > MAX_CONFIGURATION_BYTES as isize {
            return QString::from("Upstream configuration exceeds the input byte limit.");
        }
        let mut owner = self.as_mut().rust_mut();
        let Some(model) = owner.ensure_model() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        QString::from(
            model
                .apply(document.to_string().as_bytes())
                .err()
                .unwrap_or_default(),
        )
    }

    /// Requests a bounded upstream run through the application model.
    /// # Returns
    /// Empty after request; failure leaves previous results intact.
    pub fn start(mut self: core::pin::Pin<&mut Self>) -> QString {
        let mut owner = self.as_mut().rust_mut();
        let Some(model) = owner.ensure_model() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        QString::from(model.start().err().unwrap_or_default())
    }

    /// Toggles the existing public upstream pause/resume control.
    /// # Returns
    /// Empty after request, or the original ownership/control error.
    pub fn pause(&self) -> QString {
        let Some(model) = self.rust().model.as_deref() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        QString::from(model.pause().err().unwrap_or_default())
    }

    /// Cancels owned preparation or active upstream work without joining.
    /// # Returns
    /// Empty after request; the actual terminal still owns active admission.
    pub fn stop(mut self: core::pin::Pin<&mut Self>) -> QString {
        let mut owner = self.as_mut().rust_mut();
        let Some(model) = owner.model.as_deref_mut() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        QString::from(model.stop().err().unwrap_or_default())
    }

    /// Polls application events and projects bounded authoritative metadata.
    /// # Returns
    /// Scalar JSON, excluding captures, sample arrays and full scientific reports.
    pub fn poll(mut self: core::pin::Pin<&mut Self>) -> QString {
        let state = if let Some(model) = self.as_mut().rust_mut().ensure_model() {
            model.poll()
        } else {
            self.rust().current_state()
        };
        QString::from(state.to_string())
    }

    /// Reads accepted scalar state independently of the selected tab.
    /// # Returns
    /// No simulated result is fabricated before an actual report exists.
    pub fn state(&self) -> QString {
        QString::from(self.rust().current_state().to_string())
    }
}

#[cfg(test)]
#[path = "upstream_session/runtime_tests.rs"]
mod runtime_tests;
