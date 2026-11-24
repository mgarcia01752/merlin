/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtTest
import "fixtures"
import "../qml/shared/components"
import "../qml/shared/theme"

Pane {
    id: window
    width: 640
    height: 400
    visible: true
    PresentationBudget {
        id: presentationBudget
        // Independent capacities for existing presentation-only fixture graphs.
        maximumBytes: 10000000
        maximumEntries: 100000
        maximumViews: 96
        maximumOpen: 12
    }
    Theme { presentationBackendFactory: presentationBudget.createAdapter; id: testTheme }
    Component {
        id: popupComponent
        WorkbenchDialog {
            theme: testTheme
            title: "Action Roles"
            parent: Overlay.overlay
            anchors.centerIn: parent
            modal: true
            standardButtons: Dialog.Yes | Dialog.Cancel
            contentItem: Label { text: "Choose an action" }
            property int acceptedCount: 0
            property int rejectedCount: 0
            onAccepted: acceptedCount++
            onRejected: rejectedCount++
        }
    }
    Component {
        id: actionComponent
        WorkbenchActionButton {
            theme: testTheme
            text: "Action"
            property int clickCount: 0
            onClicked: clickCount++
        }
    }
    TestCase {
        name: "SharedButtons"
        when: windowShown
        function test_keyboard_disabled_and_theme_states() {
            const action = createTemporaryObject(actionComponent, window.contentItem)
            verify(action !== null)
            for (const dark of [true, false]) {
                testTheme.dark = dark
                action.enabled = false
                mouseClick(action)
                compare(action.clickCount, dark ? 0 : 1)
                action.enabled = true
                action.forceActiveFocus()
                tryCompare(action, "activeFocus", true)
                keyClick(Qt.Key_Space)
                compare(action.clickCount, dark ? 1 : 2)
            }
            testTheme.dark = true
        }
        function test_open_admission_precedes_visibility_and_preserves_existing_dialog() {
            const popup = createTemporaryObject(popupComponent, window.contentItem)
            verify(popup !== null)
            const previousLimit = presentationBudget.maximumOpen
            presentationBudget.maximumOpen = 0
            verify(!popup.openChecked())
            compare(popup.visible, false)
            compare(presentationBudget.open, 0)
            verify(popup.openAdmissionRejected)
            verify(testTheme.presentationAdmissionRejected)
            presentationBudget.maximumOpen = previousLimit
            verify(popup.openChecked())
            tryCompare(popup, "opened", true)
            compare(presentationBudget.open, 1)
            popup.close()
            tryCompare(popup, "visible", false)
            compare(presentationBudget.open, 0)
        }
        function test_standard_dialog_roles() {
            const popup = createTemporaryObject(popupComponent, window.contentItem)
            verify(popup !== null)
            popup.openChecked()
            tryCompare(popup, "opened", true)
            const cancel = popup.footer.standardButton(Dialog.Cancel)
            const yes = popup.footer.standardButton(Dialog.Yes)
            verify(cancel !== null && yes !== null)
            mouseClick(cancel)
            tryCompare(popup, "visible", false)
            compare(popup.acceptedCount, 0)
            compare(popup.rejectedCount, 1)
            popup.openChecked()
            tryCompare(popup, "opened", true)
            yes.forceActiveFocus()
            keyClick(Qt.Key_Space)
            tryCompare(popup, "visible", false)
            compare(popup.acceptedCount, 1)
            compare(popup.rejectedCount, 1)
            popup.standardButtons = Dialog.Close
            popup.openChecked()
            tryCompare(popup, "opened", true)
            const close = popup.footer.standardButton(Dialog.Close)
            verify(close !== null)
            mouseClick(close)
            tryCompare(popup, "visible", false)
            compare(popup.acceptedCount, 1)
            compare(popup.rejectedCount, 2)
        }
    }
}
