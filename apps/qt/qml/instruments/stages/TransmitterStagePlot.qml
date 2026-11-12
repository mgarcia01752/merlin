/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import "../qam"

Item {
    id: plot
    required property var theme
    property var observation: ({})
    property var graph: observation.graph || ({kind:"empty"})
    property bool miniature: false
    property string readout: ""
    function roleColor(role) {
        if (role === "data_location") return theme.qamData
        if (role === "plc") return theme.txPlc
        if (role === "continuous_pilot") return theme.txContinuousPilot
        if (role === "scattered_pilot") return theme.txScatteredPilot
        return theme.qamUnknown
    }
    onGraphChanged: {readout="";canvas.requestPaint()}
    onWidthChanged: canvas.requestPaint()
    onHeightChanged: canvas.requestPaint()
    Connections { target:plot.theme;function onDarkChanged(){canvas.requestPaint()} }
    TransmitterScatter { anchors.fill:parent;visible:plot.graph.kind === "constellation";theme:plot.theme;observation:plot.observation }
    Canvas {
        id:canvas;anchors.fill:parent;visible:plot.graph.kind !== "constellation"
        onAvailableChanged:if (available) requestPaint()
        onVisibleChanged:if (visible) requestPaint()
        Accessible.role:Accessible.Graphic
        Accessible.name:(plot.graph.y_label || "Transmitter graph")+" against "+(plot.graph.x_label || "index")
        onPaint: {
            const ctx=getContext("2d");ctx.reset()
            const left=plot.miniature ? 6 : 52,top=plot.miniature ? 5 : 22
            const w=width-left-(plot.miniature ? 6 : 18),h=height-top-(plot.miniature ? 6 : 38)
            if (w<=0 || h<=0) return
            ctx.strokeStyle=plot.theme.line;ctx.lineWidth=1
            ctx.strokeRect(left,top,w,h)
            const graph=plot.graph,points=graph.points || []
            if (!plot.miniature) {
                ctx.font="11px sans-serif";ctx.fillStyle=plot.theme.muted
                ctx.textAlign="left";ctx.fillText(graph.first || "",left,top+h+17)
                ctx.textAlign="right";ctx.fillText(graph.last || "",left+w,top+h+17)
                ctx.textAlign="center";ctx.fillText(graph.x_label || "",left+w/2,top+h+32)
                ctx.textAlign="left";ctx.fillText(graph.y_label || "",left,12)
                if (graph.kind === "bits") {ctx.fillText("1",left-16,top+h*0.2+4);ctx.fillText("0",left-16,top+h*0.8+4)}
                if (graph.kind === "physical") {ctx.fillText(Number(graph.maximum).toFixed(2),2,top+8);ctx.fillText("0",left-16,top+h)}
                if (graph.kind === "sequence") {ctx.fillText("+"+Number(graph.extent).toFixed(2),2,top+8);ctx.fillText("0",left-16,top+h/2);ctx.fillText("−"+Number(graph.extent).toFixed(2),2,top+h)}
            }
            if (graph.kind === "raster" || graph.kind === "labels") {
                for (const point of points) {
                    if (point.value === null) continue
                    ctx.fillStyle=point.value===0 ? plot.theme.line : plot.theme.accent
                    ctx.globalAlpha=graph.kind === "labels" ? 0.2+0.8*(point.intensity || 0) : 1
                    const rw=graph.kind === "raster" ? w/graph.columns : w/Math.max(1,graph.count)
                    const rh=graph.kind === "raster" ? h/graph.rows : h*0.6
                    ctx.fillRect(left+point.x*w,top+point.y*h,Math.max(1,rw-0.5),Math.max(1,rh-0.5))
                }
                ctx.globalAlpha=1;return
            }
            if (graph.kind === "sequence") {
                ctx.beginPath();ctx.moveTo(left,top+h/2);ctx.lineTo(left+w,top+h/2);ctx.stroke()
            }
            for (const field of graph.kind === "sequence" ? ["y","q_y"] : ["y"]) {
                ctx.strokeStyle=field === "q_y" ? plot.theme.statusAttention : plot.theme.accent;ctx.lineWidth=1.3
                let previous=null
                for (const point of points) {
                    if (point[field] === null || point[field] === undefined) {previous=null;continue}
                    const x=left+point.x*w,y=top+point[field]*h
                    if (previous && !point.gap && !previous.gap) {
                        ctx.beginPath();ctx.moveTo(left+previous.x*w,top+previous[field]*h)
                        if (graph.kind === "bits") ctx.lineTo(x,top+previous[field]*h)
                        ctx.lineTo(x,y);ctx.stroke()
                    }
                    ctx.fillStyle=ctx.strokeStyle;ctx.beginPath();ctx.arc(x,y,plot.miniature ? 1 : 1.7,0,2*Math.PI);ctx.fill()
                    previous=point
                }
            }
            for (const band of graph.bands || []) {
                ctx.fillStyle=plot.roleColor(band.role)
                ctx.fillRect(left+band.x*w,top+h-5,Math.max(1,(band.end_x-band.x)*w),5)
            }
        }
    }
    MouseArea {
        anchors.fill:parent;hoverEnabled:true;acceptedButtons:Qt.NoButton;enabled:!plot.miniature
        onExited:plot.readout=""
        onPositionChanged:function(mouse) {
            const points=plot.graph.points || []
            const x=(mouse.x-52)/Math.max(1,width-70),y=(mouse.y-22)/Math.max(1,height-60)
            let nearest=null,distance=Infinity
            for (const point of points) {
                const delta=plot.graph.kind === "raster" ? Math.abs(point.x-x)+Math.abs(point.y-y) : Math.abs(point.x-x)
                if (delta<distance) {distance=delta;nearest=point}
            }
            plot.readout=nearest ? nearest.readout || "" : ""
        }
        ToolTip.visible:containsMouse && plot.readout.length>0
        ToolTip.text:plot.readout
    }
    Label { anchors.centerIn:parent;visible:!plot.observation.available || plot.graph.kind === "empty";text:plot.graph.reason || "No capture";color:plot.theme.muted;font.pixelSize:plot.miniature ? 10 : 13 }
}
