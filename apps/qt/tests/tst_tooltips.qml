/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../qml/theme"
import "../qml/components"

Pane {
    id: window
    width: 640; height: 360; visible: true
    Theme { id: theme }
    WorkbenchActionButton {
        id: action
        theme: theme
        x: 20; y: 20
        text: "Hover help"
        ToolTip.text: "Shared action help"
        ToolTip.visible: hovered || visualFocus
    }
    ContextualStatus {
        id: status
        theme: theme
        x: 20; y: 160
        text: "Context help"
        diagnostic: ({code: "test", tooltip: "Shared status help"})
    }
    TestCase {
        name: "TooltipTiming"
        when: windowShown
        function cleanup() {
            mouseMove(window, 600, 320)
            wait(100)
        }
        function checkDelay(item, tip) {
            mouseMove(item, 10, 10)
            mouseClick(item, 10, 10)
            wait(250)
            verify(!tip.visible, "Tooltip must not appear immediately")
            mouseMove(window, 600, 320)
            wait(1000)
            verify(!tip.visible, "Leaving cancels pending help")
            mouseMove(item, 10, 10)
            wait(700)
            verify(!tip.visible, "Tooltip must wait one second")
            tryCompare(tip, "visible", true, 1000)
            mouseMove(window, 600, 320)
            tryCompare(tip, "visible", false, 500)
        }
        function test_attached_shared_button() {
            compare(theme.tooltipDelayMs, 1000)
            compare(action.ToolTip.delay, theme.tooltipDelayMs)
            checkDelay(action, action.ToolTip.toolTip)
        }
        function test_explicit_context_help() {
            const tip=findChild(status, "contextualStatusTooltip")
            compare(tip.delay, theme.tooltipDelayMs)
            checkDelay(status, tip)
        }
    }
}
