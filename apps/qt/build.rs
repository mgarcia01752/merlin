/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
use cxx_qt_build::{CxxQtBuilder, QmlModule};

/// Embeds the workbench QML and links native Qt Quick presentation modules.
fn main() {
    CxxQtBuilder::new()
        .qrc("branding.qrc")
        .cc_builder(|builder| {
            builder.include("src");
        })
        .qt_module("Quick")
        .qt_module("QuickControls2")
        .qml_module(QmlModule {
            uri: "Merlin.Workbench",
            rust_files: &["src/branding.rs", "src/ofdm_settings.rs"],
            qml_files: &[
                "qml/Main.qml",
                "qml/Workbench.qml",
                "qml/OfdmSettingsDialog.qml",
                "qml/SectionPreviewDialog.qml",
                "qml/WorkbenchBlock.qml",
                "qml/Connection.qml",
                "qml/Theme.qml",
                "qml/ThemePalette.qml",
            ],
            ..Default::default()
        })
        .build();
}
