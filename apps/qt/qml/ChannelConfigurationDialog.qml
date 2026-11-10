/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
WorkbenchDialog {
    id: dialog
    required property var configuration
    signal rxmerRequested()
    signal responseRequested()
    title: "Channel Simulator · Configuration"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(650, parent ? parent.width - 40 : 650)
    modal: true; standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 16
        Label { text: "RxMER"; color: theme.accent; font.bold: true }
        Label { text: dialog.configuration.rxmer ? dialog.configuration.rxmer.reason : "Configure RxMER"; color: theme.text; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        Button { text: "Configure RxMER…"; onClicked: { dialog.close(); dialog.rxmerRequested() } }
        Label { text: "Channel response H"; color: theme.accent; font.bold: true }
        Label { text: dialog.configuration.flat ? "Flat / unity · H = 1 + 0i per subcarrier · 0 dB / 0°" : "Imported channel-response capture"; color: theme.text; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        Label { text: dialog.configuration.response ? dialog.configuration.response.reason : "Apply DPD to define the grid"; color: theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        Button { text: "Configure channel response…"; onClicked: { dialog.close(); dialog.responseRequested() } }
        Label { text: "Green indicates valid configuration. Full-loop UI run dispatch remains pending; the TX Spectrum Analyzer runs before these impairments."; color: theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap }
    }
}
