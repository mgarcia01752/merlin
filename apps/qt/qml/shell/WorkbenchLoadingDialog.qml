/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../components"

WorkbenchDialog {
    id: dialog
    objectName: "workbenchLoadingDialog"
    property var loadState: ({phase:"",elapsed_ms:0,cancelling:false})
    signal cancelRequested()
    title: "Open Workbench"
    width: Math.min(500, parent ? parent.width - 32 : 500)
    height: 230
    x: parent ? (parent.width-width)/2 : 0
    y: parent ? (parent.height-height)/2 : 0
    modal: true
    closePolicy: Popup.NoAutoClose
    Shortcut { sequence:"Escape";enabled:dialog.visible && !dialog.loadState.cancelling;onActivated:dialog.cancelRequested() }
    header: Label { text: dialog.title; color: dialog.theme.text; font.bold:true; padding:12 }
    background: Rectangle { color:dialog.theme.background; border.color:dialog.theme.line; radius:12 }
    contentItem: ColumnLayout {
        spacing: 12
        RowLayout {
            spacing: 16
            BusyIndicator { running:dialog.visible; Layout.preferredWidth:40; Layout.preferredHeight:40 }
            Label { text:dialog.loadState.phase || "Opening workbench"; color:dialog.theme.text; wrapMode:Text.WordWrap; Layout.fillWidth:true }
        }
        Label { objectName:"loadElapsed"; text:"Elapsed: "+((dialog.loadState.elapsed_ms || 0)/1000).toFixed(1)+" s"; color:dialog.theme.muted }
        Item { Layout.fillHeight:true }
        WorkbenchActionButton {
            objectName:"cancelLoadButton"; theme:dialog.theme; text:"Cancel"
            Layout.alignment:Qt.AlignRight; enabled:!dialog.loadState.cancelling
            onClicked:dialog.cancelRequested()
        }
    }
}
