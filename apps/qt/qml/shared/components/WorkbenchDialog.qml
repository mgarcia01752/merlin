/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../presentation"

Dialog {
    id: dialog
    required property var theme
    property PresentationOwner visibilityCoverage: PresentationOwner { id: opening; theme: dialog.theme }
    readonly property bool openAdmissionRejected: opening.rejected
    /**
     * Admit visibility before opening, preserving all hidden data on rejection.
     * @returns True after Qt receives open, false with the dialog unchanged.
     */
    function openChecked() {
        if (!opening.beginOpen()) {
            if (typeof theme.presentationAdmissionRejected === "boolean") {
                theme.presentationAdmissionRejected = true
            }
            return false
        }
        if (typeof theme.presentationAdmissionRejected === "boolean") {
            theme.presentationAdmissionRejected = false
        }
        dialog.open()
        return true
    }
    property Connections visibilityLifecycle: Connections {
        target: dialog
        function onClosed() { opening.endOpen() }
    }

    footer: DialogButtonBox {
        standardButtons: dialog.standardButtons
        visible: count > 0
        alignment: Qt.AlignRight
        spacing: 10
        padding: 10
        background: Item {}
        delegate: WorkbenchActionButton { theme: dialog.theme }
        // Dialog forwards button-box roles automatically.
    }
    header: RowLayout {
        spacing: 8
        Label {
            text: dialog.title
            color: dialog.theme.text
            font.bold: true
            elide: Text.ElideRight
            Layout.fillWidth: true
            Layout.leftMargin: 6
            Layout.topMargin: 4
            Layout.bottomMargin: 4
        }
        ToolButton {
            objectName: "popupCloseButton"
            text: "×"
            font.pixelSize: 21
            Layout.preferredWidth: 28
            Layout.preferredHeight: 28
            Layout.rightMargin: 4
            Layout.topMargin: 4
            Layout.bottomMargin: 4
            focusPolicy: Qt.StrongFocus
            Accessible.name: "Close " + dialog.title
            ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered
            ToolTip.text: "Close"
            background: Rectangle {
                radius: width / 2
                color: parent.down ? dialog.theme.line : parent.hovered ? dialog.theme.line : dialog.theme.button
                border.color: parent.visualFocus ? dialog.theme.accent : "transparent"
                border.width: 2
            }
            contentItem: Label {
                text: parent.text
                font: parent.font
                color: dialog.theme.text
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }
            onClicked: dialog.reject()
        }
    }
}
