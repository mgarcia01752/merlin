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
    property alias signalIndex: selection.signalIndex
    property alias viewIndex: selection.viewIndex
    property alias plcPartIndex: selection.plcPartIndex
    property alias selectionControls: selection
    property var observation: ({points: [], profiles: []})
    readonly property string signalName: ["Data Profile", "NCP", "PLC", "Continuous Pilots", "Scattered Pilots"][signalIndex] || "Unavailable"
    function refresh() { if (!backend || !selection) { return }; observation = JSON.parse(backend.qam_selected(JSON.stringify(selection.query()))) }
    onOpened: refresh()
    title: "QAM Analyzer · " + signalName
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(1080, parent ? parent.width - 40 : 1080)
    height: Math.min(690, parent ? parent.height - 40 : 690)
    modal: true
    standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 12
        ReceiverCellControls {
            id:selection;theme:dialog.theme;metadata:dialog.observation
            Layout.fillWidth:true
            onSelectionChanged:dialog.refresh()
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
                        Canvas {
                            id: scatter
                            Layout.fillWidth:true; Layout.fillHeight:true
                            property bool raw: modelData.indexOf("Raw") === 0
                            Connections { target: dialog; function onObservationChanged() { scatter.requestPaint() } }
                            Connections { target: dialog.theme; function onDarkChanged() { scatter.requestPaint() } }
                            onWidthChanged: requestPaint()
                            onHeightChanged: requestPaint()
                            onPaint: {
                                const ctx=getContext("2d");ctx.reset()
                                if (!dialog.observation.available) { return }
                                const edge=Math.min(width,height)
                                const left=(width-edge)/2
                                const top=(height-edge)/2
                                ctx.strokeStyle=dialog.theme.line;ctx.lineWidth=1
                                ctx.beginPath();ctx.moveTo(left+edge/2,top);ctx.lineTo(left+edge/2,top+edge);ctx.moveTo(left,top+edge/2);ctx.lineTo(left+edge,top+edge/2);ctx.stroke()
                                ctx.fillStyle=dialog.theme.muted;ctx.font="11px sans-serif"
                                ctx.fillText("I",left+edge-10,top+edge/2-6);ctx.fillText("Q",left+edge/2+6,top+12)
                                ctx.fillStyle=dialog.theme.accent
                                for (const point of dialog.observation.points || []) {
                                    ctx.fillRect(left+(raw ? point.raw_x : point.eq_x)*edge-1,top+(raw ? point.raw_y : point.eq_y)*edge-1,2,2)
                                }
                            }
                        }
                        Label { objectName: "receiverQamUnavailable"; visible: !dialog.observation.available; text: dialog.observation.reason || "No retained receiver results"; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap; horizontalAlignment: Text.AlignHCenter }
                        Item { visible: !dialog.observation.available; Layout.fillHeight: true }
                        Label { text: dialog.observation.available ? dialog.observation.points.length+" actual matched cells · I / Q" : "No I/Q points retained"; color: dialog.theme.muted; Layout.alignment: Qt.AlignHCenter; font.pixelSize: 11 }
                    }
                }
            }
        }
        Label { objectName: "receiverQamDetail"; text: dialog.observation.detail || ""; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap; font.pixelSize: 12 }
        Label { text: "Raw: " + (dialog.observation.raw_tap || "—") + "    Equalized: " + (dialog.observation.equalized_tap || "—"); color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap; font.pixelSize: 11 }
    }
}
