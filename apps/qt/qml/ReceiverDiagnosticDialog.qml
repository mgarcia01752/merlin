/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Dialog {
    id: dialog
    required property var theme
    property var runState: ({})
    readonly property bool hasFec: title === "FEC Monitor" && !!runState.result
    property string testPoint: ""
    property string description: ""
    function showInstrument(name, point, detail) { title = name; testPoint = point; description = detail; open() }
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(650, parent ? parent.width - 40 : 650)
    modal: true; standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 18
        Label { text: dialog.testPoint; color: dialog.theme.accent; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        Label { text: dialog.description; color: dialog.theme.text; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        Label { visible: dialog.hasFec; text: dialog.hasFec ? "Status: "+dialog.runState.result.status+"\nRecovered payloads: "+dialog.runState.result.recovered_payloads+"\nLDPC failures: "+dialog.runState.result.ldpc_failures+"\nBCH failures: "+dialog.runState.result.bch_failures+"\nMean LDPC iterations: "+dialog.runState.result.mean_iterations+"\nAccepted NCP fields: "+dialog.runState.result.ncp_fields+"\nPLC frames: "+dialog.runState.result.plc_frames : ""; color:theme.text; Layout.fillWidth:true; wrapMode:Text.WordWrap }
        Label { visible: !dialog.hasFec; text: "This instrument's result connection is pending; opening it does not run or modify the simulation."; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap }
    }
}
