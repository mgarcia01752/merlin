/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtTest
import "fixtures"
import "../qml/shell"
import "../qml/shared/theme"
Pane {
    id:window;width:960;height:640;visible:true
    PresentationBudget {
        id: presentationBudget
        // Independent capacities for existing presentation-only fixture graphs.
        maximumBytes: 10000000
        maximumEntries: 100000
        maximumViews: 96
        maximumOpen: 12
    }
    Theme { presentationBackendFactory: presentationBudget.createAdapter;id:theme}
    ThemePalette { targetControl: window; theme: theme }
    ThemePalette { targetControl: window.Window.window; theme: theme }
    ThemePalette { targetControl: updates.popup; theme: theme }
    TextEdit { id: clipboardProbe; visible: false }
    QtObject {
        id:backend
        property var current:({enabled:true,busy:false,available:false,status:"No release yet."})
        property int installs:0
        function issue_report(){return JSON.stringify({text:"Diagnostic report",url:"https://github.com/mgarcia01752/merlin/issues/new"})}
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
        function init(){updates.preparingRecovery=false;updates.pollRecovery=function(){return {error:"No pending recovery"}};backend.current={enabled:true,busy:false,available:false,status:"No release yet."};backend.installs=0;updates.state=JSON.parse(backend.poll());updates.canUpdate=true;updates.prepareUpdate=function(){return {saved:true}}}
        function cleanup(){updates.preparingRecovery=false;updates.popup.close()}
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
        function test_queued_recovery_waits_for_actual_completion_before_install() {
            updates.prepareUpdate = function() { return {preparing:true} }
            updates.pollRecovery = function() { return {preparing:true} }
            updates.installUpdate()
            verify(updates.preparingRecovery)
            wait(150)
            compare(backend.installs, 0)
            updates.installUpdate()
            compare(backend.installs, 0)
            updates.pollRecovery = function() { return {saved:true,preparing:false} }
            tryCompare(backend, "installs", 1)
            verify(!updates.preparingRecovery)
        }
        function test_async_recovery_failure_and_incomplete_response_never_install() {
            updates.prepareUpdate = function() { return {preparing:true} }
            updates.pollRecovery = function() { return {error:"Recovery write failed"} }
            updates.installUpdate()
            tryCompare(updates, "preparingRecovery", false)
            compare(backend.installs, 0)
            compare(updates.state.status, "Recovery write failed")
            updates.prepareUpdate = function() { return {} }
            updates.installUpdate()
            compare(backend.installs, 0)
            verify(updates.state.error)
        }
        function test_changed_run_admission_during_recovery_prevents_install() {
            updates.prepareUpdate = function() { return {preparing:true} }
            updates.pollRecovery = function() { return {preparing:true} }
            updates.installUpdate()
            updates.canUpdate = false
            updates.pollRecovery = function() { return {saved:true} }
            tryCompare(updates, "preparingRecovery", false)
            compare(backend.installs, 0)
            verify(updates.state.error)
        }
        function test_windows_update_saves_recovery_and_starts_installer(){
            let recovered=false
            updates.prepareUpdate=function(){recovered=true;return {saved:true}}
            updates.state={enabled:true,busy:false,available:true,windows:true}
            updates.showUpdates();tryCompare(updates.popup,"opened",true)
            const action=findChild(updates,"installUpdate")
            compare(action.text,"Update")
            mouseClick(action)
            verify(recovered);compare(backend.installs,1)
            verify(updates.state.installing)
            updates.installUpdate();compare(backend.installs,1)
        }
        function test_error_report_requires_click_and_handles_browser_failure(){
            let opened=0
            updates.openIssue=function(url){opened++;return false}
            updates.state={enabled:true,busy:false,available:false,error:true,status:"Update failed (Windows Installer code 1619). Your workbench was restored."}
            updates.showUpdates();tryCompare(updates.popup,"opened",true)
            const action=findChild(updates,"reportUpdateIssue")
            verify(action.visible);compare(opened,0)
            wait(150)
            grabImage(updates.popup.contentItem.parent).save("update-error-dark.png")
            theme.dark=false;wait(150)
            grabImage(updates.popup.contentItem.parent).save("update-error-light.png")
            theme.dark=true
            mouseClick(action);compare(opened,1)
            clipboardProbe.text="";clipboardProbe.paste()
            compare(clipboardProbe.text,"Diagnostic report")
            verify(updates.popup.reportFeedback.indexOf("Browser could not open")>=0)
            updates.state={enabled:true,busy:false,available:false,error:false}
            verify(!action.visible)
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
