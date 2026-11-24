/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../qml/shared/theme"
import "../qml/shared/instruments"

// Literal screen coordinates exercise rendering only, without numerical science.
Pane {
    id: window
    width: 640
    height: 480
    visible: true
    padding: 0
    Theme { id: theme }
    SpectrumPlot {
        id: plot
        anchors.fill: parent
        theme: theme
        showDetails: false
    }
    TestCase {
        name: "SharedSpectrumPlot"
        when: windowShown
        readonly property int paintSettleMs: 60
        readonly property real centerFraction: 0.5

        function viewport(points) {
            return {points: points, x_ticks: [], y_ticks: [], bands: [],
                axis_label: "Original backend response (dB)", frequency_label: "Original frequency"};
        }

        function centerAlpha() {
            const canvas = findChild(plot, "saCanvas");
            verify(canvas && canvas.available);
            canvas.requestPaint();
            waitForRendering(canvas);
            wait(paintSettleMs);
            const x = canvas.leftMargin + (canvas.width - canvas.leftMargin - canvas.rightMargin) * centerFraction;
            const y = canvas.topMargin + (canvas.height - canvas.topMargin - canvas.bottomMargin) * centerFraction;
            return canvas.getContext("2d").getImageData(Math.floor(x), Math.floor(y), 1, 1).data[3];
        }

        function test_original_null_hole_has_no_joined_line_or_finite_floor() {
            const left = {x: 0, y: 0.25, minimum_y: 0.25, readout: "Original left"};
            const right = {x: 1, y: 0.75, minimum_y: 0.75, readout: "Original right"};
            plot.viewport = viewport([left, right]);
            verify(centerAlpha() > 0, "Positive control must draw the original continuous line.");
            const absent = {x: 0.5, y: null, minimum_y: null, readout: "Missing original estimate"};
            plot.viewport = viewport([left, absent, right]);
            compare(centerAlpha(), 0, "A missing original value must break the curve.");
            plot.selectedIndex = 1;
            compare(plot.selectedPoint, absent);
            compare(centerAlpha(), 0, "Selecting a missing value must not invent a marker or floor.");
            compare(plot.viewport.points[1].y, null);
        }

        function test_exact_zero_coordinate_is_drawn_and_selection_preserves_original_details() {
            const zero = {x: 0.5, y: 0, minimum_y: 0, power_per_hz: 0, readout: "Actual zero"};
            plot.viewport = viewport([zero]);
            plot.selectedIndex = 0;
            compare(plot.selectedPoint, zero);
            compare(plot.selectedPoint.power_per_hz, 0);
            const canvas = findChild(plot, "saCanvas");
            compare(canvas.Accessible.name, "Original backend response (dB)");
            compare(canvas.Accessible.description, "Actual zero");
            waitForRendering(canvas);
        }

        function test_extended_axis_displays_complete_original_scientific_tick_labels() {
            const canvas = findChild(plot, "saCanvas");
            // Historical downstream axis geometry remains the default.
            compare(canvas.leftMargin, 60);
            plot.valueAxisWidth = plot.extendedValueAxisWidth;
            const value = viewport([]);
            const original = "9.9999e-308";
            value.y_ticks = [{y: 0.5, label: original}];
            plot.viewport = value;
            canvas.requestPaint();
            waitForRendering(canvas);
            wait(paintSettleMs);
            // Original text plus the existing nine-pixel label gap must fit.
            verify(canvas.getContext("2d").measureText(original).width + 9 < canvas.leftMargin);
            compare(plot.viewport.y_ticks[0].label, original);
            plot.valueAxisWidth = plot.compactValueAxisWidth;
        }
    }
}
