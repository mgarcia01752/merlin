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

// Explicit presentation projection fixtures, not API/scientific acceptance.
Pane {
    id: window
    width: 1200
    height: 920
    padding: 0
    visible: true
    readonly property int fixtureBytes: 8 * 1024 * 1024
    readonly property int fixtureEntries: 100000
    readonly property int fixtureViews: 32
    readonly property int fixtureOpen: 4
    PresentationBudget {
        id: budget
        maximumBytes: window.fixtureBytes
        maximumEntries: window.fixtureEntries
        maximumViews: window.fixtureViews
        maximumOpen: window.fixtureOpen
    }
    Theme {
        id: theme
        presentationBackendFactory: budget.createAdapter
    }
    ThemePalette {
        targetControl: window
        theme: theme
    }
    ThemePalette {
        targetControl: window.Window.window
        theme: theme
    }
    ThemePalette {
        targetControl: qam
        theme: theme
    }
    ThemePalette {
        targetControl: spectrum
        theme: theme
    }
    QtObject {
        id: host
        property var instrumentState: ({})
    }
    UpstreamQamDialog {
        id: qam
        theme: theme
        host: host
    }
    UpstreamSpectrumDialog {
        id: spectrum
        theme: theme
        host: host
    }
    SignalSpy {
        id: qamSpy
        target: qam
        signalName: "instrumentRequested"
    }
    SignalSpy {
        id: spectrumSpy
        target: spectrum
        signalName: "instrumentRequested"
    }
    TestCase {
        name: "UpstreamInstrumentsPresentation"
        when: windowShown
        readonly property real narrowWidth: 640
        readonly property real narrowHeight: 480
        readonly property int renderSettleMs: 50
        readonly property int wheelZoomDelta: 120
        readonly property real dragFraction: 0.1

        function projection(kind, sequence) {
            return {
                status: "ready",
                reason: "",
                query_sequence: sequence || "9007199254740993",
                session_token: "session-9007199254740993",
                configuration_generation: "18446744073709551615",
                run: null,
                snapshot: {
                    kind: kind,
                    status: "ready",
                    message: "",
                    summary: "<b>Native exact summary</b>",
                    metadata_rows: [
                        {
                            label: "Plane",
                            value: "Raw RX FFT"
                        },
                        {
                            label: "Source",
                            value: "<hash>&bytes"
                        }
                    ],
                    metadata: {
                        tick: "18446744073709551615",
                        missing: null
                    },
                    choices: {
                        taps: [
                            {
                                label: "Native TX",
                                value: "TxGrid",
                                available: true,
                                unit_reference: false
                            },
                            {
                                label: "Native RX equalized",
                                value: "RxEqualized",
                                available: true,
                                unit_reference: false
                            },
                            {
                                label: "Unavailable actual tap",
                                value: "RxFft",
                                available: false,
                                unit_reference: true
                            }
                        ],
                        loadings: [
                            {
                                label: "Native all",
                                value: null
                            },
                            {
                                label: "Native loading",
                                value: 4
                            }
                        ],
                        modes: [
                            {
                                label: "Raw native",
                                value: "raw"
                            },
                            {
                                label: "Constellation native",
                                value: "constellation"
                            },
                            {
                                label: "Pair native",
                                value: "pairing"
                            }
                        ]
                    },
                    selection: kind === "qam" ? {
                        mode: "raw",
                        tap: "RxEqualized",
                        loading: null
                    } : {
                        tap: "RxEqualized",
                        first: 20,
                        count: 40
                    },
                    limits: {
                        maximum_points: 77,
                        maximum_references: 33,
                        maximum_pairs: 55
                    },
                    scatter: {
                        extent: 2,
                        points: [
                            {
                                x: 0.25,
                                y: 0.25,
                                role: "data",
                                series: "left",
                                readout: "Actual original point",
                                detail: {
                                    grant_id: "9007199254740993",
                                    physical_carrier: 100,
                                    missing: null
                                }
                            }
                        ],
                        references: [
                            {
                                x: 0.75,
                                y: 0.75,
                                series: "reference",
                                readout: "Exact reference",
                                detail: "Original reference detail"
                            }
                        ]
                    },
                    viewport: {
                        points: [
                            {
                                x: 0.25,
                                y: 0.5,
                                minimum_y: 0.6,
                                readout: "Original PSD point"
                            }
                        ],
                        x_ticks: [
                            {
                                x: 0,
                                label: "Original first"
                            },
                            {
                                x: 1,
                                label: "Original last"
                            }
                        ],
                        y_ticks: [
                            {
                                y: 0,
                                label: "Original maximum"
                            }
                        ],
                        start: 20,
                        count: 40,
                        total: 100,
                        axis_label: "Backend PSD units",
                        frequency_label: "Backend frequency units"
                    }
                }
            };
        }

        function init() {
            qam.close();
            spectrum.close();
            tryCompare(qam, "visible", false);
            tryCompare(spectrum, "visible", false);
            verify(qam.presentationCoverage.clear());
            verify(spectrum.presentationCoverage.clear());
            child(qam, "upstreamQamActualPlot").resetView();
            budget.maximumBytes = window.fixtureBytes;
            budget.maximumEntries = window.fixtureEntries;
            budget.maximumOpen = window.fixtureOpen;
            theme.dark = true;
            window.width = 1200;
            window.height = 920;
            host.instrumentState = {};
            qamSpy.clear();
            spectrumSpy.clear();
        }
        function cleanup() {
            qam.close();
            spectrum.close();
        }
        function openQam() {
            host.instrumentState = projection("qam");
            verify(qam.openChecked());
            tryCompare(qam, "opened", true);
            tryCompare(qam, "actionsEnabled", true);
        }
        function openSpectrum() {
            host.instrumentState = projection("spectrum");
            verify(spectrum.openChecked());
            tryCompare(spectrum, "opened", true);
            tryCompare(spectrum, "actionsEnabled", true);
        }
        function child(dialog, name) {
            return findChild(dialog.contentItem, name);
        }
        function refreshState(kind, sequence) {
            host.instrumentState = projection(kind, sequence);
        }

        function test_loading_reply_adopts_its_own_authoritative_selector_data() {
            return [{tag:"qam",kind:"qam"},{tag:"spectrum",kind:"spectrum"}];
        }
        function test_loading_reply_adopts_its_own_authoritative_selector(data) {
            if (data.kind === "qam") { openQam(); } else { openSpectrum(); }
            const popup = data.kind === "qam" ? qam : spectrum;
            const loading = projection(data.kind === "qam" ? "spectrum" : "qam", "9007199254740994");
            loading.status = "loading";
            host.instrumentState = loading;
            tryCompare(popup,"displayable",false);
            const accepted = projection(data.kind,"9007199254740994");
            host.instrumentState = accepted;
            tryCompare(popup,"actionsEnabled",true);
            verify(popup.selection === accepted.snapshot.selection);
            verify(child(popup,data.kind === "qam" ? "upstreamQamRefresh" : "upstreamSpectrumRefresh").enabled);
        }

        function test_open_never_dispatches_and_displays_only_native_choices() {
            openQam();
            compare(qamSpy.count, 0);
            compare(spectrumSpy.count, 0);
            compare(child(qam, "upstreamQamMode").count, 3);
            compare(child(qam, "upstreamQamTap").count, 3);
            compare(child(qam, "upstreamQamLoading").count, 2);
            compare(child(qam, "upstreamQamSummary").text, "<b>Native exact summary</b>");
            compare(child(qam, "upstreamQamSummary").textFormat, Text.PlainText);
            verify(qam.envelope === host.instrumentState);
        }

        function test_qam_raw_request_is_flat_bounded_and_lossless() {
            openQam();
            mouseClick(child(qam, "upstreamQamRefresh"));
            compare(qamSpy.count, 1);
            compare(qamSpy.signalArguments[0][0], "qam");
            compare(JSON.stringify(qamSpy.signalArguments[0][1]), JSON.stringify({
                mode: "raw",
                tap: "RxEqualized",
                loading: null,
                maximum_points: 77
            }));
            compare(qamSpy.signalArguments[0][2].query_sequence, "9007199254740993");
            compare(qamSpy.signalArguments[0][2].configuration_generation, "18446744073709551615");
            compare(qamSpy.signalArguments[0][2].session_token, "session-9007199254740993");
            compare(qamSpy.signalArguments[0][2].run, null);
            verify(qam.pending && !qam.actionsEnabled);
            host.instrumentState = projection("qam");
            verify(qam.pending);
            refreshState("qam", "9007199254740994");
            tryCompare(qam, "actionsEnabled", true);
        }

        function test_qam_constellation_and_pairing_use_only_native_limits() {
            openQam();
            verify(qam.edit("mode", "constellation"));
            verify(qam.edit("loading", 4));
            verify(qam.edit("tap", "TxGrid"));
            verify(qam.request());
            compare(JSON.stringify(qamSpy.signalArguments[0][1]), JSON.stringify({
                mode: "constellation",
                tap: "TxGrid",
                loading: 4,
                maximum_points: 77,
                maximum_references: 33
            }));
            refreshState("qam", "next-query");
            verify(qam.edit("mode", "pairing"));
            verify(qam.edit("left", "TxGrid"));
            verify(qam.edit("right", "RxEqualized"));
            verify(qam.request());
            compare(JSON.stringify(qamSpy.signalArguments[1][1]), JSON.stringify({
                mode: "pairing",
                left: "TxGrid",
                right: "RxEqualized",
                loading: null,
                maximum_pairs: 55
            }));
        }

        function test_qam_draft_survives_same_query_poll_and_missing_policy_fails_closed() {
            openQam();
            verify(qam.edit("loading", 4));
            host.instrumentState = projection("qam");
            compare(qam.selection.loading, 4);
            const absent = projection("qam", "absent-policy");
            delete absent.snapshot.limits.maximum_points;
            host.instrumentState = absent;
            compare(qam.requestSelection(), null);
            verify(!child(qam, "upstreamQamRefresh").enabled);
            const unavailable = projection("qam", "unavailable-choice");
            unavailable.snapshot.selection.tap = "RxFft";
            host.instrumentState = unavailable;
            verify(!qam.request());
            compare(qamSpy.count, 0);
        }

        function test_qam_aliases_series_and_exact_selected_detail_data() {
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
        function test_qam_aliases_series_and_exact_selected_detail(data) {
            openQam();
            theme.dark = data.dark;
            const actual = child(qam, "upstreamQamActualPlot");
            const reference = child(qam, "upstreamQamReferencePlot");
            tryCompare(actual, "paintedPointCount", 1);
            tryCompare(reference, "paintedPointCount", 1);
            verify(actual.points === host.instrumentState.snapshot.scatter.points);
            verify(reference.points === host.instrumentState.snapshot.scatter.references);
            verify(actual.receivedSamples && !actual.roleMarkers);
            compare(actual.seriesColors.left, theme.accent);
            compare(reference.seriesColors.reference, theme.muted);
            const point = actual.pointPosition(actual.points[0]);
            mouseClick(child(qam, "upstreamQamInspectionArea"), point.x, point.y);
            const text = child(qam, "upstreamQamInspection").text;
            verify(text.indexOf("9007199254740993") >= 0);
            verify(text.indexOf('"missing": null') >= 0);
            compare(qam.presentationCoverage.value.selected, host.instrumentState.snapshot.scatter.points[0]);
            const referencePoint = reference.pointPosition(reference.points[0]);
            verify(qam.inspectAt(referencePoint.x, referencePoint.y));
            compare(child(qam, "upstreamQamInspection").text, "Exact reference\nOriginal reference detail");
        }

        function test_qam_screen_toolbar_drag_wheel_selection_and_reset() {
            openQam();
            const actual = child(qam, "upstreamQamActualPlot");
            const reference = child(qam, "upstreamQamReferencePlot");
            const pointer = child(qam, "upstreamQamInspectionArea");
            const original = JSON.stringify(host.instrumentState);
            const initial = actual.pointPosition(actual.points[0]);
            mouseClick(child(qam, "upstreamQamZoomIn"));
            compare(actual.magnification, 2);
            compare(reference.magnification, actual.magnification);
            const centerX = actual.plotLeft + actual.plotEdge * actual.centerFraction;
            const centerY = actual.plotTop + actual.plotEdge * actual.centerFraction;
            mouseDrag(pointer, centerX, centerY, actual.plotEdge * dragFraction, actual.plotEdge * dragFraction);
            verify(actual.panX > 0 && actual.panY > 0);
            compare(reference.panX, actual.panX);
            compare(reference.panY, actual.panY);
            compare(reference.pointPosition(actual.points[0]), actual.pointPosition(actual.points[0]));
            const transformed = actual.pointPosition(actual.points[0]);
            verify(transformed !== initial);
            mouseClick(pointer, transformed.x, transformed.y);
            compare(qam.presentationCoverage.value.selected, host.instrumentState.snapshot.scatter.points[0]);
            verify(child(qam, "upstreamQamInspection").text.indexOf("9007199254740993") >= 0);
            mouseWheel(pointer, centerX, centerY, 0, wheelZoomDelta);
            compare(actual.magnification, 4);
            compare(reference.magnification, actual.magnification);
            mouseClick(child(qam, "upstreamQamZoomOut"));
            compare(actual.magnification, 2);
            mouseClick(child(qam, "upstreamQamReset"));
            compare(actual.magnification, 1);
            compare(actual.panX, 0);
            compare(actual.panY, 0);
            compare(reference.magnification, 1);
            compare(reference.panX, 0);
            compare(reference.panY, 0);
            compare(actual.pointPosition(actual.points[0]), initial);
            verify(actual.points === host.instrumentState.snapshot.scatter.points);
            verify(reference.points === host.instrumentState.snapshot.scatter.references);
            compare(JSON.stringify(host.instrumentState), original);
            compare(qamSpy.count, 0);
        }

        function test_spectrum_original_viewport_units_and_navigation_signal() {
            openSpectrum();
            compare(spectrumSpy.count, 0);
            const plot = child(spectrum, "upstreamSpectrumPlot");
            verify(plot.viewport === host.instrumentState.snapshot.viewport);
            compare(plot.viewport.axis_label, "Backend PSD units");
            compare(plot.viewport.frequency_label, "Backend frequency units");
            verify(plot.viewport.bands === undefined);
            const navigation = findChild(plot, "chartNavigation");
            navigation.forceActiveFocus();
            keyClick(Qt.Key_Home);
            compare(spectrumSpy.count, 1);
            compare(JSON.stringify(spectrumSpy.signalArguments[0][1]), JSON.stringify({
                tap: "RxEqualized",
                first: 0,
                count: 0
            }));
            compare(spectrumSpy.signalArguments[0][2].query_sequence, "9007199254740993");
            verify(spectrum.pending && !spectrum.actionsEnabled);
        }

        function test_spectrum_tap_and_refresh_preserve_backend_coordinates() {
            openSpectrum();
            const taps = child(spectrum, "upstreamSpectrumTap");
            taps.activated(0);
            compare(spectrum.selection.tap, "TxGrid");
            taps.activated(2);
            compare(spectrum.selection.tap, "TxGrid");
            mouseClick(child(spectrum, "upstreamSpectrumRefresh"));
            compare(JSON.stringify(spectrumSpy.signalArguments[0][1]), JSON.stringify({
                tap: "TxGrid",
                first: 20,
                count: 40
            }));
            refreshState("spectrum", "next-spectrum");
            tryCompare(spectrum, "actionsEnabled", true);
            compare(spectrum.selection.tap, "RxEqualized");
        }

        function test_error_unavailable_loading_and_foreign_kind_disable_actions_data() {
            return [
                {
                    tag: "error",
                    status: "error"
                },
                {
                    tag: "unavailable",
                    status: "unavailable"
                },
                {
                    tag: "loading",
                    status: "loading"
                },
                {
                    tag: "foreign",
                    status: "ready",
                    kind: "spectrum"
                }
            ];
        }
        function test_error_unavailable_loading_and_foreign_kind_disable_actions(data) {
            openQam();
            const value = projection(data.kind || "qam", "blocked-query");
            value.status = data.status;
            value.reason = "<b>Original native diagnostic</b>";
            host.instrumentState = value;
            verify(!qam.actionsEnabled);
            verify(!qam.request());
            compare(child(qam, "upstreamQamStatus").text, value.reason);
            compare(child(qam, "upstreamQamStatus").textFormat, Text.PlainText);
            tryCompare(child(qam, "upstreamQamActualPlot"), "paintedPointCount", 0);
        }

        function test_pressure_preserves_old_owner_and_disables_stale_context_data() {
            return [
                {
                    tag: "qam",
                    kind: "qam"
                },
                {
                    tag: "spectrum",
                    kind: "spectrum"
                }
            ];
        }
        function test_pressure_preserves_old_owner_and_disables_stale_context(data) {
            if (data.kind === "qam")
                openQam();
            else
                openSpectrum();
            const dialog = data.kind === "qam" ? qam : spectrum;
            const previous = dialog.envelope;
            budget.maximumBytes = 0;
            host.instrumentState = projection(data.kind, "rejected-query");
            verify(dialog.presentationRejected);
            verify(dialog.envelope === previous);
            verify(!dialog.contextCurrent && !dialog.actionsEnabled);
            compare(child(dialog, data.kind === "qam" ? "upstreamQamStatus" : "upstreamSpectrumStatus").text, dialog.presentationCoverage.message);
        }

        function test_missing_exact_context_fails_closed_data() {
            return [
                {
                    tag: "qam",
                    kind: "qam"
                },
                {
                    tag: "spectrum",
                    kind: "spectrum"
                }
            ];
        }
        function test_missing_exact_context_fails_closed(data) {
            if (data.kind === "qam")
                openQam();
            else
                openSpectrum();
            const value = projection(data.kind, "missing-context");
            delete value.run;
            host.instrumentState = value;
            const dialog = data.kind === "qam" ? qam : spectrum;
            verify(!dialog.contextAvailable && !dialog.actionsEnabled);
            compare(qamSpy.count, 0);
            compare(spectrumSpy.count, 0);
        }

        function test_request_and_inspection_rejection_emit_nothing() {
            openQam();
            budget.maximumBytes = 0;
            verify(!qam.request());
            compare(qamSpy.count, 0);
            compare(qam.presentationCoverage.value.selected, null);
            verify(!qam.inspectAt(10, 10));
            compare(qam.presentationCoverage.value.selected, null);
        }

        function test_spectrum_error_preserves_original_message_and_disables_navigation() {
            openSpectrum();
            const value = projection("spectrum", "failed-spectrum");
            value.status = "error";
            value.reason = "<b>Original spectrum error</b>";
            value.run = "9007199254740993";
            host.instrumentState = value;
            verify(!spectrum.actionsEnabled);
            verify(!spectrum.requestViewport(20, 40));
            compare(child(spectrum, "upstreamSpectrumStatus").text, value.reason);
            compare(child(spectrum, "upstreamSpectrumStatus").textFormat, Text.PlainText);
            compare(child(spectrum, "upstreamSpectrumPlot").viewport.points.length, 0);
            compare(spectrumSpy.count, 0);
        }

        function test_checked_open_pressure_and_close_reopen_resize() {
            budget.maximumOpen = 0;
            verify(!qam.openChecked());
            verify(!qam.visible);
            budget.maximumOpen = window.fixtureOpen;
            openQam();
            window.width = narrowWidth;
            window.height = narrowHeight;
            tryVerify(function () {
                return qam.width <= window.width - qam.windowMargin * 2;
            });
            tryVerify(function () {
                return qam.height <= window.height - qam.windowMargin * 2;
            });
            keyClick(Qt.Key_Escape);
            tryCompare(qam, "visible", false);
            const held = qam.envelope;
            host.instrumentState = projection("qam", "closed-query");
            verify(qam.envelope === held);
            verify(qam.openChecked());
            tryCompare(qam, "opened", true);
            compare(qam.envelope.query_sequence, "closed-query");
            compare(qamSpy.count, 0);
        }
    }
}
