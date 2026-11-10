/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Rectangle {
    id: panel
    required property var theme
    required property var backend
    required property bool hasResult
    required property string resultToken
    property var timing: ({points: [], ticks: []})
    property real velocityFactor: cable.currentIndex === 0 ? 0.82 : cable.currentIndex === 1 ? 0.87 : (customVelocity.text.length ? Number(customVelocity.text) : NaN)
    function refresh() {
        timing = hasResult ? JSON.parse(backend.timing(velocityFactor)) : ({points: [], ticks: []})
        windowPlot.requestPaint()
    }
    onHasResultChanged: refresh()
    onResultTokenChanged: refresh()
    onVelocityFactorChanged: refresh()
    Component.onCompleted: refresh()
    color: theme.panel; border.color: theme.line; radius: 6
    implicitHeight: cable.currentIndex === 2 ? 240 : 220
    Accessible.name: "Symbol timing and ideal cable echo budget"
    RowLayout {
        anchors.fill: parent; anchors.margins: 10; spacing: 18
        ColumnLayout {
            Layout.fillWidth: true; Layout.fillHeight: true; spacing: 5
            Label { text: "Symbol start · transmit window coefficient"; color: theme.text; font.bold: true; font.pixelSize: 13 }
            Label { text: panel.timing.overview || "Run simulation to display frozen symbol timing."; color: theme.muted; font.pixelSize: 11; Layout.fillWidth: true; elide: Text.ElideRight }
            Canvas {
                id: windowPlot
                objectName: "saTimingPlot"
                Layout.fillWidth: true; Layout.fillHeight: true
                onWidthChanged: requestPaint()
                onHeightChanged: requestPaint()
                Connections { target: panel.theme; function onDarkChanged() { windowPlot.requestPaint() } }
                Accessible.role: Accessible.Graphic
                Accessible.name: "Transmit window at symbol start, not signal amplitude"
                Accessible.description: panel.timing.guard_label || panel.timing.error || "No completed simulation"
                onPaint: {
                    const ctx = getContext("2d")
                    ctx.reset()
                    if (!panel.timing.points.length) { return }
                    const left = 20, top = 8, width = windowPlot.width - 40, height = windowPlot.height - 28
                    if (width <= 0 || height <= 0) { return }
                    const roll = panel.timing.roll_off_fraction * width
                    const prefix = panel.timing.prefix_fraction * width
                    ctx.fillStyle = panel.theme.accentSoft
                    ctx.fillRect(left, top, prefix, height)
                    ctx.fillStyle = panel.theme.button
                    ctx.fillRect(left, top, roll, height)
                    ctx.strokeStyle = panel.theme.line
                    ctx.beginPath(); ctx.moveTo(left, top); ctx.lineTo(left, top + height); ctx.lineTo(left + width, top + height); ctx.stroke()
                    ctx.strokeStyle = panel.theme.statusAttention
                    ctx.beginPath(); ctx.moveTo(left + roll, top); ctx.lineTo(left + roll, top + height); ctx.stroke()
                    ctx.strokeStyle = panel.theme.muted
                    ctx.beginPath(); ctx.moveTo(left + prefix, top); ctx.lineTo(left + prefix, top + height); ctx.stroke()
                    ctx.strokeStyle = panel.theme.accent; ctx.lineWidth = 2
                    ctx.beginPath()
                    for (let index = 0; index < panel.timing.points.length; ++index) {
                        const point = panel.timing.points[index]
                        const x = left + point.x * width, y = top + (1 - point.coefficient) * height
                        if (index === 0) { ctx.moveTo(x, y) } else { ctx.lineTo(x, y) }
                    }
                    ctx.stroke()
                    ctx.fillStyle = panel.theme.muted; ctx.font = "11px sans-serif"
                    ctx.textAlign = "right"; ctx.fillText("1", left - 5, top + 9); ctx.fillText("0", left - 5, top + height)
                    const endLabel = panel.timing.ticks.length ? panel.timing.ticks[panel.timing.ticks.length - 1].label : ""
                    const endLabelLeft = left + width - ctx.measureText(endLabel).width
                    for (const tick of panel.timing.ticks) {
                        ctx.textAlign = tick.x === 0 ? "left" : tick.x === 1 ? "right" : "center"
                        let labelX = left + tick.x * width
                        if (tick.x > 0 && tick.x < 1 && labelX + ctx.measureText(tick.label).width / 2 > endLabelLeft - 6) {
                            ctx.textAlign = "right"
                            labelX = Math.min(labelX, endLabelLeft - 6)
                        }
                        ctx.fillText(tick.label, labelX, top + height + 17)
                    }
                }
            }
            RowLayout {
                Layout.fillWidth: true
                Label { text: panel.timing.roll_label || "δ —"; color: theme.statusAttention; font.pixelSize: 11 }
                Item { Layout.fillWidth: true }
                Label { text: panel.timing.guard_label || "Flat guard —"; color: theme.accent; font.pixelSize: 11 }
                Item { Layout.fillWidth: true }
                Label { text: "CP end → FFT start"; color: theme.muted; font.pixelSize: 11 }
            }
        }
        Rectangle { Layout.fillHeight: true; width: 1; color: theme.line }
        ColumnLayout {
            Layout.preferredWidth: 410; Layout.fillHeight: true; spacing: 4
            Label { text: "Cable Echo Estimate"; color: theme.text; font.bold: true; font.pixelSize: 13 }
            Label { text: "Echo distance from the flat guard time and velocity factor (VF). VF is signal speed relative to light; a reflected echo travels out and back."; color: theme.muted; font.pixelSize: 11; Layout.fillWidth: true; wrapMode: Text.WordWrap }
            ComboBox {
                id: cable; objectName: "saCableModel"; Layout.fillWidth: true
                model: ["Belden 1694WB RG-6 · VF 0.82", "CommScope P3 500 · VF 0.87", "Custom cable / velocity factor"]
                Accessible.name: "Cable model for timing estimate"
            }
            RowLayout {
                visible: cable.currentIndex === 2
                Label { text: "Velocity factor"; color: theme.muted; font.pixelSize: 12 }
                TextField {
                    id: customVelocity; objectName: "saCustomVelocity"
                    text: ""; placeholderText: "0 < VF ≤ 1"; Layout.fillWidth: true
                    validator: DoubleValidator { bottom: 0; top: 1; decimals: 6; locale: "C" }
                    Accessible.name: "Custom cable velocity factor"
                }
            }
            GridLayout {
                objectName: "saCableGrid"
                columns: 2; columnSpacing: 18; rowSpacing: 3; Layout.fillWidth: true
                Label { text: "Flat guard time"; color: theme.muted; font.pixelSize: 12 }
                Label { text: panel.timing.guard_label ? panel.timing.guard_label.substring("Flat guard ".length) : "—"; color: theme.accent; font.pixelSize: 12 }
                Label { text: "Extra cable path"; color: theme.muted; font.pixelSize: 12 }
                Label { objectName: "saExtraPath"; text: panel.timing.extra_path || "—"; color: theme.text; font.pixelSize: 12 }
                Label { text: "Distance to reflector"; color: theme.muted; font.pixelSize: 12 }
                Label { objectName: "saEchoSpacing"; text: panel.timing.reflection_spacing || "—"; color: theme.statusAttention; font.pixelSize: 12 }
            }
            Label { visible: !!panel.timing.error; text: panel.timing.error || ""; color: theme.statusAttention; font.pixelSize: 11; Layout.fillWidth: true; wrapMode: Text.WordWrap }
            Label {
                Layout.fillWidth: true
                text: "Assumes uniform cable and zero timing margin. Echo estimate, not a cable-length limit or guaranteed protection."
                color: theme.muted; font.pixelSize: 11; wrapMode: Text.WordWrap
            }
            Item { Layout.fillHeight: true }
        }
    }
}
