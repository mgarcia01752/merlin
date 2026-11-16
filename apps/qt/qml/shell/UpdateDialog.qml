/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../components"
WorkbenchDialog {
        id:dialog;title:"MERLIN Updates"
        required property var state
        signal checkRequested()
        signal preferenceRequested(bool enabled)
        signal installRequested()
        signal reportRequested()
        property string reportFeedback: ""
        property bool canInstall: true
        parent:Overlay.overlay;anchors.centerIn:parent
        width:Math.min(560,parent ? parent.width-48 : 560)
        modal:true;standardButtons:Dialog.Close
        contentItem:ColumnLayout {
            spacing:16
            Label {text:dialog.state.status || "Check for a newer MERLIN release.";textFormat:Text.PlainText;Layout.fillWidth:true;wrapMode:Text.WordWrap;color:dialog.theme.text}
            CheckBox {
                objectName:"startupUpdateCheck";text:"Check For Updates At Startup";checked:dialog.state.enabled
                onToggled:dialog.preferenceRequested(checked)
            }
            RowLayout {
                WorkbenchActionButton { theme: dialog.theme;objectName:"checkUpdates";text:dialog.state.busy ? "Checking…" : "Check For Updates";enabled:!dialog.state.busy;onClicked:dialog.checkRequested()}
                WorkbenchActionButton { theme: dialog.theme;objectName:"installUpdate";text:"Update";visible:dialog.state.available;enabled:dialog.canInstall && !dialog.state.busy;onClicked:dialog.installRequested()}
            }
            WorkbenchActionButton {
                theme: dialog.theme; objectName: "reportUpdateIssue"; text: "Report Issue…"
                visible: dialog.state.error === true; enabled: !dialog.state.busy
                onClicked: dialog.reportRequested()
                ToolTip.delay: dialog.theme.tooltipDelayMs; ToolTip.visible: hovered
                ToolTip.text: "Copy an update diagnostic report and open GitHub. Review and submit it yourself."
            }
            Label { visible: dialog.reportFeedback.length > 0; text: dialog.reportFeedback; textFormat: Text.PlainText; Layout.fillWidth: true; wrapMode: Text.WordWrap; color: dialog.theme.muted }
            Label {visible:dialog.state.available;text:dialog.state.windows ? "MERLIN saves your workbench, downloads and verifies the MSI, then closes for installation. Approve the Windows administrator prompt. MERLIN reopens afterward; Windows is never restarted automatically.":"Ubuntu may request administrator approval. MERLIN restarts after installation.";Layout.fillWidth:true;wrapMode:Text.WordWrap;color:dialog.theme.muted}
        }
    }

