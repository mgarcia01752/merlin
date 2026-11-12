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
    property string pnmFileType: ""
    property string indicatorText: ""
    property string indicatorDescription: ""
    Accessible.name: heading
    Accessible.description: statusLabel + ": " + statusReason + (pnmFileType ? ". CM PNM File: " + pnmFileType : "") + (indicatorText ? ". " + indicatorDescription : "")
    ToolTip.visible: hovered || visualFocus
    ToolTip.delay: 500
    ToolTip.text: statusLabel + ": " + statusReason + (indicatorText ? "\n" + indicatorDescription : "")
    hoverEnabled: true
    Rectangle {
        objectName: "blockModeIndicator"
        visible: block.indicatorText.length > 0
        anchors.top: parent.top; anchors.right: parent.right
        anchors.topMargin: 6; anchors.rightMargin: 7
        width: indicatorLabel.implicitWidth + 12; height: 18; radius: 4
        color: block.theme.accentSoft
        Text {
            id: indicatorLabel
            anchors.centerIn: parent
            text: block.indicatorText; color: block.theme.accent
            font.pixelSize: 10; font.weight: Font.DemiBold
        }
    }
    background: Rectangle {
        radius: 12
        gradient: Gradient {
            GradientStop { position: 0; color: block.hovered || block.visualFocus ? block.theme.accentSoft : block.theme.dark ? "#102d46" : "#f4fbff" }
            GradientStop { position: 1; color: block.hovered || block.visualFocus ? block.theme.accentSoft : block.theme.panel }
        }
        color: block.hovered || block.visualFocus ? block.theme.accentSoft : block.theme.panel
        border.color: block.hovered || block.visualFocus ? block.theme.accent : block.theme.dark ? "#168cb5" : "#659db8"
        border.width: block.visualFocus ? 2 : 1
        Rectangle {
            objectName: "statusStripe"
            width: 3; height: parent.height - 26
            x: 0; y: 13; radius: 1
            color: block.statusColor
        }
    }
    contentItem: Item {
        Text {
            objectName: "blockHeading"
            anchors.left: parent.left; anchors.right: parent.right
            anchors.top: parent.top; anchors.bottom: fileFootnote.top
            anchors.bottomMargin: block.pnmFileType ? 4 : 0
            text: block.heading
            color: block.theme.text
            font.pixelSize: 18; minimumPixelSize: 12; fontSizeMode: Text.Fit
            font.weight: Font.DemiBold
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
            wrapMode: Text.WordWrap
        }
        Text {
            id: fileFootnote; objectName: "pnmFileTypeFootnote"
            anchors.left: parent.left; anchors.right: parent.right; anchors.bottom: parent.bottom
            height: block.pnmFileType ? implicitHeight : 0
            visible: block.pnmFileType !== ""
            text: block.pnmFileType ? "CM PNM File:\n" + block.pnmFileType : ""
            color: block.theme.muted; font.pixelSize: 9
            horizontalAlignment: Text.AlignHCenter
            wrapMode: Text.WrapAnywhere
        }
    }
}
