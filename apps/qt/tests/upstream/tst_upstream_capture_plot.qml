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

// Presentation fixtures only. Native decoder/request tests establish source values.
Pane {
    id: window
    width: 1280
    height: 780
    visible: true
    padding: 0
    readonly property int fixtureBytes: 100000
    readonly property int fixtureEntries: 10000
    PresentationBudget {
        id: budget
        maximumBytes: window.fixtureBytes
        maximumEntries: window.fixtureEntries
        maximumViews: 8
        maximumOpen: 1
    }
    Theme {
        id: fixtureTheme
        presentationBackendFactory: budget.createAdapter
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
        id: requested
        target: tests.popup
        signalName: "plotRequested"
    }
    TestCase {
        id: tests
        name: "UpstreamOriginalCapturePlot"
        when: windowShown
        property var popup: null

        /** Native-shaped coordinates, explicitly including one missing original. */
        function fixture() {
            return {
                title: "Original RxMER capture",
                summary: "Inspection only; unassociated original source.",
                reference_plane: "Original equalized upstream probe relative to ideal BPSK.",
                metadata_rows: [{label: "Original binary SHA-256", value: "fixture-source-hash"}],
                columns: ["Physical subcarrier", "RxMER (dB)", "Raw octet (hex)"],
                data_rows: [{coordinate: "130", first: "Unavailable", second: "ff"}],
                page: {has_previous: false, has_next: false, previous_first: 0, next_first: 0},
                error: "",
                plot: {
                    points: [
                        {x: 0, y: 1, minimum_y: 1, gap: false, record: 1, physical_carrier: 129, rxmer_db: 0, readout: "Physical carrier 129 · 0 dB"},
                        {x: 0.5, y: null, minimum_y: null, gap: true, record: 2, physical_carrier: 130, rxmer_db: null, readout: "Physical carrier 130 · Unavailable original reading"},
                        {x: 1, y: 0, minimum_y: 0, gap: false, record: 3, physical_carrier: 131, rxmer_db: 63.5, readout: "Physical carrier 131 · 63.5 dB"}
                    ],
                    x_ticks: [{x: 0, label: "129"}, {x: 1, label: "131"}],
                    y_ticks: [{x: 0, label: "63.5"}, {x: 1, label: "0"}],
                    bands: [], start: 1, count: 3, total: 8, maximum_count: 4096,
                    axis_label: "Original upstream RxMER (dB)",
                    frequency_label: "Original physical subcarrier index",
                    summary: "Original records 2–4 of 8; unassociated inspection only"
                }
            };
        }

        function init() {
            compare(budget.bytes, 0);
            compare(budget.entries, 0);
            compare(budget.views, 0);
            compare(budget.open, 0);
            budget.maximumBytes = window.fixtureBytes;
            budget.maximumEntries = window.fixtureEntries;
            requested.clear();
        }

        function cleanup() {
            budget.maximumBytes = window.fixtureBytes;
            budget.maximumEntries = window.fixtureEntries;
            if (popup) {
                popup.close();
                tryCompare(popup, "visible", false);
            }
            popup = null;
        }

        function open(value, family) {
            popup = createTemporaryObject(dialogComponent, window, {family: family || "rxmer"});
            verify(popup !== null);
            popup.presentation = value;
            tryCompare(popup, "contextCurrent", true);
            verify(popup.openChecked());
            tryCompare(popup, "opened", true);
            verify(waitForRendering(popup.contentItem));
            return popup;
        }

        function test_shared_plot_aliases_exact_original_null_and_selected_details() {
            const source = fixture();
            open(source);
            const chart = findChild(popup, "upstreamCapturePlot");
            verify(chart !== null);
            verify(chart.viewport === source.plot);
            verify(popup.plot === source.plot);
            verify(chart.viewport.points === source.plot.points);
            compare(chart.viewport.points[0].rxmer_db, 0);
            compare(chart.viewport.points[1].rxmer_db, null);
            compare(chart.viewport.points[1].y, null);
            compare(chart.viewport.points[1].physical_carrier, 130);
            const navigation = findChild(chart, "chartNavigation");
            verify(navigation !== null);
            navigation.pointHovered(0.5);
            compare(chart.readout, "Physical carrier 130 · Unavailable original reading");
            navigation.pointHovered(0);
            compare(chart.readout, "Physical carrier 129 · 0 dB");
            // Native host polling can notify an unchanged original alias. Its
            // existing admission witness and hovered reading must survive.
            popup.presentationChanged();
            compare(chart.readout, "Physical carrier 129 · 0 dB");
            verify(chart.viewport === source.plot);
            compare(popup.dataRows[0].second, "ff");
        }

        function test_navigation_forwards_original_bounds_without_a_presentation_copy() {
            open(fixture());
            verify(popup.presentationCoverage.textAdmitted);
            // Independent nonnegative i32 transport anchor, not a capture limit.
            compare(findChild(popup, "upstreamCapturePlotFirst").contentItem.maximumLength, 10);
            compare(findChild(popup, "upstreamCapturePlotCount").contentItem.maximumLength, 10);
            const bytes = budget.bytes;
            const entries = budget.entries;
            const chart = findChild(popup, "upstreamCapturePlot");
            chart.viewportRequested(2, 4);
            compare(requested.count, 1);
            compare(requested.signalArguments[0][0], "rxmer");
            compare(requested.signalArguments[0][1], 2);
            compare(requested.signalArguments[0][2], 4);
            findChild(popup, "upstreamCapturePlotFull").clicked();
            compare(requested.count, 2);
            compare(requested.signalArguments[1][1], 0);
            compare(requested.signalArguments[1][2], 0);
            compare(findChild(popup, "upstreamCapturePlotFirst").to, 7);
            compare(findChild(popup, "upstreamCapturePlotCount").to, 7);
            compare(budget.bytes, bytes);
            compare(budget.entries, entries);
            for (const range of [[-1, 1], [8, 0], [0, 4097], [7, 2], [0.5, 1], [0, -1]]) {
                verify(!popup.requestPlot(range[0], range[1]));
            }
            compare(requested.count, 2);
        }

        function test_busy_or_rejected_candidate_keeps_original_alias_and_blocks_dispatch() {
            const source = fixture();
            open(source);
            popup.busy = true;
            verify(!popup.requestPlot(0, 1));
            popup.busy = false;
            const chart = findChild(popup, "upstreamCapturePlot");
            const bytes = budget.bytes;
            const entries = budget.entries;
            budget.maximumBytes = bytes;
            const candidate = fixture();
            candidate.plot.points[0].physical_carrier = 777;
            popup.presentation = candidate;
            tryCompare(popup, "presentationRejected", true);
            verify(chart.viewport === source.plot);
            compare(chart.viewport.points[0].physical_carrier, 129);
            verify(!popup.requestPlot(0, 1));
            compare(requested.count, 0);
            compare(budget.bytes, bytes);
            compare(budget.entries, entries);
            budget.maximumBytes = window.fixtureBytes;
            popup.presentation = source;
            verify(!popup.presentationRejected);
            verify(popup.actionsAvailable);
            popup.presentation = candidate;
            verify(popup.adoptPresentation());
            verify(chart.viewport === candidate.plot);
            verify(popup.requestPlot(0, 1));
            compare(requested.count, 1);
        }

        function test_empty_original_view_has_no_fake_zero_or_request() {
            const source = fixture();
            source.plot.points = [];
            source.plot.x_ticks = [];
            source.plot.y_ticks = [];
            source.plot.start = 0;
            source.plot.count = 0;
            source.plot.total = 0;
            source.plot.summary = "No original RxMER records; unassociated inspection only.";
            source.data_rows = [];
            open(source);
            verify(popup.hasPlot);
            verify(!popup.plotAvailable);
            compare(popup.plot.points.length, 0);
            compare(popup.dataRows.length, 0);
            verify(!popup.requestPlot(0, 0));
            compare(requested.count, 0);
            verify(!findChild(popup, "upstreamCapturePlotFull").enabled);
        }

        function test_foreign_family_cannot_dispatch_an_rxmer_plot() {
            open(fixture(), "pre_eq");
            verify(!popup.hasPlot);
            verify(!popup.plotAvailable);
            verify(!popup.requestPlot(0, 1));
            compare(requested.count, 0);
        }
    }
}
