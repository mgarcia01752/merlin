/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Thin logical presentation admission adapter on the canonical application host.
use crate::{shared::presentation::retention::View, shell::RetentionBudget};
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
        type PresentationRetention = super::PresentationRetentionRust;

        /// Returns the named logical byte/entry/traversal policies for QML counting.
        /// # Returns
        /// A bounded JSON policy; no data or view reservation is created.
        #[qinvokable]
        fn limits(self: &PresentationRetention) -> QString;

        /// Pre-admits measured candidate data while current data remains covered.
        /// # Arguments
        /// * `utf16_bytes` - Nonnegative exact logical UTF-16 byte count.
        /// * `entries` - Nonnegative exact array/object entry count.
        /// # Returns
        /// True if pending, false without changing the current presentation.
        #[qinvokable]
        fn prepare(self: Pin<&mut PresentationRetention>, utf16_bytes: f64, entries: f64) -> bool;

        /// Retires old coverage after QML assigns the pre-admitted candidate.
        /// # Returns
        /// True for a pending replacement, false without mutation otherwise.
        #[qinvokable]
        fn commit(self: Pin<&mut PresentationRetention>) -> bool;

        /// Drops an unpublished candidate, preserving accepted data and open status.
        #[qinvokable]
        fn cancel(self: Pin<&mut PresentationRetention>);

        /// Admits one view-opening slot before Qt opens the dialog.
        /// # Returns
        /// False preserves its existing retained data and visibility state.
        #[qinvokable]
        fn begin_open(self: Pin<&mut PresentationRetention>) -> bool;

        /// Releases open status only; hidden retained data stays admitted.
        #[qinvokable]
        fn end_open(self: Pin<&mut PresentationRetention>);

        /// Releases data after QML clears its corresponding persistent aliases.
        /// Empty closed state is disposed, allowing future same-host retry.
        #[qinvokable]
        fn clear(self: Pin<&mut PresentationRetention>);
    }
}

/// Minimal adapter handles; admitted fixed state is allocated only when needed.
pub struct PresentationRetentionRust {
    state: Option<Box<View>>,
    budget: RetentionBudget,
}
impl Default for PresentationRetentionRust {
    /// Retains the same canonical main-thread budget used by both direction adapters.
    /// # Returns
    /// Bare handles; first data/open admission constructs the guarded state.
    fn default() -> Self {
        Self {
            state: None,
            budget: crate::downstream::workbench::session::Context::default()
                .application
                .budget()
                .clone(),
        }
    }
}
impl PresentationRetentionRust {
    fn prepare(&mut self, utf16_bytes: f64, entries: f64) -> bool {
        if let Some(state) = &mut self.state {
            return state.prepare(utf16_bytes, entries);
        }
        let Ok(mut state) = View::new(&self.budget) else {
            return false;
        };
        if !state.prepare(utf16_bytes, entries) {
            return false;
        }
        self.state = Some(state);
        true
    }
    fn begin_open(&mut self) -> bool {
        if let Some(state) = &mut self.state {
            return state.begin_open();
        }
        let Ok(mut state) = View::new(&self.budget) else {
            return false;
        };
        if !state.begin_open() {
            return false;
        }
        self.state = Some(state);
        true
    }
    fn dispose_empty(&mut self) {
        if self.state.as_ref().is_some_and(|state| state.is_empty()) {
            self.state = None;
        }
    }
}
impl bridge::PresentationRetention {
    /// Supplies authoritative application limits to the pure QML counter.
    /// # Returns
    /// Bounded JSON containing logical UTF-16 byte, entry and traversal ceilings.
    pub fn limits(&self) -> QString {
        use crate::shell::constants::{
            MAX_SESSION_PRESENTATION_ENTRIES, MAX_SESSION_PRESENTATION_UTF16_BYTES,
        };
        QString::from(
            serde_json::json!({
                "max_utf16_bytes":MAX_SESSION_PRESENTATION_UTF16_BYTES,
                "max_entries":MAX_SESSION_PRESENTATION_ENTRIES,
                "max_depth":crate::shared::presentation::constants::MAX_PRESENTATION_VALUE_DEPTH,
            })
            .to_string(),
        )
    }
    /// Reserves both old and candidate data through the QML assignment boundary.
    /// # Arguments
    /// * `utf16_bytes` - Measured nonnegative safe-integer UTF-16 byte count.
    /// * `entries` - Measured nonnegative safe-integer array/object count.
    /// # Returns
    /// False leaves the prior accepted owner and application usage unchanged.
    pub fn prepare(mut self: core::pin::Pin<&mut Self>, utf16_bytes: f64, entries: f64) -> bool {
        self.as_mut().rust_mut().prepare(utf16_bytes, entries)
    }
    /// Finishes an admitted replacement after QML assigns its candidate.
    /// # Returns
    /// True exactly when a pending reservation is moved to accepted ownership.
    pub fn commit(mut self: core::pin::Pin<&mut Self>) -> bool {
        self.as_mut()
            .rust_mut()
            .state
            .as_mut()
            .is_some_and(|state| state.commit())
    }
    /// Aborts only unpublished candidate coverage; accepted data stays retained.
    pub fn cancel(mut self: core::pin::Pin<&mut Self>) {
        let mut owner = self.as_mut().rust_mut();
        if let Some(state) = &mut owner.state {
            state.cancel();
        }
        owner.dispose_empty();
    }
    /// Pre-admits visibility without discarding current retained data on failure.
    /// # Returns
    /// True for an admitted or already open slot; false leaves all owners unchanged.
    pub fn begin_open(mut self: core::pin::Pin<&mut Self>) -> bool {
        self.as_mut().rust_mut().begin_open()
    }
    /// Releases visibility coverage while closed/hidden presentation data persists.
    pub fn end_open(mut self: core::pin::Pin<&mut Self>) {
        let mut owner = self.as_mut().rust_mut();
        if let Some(state) = &mut owner.state {
            state.end_open();
        }
        owner.dispose_empty();
    }
    /// Releases data after its QML aliases clear and disposes empty closed state.
    pub fn clear(mut self: core::pin::Pin<&mut Self>) {
        let mut owner = self.as_mut().rust_mut();
        if let Some(state) = &mut owner.state {
            state.clear();
        }
        owner.dispose_empty();
    }
}
