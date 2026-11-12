/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
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
    Component.onCompleted: { branding.update_icon(theme.dark); if (Qt.application.arguments.indexOf("--restore-update")>=0) Qt.callLater(function(){workbench.restoreUpdate()}); else Qt.callLater(function(){workbench.beginOpen(JSON.parse(workbenchFile.startup()))}); if (Qt.application.arguments.indexOf("--smoke-test")>=0) Qt.callLater(Qt.quit); else if (Qt.application.arguments.indexOf("--check-updates")>=0) Qt.callLater(function(){updates.showUpdates();updates.refresh()}) }
    ThemePalette { targetControl: window; theme: theme }
    color: theme.background
    onClosing: function(close) {
        if (updates.state.installing) {
            close.accepted = false
            updates.showUpdates()
        }
    }
    menuBar: Item {
        implicitHeight: Math.max(topMenu.implicitHeight, 32)
        Rectangle { anchors.fill: parent; color: theme.panel }
        RowLayout {
            anchors.fill: parent; spacing: 12
            WorkbenchMenuBar {
                id: topMenu
                Layout.fillWidth: true
                theme: theme
                fileActionsEnabled: !workbench.runState.busy && !workbench.loadingProject && !updates.state.installing
                onNewRequested: workbench.requestNewProject()
                onOpenRequested: workbench.openProject()
                onSaveRequested: function(saveAs) { workbench.saveProject(saveAs) }
                onGlobalSettingsRequested: { if (!updates.state.installing) workbench.settingsPopup.open() }
                onAboutRequested: workbench.aboutPopup.open()
                onFaqRequested: workbench.openHelp("")
                onTerminologyRequested: function(target) { Qt.openUrlExternally(target) }
            }
            UpdateNotification {
                id: updates
                Layout.preferredWidth: 80; Layout.preferredHeight: 28
                Layout.rightMargin: 12
                theme: theme; backend: updateInfo
                canUpdate: !workbench.runState.busy && !workbench.loadingProject
                prepareUpdate: function() { return workbench.saveUpdateRecovery() }
                startupCheck: Qt.application.arguments.indexOf("--check-updates")<0
            }
        }
    }
    Workbench {
        id: workbench
        enabled: !updates.state.installing
        anchors.fill: parent
        theme: theme
        settingsBackend: ofdmSettings
        profileBackend: profileInput
        sourceBackend: sourceInput
        transmitterBackend: transmitterAnalyzer
        measurementBackend: measurementInputs
        aboutBackend: aboutInfo
        receiverBackend: receiverInstruments
        runBackend: closedLoop
        fileBackend: workbenchFile
        onUpdatesRequested: { updates.showUpdates(); updates.refresh() }
        onExternalLinkRequested: function(target) { Qt.openUrlExternally(target) }
    }
}
