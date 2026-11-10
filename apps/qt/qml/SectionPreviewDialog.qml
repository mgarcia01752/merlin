/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

WorkbenchDialog {
    property string description: ""
    objectName: "sectionDialog"
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(540, parent ? parent.width - 48 : 540)
    modal: true
    standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 18
        Label { text: "SECTION PREVIEW"; color: theme.accent; font.pixelSize: 11; font.letterSpacing: 1.5 }
        Label { Layout.fillWidth: true; text: description; wrapMode: Text.WordWrap; color: theme.text; font.pixelSize: 15 }
    }
}
