/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Thin selected-direction routing over the already accepted canonical host.
use crate::shell::{presentation, ApplicationSessions, Direction};
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
        type DirectionShell = super::DirectionShellRust;

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
}

impl Default for DirectionShellRust {
    /// Borrows the same main-thread canonical application as both direction adapters.
    /// # Returns
    /// Thin host handles; selection and execution are not changed on construction.
    fn default() -> Self {
        Self {
            application: crate::downstream::workbench::session::Context::default().application,
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
