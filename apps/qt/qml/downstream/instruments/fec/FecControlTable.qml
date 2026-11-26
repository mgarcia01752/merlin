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
    // The parent admits this complete projection before any table conversion.
    property var projection: ({})
    readonly property var rows: projection.rows || []

    /**
     * Prepare the existing control-channel table without retaining it here.
     * @param statistics Original FEC DTO, borrowed during candidate construction.
     * @returns Plain rows; parent must admit them before assigning projection.
     */
    function prepare(statistics) {
        const value = function(number){return number===undefined || number===null ? "—" : String(number)}
        const original=[
            ["Channel","Metric","Value"],
            ["NCP","CRC-accepted fields",statistics.ncp_fields ?? null],
            ["NCP","Total LDPC iterations",statistics.ncp_iterations ?? null],
            ["PLC","Frames",statistics.plc_frames ?? null],
            ["PLC","Total LDPC iterations",statistics.plc_iterations ?? null],
            ["PLC","Compared bit errors",statistics.plc_bit_errors ?? null]
        ]
        return {rows:original,text:original.map(function(row){return value(row[2])})}
    }
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
                    text: table.projection.text[index]
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
