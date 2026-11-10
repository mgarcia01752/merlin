/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bundled application identity and license documents; no network or file reads.
use cxx_qt_lib::QString;
use serde_json::json;
#[cxx_qt::bridge]
pub mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type AboutInfo = super::AboutInfoRust;
        /// Reports the version compiled from this package's reviewed Cargo metadata.
        /// # Returns
        /// The application version; no independently maintained UI constant.
        #[qinvokable]
        fn version(self: &AboutInfo) -> QString;
        /// Returns one whitelisted bundled license or attribution document.
        /// # Arguments
        /// * `key` - gpl, apache, notice, or pnm identifiers.
        /// # Returns
        /// Bounded title/text JSON; unknown keys return an explicit diagnostic.
        #[qinvokable]
        fn license_document(self: &AboutInfo, key: &QString) -> QString;
    }
}
/// Stateless About-dialog adapter, independent of scientific state.
#[derive(Default)]
pub struct AboutInfoRust;
impl bridge::AboutInfo {
    /// Reports the build's package version without I/O.
    /// # Returns
    /// Same version as the compiled Qt package.
    pub fn version(&self) -> QString {
        QString::from(env!("CARGO_PKG_VERSION"))
    }
    /// Adapts a fixed bundled document for offline display.
    /// # Arguments
    /// * `key` - Whitelisted document selector, never a filesystem path.
    /// # Returns
    /// Document title/body, or an unavailable-document message.
    pub fn license_document(&self, key: &QString) -> QString {
        let (title, text) = match key.to_string().as_str() {
            "gpl" => ("GNU General Public License v3", include_str!("../LICENSE")),
            "apache" => ("Apache License 2.0", include_str!("../../../LICENSE")),
            "notice" => ("MERLIN Notices", include_str!("../NOTICE")),
            "pnm" => (
                "PNM Decoder Notices",
                include_str!("../third-party/pnm-NOTICE"),
            ),
            _ => ("Document Unavailable", "Unknown bundled license document."),
        };
        QString::from(json!({"title":title,"text":text}).to_string())
    }
}
