/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

WorkbenchDialog {
    id: dialog
    title: "Settings"
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(540, parent ? parent.width - 48 : 540)
    modal: true
    standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 18
        Label { text: "Global"; color: dialog.theme.accent; font.bold: true; font.pixelSize: 18 }
        RowLayout {
            Label { text: "Light/Dark mode"; color: dialog.theme.text }
            ComboBox {
                objectName: "globalThemeMode"
                model: ["Dark", "Light"]
                currentIndex: dialog.theme.dark ? 0 : 1
                onActivated: dialog.theme.dark = currentIndex === 0
                Accessible.name: "Global Light/Dark mode"
            }
        }
        Label {
            Layout.fillWidth: true
            text: "Applies immediately to all workbench windows. Appearance is kept for this session."
            wrapMode: Text.WordWrap; color: dialog.theme.muted
        }
    }
}
