/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import "../shared/components"

// Presentation roles distinguish routes without changing backend semantics.
Item {
    id: route
    required property var theme
    required property string role
    property real fromX: 0
    property real fromY: 0
    property real toX: 0
    property real toY: 0
    property bool arrowHead: true
    property bool showMarker: true
    readonly property real markerSize: 7
    readonly property real markerBorderWidth: 1.5
    readonly property real diamondRotationDegrees: 45
    readonly property color roleColor: role === "data" ? theme.accent : role === "control" ? theme.txPlc : role === "measurement" ? theme.txScatteredPilot : theme.statusComplete
    readonly property bool dashed: role !== "data"
    readonly property string markerShape: role === "control" ? "square" : role === "measurement" ? "circle" : role === "verification" ? "diamond" : "none"

    QtObject {
        id: routePalette
        readonly property color muted: route.roleColor
        readonly property bool dark: route.theme.dark
    }
    Connection {
        anchors.fill: parent
        theme: routePalette
        fromX: route.fromX
        fromY: route.fromY
        toX: route.toX
        toY: route.toY
        comparison: route.dashed
        arrowHead: route.arrowHead
    }
    Rectangle {
        visible: route.showMarker && route.markerShape !== "none"
        x: (route.fromX + route.toX - width) / 2
        y: (route.fromY + route.toY - height) / 2
        width: route.markerSize
        height: width
        radius: route.markerShape === "circle" ? width / 2 : 0
        rotation: route.markerShape === "diamond" ? route.diamondRotationDegrees : 0
        color: route.theme.background
        border.width: route.markerBorderWidth
        border.color: route.roleColor
    }
}
