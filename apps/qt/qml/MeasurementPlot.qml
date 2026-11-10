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
ColumnLayout {
    id: chart
    required property var theme
    property var viewport: ({points: [], x_ticks: [], y_ticks: []})
    property string readout: ""
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
            ctx.strokeStyle = chart.theme.accent; ctx.lineWidth = 1.2
            let previous = null
            for (const point of chart.viewport.points || []) {
                if (point.y === null || point.minimum_y === null) { previous = null; continue }
                const x = left + point.x * width, y = top + point.y * height
                ctx.beginPath(); ctx.moveTo(x, y); ctx.lineTo(x, top + point.minimum_y * height); ctx.stroke()
                if (previous && !previous.gap && !point.gap) {
                    ctx.beginPath(); ctx.moveTo(left + previous.x * width, top + previous.y * height); ctx.lineTo(x, y); ctx.stroke()
                }
                ctx.fillStyle = chart.theme.accent; ctx.fillRect(x - 1, y - 1, 2, 2)
                previous = point
            }
        }
        MouseArea {
            anchors.fill: parent; hoverEnabled: true
            onPositionChanged: function(mouse) {
                const points = chart.viewport.points || []
                const fraction = Math.max(0, Math.min(1, (mouse.x - 52) / Math.max(1, width - 70)))
                const index = Math.min(points.length - 1, Math.floor(fraction * points.length))
                chart.readout = index >= 0 ? points[index].readout : ""
            }
        }
        Label { anchors.centerIn: parent; visible: !(chart.viewport.points || []).length; text: chart.viewport.error || "Select an input to view"; color: theme.muted; width: parent.width - 50; wrapMode: Text.WordWrap; horizontalAlignment: Text.AlignHCenter }
    }
    Label { text: chart.viewport.frequency_label || "Frequency (MHz)"; color: theme.muted; font.pixelSize: 11; Layout.alignment: Qt.AlignHCenter }
    Label { text: chart.readout || chart.viewport.summary || ""; color: theme.text; font.pixelSize: 11; Layout.fillWidth: true; elide: Text.ElideRight }
}
