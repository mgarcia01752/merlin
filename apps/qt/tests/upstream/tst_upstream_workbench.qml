/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../fixtures"
import "../../qml/shared/theme"
import "../../qml/upstream"

Pane {
    id: window
    readonly property int fixturePresentationBytes: 4 * 1024 * 1024
    readonly property int fixturePresentationEntries: 100000
    readonly property int fixtureRetainedViews: 64
    readonly property int fixtureOpenViews: 4
    PresentationBudget {
        id: presentationBudget
        maximumBytes: window.fixturePresentationBytes
        maximumEntries: window.fixturePresentationEntries
        maximumViews: window.fixtureRetainedViews
        maximumOpen: window.fixtureOpenViews
    }
    width: tests.desktopWidth
    height: tests.desktopHeight
    visible: true
    padding: 0
    Theme {
        id: theme
        presentationBackendFactory: presentationBudget.createAdapter
    }
    ThemePalette {
        targetControl: window
        theme: theme
    }
    ThemePalette {
        targetControl: window.Window.window
        theme: theme
    }
    QtObject {
        id: host
        property var presentation: ({})
    }
    UpstreamWorkbench {
        id: workbench
        anchors.fill: parent
        theme: theme
        host: host
    }
    SignalSpy {
        id: navigationSpy
        target: workbench
        signalName: "sectionRequested"
    }
    TestCase {
        id: tests
        name: "UpstreamStartupPresentation"
        when: windowShown
        readonly property real desktopWidth: 1280
        readonly property real desktopHeight: 780
        readonly property real narrowWidth: 640
        readonly property real narrowHeight: 480
        readonly property int renderSettleDelayMs: 100
        readonly property var sectionIds: ["ucd", "profiles", "grants", "transmitter", "channel", "receiver", "source", "preEqualization", "rxmer", "verification"]

        /* Return a block by its documented stable navigation identifier. */
        function blockFor(key) {
            return findChild(workbench, "upstream" + key.charAt(0).toUpperCase() + key.slice(1) + "Block");
        }

        /* Reset presentation fixtures. These cases do not execute a backend. */
        function init() {
            presentationBudget.maximumBytes = window.fixturePresentationBytes;
            presentationBudget.maximumEntries = window.fixturePresentationEntries;
            presentationBudget.maximumOpen = window.fixtureOpenViews;
            window.width = desktopWidth;
            window.height = desktopHeight;
            theme.dark = true;
            host.presentation = ({});
            navigationSpy.clear();
            const viewport = findChild(workbench, "upstreamViewport");
            viewport.contentX = 0;
            viewport.contentY = 0;
            for (const key of sectionIds) {
                workbench.sectionDialogs[key].close();
            }
            waitForRendering(workbench);
        }

        /* Exercise the shared popup close control and focus restoration. */
        function closeWithX(popup) {
            mouseClick(findChild(popup.header, "popupCloseButton"));
            tryCompare(popup, "opened", false);
        }

        function test_all_blocks_open_individual_drill_ins_by_mouse() {
            for (const key of sectionIds) {
                const block = blockFor(key);
                verify(block !== null, key);
                mouseClick(block);
                const popup = workbench.sectionDialogs[key];
                tryCompare(popup, "opened", true);
                compare(navigationSpy.signalArguments[navigationSpy.count - 1][0], key);
                verify(findChild(popup, "upstreamObservationSummary").text.indexOf("Unavailable") === 0);
                closeWithX(popup);
                tryCompare(block, "activeFocus", true);
            }
            compare(navigationSpy.count, sectionIds.length);
        }

        function test_keyboard_navigation_escape_and_focus_return() {
            const source = blockFor("source");
            source.forceActiveFocus(Qt.TabFocusReason);
            keyClick(Qt.Key_Space);
            tryCompare(workbench.sectionDialogs.source, "opened", true);
            keyClick(Qt.Key_Escape);
            tryCompare(workbench.sectionDialogs.source, "opened", false);
            tryCompare(source, "activeFocus", true);
            keyClick(Qt.Key_Tab);
            tryCompare(blockFor("rxmer"), "activeFocus", true);
            for (const key of sectionIds) {
                const block = blockFor(key);
                block.forceActiveFocus(Qt.TabFocusReason);
                keyClick(Qt.Key_Return);
                tryCompare(workbench.sectionDialogs[key], "opened", true);
                keyClick(Qt.Key_Escape);
                tryCompare(workbench.sectionDialogs[key], "opened", false);
                tryCompare(block, "activeFocus", true);
            }
        }

        function test_host_readiness_and_observations_are_display_only() {
            const block = blockFor("transmitter");
            compare(block.blockStatus, "attention");
            host.presentation = ({
                    blocks: {
                        transmitter: {
                            status: "complete",
                            reason: "Adapter-provided readiness fixture"
                        }
                    },
                    details: {
                        transmitter: {
                            summary: "Adapter-provided display fixture",
                            rows: [
                                {
                                    label: "State",
                                    value: "Accepted display fixture"
                                }
                            ]
                        }
                    }
                });
            compare(block.blockStatus, "complete");
            compare(block.statusReason, "Adapter-provided readiness fixture");
            mouseClick(block);
            const popup = workbench.sectionDialogs.transmitter;
            tryCompare(popup, "opened", true);
            compare(findChild(popup, "upstreamObservationSummary").text, "Adapter-provided display fixture");
            closeWithX(popup);
            compare(host.presentation.details.transmitter.rows[0].value, "Accepted display fixture");
            host.presentation = ({});
            compare(block.blockStatus, "attention");
            compare(blockFor("receiver").blockStatus, "attention");
            const before = navigationSpy.count;
            workbench.openSection("unknown");
            compare(navigationSpy.count, before);
        }

        function test_rejected_host_snapshot_preserves_admitted_readiness() {
            host.presentation = {blocks: {transmitter: {status: "complete", reason: "Admitted display"}}};
            compare(blockFor("transmitter").blockStatus, "complete");
            const accepted = workbench.presentation;
            presentationBudget.maximumBytes = presentationBudget.bytes;
            host.presentation = {blocks: {transmitter: {status: "attention", reason: "Candidate display"}}};
            verify(workbench.presentationRejected);
            compare(workbench.presentation, accepted);
            compare(blockFor("transmitter").blockStatus, "complete");
            compare(blockFor("transmitter").statusReason, "Admitted display");
        }

        function test_checked_open_rejection_preserves_hidden_dialog_and_recovers() {
            const popup = workbench.sectionDialogs.receiver;
            presentationBudget.maximumOpen = 0;
            verify(!workbench.openSection("receiver"));
            compare(popup.opened, false);
            presentationBudget.maximumOpen = window.fixtureOpenViews;
            verify(workbench.openSection("receiver"));
            tryCompare(popup, "opened", true);
            closeWithX(popup);
        }

        function test_reference_planes_and_distinct_route_roles() {
            const tx = blockFor("transmitter");
            const channel = blockFor("channel");
            const receiver = blockFor("receiver");
            compare(tx.heading, "CM Transmitter");
            compare(channel.heading, "Channel Simulator");
            compare(receiver.heading, "CMTS Receiver");
            verify(tx.x + tx.width < channel.x);
            verify(channel.x + channel.width < receiver.x);
            compare(channel.x - tx.x - tx.width, receiver.x - channel.x - channel.width);
            const data = findChild(workbench, "txChannelData");
            const control = findChild(workbench, "transmitterControl");
            const measurement = findChild(workbench, "preEqualizationMeasurement");
            const verification = findChild(workbench, "receiverVerification");
            compare(data.role, "data");
            compare(data.dashed, false);
            compare(data.fromX, tx.x + tx.width);
            compare(data.toX, channel.x);
            compare(control.markerShape, "square");
            compare(control.toX, tx.x + tx.width / 2);
            compare(control.toY, tx.y + tx.height);
            compare(findChild(workbench, "configurationControl").toX, control.toX);
            compare(measurement.markerShape, "circle");
            compare(verification.markerShape, "diamond");
            compare(measurement.toY, data.fromY);
            verify(measurement.toX > data.fromX && measurement.toX < data.toX);
            compare(verification.fromY, receiver.y);
            verify(verification.toY < verification.fromY);
            compare(findChild(workbench, "upstreamCorrectionLabel").text, "C before H");
            compare(findChild(workbench, "upstreamChannelLabel").text, "Physical channel H");
            verify(findChild(workbench, "upstreamReceiverLabel").text.indexOf("C×H") >= 0);
            verify(workbench.sectionDialogs.receiver.referencePlane.indexOf("received observations") >= 0);
            verify(workbench.sectionDialogs.preEqualization.referencePlane.indexOf("not a CMTS") >= 0);
        }

        function test_both_themes_update_routes_blocks_and_popups_data() {
            return [
                {
                    tag: "dark",
                    dark: true
                },
                {
                    tag: "light",
                    dark: false
                }
            ];
        }

        function test_both_themes_update_routes_blocks_and_popups(data) {
            theme.dark = data.dark;
            compare(findChild(workbench, "txChannelData").roleColor, theme.accent);
            compare(findChild(workbench, "rxmerMeasurement").roleColor, theme.txScatteredPilot);
            mouseClick(blockFor("transmitter"));
            const popup = workbench.sectionDialogs.transmitter;
            tryCompare(popup, "opened", true);
            compare(popup.background.color, theme.panel);
            compare(findChild(popup, "upstreamObservationSummary").color, theme.muted);
            verify(waitForRendering(popup.contentItem));
            closeWithX(popup);
            verify(waitForRendering(workbench));
        }

        function test_narrow_resize_keeps_geometry_and_scrolls_data() {
            return [
                {
                    tag: "dark",
                    dark: true
                },
                {
                    tag: "light",
                    dark: false
                }
            ];
        }

        function test_narrow_resize_keeps_geometry_and_scrolls(data) {
            theme.dark = data.dark;
            window.width = narrowWidth;
            window.height = narrowHeight;
            const viewport = findChild(workbench, "upstreamViewport");
            const receiver = blockFor("receiver");
            compare(viewport.contentWidth, workbench.minimumDiagramWidth);
            compare(viewport.contentHeight, workbench.minimumDiagramHeight);
            verify(viewport.contentWidth > viewport.width);
            const scrollbar = findChild(workbench, "upstreamHorizontalScrollBar");
            tryVerify(function () {
                return scrollbar.visible;
            });
            // A real scrollbar drag must expose content beyond the left edge.
            mouseDrag(scrollbar, scrollbar.width / 4, scrollbar.height / 2, scrollbar.width / 2, 0);
            verify(viewport.contentX > 0);
            blockFor("source").forceActiveFocus(Qt.TabFocusReason);
            viewport.contentX = 0;
            receiver.forceActiveFocus(Qt.TabFocusReason);
            verify(viewport.contentX > 0);
            keyClick(Qt.Key_Return);
            const popup = workbench.sectionDialogs.receiver;
            tryCompare(popup, "opened", true);
            verify(popup.width <= window.width - popup.windowMargin * 2);
            verify(popup.height <= window.height - popup.windowMargin * 2);
            verify(waitForRendering(popup.contentItem));
            keyClick(Qt.Key_Escape);
            tryCompare(popup, "opened", false);
            tryCompare(receiver, "activeFocus", true);
            window.width = desktopWidth + workbench.blockWidth;
            window.height = desktopHeight;
            const tx = blockFor("transmitter");
            const channel = blockFor("channel");
            compare(channel.x - tx.x - tx.width, receiver.x - channel.x - channel.width);
        }

        function test_render_to_ignored_evidence() {
            wait(renderSettleDelayMs);
            compare(grabImage(workbench).width, workbench.width);
            grabImage(workbench).save("upstream-startup-dark.png");
            theme.dark = false;
            wait(renderSettleDelayMs);
            compare(grabImage(workbench).height, workbench.height);
            grabImage(workbench).save("upstream-startup-light.png");
            mouseClick(blockFor("receiver"));
            tryCompare(workbench.sectionDialogs.receiver, "opened", true);
            grabImage(window).save("upstream-receiver-light.png");
            closeWithX(workbench.sectionDialogs.receiver);
        }
    }
}
