/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import Merlin.Workbench

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
    OfdmSettings { id: ofdmSettings }
    ProfileInput { id: profileInput }
    SourceInput { id: sourceInput }
    MeasurementInputs { id: measurementInputs }
    ReceiverInstruments { id: receiverInstruments }
    TransmitterAnalyzer { id: transmitterAnalyzer }
    Theme { id: theme; onDarkChanged: branding.update_icon(dark) }
    Component.onCompleted: branding.update_icon(theme.dark)
    ThemePalette { targetControl: window; theme: theme }
    color: theme.background
    menuBar: WorkbenchMenuBar {
        theme: theme
        onGlobalSettingsRequested: workbench.settingsPopup.open()
        onAboutRequested: workbench.aboutPopup.open()
    }
    Workbench { id: workbench; anchors.fill: parent; theme: theme; settingsBackend: ofdmSettings; profileBackend: profileInput; sourceBackend: sourceInput; transmitterBackend: transmitterAnalyzer; measurementBackend: measurementInputs; aboutBackend: aboutInfo; receiverBackend: receiverInstruments }
}
