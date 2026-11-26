/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls

// Drawing only. Count labels and normalized fractions are supplied and admitted
// by the caller; no count, decoder outcome or comparison metric is calculated.
Rectangle {
    id: chartPanel
    required property var theme
    required property string heading
    property var chart: ({
            points: []
        })
    property string descriptionText: ""
    property bool horizontal: false
    property string emptyText: "No retained count data."
    property string categoryAxisLabel: ""
    property string zeroMessage: ""

    // Independent display geometry, preserving the original DS chart pixels.
    readonly property real panelCornerRadius: 10
    readonly property real headingLeftInset: 14
    readonly property real headingTopInset: 13
    readonly property int headingFontPixels: 13
    readonly property real plotOuterMargin: 14
    readonly property real plotTopMargin: 40
    readonly property int plotFontPixels: 11
    readonly property real horizontalLabelBaseline: 12
    readonly property real horizontalRowPitch: 60
    readonly property real horizontalBarTopOffset: 10
    readonly property real horizontalBarHeight: 12
    readonly property real valueAxisWidth: 30
    readonly property real verticalTopInset: 12
    readonly property real verticalBottomInset: 27
    readonly property real zeroLabelBaselineOffset: 3
    readonly property real barLeftInset: 1
    readonly property real barGap: 2
    readonly property real minimumBarWidth: 1
    readonly property real zeroBarHeight: 1
    readonly property real categoryLabelBottomInset: 4
    readonly property real centerFraction: 0.5
    readonly property real emptyVerticalOffset: 12
    readonly property real emptyHorizontalInset: 18

    radius: panelCornerRadius
    color: theme.panel
    border.color: theme.line
    Label {
        x: chartPanel.headingLeftInset
        y: chartPanel.headingTopInset
        text: chartPanel.heading
        color: chartPanel.theme.text
        font.bold: true
        font.pixelSize: chartPanel.headingFontPixels
    }
    Canvas {
        id: plot
        objectName: "countBarCanvas"
        anchors.fill: parent
        anchors.topMargin: chartPanel.plotTopMargin
        anchors.margins: chartPanel.plotOuterMargin
        Accessible.role: Accessible.Graphic
        Accessible.name: chartPanel.heading
        Accessible.description: chartPanel.descriptionText || chartPanel.emptyText
        onWidthChanged: requestPaint()
        onHeightChanged: requestPaint()
        Connections {
            target: chartPanel
            function onChartChanged() {
                plot.requestPaint();
            }
            function onHorizontalChanged() {
                plot.requestPaint();
            }
            function onCategoryAxisLabelChanged() {
                plot.requestPaint();
            }
            function onZeroMessageChanged() {
                plot.requestPaint();
            }
        }
        Connections {
            target: chartPanel.theme
            function onDarkChanged() {
                plot.requestPaint();
            }
        }
        onPaint: {
            const ctx = getContext("2d");
            ctx.reset();
            ctx.font = chartPanel.plotFontPixels + "px sans-serif";
            const points = chartPanel.chart ? chartPanel.chart.points || [] : [];
            if (!points.length) {
                return;
            }
            if (chartPanel.horizontal) {
                for (let index = 0; index < points.length; ++index) {
                    const point = points[index], y = chartPanel.horizontalLabelBaseline + index * chartPanel.horizontalRowPitch;
                    ctx.fillStyle = chartPanel.theme.muted;
                    ctx.textAlign = "left";
                    ctx.fillText(point.label, 0, y);
                    ctx.textAlign = "right";
                    ctx.fillStyle = chartPanel.theme.text;
                    ctx.fillText(point.count, width, y);
                    ctx.fillStyle = chartPanel.theme.line;
                    ctx.fillRect(0, y + chartPanel.horizontalBarTopOffset, width, chartPanel.horizontalBarHeight);
                    ctx.fillStyle = point.attention ? chartPanel.theme.statusAttention : chartPanel.theme.accent;
                    ctx.fillRect(0, y + chartPanel.horizontalBarTopOffset, width * point.fraction, chartPanel.horizontalBarHeight);
                }
            } else {
                const left = chartPanel.valueAxisWidth, top = chartPanel.verticalTopInset, bottom = height - chartPanel.verticalBottomInset;
                const step = (width - left) / points.length;
                ctx.fillStyle = chartPanel.theme.muted;
                ctx.fillText(chartPanel.chart.maximum || "0", 0, top);
                ctx.fillText("0", 0, bottom + chartPanel.zeroLabelBaselineOffset);
                ctx.strokeStyle = chartPanel.theme.line;
                ctx.beginPath();
                ctx.moveTo(left, top);
                ctx.lineTo(left, bottom);
                ctx.lineTo(width, bottom);
                ctx.stroke();
                for (let index = 0; index < points.length; ++index) {
                    const point = points[index], barHeight = (bottom - top) * point.fraction;
                    ctx.fillStyle = point.attention ? chartPanel.theme.statusAttention : chartPanel.theme.accent;
                    if (barHeight > 0) {
                        ctx.fillRect(left + index * step + chartPanel.barLeftInset, bottom - barHeight, Math.max(chartPanel.minimumBarWidth, step - chartPanel.barGap), barHeight);
                    } else {
                        ctx.fillRect(left + index * step + chartPanel.barLeftInset, bottom - chartPanel.zeroBarHeight, Math.max(chartPanel.minimumBarWidth, step - chartPanel.barGap), chartPanel.zeroBarHeight);
                    }
                }
                ctx.fillStyle = chartPanel.theme.muted;
                ctx.textAlign = "center";
                const center = left + (width - left) * chartPanel.centerFraction;
                const range = points[0].label + "–" + points[points.length - 1].label;
                ctx.fillText((chartPanel.categoryAxisLabel ? chartPanel.categoryAxisLabel + " · " : "") + range, center, height - chartPanel.categoryLabelBottomInset);
                if (chartPanel.chart.maximum === "0" && chartPanel.zeroMessage) {
                    ctx.fillStyle = chartPanel.theme.accent;
                    ctx.fillText(chartPanel.zeroMessage, center, (top + bottom) * chartPanel.centerFraction);
                }
            }
        }
    }
    Label {
        anchors.centerIn: parent
        anchors.verticalCenterOffset: chartPanel.emptyVerticalOffset
        width: parent.width - chartPanel.emptyHorizontalInset * 2
        wrapMode: Text.WordWrap
        horizontalAlignment: Text.AlignHCenter
        color: chartPanel.theme.muted
        visible: !chartPanel.chart || !(chartPanel.chart.points || []).length
        text: chartPanel.emptyText
    }
}
