/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../components"

WorkbenchDialog {
    id: dialog
    objectName: "workbenchEntryDialog"
    title: "Configure WorkBench"
    parent: Overlay.overlay
    anchors.centerIn: parent
    modal: true
    width: Math.min(480, parent ? parent.width - 40 : 480)
    signal createRequested()
    signal openRequested()
    contentItem: ColumnLayout {
        spacing: 18
        Label {
            text: "Create a new workbench or open an existing one."
            color: dialog.theme.text
            wrapMode: Text.WordWrap
            Layout.fillWidth: true
        }
        RowLayout {
            uniformCellSizes: true
            spacing: 12
            Layout.fillWidth: true
            WorkbenchActionButton {
                objectName: "createWorkbenchChoice"
                theme: dialog.theme
                text: "Create New"
                primary: true
                Layout.fillWidth: true
                onClicked: { dialog.close(); dialog.createRequested() }
            }
            WorkbenchActionButton {
                objectName: "openWorkbenchChoice"
                theme: dialog.theme
                text: "Open Existing…"
                Layout.fillWidth: true
                onClicked: { dialog.close(); dialog.openRequested() }
            }
        }
    }
    footer: DialogButtonBox {
        WorkbenchActionButton {
            theme: dialog.theme
            objectName: "cancelWorkbenchChoice"
            text: "Cancel"
            DialogButtonBox.buttonRole: DialogButtonBox.RejectRole
        }
    }
}
