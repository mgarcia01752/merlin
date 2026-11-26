/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../shared/components"
import "../shared/presentation"
import "../shared/instruments"

// Read-only capture inspection. Native Rust supplies source evidence, exact
// display strings and bounded pages; decoding never establishes association here.
WorkbenchDialog {
    id: dialog
    property string family: ""
    property var presentation: ({})
    property bool busy: false
    signal loadRequested(string family)
    signal pageRequested(string family, int first)
    signal plotRequested(string family, int first, int count)
    signal measuredPlanRequested
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: coverage
        theme: dialog.theme
    }
    property bool presentationReady: false
    // Each native range field is a nonnegative i32; ten decimal characters cover
    // its complete transport domain. This is not a scientific reading-count limit.
    readonly property int plotCoordinateCharacters: 10
    property Binding plotFirstLength: Binding {
        target: plotFirst.contentItem
        property: "maximumLength"
        value: dialog.plotCoordinateCharacters
    }
    property Binding plotCountLength: Binding {
        target: plotCount.contentItem
        property: "maximumLength"
        value: dialog.plotCoordinateCharacters
    }
    property Binding plotFirstText: Binding {
        target: plotFirst.contentItem
        property: "text"
        value: coverage.textAdmitted ? plotFirst.displayText : ""
    }
    property Binding plotCountText: Binding {
        target: plotCount.contentItem
        property: "text"
        value: coverage.textAdmitted ? plotCount.displayText : ""
    }
    readonly property var displayedPresentation: coverage.value ? coverage.value.presentation : ({})
    readonly property string displayedFamily: coverage.value ? coverage.value.family : ""
    readonly property var metadataRows: displayedPresentation.metadata_rows || []
    readonly property var columns: displayedPresentation.columns || []
    readonly property var dataRows: displayedPresentation.data_rows || []
    readonly property var page: displayedPresentation.page || ({})
    readonly property var plot: displayedPresentation.plot || ({})
    readonly property bool hasPlot: displayedFamily === "rxmer" && !!displayedPresentation.plot
    readonly property bool plotAvailable: actionsAvailable && hasPlot && Number.isInteger(plot.total) && plot.total > 0 && Number.isInteger(plot.maximum_count) && plot.maximum_count > 0
    readonly property bool presentationRejected: coverage.rejected
    readonly property bool contextCurrent: !!coverage.value && displayedFamily === family && coverage.value.presentation === presentation
    readonly property bool actionsAvailable: contextCurrent && !busy && !coverage.rejected && validFamily(displayedFamily)
    readonly property bool previousAvailable: actionsAvailable && page.has_previous === true && Number.isInteger(page.previous_first) && page.previous_first >= 0
    readonly property bool nextAvailable: actionsAvailable && page.has_next === true && Number.isInteger(page.next_first) && page.next_first >= 0

    // Independent display geometry, never PHY or capture admission limits.
    readonly property real preferredDialogWidth: 900
    readonly property real preferredDialogHeight: 680
    readonly property real windowMargin: 24
    readonly property real sectionSpacing: 16
    readonly property real rowSpacing: 8
    readonly property real panelInset: 12
    readonly property real panelRadius: 8
    readonly property real headingInset: 6
    readonly property real closeButtonWidth: 36
    readonly property real footerSpacing: 10
    readonly property real bodyFontPixels: 13
    readonly property real captionFontPixels: 12
    readonly property real plotHeight: 280
    title: displayedPresentation.title || "Upstream Capture"
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(preferredDialogWidth, parent ? parent.width - windowMargin * 2 : preferredDialogWidth)
    height: Math.min(preferredDialogHeight, parent ? parent.height - windowMargin * 2 : preferredDialogHeight)
    modal: true
    focus: true
    closePolicy: Popup.CloseOnEscape

    /**
     * Recognize only the existing native capture-family tags.
     * @param value Original host family, without source interpretation.
     * @returns Whether acquisition/page requests have an explicit destination.
     */
    function validFamily(value) {
        return value === "ucd" || value === "rxmer" || value === "pre_eq";
    }

    /**
     * Admit the original presentation and its family before persistent replacement.
     * Rows and labels bind to this witness rather than retaining derived copies.
     * @returns False leaves the entire previously admitted inspection intact.
     */
    function adoptPresentation() {
        // Qt may notify a binding again for the same immutable source alias.
        // Keep its existing witness and pointer readout; actual source, family,
        // page and viewport changes still require complete candidate admission.
        if (presentationReady && coverage.value && !coverage.rejected
                && coverage.value.family === family
                && coverage.value.presentation === presentation) {
            return true;
        }
        return presentationReady && coverage.replace({
            family: family,
            presentation: presentation
        });
    }

    /** Request the native picker without modifying source or accepted configuration. */
    function requestLoad() {
        if (actionsAvailable) {
            loadRequested(displayedFamily);
        }
    }

    /** Request complete measured-plan acquisition; this inspector never composes one. */
    function requestMeasuredPlan() {
        if (actionsAvailable) {
            measuredPlanRequested();
        }
    }

    /**
     * Forward one authoritative page origin without reconstructing an extent.
     * @param previous True selects the supplied previous coordinate; false the next.
     */
    function requestPage(previous) {
        if (previous ? previousAvailable : nextAvailable) {
            pageRequested(displayedFamily, previous ? page.previous_first : page.next_first);
        }
    }

    /**
     * Request original records only; zero count restores the native bounded span.
     * @param first Original zero-based reading index, never a reconstructed carrier.
     * @param count Finite reading count; the native source remains authoritative.
     * @returns Whether the current accepted source can dispatch this viewport.
     */
    function requestPlot(first, count) {
        if (!plotAvailable || !Number.isInteger(first) || first < 0 || first >= plot.total || !Number.isInteger(count) || count < 0 || count > plot.maximum_count || count > plot.total - first) {
            return false;
        }
        plotRequested(displayedFamily, first, count);
        return true;
    }

    onFamilyChanged: adoptPresentation()
    onPresentationChanged: adoptPresentation()
    onOpened: {
        coverage.reserveTextFields([plotFirst.contentItem, plotCount.contentItem]);
        adoptPresentation();
    }
    Component.onCompleted: {
        presentationReady = true;
        coverage.reserveTextFields([plotFirst.contentItem, plotCount.contentItem]);
        adoptPresentation();
    }
    Component.onDestruction: {
        presentationReady = false;
        if (coverage) {
            coverage.clear();
        }
    }
    header: RowLayout {
        spacing: dialog.rowSpacing
        Label {
            objectName: "upstreamCaptureTitle"
            Layout.fillWidth: true
            Layout.margins: dialog.headingInset
            text: dialog.title
            textFormat: Text.PlainText
            color: dialog.theme.text
            font.bold: true
            elide: Text.ElideRight
        }
        WorkbenchActionButton {
            objectName: "popupCloseButton"
            theme: dialog.theme
            compact: true
            Layout.preferredWidth: dialog.closeButtonWidth
            text: "×"
            Accessible.name: "Close capture inspection"
            ToolTip.visible: hovered
            ToolTip.text: "Close"
            onClicked: dialog.reject()
        }
    }
    background: Rectangle {
        color: dialog.theme.panel
        border.color: dialog.theme.line
        radius: dialog.panelRadius
    }
    contentItem: ScrollView {
        id: scroller
        clip: true
        contentWidth: availableWidth
        ColumnLayout {
            width: scroller.availableWidth
            spacing: dialog.sectionSpacing
            Label {
                objectName: "upstreamCaptureSummary"
                Layout.fillWidth: true
                text: dialog.displayedPresentation.summary || "No capture inspection is available."
                textFormat: Text.PlainText
                color: dialog.theme.text
                font.pixelSize: dialog.bodyFontPixels
                wrapMode: Text.WrapAnywhere
            }
            Label {
                objectName: "upstreamCaptureReferencePlane"
                Layout.fillWidth: true
                text: dialog.displayedPresentation.reference_plane || "Inspection only. Association and simulation readiness are unavailable."
                textFormat: Text.PlainText
                color: dialog.theme.accent
                font.pixelSize: dialog.captionFontPixels
                wrapMode: Text.WrapAnywhere
            }
            MeasurementPlot {
                id: capturePlot
                objectName: "upstreamCapturePlot"
                visible: dialog.hasPlot
                enabled: dialog.plotAvailable && coverage.textAdmitted
                Layout.fillWidth: true
                Layout.preferredHeight: dialog.plotHeight
                theme: dialog.theme
                viewport: dialog.plot
                onViewportRequested: function (start, count) {
                    dialog.requestPlot(start, count);
                }
            }
            RowLayout {
                visible: dialog.hasPlot
                enabled: dialog.plotAvailable
                Layout.fillWidth: true
                spacing: dialog.rowSpacing
                Label {
                    text: "First original record"
                    color: dialog.theme.muted
                }
                SpinBox {
                    id: plotFirst
                    objectName: "upstreamCapturePlotFirst"
                    from: 0
                    to: Math.max(0, (dialog.plot.total || 0) - 1)
                    value: dialog.plot.start || 0
                    onValueModified: {
                        const count = Math.min(dialog.plot.count, dialog.plot.total - value);
                        if (!dialog.requestPlot(value, count)) {
                            value = Qt.binding(function() { return dialog.plot.start || 0; });
                        }
                    }
                }
                Label {
                    text: "Original readings"
                    color: dialog.theme.muted
                }
                SpinBox {
                    id: plotCount
                    objectName: "upstreamCapturePlotCount"
                    from: dialog.plot.total > 0 ? 1 : 0
                    to: Math.min(dialog.plot.maximum_count || 0, (dialog.plot.total || 0) - (dialog.plot.start || 0))
                    value: dialog.plot.count || 0
                    onValueModified: {
                        if (!dialog.requestPlot(dialog.plot.start, value)) {
                            value = Qt.binding(function() { return dialog.plot.count || 0; });
                        }
                    }
                }
                Item {
                    Layout.fillWidth: true
                }
                Button {
                    objectName: "upstreamCapturePlotFull"
                    text: "Full Source"
                    onClicked: dialog.requestPlot(0, 0)
                }
            }
            Repeater {
                id: metadataRepeater
                objectName: "upstreamCaptureMetadata"
                model: dialog.metadataRows
                RowLayout {
                    id: metadataRow
                    required property int index
                    required property var modelData
                    Layout.fillWidth: true
                    spacing: dialog.rowSpacing
                    Label {
                        objectName: "upstreamCaptureMetadataLabel_" + metadataRow.index
                        Layout.fillWidth: true
                        Layout.preferredWidth: 0
                        text: metadataRow.modelData.label
                        textFormat: Text.PlainText
                        color: dialog.theme.muted
                        wrapMode: Text.WrapAnywhere
                    }
                    Label {
                        objectName: "upstreamCaptureMetadataValue_" + metadataRow.index
                        Layout.fillWidth: true
                        Layout.preferredWidth: 0
                        text: metadataRow.modelData.value
                        textFormat: Text.PlainText
                        color: dialog.theme.text
                        wrapMode: Text.WrapAnywhere
                    }
                }
            }
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: table.implicitHeight + dialog.panelInset * 2
                color: dialog.theme.background
                radius: dialog.panelRadius
                ColumnLayout {
                    id: table
                    anchors.fill: parent
                    anchors.margins: dialog.panelInset
                    spacing: dialog.rowSpacing
                    RowLayout {
                        Layout.fillWidth: true
                        visible: dialog.columns.length > 0
                        spacing: dialog.rowSpacing
                        Repeater {
                            model: dialog.columns
                            Label {
                                required property string modelData
                                Layout.fillWidth: true
                                Layout.preferredWidth: 0
                                text: modelData
                                textFormat: Text.PlainText
                                font.bold: true
                                color: dialog.theme.muted
                                wrapMode: Text.WrapAnywhere
                            }
                        }
                    }
                    Repeater {
                        id: dataRepeater
                        objectName: "upstreamCaptureData"
                        model: dialog.dataRows
                        RowLayout {
                            id: dataRow
                            required property int index
                            required property var modelData
                            Layout.fillWidth: true
                            spacing: dialog.rowSpacing
                            Label {
                                objectName: "upstreamCaptureCoordinate_" + dataRow.index
                                Layout.fillWidth: true
                                Layout.preferredWidth: 0
                                text: dataRow.modelData.coordinate
                                textFormat: Text.PlainText
                                color: dialog.theme.text
                                wrapMode: Text.WrapAnywhere
                            }
                            Label {
                                objectName: "upstreamCaptureFirst_" + dataRow.index
                                Layout.fillWidth: true
                                Layout.preferredWidth: 0
                                text: dataRow.modelData.first
                                textFormat: Text.PlainText
                                color: dialog.theme.text
                                wrapMode: Text.WrapAnywhere
                            }
                            Label {
                                objectName: "upstreamCaptureSecond_" + dataRow.index
                                Layout.fillWidth: true
                                Layout.preferredWidth: 0
                                text: dataRow.modelData.second
                                textFormat: Text.PlainText
                                color: dialog.theme.text
                                wrapMode: Text.WrapAnywhere
                            }
                        }
                    }
                }
            }
            RowLayout {
                Layout.fillWidth: true
                spacing: dialog.rowSpacing
                WorkbenchActionButton {
                    objectName: "upstreamCapturePrevious"
                    theme: dialog.theme
                    text: "Previous"
                    enabled: dialog.previousAvailable
                    onClicked: dialog.requestPage(true)
                }
                Label {
                    objectName: "upstreamCapturePageSummary"
                    Layout.fillWidth: true
                    text: dialog.page.summary || ""
                    textFormat: Text.PlainText
                    color: dialog.theme.muted
                    wrapMode: Text.WrapAnywhere
                }
                WorkbenchActionButton {
                    objectName: "upstreamCaptureNext"
                    theme: dialog.theme
                    text: "Next"
                    enabled: dialog.nextAvailable
                    onClicked: dialog.requestPage(false)
                }
            }
            Label {
                objectName: "upstreamCaptureError"
                Layout.fillWidth: true
                visible: text.length > 0
                text: dialog.displayedPresentation.error || ""
                textFormat: Text.PlainText
                color: dialog.theme.statusError
                wrapMode: Text.WrapAnywhere
            }
            Label {
                objectName: "upstreamCaptureAdmission"
                Layout.fillWidth: true
                visible: coverage.rejected
                text: coverage.message
                textFormat: Text.PlainText
                color: dialog.theme.statusAttention
                wrapMode: Text.WordWrap
            }
        }
    }
    footer: Pane {
        id: footerPane
        padding: dialog.footerSpacing
        background: Item {}
        contentItem: GridLayout {
            columns: footerPane.availableWidth >= secondaryActions.implicitWidth + closeButton.implicitWidth + dialog.footerSpacing ? 2 : 1
            columnSpacing: dialog.footerSpacing
            rowSpacing: dialog.footerSpacing
            RowLayout {
                id: secondaryActions
                Layout.fillWidth: true
                spacing: dialog.footerSpacing
                WorkbenchActionButton {
                    objectName: "upstreamCaptureLoad"
                    theme: dialog.theme
                    text: "Load Capture"
                    enabled: dialog.actionsAvailable
                    onClicked: dialog.requestLoad()
                }
                WorkbenchActionButton {
                    objectName: "upstreamCaptureMeasuredPlan"
                    theme: dialog.theme
                    text: "Load Measured Plan"
                    enabled: dialog.actionsAvailable
                    onClicked: dialog.requestMeasuredPlan()
                }
                Item {
                    Layout.fillWidth: true
                }
            }
            WorkbenchActionButton {
                id: closeButton
                objectName: "upstreamCaptureClose"
                theme: dialog.theme
                text: "Close"
                Layout.alignment: Qt.AlignRight
                onClicked: dialog.close()
            }
        }
    }
}
