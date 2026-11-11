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
                Button {objectName:"checkUpdates";text:dialog.state.busy ? "Checking…" : "Check For Updates";enabled:!dialog.state.busy;onClicked:dialog.checkRequested()}
                Button {objectName:"installUpdate";text:"Update";visible:dialog.state.available;enabled:dialog.canInstall && !dialog.state.busy;onClicked:dialog.installRequested()}
            }
            Label {visible:dialog.state.available;text:"Ubuntu may request administrator approval. MERLIN restarts after installation.";Layout.fillWidth:true;wrapMode:Text.WordWrap;color:dialog.theme.muted}
        }
    }

