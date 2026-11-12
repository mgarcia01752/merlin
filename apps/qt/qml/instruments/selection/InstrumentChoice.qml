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
    implicitHeight: 112
    Accessible.name: text
    ToolTip.visible: hovered || visualFocus
    ToolTip.text: "Open " + text
    contentItem: Column {
        spacing: 8
        Label {
            width: parent.width
            text: choice.text
            color: choice.theme.text
            font.bold: true
            font.pixelSize: 12
            fontSizeMode: Text.HorizontalFit
            minimumPixelSize: 10
            wrapMode: Text.NoWrap
            horizontalAlignment: Text.AlignHCenter
        }
        Canvas {
            id: icon
            width: parent.width
            height: 67
            onAvailableChanged: if (available) requestPaint()
            onVisibleChanged: if (visible) requestPaint()
            onWidthChanged: requestPaint()
            Connections { target: choice.theme; function onDarkChanged() { icon.requestPaint() } }
            onPaint: {
                const context = getContext("2d")
                context.reset()
                const size = Math.min(width, height) - 12
                const left = 6
                const top = 5
                const plotWidth = width - 12
                const plotHeight = height - 10
                context.strokeStyle = choice.theme.line
                context.lineWidth = 1
                context.strokeRect(left, top, plotWidth, plotHeight)
                context.beginPath()
                if (choice.constellation) {
                    context.moveTo(left, top + plotHeight / 2)
                    context.lineTo(left + plotWidth, top + plotHeight / 2)
                    context.moveTo(left + plotWidth / 2, top)
                    context.lineTo(left + plotWidth / 2, top + plotHeight)
                }
                context.stroke()
                context.strokeStyle = choice.theme.accent
                context.fillStyle = choice.theme.accent
                context.lineWidth = 2
                if (choice.constellation) {
                    for (let row = 0; row < 4; row++) {
                        for (let column = 0; column < 4; column++) {
                            context.beginPath()
                            context.arc(width / 2 + size * (column - 1.5) * 0.2, height / 2 + size * (row - 1.5) * 0.2, 2, 0, Math.PI * 2)
                            context.fill()
                        }
                    }
                } else {
                    // A fixed spectrum navigation glyph, not a measured trace.
                    const trace = [[0,0.92],[0.025,0.73],[0.045,0.2],[0.065,0.18],[0.1,0.2],[0.15,0.18],[0.2,0.2],[0.25,0.18],[0.3,0.2],[0.35,0.18],[0.4,0.2],[0.45,0.18],[0.5,0.2],[0.55,0.18],[0.6,0.2],[0.65,0.18],[0.7,0.2],[0.75,0.18],[0.8,0.2],[0.85,0.18],[0.9,0.2],[0.935,0.18],[0.955,0.2],[0.975,0.73],[1,0.92]]
                    context.beginPath()
                    for (let index = 0; index < trace.length; index++) {
                        const x = left + trace[index][0] * plotWidth
                        const y = top + trace[index][1] * plotHeight
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
    }
}
