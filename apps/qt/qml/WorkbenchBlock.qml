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
    required property string blockStatus
    required property string statusReason
    readonly property string statusLabel: blockStatus === "complete" ? "Complete" : blockStatus === "error" ? "Error" : "Needs attention"
    readonly property color statusColor: blockStatus === "complete" ? theme.statusComplete : blockStatus === "error" ? theme.statusError : theme.statusAttention
    property bool inputBlock: false
    Accessible.name: heading
    Accessible.description: statusLabel + ": " + statusReason
    ToolTip.visible: hovered || visualFocus
    ToolTip.delay: 500
    ToolTip.text: statusLabel + ": " + statusReason
    hoverEnabled: true
    background: Rectangle {
        radius: block.inputBlock ? 6 : 12
        color: block.hovered || block.visualFocus ? block.theme.accentSoft : block.theme.panel
        border.color: block.hovered || block.visualFocus ? block.theme.accent : block.theme.line
        border.width: block.visualFocus ? 2 : 1
        Rectangle {
            objectName: "statusStripe"
            width: 3; height: parent.height - 26
            x: 0; y: 13; radius: 1
            color: block.statusColor
        }
    }
    contentItem: Text {
        text: block.heading
        color: block.theme.text
        font.pixelSize: 15
        font.weight: Font.DemiBold
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
        wrapMode: Text.WordWrap
    }
}
