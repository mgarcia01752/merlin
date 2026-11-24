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
    model: Model,
}

impl Default for UpstreamSessionRust {
    /// Binds the upstream adapter to the actual shared desktop application host.
    /// # Returns
    /// Independent idle upstream model sharing only lifecycle/resource admission.
    fn default() -> Self {
        Self {
            model: Model::new(
                crate::downstream::workbench::session::Context::default().application,
            ),
        }
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
        QString::from(
            self.as_mut()
                .rust_mut()
                .model
                .apply(document.to_string().as_bytes())
                .err()
                .unwrap_or_default(),
        )
    }

    /// Requests a bounded upstream run through the application model.
    /// # Returns
    /// Empty after request; failure leaves previous results intact.
    pub fn start(mut self: core::pin::Pin<&mut Self>) -> QString {
        QString::from(
            self.as_mut()
                .rust_mut()
                .model
                .start()
                .err()
                .unwrap_or_default(),
        )
    }

    /// Toggles the existing public upstream pause/resume control.
    /// # Returns
    /// Empty after request, or the original ownership/control error.
    pub fn pause(&self) -> QString {
        QString::from(self.rust().model.pause().err().unwrap_or_default())
    }

    /// Cancels owned preparation or active upstream work without joining.
    /// # Returns
    /// Empty after request; the actual terminal still owns active admission.
    pub fn stop(mut self: core::pin::Pin<&mut Self>) -> QString {
        QString::from(
            self.as_mut()
                .rust_mut()
                .model
                .stop()
                .err()
                .unwrap_or_default(),
        )
    }

    /// Polls application events and projects bounded authoritative metadata.
    /// # Returns
    /// Scalar JSON, excluding captures, sample arrays and full scientific reports.
    pub fn poll(mut self: core::pin::Pin<&mut Self>) -> QString {
        QString::from(self.as_mut().rust_mut().model.poll().to_string())
    }

    /// Reads accepted scalar state independently of the selected tab.
    /// # Returns
    /// No simulated result is fabricated before an actual report exists.
    pub fn state(&self) -> QString {
        QString::from(self.rust().model.state().to_string())
    }
}
