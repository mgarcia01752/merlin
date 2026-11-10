/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
use cxx_qt_build::{CxxQtBuilder, QmlModule};

/// Embeds the workbench QML and links native Qt Quick presentation modules.
fn main() {
    let gtk_flags = std::process::Command::new("pkg-config")
        .args(["--cflags", "--libs", "gtk+-3.0"])
        .output()
        .expect("Install GTK development prerequisites with ./install.sh --development");
    assert!(
        gtk_flags.status.success(),
        "GTK development prerequisites missing"
    );
    let gtk_flags = String::from_utf8(gtk_flags.stdout).expect("GTK flags must be UTF-8");
    for flag in gtk_flags.split_whitespace() {
        if let Some(library) = flag.strip_prefix("-l") {
            println!("cargo:rustc-link-lib={library}");
        } else if let Some(directory) = flag.strip_prefix("-L") {
            println!("cargo:rustc-link-search=native={directory}");
        }
    }
    CxxQtBuilder::new()
        .file("src/native_file_picker.rs")
        .qrc("branding.qrc")
        .cc_builder(|builder| {
            builder.include("src");
            for flag in gtk_flags
                .split_whitespace()
                .filter(|flag| flag.starts_with("-I") || *flag == "-pthread")
            {
                builder.flag(flag);
            }
        })
        .qt_module("Quick")
        .qt_module("QuickControls2")
        .qml_module(QmlModule {
            uri: "Merlin.Workbench",
            rust_files: &[
                "src/branding.rs",
                "src/about_info.rs",
                "src/ofdm_settings.rs",
                "src/profile_input.rs",
                "src/source_input.rs",
                "src/measurement_input.rs",
                "src/receiver_instruments.rs",
                "src/closed_loop.rs",
                "src/workbench_file.rs",
                "src/transmitter_analyzer.rs",
            ],
            qml_files: &[
                "qml/Main.qml",
                "qml/Workbench.qml",
                "qml/WorkbenchDialog.qml",
                "qml/WorkbenchMenuBar.qml",
                "qml/SettingsDialog.qml",
                "qml/AboutDialog.qml",
                "qml/LicenseDialog.qml",
                "qml/OfdmSettingsDialog.qml",
                "qml/ProfileInputDialog.qml",
                "qml/SourceInputDialog.qml",
                "qml/SpectrumAnalyzerDialog.qml",
                "qml/SpectrumPlot.qml",
                "qml/SymbolTimingPanel.qml",
                "qml/MeasurementPlot.qml",
                "qml/MeasurementCaptureTree.qml",
                "qml/MeasurementViewer.qml",
                "qml/MeasurementInputDialog.qml",
                "qml/RxmerInputDialog.qml",
                "qml/ChannelResponseDialog.qml",
                "qml/ChannelConfigurationDialog.qml",
                "qml/ReceiverWorkbenchDialog.qml",
                "qml/ReceiverChip.qml",
                "qml/ReceiverConfigurationDialog.qml",
                "qml/VerificationDialog.qml",
                "qml/WorkbenchFileFeedback.qml",
                "qml/ReceiverQamDialog.qml",
                "qml/ReceiverCellControls.qml",
                "qml/ReceiverResponseDialog.qml",
                "qml/ReceiverSlicerDialog.qml",
                "qml/ReceiverFecDialog.qml",
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
