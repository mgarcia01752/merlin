/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtTest
import "../qml/shell"
import "../qml/theme"
Pane {
    id:window;width:960;height:640;visible:true
    Theme {id:theme}
    QtObject {
        id:backend
        property var current:({enabled:true,busy:false,available:false,status:"No release yet."})
        property int installs:0
        function poll(){return JSON.stringify(current)}
        function check(){current={enabled:current.enabled,busy:false,available:true,status:"MERLIN 0.2.0 is available."};return JSON.stringify(current)}
        function install(){installs++;current.busy=true;current.installing=true;current.status="Downloading update…";return JSON.stringify(current)}
        function set_enabled(value){current.enabled=value;return JSON.stringify(current)}
    }
    RowLayout {
        id:header;width:parent.width;height:32
        WorkbenchMenuBar { id:menu;theme:theme;Layout.fillWidth:true }
        UpdateNotification {id:updates;Layout.preferredWidth:80;Layout.preferredHeight:28;theme:theme;backend:backend;startupCheck:false}
    }
    TestCase {
        name:"Updates";when:windowShown
        function init(){backend.current={enabled:true,busy:false,available:false,status:"No release yet."};backend.installs=0;updates.state=JSON.parse(backend.poll());updates.canUpdate=true;updates.prepareUpdate=function(){return {saved:true}}}
        function cleanup(){updates.popup.close()}
        function test_chip_is_on_menu_right_and_starts_directly(){
            const chip=findChild(updates,"updateAvailable")
            verify(!chip.visible)
            updates.refresh()
            verify(chip.visible);compare(chip.text,"Update")
            compare(updates.parent,header)
            verify(updates.x>header.width/2)
            mouseClick(chip)
            compare(backend.installs,1)
            tryCompare(updates.popup,"opened",true)
            verify(updates.state.installing);verify(!chip.enabled)
            updates.installUpdate();compare(backend.installs,1)
        }
        function test_run_lock_and_recovery_failure_prevent_install(){
            updates.refresh();updates.canUpdate=false
            verify(!findChild(updates,"updateAvailable").enabled)
            updates.installUpdate();compare(backend.installs,0)
            updates.canUpdate=true
            updates.prepareUpdate=function(){return {error:"Recovery failed"}}
            updates.installUpdate();compare(backend.installs,0)
            compare(updates.state.status,"Recovery failed")
        }
        function test_manual_check_and_preference(){
            updates.showUpdates();tryCompare(updates.popup,"opened",true)
            mouseClick(findChild(updates,"checkUpdates"))
            verify(updates.state.available)
            mouseClick(findChild(updates,"startupUpdateCheck"))
            compare(updates.state.enabled,false)
            compare(findChild(updates,"installUpdate").text,"Update")
            verify(findChild(updates,"openUpdateRelease")===null)
        }
    }
}
