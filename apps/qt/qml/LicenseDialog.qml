/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
WorkbenchDialog {
    id: dialog
    required property var backend
    property string documentText: ""
    function showDocument(key) {
        const document = JSON.parse(backend.license_document(key))
        title = document.title; documentText = document.text; open()
    }
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(820, parent ? parent.width - 48 : 820)
    height: Math.min(650, parent ? parent.height - 48 : 650)
    modal: true; standardButtons: Dialog.Close
    contentItem: ScrollView {
        clip: true
        TextArea { objectName: "licenseDocumentText"; text: dialog.documentText; readOnly: true; selectByMouse: true; wrapMode: TextEdit.Wrap; textFormat: TextEdit.PlainText; color: dialog.theme.text; font.pixelSize: 12; background: Rectangle { color: dialog.theme.background } }
    }
}
