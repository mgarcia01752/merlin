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
    signal releaseRequested(url target)
    function refresh() { state=JSON.parse(backend.check()) }
    function showUpdates() { dialog.open() }
    Component.onCompleted: state=JSON.parse(backend.poll())
    Timer { interval:2000;running:control.startupCheck;repeat:false;onTriggered: {if(control.state.enabled) control.refresh()} }
    Timer { interval:150;running:control.state.busy;repeat:true;onTriggered:control.state=JSON.parse(control.backend.poll()) }
    Button {
        objectName:"updateAvailable"
        anchors.right:parent.right;anchors.top:parent.top
        visible:control.state.available
        text:"Update Available";font.bold:true
        Accessible.name:"A MERLIN update is available"
        onClicked:control.showUpdates()
        palette.button:control.theme.statusComplete;palette.buttonText:control.theme.background
    }
    UpdateDialog {
        id:dialog;theme:control.theme;state:control.state
        onCheckRequested:control.refresh()
        onPreferenceRequested:function(enabled){control.state=JSON.parse(control.backend.set_enabled(enabled))}
        onReleaseRequested:function(target){control.releaseRequested(target)}
    }
}
