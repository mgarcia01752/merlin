/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"

ColumnLayout {
    id: chart
    required property var theme
    property var viewport: ({points: [], x_ticks: [], y_ticks: []})
    property int selectedIndex: 0
    property bool zoomEnabled: false
    property string detailsText: ""
    property bool showDetails: true
    signal viewportRequested(int start, int count)
    readonly property var selectedPoint: viewport.points.length > 0 ? viewport.points[Math.max(0, Math.min(selectedIndex, viewport.points.length - 1))] : null
    readonly property real selectedFraction: selectedPoint ? selectedPoint.x : 0.5
    onViewportChanged: { selectedIndex = Math.max(0, Math.min(selectedIndex, viewport.points.length - 1)); plot.requestPaint() }
    onSelectedIndexChanged: plot.requestPaint()
    spacing: 6
    Label { text: chart.viewport.axis_label || "Normalized power spectral density"; color: theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap }
    Rectangle {
        Layout.fillWidth: true; Layout.fillHeight: true
        color: theme.background; border.color: theme.line; radius: 4
        Canvas {
            id: plot
            objectName: "saCanvas"
            anchors.fill: parent
            readonly property real leftMargin: 60
            readonly property real rightMargin: 20
            readonly property real topMargin: 20
            readonly property real bottomMargin: 40
            focus: true
            Accessible.role: Accessible.Graphic
            Accessible.name: "OFDM sample spectrum"
            Accessible.description: chart.selectedPoint ? chart.selectedPoint.readout : "Generate or select a sample snapshot to display its spectrum."
            Keys.onLeftPressed: chart.selectedIndex = Math.max(0, chart.selectedIndex - 1)
            Keys.onRightPressed: chart.selectedIndex = Math.min(chart.viewport.points.length - 1, chart.selectedIndex + 1)
            onWidthChanged: requestPaint()
            onHeightChanged: requestPaint()
            Connections { target: chart.theme; function onDarkChanged() { plot.requestPaint() } }
            onPaint: {
                const ctx = getContext("2d")
                ctx.reset()
                const width = plot.width - leftMargin - rightMargin
                const height = plot.height - topMargin - bottomMargin
                if (width <= 0 || height <= 0) { return }
                ctx.font = "12px sans-serif"
                ctx.lineWidth = 1
                ctx.strokeStyle = chart.theme.line
                ctx.fillStyle = chart.theme.muted
                for (const tick of chart.viewport.y_ticks) {
                    const y = topMargin + tick.y * height
                    ctx.beginPath(); ctx.moveTo(leftMargin, y); ctx.lineTo(leftMargin + width, y); ctx.stroke()
                    ctx.textAlign = "right"; ctx.fillText(tick.label, leftMargin - 9, y + 4)
                }
                for (const tick of chart.viewport.x_ticks) {
                    const x = leftMargin + tick.x * width
                    ctx.beginPath(); ctx.moveTo(x, topMargin); ctx.lineTo(x, topMargin + height); ctx.stroke()
                    ctx.textAlign = tick.x === 0 ? "left" : tick.x === 1 ? "right" : "center"
                    ctx.fillText(tick.label, x, topMargin + height + 22)
                }
                ctx.save()
                ctx.beginPath(); ctx.rect(leftMargin, topMargin, width, height); ctx.clip()
                ctx.strokeStyle = chart.theme.accent
                ctx.globalAlpha = 0.3
                ctx.beginPath()
                for (const point of chart.viewport.points) {
                    const x = leftMargin + point.x * width
                    ctx.moveTo(x, topMargin + point.y * height)
                    ctx.lineTo(x, topMargin + point.minimum_y * height)
                }
                ctx.stroke()
                ctx.globalAlpha = 1
                ctx.lineWidth = 1.2
                ctx.beginPath()
                for (let index = 0; index < chart.viewport.points.length; ++index) {
                    const point = chart.viewport.points[index]
                    const x = leftMargin + point.x * width
                    const y = topMargin + point.y * height
                    if (index === 0 || point.gap) { ctx.moveTo(x, y) } else { ctx.lineTo(x, y) }
                }
                ctx.stroke()
                if (chart.selectedPoint) {
                    const x = leftMargin + chart.selectedPoint.x * width
                    const y = topMargin + chart.selectedPoint.y * height
                    ctx.strokeStyle = chart.theme.muted; ctx.lineWidth = 1
                    ctx.beginPath(); ctx.moveTo(x, topMargin); ctx.lineTo(x, topMargin + height); ctx.stroke()
                    ctx.fillStyle = chart.theme.accent
                    ctx.beginPath(); ctx.arc(x, y, 3, 0, Math.PI * 2); ctx.fill()
                }
                ctx.restore()
            }
            ChartNavigation {
                anchors.fill: parent
                theme: chart.theme; viewport: chart.viewport; zoomEnabled: chart.zoomEnabled
                leftMargin: plot.leftMargin; rightMargin: plot.rightMargin
                topMargin: plot.topMargin; bottomMargin: plot.bottomMargin
                onRangeRequested: function(start, count) { chart.viewportRequested(start, count) }
                onPointHovered: function(fraction) {
                    chart.selectedIndex = Math.round(fraction * Math.max(0, chart.viewport.points.length - 1))
                }
                Keys.onLeftPressed: chart.selectedIndex = Math.max(0, chart.selectedIndex - 1)
                Keys.onRightPressed: chart.selectedIndex = Math.min(chart.viewport.points.length - 1, chart.selectedIndex + 1)
            }
        }
        Label {
            anchors.centerIn: parent
            width: parent.width - 80
            wrapMode: Text.WordWrap
            horizontalAlignment: Text.AlignHCenter
            visible: chart.viewport.points.length === 0
            text: chart.viewport.error || "Generate or select samples to display the spectrum"
            color: theme.muted
        }
    }
    Label { text: chart.viewport.frequency_label || "Baseband frequency offset (MHz)"; color: theme.muted; Layout.alignment: Qt.AlignHCenter }
    Rectangle {
        objectName: "saAcquisitionDetails"
        visible: chart.showDetails && chart.detailsText.length > 0
        Layout.fillWidth: true
        implicitHeight: metadata.implicitHeight + 16
        color: chart.theme.panel
        border.color: chart.theme.line
        radius: 6
        Label {
            id: metadata
            anchors.fill: parent
            anchors.margins: 8
            text: chart.detailsText
            textFormat: Text.PlainText
            color: chart.theme.accent
            font.pixelSize: 11
            wrapMode: Text.WordWrap
        }
    }
}
