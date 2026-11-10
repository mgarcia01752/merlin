/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Dialog {
    id: dialog
    required property var theme
    property string testPoint: ""
    property string description: ""
    function showInstrument(name, point, detail) { title = name; testPoint = point; description = detail; open() }
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(650, parent ? parent.width - 40 : 650)
    modal: true; standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 18
        Label { text: dialog.testPoint; color: dialog.theme.accent; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        Label { text: dialog.description; color: dialog.theme.text; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        Label { text: "No retained receiver results. This instrument's result connection is pending; opening it does not run or modify the simulation."; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap }
    }
}
