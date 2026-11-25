/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../shared/components"
Item {
    id: control
    required property var theme
    required property var backend
    property bool startupCheck: true
    property var state: ({enabled:true,busy:false,available:false,status:"",url:""})
    property alias popup: dialog
    property bool canUpdate: true
    property bool preparingRecovery: false
    // Independent application scheduling policies, not simulation timing.
    readonly property int startupUpdateDelayMs: 2000
    readonly property int updateStatusPollIntervalMs: 150
    property var pollRecovery: function() { return {error:"Recovery completion is unavailable."} }
    property var prepareUpdate: function() { return {saved:true} }
    property var openIssue: function(url) { return Qt.openUrlExternally(url) }
    TextEdit { id: reportClipboard; visible: false; textFormat: TextEdit.PlainText }
    function reportIssue() {
        if (state.error !== true || state.busy) { return }
        const report = JSON.parse(backend.issue_report(state.status || "Update failed"))
        if (report.error) { dialog.reportFeedback = report.error; return }
        reportClipboard.text = report.text
        reportClipboard.selectAll()
        reportClipboard.copy()
        reportClipboard.deselect()
        dialog.reportFeedback = openIssue(report.url)
            ? "Report copied. Paste it into GitHub, review for private information, then submit."
            : "Report copied. Browser could not open; visit github.com/mgarcia01752/merlin/issues to paste and review it."
    }
    /** Keep the live app open and show the bounded native recovery failure. */
    function recoveryFailed(message) {
        preparingRecovery = false
        state = Object.assign({}, state, {busy:false,status:message,error:true})
        dialog.openChecked()
    }
    /** Consume a native completion; queued preparation never starts installation. */
    function finishRecovery(recovery) {
        if (recovery.error) {
            recoveryFailed(recovery.error)
        } else if (recovery.preparing === true) {
            preparingRecovery = true
            state = Object.assign({}, state, {busy:true,status:"Saving both projects before update",error:false})
            dialog.openChecked()
        } else if (recovery.saved === true && canUpdate) {
            preparingRecovery = false
            state = JSON.parse(backend.install())
            dialog.openChecked()
        } else {
            recoveryFailed("Update recovery did not complete. Finish pending work and retry.")
        }
    }
    /** Request one recovery export before asking the installed updater to proceed. */
    function installUpdate() {
        if (!canUpdate || state.busy || preparingRecovery) {
            return
        }
        finishRecovery(prepareUpdate())
    }
    /** Check release metadata only when it cannot race with recovery preparation. */
    function refresh() {
        if (preparingRecovery) {
            return
        }
        dialog.reportFeedback = ""
        state = JSON.parse(backend.check())
    }
    function showUpdates() { dialog.openChecked() }
    Component.onCompleted: {state=JSON.parse(backend.poll());if(state.show_result) dialog.openChecked()}
    Timer { interval:control.startupUpdateDelayMs;running:control.startupCheck;repeat:false;onTriggered: {if(control.state.enabled && !control.state.show_result && !control.state.busy && !control.preparingRecovery) control.refresh()} }
    // Existing cadence polls download/install status, independently of
    // the shared native-session cadence used by recovery completion below.
    Timer { interval:control.updateStatusPollIntervalMs;running:control.state.busy && !control.preparingRecovery;repeat:true;onTriggered: {
        control.state=JSON.parse(control.backend.poll())
        if (control.state.restarted) { Qt.quit() }
    } }
    // This poll consumes only bounded native completion tags; it never starts install early.
    Timer {
        interval: control.theme.sessionPollIntervalMs
        running: control.preparingRecovery
        repeat: true
        onTriggered: control.finishRecovery(control.pollRecovery())
    }
    WorkbenchActionButton {
        theme: control.theme
        objectName:"updateAvailable"
        anchors.right:parent.right;anchors.top:parent.top
        visible:!!(control.state.available || control.state.installing)
        text:"Update";font.bold:true
        enabled:control.canUpdate && !control.state.busy
        compact:true;height:28;implicitWidth:80
        Accessible.name:"A MERLIN update is available"
        onClicked:control.installUpdate()
        contentItem: Label { text:parent.text; color:"white"; font.bold:true; horizontalAlignment:Text.AlignHCenter; verticalAlignment:Text.AlignVCenter }
        background: Rectangle { radius:9; color:parent.hovered ? "#2ea043" : "#238636"; border.color:parent.activeFocus ? "white" : "#2ea043"; border.width:parent.activeFocus ? 2 : 1 }
        ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered
        ToolTip.text: control.canUpdate ? "Install the available update and restart MERLIN" : "Stop the simulation before updating"
    }
    UpdateDialog {
        id:dialog;theme:control.theme;state:control.state
        onCheckRequested:control.refresh()
        onPreferenceRequested:function(enabled){control.state=JSON.parse(control.backend.set_enabled(enabled))}
        onInstallRequested:control.installUpdate()
        onReportRequested:control.reportIssue()
        canInstall:control.canUpdate
    }
}
