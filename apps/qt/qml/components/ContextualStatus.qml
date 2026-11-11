/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls

Label {
    id: status
    required property var theme
    property var diagnostic: null
    property string activityTooltip: ""
    readonly property bool hasHelp: !!diagnostic && !!diagnostic.code && !!diagnostic.tooltip
    signal helpRequested(string code)
    textFormat: Text.PlainText
    wrapMode: Text.WordWrap
    color: hasHelp ? theme.statusAttention : theme.muted
    focusPolicy: hasHelp ? Qt.StrongFocus : Qt.NoFocus
    activeFocusOnTab: hasHelp
    font.underline: hasHelp && (hover.containsMouse || activeFocus)
    Accessible.role: hasHelp ? Accessible.Link : Accessible.StaticText
    Accessible.name: text
    Accessible.description: hasHelp ? diagnostic.tooltip + " Open FAQ for details." : activityTooltip
    background: Rectangle {
        color: "transparent"
        radius: 4
        border.width: status.activeFocus && status.hasHelp ? 1 : 0
        border.color: status.theme.accent
    }
    function openHelp() {
        if (hasHelp) { helpRequested(diagnostic.code) }
    }
    Keys.onReturnPressed: openHelp()
    Keys.onEnterPressed: openHelp()
    Keys.onSpacePressed: openHelp()
    Keys.onPressed: function(event) {
        if (hasHelp && event.key === Qt.Key_F1) { openHelp(); event.accepted = true }
        else { event.accepted = false }
    }
    MouseArea {
        id: hover
        anchors.fill: parent
        hoverEnabled: true
        acceptedButtons: status.hasHelp ? Qt.LeftButton : Qt.NoButton
        cursorShape: status.hasHelp ? Qt.PointingHandCursor : Qt.ArrowCursor
        onClicked: status.openHelp()
    }
    ToolTip {
        id: tip
        objectName: "contextualStatusTooltip"
        text: status.hasHelp ? status.diagnostic.tooltip + "\nClick or press Enter for FAQ details." : status.activityTooltip
        visible: (hover.containsMouse || status.activeFocus) && text.length > 0
        delay: 400
        timeout: 15000
        width: Math.min(520, status.Window.window ? status.Window.window.width - 40 : 520)
        contentItem: Label {
            text: tip.text
            textFormat: Text.PlainText
            color: status.theme.text
            wrapMode: Text.WordWrap
        }
    }
}
