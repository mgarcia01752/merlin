/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
Button {
    id:control
    required property var theme
    property bool amplifier:true
    property string valueText:"0 dB"
    padding:0;hoverEnabled:true
    Accessible.name:amplifier ? "Configure Launch Amplifier" : "Configure Receiver Input Attenuator"
    HoverHandler {cursorShape:Qt.PointingHandCursor}
    background:Rectangle {color:control.hovered ? control.theme.accentSoft : "transparent";radius:8;border.width:control.visualFocus ? 1 : 0;border.color:control.theme.accent}
    contentItem:Canvas {
        id:drawing
        onWidthChanged:requestPaint();onHeightChanged:requestPaint()
        Connections {target:control;function onAmplifierChanged(){drawing.requestPaint()}}
        Connections {target:control.theme;function onDarkChanged(){drawing.requestPaint()}}
        onPaint: {
            const c=getContext("2d");c.reset();c.strokeStyle=control.theme.accent;c.fillStyle=control.theme.accentSoft;c.lineWidth=2
            if(control.amplifier){c.beginPath();c.moveTo(2,2);c.lineTo(width-2,height/2);c.lineTo(2,height-2);c.closePath();c.fill();c.stroke()}
            else {
                c.beginPath();c.moveTo(0,height/2);c.lineTo(8,height/2)
                for(let i=0;i<6;i++){c.lineTo(12+i*(width-24)/5,height/2+(i%2 ? 13 : -13))}
                c.lineTo(width-8,height/2);c.lineTo(width,height/2);c.stroke()
                c.beginPath();c.moveTo(10,height-8);c.lineTo(width-10,8);c.lineTo(width-10,18);c.moveTo(width-10,8);c.lineTo(width-20,8);c.stroke()
            }
        }
    }
    Label {anchors.horizontalCenter:parent.horizontalCenter;anchors.top:parent.bottom;anchors.topMargin:8;text:control.valueText;color:control.theme.accent;font.pixelSize:12}
    Label {anchors.horizontalCenter:parent.horizontalCenter;anchors.top:parent.bottom;anchors.topMargin:27;text:control.amplifier ? "Launch Amplifier" : "RX Attenuator";color:control.theme.muted;font.pixelSize:11}
}
