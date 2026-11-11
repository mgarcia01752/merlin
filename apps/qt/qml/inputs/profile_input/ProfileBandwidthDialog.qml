/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
// Adapted from PNM Viewer ProfileBandwidth.qml.
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
WorkbenchDialog {
    id: dialog
    objectName: "profileBandwidthDialog"
    property var profiles: []
    property var selectedIds: []
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(460, parent.width - 32)
    height: Math.min(400, parent.height - 32)
    modal: true
    title: "Profile Bandwidth"
    standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        RowLayout {
            Label { text: "Profile"; color: dialog.theme.muted; Layout.fillWidth: true }
            Label { text: "Bandwidth"; color: dialog.theme.muted }
        }
        ScrollView {
            id: scroll
            Layout.fillWidth: true; Layout.fillHeight: true; contentWidth: availableWidth; clip: true
            ColumnLayout {
                width: scroll.availableWidth
                Repeater {
                    model: dialog.profiles
                    Rectangle {
                        required property var modelData
                        Layout.fillWidth: true; implicitHeight: 36; radius: 4
                        color: dialog.selectedIds.indexOf(modelData.id) >= 0 ? dialog.theme.accentSoft : "transparent"
                        RowLayout {
                            anchors.fill: parent; anchors.margins: 8
                            Label { text: "Profile " + modelData.id; color: dialog.theme.text; Layout.fillWidth: true }
                            Label { text: modelData.bandwidth_text; color: dialog.theme.text; font.family: "monospace" }
                        }
                    }
                }
            }
        }
        Label { text: "Data carriers only · ideal gross rate before overhead"; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap }
    }
}
