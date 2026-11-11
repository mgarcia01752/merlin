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
    required property var backend
    property string documentText: ""
    property string contextText: ""
    function showEntry(code, diagnostic) {
        const document = JSON.parse(backend.faq_document(code))
        title = document.title
        documentText = document.text
        contextText = diagnostic ? diagnostic.tooltip : ""
        open()
    }
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(800, parent ? parent.width - 40 : 800)
    height: Math.min(640, parent ? parent.height - 40 : 640)
    modal: true
    standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 14
        Label {
            objectName: "faqContext"
            visible: text.length > 0
            text: dialog.contextText
            textFormat: Text.PlainText
            color: dialog.theme.accent
            Layout.fillWidth: true
            wrapMode: Text.WordWrap
        }
        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            TextArea {
                objectName: "faqDocumentText"
                text: dialog.documentText
                readOnly: true
                selectByMouse: true
                wrapMode: TextEdit.Wrap
                textFormat: TextEdit.MarkdownText
                color: dialog.theme.text
                background: Rectangle { color: dialog.theme.background }
            }
        }
    }
}
