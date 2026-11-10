/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

MouseArea {
    id: navigation
    objectName: "chartNavigation"
    required property var theme
    required property var viewport
    property real leftMargin: 0
    property real rightMargin: 0
    property real topMargin: 0
    property real bottomMargin: 0
    property bool zoomEnabled: true
    property real dragStart: 0
    property real dragEnd: 0
    readonly property real plotWidth: Math.max(1, width - leftMargin - rightMargin)
    readonly property bool canZoom: zoomEnabled && viewport.total > 0 && viewport.count > 0 && (viewport.points || []).length > 0
    signal rangeRequested(int start, int count)
    signal pointHovered(real fraction)
    signal activated()
    hoverEnabled: true
    focus: false
    Accessible.name: "Chart navigation"
    Accessible.description: "Scroll to zoom, drag horizontally to select a range. Plus or minus zooms; Home restores Full Span."
    function fraction(x) { return Math.max(0, Math.min(1, (x - leftMargin) / plotWidth)) }
    function inside(x, y) { return x >= leftMargin && x <= width - rightMargin && y >= topMargin && y <= height - bottomMargin }
    function zoom(factor, anchor) {
        if (!canZoom) {
            return
        }
        const count = Math.max(Math.min(2, viewport.total), Math.min(viewport.total, Math.round(viewport.count * factor)))
        const bin = viewport.start + anchor * (viewport.count - 1)
        const start = Math.max(0, Math.min(viewport.total - count, Math.round(bin - anchor * (count - 1))))
        rangeRequested(start, count)
    }
    onWheel: function(wheel) {
        if (!canZoom || !inside(wheel.x, wheel.y)) {
            wheel.accepted = false
            return
        }
        const delta = wheel.angleDelta.y || wheel.pixelDelta.y
        if (!delta) {
            wheel.accepted = false
            return
        }
        zoom(Math.pow(2, -Math.max(-240, Math.min(240, delta)) / 240), fraction(wheel.x))
        wheel.accepted = true
    }
    onPressed: function(mouse) {
        if (!inside(mouse.x, mouse.y)) {
            mouse.accepted = false
            return
        }
        forceActiveFocus()
        activated()
        dragStart = Math.max(leftMargin, Math.min(width - rightMargin, mouse.x))
        dragEnd = dragStart
    }
    onPositionChanged: function(mouse) {
        pointHovered(fraction(mouse.x))
        if (pressed) {
            dragEnd = Math.max(leftMargin, Math.min(width - rightMargin, mouse.x))
        }
    }
    onReleased: function(mouse) {
        if (!canZoom) {
            return
        }
        dragEnd = Math.max(leftMargin, Math.min(width - rightMargin, mouse.x))
        if (Math.abs(dragEnd - dragStart) < 6) {
            return
        }
        const first = Math.floor(fraction(Math.min(dragStart, dragEnd)) * (viewport.count - 1))
        const last = Math.ceil(fraction(Math.max(dragStart, dragEnd)) * (viewport.count - 1))
        rangeRequested(viewport.start + first, Math.min(viewport.count - first, Math.max(Math.min(2, viewport.total), last - first + 1)))
    }
    Keys.onPressed: function(event) {
        if (event.key === Qt.Key_Home && canZoom) {
            rangeRequested(0, 0)
            event.accepted = true
        } else if (event.key === Qt.Key_Plus || event.key === Qt.Key_Equal) {
            zoom(0.5, 0.5)
            event.accepted = true
        } else if (event.key === Qt.Key_Minus) {
            zoom(2, 0.5)
            event.accepted = true
        }
    }
    Rectangle {
        visible: navigation.pressed && navigation.canZoom && Math.abs(navigation.dragEnd - navigation.dragStart) >= 6
        x: Math.min(navigation.dragStart, navigation.dragEnd); y: navigation.topMargin
        width: Math.abs(navigation.dragEnd - navigation.dragStart)
        height: Math.max(0, navigation.height - navigation.topMargin - navigation.bottomMargin)
        color: navigation.theme.accent; opacity: 0.2
        border.color: navigation.theme.text
    }
}
