/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls

Button {
    id: block
    required property var theme
    required property string heading
    required property string detail
    property bool inputBlock: false
    Accessible.name: heading
    hoverEnabled: true
    background: Rectangle {
        radius: block.inputBlock ? 6 : 12
        color: block.hovered || block.visualFocus ? block.theme.accentSoft : block.theme.panel
        border.color: block.hovered || block.visualFocus ? block.theme.accent : block.theme.line
        border.width: block.visualFocus ? 2 : 1
        Rectangle {
            width: 3; height: parent.height - 26
            x: 0; y: 13; radius: 1
            color: block.theme.accent
        }
    }
    contentItem: Item {
      Column {
        spacing: 8
        anchors.centerIn: parent
        width: parent.width - 20
        Text {
            width: parent.width; text: block.heading; color: block.theme.text
            font.pixelSize: 15; font.weight: Font.DemiBold
            horizontalAlignment: Text.AlignHCenter; wrapMode: Text.WordWrap
        }
        Text {
            width: parent.width; text: block.detail; color: block.theme.muted
            font.pixelSize: 12; horizontalAlignment: Text.AlignHCenter
            wrapMode: Text.WordWrap
        }
      }
    }
}
