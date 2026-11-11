/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ColumnLayout {
    id: chart
    required property var theme
    property var allocation: ({segments:[],total:0})
    property string emptyText: "No completed codewords."
    function seriesColor(key) {
        if (key === "corrected") { return theme.statusAttention }
        if (key === "failed" || key === "mismatch") { return theme.statusError }
        if (key === "pending") { return theme.muted }
        return theme.statusComplete
    }
    spacing: 12
    Item {
        Layout.fillWidth: true; Layout.fillHeight: true; Layout.minimumHeight: 34
        Rectangle {
            anchors.verticalCenter: parent.verticalCenter
            width: parent.width; height: 32; color: chart.theme.line
            Row {
                anchors.fill: parent
                Repeater {
                    model: chart.allocation ? chart.allocation.segments || [] : []
                    Rectangle {
                        required property var modelData
                        width: parent.width * modelData.fraction; height: parent.height
                        color: chart.seriesColor(modelData.key)
                        Accessible.role: Accessible.Graphic
                        Accessible.name: modelData.label + ": " + modelData.count
                        Label { anchors.centerIn: parent; visible: parent.width > implicitWidth + 12; text: modelData.count; color: chart.theme.background; font.bold: true }
                        MouseArea { id: hover; anchors.fill: parent; hoverEnabled: true }
                        ToolTip.visible: hover.containsMouse
                        ToolTip.text: modelData.label + ": " + modelData.count
                    }
                }
            }
        }
        Label { anchors.centerIn: parent; visible: !chart.allocation || !chart.allocation.total; text: chart.emptyText; color: chart.theme.text }
    }
    Flow {
        Layout.fillWidth: true
        spacing: 14
        Repeater {
            model: chart.allocation ? chart.allocation.segments || [] : []
            Row {
                required property var modelData
                spacing: 6
                Rectangle { width: 10; height: 10; anchors.verticalCenter: parent.verticalCenter; color: chart.seriesColor(modelData.key) }
                Label { text: modelData.label + " " + modelData.count; color: chart.theme.text }
            }
        }
    }
}
