/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Column {
    id: table
    required property var theme
    property var statistics: ({})
    readonly property var rows: [
        ["Channel", "Metric", "Value"],
        ["NCP", "CRC-accepted fields", statistics.ncp_fields],
        ["NCP", "Total LDPC iterations", statistics.ncp_iterations],
        ["PLC", "Frames", statistics.plc_frames],
        ["PLC", "Total LDPC iterations", statistics.plc_iterations],
        ["PLC", "Compared bit errors", statistics.plc_bit_errors]
    ]
    Repeater {
        model: table.rows
        Rectangle {
            required property var modelData
            required property int index
            width: table.width
            height: 34
            color: index === 0 ? table.theme.line : index % 2 ? table.theme.panel : table.theme.background
            border.color: table.theme.line
            RowLayout {
                anchors.fill: parent
                spacing: 0
                Label {
                    text: modelData[0]
                    color: index === 0 ? table.theme.text : table.theme.accent
                    font.bold: index === 0
                    leftPadding: 10
                    Layout.preferredWidth: 82
                }
                Rectangle { width: 1; Layout.fillHeight: true; color: table.theme.line }
                Label {
                    text: modelData[1]
                    color: table.theme.text
                    font.bold: index === 0
                    leftPadding: 10
                    Layout.fillWidth: true
                }
                Rectangle { width: 1; Layout.fillHeight: true; color: table.theme.line }
                Label {
                    objectName: "fecControlValue" + index
                    text: modelData[2] === undefined || modelData[2] === null ? "—" : String(modelData[2])
                    color: table.theme.text
                    font.bold: index === 0
                    horizontalAlignment: Text.AlignRight
                    rightPadding: 10
                    Layout.preferredWidth: 100
                }
            }
        }
    }
}
