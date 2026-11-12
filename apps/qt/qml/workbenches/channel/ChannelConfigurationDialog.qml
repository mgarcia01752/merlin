/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
WorkbenchDialog {
    id: dialog
    required property var configuration
    signal rxmerRequested()
    signal responseRequested()
    signal lossRequested()
    signal filterRequested()
    title: "Channel Model · Configuration"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(650, parent ? parent.width - 40 : 650)
    modal: true; standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 16
        WorkbenchActionButton {theme:dialog.theme;objectName:"channelConfigureFilter";text:"Waveform Channel";onClicked:dialog.filterRequested()}
        Label { text: {
            const loss=dialog.configuration.additional_loss || {mode:"off"}
            if (loss.mode==="manual") {return "Additional Tilt: "+loss.tilt_db+" dB"}
            if (loss.mode==="cable") {return "Additional Cable: Belden 1694A · "+loss.length+" "+(loss.unit==="metres" ? "m" : "ft")}
            return "Additional Loss: Off"
        }
        color:dialog.theme.text }
        WorkbenchActionButton { theme: dialog.theme; objectName:"channelConfigureLoss";text:"Additional Tilt / Cable Loss…";onClicked:dialog.lossRequested() }
        Label { text: "RxMER"; color: theme.accent; font.bold: true }
        Label { text: dialog.configuration.rxmer ? dialog.configuration.rxmer.reason : "Configure RxMER"; color: theme.text; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        WorkbenchActionButton { theme: dialog.theme; text: "Configure RxMER…"; onClicked: { dialog.close(); dialog.rxmerRequested() } }
        Label { text: "Channel response H"; color: theme.accent; font.bold: true }
        Label { text: dialog.configuration.flat ? "Flat / unity · H = 1 + 0i per subcarrier · 0 dB / 0°" : "Imported channel-response capture"; color: theme.text; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        Label { text: dialog.configuration.response ? dialog.configuration.response.reason : "Apply DPD to define the grid"; color: theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        WorkbenchActionButton { theme: dialog.theme; text: "Configure channel response…"; onClicked: { dialog.close(); dialog.responseRequested() } }
    }
}
