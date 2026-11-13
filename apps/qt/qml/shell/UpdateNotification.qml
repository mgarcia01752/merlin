/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../components"
Item {
    id: control
    required property var theme
    required property var backend
    property bool startupCheck: true
    property var state: ({enabled:true,busy:false,available:false,status:"",url:""})
    property alias popup: dialog
    property bool canUpdate: true
    property var openRelease: function(url) { Qt.openUrlExternally(url) }
    property var prepareUpdate: function() { return {saved:true} }
    function installUpdate() {
        if (!canUpdate || state.busy) { return }
        if (state.manual_update) {
            openRelease(state.url)
            return
        }
        const recovery = prepareUpdate()
        if (recovery.error) {
            state = Object.assign({}, state, {status:recovery.error})
            dialog.open()
            return
        }
        state = JSON.parse(backend.install())
        dialog.open()
    }
    function refresh() { state=JSON.parse(backend.check()) }
    function showUpdates() { dialog.open() }
    Component.onCompleted: state=JSON.parse(backend.poll())
    Timer { interval:2000;running:control.startupCheck;repeat:false;onTriggered: {if(control.state.enabled) control.refresh()} }
    Timer { interval:150;running:control.state.busy;repeat:true;onTriggered: {
        control.state=JSON.parse(control.backend.poll())
        if (control.state.restarted) { Qt.quit() }
    } }
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
        ToolTip.visible: hovered
        ToolTip.text: control.canUpdate ? (control.state.manual_update ? "Download the Windows MSI from Releases" : "Install the available update and restart MERLIN") : "Stop the simulation before updating"
    }
    UpdateDialog {
        id:dialog;theme:control.theme;state:control.state
        onCheckRequested:control.refresh()
        onPreferenceRequested:function(enabled){control.state=JSON.parse(control.backend.set_enabled(enabled))}
        onInstallRequested:control.installUpdate()
        canInstall:control.canUpdate
    }
}
