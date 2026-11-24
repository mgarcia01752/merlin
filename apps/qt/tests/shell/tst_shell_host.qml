/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../../qml/shell/directions"
import "../../qml/shared/theme"
import "../fixtures"

Item {
    id: window
    width: 1240
    height: 300
    // Independent logical fixture limits, distinct from production desktop policy.
    readonly property int fixtureLogicalBytes: 100000
    readonly property int fixtureEntries: 10000
    readonly property int fixtureViews: 16
    readonly property int fixtureOpenViews: 4
    PresentationBudget {
        id: budget
        // Independent capacity for these small routing/text fixtures, not desktop policy.
        maximumBytes: window.fixtureLogicalBytes
        maximumEntries: window.fixtureEntries
        maximumViews: window.fixtureViews
        maximumOpen: window.fixtureOpenViews
    }
    Theme { id: fixtureTheme; presentationBackendFactory: budget.createAdapter }
    QtObject {
        id: metadata
        property string selected: "downstream"
        property string downstreamIdentity: "downstream/18446744073709551614/9007199254740993"
        property string upstreamIdentity: "upstream/18446744073709551613/9007199254740995"
        function state() {
            return JSON.stringify({selectedDirection:selected,downstreamIdentity:downstreamIdentity,
                upstreamIdentity:upstreamIdentity,downstreamDirty:false,upstreamDirty:false,
                executionDirection:downstream.busy ? "downstream" : upstream.busy ? "upstream" : null,
                executionStatus:"",applicationBusy:downstream.busy || upstream.busy})
        }
        function select(direction) { selected=direction; return true }
        function accepts(direction, identity) {
            return direction === selected && identity === (direction === "downstream"
                ? downstreamIdentity : upstreamIdentity)
        }
    }
    Component {
        id: engineComponent
        QtObject {
            property bool busy: false
            property bool paused: false
            property bool canStart: true
            property string startError: ""
            property bool voidControls: false
            property string reason: ""
            property string resultToken: "original"
            property var timing: null
            property bool finishImmediately: false
            property var calls: []
            function state() { return poll() }
            function poll() { return JSON.stringify({configured:true,preparing:false,busy:busy,
                can_start:canStart && !busy,has_result:false,result:null,error:"",progress:null,
                reason:reason,result_token:resultToken,timing:timing}) }
            function start() {
                calls=calls.concat(["start"])
                reason=startError
                if (!startError) {
                    busy=!finishImmediately
                    resultToken="new run"
                    timing={completed:true}
                }
                // ClosedLoop Start/Stop/Pause return void; UpstreamSession returns text.
                if (!voidControls) return startError
            }
            function pause() { calls=calls.concat(["pause"]); paused=!paused; return "" }
            function stop() { calls=calls.concat(["stop"]); busy=false; return "" }
        }
    }
    property var downstream: null
    property var upstream: null
    QtObject {
        id: downstreamFixture
        property bool hasProject: true
        property bool allBlocksComplete: true
        property bool loadingProject: false
        property int configured: 0
        property int polled: 0
        function configureProject() { configured+=1 }
        function pollRun() { polled+=1 }
    }
    QtObject {
        id: upstreamFixture
        property string openedSection: ""
        function openSection(section) { openedSection=section }
    }
    Component {
        id: hostFactory
        DirectionShellHost {
            theme: fixtureTheme
            backend: metadata
            downstreamBackend: window.downstream
            upstreamBackend: window.upstream
            downstreamView: downstreamFixture
            upstreamView: upstreamFixture
        }
    }
    Component {
        id: controlsFactory
        DirectionRunControls {
            theme: fixtureTheme
            direction: "downstream"
            identity: metadata.downstreamIdentity
            canConfigure: true
            canStart: true
            busy: true
            status: "Accepted downstream status"
            property var requests: []
            property var helpRequests: []
            onActionRequested: function(action, direction, identity) {
                requests=requests.concat([{action:action,direction:direction,identity:identity}])
            }
            onHelpRequested: function(code, direction, identity) {
                helpRequests=helpRequests.concat([{code:code,direction:direction,identity:identity}])
            }
        }
    }
    TestCase {
        id: cases
        name: "DirectionShellHost"
        when: windowShown
        function init() {
            compare(budget.bytes,0)
            compare(budget.entries,0)
            compare(budget.views,0)
            budget.maximumBytes=window.fixtureLogicalBytes
            metadata.selected="downstream"
            metadata.downstreamIdentity="downstream/18446744073709551614/9007199254740993"
            downstream=createTemporaryObject(engineComponent,window)
            downstream.voidControls=true
            upstream=createTemporaryObject(engineComponent,window)
            downstreamFixture.hasProject=true
            downstreamFixture.allBlocksComplete=true
            downstreamFixture.loadingProject=false
            downstreamFixture.configured=0
            upstreamFixture.openedSection=""
        }
        function host() {
            const result=createTemporaryObject(hostFactory,window)
            verify(result!==null)
            return result
        }
        function test_only_current_selected_origin_reaches_its_native_owner() {
            const value=host()
            verify(value.runAction("configure","downstream",metadata.downstreamIdentity))
            compare(downstreamFixture.configured,1)
            verify(value.selectDirection("upstream"))
            verify(!value.runAction("start","downstream",metadata.downstreamIdentity))
            compare(downstream.calls.length,0)
            verify(value.runAction("configure","upstream",metadata.upstreamIdentity))
            compare(upstreamFixture.openedSection,"ucd")
            verify(value.runAction("start","upstream",metadata.upstreamIdentity))
            compare(upstream.calls,["start"])
            verify(value.runAction("pause","upstream",metadata.upstreamIdentity))
            verify(value.runAction("stop","upstream",metadata.upstreamIdentity))
            compare(upstream.calls,["start","pause","stop"])
            compare(downstream.calls.length,0)
        }
        function test_stale_key_and_downstream_readiness_never_dispatch() {
            const value=host()
            const old=metadata.downstreamIdentity
            metadata.downstreamIdentity=old+"/changed"
            verify(!value.runAction("start","downstream",old))
            compare(downstream.calls.length,0)
            verify(value.poll())
            downstreamFixture.allBlocksComplete=false
            verify(!value.runAction("start","downstream",metadata.downstreamIdentity))
            compare(downstream.calls.length,0)
            downstreamFixture.allBlocksComplete=true
            verify(value.runAction("start","downstream",metadata.downstreamIdentity))
            compare(downstream.calls,["start"])
        }
        function test_rejected_selection_preserves_native_and_visible_context() {
            const value=host()
            const old=value.presentation
            budget.maximumBytes=budget.bytes
            verify(!value.selectDirection("upstream"))
            compare(value.presentation,old)
            compare(value.presentation.selectedDirection,"downstream")
            compare(metadata.selected,"downstream")
            verify(fixtureTheme.presentationAdmissionRejected)
        }
        function test_native_success_and_controls_survive_optional_detail_rejection() {
            const value=host()
            budget.maximumBytes=budget.bytes
            verify(value.runAction("start","downstream",metadata.downstreamIdentity))
            compare(downstream.calls,["start"])
            verify(value.presentationRejected)
            verify(value.presentation.applicationBusy)
            verify(value.runAction("pause","downstream",metadata.downstreamIdentity))
            verify(value.runAction("stop","downstream",metadata.downstreamIdentity))
            compare(downstream.calls,["start","pause","stop"])
        }
        function test_native_error_is_not_reported_as_success() {
            const value=host()
            downstream.startError="Original native admission error"
            verify(!value.runAction("start","downstream",metadata.downstreamIdentity))
            compare(value.presentation.feedbackMessage,"Original native admission error")
            verify(value.presentation.feedbackIsError)
        }
        function test_void_downstream_start_can_complete_before_its_first_poll() {
            const value=host()
            downstream.finishImmediately=true
            verify(value.runAction("start","downstream",metadata.downstreamIdentity))
            compare(downstream.busy,false)
            compare(downstream.resultToken,"new run")
            compare(value.presentation.feedbackMessage,"")
        }
        function test_controls_keep_live_stop_but_reject_a_new_context_under_pressure() {
            const controls=createTemporaryObject(controlsFactory,window)
            verify(controls!==null)
            tryCompare(controls,"contextCurrent",true)
            const stop=findChild(controls,"directionStop")
            const start=findChild(controls,"directionStart")
            const original=controls.presentationCoverage.value
            budget.maximumBytes=budget.bytes
            controls.status="A rejected optional status replacement"
            tryCompare(controls.presentationCoverage,"rejected",true)
            compare(controls.presentationCoverage.value,original)
            compare(stop.enabled,true)
            compare(start.enabled,false)
            budget.maximumBytes=window.fixtureLogicalBytes
            controls.retryRejectedText()
            tryCompare(controls.presentationCoverage,"rejected",false)
            compare(stop.enabled,true)
            compare(start.enabled,true)
            budget.maximumBytes=budget.bytes
            controls.direction="upstream"
            controls.identity=metadata.upstreamIdentity
            tryCompare(controls,"contextCurrent",false)
            compare(stop.enabled,false)
            compare(start.enabled,false)
            compare(controls.requests.length,0)
        }
        function test_admitted_contextual_status_keeps_keyboard_help_and_activity() {
            const controls=createTemporaryObject(controlsFactory,window)
            verify(controls!==null)
            controls.diagnostic={code:"fixture.help",tooltip:"Original bounded diagnostic"}
            controls.activityTooltip="Original progress age"
            const progress=findChild(controls,"directionRunProgress")
            tryCompare(progress,"hasHelp",true)
            progress.forceActiveFocus(Qt.TabFocusReason)
            keyClick(Qt.Key_F1)
            compare(controls.helpRequests,[{code:"fixture.help",direction:"downstream",
                identity:metadata.downstreamIdentity}])
            controls.diagnostic=null
            tryCompare(progress,"hasHelp",false)
            compare(progress.activityTooltip,"Original progress age")
        }
    }
}
