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

// Transport/presentation fixtures; actual public reports and native runs own science.
Pane {
    id: window
    width: 1280
    height: 800
    visible: true
    readonly property int fixtureBytes: 1000000
    readonly property int fixtureEntries: 100000
    PresentationBudget {
        id: budget
        maximumBytes: window.fixtureBytes
        maximumEntries: window.fixtureEntries
        maximumViews: 8
        maximumOpen: 1
    }
    Theme { id: fixtureTheme; presentationBackendFactory: budget.createAdapter }
    ThemePalette { targetControl: window; theme: fixtureTheme }
    QtObject { id: nativeHost; property var instrumentState: ({}) }
    Component {
        id: factory
        UpstreamFecDialog { theme: fixtureTheme; host: nativeHost }
    }
    SignalSpy { id: requested; target: cases.popup; signalName: "instrumentRequested" }
    TestCase {
        id: cases
        name: "UpstreamFecObservation"
        when: windowShown
        property var popup: null

        function fixture() {
            return {status: "ready", reason: "", query_sequence: "3",
                session_token: "upstream/18446744073709551614/9007199254740993",
                configuration_generation: "9007199254740993", run: "18446744073709551615",
                snapshot: {kind: "fec", status: "ready", message: "",
                    selection: {mode: "words", grant_index: 0, first_word: 0, maximum_words: 1, max_decision_bits: 262144},
                    choices: {modes: [{value: "words", label: "Completed LDPC words"},
                            {value: "soft_cells", label: "Raw / derandomized cell LLRs"},
                            {value: "soft_word", label: "Completed LDPC input LLRs"}],
                        grants: [{value: 0, label: "Started grant 0"}], soft_words: [{value: 0, label: "Retained word 0"}],
                        mode_requests: {words: {mode: "words", grant_index: 0, first_word: 0, maximum_words: 1, max_decision_bits: 262144},
                            soft_cells: {mode: "soft_cells", first_index: 0, maximum_cells: 8, maximum_llrs: 4096},
                            soft_word: {mode: "soft_word", retained_index: 0, maximum_llrs: 32768, first_bit: 0, maximum_bits: 64}}},
                    page: {first: 0, previous: null, next: 1, total: 2},
                    table: {headings: ["Word", "Family", "Iterations"], rows: [["0", "upstream_short", "2"]]},
                    chart: {points: [{label: "0", count: "2", fraction: 1, attention: false}], maximum: "2"},
                    summary: "Original completed upstream LDPC outcomes; source verification remains independent.",
                    metadata_rows: [{label: "Source seed", value: "1"}, {label: "BER", value: "Unavailable without compared bits"}]}};
        }

        function init() {
            budget.maximumBytes = window.fixtureBytes;
            budget.maximumEntries = window.fixtureEntries;
            compare(budget.bytes, 0);
            compare(budget.views, 0);
            requested.clear();
            nativeHost.instrumentState = fixture();
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

        function open() {
            popup = createTemporaryObject(factory, window);
            verify(popup !== null);
            verify(popup.openChecked());
            tryCompare(popup, "opened", true);
            tryCompare(popup, "actionsEnabled", true);
            verify(waitForRendering(popup.contentItem));
        }

        function test_actual_shaped_table_chart_and_wide_context_are_original_aliases() {
            open();
            const table = findChild(popup, "upstreamFecTable");
            const chart = findChild(popup, "upstreamFecIterations");
            verify(table.rows === nativeHost.instrumentState.snapshot.table.rows);
            verify(table.headings === nativeHost.instrumentState.snapshot.table.headings);
            verify(chart.chart === nativeHost.instrumentState.snapshot.chart);
            compare(chart.categoryAxisLabel, "Original completed-word index");
            verify(!chart.zeroMessage.toLowerCase().includes("zero errors"));
            verify(!table.headings.includes("BCH"));
            findChild(popup, "upstreamFecRefresh").clicked();
            compare(requested.count, 1);
            compare(requested.signalArguments[0][0], "fec");
            compare(requested.signalArguments[0][1], nativeHost.instrumentState.snapshot.selection);
            compare(requested.signalArguments[0][2], {query_sequence: "3", session_token: "upstream/18446744073709551614/9007199254740993",
                configuration_generation: "9007199254740993", run: "18446744073709551615"});
        }

        function test_mode_dispatch_uses_native_defaults_and_loading_reply_adopts_its_selector() {
            open();
            verify(popup.requestMode(1));
            compare(requested.signalArguments[0][1], nativeHost.instrumentState.snapshot.choices.mode_requests.soft_cells);
            verify(!popup.actionsEnabled);
            const loading = fixture();
            loading.query_sequence = "4";
            loading.status = "loading";
            nativeHost.instrumentState = loading;
            compare(popup.selection.mode, "words");
            verify(!popup.actionsEnabled);
            const ready = fixture();
            ready.query_sequence = "4";
            ready.snapshot.selection = ready.snapshot.choices.mode_requests.soft_cells;
            ready.snapshot.table = {headings: ["Raw LLR", "Derandomized LLR"], rows: [["-0.75", "0.75"]]};
            ready.snapshot.page = {first: 8, previous: 0, next: 16, total: 64};
            nativeHost.instrumentState = ready;
            tryCompare(popup, "actionsEnabled", true);
            compare(popup.selection.mode, "soft_cells");
            compare(findChild(popup, "upstreamFecMode").currentIndex, 1);
            verify(!findChild(popup, "upstreamFecIterations").visible);
            findChild(popup, "upstreamFecNext").clicked();
            compare(requested.signalArguments[1][1], {mode: "soft_cells", first_index: 16, maximum_cells: 8, maximum_llrs: 4096});
        }

        function test_native_page_coordinates_are_forwarded_for_distinct_word_input_plane() {
            open();
            const value = fixture();
            value.query_sequence = "5";
            value.snapshot.selection = value.snapshot.choices.mode_requests.soft_word;
            value.snapshot.selection.first_bit = 64;
            value.snapshot.page = {first: 64, previous: 0, next: 128, total: 4096};
            value.snapshot.table = {headings: ["Original bit", "LDPC input LLR"], rows: [["64", "-0.5"]]};
            nativeHost.instrumentState = value;
            tryCompare(popup, "actionsEnabled", true);
            findChild(popup, "upstreamFecPrevious").clicked();
            compare(requested.signalArguments[0][1].first_bit, 0);
            verify(!Object.prototype.hasOwnProperty.call(requested.signalArguments[0][1], "first_word"));
            verify(!popup.requestPage(-1));
            verify(!popup.requestPage(0.5));
        }

        function test_unchanged_notification_preserves_existing_witness_and_table() {
            open();
            const owner = popup.presentationCoverage.value;
            const source = findChild(popup, "upstreamFecTable").rows;
            nativeHost.instrumentStateChanged();
            verify(popup.presentationCoverage.value === owner);
            verify(findChild(popup, "upstreamFecTable").rows === source);
        }

        function test_actual_pointer_drag_scrolls_bounded_original_soft_rows() {
            // Independent scroll fixture population and pointer geometry, not
            // receiver counts or scientific scalar limits.
            const fixtureRows = 96, pointerTargetFraction = 0.75;
            const value = fixture();
            value.snapshot.selection = value.snapshot.choices.mode_requests.soft_cells;
            value.snapshot.table = {headings: ["Original bit", "Raw LLR"], rows: []};
            for (let index = 0; index < fixtureRows; ++index) {
                value.snapshot.table.rows.push([String(index), "0.75"]);
            }
            nativeHost.instrumentState = value;
            open();
            const list = findChild(popup, "upstreamFecRows"), bar = list.ScrollBar.vertical;
            list.forceLayout();
            tryVerify(() => list.contentHeight > list.height && bar.width > 0 && bar.height > 0);
            const source = findChild(popup, "upstreamFecTable").rows;
            const middle = bar.width / 2, thumbMiddle = bar.contentItem.y + bar.contentItem.height / 2;
            mouseMove(bar, middle, thumbMiddle);
            mousePress(bar, middle, thumbMiddle);
            verify(bar.pressed, "The real scrollbar handle must accept the pointer before dragging: " + JSON.stringify({
                visible:bar.visible,enabled:bar.enabled,interactive:bar.interactive,width:bar.width,height:bar.height,
                position:bar.position,size:bar.size,point:bar.mapToItem(window,middle,thumbMiddle),
                tableWidth:list.width,tableHeight:list.height,contentHeight:list.contentHeight,
                thumbY:bar.contentItem.y,thumbHeight:bar.contentItem.height}));
            mouseMove(bar, middle, bar.height * pointerTargetFraction);
            mouseRelease(bar, middle, bar.height * pointerTargetFraction);
            tryVerify(() => list.contentY > 0);
            verify(findChild(popup, "upstreamFecTable").rows === source);
        }

        function test_busy_or_unavailable_history_has_no_fabricated_zero_or_new_dispatch() {
            open();
            const unavailable = fixture();
            unavailable.snapshot.status = "unavailable";
            unavailable.snapshot.message = "Original soft history was not retained.";
            unavailable.snapshot.table = {headings: [], rows: []};
            unavailable.snapshot.chart = {points: [], maximum: "0"};
            nativeHost.instrumentState = unavailable;
            compare(findChild(popup, "upstreamFecTable").emptyText, unavailable.snapshot.message);
            compare(findChild(popup, "upstreamFecTable").rows.length, 0);
            verify(popup.requestMode(0));
            const busy = fixture(); busy.status = "loading";
            nativeHost.instrumentState = busy;
            verify(!popup.actionsEnabled);
            verify(!popup.requestMode(2));
            compare(requested.count, 1);
        }

        function test_rejected_display_keeps_original_rows_and_prevents_foreign_context_dispatch() {
            open();
            const source = popup.envelope;
            budget.maximumBytes = budget.bytes;
            const foreign = fixture();
            foreign.session_token = "upstream/999/999";
            foreign.snapshot.metadata_rows.push({label: "New source", value: "X".repeat(window.fixtureBytes)});
            nativeHost.instrumentState = foreign;
            tryCompare(popup, "presentationRejected", true);
            verify(popup.envelope === source);
            verify(!popup.contextCurrent);
            verify(!popup.requestMode(1));
            compare(requested.count, 0);
            budget.maximumBytes = window.fixtureBytes;
            nativeHost.instrumentState = fixture();
            tryCompare(popup, "actionsEnabled", true);
        }
    }
}
