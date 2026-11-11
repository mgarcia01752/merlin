/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

Canvas {
    id:plot
    required property var theme
    required property var observation
    property bool receivedSamples: false
    onReceivedSamplesChanged: requestPaint()
    function roleColor(role) {
        if (role === "data") {
            return theme.qamData
        }
        if (role === "ncp") {
            return theme.qamNcp
        }
        if (role === "filler") {
            return theme.qamFiller
        }
        if (role === "plc") {
            return theme.accent
        }
        return theme.qamUnknown
    }
    onObservationChanged:requestPaint()
    onWidthChanged:requestPaint()
    onHeightChanged:requestPaint()
    Connections { target:plot.theme;function onDarkChanged(){plot.requestPaint()} }
    onPaint:{
        const ctx=getContext("2d");ctx.reset()
        const edge=Math.max(0,Math.min(width,height)-40),left=(width-edge)/2,top=(height-edge)/2
        ctx.strokeStyle=theme.line;ctx.lineWidth=1
        ctx.beginPath();ctx.moveTo(left,top+edge/2);ctx.lineTo(left+edge,top+edge/2);ctx.moveTo(left+edge/2,top);ctx.lineTo(left+edge/2,top+edge);ctx.stroke()
        ctx.fillStyle=theme.muted;ctx.font="12px sans-serif";ctx.fillText("I",left+edge,top+edge/2-6);ctx.fillText("Q",left+edge/2+6,top)
        // Paint control cells last so overlapping data does not hide them.
        const points = observation.points || []
        for (const ncpPass of [false, true]) {
            for (const point of points) {
                if ((point.role === "ncp") !== ncpPass) {
                    continue
                }
                const x = left + point.x * edge
                const y = top + point.y * edge
                ctx.fillStyle = roleColor(point.role)
                if (plot.receivedSamples) {
                    ctx.beginPath(); ctx.arc(x, y, 2, 0, 2 * Math.PI); ctx.fill()
                } else if (point.role === "ncp") {
                    ctx.beginPath(); ctx.arc(x, y, 3.5, 0, 2 * Math.PI); ctx.fill()
                } else if (point.role === "filler") {
                    ctx.beginPath(); ctx.moveTo(x, y - 3); ctx.lineTo(x + 3, y)
                    ctx.lineTo(x, y + 3); ctx.lineTo(x - 3, y); ctx.closePath(); ctx.fill()
                } else {
                    ctx.fillRect(x - 2, y - 2, 4, 4)
                }
            }
        }
    }
}
