/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import Merlin.Workbench
import "shell"
import "theme"

ApplicationWindow {
    id: window
    width: 1240
    height: 780
    minimumWidth: 960
    minimumHeight: 640
    visible: true
    title: "MERLIN — DOCSIS OFDM Simulation Toolkit"
    Branding { id: branding }
    AboutInfo { id: aboutInfo }
    UpdateInfo { id: updateInfo }
    OfdmSettings { id: ofdmSettings }
    ProfileInput { id: profileInput }
    SourceInput { id: sourceInput }
    MeasurementInputs { id: measurementInputs }
    ReceiverInstruments { id: receiverInstruments }
    ClosedLoop { id: closedLoop }
    WorkbenchFile { id: workbenchFile }
    TransmitterAnalyzer { id: transmitterAnalyzer }
    Theme { id: theme; onDarkChanged: branding.update_icon(dark) }
    Component.onCompleted: { branding.update_icon(theme.dark); if (Qt.application.arguments.indexOf("--smoke-test")>=0) Qt.callLater(Qt.quit); else if (Qt.application.arguments.indexOf("--check-updates")>=0) Qt.callLater(function(){updates.showUpdates();updates.refresh()}) }
    ThemePalette { targetControl: window; theme: theme }
    color: theme.background
    menuBar: WorkbenchMenuBar {
        theme: theme
        fileActionsEnabled: !workbench.runState.busy
        onOpenRequested: workbench.openProject()
        onSaveRequested: function(saveAs) { workbench.saveProject(saveAs) }
        onGlobalSettingsRequested: workbench.settingsPopup.open()
        onAboutRequested: workbench.aboutPopup.open()
        onUpdatesRequested: updates.showUpdates()
        onTerminologyRequested: function(target) { Qt.openUrlExternally(target) }
    }
    UpdateNotification { id:updates; z:20;anchors.top:parent.top;anchors.right:parent.right;anchors.margins:16;width:180;height:36;theme:theme;backend:updateInfo;startupCheck:Qt.application.arguments.indexOf("--check-updates")<0;onReleaseRequested:function(target){Qt.openUrlExternally(target)} }
    Workbench { id: workbench; anchors.fill: parent; theme: theme; settingsBackend: ofdmSettings; profileBackend: profileInput; sourceBackend: sourceInput; transmitterBackend: transmitterAnalyzer; measurementBackend: measurementInputs; aboutBackend: aboutInfo; receiverBackend: receiverInstruments; runBackend: closedLoop; fileBackend: workbenchFile }
}
