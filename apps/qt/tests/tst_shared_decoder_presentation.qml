/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtTest
import "../qml/shared/theme"
import "../qml/shared/instruments"
import "../qml/downstream/instruments/fec"
import "../qml/downstream/receiver"

// Presentation fixtures only. Counts/fractions and formatted strings are supplied
// inputs; none of these cases establishes scientific or native acceptance.
Pane {
    id: window
    width: 1000
    height: 720
    visible: true
    padding: 0
    Theme {
        id: testTheme
    }
    ThemePalette {
        targetControl: window
        theme: testTheme
    }
    ThemePalette {
        targetControl: window.Window.window
        theme: testTheme
    }

    Component {
        id: tableComponent
        ObservationTable {
            theme: testTheme
            width: 840
            height: 260
        }
    }
    Component {
        id: fecComponent
        FecCodewordTable {
            theme: testTheme
            width: 840
            height: 300
        }
    }
    Component {
        id: countComponent
        CountBarChart {
            theme: testTheme
            heading: "Reported counts"
            width: 420
            height: 260
        }
    }
    Component {
        id: verificationComponent
        VerificationBarChart {
            theme: testTheme
            heading: "Independent source comparison"
            width: 420
            height: 260
        }
    }
    TestCase {
        name: "SharedDecoderPresentation"
        when: windowShown
        readonly property int renderSettleMs: 50
        readonly property int traversalLimit: 4096
        readonly property int fixtureRowCount: 512
        readonly property int maximumVisibleFixtureRows: 32
        readonly property int zeroMessageBandHeight: 24
        readonly property point horizontalAccentPixel: Qt.point(50, 28)
        readonly property point horizontalAttentionPixel: Qt.point(50, 88)
        readonly property point verticalAccentPixel: Qt.point(50, 160)
        readonly property point verticalAttentionPixel: Qt.point(250, 50)

        function init() {
            testTheme.dark = true;
        }

        /** Inspect bounded actual visual descendants, including ListView delegates. */
        function descendants(root) {
            const pending = [root], values = [];
            while (pending.length) {
                const item = pending.pop();
                verify(values.length < traversalLimit);
                values.push(item);
                for (const child of item.children || []) {
                    pending.push(child);
                }
            }
            return values;
        }

        /** Find the actual displayed label rather than reconstructing its output. */
        function label(root, text) {
            return descendants(root).find(item => item.text === text && item.color !== undefined);
        }

        /** Locate the extracted table without requiring a new wrapper API. */
        function observationTable(root) {
            return descendants(root).find(item => item.rowsObjectName === "receiverFecWords");
        }

        /** Wait for the real Canvas update before inspecting rendered pixels. */
        function rendered(chart) {
            const canvas = findChild(chart, "countBarCanvas");
            verify(canvas);
            canvas.requestPaint();
            waitForRendering(canvas);
            wait(renderSettleMs);
            return grabImage(canvas);
        }

        /** Original DS fields include independent BCH and optional source comparison. */
        function words() {
            return [
                {
                    receive_sequence: "9007199254740993",
                    first_source_symbol: "18446744073709551614",
                    last_source_symbol: "18446744073709551615",
                    ldpc_success: true,
                    ldpc_iterations: 2,
                    bch_success: true,
                    bch_corrected_bits: 0,
                    comparison: {
                        bit_errors: 0
                    }
                },
                {
                    receive_sequence: 2,
                    first_source_symbol: 3,
                    last_source_symbol: 4,
                    ldpc_success: false,
                    ldpc_iterations: 5,
                    bch_success: true,
                    bch_corrected_bits: 0,
                    comparison: {
                        bit_errors: 0
                    }
                },
                {
                    receive_sequence: 3,
                    first_source_symbol: 5,
                    last_source_symbol: 6,
                    ldpc_success: true,
                    ldpc_iterations: 1,
                    bch_success: false,
                    bch_corrected_bits: 7,
                    comparison: {
                        bit_errors: 0
                    }
                },
                {
                    receive_sequence: 4,
                    first_source_symbol: 7,
                    last_source_symbol: 8,
                    ldpc_success: true,
                    ldpc_iterations: 1,
                    bch_success: true,
                    bch_corrected_bits: 0,
                    comparison: {
                        bit_errors: 1
                    }
                },
                {
                    receive_sequence: 5,
                    first_source_symbol: 9,
                    last_source_symbol: 10,
                    ldpc_success: true,
                    ldpc_iterations: null,
                    bch_success: true,
                    bch_corrected_bits: null,
                    comparison: null
                }
            ];
        }

        function test_table_original_string_aliases_and_plain_text() {
            const headings = ["Original word", "Native detail"];
            const rows = [["18446744073709551615", "<b>Unchanged native text</b>"], ["0", "Fail"]];
            const original = JSON.stringify({
                headings: headings,
                rows: rows
            });
            const table = createTemporaryObject(tableComponent, window.contentItem, {
                headings: headings,
                rows: rows
            });
            verify(table);
            // Test the actual QML binding path after component-construction transport.
            table.headings = headings;
            table.rows = rows;
            verify(table.headings === headings);
            verify(table.rows === rows);
            const list = findChild(table, "observationRows");
            tryCompare(list, "count", rows.length);
            list.forceLayout();
            tryVerify(() => !!label(list, rows[0][1]));
            compare(label(list, rows[0][0]).text, "18446744073709551615");
            compare(label(list, rows[0][1]).textFormat, Text.PlainText);
            compare(label(list, "Fail").color, testTheme.statusError);
            compare(JSON.stringify({
                headings: headings,
                rows: rows
            }), original);
        }

        function test_table_custom_list_name_and_virtualized_rows() {
            const rows = [];
            for (let index = 0; index < fixtureRowCount; ++index) {
                rows.push(["original-" + index, "reported-" + index]);
            }
            const table = createTemporaryObject(tableComponent, window.contentItem, {
                headings: ["Original index", "Reported value"],
                rows: rows,
                rowsObjectName: "fixtureRows"
            });
            table.rows = rows;
            const list = findChild(table, "fixtureRows");
            verify(list);
            tryCompare(list, "count", fixtureRowCount);
            list.forceLayout();
            verify(table.rows === rows);
            const retainedDelegates = descendants(list.contentItem).filter(item => item.values !== undefined);
            verify(retainedDelegates.length > 0);
            verify(retainedDelegates.length < maximumVisibleFixtureRows);
            list.positionViewAtEnd();
            list.forceLayout();
            tryVerify(() => !!label(list, "original-" + (fixtureRowCount - 1)));
            verify(table.rows === rows);
        }

        function test_table_empty_and_null_values() {
            const table = createTemporaryObject(tableComponent, window.contentItem, {
                headings: [],
                rows: [],
                emptyText: "Native observation unavailable"
            });
            const list = findChild(table, "observationRows");
            tryCompare(list, "count", 0);
            verify(label(list, "Native observation unavailable").visible);
            table.headings = null;
            table.rows = null;
            tryCompare(list, "count", 0);
            verify(label(list, "Native observation unavailable").visible);
            table.headings = ["Original coordinate"];
            table.rows = [["0"]];
            tryCompare(list, "count", 1);
            tryVerify(() => !!label(list, "0"));
            compare(label(list, "Native observation unavailable").visible, false);
        }

        function test_downstream_prepare_keeps_bch_comparison_and_source_indices() {
            const source = words(), original = JSON.stringify(source);
            const table = createTemporaryObject(fecComponent, window.contentItem);
            const projection = table.prepare(source, 9);
            verify(projection.words === source);
            compare(projection.headings, ["Word", "Source Symbols", "LDPC", "Iterations", "BCH", "BCH Corrections", "Bit Errors"]);
            compare(projection.all[0], ["9007199254740993", "18446744073709551614–18446744073709551615", "Pass", "2", "Pass", "0", "0"]);
            compare(projection.all[2], ["3", "5–6", "Pass", "1", "Fail", "7", "0"]);
            compare(projection.all[4], ["5", "9–10", "Pass", "", "Pass", "", "—"]);
            compare(projection.summary, "Whole run · 5 retained · 9 omitted");
            compare(projection.filtered.length, 3);
            verify(projection.filtered[0] === source[1]);
            verify(projection.filtered[1] === source[2]);
            verify(projection.filtered[2] === source[3]);
            compare(JSON.stringify(source), original);
        }

        function test_downstream_errors_only_forwards_actual_list_and_aliases() {
            const table = createTemporaryObject(fecComponent, window.contentItem);
            table.projection = table.prepare(words(), 0);
            const shared = observationTable(table), list = findChild(table, "receiverFecWords");
            verify(shared && list);
            verify(shared.headings === table.headings);
            verify(shared.rows === table.projection.all);
            tryCompare(list, "count", 5);
            mouseClick(findChild(table, "fecErrorsOnly"));
            compare(table.errorsOnly, true);
            verify(shared.rows === table.projection.errors);
            verify(table.filtered === table.projection.filtered);
            tryCompare(list, "count", 3);
            mouseClick(findChild(table, "fecErrorsOnly"));
            tryCompare(list, "count", 5);
            verify(shared.rows === table.projection.all);
        }

        function test_downstream_empty_filter_wording() {
            const table = createTemporaryObject(fecComponent, window.contentItem);
            table.projection = table.prepare([], 0);
            const list = findChild(table, "receiverFecWords");
            tryCompare(list, "count", 0);
            verify(label(list, "No retained codewords.").visible);
            table.projection = table.prepare([words()[0]], 0);
            mouseClick(findChild(table, "fecErrorsOnly"));
            tryCompare(list, "count", 0);
            verify(label(list, "No errors in retained records.").visible);
        }

        function test_count_chart_neutral_defaults_and_original_alias() {
            const value = {
                points: [
                    {
                        label: "9007199254740993",
                        count: "18446744073709551615",
                        fraction: 1,
                        attention: false
                    }
                ],
                maximum: "18446744073709551615"
            };
            const original = JSON.stringify(value);
            const chart = createTemporaryObject(countComponent, window.contentItem, {
                chart: value
            });
            chart.chart = value;
            verify(chart.chart === value);
            verify(chart.chart.points === value.points);
            compare(chart.categoryAxisLabel, "");
            compare(chart.zeroMessage, "");
            compare(chart.emptyText, "No retained count data.");
            rendered(chart);
            compare(JSON.stringify(value), original);
            compare(descendants(chart).filter(item => typeof item.getContext === "function").length, 1);
        }

        function test_downstream_chart_wording_and_alias() {
            const value = {
                points: [
                    {
                        label: "7",
                        count: "0",
                        fraction: 0,
                        attention: false
                    }
                ],
                maximum: "0"
            };
            const chart = createTemporaryObject(verificationComponent, window.contentItem, {
                chart: value
            });
            chart.chart = value;
            verify(chart.chart === value);
            compare(chart.categoryAxisLabel, "Received payload sequence");
            compare(chart.zeroMessage, "Zero errors in retained comparisons");
            compare(chart.emptyText, "No retained comparison data.");
            rendered(chart);
        }

        function test_upstream_zero_iterations_does_not_claim_zero_errors() {
            const value = {
                points: [
                    {
                        label: "9007199254740993",
                        count: "0",
                        fraction: 0,
                        attention: false
                    }
                ],
                maximum: "0"
            };
            const upstreamChart = createTemporaryObject(countComponent, window.contentItem, {
                heading: "Completed-word iterations",
                categoryAxisLabel: "Original completed-word index",
                chart: value
            });
            const downstreamChart = createTemporaryObject(verificationComponent, window.contentItem, {
                x: 440,
                chart: value
            });
            upstreamChart.chart = value;
            downstreamChart.chart = value;
            verify(downstreamChart.chart.maximum === "0");
            compare(downstreamChart.zeroMessage, "Zero errors in retained comparisons");
            const upstreamImage = rendered(upstreamChart);
            upstreamChart.visible = false;
            downstreamChart.x = 0;
            const downstreamImage = rendered(downstreamChart);
            compare(upstreamChart.zeroMessage, "");
            compare(upstreamChart.categoryAxisLabel, "Original completed-word index");
            const canvas = findChild(upstreamChart, "countBarCanvas");
            const center = (upstreamChart.verticalTopInset + canvas.height - upstreamChart.verticalBottomInset) / 2;
            let upstreamMessagePixels = 0, messageDifferencePixels = 0;
            for (let y = Math.floor(center) - zeroMessageBandHeight; y <= Math.floor(center); ++y) {
                for (let x = 0; x < canvas.width; ++x) {
                    upstreamMessagePixels += Qt.colorEqual(upstreamImage.pixel(x, y), testTheme.accent) ? 1 : 0;
                    messageDifferencePixels += !Qt.colorEqual(upstreamImage.pixel(x, y), downstreamImage.pixel(x, y)) ? 1 : 0;
                }
            }
            compare(upstreamMessagePixels, 0);
            verify(messageDifferencePixels > 0);
        }

        function test_count_empty_and_null_states() {
            const chart = createTemporaryObject(countComponent, window.contentItem, {
                emptyText: "No completed-word observation",
                chart: null
            });
            verify(label(chart, "No completed-word observation").visible);
            rendered(chart);
            chart.chart = {};
            verify(label(chart, "No completed-word observation").visible);
            chart.chart = {
                points: []
            };
            verify(label(chart, "No completed-word observation").visible);
            chart.chart = {
                points: [
                    {
                        label: "0",
                        count: "0",
                        fraction: 0,
                        attention: false
                    }
                ],
                maximum: "0"
            };
            compare(label(chart, "No completed-word observation").visible, false);
            rendered(chart);
        }

        function test_count_colors_and_geometry_data() {
            return [
                {
                    tag: "dark-vertical",
                    dark: true,
                    horizontal: false
                },
                {
                    tag: "light-vertical",
                    dark: false,
                    horizontal: false
                },
                {
                    tag: "dark-horizontal",
                    dark: true,
                    horizontal: true
                },
                {
                    tag: "light-horizontal",
                    dark: false,
                    horizontal: true
                }
            ];
        }

        function test_count_colors_and_geometry(data) {
            testTheme.dark = data.dark;
            const value = {
                points: [
                    {
                        label: "41",
                        count: "5",
                        fraction: 0.5,
                        attention: false
                    },
                    {
                        label: "99",
                        count: "10",
                        fraction: 1,
                        attention: true
                    }
                ],
                maximum: "10"
            };
            const chart = createTemporaryObject(verificationComponent, window.contentItem, {
                horizontal: data.horizontal,
                chart: value
            });
            const canvas = findChild(chart, "countBarCanvas"), image = rendered(chart);
            compare(canvas.x, 14);
            compare(canvas.y, 40);
            compare(canvas.width, 392);
            compare(canvas.height, 206);
            const accent = data.horizontal ? horizontalAccentPixel : verticalAccentPixel;
            const attention = data.horizontal ? horizontalAttentionPixel : verticalAttentionPixel;
            compare(image.pixel(accent.x, accent.y), testTheme.accent);
            compare(image.pixel(attention.x, attention.y), testTheme.statusAttention);
            compare(grabImage(chart).pixel(chart.width - 20, 20), testTheme.panel);
            chart.chart = value;
            verify(chart.chart === value);
        }

        function test_table_themes_and_resize_data() {
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

        function test_table_themes_and_resize(data) {
            testTheme.dark = data.dark;
            const rows = [["41", "Pass"], ["99", "Fail"]];
            const table = createTemporaryObject(tableComponent, window.contentItem, {
                headings: ["Original word", "Outcome"],
                rows: rows
            });
            table.rows = rows;
            const list = findChild(table, "observationRows");
            tryCompare(list, "count", 2);
            tryVerify(() => !!label(list, "Fail"));
            compare(label(list, "Fail").color, testTheme.statusError);
            compare(label(list, "Pass").color, testTheme.text);
            compare(label(table, "Original word").color, testTheme.accent);
            table.width = 320;
            waitForRendering(table);
            verify(label(list, "Fail").width <= table.width);
            verify(table.rows === rows);
            compare(grabImage(table).width, 320);
            table.width = 840;
            compare(grabImage(table).width, 840);
        }
    }
}
