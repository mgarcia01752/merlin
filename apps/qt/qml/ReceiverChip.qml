/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Button {
    id: chip
    required property var theme
    property bool stage: false
    implicitWidth: 140
    implicitHeight: stage ? 64 : 36
    Layout.fillWidth: true
    Layout.minimumWidth: 0
    Layout.preferredWidth: 1
    hoverEnabled: true
    padding: 12
    HoverHandler { cursorShape: Qt.PointingHandCursor }
    background: Item {
        Rectangle {
            anchors.fill: parent
            anchors.leftMargin: chip.stage ? 5 : 0
            anchors.rightMargin: chip.stage ? 5 : 0
            radius: chip.stage ? 12 : 9
            color: chip.down || chip.hovered ? chip.theme.accentSoft : chip.theme.button
            border.color: chip.visualFocus || chip.hovered ? chip.theme.accent : chip.theme.line
            border.width: chip.visualFocus ? 2 : 1
            Rectangle {
                visible: chip.stage
                anchors.fill: parent; anchors.margins: 4
                radius: 9; color: "transparent"
                border.color: chip.theme.line
            }
        }
        Repeater {
            model: chip.stage ? 6 : 0
            Rectangle {
                required property int index
                width: 6; height: 2; radius: 1
                x: index < 3 ? 0 : parent.width - width
                y: parent.height * (0.3 + (index % 3) * 0.2) - height / 2
                color: chip.theme.accent; opacity: 0.65
            }
        }
    }
    contentItem: Text {
        text: chip.text
        color: chip.theme.text
        font.pixelSize: chip.stage ? 13 : 12
        font.weight: Font.DemiBold
        wrapMode: Text.WordWrap
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
    }
}
