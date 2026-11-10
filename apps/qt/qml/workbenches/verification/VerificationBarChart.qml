/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls

Rectangle {
    id: chartPanel
    required property var theme
    required property string heading
    property var chart: ({points: []})
    property bool horizontal: false
    property string emptyText: "No retained comparison data."
    radius: 10; color: theme.panel; border.color: theme.line
    Label { x: 14; y: 13; text: chartPanel.heading; color: chartPanel.theme.text; font.bold: true; font.pixelSize: 13 }
    Canvas {
        id: plot
        anchors.fill: parent; anchors.topMargin: 40; anchors.margins: 14
        Accessible.role: Accessible.Graphic
        Accessible.name: chartPanel.heading
        Accessible.description: (chartPanel.chart.points || []).map(point => point.label + ": " + point.count).join(", ") || chartPanel.emptyText
        onWidthChanged: requestPaint()
        onHeightChanged: requestPaint()
        Connections { target: chartPanel; function onChartChanged() { plot.requestPaint() } }
        Connections { target: chartPanel.theme; function onDarkChanged() { plot.requestPaint() } }
        onPaint: {
            const ctx = getContext("2d")
            ctx.reset(); ctx.font = "11px sans-serif"
            const points = chartPanel.chart.points || []
            if (!points.length) {
                return
            }
            if (chartPanel.horizontal) {
                for (let index = 0; index < points.length; ++index) {
                    const point = points[index], y = 12 + index * 60
                    ctx.fillStyle = chartPanel.theme.muted; ctx.textAlign = "left"
                    ctx.fillText(point.label, 0, y)
                    ctx.textAlign = "right"; ctx.fillStyle = chartPanel.theme.text
                    ctx.fillText(point.count, width, y)
                    ctx.fillStyle = chartPanel.theme.line; ctx.fillRect(0, y + 10, width, 12)
                    ctx.fillStyle = point.attention ? chartPanel.theme.statusAttention : chartPanel.theme.accent
                    ctx.fillRect(0, y + 10, width * point.fraction, 12)
                }
            } else {
                const left = 30, top = 12, bottom = height - 27
                const step = (width - left) / points.length
                ctx.fillStyle = chartPanel.theme.muted
                ctx.fillText(chartPanel.chart.maximum || "0", 0, top)
                ctx.fillText("0", 0, bottom + 3)
                ctx.strokeStyle = chartPanel.theme.line
                ctx.beginPath(); ctx.moveTo(left, top); ctx.lineTo(left, bottom); ctx.lineTo(width, bottom); ctx.stroke()
                for (let index = 0; index < points.length; ++index) {
                    const point = points[index], barHeight = (bottom - top) * point.fraction
                    ctx.fillStyle = point.attention ? chartPanel.theme.statusAttention : chartPanel.theme.accent
                    if (barHeight > 0) {
                        ctx.fillRect(left + index * step + 1, bottom - barHeight, Math.max(1, step - 2), barHeight)
                    } else {
                        ctx.fillRect(left + index * step + 1, bottom - 1, Math.max(1, step - 2), 1)
                    }
                }
                ctx.fillStyle = chartPanel.theme.muted; ctx.textAlign = "center"
                ctx.fillText("Received payload sequence · " + points[0].label + "–" + points[points.length - 1].label, left + (width - left) / 2, height - 4)
                if (chartPanel.chart.maximum === "0") {
                    ctx.fillStyle = chartPanel.theme.accent
                    ctx.fillText("Zero errors in retained comparisons", left + (width - left) / 2, (top + bottom) / 2)
                }
            }
        }
    }
    Label {
        anchors.centerIn: parent; anchors.verticalCenterOffset: 12
        width: parent.width - 36; wrapMode: Text.WordWrap
        horizontalAlignment: Text.AlignHCenter; color: chartPanel.theme.muted
        visible: !(chartPanel.chart.points || []).length; text: chartPanel.emptyText
    }
}
