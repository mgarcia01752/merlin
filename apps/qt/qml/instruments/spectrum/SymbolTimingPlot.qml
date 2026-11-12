/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls

Canvas {
    id: plot
    required property var theme
    property var graph: ({points:[],ticks:[],bands:[],markers:[]})
    property bool detail: false
    property bool miniature: false
    property string readout: ""
    onGraphChanged: { readout=""; requestPaint() }
    onWidthChanged: requestPaint()
    onHeightChanged: requestPaint()
    onAvailableChanged: if (available) requestPaint()
    onVisibleChanged: if (visible) requestPaint()
    Connections { target:plot.theme; function onDarkChanged() { plot.requestPaint() } }
    Accessible.role: Accessible.Graphic
    Accessible.name: detail ? "Adjacent symbol overlap and usable guard" : "Two complete transmit windows"
    onPaint: {
        const context=getContext("2d");context.reset()
        const left=miniature ? 24 : 46, top=miniature ? 8 : 24
        const w=width-left-16,h=height-top-(miniature ? 24 : 46)
        if (w<=0 || h<=0 || !(graph.points || []).length) return
        for (const band of graph.bands || []) {
            context.fillStyle=band.kind==="guard" ? theme.accent : theme.statusAttention
            context.globalAlpha=.12
            context.fillRect(left+band.x*w,top,(band.end_x-band.x)*w,h)
        }
        context.globalAlpha=1;context.strokeStyle=theme.line;context.lineWidth=1
        for (const weight of [0,.5,1]) {
            context.beginPath();context.moveTo(left,top+(1-weight)*h);context.lineTo(left+w,top+(1-weight)*h);context.stroke()
        }
        for (const marker of graph.markers || []) {
            context.beginPath();context.moveTo(left+marker.x*w,top);context.lineTo(left+marker.x*w,top+h);context.stroke()
        }
        context.strokeRect(left,top,w,h)
        for (const series of [{field:"first",color:theme.accent,dash:[]},{field:"second",color:theme.statusAttention,dash:[6,3]},{field:"sum",color:theme.txScatteredPilot,dash:[2,3]}]) {
            if (series.field==="sum" && !detail) continue
            context.strokeStyle=series.color;context.lineWidth=series.field==="sum" ? 1.5 : 2
            context.setLineDash(series.dash);context.beginPath()
            let started=false
            for (const point of graph.points) {
                if (point[series.field]===null || point[series.field]===undefined) {started=false;continue}
                const x=left+point.x*w,y=top+(1-point[series.field])*h
                if (started) context.lineTo(x,y)
                else {context.moveTo(x,y);started=true}
            }
            context.stroke()
        }
        context.setLineDash([]);context.font="11px sans-serif";context.fillStyle=theme.muted
        context.textAlign="right"
        for (const weight of miniature ? [0,1] : [0,.5,1]) context.fillText(String(weight),left-6,top+(1-weight)*h+4)
        let previousRight=-Infinity
        for (let index=0;index<(graph.ticks || []).length;index++) {
            const tick=graph.ticks[index],labelWidth=context.measureText(tick.label).width
            const x=Math.max(left,Math.min(left+w-labelWidth,left+tick.x*w-labelWidth/2))
            if (x<previousRight+8) continue
            context.textAlign="left";context.fillText(tick.label,x,top+h+17);previousRight=x+labelWidth
        }
        if (!miniature) {
            context.textAlign="center";context.fillText("Time from symbol 1 start (µs)",left+w/2,height-5)
            context.textAlign="left";context.fillText("Window weight",left,12)
            if (detail) {
                for (const marker of graph.markers || []) {
                    const x=left+marker.x*w
                    context.textAlign="left"
                    if (marker.label==="CP end / FFT start") {
                        context.textAlign=marker.x>.65 ? "right" : "left"
                        const labelX=x+(marker.x>.65 ? -5 : 5)
                        context.fillText(marker.label,labelX,top+h*.32)
                        context.fillText(marker.time_label,labelX,top+h*.32+15)
                    }
                }
            }
            for (const label of graph.fft_labels || []) {
                context.textAlign="center";context.fillText(label.text,left+label.x*w,top+h*.45)
            }
            for (const band of graph.bands || []) {
                if ((band.end_x-band.x)*w>28) {
                    const label=band.label || ""
                    if (context.measureText(label).width+8<(band.end_x-band.x)*w) {
                        context.textAlign="center";context.fillText(label,left+(band.x+band.end_x)*w/2,top+h*.8)
                    }
                }
            }
        }
    }
    MouseArea {
        anchors.fill:parent;hoverEnabled:true;acceptedButtons:Qt.NoButton
        enabled:!plot.miniature
        onExited:plot.readout=""
        onPositionChanged:function(mouse) {
            const x=(mouse.x-46)/Math.max(1,width-62)
            let closest=null,distance=Infinity
            for (const point of plot.graph.points || []) {
                const delta=Math.abs(point.x-x)
                if(delta<distance) {distance=delta;closest=point}
            }
            plot.readout=closest ? closest.readout : ""
        }
        ToolTip.visible:containsMouse && plot.readout.length>0
        ToolTip.text:plot.readout
    }
}
