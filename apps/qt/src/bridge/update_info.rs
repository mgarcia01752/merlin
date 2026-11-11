/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Thin UI adapter; release metadata and preference ownership stay in Rust.
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
        type UpdateInfo = super::UpdateInfoRust;
        /// Starts one bounded background release check without blocking.
        /// # Returns
        /// Current plain display state as bounded JSON.
        #[qinvokable]
        fn check(self: Pin<&mut UpdateInfo>) -> QString;
        /// Starts the installed Ubuntu update after recovery succeeds.
        /// # Returns
        /// Bounded progress state, or an installation diagnostic.
        #[qinvokable]
        fn install(self: Pin<&mut UpdateInfo>) -> QString;
        /// Polls bounded status without waiting for the worker.
        /// # Returns
        /// Current status and startup preference as bounded JSON.
        #[qinvokable]
        fn poll(self: Pin<&mut UpdateInfo>) -> QString;
        /// Saves the application-global startup check preference.
        /// # Arguments
        /// * `enabled` - Whether future launches check once at startup.
        /// # Returns
        /// Current preference/status; save failure leaves the preference unchanged.
        #[qinvokable]
        fn set_enabled(self: Pin<&mut UpdateInfo>, enabled: bool) -> QString;
    }
}
/// GUI-thread adapter holding one independent release-check state machine.
#[derive(Default)]
pub struct UpdateInfoRust {
    model: crate::platform::updates::Model,
}
impl bridge::UpdateInfo {
    /// Dispatches one read-only lookup and returns immediately with display JSON.
    /// # Returns
    /// Bounded status without source measurements or credentials.
    pub fn check(mut self: core::pin::Pin<&mut Self>) -> QString {
        QString::from(self.as_mut().rust_mut().model.start().to_string())
    }
    /// Starts the verified install worker without blocking the UI thread.
    /// # Returns
    /// Bounded status; source checkouts never install system packages.
    pub fn install(mut self: core::pin::Pin<&mut Self>) -> QString {
        QString::from(self.as_mut().rust_mut().model.install().to_string())
    }
    /// Polls completion on the UI thread without blocking.
    /// # Returns
    /// Current bounded status and startup preference.
    pub fn poll(mut self: core::pin::Pin<&mut Self>) -> QString {
        QString::from(self.as_mut().rust_mut().model.poll().to_string())
    }
    /// Persists the global startup preference independently of WB configuration.
    /// # Arguments
    /// * `enabled` - Startup lookup choice.
    /// # Returns
    /// Current state, including any persistence diagnostic.
    pub fn set_enabled(mut self: core::pin::Pin<&mut Self>, enabled: bool) -> QString {
        QString::from(
            self.as_mut()
                .rust_mut()
                .model
                .set_enabled(enabled)
                .to_string(),
        )
    }
}
