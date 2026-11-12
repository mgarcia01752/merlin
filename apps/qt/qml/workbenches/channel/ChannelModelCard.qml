/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import "../../components"
WorkbenchActionButton {
    id:card
    property bool echoes:false
    property string caption:""
    implicitHeight:140
    Accessible.name:text
    Accessible.description:caption
    ToolTip.visible:hovered || visualFocus
    ToolTip.text:"Open "+text+" settings. The picture is a navigation icon."
    contentItem:Column {
        spacing:8
        Label {width:parent.width;text:card.text;color:card.theme.text;font.bold:true;horizontalAlignment:Text.AlignHCenter}
        Canvas {
            id:icon
            width:parent.width;height:58
            onWidthChanged:requestPaint()
            onAvailableChanged:if(available){requestPaint()}
            Connections {target:card.theme;function onDarkChanged(){icon.requestPaint()}}
            onPaint:{
                const context=getContext("2d")
                context.reset()
                const left=14,right=width-14,top=7,bottom=height-7
                context.strokeStyle=card.theme.line;context.lineWidth=1
                context.beginPath();context.moveTo(left,top);context.lineTo(left,bottom);context.lineTo(right,bottom);context.stroke()
                // Fixed navigation glyphs, never simulated or measured traces.
                context.strokeStyle=card.theme.accent;context.lineWidth=2;context.beginPath()
                if(card.echoes){
                    for(const path of [[0.15,0.9],[0.5,0.45],[0.82,0.22]]){
                        const x=left+path[0]*(right-left)
                        context.moveTo(x,bottom);context.lineTo(x,bottom-path[1]*(bottom-top))
                    }
                }else{
                    context.moveTo(left+8,top+5);context.bezierCurveTo(left+(right-left)*0.35,top+10,left+(right-left)*0.65,bottom-10,right-5,bottom-3)
                }
                context.stroke()
            }
        }
        Label {width:parent.width;text:card.caption;color:card.theme.muted;font.pixelSize:11;horizontalAlignment:Text.AlignHCenter;elide:Text.ElideRight}
    }
}
