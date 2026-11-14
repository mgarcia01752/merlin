/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
use cxx_qt_build::{CxxQtBuilder, QmlModule};

/// Embeds the workbench QML and links native Qt Quick presentation modules.
fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let gtk_flags = if target_os == "linux" {
        linux_gtk_flags()
    } else {
        String::new()
    };
    if target_os == "windows" {
        windows_resources();
        for library in ["ole32", "shell32", "uuid"] {
            println!("cargo:rustc-link-lib={library}");
        }
    }
    CxxQtBuilder::new()
        .file("src/platform/native_file_picker.rs")
        .file("src/platform/application_style.rs")
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
                "src/bridge/branding.rs",
                "src/bridge/about_info.rs",
                "src/bridge/update_info.rs",
                "src/bridge/ofdm_settings.rs",
                "src/bridge/profile_input.rs",
                "src/bridge/source_input.rs",
                "src/bridge/measurement_input.rs",
                "src/bridge/receiver_instruments.rs",
                "src/bridge/closed_loop.rs",
                "src/bridge/workbench_file.rs",
                "src/bridge/transmitter_analyzer.rs",
            ],
            qml_files: &[
                "qml/Main.qml",
                "qml/shell/Workbench.qml",
                "qml/components/WorkbenchDialog.qml",
                "qml/components/WorkbenchActionButton.qml",
                "qml/shell/WorkbenchEntryDialog.qml",
                "qml/components/ApplyCancelFooter.qml",
                "qml/shell/WorkbenchMenuBar.qml",
                "qml/shell/SettingsDialog.qml",
                "qml/shell/UpdateNotification.qml",
                "qml/shell/UpdateDialog.qml",
                "qml/about/AboutDialog.qml",
                "qml/about/FaqDialog.qml",
                "qml/components/ContextualStatus.qml",
                "qml/about/LicenseDialog.qml",
                "qml/inputs/ofdm_settings/OfdmSettingsDialog.qml",
                "qml/inputs/profile_input/ProfileInputDialog.qml",
                "qml/inputs/profile_input/ProfilePlotDialog.qml",
                "qml/inputs/profile_input/ProfileChart.qml",
                "qml/inputs/profile_input/ProfileBandwidthDialog.qml",
                "qml/inputs/source_input/SourceInputDialog.qml",
                "qml/instruments/spectrum/SpectrumAnalyzerDialog.qml",
                "qml/instruments/spectrum/SpectrumPlot.qml",
                "qml/components/ChartNavigation.qml",
                "qml/instruments/spectrum/SymbolTimingPanel.qml",
                "qml/instruments/spectrum/SymbolTimingPlot.qml",
                "qml/instruments/spectrum/SymbolTimingDialog.qml",
                "qml/inputs/measurement_input/MeasurementPlot.qml",
                "qml/inputs/measurement_input/RxmerDesignerDialog.qml",
                "qml/inputs/measurement_input/RxmerDesignScope.qml",
                "qml/inputs/measurement_input/MeasurementCaptureTree.qml",
                "qml/inputs/measurement_input/MeasurementPlaybackControls.qml",
                "qml/inputs/measurement_input/MeasurementViewer.qml",
                "qml/inputs/measurement_input/MeasurementInputDialog.qml",
                "qml/inputs/measurement_input/RxmerInputDialog.qml",
                "qml/inputs/measurement_input/ChannelResponseDialog.qml",
                "qml/workbenches/channel/ChannelConfigurationDialog.qml",
                "qml/workbenches/channel/ChannelModelCard.qml",
                "qml/workbenches/channel/ChannelResponsePreviewDialog.qml",
                "qml/workbenches/channel/ChannelFilterDialog.qml",
                "qml/workbenches/channel/ChannelWorkbenchDialog.qml",
                "qml/workbenches/channel/ChannelAmplifierDialog.qml",
                "qml/workbenches/channel/ChannelAttenuatorDialog.qml",
                "qml/workbenches/channel/ConditioningSymbol.qml",
                "qml/instruments/qam/TapQamDialog.qml",
                "qml/workbenches/channel/ChannelInstrumentDialog.qml",
                "qml/instruments/selection/InstrumentChoice.qml",
                "qml/workbenches/channel/ChannelLossDialog.qml",
                "qml/workbenches/receiver/ReceiverWorkbenchDialog.qml",
                "qml/workbenches/transmitter/TransmitterWorkbenchDialog.qml",
                "qml/instruments/stages/TransmitterStageDialog.qml",
                "qml/instruments/stages/TransmitterStagePlot.qml",
                "qml/instruments/stages/TransmitterStageLegend.qml",
                "qml/workbenches/transmitter/TransmitterGraphCard.qml",
                "qml/instruments/qam/TransmitterQamDialog.qml",
                "qml/instruments/qam/TransmitterScatter.qml",
                "qml/instruments/messages/TransmitterMessageDialog.qml",
                "qml/components/TlvTreeGrid.qml",
                "qml/components/ReceiverChip.qml",
                "qml/components/StageChip.qml",
                "qml/workbenches/receiver/ReceiverConfigurationDialog.qml",
                "qml/workbenches/workbench/WorkbenchConfigurationDialog.qml",
                "qml/workbenches/verification/VerificationDialog.qml",
                "qml/workbenches/verification/VerificationBarChart.qml",
                "qml/shell/WorkbenchFileFeedback.qml",
                "qml/shell/WorkbenchLoadingDialog.qml",
                "qml/instruments/qam/ReceiverQamDialog.qml",
                "qml/instruments/qam/ReceiverCellControls.qml",
                "qml/instruments/response/ReceiverResponseDialog.qml",
                "qml/instruments/slicer/ReceiverSlicerDialog.qml",
                "qml/instruments/fec/ReceiverFecDialog.qml",
                "qml/instruments/fec/FecControlTable.qml",
                "qml/instruments/fec/FecAllocationChart.qml",
                "qml/instruments/fec/FecIterationChart.qml",
                "qml/instruments/fec/FecCodewordTable.qml",
                "qml/instruments/profile_performance/ProfilePerformanceDialog.qml",
                "qml/shell/SectionPreviewDialog.qml",
                "qml/components/WorkbenchBlock.qml",
                "qml/components/Connection.qml",
                "qml/theme/Theme.qml",
                "qml/theme/ThemePalette.qml",
            ],
            ..Default::default()
        })
        .build();
}

