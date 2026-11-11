/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ColumnLayout {
    id: grid
    required property var theme
    property var nodes: []
    property var folded: ({})
    property var visibleRows: []
    property var selectedNode: null
    readonly property int rowCount: visibleRows.length
    readonly property real fieldWidth: Math.max(200, (width - 220) * 0.40)
    property alias list: rows
    spacing: 0

    // Paths identify presentation rows within one immutable packet, not TLV types.
    function rebuild() {
        const output = []
        function append(nodes, depth, prefix) {
            for (let i = 0; i < nodes.length; i++) {
                const path = prefix + i
                const node = nodes[i]
                const children = node.children || []
                output.push({node: node, depth: depth, path: path, branch: children.length > 0})
                if (children.length && grid.folded[path]) {
                    append(children, depth + 1, path + ".")
                }
            }
        }
        append(nodes || [], 0, "")
        visibleRows = output
    }
    function reset() {
        folded = ({})
        rebuild()
        rows.currentIndex = -1
        selectedNode = null
        rows.positionViewAtBeginning()
    }
    function toggle(index) {
        const item = visibleRows[index]
        if (!item || !item.branch) {
            return
        }
        const next = Object.assign({}, folded)
        next[item.path] = !next[item.path]
        folded = next
        rebuild()
        rows.currentIndex = Math.min(index, visibleRows.length - 1)
        selectedNode = item.node
    }
    function expandAll(expand) {
        const next = {}
        function visit(nodes, prefix) {
            for (let i = 0; i < nodes.length; i++) {
                const path = prefix + i
                next[path] = expand
                visit(nodes[i].children || [], path + ".")
            }
        }
        visit(nodes || [], "")
        folded = next
        rebuild()
        rows.currentIndex = -1
        selectedNode = null
    }
    onNodesChanged: reset()
    Component.onCompleted: reset()

    RowLayout {
        Layout.fillWidth: true
        Layout.bottomMargin: 7
        Label { text: "TLV Fields"; color: grid.theme.accent; font.bold: true; Layout.fillWidth: true }
        WorkbenchActionButton { theme: grid.theme; objectName: "tlvExpandAll"; text: "Expand All"; enabled: grid.nodes.length > 0; onClicked: grid.expandAll(true) }
        WorkbenchActionButton { theme: grid.theme; objectName: "tlvCollapseAll"; text: "Collapse All"; enabled: grid.nodes.length > 0; onClicked: grid.expandAll(false) }
    }
    Rectangle {
        Layout.fillWidth: true
        implicitHeight: 30
        color: grid.theme.button
        RowLayout {
            anchors.fill: parent; anchors.leftMargin: 8; anchors.rightMargin: 16; spacing: 10
            Label { text: "Field"; color: grid.theme.text; font.bold: true; Layout.preferredWidth: grid.fieldWidth }
            Label { text: "Value"; color: grid.theme.text; font.bold: true; Layout.fillWidth: true }
            Label { text: "Type"; color: grid.theme.text; Layout.preferredWidth: 42 }
            Label { text: "Length"; color: grid.theme.text; Layout.preferredWidth: 52 }
            Label { text: "Offset"; color: grid.theme.text; Layout.preferredWidth: 58 }
        }
    }
    ListView {
        id: rows
        objectName: "tlvTreeRows"
        Layout.fillWidth: true; Layout.fillHeight: true; Layout.minimumHeight: 100
        clip: true; model: grid.visibleRows; reuseItems: true
        ScrollBar.vertical: ScrollBar {}
        onCurrentIndexChanged: {
            const row = grid.visibleRows[currentIndex]
            if (row) {
                grid.selectedNode = row.node
            }
        }
        Keys.onRightPressed: {
            const row = grid.visibleRows[currentIndex]
            if (row && row.branch && !grid.folded[row.path]) {
                grid.toggle(currentIndex)
            }
        }
        Keys.onLeftPressed: {
            const row = grid.visibleRows[currentIndex]
            if (row && row.branch && grid.folded[row.path]) {
                grid.toggle(currentIndex)
            } else if (row && row.depth > 0) {
                for (let i = currentIndex - 1; i >= 0; i--) {
                    if (grid.visibleRows[i].depth < row.depth) {
                        currentIndex = i
                        break
                    }
                }
            }
        }
        Keys.onSpacePressed: grid.toggle(currentIndex)
        Keys.onReturnPressed: grid.toggle(currentIndex)
        delegate: ItemDelegate {
            id: fieldRow
            required property var modelData
            required property int index
            width: rows.width; height: 32
            padding: 0
            highlighted: rows.currentIndex === index
            Accessible.name: modelData.node.name + ": " + (modelData.node.value || "")
            Accessible.description: modelData.branch ? (grid.folded[modelData.path] ? "Expanded" : "Collapsed") : "Field value"
            onClicked: { rows.currentIndex = index; rows.forceActiveFocus() }
            onDoubleClicked: grid.toggle(index)
            background: Rectangle {
                color: fieldRow.highlighted ? grid.theme.button : fieldRow.index % 2 ? grid.theme.panel : grid.theme.background
                Rectangle { anchors.bottom: parent.bottom; width: parent.width; height: 1; color: grid.theme.line; opacity: 0.4 }
            }
            contentItem: RowLayout {
                spacing: 10
                anchors.left: parent.left; anchors.right: parent.right
                anchors.leftMargin: 8; anchors.rightMargin: 16
                Item {
                    Layout.preferredWidth: grid.fieldWidth; Layout.fillHeight: true
                    ToolButton {
                        objectName: "tlvFold" + fieldRow.index
                        visible: fieldRow.modelData.branch
                        x: fieldRow.modelData.depth * 18; width: 26; height: 30
                        anchors.verticalCenter: parent.verticalCenter
                        text: grid.folded[fieldRow.modelData.path] ? "▾" : "▸"
                        Accessible.name: (grid.folded[fieldRow.modelData.path] ? "Collapse " : "Expand ") + fieldRow.modelData.node.name
                        onClicked: { grid.list.forceActiveFocus(); grid.toggle(fieldRow.index) }
                    }
                    Label {
                        x: fieldRow.modelData.depth * 18 + 28
                        width: parent.width - x; anchors.verticalCenter: parent.verticalCenter
                        text: fieldRow.modelData.node.name; color: grid.theme.text
                        font.bold: fieldRow.modelData.branch; elide: Text.ElideRight
                    }
                }
                Label { text: fieldRow.modelData.node.value || ""; color: fieldRow.modelData.node.supported === false ? grid.theme.statusAttention : grid.theme.text; Layout.fillWidth: true; elide: Text.ElideRight }
                Label { text: fieldRow.modelData.node.kind === undefined ? "" : fieldRow.modelData.node.kind; color: grid.theme.muted; Layout.preferredWidth: 42 }
                Label { text: fieldRow.modelData.node.length === undefined ? "" : fieldRow.modelData.node.length; color: grid.theme.muted; Layout.preferredWidth: 52 }
                Label { text: fieldRow.modelData.node.offset === undefined ? "" : fieldRow.modelData.node.offset; color: grid.theme.muted; Layout.preferredWidth: 58 }
            }
            ToolTip.visible: hovered
            ToolTip.text: modelData.node.name + ": " + (modelData.node.value || "")
        }
        Label { visible: grid.nodes.length === 0; anchors.centerIn: parent; text: "No TLV fields available"; color: grid.theme.muted }
    }
    Rectangle {
        Layout.fillWidth: true; Layout.topMargin: 7
        implicitHeight: 76; color: grid.theme.panel; border.color: grid.theme.line; radius: 5
        ColumnLayout {
            anchors.fill: parent; anchors.margins: 7; spacing: 2
            Label {
                Layout.fillWidth: true; color: grid.theme.text; elide: Text.ElideRight
                text: grid.selectedNode ? grid.selectedNode.name + ": " + grid.selectedNode.value : "Select a field for its value and bytes. Offsets are absolute packet bytes."
            }
            Label { Layout.fillWidth: true; visible: text.length > 0; text: grid.selectedNode ? grid.selectedNode.description || "" : ""; color: grid.theme.muted; font.pixelSize: 11; elide: Text.ElideRight }
            TextInput {
                Layout.fillWidth: true; color: grid.theme.accent; font.family: "monospace"; font.pixelSize: 11
                text: grid.selectedNode ? grid.selectedNode.hex || "" : ""
                readOnly: true; selectByMouse: true; clip: true
                Accessible.name: "Selected field value bytes"
            }
        }
    }
}
