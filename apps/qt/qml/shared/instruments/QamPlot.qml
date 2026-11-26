/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

// Drawing only: the parent owns and admits the original DTO; Rust supplies normalized
// coordinates and all physical/reference metadata. No point array is copied.
Canvas {
    id: plot
    required property var theme
    required property var observation
    property string xCoordinate: "x"
    property string yCoordinate: "y"
    property bool receivedSamples: false
    property bool roleMarkers: true
    // Optional native series labels affect color only; default DS palette is unchanged.
    property var seriesColors: ({})
    property bool showAxesWithoutData: true
    property real plotInset: 20
    property int axisFontPixels: 12
    property real horizontalLabelInset: 0
    property real verticalLabelOffset: 0
    // Display-only navigation policy; these bounds do not constrain I/Q values.
    readonly property real minimumMagnification: 1
    readonly property real maximumMagnification: 16
    readonly property real zoomFactor: 2
    property real magnification: minimumMagnification
    property real panX: 0
    property real panY: 0
    property bool clipToPlot: false
    readonly property bool drawingClipped: clipToPlot || magnification !== minimumMagnification || panX !== 0 || panY !== 0
    readonly property real receivedPointRadius: 0.85
    readonly property real controlPointRadius: 3.5
    readonly property real fillerPointRadius: 3
    readonly property real squarePointHalfWidth: 2
    readonly property real axisLineWidth: 1
    readonly property real axisLabelOffset: 6
    readonly property real centerFraction: 0.5
    readonly property real plotEdge: Math.max(0, Math.min(width, height) - plotInset * 2)
    readonly property real plotLeft: (width - plotEdge) * centerFraction
    readonly property real plotTop: (height - plotEdge) * centerFraction
    readonly property var points: observation.points || []
    property int paintedPointCount: 0

    /** Return the unchanged downstream role palette; no role is inferred. */
    function roleColor(role) {
        if (role === "data") {
            return theme.qamData;
        }
        if (role === "ncp") {
            return theme.qamNcp;
        }
        if (role === "filler") {
            return theme.qamFiller;
        }
        if (role === "plc") {
            return theme.accent;
        }
        return theme.qamUnknown;
    }

    /** Map only backend-normalized coordinates into the current square pixels. */
    function pointPosition(point) {
        return Qt.point(plotLeft + (point[xCoordinate] * magnification + (1 - magnification) * centerFraction + panX) * plotEdge, plotTop + (point[yCoordinate] * magnification + (1 - magnification) * centerFraction + panY) * plotEdge);
    }

    /** Test only the drawing rectangle, independent of physical signal units. */
    function containsPlotPosition(x, y) {
        return x >= plotLeft && x <= plotLeft + plotEdge && y >= plotTop && y <= plotTop + plotEdge;
    }

    /** Constrain display translation to the enlarged normalized screen rectangle. */
    function boundedPan(value, zoom) {
        const extent = (zoom - minimumMagnification) * centerFraction;
        return Math.max(-extent, Math.min(extent, value));
    }

    /** Zoom already-normalized screen positions around a supplied plot fraction. */
    function zoom(factor, anchorX, anchorY) {
        if (!Number.isFinite(factor) || factor <= 0) {
            return false;
        }
        const horizontal = anchorX === undefined ? centerFraction : anchorX;
        const vertical = anchorY === undefined ? centerFraction : anchorY;
        if (!Number.isFinite(horizontal) || !Number.isFinite(vertical)) {
            return false;
        }
        const next = Math.max(minimumMagnification, Math.min(maximumMagnification, magnification * factor));
        const ratio = next / magnification;
        const x = (Math.max(0, Math.min(1, horizontal)) - centerFraction) * (1 - ratio) + panX * ratio;
        const y = (Math.max(0, Math.min(1, vertical)) - centerFraction) * (1 - ratio) + panY * ratio;
        magnification = next;
        panX = boundedPan(x, next);
        panY = boundedPan(y, next);
        return true;
    }

    /** Move the display by pixels without changing original point coordinates. */
    function pan(horizontalPixels, verticalPixels) {
        if (!Number.isFinite(horizontalPixels) || !Number.isFinite(verticalPixels) || plotEdge <= 0) {
            return false;
        }
        panX = boundedPan(panX + horizontalPixels / plotEdge, magnification);
        panY = boundedPan(panY + verticalPixels / plotEdge, magnification);
        return true;
    }

    /** Restore the exact original default display transform. */
    function resetView() {
        magnification = minimumMagnification;
        panX = 0;
        panY = 0;
    }

    onAvailableChanged: if (available)
        requestPaint()
    onVisibleChanged: if (visible)
        requestPaint()
    onObservationChanged: requestPaint()
    onXCoordinateChanged: requestPaint()
    onYCoordinateChanged: requestPaint()
    onReceivedSamplesChanged: requestPaint()
    onRoleMarkersChanged: requestPaint()
    onSeriesColorsChanged: requestPaint()
    onShowAxesWithoutDataChanged: requestPaint()
    onPlotInsetChanged: requestPaint()
    onAxisFontPixelsChanged: requestPaint()
    onHorizontalLabelInsetChanged: requestPaint()
    onVerticalLabelOffsetChanged: requestPaint()
    onMagnificationChanged: requestPaint()
    onPanXChanged: requestPaint()
    onPanYChanged: requestPaint()
    onClipToPlotChanged: requestPaint()
    onWidthChanged: requestPaint()
    onHeightChanged: requestPaint()
    Connections {
        target: plot.theme
        function onDarkChanged() {
            plot.requestPaint();
        }
    }
    onPaint: {
        const context = getContext("2d");
        context.reset();
        paintedPointCount = 0;
        if (!showAxesWithoutData && !observation.available) {
            return;
        }
        if (drawingClipped) {
            context.save();
            context.beginPath();
            context.rect(plotLeft, plotTop, plotEdge, plotEdge);
            context.clip();
        }
        context.strokeStyle = theme.line;
        context.lineWidth = axisLineWidth;
        const centerX = plotLeft + plotEdge * (centerFraction + panX);
        const centerY = plotTop + plotEdge * (centerFraction + panY);
        context.beginPath();
        context.moveTo(plotLeft, centerY);
        context.lineTo(plotLeft + plotEdge, centerY);
        context.moveTo(centerX, plotTop);
        context.lineTo(centerX, plotTop + plotEdge);
        context.stroke();
        context.fillStyle = theme.muted;
        context.font = axisFontPixels + "px sans-serif";
        context.fillText("I", plotLeft + plotEdge - horizontalLabelInset, centerY - axisLabelOffset);
        context.fillText("Q", centerX + axisLabelOffset, plotTop + verticalLabelOffset);
        // Preserve downstream NCP visibility by painting its markers last.
        for (const controlPass of roleMarkers ? [false, true] : [false]) {
            for (const point of points) {
                if (roleMarkers && (point.role === "ncp") !== controlPass) {
                    continue;
                }
                const position = pointPosition(point);
                context.fillStyle = seriesColors[point.series] || (roleMarkers ? roleColor(point.role) : theme.accent);
                if (receivedSamples) {
                    context.beginPath();
                    context.arc(position.x, position.y, receivedPointRadius, 0, 2 * Math.PI);
                    context.fill();
                } else if (point.role === "ncp") {
                    context.beginPath();
                    context.arc(position.x, position.y, controlPointRadius, 0, 2 * Math.PI);
                    context.fill();
                } else if (point.role === "filler") {
                    context.beginPath();
                    context.moveTo(position.x, position.y - fillerPointRadius);
                    context.lineTo(position.x + fillerPointRadius, position.y);
                    context.lineTo(position.x, position.y + fillerPointRadius);
                    context.lineTo(position.x - fillerPointRadius, position.y);
                    context.closePath();
                    context.fill();
                } else {
                    context.fillRect(position.x - squarePointHalfWidth, position.y - squarePointHalfWidth, squarePointHalfWidth * 2, squarePointHalfWidth * 2);
                }
                paintedPointCount += 1;
            }
        }
        if (drawingClipped) {
            context.restore();
        }
    }
}
