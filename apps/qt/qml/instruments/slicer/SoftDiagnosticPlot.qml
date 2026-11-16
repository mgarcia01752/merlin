/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
Item {
    id: plot
    required property var theme
    property var observations: ({})
    property var cell: ({})
    property int mode: 1
    property bool nearZero: false
    property real zoom: 1
    property string hoverText: ""
    readonly property var histogram: (nearZero ? observations.near : observations.full) || ({bars:[]})
    onObservationsChanged: canvas.requestPaint()
    onCellChanged: canvas.requestPaint()
    onModeChanged: canvas.requestPaint()
    onNearZeroChanged: canvas.requestPaint()
    onZoomChanged: canvas.requestPaint()
    Connections { target:plot.theme; function onDarkChanged(){canvas.requestPaint()} }
    Canvas {
        id:canvas; anchors.fill:parent
        onPaint: {
            const ctx=getContext("2d");ctx.reset()
            const left=65,top=28,w=width-left-20,h=height-top-38
            if(w<=0 || h<=0) return
            ctx.font="12px sans-serif";ctx.fillStyle=plot.theme.muted
            ctx.strokeStyle=plot.theme.line
            for(let i=0;i<=4;i++){const y=top+h*i/4;ctx.beginPath();ctx.moveTo(left,y);ctx.lineTo(left+w,y);ctx.stroke()}
            if(plot.mode===0){
                const local=plot.cell.neighborhood
                if(!local || !local.center) return
                const radius=local.radius/plot.zoom,scale=Math.min(w,h)/(2*radius)
                const cx=left+w/2,cy=top+h/2
                ctx.save();ctx.beginPath();ctx.rect(left,top,w,h);ctx.clip()
                ctx.strokeStyle=plot.theme.statusAttention;ctx.lineWidth=1.5;ctx.beginPath()
                const poly=local.polygon || []
                poly.forEach(function(p,i){if(i===0)ctx.moveTo(cx+p[0]*scale,cy-p[1]*scale);else ctx.lineTo(cx+p[0]*scale,cy-p[1]*scale)})
                ctx.closePath();ctx.stroke()
                ctx.strokeStyle=plot.theme.muted
                ;(local.references || []).forEach(function(p){const x=cx+p[0]*scale,y=cy-p[1]*scale;ctx.beginPath();ctx.moveTo(x-4,y);ctx.lineTo(x+4,y);ctx.moveTo(x,y-4);ctx.lineTo(x,y+4);ctx.stroke()})
                const iq=plot.cell.normalized_iq
                if(iq){const x=cx+(iq[0]-local.center[0])*scale,y=cy-(iq[1]-local.center[1])*scale;ctx.strokeStyle=plot.theme.accent;ctx.beginPath();ctx.moveTo(cx,cy);ctx.lineTo(x,y);ctx.stroke();ctx.fillStyle=plot.theme.accent;ctx.beginPath();ctx.arc(x,y,4,0,2*Math.PI);ctx.fill()}
                ctx.restore();ctx.fillStyle=plot.theme.muted
                ctx.fillText("Q",left,top-10);ctx.fillText("I · offset from selected ideal symbol",left+w/2-95,top+h+27)
                ctx.fillText("±"+radius.toPrecision(3)+" normalized units",left+8,top+16)
            } else if(plot.mode===1){
                const bars=plot.histogram.bars || [],limit=plot.histogram.limit || 100
                ctx.fillText("Bits per bin",left,top-10)
                for(let i=0;i<=4;i++){ctx.fillText(String(Math.round((plot.histogram.maximum || 0)*(1-i/4))),2,top+h*i/4+4)}
                bars.forEach(function(bar,i){ctx.fillStyle=bar.high<=0 ? "#79b8ff" : plot.theme.accent;ctx.fillRect(left+i*w/bars.length+1,top+h*(1-bar.height),Math.max(1,w/bars.length-2),h*bar.height)})
                ctx.strokeStyle=plot.theme.text;ctx.beginPath();ctx.moveTo(left+w/2,top);ctx.lineTo(left+w/2,top+h);ctx.stroke()
                ctx.fillStyle=plot.theme.muted;ctx.fillText("−"+limit+" · Favors 1",left,top+h+24);ctx.fillText("0 · Uncertain",left+w/2-36,top+h+24);ctx.fillText("Favors 0 · +"+limit,left+w-105,top+h+24)
            } else {
                const limit=plot.observations.sequence_limit || 1,count=plot.observations.count || 1
                ctx.fillText("LLR · positive favors 0; negative favors 1",left,top-10)
                ctx.fillText(limit.toPrecision(3),2,top+4);ctx.fillText("0",40,top+h/2+4);ctx.fillText((-limit).toPrecision(3),2,top+h)
                ctx.strokeStyle=plot.theme.text;ctx.beginPath();ctx.moveTo(left,top+h/2);ctx.lineTo(left+w,top+h/2);ctx.stroke()
                ctx.strokeStyle=plot.theme.accent
                ;(plot.observations.sequence || []).forEach(function(p){const x=left+(p.first+p.last)/2/Math.max(1,count-1)*w;const y=top+h*(1-p.high/limit)/2;ctx.beginPath();ctx.moveTo(x,y);ctx.lineTo(x,top+h*(1-p.low/limit)/2);ctx.stroke();ctx.fillStyle=plot.theme.accent;ctx.fillRect(x-1,y-1,2,2)})
                ctx.fillStyle=plot.theme.muted;ctx.fillText("0",left,top+h+24);ctx.fillText("Decoder bit index · min/max per display group",left+w/2-140,top+h+24);ctx.fillText(String(count-1),left+w-35,top+h+24)
            }
        }
    }
    MouseArea {
        anchors.fill:parent;hoverEnabled:true;acceptedButtons:Qt.NoButton
        onPositionChanged:function(mouse){
            const fraction=(mouse.x-65)/Math.max(1,width-85)
            if(fraction<0 || fraction>=1){plot.hoverText="";return}
            if(plot.mode===1){const bars=plot.histogram.bars || [];const bar=bars[Math.floor(fraction*bars.length)];plot.hoverText=bar ? bar.low.toFixed(1)+" to "+bar.high.toFixed(1)+": "+bar.count+" bits ("+bar.percent.toFixed(2)+"% of retained)" : ""}
            else if(plot.mode===2){const groups=plot.observations.sequence || [];const group=groups[Math.min(groups.length-1,Math.floor(fraction*groups.length))];plot.hoverText=group ? "Bits "+group.first+"–"+group.last+": LLR "+group.low.toPrecision(4)+" to "+group.high.toPrecision(4) : ""}
        }
        onExited:plot.hoverText=""
        ToolTip.visible:containsMouse && plot.hoverText.length>0
        ToolTip.delay:plot.theme.tooltipDelayMs
        ToolTip.text:plot.hoverText
    }
}
