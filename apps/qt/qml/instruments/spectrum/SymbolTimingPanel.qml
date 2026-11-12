/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Rectangle {
    id: panel
    required property var theme
    required property var backend
    required property bool hasResult
    required property string resultToken
    property alias timingPopup:expanded
    property var timing: ({points: [], ticks: []})
    property real velocityFactor: cable.currentIndex === 0 ? 0.82 : cable.currentIndex === 1 ? 0.87 : (customVelocity.text.length ? Number(customVelocity.text) : NaN)
    function refresh() {
        timing = backend && typeof backend.timing === "function" ? JSON.parse(backend.timing(velocityFactor)) : ({points: [], ticks: []})

    }
    onHasResultChanged: refresh()
    onResultTokenChanged: refresh()
    onVelocityFactorChanged: refresh()
    Component.onCompleted: refresh()
    color: theme.panel; border.color: theme.line; radius: 6
    implicitHeight: cable.currentIndex === 2 ? 240 : 220
    Accessible.name: "Symbol timing and ideal cable echo budget"
    SymbolTimingDialog {id:expanded;theme:panel.theme;timing:panel.timing}
    RowLayout {
        anchors.fill: parent; anchors.margins: 10; spacing: 18
        ColumnLayout {
            Layout.fillWidth: true; Layout.fillHeight: true; spacing: 5
            Label { text: "Symbol Timing · Two Symbols"; color: theme.text; font.bold: true; font.pixelSize: 13 }
            Label { text: panel.timing.source ? panel.timing.source+" · "+panel.timing.overview : "Apply an OFDM configuration to display timing."; color: theme.muted; font.pixelSize: 11; Layout.fillWidth: true; elide: Text.ElideRight }
            Item {
                Layout.fillWidth:true;Layout.fillHeight:true
                SymbolTimingPlot {
                    id:windowPlot;objectName:"saTimingPlot"
                    anchors.fill:parent;theme:panel.theme
                    graph:(panel.timing.window || {}).overview || ({})
                    miniature:true
                }
                Rectangle {anchors.fill:parent;color:"transparent";border.color:panel.theme.accent;border.width:1;visible:expand.activeFocus}
                MouseArea {
                    id:expand;objectName:"expandSymbolTiming"
                    anchors.fill:parent;enabled:!!panel.timing.window
                    hoverEnabled:true;cursorShape:Qt.PointingHandCursor
                    activeFocusOnTab:true
                    Accessible.role:Accessible.Button
                    Accessible.name:"Expand Symbol Timing"
                    onClicked:{forceActiveFocus();expanded.open()}
                    Keys.onSpacePressed:expanded.open()
                    Keys.onReturnPressed:expanded.open()
                    ToolTip.visible:containsMouse || activeFocus
                    ToolTip.text:"Open full symbol timing and overlap"
                }
            }
            RowLayout {
                Layout.fillWidth: true
                Label { text: panel.timing.roll_label || "δ —"; color: theme.statusAttention; font.pixelSize: 11 }
                Item { Layout.fillWidth: true }
                Label { text: panel.timing.guard_label || "Flat guard —"; color: theme.accent; font.pixelSize: 11 }
                Item { Layout.fillWidth: true }
                Label { text: "Click graph to expand"; color: theme.muted; font.pixelSize: 11 }
            }
        }
        Rectangle { Layout.fillHeight: true; width: 1; color: theme.line }
        ColumnLayout {
            Layout.preferredWidth: 410; Layout.fillHeight: true; spacing: 4
            Label { text: "Cable Echo Estimate"; color: theme.text; font.bold: true; font.pixelSize: 13 }
            Label { text: "Echo distance from the flat guard time and velocity factor (VF). VF is signal speed relative to light; a reflected echo travels out and back."; color: theme.muted; font.pixelSize: 11; Layout.fillWidth: true; wrapMode: Text.WordWrap }
            ComboBox {
                id: cable; objectName: "saCableModel"; Layout.fillWidth: true
                model: ["Belden 1694WB RG-6 · VF 0.82", "CommScope P3 500 · VF 0.87", "Custom cable / velocity factor"]
                Accessible.name: "Cable model for timing estimate"
            }
            RowLayout {
                visible: cable.currentIndex === 2
                Label { text: "Velocity factor"; color: theme.muted; font.pixelSize: 12 }
                TextField {
                    id: customVelocity; objectName: "saCustomVelocity"
                    text: ""; placeholderText: "0 < VF ≤ 1"; Layout.fillWidth: true
                    validator: DoubleValidator { bottom: 0; top: 1; decimals: 6; locale: "C" }
                    Accessible.name: "Custom cable velocity factor"
                }
            }
            GridLayout {
                objectName: "saCableGrid"
                columns: 2; columnSpacing: 18; rowSpacing: 3; Layout.fillWidth: true
                Label { text: "Flat guard time"; color: theme.muted; font.pixelSize: 12 }
                Label { text: panel.timing.guard_label ? panel.timing.guard_label.substring("Flat guard ".length) : "—"; color: theme.accent; font.pixelSize: 12 }
                Label { text: "Extra cable path"; color: theme.muted; font.pixelSize: 12 }
                Label { objectName: "saExtraPath"; text: panel.timing.extra_path || "—"; color: theme.text; font.pixelSize: 12 }
                Label { text: "Distance to reflector"; color: theme.muted; font.pixelSize: 12 }
                Label { objectName: "saEchoSpacing"; text: panel.timing.reflection_spacing || "—"; color: theme.statusAttention; font.pixelSize: 12 }
            }
            Label { visible: !!panel.timing.error; text: panel.timing.error || ""; color: theme.statusAttention; font.pixelSize: 11; Layout.fillWidth: true; wrapMode: Text.WordWrap }
            Label {
                Layout.fillWidth: true
                text: "Assumes uniform cable and zero timing margin. Echo estimate, not a cable-length limit or guaranteed protection."
                color: theme.muted; font.pixelSize: 11; wrapMode: Text.WordWrap
            }
            Item { Layout.fillHeight: true }
        }
    }
}
