/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
WorkbenchDialog {
    id: dialog
    required property var backend
    required property var runState
    signal configurationApplied()
    title: "PRBS-RX / Recovered Data"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(650,parent ? parent.width - 40 : 650)
    modal:true; standardButtons:Dialog.Close
    contentItem: ColumnLayout {
        spacing:14
        Label { text:"Independent Source Comparison"; color:theme.accent; font.bold:true }
        Label { text:"Compare decoded payload with the accepted original source. PRBS15 uses its independent expected sequence; files use exact original bytes and explicit fill. This never feeds the receiver or reseeds PRBS-TX."; color:theme.text; Layout.fillWidth:true; wrapMode:Text.WordWrap }
        Button { objectName:"verificationApply"; text:dialog.runState.verified ? "Comparison Enabled" : "Enable Comparison"; enabled:!dialog.runState.verified; onClicked: { backend.verify();dialog.configurationApplied() } }
        Label { text:dialog.runState.result ? "Status: "+dialog.runState.result.status+"\nCompared bits: "+dialog.runState.result.bits_tested+"\nBit errors: "+dialog.runState.result.bit_errors+"\nBER: "+(dialog.runState.result.ber === null ? "Unavailable (no compared bits)" : dialog.runState.result.ber)+"\nRecovered payloads: "+dialog.runState.result.recovered_payloads+"\nUnrecovered generated payloads: "+dialog.runState.result.unrecovered : "No completed or partial result yet."; color:theme.text; Layout.fillWidth:true; wrapMode:Text.WordWrap }
        Label { text:"Noisy decode failures are simulation outcomes. Complete, cancelled, budget-exceeded and failed runs remain distinguishable."; color:theme.muted; Layout.fillWidth:true; wrapMode:Text.WordWrap }
    }
}
