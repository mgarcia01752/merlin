/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

Canvas {
    id:plot
    required property var theme
    required property var observation
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
        ctx.fillStyle=theme.accent
        for(const point of observation.points || []){ctx.fillRect(left+point.x*edge-2,top+point.y*edge-2,4,4)}
    }
}
