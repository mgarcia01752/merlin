/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
// Adapted from RxMerCollectionTree: virtualized expandable rows, original file
// metadata and explicit selection. The adapter supplies compatibility and order.
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
Item {
    id: tree
    required property var theme
    property var files: []
    property int selectedIndex: -1
    property var expanded: ({})
    signal selected(int index)
    onFilesChanged: expanded = ({})
    function rows() {
        const rows = []
        for (const file of files) {
            rows.push({kind: "file", index: file.index, label: file.name, detail: file.frequency, compatible: file.compatible})
            if (expanded[file.index]) {
                for (const detail of [file.mac + " · channel " + file.channel, file.frequency, "Capture epoch " + file.capture_time, file.format + " · " + file.bytes + " bytes", file.reason, "SHA-256 " + file.sha256]) {
                    rows.push({kind: "detail", index: file.index, label: detail, detail: detail})
                }
            }
        }
        return rows
    }
    ListView {
        id: nodes; objectName: "measurementCaptureTree"
        anchors.fill: parent; clip: true; model: tree.rows()
        ScrollBar.vertical: ScrollBar {}
        delegate: Rectangle {
            required property var modelData
            width: nodes.width; height: modelData.kind === "file" ? 34 : 24
            color: modelData.kind === "file" && tree.selectedIndex === modelData.index ? tree.theme.accentSoft : "transparent"
            RowLayout {
                anchors.fill: parent; spacing: 2
                ToolButton {
                    visible: modelData.kind === "file"; Layout.preferredWidth: 24
                    text: tree.expanded[modelData.index] ? "▾" : "▸"
                    onClicked: { const next = Object.assign({}, tree.expanded); next[modelData.index] = !next[modelData.index]; tree.expanded = next }
                    Accessible.name: "Expand capture metadata"
                }
                Label { visible: modelData.kind === "file"; text: "●"; font.pixelSize: 10; color: modelData.compatible === true ? tree.theme.statusComplete : modelData.compatible === false ? tree.theme.statusError : tree.theme.statusAttention }
                ItemDelegate {
                    Layout.fillWidth: true
                    text: modelData.label
                    contentItem: Label { text: parent.text; color: modelData.kind === "file" ? tree.theme.text : tree.theme.muted; font.pixelSize: 11; elide: Text.ElideRight; verticalAlignment: Text.AlignVCenter; leftPadding: modelData.kind === "file" ? 0 : 24 }
                    onClicked: tree.selected(modelData.index)
                    ToolTip.visible: hovered; ToolTip.text: modelData.detail || modelData.label
                }
            }
        }
    }
}
