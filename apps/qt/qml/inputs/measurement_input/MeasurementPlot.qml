/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
// Adapted from PNM Viewer's RxMerBitLoadingChart Canvas: Rust now supplies all
// signal coordinates/units and bounded min/max groups for both input kinds.
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
ColumnLayout {
    id: chart
    required property var theme
    property var viewport: ({points: [], x_ticks: [], y_ticks: []})
    property string readout: ""
    property var roleColors: ({})
    function pointColor(point) { return roleColors[point.role] || theme.accent }
    signal viewportRequested(int start, int count)
    onViewportChanged: { readout = ""; canvas.requestPaint() }
    Label { text: chart.viewport.axis_label || "Measurement"; color: theme.muted; font.pixelSize: 12 }
    Canvas {
        id: canvas; objectName: "measurementChart"
        Layout.fillWidth: true; Layout.fillHeight: true; Layout.minimumHeight: 100
        Accessible.role: Accessible.Graphic
        Accessible.name: chart.viewport.axis_label || "Measurement frequency plot"
        Accessible.description: chart.readout || chart.viewport.summary || "No measurement"
        onWidthChanged: requestPaint()
        onHeightChanged: requestPaint()
        Connections { target: chart.theme; function onDarkChanged() { canvas.requestPaint() } }
        onPaint: {
            const ctx = getContext("2d")
            ctx.reset()
            const left = 52, top = 12, width = canvas.width - 70, height = canvas.height - 45
            if (width <= 0 || height <= 0) { return }
            ctx.fillStyle = chart.theme.background; ctx.fillRect(0, 0, canvas.width, canvas.height)
            ctx.font = "11px sans-serif"; ctx.lineWidth = 1
            for (const tick of chart.viewport.y_ticks || []) {
                const y = top + tick.x * height
                ctx.strokeStyle = chart.theme.line; ctx.beginPath(); ctx.moveTo(left, y); ctx.lineTo(left + width, y); ctx.stroke()
                ctx.fillStyle = chart.theme.muted; ctx.textAlign = "right"; ctx.fillText(tick.label, left - 6, y + 4)
            }
            for (const tick of chart.viewport.x_ticks || []) {
                const x = left + tick.x * width
                ctx.strokeStyle = chart.theme.line; ctx.beginPath(); ctx.moveTo(x, top); ctx.lineTo(x, top + height); ctx.stroke()
                ctx.fillStyle = chart.theme.muted; ctx.textAlign = tick.x === 0 ? "left" : tick.x === 1 ? "right" : "center"
                ctx.fillText(tick.label, x, top + height + 20)
            }
            for (const band of chart.viewport.bands || []) {
                ctx.fillStyle = Qt.rgba(0.95, 0.74, 0.25, 0.24)
                ctx.fillRect(left + band.first * width, top, (band.last - band.first) * width, height)
                ctx.strokeStyle = Qt.rgba(0.95,0.74,0.25,0.75);ctx.lineWidth=1
                ctx.beginPath();ctx.moveTo(left+band.first*width,top);ctx.lineTo(left+band.first*width,top+height)
                ctx.moveTo(left+band.last*width,top);ctx.lineTo(left+band.last*width,top+height);ctx.stroke()
                ctx.fillStyle = chart.theme.muted; ctx.textAlign = "left"
                ctx.fillText(band.label, Math.min(left + band.first * width + 3, left + width - 25), top + 12)
            }
            // Both curves use Rust's shared axis; the original is a dashed muted trace.
            ctx.strokeStyle = chart.theme.muted; ctx.lineWidth = 1; ctx.setLineDash([4,4])
            let comparisonPrevious = null
            for (const point of chart.viewport.comparison || []) {
                if (point.y === null || point.minimum_y === null) {comparisonPrevious=null;continue}
                const x=left+point.x*width, y=top+point.y*height
                ctx.beginPath();ctx.moveTo(x,y);ctx.lineTo(x,top+point.minimum_y*height);ctx.stroke()
                if (comparisonPrevious && !comparisonPrevious.gap && !point.gap) {
                    ctx.beginPath();ctx.moveTo(left+comparisonPrevious.x*width,top+comparisonPrevious.y*height);ctx.lineTo(x,y);ctx.stroke()
                }
                comparisonPrevious=point
            }
            ctx.setLineDash([])
            ctx.strokeStyle = chart.theme.accent; ctx.lineWidth = 1.2
            let previous = null
            for (const point of chart.viewport.points || []) {
                if (point.y === null || point.minimum_y === null) { previous = null; continue }
                const x = left + point.x * width, y = top + point.y * height
                ctx.strokeStyle = chart.pointColor(point)
                ctx.beginPath(); ctx.moveTo(x, y); ctx.lineTo(x, top + point.minimum_y * height); ctx.stroke()
                if (previous && !previous.gap && !point.gap && previous.role === point.role) {
                    ctx.beginPath(); ctx.moveTo(left + previous.x * width, top + previous.y * height)
                    if (chart.viewport.step) { ctx.lineTo(x, top + previous.y * height) }
                    ctx.lineTo(x, y); ctx.stroke()
                }
                const marker = chart.viewport.step ? 3 : 2
                ctx.fillStyle = chart.pointColor(point); ctx.fillRect(x - marker / 2, y - marker / 2, marker, marker)
                previous = point
            }
        }
        ChartNavigation {
            anchors.fill: parent
            theme: chart.theme; viewport: chart.viewport
            leftMargin: 52; rightMargin: 18; topMargin: 12; bottomMargin: 33
            onRangeRequested: function(start, count) { chart.viewportRequested(start, count) }
            onPointHovered: function(fraction) {
                const points = chart.viewport.points || []
                let index = -1
                let distance = Infinity
                for (let candidate = 0; candidate < points.length; candidate++) {
                    const delta = Math.abs(points[candidate].x - fraction)
                    if (delta < distance) { index = candidate; distance = delta }
                }
                chart.readout = index >= 0 ? points[index].readout : ""
            }
        }
        Label { anchors.centerIn: parent; visible: !(chart.viewport.points || []).length; text: chart.viewport.error || "Select an input to view"; color: theme.muted; width: parent.width - 50; wrapMode: Text.WordWrap; horizontalAlignment: Text.AlignHCenter }
    }
    Label { text: chart.viewport.frequency_label || "Frequency (MHz)"; color: theme.muted; font.pixelSize: 11; Layout.alignment: Qt.AlignHCenter }
    Label { text: chart.readout || chart.viewport.summary || ""; color: theme.text; font.pixelSize: 11; Layout.fillWidth: true; elide: Text.ElideRight }
}
