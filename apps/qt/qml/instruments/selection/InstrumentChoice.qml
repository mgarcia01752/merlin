/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import "../../components"

WorkbenchActionButton {
    id: choice
    property bool constellation: false
    implicitHeight: 132
    Accessible.name: text
    ToolTip.visible: hovered || visualFocus
    ToolTip.text: "Open " + text
    background: Rectangle {
        radius: 9
        color: choice.down ? choice.theme.line : choice.hovered ? choice.theme.accentSoft : "transparent"
        border.width: choice.visualFocus ? 2 : 1
        border.color: choice.hovered || choice.visualFocus ? choice.theme.accent : "transparent"
    }
    contentItem: Column {
        spacing: 8
        Canvas {
            id: icon
            width: parent.width
            height: 84
            onAvailableChanged: if (available) requestPaint()
            onVisibleChanged: if (visible) requestPaint()
            onWidthChanged: requestPaint()
            Connections { target: choice.theme; function onDarkChanged() { icon.requestPaint() } }
            onPaint: {
                const context = getContext("2d")
                context.reset()
                const size = Math.min(width, height) - 12
                const left = (width - size) / 2
                const top = (height - size) / 2
                context.strokeStyle = choice.theme.muted
                context.lineWidth = 1
                context.beginPath()
                if (choice.constellation) {
                    context.moveTo(left, top + size / 2)
                    context.lineTo(left + size, top + size / 2)
                    context.moveTo(left + size / 2, top)
                    context.lineTo(left + size / 2, top + size)
                } else {
                    context.moveTo(left, top)
                    context.lineTo(left, top + size)
                    context.lineTo(left + size, top + size)
                }
                context.stroke()
                context.strokeStyle = choice.theme.accent
                context.fillStyle = choice.theme.accent
                context.lineWidth = 2
                if (choice.constellation) {
                    for (let row = 0; row < 4; row++) {
                        for (let column = 0; column < 4; column++) {
                            context.beginPath()
                            context.arc(left + size * (0.2 + column * 0.2), top + size * (0.2 + row * 0.2), 2.5, 0, Math.PI * 2)
                            context.fill()
                        }
                    }
                } else {
                    // A fixed spectrum navigation glyph, not a measured trace.
                    const trace = [[0,0.86],[0.12,0.82],[0.2,0.85],[0.27,0.75],[0.34,0.78],[0.41,0.24],[0.47,0.63],[0.54,0.12],[0.61,0.65],[0.68,0.42],[0.74,0.77],[0.84,0.82],[1,0.85]]
                    context.beginPath()
                    for (let index = 0; index < trace.length; index++) {
                        const x = left + trace[index][0] * size
                        const y = top + trace[index][1] * size
                        if (index === 0) {
                            context.moveTo(x, y)
                        } else {
                            context.lineTo(x, y)
                        }
                    }
                    context.stroke()
                }
            }
        }
        Label {
            width: parent.width
            text: choice.text
            color: choice.theme.text
            font: choice.font
            fontSizeMode: Text.HorizontalFit
            minimumPixelSize: 10
            wrapMode: Text.NoWrap
            horizontalAlignment: Text.AlignHCenter
        }
    }
}
