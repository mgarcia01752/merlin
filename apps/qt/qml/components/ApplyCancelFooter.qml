/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Pane {
    id: footer
    default property alias secondaryActions: secondary.data
    property string applyObjectName: ""
    property string cancelObjectName: "dialogCancel"
    property bool applyEnabled: true
    signal applied()
    signal cancelled()

    padding: 10
    background: Item {}
    contentItem: RowLayout {
        spacing: 10
        RowLayout { id: secondary; spacing: 10 }
        Item { Layout.fillWidth: true }
        Button {
            id: applyButton
            objectName: footer.applyObjectName
            text: "Apply"
            enabled: footer.applyEnabled
            Layout.preferredWidth: Math.max(80, implicitWidth, cancelButton.implicitWidth)
            Layout.preferredHeight: Math.max(implicitHeight, cancelButton.implicitHeight)
            onClicked: footer.applied()
        }
        Button {
            id: cancelButton
            objectName: footer.cancelObjectName
            text: "Cancel"
            Layout.preferredWidth: Math.max(80, implicitWidth, applyButton.implicitWidth)
            Layout.preferredHeight: Math.max(implicitHeight, applyButton.implicitHeight)
            onClicked: footer.cancelled()
        }
    }
}
