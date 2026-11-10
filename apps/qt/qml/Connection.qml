/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

Canvas {
    id: line
    required property var theme
    property real fromX
    property real fromY
    property real toX
    property real toY
    property bool comparison: false
    property bool arrowHead: true
    onArrowHeadChanged: requestPaint()
    onWidthChanged: requestPaint()
    onHeightChanged: requestPaint()
    onFromXChanged: requestPaint()
    onFromYChanged: requestPaint()
    onToXChanged: requestPaint()
    onToYChanged: requestPaint()
    Connections { target: line.theme; function onDarkChanged() { line.requestPaint() } }
    onPaint: {
        const ctx = getContext("2d")
        ctx.reset()
        ctx.strokeStyle = theme.muted
        ctx.fillStyle = theme.muted
        ctx.lineWidth = 1.6
        if (comparison) {
            ctx.setLineDash([6, 5])
        }
        ctx.beginPath(); ctx.moveTo(fromX, fromY); ctx.lineTo(toX, toY); ctx.stroke()
        ctx.setLineDash([])
        if (!arrowHead) {
            return
        }
        const angle = Math.atan2(toY - fromY, toX - fromX)
        ctx.beginPath(); ctx.moveTo(toX, toY)
        ctx.lineTo(toX - 10 * Math.cos(angle - 0.4), toY - 10 * Math.sin(angle - 0.4))
        ctx.lineTo(toX - 10 * Math.cos(angle + 0.4), toY - 10 * Math.sin(angle + 0.4))
        ctx.closePath(); ctx.fill()
    }
}