/// Queries and links GTK only for the native Linux presentation helper.
fn linux_gtk_flags() -> String {
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
    gtk_flags
}

/// Compiles the embedded Windows icon/version resource using the native SDK.
/// Fails the build when rc.exe is missing or rejects the controlled resource.
fn windows_resources() {
    use std::{path::PathBuf, process::Command};
    let manifest = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let artwork = manifest.join("../../packaging/windows/merlin.ico");
    let output = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let resource = output.join("merlin.rc");
    let version = std::env::var("CARGO_PKG_VERSION").unwrap();
    let numeric = version.replace('.', ",") + ",0";
    let icon = artwork.display().to_string().replace('\\', "/");
    std::fs::write(
        &resource,
        format!(
            r#"
#include <windows.h>
1 ICON "{icon}"
1 VERSIONINFO
FILEVERSION {numeric}
PRODUCTVERSION {numeric}
FILEOS VOS_NT_WINDOWS32
FILETYPE VFT_APP
BEGIN
  BLOCK "StringFileInfo"
  BEGIN
    BLOCK "040904B0"
    BEGIN
      VALUE "CompanyName", "MERLIN contributors\0"
      VALUE "FileDescription", "MERLIN DOCSIS OFDM Simulation Toolkit\0"
      VALUE "FileVersion", "{version}\0"
      VALUE "ProductName", "MERLIN\0"
      VALUE "ProductVersion", "{version}\0"
      VALUE "OriginalFilename", "merlin-qt.exe\0"
    END
  END
  BLOCK "VarFileInfo"
  BEGIN
    VALUE "Translation", 0x0409, 1200
  END
END
"#
        ),
    )
    .expect("Write Windows executable resources");
    let compiled = output.join("merlin.res");
    let status = Command::new("rc.exe")
        .arg("/nologo")
        .arg("/fo")
        .arg(&compiled)
        .arg(&resource)
        .status()
        .expect("Windows SDK rc.exe must be available in the MSVC environment");
    assert!(status.success(), "Windows resource compilation failed");
    println!("cargo:rerun-if-changed={}", artwork.display());
    println!("cargo:rustc-link-arg-bin=merlin-qt={}", compiled.display());
}
