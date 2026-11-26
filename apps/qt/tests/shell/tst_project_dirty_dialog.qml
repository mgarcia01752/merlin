/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
pragma ComponentBehavior: Bound
import QtQuick
import QtQuick.Controls
import QtQuick.Window
import QtTest
import "../fixtures"
import "../../qml/shared/theme"
import "../../qml/shell"

// Presentation fixtures only. Native revisions, project data and file operations
// remain outside this dialog and these standalone tests.
Pane {
    id: window
    width: 840
    height: 620
    visible: true
    padding: 0
    readonly property int fixtureBytes: 32768
    readonly property int fixtureEntries: 256
    readonly property int fixtureViews: 4
    readonly property int fixtureOpenViews: 1
    property var current: null
    property bool reopenOnSave: false
    property bool reopened: false
    PresentationBudget {
        id: budget
        maximumBytes: window.fixtureBytes
        maximumEntries: window.fixtureEntries
        maximumViews: window.fixtureViews
        maximumOpen: window.fixtureOpenViews
    }
    Theme {
        id: testTheme
        presentationBackendFactory: budget.createAdapter
    }
    ThemePalette {
        targetControl: window
        theme: testTheme
    }
    ThemePalette {
        targetControl: window.Window.window
        theme: testTheme
    }
    ThemePalette {
        targetControl: window.current
        theme: testTheme
    }
    Component {
        id: dialogComponent
        ProjectDirtyDialog {
            theme: testTheme
        }
    }
    SignalSpy {
        id: saves
        target: window.current
        signalName: "saveRequested"
    }
    SignalSpy {
        id: discards
        target: window.current
        signalName: "discardRequested"
    }
    SignalSpy {
        id: cancellations
        target: window.current
        signalName: "cancelled"
    }
    property Connections followup: Connections {
        target: window.current
        function onSaveRequested() {
            if (window.reopenOnSave) {
                window.current.direction = "upstream";
                window.current.action = "close";
                window.reopened = window.current.openChecked();
            }
        }
    }
    TestCase {
        name: "ProjectDirtyDialog"
        when: windowShown
        readonly property int wideWindowWidth: 840
        readonly property int narrowWindowWidth: 320
        readonly property int windowHeight: 620
        readonly property int renderSettleMs: 50
        readonly property int layoutRoundingPixels: 1

        function init() {
            budget.maximumBytes = window.fixtureBytes;
            budget.maximumEntries = window.fixtureEntries;
            budget.maximumViews = window.fixtureViews;
            budget.maximumOpen = window.fixtureOpenViews;
            budget.failCommit = false;
            testTheme.presentationBackendFactory = budget.createAdapter;
            testTheme.presentationAdmissionRejected = false;
            testTheme.dark = true;
            window.reopenOnSave = false;
            window.reopened = false;
            window.Window.window.width = wideWindowWidth;
            window.Window.window.height = windowHeight;
            window.current = createTemporaryObject(dialogComponent, window.contentItem, {
                direction: "downstream",
                action: "open",
                saveAllowed: true
            });
            verify(window.current);
            saves.clear();
            discards.clear();
            cancellations.clear();
        }

        function cleanup() {
            window.reopenOnSave = false;
            if (window.current) {
                window.current.close();
                tryCompare(window.current, "visible", false);
                tryCompare(budget, "open", 0);
                tryCompare(budget, "bytes", 0);
                window.current.destroy();
                window.current = null;
            }
            tryCompare(budget, "views", 0);
            window.Window.window.width = wideWindowWidth;
        }

        function button(name) {
            const value = findChild(window.current.footer, "projectDirty" + name);
            verify(value);
            return value;
        }

        function openPrompt() {
            verify(window.current.openChecked());
            tryCompare(window.current, "opened", true);
            tryVerify(() => button("Cancel").activeFocus);
            compare(budget.open, 1);
            verify(window.current.contextCurrent);
        }

        function expectOutcome(saveCount, discardCount, cancelCount) {
            tryCompare(window.current, "visible", false);
            tryCompare(saves, "count", saveCount);
            tryCompare(discards, "count", discardCount);
            tryCompare(cancellations, "count", cancelCount);
            compare(budget.open, 0);
            compare(budget.bytes, 0);
        }

        function test_compiled_tags_data() {
            return [
                {
                    tag: "ds-open",
                    direction: "downstream",
                    action: "open",
                    purpose: "opening another project"
                },
                {
                    tag: "ds-new",
                    direction: "downstream",
                    action: "new",
                    purpose: "creating a new workbench"
                },
                {
                    tag: "ds-close",
                    direction: "downstream",
                    action: "close",
                    purpose: "closing"
                },
                {
                    tag: "us-open",
                    direction: "upstream",
                    action: "open",
                    purpose: "opening another project"
                },
                {
                    tag: "us-new",
                    direction: "upstream",
                    action: "new",
                    purpose: "creating a new workbench"
                },
                {
                    tag: "us-close",
                    direction: "upstream",
                    action: "close",
                    purpose: "closing"
                }
            ];
        }

        function test_compiled_tags(data) {
            window.current.direction = data.direction;
            window.current.action = data.action;
            openPrompt();
            const message = findChild(window.current.contentItem, "projectDirtyMessage");
            compare(message.text, "Save changes to the " + data.direction + " workbench before " + data.purpose + "?");
            compare(message.textFormat, Text.PlainText);
            compare(button("Save").text, "Save");
            compare(button("Discard").text, "Discard");
            compare(button("Cancel").text, "Cancel");
            verify(button("Cancel").primary);
            mouseClick(button("Cancel"));
            expectOutcome(0, 0, 1);
        }

        function test_invalid_tags_data() {
            return [
                {
                    tag: "empty-direction",
                    direction: "",
                    action: "open"
                },
                {
                    tag: "foreign-direction",
                    direction: "other",
                    action: "open"
                },
                {
                    tag: "direction-case",
                    direction: "Upstream",
                    action: "new"
                },
                {
                    tag: "empty-action",
                    direction: "upstream",
                    action: ""
                },
                {
                    tag: "foreign-action",
                    direction: "downstream",
                    action: "save"
                },
                {
                    tag: "action-case",
                    direction: "downstream",
                    action: "Open"
                }
            ];
        }

        function test_invalid_tags(data) {
            window.current.direction = data.direction;
            window.current.action = data.action;
            compare(window.current.openChecked(), false);
            compare(window.current.visible, false);
            compare(window.current.resolve("save"), false);
            compare(window.current.resolve("discard"), false);
            compare(budget.open, 0);
            compare(budget.bytes, 0);
            compare(saves.count + discards.count + cancellations.count, 0);
        }

        function test_save_once_closes_before_dispatch() {
            openPrompt();
            mouseClick(button("Save"));
            compare(window.current.resolve("save"), false);
            compare(window.current.resolve("discard"), false);
            compare(window.current.openChecked(), false);
            expectOutcome(1, 0, 0);
            window.current.close();
            compare(saves.count, 1);
            compare(cancellations.count, 0);
        }

        function test_discard_once_preserves_save_disabled() {
            window.current.saveAllowed = false;
            openPrompt();
            compare(button("Save").enabled, false);
            compare(window.current.resolve("save"), false);
            verify(button("Discard").enabled);
            mouseClick(button("Discard"));
            expectOutcome(0, 1, 0);
            compare(window.current.resolve("discard"), false);
        }

        function test_cancel_routes_data() {
            return [
                {
                    tag: "button",
                    route: "button"
                },
                {
                    tag: "escape",
                    route: "escape"
                },
                {
                    tag: "header",
                    route: "header"
                },
                {
                    tag: "close",
                    route: "close"
                },
                {
                    tag: "reject",
                    route: "reject"
                }
            ];
        }

        function test_cancel_routes(data) {
            openPrompt();
            if (data.route === "button") {
                mouseClick(button("Cancel"));
            } else if (data.route === "escape") {
                keyClick(Qt.Key_Escape);
            } else if (data.route === "header") {
                mouseClick(findChild(window.current.header, "popupCloseButton"));
            } else if (data.route === "close") {
                window.current.close();
            } else {
                window.current.reject();
            }
            expectOutcome(0, 0, 1);
            window.current.close();
            compare(window.current.resolve("cancel"), false);
            compare(cancellations.count, 1);
        }

        function test_cancel_is_default_enter_action() {
            openPrompt();
            keyClick(Qt.Key_Return);
            expectOutcome(0, 0, 1);
        }

        function test_keyboard_traversal_and_save() {
            openPrompt();
            keyClick(Qt.Key_Backtab);
            tryVerify(() => button("Discard").activeFocus);
            keyClick(Qt.Key_Backtab);
            tryVerify(() => button("Save").activeFocus);
            keyClick(Qt.Key_Return);
            expectOutcome(1, 0, 0);
        }

        function test_keyboard_discard_space() {
            openPrompt();
            button("Discard").forceActiveFocus(Qt.TabFocusReason);
            keyClick(Qt.Key_Space);
            expectOutcome(0, 1, 0);
        }

        function test_context_drift_cannot_save_or_discard() {
            openPrompt();
            const originalTitle = window.current.title;
            window.current.direction = "upstream";
            compare(window.current.contextCurrent, false);
            compare(window.current.title, originalTitle);
            compare(button("Save").enabled, false);
            compare(button("Discard").enabled, false);
            compare(window.current.resolve("save"), false);
            compare(window.current.resolve("discard"), false);
            window.current.action = "invalid";
            compare(window.current.resolve("discard"), false);
            verify(button("Cancel").enabled);
            mouseClick(button("Cancel"));
            expectOutcome(0, 0, 1);
        }

        function test_save_allowed_change_is_authoritative() {
            openPrompt();
            window.current.saveAllowed = false;
            compare(button("Save").enabled, false);
            compare(window.current.resolve("save"), false);
            window.current.saveAllowed = true;
            verify(button("Save").enabled);
            mouseClick(button("Save"));
            expectOutcome(1, 0, 0);
        }

        function test_repeated_open_does_not_replace_context() {
            openPrompt();
            const original = window.current.presentationCoverage.value;
            const bytes = budget.bytes, entries = budget.entries;
            compare(window.current.openChecked(), false);
            verify(window.current.presentationCoverage.value === original);
            compare(budget.bytes, bytes);
            compare(budget.entries, entries);
            mouseClick(button("Cancel"));
            expectOutcome(0, 0, 1);
            window.current.direction = "upstream";
            window.current.action = "new";
            openPrompt();
            verify(window.current.title.indexOf("Upstream") >= 0);
            mouseClick(button("Cancel"));
            expectOutcome(0, 0, 2);
        }

        function test_reentrant_followup_waits_for_open_slot_cleanup() {
            openPrompt();
            window.reopenOnSave = true;
            mouseClick(button("Save"));
            tryCompare(saves, "count", 1);
            tryCompare(window, "reopened", true);
            tryCompare(window.current, "opened", true);
            compare(budget.open, 1);
            compare(window.current.direction, "upstream");
            compare(window.current.action, "close");
            window.reopenOnSave = false;
            mouseClick(button("Cancel"));
            expectOutcome(1, 0, 1);
        }

        function test_presentation_pressure_rejects_without_outcome() {
            budget.maximumBytes = 1;
            compare(window.current.openChecked(), false);
            compare(window.current.visible, false);
            compare(budget.bytes, 0);
            compare(budget.open, 0);
            compare(saves.count + discards.count + cancellations.count, 0);
            verify(testTheme.presentationAdmissionRejected);
            budget.maximumBytes = window.fixtureBytes;
            openPrompt();
            mouseClick(button("Cancel"));
            expectOutcome(0, 0, 1);
        }

        function test_visibility_pressure_clears_unshown_strings() {
            budget.maximumOpen = 0;
            compare(window.current.openChecked(), false);
            compare(window.current.openAdmissionRejected, true);
            compare(window.current.visible, false);
            compare(window.current.presentationCoverage.value, null);
            compare(budget.bytes, 0);
            compare(budget.open, 0);
            compare(saves.count + discards.count + cancellations.count, 0);
            budget.maximumOpen = window.fixtureOpenViews;
            openPrompt();
            mouseClick(button("Cancel"));
            expectOutcome(0, 0, 1);
        }

        function test_failed_commit_fails_closed() {
            budget.failCommit = true;
            compare(window.current.openChecked(), false);
            compare(budget.bytes, 0);
            compare(budget.open, 0);
            compare(saves.count + discards.count + cancellations.count, 0);
            budget.failCommit = false;
            openPrompt();
            mouseClick(button("Cancel"));
            expectOutcome(0, 0, 1);
        }

        function test_missing_factory_fails_closed() {
            testTheme.presentationBackendFactory = null;
            compare(window.current.openChecked(), false);
            compare(window.current.visible, false);
            compare(budget.bytes, 0);
            compare(budget.open, 0);
            compare(saves.count + discards.count + cancellations.count, 0);
            testTheme.presentationBackendFactory = budget.createAdapter;
            openPrompt();
            mouseClick(button("Cancel"));
            expectOutcome(0, 0, 1);
        }

        function test_destroy_releases_actual_retained_and_open_views() {
            openPrompt();
            verify(budget.bytes > 0);
            verify(budget.views > 0);
            window.current.destroy();
            window.current = null;
            tryCompare(budget, "open", 0);
            tryCompare(budget, "bytes", 0);
            tryCompare(budget, "views", 0);
        }

        function test_themes_narrow_layout_and_accessibility_data() {
            return [
                {
                    tag: "dark-wide",
                    dark: true,
                    width: wideWindowWidth
                },
                {
                    tag: "light-wide",
                    dark: false,
                    width: wideWindowWidth
                },
                {
                    tag: "dark-narrow",
                    dark: true,
                    width: narrowWindowWidth
                },
                {
                    tag: "light-narrow",
                    dark: false,
                    width: narrowWindowWidth
                }
            ];
        }

        function test_themes_narrow_layout_and_accessibility(data) {
            testTheme.dark = data.dark;
            window.Window.window.width = data.width;
            openPrompt();
            wait(renderSettleMs);
            const save = button("Save"), discard = button("Discard"), cancel = button("Cancel");
            verify(Math.abs(save.width - discard.width) <= layoutRoundingPixels);
            verify(Math.abs(save.width - cancel.width) <= layoutRoundingPixels);
            compare(save.height, cancel.height);
            verify(save.Accessible.description.length > 0);
            verify(discard.Accessible.description.length > 0);
            verify(cancel.Accessible.description.length > 0);
            if (data.width === narrowWindowWidth) {
                verify(save.y < discard.y && discard.y < cancel.y);
            } else {
                compare(save.y, discard.y);
                compare(save.y, cancel.y);
            }
            const image = grabImage(window.current.contentItem.parent);
            verify(image.width > 0 && image.height > 0);
            image.save("project-dirty-" + data.tag + ".png");
            mouseClick(cancel);
            expectOutcome(0, 0, 1);
        }
    }
}
