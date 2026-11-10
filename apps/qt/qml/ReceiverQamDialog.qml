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
    required property var backend
    property int signalIndex: 0
    property int viewIndex: 1
    property int plcPartIndex: 0
    property var observation: ({points: [], profiles: []})
    readonly property string signalName: ["Data Profile", "NCP", "PLC"][signalIndex] || "Unavailable"
    function refresh() { observation = JSON.parse(backend.qam(signalIndex, viewIndex, plcPartIndex)) }
    onSignalIndexChanged: refresh()
    onViewIndexChanged: refresh()
    onPlcPartIndexChanged: refresh()
    onOpened: refresh()
    title: "QAM Analyzer · " + signalName
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(1080, parent ? parent.width - 40 : 1080)
    height: Math.min(690, parent ? parent.height - 40 : 690)
    modal: true
    standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 12
        RowLayout {
            Label { text: "Signal"; color: dialog.theme.text }
            ComboBox { objectName: "receiverQamSignal"; model: ["Data Profile", "NCP", "PLC"]; currentIndex: dialog.signalIndex; Layout.preferredWidth: 170; onActivated: function(index) { dialog.signalIndex = index } }
            Label { text: "QAM View"; color: dialog.theme.text }
            ComboBox { objectName: "receiverQamView"; model: ["Raw", "Equalized", "Paired"]; currentIndex: dialog.viewIndex; Layout.preferredWidth: 155; onActivated: function(index) { dialog.viewIndex = index } }
            Label { visible: dialog.signalIndex === 2; text: "PLC Part"; color: dialog.theme.text }
            ComboBox { objectName: "receiverPlcPart"; visible: dialog.signalIndex === 2; model: ["Data", "Preamble"]; currentIndex: dialog.plcPartIndex; onActivated: function(index) { dialog.plcPartIndex = index } }
            Item { Layout.fillWidth: true }
        }
        RowLayout {
            Label { visible: dialog.signalIndex === 0; text: "Received Profile"; color: dialog.theme.muted }
            ComboBox { objectName: "receiverProfile"; visible: dialog.signalIndex === 0; model: dialog.observation.profiles || []; displayText: "No received profile"; enabled: false; Layout.preferredWidth: 190 }
            Label { text: "Carrier / Symbol Window"; color: dialog.theme.muted }
            Label { text: "Available with retained receiver observations"; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        }
        Label { text: "Received I/Q before hard decisions · supplied H and estimated H-hat remain distinct"; color: dialog.theme.accent; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        RowLayout {
            Layout.fillWidth: true; Layout.fillHeight: true; spacing: 12
            Repeater {
                model: dialog.viewIndex === 2 ? ["Raw · RX FFT", "Equalized · Before Slicing"] : [dialog.viewIndex === 0 ? "Raw · RX FFT" : "Equalized · Before Slicing"]
                Rectangle {
                    required property string modelData
                    Layout.fillWidth: true; Layout.fillHeight: true
                    radius: 8; color: dialog.theme.background; border.color: dialog.theme.line
                    ColumnLayout {
                        anchors.fill: parent; anchors.margins: 18
                        Label { text: modelData; color: dialog.theme.text; font.bold: true }
                        Item { Layout.fillHeight: true }
                        Label { objectName: "receiverQamUnavailable"; text: dialog.observation.reason || "No retained receiver results"; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap; horizontalAlignment: Text.AlignHCenter }
                        Item { Layout.fillHeight: true }
                        Label { text: "No I/Q points retained"; color: dialog.theme.muted; Layout.alignment: Qt.AlignHCenter; font.pixelSize: 11 }
                    }
                }
            }
        }
        Label { objectName: "receiverQamDetail"; text: dialog.observation.detail || ""; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap; font.pixelSize: 12 }
        Label { text: "Raw: " + (dialog.observation.raw_tap || "—") + "    Equalized: " + (dialog.observation.equalized_tap || "—"); color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap; font.pixelSize: 11 }
    }
}
