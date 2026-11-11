/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
// Adapted from PNM Viewer MeasurementChart.qml modulation overlays and lanes.
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
ColumnLayout {
    id: chart
    required property var theme
    required property var viewport
    required property var roleColors
    required property var profileColor
    property string readout: ""
    signal viewportRequested(int start, int count)
    onViewportChanged: { readout = ""; canvas.requestPaint() }
    Canvas {
        id: canvas
        objectName: "profileChart"
        Layout.fillWidth: true; Layout.fillHeight: true; Layout.minimumHeight: 120
        readonly property real plotBottomMargin: 40 + (chart.viewport.lanes || []).length * 15
        Accessible.role: Accessible.Graphic
        Accessible.name: "Modulation profiles by frequency"
        Accessible.description: chart.readout || chart.viewport.summary || ""
        onWidthChanged: requestPaint()
        onHeightChanged: requestPaint()
        Connections { target: chart.theme; function onDarkChanged() { canvas.requestPaint() } }
        onPaint: {
            const ctx = getContext("2d"); ctx.reset()
            const left = 58, top = 22, w = width - left - 18, h = height - top - plotBottomMargin
            if (w <= 0 || h <= 0) return
            ctx.font = "11px sans-serif"; ctx.lineWidth = 1
            ctx.fillStyle = chart.theme.muted; ctx.fillText(chart.viewport.axis_label || "dB", left, 12)
            for (const tick of chart.viewport.y_ticks || []) {
                const y = top + tick.x * h
                ctx.strokeStyle = chart.theme.line; ctx.beginPath(); ctx.moveTo(left,y); ctx.lineTo(left+w,y); ctx.stroke()
                ctx.textAlign = "right"; ctx.fillStyle = chart.theme.muted; ctx.fillText(tick.label,left-9,y+4)
            }
            for (const tick of chart.viewport.x_ticks || []) {
                ctx.textAlign = tick.x === 0 ? "left" : tick.x === 1 ? "right" : "center"
                ctx.fillText(tick.label,left+tick.x*w,height-20)
            }
            ctx.textAlign = "center"; ctx.fillText("Frequency · MHz",left+w/2,height-3)
            for (const series of chart.viewport.series || []) {
                ctx.strokeStyle = chart.profileColor(series.id); ctx.fillStyle = ctx.strokeStyle
                let previous = null
                for (const p of series.points) {
                    if (p.y === null) { previous = null; continue }
                    const x = left+p.x*w, y = top+p.y*h
                    ctx.beginPath(); ctx.moveTo(x,y); ctx.lineTo(x,top+p.minimum_y*h); ctx.stroke()
                    if (previous && !p.gap && !previous.gap) { ctx.beginPath(); ctx.moveTo(left+previous.x*w,top+previous.y*h); ctx.lineTo(x,top+previous.y*h); ctx.lineTo(x,y); ctx.stroke() }
                    ctx.fillRect(x-1,y-1,2,2); previous=p
                }
            }
            const lanes = chart.viewport.lanes || [], labels = {continuous_pilot:"Pilot",plc:"PLC",exclusion:"Excl.",zero_bit_loaded:"Zero",reserved:"Res.",unknown:"Unk."}
            for (let i=0;i<lanes.length;i++) {
                const y=top+h+12+i*15
                ctx.strokeStyle=chart.theme.line; ctx.beginPath(); ctx.moveTo(left,y); ctx.lineTo(left+w,y); ctx.stroke()
                ctx.fillStyle=chart.theme.muted; ctx.textAlign="right"; ctx.fillText(labels[lanes[i]],left-8,y+3)
            }
            for (const m of chart.viewport.markers || []) {
                const y=top+h+12+lanes.indexOf(m.role)*15
                ctx.strokeStyle=chart.roleColors[m.role]; ctx.lineWidth=4
                ctx.beginPath(); ctx.moveTo(left+m.x*w-1,y); ctx.lineTo(left+m.last_x*w+1,y); ctx.stroke()
            }
        }
        ChartNavigation {
            anchors.fill: parent; theme: chart.theme; viewport: chart.viewport
            leftMargin: 58; rightMargin: 18; topMargin: 22; bottomMargin: canvas.plotBottomMargin
            onRangeRequested: function(start,count) { chart.viewportRequested(start,count) }
            onPointHovered: function(fraction) {
                let nearest = null, distance = Infinity
                for (const s of chart.viewport.series || []) for (const p of s.points) {
                    const delta = Math.abs(p.x-fraction)
                    if (p.y !== null && delta < distance) { distance=delta; nearest=p }
                }
                chart.readout = nearest ? nearest.readout : ""
            }
        }
    }
    Label { text: chart.readout || chart.viewport.summary || ""; color: chart.theme.muted; Layout.fillWidth: true; elide: Text.ElideRight; font.family: "monospace"; font.pixelSize: 11 }
}
