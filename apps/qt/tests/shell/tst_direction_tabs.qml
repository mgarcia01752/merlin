/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
pragma ComponentBehavior: Bound
import QtQuick
import QtQuick.Window
import QtTest
import "../../qml/shell/directions"
import "../../qml/shared/theme"

Item {
    id: window
    // Dimensions exercise presentation layouts, not backend/runtime limits.
    readonly property int wideWidth: 1240
    readonly property int narrowWidth: 360
    readonly property int fixtureHeight: 620
    width: wideWidth
    height: fixtureHeight
    visible: true
    Theme { id: testTheme }

    Component {
        id: shellFactory
        PersistentDirectionTabs {
            id: tabs
            width: window.width
            height: window.height
            theme: testTheme
            property alias hostObject: host
            property var runRequests: []
            property var saves: []
            property var openTargets: []
            property int openCount: 0
            property var selectionRequests: []
            property bool acceptSelection: true
            presentationHost: QtObject {
                id: host
                property string selectedDirection: "downstream"
                // String anchors exceed JavaScript's exact integer range on purpose.
                property var downstreamIdentity: "ds/session/18446744073709551614/generation/9007199254740993"
                property var upstreamIdentity: "us/session/18446744073709551613/generation/9007199254740995"
                property string executionDirection: ""
                property string executionStatus: ""
                property string feedbackMessage: ""
                property bool feedbackIsError: false
                // Predetermined host response; no simulation dispatch is emulated.
                property string nextFeedback: ""
            }
            downstreamComponent: Component {
                DirectionPresentationFixture {
                    theme: testTheme
                    direction: "downstream"
                    tabHost: tabs
                }
            }
            upstreamComponent: Component {
                DirectionPresentationFixture {
                    theme: testTheme
                    direction: "upstream"
                    tabHost: tabs
                }
            }
            onDirectionRequested: function(direction) {
                selectionRequests = selectionRequests.concat([direction])
                if (acceptSelection) {
                    host.selectedDirection = direction
                }
            }
            onRunActionRequested: function(action, direction, identity) {
                runRequests = runRequests.concat([{action: action, direction: direction, identity: identity}])
                host.feedbackMessage = host.nextFeedback
                host.feedbackIsError = host.nextFeedback.length > 0
            }
            onSaveRequested: function(direction, identity, saveAs) {
                saves = saves.concat([{direction: direction, identity: identity, saveAs: saveAs}])
            }
            onOpenRequested: openCount++
            onOpenTargetRequested: function(direction, guard) {
                openTargets = openTargets.concat([{direction: direction, guard: guard}])
            }
        }
    }

    TestCase {
        id: tests
        name: "PersistentDirectionPresentation"
        when: windowShown

        function init() {
            window.width = window.wideWidth
            testTheme.dark = true
        }

        function createShell() {
            const shell = createTemporaryObject(shellFactory, window)
            verify(shell !== null)
            tryVerify(function() { return shell.downstreamItem !== null && shell.upstreamItem !== null })
            verify(waitForRendering(shell))
            return shell
        }

        function selectWithPointer(shell, direction) {
            const tab = findChild(shell, direction + "DirectionTab")
            verify(tab !== null)
            mouseClick(tab)
            tryCompare(shell, "selectedDirection", direction)
            verify(waitForRendering(shell))
        }

        function test_children_persist_during_idle_running_and_paused_switches() {
            const shell = createShell()
            const downstream = shell.downstreamItem
            const upstream = shell.upstreamItem
            downstream.firstField.text = "Downstream edited presentation text"
            downstream.selectedPanel = "Downstream fixture selection"
            downstream.viewportFraction = 0.75
            upstream.firstField.text = "Upstream edited presentation text"
            upstream.selectedPanel = "Upstream fixture selection"
            upstream.viewportFraction = 0.5
            const states = [
                {owner: "", status: ""},
                {owner: "downstream", status: "Presentation fixture: Downstream running"},
                {owner: "downstream", status: "Presentation fixture: Downstream paused"},
                {owner: "upstream", status: "Presentation fixture: Upstream running"},
                {owner: "upstream", status: "Presentation fixture: Upstream paused"}
            ]
            for (const state of states) {
                shell.hostObject.executionDirection = state.owner
                shell.hostObject.executionStatus = state.status
                selectWithPointer(shell, "upstream")
                compare(shell.downstreamItem, downstream)
                compare(shell.upstreamItem, upstream)
                compare(downstream.visible, false)
                compare(downstream.enabled, false)
                compare(upstream.visible, true)
                compare(upstream.enabled, true)
                compare(shell.runRequests.length, 0)
                compare(shell.hostObject.executionDirection, state.owner)
                compare(shell.hostObject.executionStatus, state.status)
                selectWithPointer(shell, "downstream")
                compare(downstream.firstField.text, "Downstream edited presentation text")
                compare(upstream.firstField.text, "Upstream edited presentation text")
                compare(downstream.selectedPanel, "Downstream fixture selection")
                compare(upstream.selectedPanel, "Upstream fixture selection")
                compare(downstream.viewportFraction, 0.75)
                compare(upstream.viewportFraction, 0.5)
            }
        }

        function test_actions_and_save_keep_selected_direction_and_opaque_identity() {
            const shell = createShell()
            mouseClick(findChild(shell.downstreamItem, "downstreamStartIntent"))
            compare(shell.runRequests.length, 1)
            compare(shell.runRequests[0].action, "start")
            compare(shell.runRequests[0].direction, "downstream")
            compare(shell.runRequests[0].identity, shell.hostObject.downstreamIdentity)
            const opaque = {session: "18446744073709551613", generation: "9007199254740995"}
            shell.hostObject.upstreamIdentity = opaque
            // Explicit adoption refreshes the fixture's displayed-state identity.
            shell.upstreamItem.sourceIdentity = opaque
            selectWithPointer(shell, "upstream")
            mouseClick(findChild(shell.upstreamItem, "upstreamPauseIntent"))
            compare(shell.runRequests.length, 2)
            compare(shell.runRequests[1].direction, "upstream")
            verify(shell.runRequests[1].identity === opaque)
            mouseClick(findChild(shell.upstreamItem, "upstreamSaveIntent"))
            compare(shell.saves.length, 1)
            compare(shell.saves[0].direction, "upstream")
            verify(shell.saves[0].identity === opaque)
            compare(shell.saves[0].saveAs, false)
            verify(shell.requestSave(true))
            compare(shell.saves[1].saveAs, true)
            compare(shell.requestRunAction("stop", "downstream", shell.hostObject.downstreamIdentity), false)
            compare(shell.requestSave(false, "downstream", shell.hostObject.downstreamIdentity), false)
            compare(shell.runRequests.length, 2)
            compare(shell.saves.length, 2)
        }

        function test_paused_other_owner_and_busy_error_remain_host_supplied() {
            const shell = createShell()
            shell.hostObject.executionDirection = "downstream"
            shell.hostObject.executionStatus = "Presentation fixture: Downstream paused"
            shell.hostObject.nextFeedback = "Presentation fixture: Busy; Downstream retains execution"
            selectWithPointer(shell, "upstream")
            const status = findChild(shell, "otherDirectionExecutionStatus")
            compare(status.visible, true)
            compare(status.text, shell.hostObject.executionStatus)
            mouseClick(findChild(shell.upstreamItem, "upstreamStartIntent"))
            compare(shell.runRequests.length, 1)
            compare(shell.runRequests[0].action, "start")
            compare(shell.runRequests[0].direction, "upstream")
            compare(shell.hostObject.executionDirection, "downstream")
            const feedback = findChild(shell, "directionActionFeedback")
            compare(feedback.text, shell.hostObject.nextFeedback)
            compare(feedback.color, testTheme.statusError)
            compare(feedback.visible, true)
            shell.hostObject.nextFeedback = "Presentation fixture: Host rejected this action"
            mouseClick(findChild(shell.upstreamItem, "upstreamPauseIntent"))
            compare(shell.runRequests[1].direction, "upstream")
            compare(feedback.text, shell.hostObject.nextFeedback)
            window.width = window.narrowWidth
            for (const dark of [true, false]) {
                testTheme.dark = dark
                verify(waitForRendering(shell))
                verify(status.height >= status.contentHeight)
                verify(feedback.height >= feedback.contentHeight)
                verify(shell.upstreamItem.height > 0)
                grabImage(shell).save("tabs-busy-"
                    + (dark ? "dark" : "light") + "-narrow.png")
            }
            selectWithPointer(shell, "downstream")
            compare(status.visible, false)
            compare(shell.runRequests.length, 2)
        }

        function test_open_uses_explicit_detected_target_and_preserves_guard() {
            const shell = createShell()
            const guard = {session: "18446744073709551613", revision: "9007199254740997", dirty: true}
            verify(shell.requestOpen())
            compare(shell.openCount, 1)
            verify(shell.requestOpenTarget("upstream", guard))
            compare(shell.openTargets.length, 1)
            compare(shell.openTargets[0].direction, "upstream")
            verify(shell.openTargets[0].guard === guard)
            compare(shell.selectedDirection, "downstream")
            compare(shell.requestOpenTarget("", guard), false)
            compare(shell.requestOpenTarget("downstream", null), false)
            compare(shell.requestOpenTarget("downstream", 9007199254740992), false)
            compare(shell.openTargets.length, 1)
        }

        function test_missing_selection_or_lossy_identity_never_defaults_or_emits() {
            const shell = createShell()
            shell.hostObject.selectedDirection = "unavailable"
            compare(shell.selectedDirection, "")
            compare(shell.scopedActionsAvailable, false)
            compare(shell.requestRunAction("start"), false)
            compare(shell.requestSave(false), false)
            compare(shell.downstreamItem.enabled, false)
            compare(shell.upstreamItem.enabled, false)
            shell.hostObject.selectedDirection = "upstream"
            for (const identity of [undefined, null, "", false, 9007199254740992]) {
                shell.hostObject.upstreamIdentity = identity
                compare(shell.scopedActionsAvailable, false)
                compare(shell.requestRunAction("pause"), false)
                compare(shell.requestSave(true), false)
                compare(shell.upstreamItem.enabled, false)
            }
            compare(shell.runRequests.length, 0)
            compare(shell.saves.length, 0)
            shell.presentationHost = {}
            compare(shell.selectedDirection, "")
            compare(shell.requestRunAction("stop"), false)
            compare(shell.requestSave(false), false)
            shell.enabled = false
            compare(shell.scopedActionsAvailable, false)
            compare(shell.requestOpen(), false)
            compare(shell.openCount, 0)
            compare(shell.requestRunAction("stop"), false)
            shell.presentationHost = null
            compare(shell.selectedDirection, "")
            compare(shell.requestRunAction("stop"), false)
            compare(shell.requestSave(false), false)
            compare(shell.requestDirection("unknown"), false)
            compare(shell.selectionRequests.length, 0)
        }

        function test_keyboard_focus_returns_to_last_valid_selected_child() {
            const shell = createShell()
            const downstream = shell.downstreamItem
            const upstream = shell.upstreamItem
            mouseClick(downstream.secondField)
            tryCompare(downstream.secondField, "activeFocus", true)
            selectWithPointer(shell, "upstream")
            mouseClick(upstream.firstField)
            tryCompare(upstream.firstField, "activeFocus", true)
            const upstreamTab = findChild(shell, "upstreamDirectionTab")
            upstreamTab.forceActiveFocus(Qt.TabFocusReason)
            keyClick(Qt.Key_Left)
            tryCompare(shell, "selectedDirection", "downstream")
            tryCompare(downstream.secondField, "activeFocus", true)
            compare(upstream.activeFocus, false)
            const downstreamTab = findChild(shell, "downstreamDirectionTab")
            downstreamTab.forceActiveFocus(Qt.TabFocusReason)
            keyClick(Qt.Key_Right)
            tryCompare(shell, "selectedDirection", "upstream")
            tryCompare(upstream.firstField, "activeFocus", true)
            downstream.extraFocusEnabled = false
            tryCompare(downstream, "secondField", null)
            selectWithPointer(shell, "downstream")
            tryVerify(function() {
                return shell.belongsTo(window.Window.window.activeFocusItem, downstream)
            })
            compare(upstream.activeFocus, false)
        }

        function test_stale_child_identity_is_rejected_without_relabeling_global_context() {
            const shell = createShell()
            const oldIdentity = shell.downstreamItem.sourceIdentity
            const replacement = "ds/session/18446744073709551612/generation/9007199254740997"
            shell.hostObject.downstreamIdentity = replacement
            mouseClick(findChild(shell.downstreamItem, "downstreamStartIntent"))
            compare(shell.runRequests.length, 0)
            compare(shell.requestRunAction("pause", "downstream", oldIdentity), false)
            compare(shell.requestSave(false, "downstream", oldIdentity), false)
            compare(shell.saves.length, 0)
            verify(shell.requestRunAction("start"))
            compare(shell.runRequests[0].identity, replacement)
            // An accepted view adoption updates its captured context explicitly.
            shell.downstreamItem.sourceIdentity = replacement
            mouseClick(findChild(shell.downstreamItem, "downstreamStartIntent"))
            compare(shell.runRequests.length, 2)
            compare(shell.runRequests[1].identity, replacement)
            compare(shell.requestRunAction("pause", "downstream", oldIdentity), false)
            compare(shell.runRequests.length, 2)
        }

        function test_host_declined_selection_keeps_checked_and_visible_direction() {
            const shell = createShell()
            shell.acceptSelection = false
            mouseClick(findChild(shell, "upstreamDirectionTab"))
            compare(shell.selectionRequests.length, 1)
            compare(shell.selectedDirection, "downstream")
            compare(findChild(shell, "downstreamDirectionTab").checked, true)
            compare(findChild(shell, "upstreamDirectionTab").checked, false)
            compare(shell.downstreamItem.visible, true)
            compare(shell.upstreamItem.visible, false)
            compare(shell.runRequests.length, 0)
            tryCompare(shell.downstreamItem.firstField, "activeFocus", true)
        }

        function test_nonvisual_fixture_shortcut_is_explicitly_direction_scoped() {
            const shell = createShell()
            shell.downstreamItem.firstField.forceActiveFocus()
            keyClick(Qt.Key_P, Qt.ControlModifier | Qt.AltModifier)
            compare(shell.runRequests.length, 1)
            compare(shell.runRequests[0].direction, "downstream")
            selectWithPointer(shell, "upstream")
            keyClick(Qt.Key_P, Qt.ControlModifier | Qt.AltModifier)
            compare(shell.runRequests.length, 2)
            compare(shell.runRequests[1].direction, "upstream")
        }

        function test_theme_resize_and_rendering_preserve_live_children() {
            const shell = createShell()
            const downstream = shell.downstreamItem
            const upstream = shell.upstreamItem
            for (const dark of [true, false]) {
                testTheme.dark = dark
                for (const width of [window.wideWidth, window.narrowWidth]) {
                    window.width = width
                    verify(waitForRendering(shell))
                    compare(shell.width, width)
                    compare(shell.downstreamItem, downstream)
                    compare(shell.upstreamItem, upstream)
                    compare(downstream.surfaceColor, testTheme.panel)
                    compare(upstream.surfaceColor, testTheme.panel)
                    const tab = findChild(shell, "downstreamDirectionTab")
                    verify(tab.width > 0 && tab.width <= shell.width)
                    verify(downstream.height > 0 && downstream.width === shell.width)
                    const rendered = grabImage(shell)
                    const sample = downstream.mapToItem(shell, 2, 2)
                    compare(rendered.pixel(Math.round(sample.x), Math.round(sample.y)), testTheme.panel)
                    const downstreamTab = findChild(shell, "downstreamDirectionTab")
                    const upstreamTab = findChild(shell, "upstreamDirectionTab")
                    verify(downstreamTab.width + upstreamTab.width < shell.width)
                    for (const direction of ["downstream", "upstream"]) {
                        selectWithPointer(shell, direction)
                        const selectedTab = findChild(shell, direction + "DirectionTab")
                        const inactiveTab = direction === "downstream" ? upstreamTab : downstreamTab
                        compare(selectedTab.contentItem.lineCount, 1)
                        verify(!selectedTab.contentItem.truncated,
                            direction + " label clipped at viewport " + width
                            + ": tab=" + selectedTab.width
                            + ", text=" + selectedTab.contentItem.implicitWidth)
                        const image = grabImage(shell)

                        // Sample the shared edge inside and outside the selected folder.
                        const joined = selectedTab.mapToItem(shell,
                            selectedTab.width / 2, selectedTab.height - shell.normalBorderWidth)
                        const separated = inactiveTab.mapToItem(shell,
                            inactiveTab.width / 2, inactiveTab.height - shell.normalBorderWidth)
                        compare(image.pixel(Math.floor(joined.x), Math.floor(joined.y)),
                            testTheme.background)
                        compare(image.pixel(Math.floor(separated.x), Math.floor(separated.y)),
                            testTheme.line)
                    }
                    if (width === window.narrowWidth) {
                        grabImage(shell).save("tabs-"
                            + (dark ? "dark" : "light") + "-narrow.png")
                    }
                }
            }
        }
    }
}
