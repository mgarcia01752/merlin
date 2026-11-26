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

// Presentation-only native projection fixtures; no transfer or estimate science.
Pane {
    id: window
    width: 1200
    height: 920
    padding: 0
    visible: true
    readonly property int fixtureBytes: 8 * 1024 * 1024
    readonly property int fixtureEntries: 100000
    readonly property int fixtureViews: 24
    readonly property int fixtureOpen: 3
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
        targetControl: dialog
        theme: theme
    }
    QtObject {
        id: host
        property var instrumentState: ({})
    }
    UpstreamResponseDialog {
        id: dialog
        theme: theme
        host: host
    }
    SignalSpy {
        id: requested
        target: dialog
        signalName: "instrumentRequested"
    }
    TestCase {
        name: "UpstreamResponsePresentation"
        when: windowShown
        readonly property real narrowWidth: 640
        readonly property real narrowHeight: 480
        function projection(sequence) {
            return {
                status: "ready",
                reason: "",
                query_sequence: sequence || "9007199254740993",
                session_token: "session-9007199254740993",
                configuration_generation: "18446744073709551615",
                run: "9007199254740995",
                snapshot: {
                    kind: "response",
                    status: "ready",
                    message: "",
                    summary: "<b>Configured physical H</b>",
                    metadata_rows: [
                        {
                            label: "Plane",
                            value: "Original configured H"
                        }
                    ],
                    metadata: {
                        grant_id: "18446744073709551615",
                        missing: null,
                        reference_plane: "Original configured H"
                    },
                    choices: {
                        sources: [
                            {
                                value: "physical_channel",
                                label: "Configured physical H"
                            },
                            {
                                value: "cm_pre_eq",
                                label: "Configured transmit correction C"
                            },
                            {
                                value: "composite",
                                label: "Configured deterministic C×H"
                            },
                            {
                                value: "received_estimate",
                                label: "Actual received-pilot estimate",
                                available: true
                            }
                        ],
                        metrics: [
                            {
                                value: "amplitude",
                                label: "Amplitude (dB)"
                            },
                            {
                                value: "phase",
                                label: "Principal phase (rad)"
                            },
                            {
                                value: "impulse",
                                label: "Periodic impulse (relative dB)",
                                available: true
                            }
                        ],
                        frames: [
                            {
                                value: 7,
                                label: "Original retained 7",
                                original: {
                                    retained_index: 7,
                                    frame: "9007199254740993",
                                    identity: {
                                        run: "9007199254740995",
                                        generation: "18446744073709551615",
                                        grant: "9007199254740997"
                                    }
                                }
                            },
                            {
                                value: 19,
                                label: "Original retained 19",
                                original: {
                                    retained_index: 19,
                                    frame: "9007199254740999"
                                }
                            }
                        ]
                    },
                    selection: {
                        source: "physical_channel",
                        metric: "amplitude",
                        grant_symbol: 3,
                        retained_index: 19,
                        first: 100,
                        count: 4
                    },
                    limits: {
                        maximum_points: 77,
                        maximum_impulse_samples: 88,
                        maximum_received_points: 66
                    },
                    viewport: {
                        points: [
                            {
                                x: 0,
                                y: 0.5,
                                minimum_y: 0.5,
                                gap: true,
                                index: 100,
                                value: 0,
                                coordinate: -1,
                                readout: "Original finite ratio",
                                detail: {
                                    amplitude_db: 0,
                                    value: {
                                        re: 1,
                                        im: 0
                                    }
                                }
                            },
                            {
                                x: 0.3333333333333333,
                                y: null,
                                minimum_y: null,
                                gap: true,
                                index: 101,
                                value: null,
                                coordinate: 0,
                                readout: "Exact zero: logarithm unavailable",
                                detail: {
                                    value: {
                                        re: 0,
                                        im: 0
                                    },
                                    amplitude_db: null
                                }
                            },
                            {
                                x: 0.6666666666666666,
                                y: null,
                                minimum_y: null,
                                gap: true,
                                index: 102,
                                value: null,
                                coordinate: 1,
                                readout: "Missing actual estimate",
                                detail: {
                                    value: null,
                                    amplitude_db: null
                                }
                            },
                            {
                                x: 1,
                                y: 0.25,
                                minimum_y: 0.25,
                                gap: true,
                                index: 103,
                                value: 1,
                                coordinate: 2,
                                readout: "Original finite point after gap",
                                detail: {
                                    value: {
                                        re: 2,
                                        im: 0
                                    }
                                }
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
                                y: 0.5,
                                label: "Native ratio"
                            }
                        ],
                        start: 100,
                        count: 4,
                        total: 2048,
                        bands: [],
                        axis_label: "Native amplitude ratio (dB)",
                        frequency_label: "Native frequency units",
                        error: ""
                    }
                }
            };
        }
        function init() {
            dialog.close();
            tryCompare(dialog, "visible", false);
            verify(dialog.presentationCoverage.clear());
            budget.maximumBytes = window.fixtureBytes;
            budget.maximumEntries = window.fixtureEntries;
            budget.maximumOpen = window.fixtureOpen;
            theme.dark = true;
            window.width = 1200;
            window.height = 920;
            host.instrumentState = {};
            requested.clear();
        }
        function cleanup() {
            dialog.close();
        }
        function child(name) {
            return findChild(dialog.contentItem, name);
        }
        function open() {
            host.instrumentState = projection();
            verify(dialog.openChecked());
            tryCompare(dialog, "opened", true);
            tryCompare(dialog, "actionsEnabled", true);
        }
        function choose(name, index) {
            const selector = child(name);
            selector.currentIndex = index;
            selector.activated(index);
        }

        function test_ready_reply_replaces_the_loading_instruments_foreign_selector() {
            open();
            const loading = projection("9007199254740994");
            loading.status = "loading";
            loading.snapshot.kind = "spectrum";
            loading.snapshot.selection = {tap:"TxOutput",first:0,count:0};
            host.instrumentState = loading;
            tryCompare(dialog,"displayable",false);
            const accepted = projection("9007199254740994");
            host.instrumentState = accepted;
            tryCompare(dialog,"actionsEnabled",true);
            verify(dialog.selection === accepted.snapshot.selection);
            verify(child("upstreamResponseRefresh").enabled);
        }

        function test_open_consumes_original_labels_viewport_and_never_queries() {
            open();
            compare(requested.count, 0);
            verify(dialog.envelope === host.instrumentState);
            verify(child("upstreamResponsePlot").viewport === host.instrumentState.snapshot.viewport);
            compare(child("upstreamResponseSource").count, 4);
            compare(child("upstreamResponseMetric").count, 3);
            compare(child("upstreamResponseSummary").text, "<b>Configured physical H</b>");
            compare(child("upstreamResponseSummary").textFormat, Text.PlainText);
            compare(child("upstreamResponsePlot").viewport.frequency_label, "Native frequency units");
        }

        function test_configured_source_metric_requests_data() {
            return [
                {
                    tag: "H-amplitude",
                    source: 0,
                    metric: 0,
                    sourceValue: "physical_channel",
                    metricValue: "amplitude"
                },
                {
                    tag: "C-phase",
                    source: 1,
                    metric: 1,
                    sourceValue: "cm_pre_eq",
                    metricValue: "phase"
                },
                {
                    tag: "composite-impulse",
                    source: 2,
                    metric: 2,
                    sourceValue: "composite",
                    metricValue: "impulse"
                }
            ];
        }
        function test_configured_source_metric_requests(data) {
            open();
            choose("upstreamResponseSource", data.source);
            choose("upstreamResponseMetric", data.metric);
            mouseClick(child("upstreamResponseRefresh"));
            compare(requested.count, 1);
            compare(requested.signalArguments[0][0], "response");
            compare(JSON.stringify(requested.signalArguments[0][1]), JSON.stringify({
                source: data.sourceValue,
                metric: data.metricValue,
                grant_symbol: 3,
                retained_index: 19,
                first: 100,
                count: 4
            }));
            const context = requested.signalArguments[0][2];
            compare(Object.keys(context).sort().join(","), "configuration_generation,query_sequence,run,session_token");
            compare(context.query_sequence, "9007199254740993");
            compare(context.configuration_generation, "18446744073709551615");
            compare(context.run, "9007199254740995");
            verify(dialog.pending && !dialog.actionsEnabled);
            host.instrumentState = projection();
            verify(dialog.pending);
            host.instrumentState = projection("9007199254740994");
            tryCompare(dialog, "actionsEnabled", true);
        }

        function test_frame_selector_uses_actual_retained_index_and_original_identity() {
            open();
            choose("upstreamResponseSource", 3);
            const frames = child("upstreamResponseFrame");
            verify(frames.visible);
            compare(frames.currentIndex, 1);
            choose("upstreamResponseFrame", 0);
            compare(dialog.selection.retained_index, 7);
            mouseClick(child("upstreamResponseRefresh"));
            compare(requested.signalArguments[0][1].retained_index, 7);
            compare(dialog.choices.frames[0].original.frame, "9007199254740993");
            compare(dialog.choices.frames[0].original.identity.grant, "9007199254740997");
        }

        function test_inner_unavailable_allows_source_metric_recovery() {
            open();
            const value = projection("unavailable-estimate");
            value.snapshot.status = "unavailable";
            value.snapshot.message = "Original actual inverse unavailable";
            value.snapshot.selection.source = "received_estimate";
            value.snapshot.selection.metric = "impulse";
            value.snapshot.choices.metrics[2].available = false;
            value.snapshot.viewport = {
                points: [],
                x_ticks: [],
                y_ticks: []
            };
            host.instrumentState = value;
            verify(dialog.actionsEnabled && !dialog.displayable);
            verify(child("upstreamResponseSource").enabled && child("upstreamResponseMetric").enabled);
            verify(!dialog.canRequest());
            compare(child("upstreamResponsePlot").visible, false);
            choose("upstreamResponseMetric", 0);
            choose("upstreamResponseSource", 0);
            verify(dialog.canRequest());
            mouseClick(child("upstreamResponseRefresh"));
            compare(requested.signalArguments[0][1].source, "physical_channel");
            compare(requested.signalArguments[0][1].metric, "amplitude");
        }

        function test_null_zero_missing_details_and_gap_flags_data() {
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
        function test_null_zero_missing_details_and_gap_flags(data) {
            open();
            theme.dark = data.dark;
            const plot = child("upstreamResponsePlot");
            const points = host.instrumentState.snapshot.viewport.points;
            verify(plot.viewport.points === points);
            compare(points[0].value, 0);
            compare(points[1].y, null);
            compare(points[2].y, null);
            verify(points[3].gap);
            plot.selectedIndex = 1;
            tryVerify(function () {
                return child("upstreamResponseInspection").text.indexOf("Exact zero") >= 0;
            });
            verify(child("upstreamResponseInspection").text.indexOf('"re": 0') >= 0);
            plot.selectedIndex = 2;
            tryVerify(function () {
                return child("upstreamResponseInspection").text.indexOf("Missing actual estimate") >= 0;
            });
            verify(child("upstreamResponseInspection").text.indexOf('"value": null') >= 0);
            compare(dialog.presentationCoverage.value.selected, points[2]);
        }

        function test_native_all_missing_error_plaintext_and_original_metadata() {
            open();
            const value = projection("no-finite");
            value.snapshot.viewport.error = "<b>No finite readings; original diagnostic</b>";
            value.snapshot.viewport.points = value.snapshot.viewport.points.slice(1, 3);
            value.snapshot.viewport.y_ticks = [];
            host.instrumentState = value;
            compare(child("upstreamResponseStatus").text, value.snapshot.viewport.error);
            compare(child("upstreamResponseStatus").textFormat, Text.PlainText);
            child("upstreamResponseDetails").checked = true;
            verify(child("upstreamResponseMetadata").text.indexOf("18446744073709551615") >= 0);
            verify(child("upstreamResponseMetadata").text.indexOf('"missing": null') >= 0);
        }

        function test_navigation_forwards_exact_bounded_original_window() {
            open();
            const plot = child("upstreamResponsePlot");
            const navigation = findChild(plot, "chartNavigation");
            navigation.forceActiveFocus();
            keyClick(Qt.Key_Home);
            compare(requested.count, 1);
            compare(requested.signalArguments[0][1].first, 0);
            compare(requested.signalArguments[0][1].count, 0);
            compare(requested.signalArguments[0][1].grant_symbol, 3);
            compare(requested.signalArguments[0][1].retained_index, 19);
        }

        function test_outer_blocked_or_missing_context_disables_every_request_data() {
            return [
                {
                    tag: "error",
                    status: "error"
                },
                {
                    tag: "loading",
                    status: "loading"
                },
                {
                    tag: "unavailable",
                    status: "unavailable"
                },
                {
                    tag: "missing-context",
                    status: "ready",
                    missing: true
                }
            ];
        }
        function test_outer_blocked_or_missing_context_disables_every_request(data) {
            open();
            const value = projection("blocked");
            value.status = data.status;
            value.reason = "Original native reason";
            if (data.missing)
                delete value.run;
            host.instrumentState = value;
            verify(!dialog.actionsEnabled);
            verify(!dialog.requestViewport(100, 4));
            compare(requested.count, 0);
            verify(!child("upstreamResponseSource").enabled);
        }

        function test_pressure_preserves_old_owner_and_rejects_requests() {
            open();
            const previous = dialog.envelope;
            budget.maximumBytes = 0;
            verify(!dialog.requestViewport(100, 4));
            compare(requested.count, 0);
            host.instrumentState = projection("pressure");
            verify(dialog.presentationRejected);
            verify(dialog.envelope === previous);
            verify(!dialog.contextCurrent && !dialog.actionsEnabled);
            compare(child("upstreamResponseStatus").text, dialog.presentationCoverage.message);
        }

        function test_hidden_retention_checked_open_and_resize() {
            budget.maximumOpen = 0;
            verify(!dialog.openChecked());
            budget.maximumOpen = window.fixtureOpen;
            open();
            dialog.close();
            tryCompare(dialog, "visible", false);
            const previous = dialog.envelope;
            host.instrumentState = projection("closed-query");
            verify(dialog.envelope === previous);
            window.width = narrowWidth;
            window.height = narrowHeight;
            verify(dialog.openChecked());
            tryCompare(dialog, "opened", true);
            compare(dialog.envelope.query_sequence, "closed-query");
            tryVerify(function () {
                return dialog.width <= window.width - dialog.windowMargin * 2;
            });
            tryVerify(function () {
                return dialog.height <= window.height - dialog.windowMargin * 2;
            });
            keyClick(Qt.Key_Escape);
            tryCompare(dialog, "visible", false);
            compare(requested.count, 0);
        }
    }
}
