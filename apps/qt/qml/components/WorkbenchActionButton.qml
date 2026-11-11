/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls

Button {
    id: control
    required property var theme
    property bool primary: false
    implicitWidth: Math.max(96, contentItem.implicitWidth + 32)
    implicitHeight: 38
    leftPadding: 16
    rightPadding: 16
    hoverEnabled: true
    focusPolicy: Qt.StrongFocus
    font.pixelSize: 13
    font.weight: Font.DemiBold
    background: Rectangle {
        radius: 9
        color: !control.enabled ? control.theme.panel
            : control.down ? control.theme.line
            : control.primary || control.hovered ? control.theme.accentSoft : control.theme.button
        border.width: control.visualFocus ? 2 : 1
        border.color: !control.enabled ? control.theme.line
            : control.primary || control.hovered || control.visualFocus ? control.theme.accent : control.theme.line
    }
    contentItem: Label {
        text: control.text
        font: control.font
        color: !control.enabled ? control.theme.muted
            : control.primary ? control.theme.accent : control.theme.text
        opacity: control.enabled ? 1 : 0.55
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
    }
}
