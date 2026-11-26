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

// Presentation-only fixtures. These strings never establish capture admission.
Pane {
    id: window
    width: 1280
    height: 780
    visible: true
    padding: 0
    readonly property int fixtureBytes: 100000
    readonly property int fixtureEntries: 10000
    readonly property int fixtureViews: 8
    readonly property int fixtureOpenViews: 1
    PresentationBudget {
        id: budget
        maximumBytes: window.fixtureBytes
        maximumEntries: window.fixtureEntries
        maximumViews: window.fixtureViews
        maximumOpen: window.fixtureOpenViews
    }
    Theme {
        id: fixtureTheme
        presentationBackendFactory: budget.createAdapter
    }
    Theme {
        id: unavailableTheme
    }
    ThemePalette {
        targetControl: window
        theme: fixtureTheme
    }
    Component {
        id: dialogComponent
        UpstreamCaptureDialog {
            theme: fixtureTheme
        }
    }
    SignalSpy {
        id: loaded
        target: tests.popup
        signalName: "loadRequested"
    }
    SignalSpy {
        id: paged
        target: tests.popup
        signalName: "pageRequested"
    }
    SignalSpy {
        id: measuredPlan
        target: tests.popup
        signalName: "measuredPlanRequested"
    }
    TestCase {
        id: tests
        name: "UpstreamCaptureInspection"
        when: windowShown
        property var popup: null
        readonly property int previousFirst: 7
        readonly property int nextFirst: 1031
        readonly property real narrowDialogWidth: 480
        readonly property real narrowDialogHeight: 500
        readonly property string wideIdentity: "18446744073709551615"
        readonly property string originalHash: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        readonly property string deliveryHash: "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"

        /* Native-shaped display metadata; no scientific interpretation occurs. */
        function fixture(family) {
            return {
                title: "Supplied " + family + " capture inspection",
                summary: "Inspection only / unassociated. No simulation is configured.",
                reference_plane: "Original source observation; caller-supplied reference plane.",
                metadata_rows: [
                    {
                        label: "Original SHA-256",
                        value: originalHash
                    },
                    {
                        label: "Delivery SHA-256",
                        value: deliveryHash
                    },
                    {
                        label: "Original byte count",
                        value: "65549"
                    },
                    {
                        label: "Delivery byte count",
                        value: "1048576"
                    },
                    {
                        label: "Source epoch seconds",
                        value: "0"
                    },
                    {
                        label: "Association",
                        value: "Inspection only / unassociated"
                    }
                ],
                columns: ["Original coordinate", "Supplied first value", "Supplied second value"],
                data_rows: [
                    {
                        coordinate: wideIdentity,
                        first: "-4.000000000000000000",
                        second: "0.00006103515625"
                    },
                    {
                        coordinate: "128",
                        first: "0xff",
                        second: "Unavailable"
                    }
                ],
                page: {
                    summary: "Supplied page extent; no extent is inferred here.",
                    previous_first: previousFirst,
                    next_first: nextFirst,
                    has_previous: true,
                    has_next: true
                },
                error: ""
            };
        }

        function init() {
            compare(budget.bytes, 0);
            compare(budget.entries, 0);
            compare(budget.views, 0);
            compare(budget.open, 0);
            budget.maximumBytes = window.fixtureBytes;
            budget.maximumEntries = window.fixtureEntries;
            budget.maximumViews = window.fixtureViews;
            budget.maximumOpen = window.fixtureOpenViews;
            fixtureTheme.dark = true;
            loaded.clear();
            paged.clear();
            measuredPlan.clear();
        }

        function cleanup() {
            budget.maximumBytes = window.fixtureBytes;
            budget.maximumEntries = window.fixtureEntries;
            budget.maximumOpen = window.fixtureOpenViews;
            if (popup) {
                popup.close();
                tryCompare(popup, "visible", false);
            }
            popup = null;
        }

        /* Assign plain native-shaped objects after Qt creation-property conversion. */
        function create(family, value) {
            popup = createTemporaryObject(dialogComponent, window, {
                family: family
            });
            verify(popup !== null);
            if (value !== undefined) {
                popup.presentation = value;
            }
            tryCompare(popup, "contextCurrent", true);
            return popup;
        }

        function open(family, value) {
            create(family, value);
            verify(popup.openChecked());
            tryCompare(popup, "opened", true);
            verify(waitForRendering(popup.contentItem));
            return popup;
        }

        function content(name) {
            return findChild(popup.contentItem, name);
        }

        function footer(name) {
            return findChild(popup.footer, name);
        }

        function test_empty_inspection_never_invents_rows_or_apply() {
            open("ucd");
            compare(popup.title, "Upstream Capture");
            compare(content("upstreamCaptureMetadata").count, 0);
            compare(content("upstreamCaptureData").count, 0);
            compare(content("upstreamCaptureSummary").text, "No capture inspection is available.");
            compare(content("upstreamCaptureReferencePlane").text, "Inspection only. Association and simulation readiness are unavailable.");
            verify(!content("upstreamCapturePrevious").enabled);
            verify(!content("upstreamCaptureNext").enabled);
            compare(findChild(popup.footer, "upstreamDraftApply"), null);
            compare(popup.standardButtons, Dialog.NoButton);
            verify(footer("upstreamCaptureLoad").enabled);
            verify(footer("upstreamCaptureMeasuredPlan").enabled);
        }

        function test_available_data() {
            return [
                {
                    tag: "ucd",
                    family: "ucd"
                },
                {
                    tag: "rxmer",
                    family: "rxmer"
                },
                {
                    tag: "pre-eq",
                    family: "pre_eq"
                }
            ];
        }
        function test_available(data) {
            const value = fixture(data.family);
            open(data.family, value);
            compare(popup.displayedPresentation, value);
            compare(popup.displayedFamily, data.family);
            compare(popup.title, value.title);
            compare(content("upstreamCaptureMetadata").count, value.metadata_rows.length);
            compare(content("upstreamCaptureData").count, value.data_rows.length);
            compare(content("upstreamCaptureCoordinate_0").text, wideIdentity);
            compare(content("upstreamCaptureFirst_0").text, value.data_rows[0].first);
            compare(content("upstreamCaptureSecond_0").text, value.data_rows[0].second);
            compare(content("upstreamCaptureSecond_1").text, "Unavailable");
            compare(content("upstreamCaptureMetadataValue_0").text, originalHash);
            compare(content("upstreamCaptureMetadataValue_1").text, deliveryHash);
            compare(content("upstreamCaptureMetadataValue_4").text, "0");
            compare(content("upstreamCaptureMetadataValue_5").text, "Inspection only / unassociated");
        }

        function test_source_metadata_and_errors_are_plain_text() {
            const value = fixture("rxmer");
            value.title = "<b>Literal title</b>";
            value.metadata_rows[0].value = "<b>" + originalHash + "</b>&source";
            value.error = "<b>Original native diagnostic</b> & exact field/offset";
            open("rxmer", value);
            compare(findChild(popup.header, "upstreamCaptureTitle").textFormat, Text.PlainText);
            compare(content("upstreamCaptureMetadataValue_0").textFormat, Text.PlainText);
            compare(content("upstreamCaptureMetadataValue_0").text, value.metadata_rows[0].value);
            compare(content("upstreamCaptureError").textFormat, Text.PlainText);
            compare(content("upstreamCaptureError").text, value.error);
            verify(content("upstreamCaptureError").visible);
            compare(content("upstreamCaptureData").count, value.data_rows.length);
        }

        function test_empty_error_retains_original_diagnostic_without_source_rows() {
            open("pre_eq", {
                title: "Native input failure",
                error: "Original capture field pre_eq.header at offset 9."
            });
            compare(content("upstreamCaptureData").count, 0);
            compare(content("upstreamCaptureMetadata").count, 0);
            compare(content("upstreamCaptureError").text, popup.presentation.error);
            verify(!content("upstreamCaptureNext").enabled);
            verify(!content("upstreamCapturePrevious").enabled);
        }

        function test_acquisition_and_paging_forward_exact_host_destinations() {
            open("pre_eq", fixture("pre_eq"));
            mouseClick(footer("upstreamCaptureLoad"));
            compare(loaded.count, 1);
            compare(loaded.signalArguments[0].length, 1);
            compare(loaded.signalArguments[0][0], "pre_eq");
            mouseClick(footer("upstreamCaptureMeasuredPlan"));
            compare(measuredPlan.count, 1);
            mouseClick(content("upstreamCapturePrevious"));
            mouseClick(content("upstreamCaptureNext"));
            compare(paged.count, 2);
            compare(paged.signalArguments[0].length, 2);
            compare(paged.signalArguments[0][0], "pre_eq");
            compare(paged.signalArguments[0][1], previousFirst);
            compare(paged.signalArguments[1].length, 2);
            compare(paged.signalArguments[1][0], "pre_eq");
            compare(paged.signalArguments[1][1], nextFirst);
            compare(content("upstreamCaptureCoordinate_0").text, wideIdentity);
        }

        function test_busy_guards_leave_close_available() {
            open("ucd", fixture("ucd"));
            popup.busy = true;
            verify(!footer("upstreamCaptureLoad").enabled);
            verify(!footer("upstreamCaptureMeasuredPlan").enabled);
            verify(!content("upstreamCapturePrevious").enabled);
            verify(!content("upstreamCaptureNext").enabled);
            // Direct signals cannot bypass the same guards used by mouse/keyboard.
            footer("upstreamCaptureLoad").clicked();
            footer("upstreamCaptureMeasuredPlan").clicked();
            content("upstreamCapturePrevious").clicked();
            content("upstreamCaptureNext").clicked();
            compare(loaded.count, 0);
            compare(measuredPlan.count, 0);
            compare(paged.count, 0);
            verify(footer("upstreamCaptureClose").enabled);
            mouseClick(footer("upstreamCaptureClose"));
            tryCompare(popup, "visible", false);
        }

        function test_missing_page_coordinates_never_become_zero_requests() {
            const value = fixture("rxmer");
            value.page = {
                summary: "No admitted next coordinate.",
                has_previous: true,
                has_next: true
            };
            open("rxmer", value);
            verify(!content("upstreamCapturePrevious").enabled);
            verify(!content("upstreamCaptureNext").enabled);
            popup.requestPage(true);
            popup.requestPage(false);
            compare(paged.count, 0);
        }

        function test_rejected_replacement_data() {
            return [
                {
                    tag: "bytes",
                    entries: false
                },
                {
                    tag: "entries",
                    entries: true
                }
            ];
        }
        function test_rejected_replacement(data) {
            open("rxmer", fixture("rxmer"));
            const original = popup.displayedPresentation;
            if (data.entries) {
                budget.maximumEntries = budget.entries;
            } else {
                budget.maximumBytes = budget.bytes;
            }
            const replacement = fixture("rxmer");
            replacement.title = "New rejected capture";
            popup.presentation = replacement;
            verify(popup.presentationRejected);
            compare(popup.displayedPresentation, original);
            compare(popup.title, original.title);
            compare(content("upstreamCaptureMetadataValue_0").text, originalHash);
            compare(content("upstreamCaptureCoordinate_0").text, wideIdentity);
            verify(content("upstreamCaptureAdmission").visible);
            verify(!popup.contextCurrent);
            popup.requestLoad();
            popup.requestMeasuredPlan();
            popup.requestPage(false);
            compare(loaded.count, 0);
            compare(measuredPlan.count, 0);
            compare(paged.count, 0);
            budget.maximumBytes = window.fixtureBytes;
            budget.maximumEntries = window.fixtureEntries;
            verify(popup.adoptPresentation());
            compare(popup.displayedPresentation, replacement);
            compare(popup.title, replacement.title);
            verify(popup.contextCurrent);
        }

        function test_unsupported_nonfinite_replacement_is_not_stringified() {
            open("pre_eq", fixture("pre_eq"));
            const original = popup.displayedPresentation;
            const bad = fixture("pre_eq");
            bad.data_rows[0].first = NaN;
            popup.presentation = bad;
            verify(popup.presentationRejected);
            compare(popup.displayedPresentation, original);
            compare(content("upstreamCaptureFirst_0").text, original.data_rows[0].first);
        }

        function test_close_reopen_keeps_the_admitted_snapshot_until_disposal() {
            open("ucd", fixture("ucd"));
            const original = popup.displayedPresentation;
            const retainedBytes = budget.bytes;
            mouseClick(footer("upstreamCaptureClose"));
            tryCompare(popup, "visible", false);
            compare(budget.open, 0);
            compare(budget.bytes, retainedBytes);
            compare(popup.displayedPresentation, original);
            verify(popup.openChecked());
            tryCompare(popup, "opened", true);
            compare(popup.displayedPresentation, original);
            compare(budget.open, 1);
            popup.close();
            tryCompare(popup, "visible", false);
            popup.destroy();
            popup = null;
            tryCompare(budget, "bytes", 0);
            tryCompare(budget, "entries", 0);
            tryCompare(budget, "views", 0);
        }

        function test_checked_open_rejection_and_recovery() {
            create("ucd", fixture("ucd"));
            const original = popup.displayedPresentation;
            budget.maximumOpen = 0;
            verify(!popup.openChecked());
            verify(popup.openAdmissionRejected);
            compare(popup.visible, false);
            compare(popup.displayedPresentation, original);
            budget.maximumOpen = window.fixtureOpenViews;
            verify(popup.openChecked());
            tryCompare(popup, "opened", true);
        }

        function test_header_and_escape_close_the_read_only_popup() {
            open("ucd", fixture("ucd"));
            mouseClick(findChild(popup.header, "popupCloseButton"));
            tryCompare(popup, "visible", false);
            verify(popup.openChecked());
            tryCompare(popup, "opened", true);
            keyClick(Qt.Key_Escape);
            tryCompare(popup, "visible", false);
        }

        function test_both_themes_and_narrow_footer_keep_real_controls_visible_data() {
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
        function test_both_themes_and_narrow_footer_keep_real_controls_visible(data) {
            fixtureTheme.dark = data.dark;
            open("rxmer", fixture("rxmer"));
            popup.width = narrowDialogWidth;
            popup.height = narrowDialogHeight;
            verify(waitForRendering(popup.contentItem));
            compare(content("upstreamCaptureSummary").color, fixtureTheme.text);
            for (const name of ["upstreamCaptureLoad", "upstreamCaptureMeasuredPlan", "upstreamCaptureClose"]) {
                const action = footer(name);
                const origin = action.mapToItem(popup.footer, 0, 0);
                verify(action.visible && action.enabled);
                verify(origin.x >= 0 && origin.y >= 0);
                verify(origin.x + action.width <= popup.footer.width);
                verify(origin.y + action.height <= popup.footer.height);
            }
            compare(content("upstreamCaptureCoordinate_0").text, wideIdentity);
        }

        function test_missing_factory_fails_closed_without_a_view() {
            popup = createTemporaryObject(dialogComponent, window, {
                family: "ucd",
                theme: unavailableTheme
            });
            verify(popup !== null);
            popup.presentation = fixture("ucd");
            verify(popup.presentationRejected);
            compare(popup.dataRows.length, 0);
            verify(!popup.openChecked());
            compare(popup.visible, false);
            verify(!popup.actionsAvailable);
            popup.requestLoad();
            popup.requestMeasuredPlan();
            compare(loaded.count, 0);
            compare(measuredPlan.count, 0);
        }
    }
}
