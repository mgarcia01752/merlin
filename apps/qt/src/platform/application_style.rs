/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Explicit Qt-owned style selection independent of process environment hints.
#[cxx::bridge]
mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("platform/application_style.h");
        fn configureQuickControlsStyle() -> QString;
    }
}

/// Configures palette-aware Fusion controls through Qt's startup API.
///
/// # Returns
/// Qt's selected style name. Call before QGuiApplication and QML controls exist;
/// no environment variable or native file-picker style is changed.
pub(crate) fn configure() -> String {
    bridge::configureQuickControlsStyle().to_string()
}
