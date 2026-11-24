/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
use cxx_qt_build::{CxxQtBuilder, QmlModule};
#[path = "build_support/branding.rs"]
mod branding;

/// Embeds the workbench QML and links native Qt Quick presentation modules.
fn main() {
    branding::embed();
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
        .qml_module({
            let module = QmlModule {
                uri: "Merlin.Workbench",
                rust_files: &[
                    "src/bridge/branding.rs",
                    "src/bridge/about_info.rs",
                    "src/bridge/presentation_retention.rs",
                    "src/bridge/update_info.rs",
                    "src/bridge/ofdm_settings.rs",
                    "src/bridge/profile_input.rs",
                    "src/bridge/source_input.rs",
                    "src/bridge/measurement_input.rs",
                    "src/bridge/receiver_instruments.rs",
                    "src/bridge/closed_loop.rs",
                    "src/bridge/direction_shell.rs",
                    "src/bridge/workbench_file.rs",
                    "src/bridge/transmitter_analyzer.rs",
                    "src/bridge/upstream_session.rs",
                ],
                qml_files: &[
                    "qml/Main.qml",
                    "qml/upstream/BurstProfilesDialog.qml",
                    "qml/upstream/ChannelDialog.qml",
                    "qml/upstream/GrantsDialog.qml",
                    "qml/upstream/PreEqualizationDialog.qml",
                    "qml/upstream/ReceiverDialog.qml",
                    "qml/upstream/RxmerDialog.qml",
                    "qml/upstream/SourceDialog.qml",
                    "qml/upstream/TransmitterDialog.qml",
                    "qml/upstream/UcdConfigurationDialog.qml",
                    "qml/upstream/UpstreamConnection.qml",
                    "qml/upstream/UpstreamDetailDialog.qml",
                    "qml/upstream/UpstreamCaptureDialog.qml",
                    "qml/upstream/UpstreamQamDialog.qml",
                    "qml/upstream/UpstreamSpectrumDialog.qml",
                    "qml/upstream/UpstreamResponseDialog.qml",
                    "qml/shared/instruments/QamPlot.qml",
                    "qml/upstream/ResourceGridDialog.qml",
                    "qml/upstream/ResourceGridView.qml",
                    "qml/upstream/UpstreamConfigurationDraftDialog.qml",
                    "qml/upstream/UpstreamWorkbench.qml",
                    "qml/upstream/UpstreamVerificationDialog.qml",
                    "qml/shell/directions/PersistentDirectionTabs.qml",
                    "qml/shell/directions/DirectionShellHost.qml",
                    "qml/shell/directions/DirectionRunControls.qml",
                    "qml/validation/SessionPresentationProbe.qml",
                    "qml/validation/DirectionShellProbe.qml",
                    "qml/validation/UpstreamConfigurationProbe.qml",
                    "qml/validation/UpstreamExecutionProbe.qml",
                    "qml/validation/UpstreamInputsProbe.qml",
                    "qml/validation/UpstreamResourceGridProbe.qml",
                    "qml/validation/UpstreamInstrumentsProbe.qml",
                    "qml/validation/DownstreamSharedInstrumentsProbe.qml",
                    "qml/shared/presentation/PresentationOwner.qml",
                    "qml/shared/presentation/LogicalFootprint.js",
                    "qml/downstream/DownstreamWorkbench.qml",
                    "qml/shared/components/WorkbenchDialog.qml",
                    "qml/shared/components/WorkbenchActionButton.qml",
                    "qml/shell/WorkbenchEntryDialog.qml",
                    "qml/shared/components/ApplyCancelFooter.qml",
                    "qml/shell/WorkbenchMenuBar.qml",
                    "qml/shell/SettingsDialog.qml",
                    "qml/shell/UpdateNotification.qml",
                    "qml/shell/UpdateDialog.qml",
                    "qml/shared/about/AboutDialog.qml",
                    "qml/shared/about/FaqDialog.qml",
                    "qml/shared/components/ContextualStatus.qml",
                    "qml/shared/about/LicenseDialog.qml",
                    "qml/downstream/configuration/ofdm_settings/OfdmSettingsDialog.qml",
                    "qml/downstream/configuration/profile_input/ProfileInputDialog.qml",
                    "qml/downstream/configuration/profile_input/ProfilePlotDialog.qml",
                    "qml/downstream/configuration/profile_input/ProfileChart.qml",
                    "qml/downstream/configuration/profile_input/ProfileBandwidthDialog.qml",
                    "qml/downstream/configuration/source_input/SourceInputDialog.qml",
                    "qml/downstream/instruments/spectrum/SpectrumAnalyzerDialog.qml",
                    "qml/shared/instruments/SpectrumPlot.qml",
                    "qml/shared/components/ChartNavigation.qml",
                    "qml/downstream/instruments/spectrum/SymbolTimingPanel.qml",
                    "qml/downstream/instruments/spectrum/SymbolTimingPlot.qml",
                    "qml/downstream/instruments/spectrum/SymbolTimingDialog.qml",
                    "qml/shared/instruments/MeasurementPlot.qml",
                    "qml/downstream/measurements/RxmerDesignerDialog.qml",
                    "qml/downstream/measurements/RxmerPnmExportDialog.qml",
                    "qml/downstream/measurements/RxmerDesignScope.qml",
                    "qml/downstream/measurements/MeasurementCaptureTree.qml",
                    "qml/downstream/measurements/MeasurementPlaybackControls.qml",
                    "qml/downstream/measurements/MeasurementViewer.qml",
                    "qml/downstream/measurements/MeasurementInputDialog.qml",
                    "qml/downstream/measurements/RxmerInputDialog.qml",
                    "qml/downstream/measurements/ChannelResponseDialog.qml",
                    "qml/downstream/channel/ChannelConfigurationDialog.qml",
                    "qml/downstream/channel/ChannelModelCard.qml",
                    "qml/downstream/channel/ChannelResponsePreviewDialog.qml",
                    "qml/downstream/channel/ChannelFilterDialog.qml",
                    "qml/downstream/channel/ChannelWorkbenchDialog.qml",
                    "qml/downstream/channel/ChannelAmplifierDialog.qml",
                    "qml/downstream/channel/ChannelAttenuatorDialog.qml",
                    "qml/downstream/channel/ConditioningSymbol.qml",
                    "qml/downstream/instruments/qam/TapQamDialog.qml",
                    "qml/downstream/channel/ChannelInstrumentDialog.qml",
                    "qml/downstream/instruments/selection/InstrumentChoice.qml",
                    "qml/downstream/channel/ChannelLossDialog.qml",
                    "qml/downstream/receiver/ReceiverWorkbenchDialog.qml",
                    "qml/downstream/transmitter/TransmitterWorkbenchDialog.qml",
                    "qml/downstream/instruments/stages/TransmitterStageDialog.qml",
                    "qml/downstream/instruments/stages/TransmitterStagePlot.qml",
                    "qml/downstream/instruments/stages/TransmitterStageLegend.qml",
                    "qml/downstream/transmitter/TransmitterGraphCard.qml",
                    "qml/downstream/instruments/qam/TransmitterQamDialog.qml",
                    "qml/downstream/instruments/qam/TransmitterScatter.qml",
                    "qml/downstream/instruments/messages/TransmitterMessageDialog.qml",
                    "qml/shared/components/TlvTreeGrid.qml",
                    "qml/shared/components/ReceiverChip.qml",
                    "qml/shared/components/StageChip.qml",
                    "qml/downstream/receiver/ReceiverConfigurationDialog.qml",
                    "qml/downstream/configuration/WorkbenchConfigurationDialog.qml",
                    "qml/downstream/receiver/VerificationDialog.qml",
                    "qml/downstream/receiver/VerificationBarChart.qml",
                    "qml/shell/WorkbenchFileFeedback.qml",
                    "qml/shell/WorkbenchLoadingDialog.qml",
                    "qml/downstream/instruments/qam/ReceiverQamDialog.qml",
                    "qml/downstream/instruments/qam/ReceiverCellControls.qml",
                    "qml/downstream/instruments/response/ReceiverResponseDialog.qml",
                    "qml/downstream/instruments/slicer/ReceiverSlicerDialog.qml",
                    "qml/downstream/instruments/slicer/SoftDiagnosticPlot.qml",
                    "qml/downstream/instruments/fec/ReceiverFecDialog.qml",
                    "qml/downstream/instruments/fec/FecControlTable.qml",
                    "qml/downstream/instruments/fec/FecAllocationChart.qml",
                    "qml/downstream/instruments/fec/FecIterationChart.qml",
                    "qml/downstream/instruments/fec/FecCodewordTable.qml",
                    "qml/downstream/instruments/profile_performance/ProfilePerformanceDialog.qml",
                    "qml/downstream/configuration/SectionPreviewDialog.qml",
                    "qml/shared/components/WorkbenchBlock.qml",
                    "qml/shared/components/Connection.qml",
                    "qml/shared/theme/Theme.qml",
                    "qml/shared/theme/ThemePalette.qml",
                ],
                ..Default::default()
            };
            // qt-build-utils 0.7 writes cache outputs by basename inside a module.
            // Duplicate names silently overwrite one compiled unit before linking.
            let mut basenames = std::collections::HashSet::new();
            for path in module.qml_files {
                let basename = std::path::Path::new(path).file_name().unwrap();
                assert!(
                    basenames.insert(basename),
                    "Duplicate QML cache name: {path}"
                );
            }
            module
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
