/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import "../presentation"

Label {
    id: status
    required property var theme
    property var diagnostic: null
    property string activityTooltip: ""
    readonly property bool hasHelp: !presentation.rejected && !!presentation.value && !!presentation.value.diagnostic && !!presentation.value.diagnostic.code && !!presentation.value.diagnostic.tooltip
    property PresentationOwner presentationCoverage: PresentationOwner {id:presentation;theme:status.theme}
    /** Admit concrete help copies and their code together before native text use. */
    function refreshPresentation() {
        const help=!!diagnostic && !!diagnostic.code && !!diagnostic.tooltip
        return presentation.replace({diagnostic:diagnostic,description:help ? diagnostic.tooltip+" Open FAQ for details." : activityTooltip,
            tooltip:help ? diagnostic.tooltip+"\nClick or press Enter for FAQ details." : activityTooltip})
    }
    onDiagnosticChanged: refreshPresentation()
    onActivityTooltipChanged: refreshPresentation()
    Component.onCompleted: refreshPresentation()
    signal helpRequested(string code)
    textFormat: Text.PlainText
    wrapMode: Text.WordWrap
    color: hasHelp ? theme.statusAttention : theme.muted
    focusPolicy: hasHelp ? Qt.TabFocus : Qt.NoFocus
    activeFocusOnTab: hasHelp
    font.underline: hasHelp && (hover.containsMouse || activeFocus)
    Accessible.role: hasHelp ? Accessible.Link : Accessible.StaticText
    Accessible.name: text
    Accessible.description:presentation.value ? presentation.value.description : ""
    background: Rectangle {
        color: "transparent"
        radius: 4
        border.width: status.activeFocus && status.hasHelp ? 1 : 0
        border.color: status.theme.accent
    }
    function openHelp() {
        if (hasHelp) {helpRequested(presentation.value.diagnostic.code)}
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
        text: !presentation.rejected && presentation.value ? presentation.value.tooltip : ""
        visible: (hover.containsMouse || status.activeFocus) && text.length > 0
        delay: status.theme.tooltipDelayMs
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
