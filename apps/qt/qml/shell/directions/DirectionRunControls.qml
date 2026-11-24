/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../shared/components"
import "../../shared/presentation"

RowLayout {
    id: controls
    required property var theme
    required property string direction
    required property var identity
    property bool canConfigure: false
    property bool canStart: false
    property bool busy: false
    property bool paused: false
    property string status: ""
    property string elapsed: ""
    property var diagnostic: null
    property string activityTooltip: ""
    readonly property int actionWidth: 104
    readonly property int controlGap: 10
    readonly property int statusGap: 4
    signal actionRequested(string action, string direction, var identity)
    signal helpRequested(string code, string direction, var identity)
    spacing: controlGap

    readonly property bool contextCurrent: !!coverage.value
        && coverage.value.direction === direction && coverage.value.identity === identity
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: coverage
        theme: controls.theme
        initialValue: ({direction: controls.direction, identity: controls.identity,
            status: controls.status, elapsed: controls.elapsed,
            diagnostic: controls.diagnostic, activity: controls.activityTooltip,
            display: {status: controls.status, elapsed: controls.elapsed}})
    }
    /** Admit independent native text copies before their visible replacement. */
    function refreshText() {
        if (!coverage.replace({direction:direction,identity:identity,status:status,elapsed:elapsed,
                diagnostic:diagnostic,activity:activityTooltip,
                display:{status:status,elapsed:elapsed}})) {
            theme.presentationAdmissionRejected = true
        }
    }
    /** Retry unchanged rejected text on the existing application poll cadence. */
    function retryRejectedText() {
        if (coverage.rejected) {
            refreshText()
        }
        if (progress.presentationCoverage.rejected) {
            progress.refreshPresentation()
        }
    }
    onDirectionChanged: Qt.callLater(refreshText)
    onIdentityChanged: Qt.callLater(refreshText)
    onStatusChanged: Qt.callLater(refreshText)
    onElapsedChanged: Qt.callLater(refreshText)
    onDiagnosticChanged: Qt.callLater(refreshText)
    onActivityTooltipChanged: Qt.callLater(refreshText)
    ColumnLayout {
        Layout.fillWidth: true
        Layout.minimumWidth: 0
        spacing: controls.statusGap
        ContextualStatus {
            id: progress
            objectName: "directionRunProgress"
            theme: controls.theme
            text: coverage.value ? coverage.value.status : ""
            diagnostic: controls.contextCurrent && !coverage.rejected && coverage.value
                ? coverage.value.diagnostic : null
            activityTooltip: controls.contextCurrent && !coverage.rejected && coverage.value
                ? coverage.value.activity : ""
            onHelpRequested: function(code) {
                if (controls.contextCurrent && !coverage.rejected) {
                    controls.helpRequested(code, coverage.value.direction, coverage.value.identity)
                }
            }
            Layout.fillWidth: true
        }
        Label {
            visible: !!text
            text: coverage.value ? coverage.value.elapsed : ""
            color: controls.theme.muted
            wrapMode: Text.WordWrap
            Layout.fillWidth: true
            MouseArea { id: elapsedHover; anchors.fill: parent; hoverEnabled: true }
            ToolTip.delay: controls.theme.tooltipDelayMs
            ToolTip.visible: elapsedHover.containsMouse
            ToolTip.text: "Wall time includes pauses and CPU scheduling waits. The run budget uses active time."
        }
        Label {
            objectName: "shellPresentationAdmissionNotice"
            visible: controls.theme.presentationAdmissionRejected
            text: "Presentation limits reached; previous view preserved."
            color: controls.theme.statusAttention
            wrapMode: Text.WordWrap
            Layout.fillWidth: true
        }
    }
    WorkbenchActionButton {
        theme: controls.theme
        objectName: "directionConfigure"
        text: "Configure WorkBench"
        enabled: controls.canConfigure && controls.contextCurrent && !coverage.rejected
        onClicked: controls.actionRequested("configure", coverage.value.direction, coverage.value.identity)
    }
    WorkbenchActionButton {
        theme: controls.theme
        objectName: "directionStart"
        text: "▶  Start"
        primary: true
        Layout.preferredWidth: controls.actionWidth
        enabled: controls.canStart && controls.contextCurrent && !coverage.rejected
        onClicked: controls.actionRequested("start", coverage.value.direction, coverage.value.identity)
        Accessible.name: "Start simulation"
        Accessible.description: "Every block must be ready and the application simulation slot available."
    }
    WorkbenchActionButton {
        theme: controls.theme
        objectName: "directionStop"
        text: "■  Stop"
        Layout.preferredWidth: controls.actionWidth
        enabled: controls.busy && controls.contextCurrent
        onClicked: controls.actionRequested("stop", coverage.value.direction, coverage.value.identity)
        Accessible.name: "Stop simulation"
    }
    WorkbenchActionButton {
        theme: controls.theme
        objectName: "directionPause"
        text: controls.paused ? "▶  Resume" : "Ⅱ  Pause"
        Layout.preferredWidth: controls.actionWidth
        enabled: controls.busy && controls.contextCurrent
        onClicked: controls.actionRequested("pause", coverage.value.direction, coverage.value.identity)
        Accessible.name: controls.paused ? "Resume simulation" : "Pause simulation"
    }
}
