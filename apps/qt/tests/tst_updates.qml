/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../qml/shell"
import "../qml/theme"
Pane {
    id:window;width:960;height:640;visible:true
    Theme {id:theme}
    QtObject {
        id:backend
        property var current:({enabled:true,busy:false,available:false,status:"No release yet.",url:""})
        function poll(){return JSON.stringify(current)}
        function check(){current={enabled:current.enabled,busy:false,available:true,status:"MERLIN 0.2.0 is available.",url:"https://github.com/mgarcia01752/merlin/releases/tag/v0.2.0"};return JSON.stringify(current)}
        function set_enabled(value){current.enabled=value;return JSON.stringify(current)}
    }
    UpdateNotification {id:updates;anchors.fill:parent;theme:theme;backend:backend;startupCheck:false}
    SignalSpy {id:releaseSpy;target:updates;signalName:"releaseRequested"}
    TestCase {
        name:"Updates";when:windowShown
        function test_manual_check_notification_preference_and_release(){
            verify(!findChild(updates,"updateAvailable").visible)
            updates.showUpdates();tryCompare(updates.popup,"opened",true)
            mouseClick(findChild(updates,"checkUpdates"))
            verify(updates.state.available)
            verify(findChild(updates,"updateAvailable").visible)
            mouseClick(findChild(updates,"openUpdateRelease"))
            compare(releaseSpy.count,1)
            compare(releaseSpy.signalArguments[0][0].toString(),backend.current.url)
            mouseClick(findChild(updates,"startupUpdateCheck"))
            compare(updates.state.enabled,false)
            updates.popup.close();tryCompare(updates.popup,"opened",false)
        }
    }
}
