/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls

Item {
    id: chart
    required property var theme
    property var histogram: ({points:[],maximum:0})
    Canvas {
        id: plot
        anchors.fill: parent
        Accessible.role: Accessible.Graphic
        Accessible.name: "LDPC iterations · Whole run"
        Accessible.description: (chart.histogram.points || []).map(point => point.iteration + " iterations: " + point.count + " codewords").join(", ") || "No completed codewords."
        onWidthChanged: requestPaint()
        onHeightChanged: requestPaint()
        Connections { target: chart; function onHistogramChanged() { plot.requestPaint() } }
        Connections { target: chart.theme; function onDarkChanged() { plot.requestPaint() } }
        onPaint: {
            const ctx = getContext("2d")
            ctx.reset()
            const points = chart.histogram.points || []
            if (!points.length || width < 100 || height < 80) { return }
            const left = 54, top = 22, bottom = height - 36, span = width - left - 12
            ctx.font = "11px sans-serif"
            ctx.fillStyle = chart.theme.muted
            ctx.fillText("Codewords", 0, 12)
            ctx.textAlign = "right"
            for (let tick = 0; tick <= 1; ++tick) {
                const y = bottom - tick * (bottom - top)
                ctx.strokeStyle = chart.theme.line
                ctx.beginPath(); ctx.moveTo(left, y); ctx.lineTo(left + span, y); ctx.stroke()
                ctx.fillText(String(chart.histogram.maximum * tick), left - 8, y + 4)
            }
            const step = span / points.length
            const labelEvery = Math.max(1, Math.ceil(points.length / Math.max(2, Math.floor(span / 48))))
            ctx.textAlign = "center"
            for (let index = 0; index < points.length; ++index) {
                const point = points[index], x = left + index * step
                const barHeight = (bottom - top) * point.fraction
                ctx.fillStyle = chart.theme.accent
                ctx.fillRect(x + step * 0.15, bottom - barHeight, step * 0.7, barHeight)
                ctx.fillStyle = chart.theme.text
                if (points.length <= 8 && point.count > 0) { ctx.fillText(String(point.count), x + step / 2, bottom - barHeight - 5) }
                if (index % labelEvery === 0 || index === points.length - 1) {
                    ctx.fillStyle = chart.theme.muted
                    ctx.fillText(String(point.iteration), x + step / 2, bottom + 15)
                }
            }
            ctx.fillStyle = chart.theme.muted
            ctx.fillText("Iterations", left + span / 2, height - 3)
        }
    }
    Label { anchors.centerIn: parent; visible: !(chart.histogram.points || []).length; text: "No completed codewords."; color: chart.theme.muted }
    MouseArea {
        id: hover
        anchors.fill: parent; hoverEnabled: true
        property int pointIndex: mouseX < 54 || mouseX >= width - 12 ? -1 : Math.floor((mouseX - 54) / (width - 66) * (chart.histogram.points || []).length)
        property var point: (chart.histogram.points || [])[pointIndex]
    }
    ToolTip.visible: hover.containsMouse && !!hover.point
    ToolTip.text: hover.point ? hover.point.iteration + " iterations · " + hover.point.count + " codewords" : ""
}
