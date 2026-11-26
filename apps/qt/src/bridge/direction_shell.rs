/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Thin selected-direction routing over the already accepted canonical host.
use crate::shell::{presentation, ApplicationSessions, Direction};
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
mod close;

#[cxx_qt::bridge]
pub mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type DirectionShell = super::DirectionShellRust;

        /// Captures both native projects before any close-time dirty decision.
        /// # Returns
        /// Next compiled direction tag or ready metadata; no cancellation occurs.
        #[qinvokable]
        fn begin_close(self: Pin<&mut DirectionShell>) -> QString;
        /// Resolves one exact native close target, preserving the live host on Cancel.
        /// # Arguments
        /// * `discard`, `cancel` - Explicit current prompt outcome.
        /// # Returns
        /// Next direction, ready, cancellation or original stale/dirty error.
        #[qinvokable]
        fn close_decision(self: Pin<&mut DirectionShell>, discard: bool, cancel: bool) -> QString;
        /// Rechecks both guards before irreversible admission shutdown.
        /// # Returns
        /// Ready only after both decisions still match; failures keep the app live.
        #[qinvokable]
        fn finish_close(self: Pin<&mut DirectionShell>) -> QString;

        /// Reads bounded canonical metadata for both persistent direction views.
        /// # Returns
        /// Scalar JSON; no scientific configuration, observation or result is copied.
        #[qinvokable]
        fn state(self: &DirectionShell) -> QString;

        /// Selects one existing direction without redirecting an active worker.
        /// # Arguments
        /// * `direction` - Exactly downstream or upstream; unknown values reject.
        /// # Returns
        /// True after selecting; false leaves the previous selection unchanged.
        #[qinvokable]
        fn select(self: &DirectionShell, direction: &QString) -> bool;

        /// Rechecks captured direction identity immediately before a scoped action.
        /// # Arguments
        /// * `direction` - Origin of the action, which must still be selected.
        /// * `identity` - Unchanged opaque canonical key captured by that origin.
        /// # Returns
        /// False for stale/wrong-direction actions without touching either owner.
        #[qinvokable]
        fn accepts(self: &DirectionShell, direction: &QString, identity: &QString) -> bool;
    }
}

/// Existing host handles only; no additional scientific owner or payload allocation.
pub struct DirectionShellRust {
    application: ApplicationSessions,
    close: Option<Box<crate::shell::Retained<close::Intent>>>,
}

impl Default for DirectionShellRust {
    /// Borrows the same main-thread canonical application as both direction adapters.
    /// # Returns
    /// Thin host handles; selection and execution are not changed on construction.
    fn default() -> Self {
        Self {
            application: crate::downstream::workbench::session::Context::default().application,
            close: None,
        }
    }
}

impl DirectionShellRust {
    fn select(&self, value: &QString) -> bool {
        let Some(direction) = parse_direction(value) else {
            return false;
        };
        self.application.select_direction(direction);
        true
    }

    fn accepts(&self, value: &QString, identity: &QString) -> bool {
        let Some(direction) = parse_direction(value) else {
            return false;
        };
        self.application.selected_direction() == direction
            && identity
                == &QString::from(presentation::identity(
                    self.application.snapshot(direction).key,
                ))
    }
}

impl bridge::DirectionShell {
    /// Captures both native guards before any dirty prompt or worker cancellation.
    /// # Returns
    /// Bounded direction/ready metadata; scientific data and revisions stay in Rust.
    pub fn begin_close(mut self: core::pin::Pin<&mut Self>) -> QString {
        let result = self.as_mut().rust_mut().begin_close();
        QString::from(
            result
                .unwrap_or_else(|error| serde_json::json!({"error":error}))
                .to_string(),
        )
    }

    /// Resolves the native current direction after the actual dialog closes.
    /// # Arguments
    /// * `discard`, `cancel` - Explicit decision, without any caller-provided guard.
    /// # Returns
    /// Next direction/ready metadata or original error; Cancel leaves admission open.
    pub fn close_decision(
        mut self: core::pin::Pin<&mut Self>,
        discard: bool,
        cancel: bool,
    ) -> QString {
        let result = self.as_mut().rust_mut().close_decision(discard, cancel);
        QString::from(
            result
                .unwrap_or_else(|error| serde_json::json!({"error":error}))
                .to_string(),
        )
    }

    /// Rechecks both exact native decisions before closing shared execution admission.
    /// # Returns
    /// Ready only after successful atomic shutdown; errors leave the window open.
    pub fn finish_close(mut self: core::pin::Pin<&mut Self>) -> QString {
        let result = self.as_mut().rust_mut().finish_close();
        QString::from(
            result
                .unwrap_or_else(|error| serde_json::json!({"error":error}))
                .to_string(),
        )
    }
    /// Projects existing application metadata through the accepted scalar owner.
    /// # Returns
    /// Bounded JSON for admission by the shell's persistent presentation owner.
    pub fn state(&self) -> QString {
        QString::from(presentation::state(&self.rust().application).to_string())
    }

    /// Changes only the canonical presentation cursor.
    /// # Arguments
    /// * `direction` - One compiled direction tag; arbitrary strings are not retained.
    /// # Returns
    /// Whether selection was recognized; existing run ownership remains unchanged.
    pub fn select(&self, direction: &QString) -> bool {
        self.rust().select(direction)
    }

    /// Rejects stale or hidden-origin control intents before native dispatch.
    /// # Arguments
    /// * `direction` - Explicit origin, which must still be selected.
    /// * `identity` - Original opaque key; comparison never parses a JavaScript number.
    /// # Returns
    /// True only for the exact current selected direction/configuration key.
    pub fn accepts(&self, direction: &QString, identity: &QString) -> bool {
        self.rust().accepts(direction, identity)
    }
}

#[cfg(test)]
#[path = "direction_shell/tests.rs"]
mod tests;

fn parse_direction(value: &QString) -> Option<Direction> {
    // Compare borrowed Qt strings; an unknown/unbounded input is never copied.
    [Direction::Downstream, Direction::Upstream]
        .into_iter()
        .find(|direction| value == &QString::from(presentation::direction_tag(*direction)))
}
