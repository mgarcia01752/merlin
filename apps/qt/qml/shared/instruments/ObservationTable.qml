/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

// Drawing only. The caller admits the original headings and formatted row arrays
// before their Qt model conversion; this component retains their aliases.
ColumnLayout {
    id: table
    required property var theme
    property var headings: []
    property var rows: []
    property string emptyText: "No observations."
    property string rowsObjectName: "observationRows"
    readonly property int rowHeight: 32
    readonly property int columnCount: headings ? headings.length : 0

    Row {
        Layout.fillWidth: true
        Repeater {
            model: table.headings
            Label {
                required property string modelData
                width: table.columnCount > 0 ? table.width / table.columnCount : 0
                text: modelData
                textFormat: Text.PlainText
                color: table.theme.accent
                font.bold: true
                wrapMode: Text.WordWrap
            }
        }
    }
    ListView {
        objectName: table.rowsObjectName
        Layout.fillWidth: true
        Layout.fillHeight: true
        clip: true
        model: table.rows
        ScrollBar.vertical: ScrollBar {}
        delegate: Row {
            id: row
            required property var modelData
            width: ListView.view.width
            height: table.rowHeight
            readonly property var values: modelData
            Repeater {
                model: row.values
                Label {
                    required property var modelData
                    width: table.columnCount > 0 ? table.width / table.columnCount : 0
                    text: modelData
                    textFormat: Text.PlainText
                    color: text === "Fail" ? table.theme.statusError : table.theme.text
                }
            }
        }
        Label {
            anchors.centerIn: parent
            visible: !table.rows || !table.rows.length
            text: table.emptyText
            textFormat: Text.PlainText
            color: table.theme.muted
        }
    }
}
